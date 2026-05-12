#!/usr/bin/env python3
"""
BERT Mesa Runner — model-agnostic simulation from TypeDB graph.

Reads any BERT model's system structure from TypeDB, instantiates Mesa agents
from archetypes/flows/time_constants, runs simulation, writes observations back.
No hardcoded scenarios — the model graph IS the simulation spec.

Usage:
    python mesa_runner.py --steps 200 --seed 42 \
        --db bert-models --model-name bitcoin --run-id abc-123
"""

import argparse
import math
import os
import sys
import json
import tempfile
import traceback

from typedb_bridge import (
    connect, read_systems, read_interactions,
    write_simulation_run, write_flow_observations, write_system_observations,
    update_run_status,
)
from model import BertModel


WRITE_INTERVAL = 10


def progress(msg: str):
    print(json.dumps({"type": "progress", "message": msg}), flush=True)


def run(args):
    progress(f"Connecting to TypeDB at {args.host}, db={args.db}")
    driver = connect(host=args.host)

    progress(f"Reading model '{args.model_name}' from TypeDB")
    systems_df = read_systems(driver, args.db, args.model_name)
    interactions_df = read_interactions(driver, args.db, args.model_name)

    if systems_df.empty:
        raise RuntimeError(f"No systems found for model '{args.model_name}' in db '{args.db}'")

    progress(f"Loaded {len(systems_df)} systems, {len(interactions_df)} interactions")

    progress(f"Creating simulation run {args.run_id}")
    write_simulation_run(driver, args.db, args.run_id, args.model_name,
                         args.seed, args.steps)

    progress(f"Initializing model (seed={args.seed})")
    model = BertModel(
        systems_df=systems_df,
        interactions_df=interactions_df,
        seed=args.seed,
    )

    progress(f"Running {args.steps} steps")
    for step in range(1, args.steps + 1):
        model.step()

        if step % WRITE_INTERVAL == 0 or step == args.steps:
            flow_obs, sys_obs = model.collect_all_observations()
            write_flow_observations(driver, args.db, args.run_id, step, flow_obs)
            write_system_observations(driver, args.db, args.run_id, step, sys_obs)
            update_run_status(driver, args.db, args.run_id, "Running", step)
            progress(f"Step {step}/{args.steps}")

    update_run_status(driver, args.db, args.run_id, "Complete", args.steps)
    progress("Simulation complete")

    summary = model.datacollector.get_model_vars_dataframe().to_dict(orient="list")
    print(json.dumps({"type": "result", "summary": summary}), flush=True)

    driver.close()


