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
import sys
import json
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


def main():
    parser = argparse.ArgumentParser(description="BERT Mesa Runner")
    parser.add_argument("--steps", type=int, default=200, help="Number of simulation steps")
    parser.add_argument("--seed", type=int, default=42, help="Random seed")
    parser.add_argument("--db", default="bert-models", help="TypeDB database name")
    parser.add_argument("--model-name", required=True, help="BERT model name (bert_id prefix)")
    parser.add_argument("--run-id", required=True, help="Unique simulation run identifier")
    parser.add_argument("--host", default="localhost:1729", help="TypeDB host:port")
    args = parser.parse_args()

    try:
        run(args)
    except Exception as e:
        error_msg = json.dumps({"type": "error", "message": str(e), "trace": traceback.format_exc()})
        print(error_msg, flush=True)

        try:
            driver = connect(host=args.host)
            update_run_status(driver, args.db, args.run_id, "Failed")
            driver.close()
        except Exception:
            pass

        sys.exit(1)


if __name__ == "__main__":
    main()