def run_json(args):
    from json_bridge import read_model

    progress(f"Reading model from {args.json_path}")
    systems_df, interactions_df = read_model(args.json_path)

    if systems_df.empty:
        raise RuntimeError(f"No systems found in {args.json_path}")

    progress(f"Loaded {len(systems_df)} systems, {len(interactions_df)} interactions")

    if args.params:
        overrides = json.loads(args.params)
        for fid, amount in overrides.items():
            mask = interactions_df["bert_id"] == fid
            interactions_df.loc[mask, "amount"] = float(amount)
        progress(f"Applied {len(overrides)} parameter override(s)")

    tmpdir = tempfile.gettempdir()
    status_path = os.path.join(tmpdir, f"{args.run_id}_status.json")
    results_path = os.path.join(tmpdir, f"{args.run_id}_results.json")

    progress(f"Initializing model (seed={args.seed})")
    model = BertModel(
        systems_df=systems_df,
        interactions_df=interactions_df,
        seed=args.seed,
    )

    flow_timeseries = {}
    sys_timeseries = {}

    progress(f"Running {args.steps} steps")
    for step in range(1, args.steps + 1):
        model.step()

        if step % WRITE_INTERVAL == 0 or step == args.steps:
            flow_obs, sys_obs = model.collect_all_observations()

            for obs in flow_obs:
                fid = obs["interaction_id"]
                if fid not in flow_timeseries:
                    flow_timeseries[fid] = {"ticks": [], "values": []}
                flow_timeseries[fid]["ticks"].append(step)
                flow_timeseries[fid]["values"].append(obs["amount"])

            for obs in sys_obs:
                key = (obs["system_id"], obs["key"])
                if key not in sys_timeseries:
                    sys_timeseries[key] = {"ticks": [], "values": []}
                sys_timeseries[key]["ticks"].append(step)
                sys_timeseries[key]["values"].append(obs["value"])

            with open(status_path, "w") as f:
                json.dump({
                    "run_id": args.run_id,
                    "status": "Running",
                    "tick_count": step,
                    "total_ticks": args.steps,
                }, f)

            progress(f"Step {step}/{args.steps}")

    ix_name_map = {}
    ix_sink_map = {}
    if not interactions_df.empty:
        ix_name_map = dict(zip(interactions_df["bert_id"], interactions_df["display_name"]))
        ix_sink_map = dict(zip(interactions_df["bert_id"], interactions_df["sink_id"]))

    sys_name_map = {}
    if not systems_df.empty:
        sys_name_map = dict(zip(systems_df["bert_id"], systems_df["display_name"]))

    results = {
        "run_id": args.run_id,
        "flow_timeseries": [
            {
                "interaction_id": fid,
                "name": ix_name_map.get(fid, fid),
                "sink_id": ix_sink_map.get(fid, ""),
                "ticks": ts["ticks"],
                "values": ts["values"],
            }
            for fid, ts in flow_timeseries.items()
        ],
        "system_timeseries": [
            {
                "system_id": sid,
                "name": sys_name_map.get(sid, sid),
                "key": skey,
                "ticks": ts["ticks"],
                "values": ts["values"],
            }
            for (sid, skey), ts in sys_timeseries.items()
        ],
    }

    def sanitize(v):
        if isinstance(v, float) and (math.isnan(v) or math.isinf(v)):
            return 0.0
        return v

    for ts in results["flow_timeseries"]:
        ts["values"] = [sanitize(v) for v in ts["values"]]
    for ts in results["system_timeseries"]:
        ts["values"] = [sanitize(v) for v in ts["values"]]

    tmp_results = results_path + ".tmp"
    with open(tmp_results, "w") as f:
        json.dump(results, f)
    os.replace(tmp_results, results_path)

    with open(status_path, "w") as f:
        json.dump({
            "run_id": args.run_id,
            "status": "Complete",
            "tick_count": args.steps,
            "total_ticks": args.steps,
        }, f)

    progress("Simulation complete")
    print(json.dumps({"type": "result", "results_path": results_path}), flush=True)


def main():
    parser = argparse.ArgumentParser(description="BERT Mesa Runner")
    parser.add_argument("--steps", type=int, default=200, help="Number of simulation steps")
    parser.add_argument("--seed", type=int, default=42, help="Random seed")
    parser.add_argument("--db", default="bert-models", help="TypeDB database name")
    parser.add_argument("--model-name", default="", help="BERT model name (bert_id prefix)")
    parser.add_argument("--run-id", required=True, help="Unique simulation run identifier")
    parser.add_argument("--host", default="localhost:1729", help="TypeDB host:port")
    parser.add_argument("--json-path", default=None, help="Path to BERT JSON model file (skips TypeDB)")
    parser.add_argument("--params", default=None, help="JSON dict of flow_id:amount overrides")
    args = parser.parse_args()

    try:
        if args.json_path:
            run_json(args)
        else:
            if not args.model_name:
                parser.error("--model-name is required when not using --json-path")
            run(args)
    except Exception as e:
        error_msg = json.dumps({"type": "error", "message": str(e), "trace": traceback.format_exc()})
        print(error_msg, flush=True)

        if not args.json_path:
            try:
                driver = connect(host=args.host)
                update_run_status(driver, args.db, args.run_id, "Failed")
                driver.close()
            except Exception:
                pass

        sys.exit(1)


if __name__ == "__main__":
    main()
