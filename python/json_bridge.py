"""Read BERT WorldModel JSON into DataFrames matching typedb_bridge schema."""

import json
import pandas as pd


def _complexity_kind(complexity):
    if isinstance(complexity, str):
        return complexity
    if isinstance(complexity, dict):
        return next(iter(complexity))
    return "Unspecified"


def _build_port_map(model):
    port_map = {}
    for s in model.get("systems", []):
        pi = s.get("boundary", {}).get("parent_interface")
        if pi:
            port_map[pi] = s["info"]["id"]
    return port_map


def read_model(json_path: str) -> tuple[pd.DataFrame, pd.DataFrame]:
    with open(json_path) as f:
        model = json.load(f)

    sys_rows = []
    for s in model.get("systems", []):
        info = s["info"]
        agent = s.get("agent")
        sys_rows.append({
            "bert_id": info["id"],
            "display_name": info["name"],
            "archetype": s.get("archetype", "Unspecified"),
            "time_constant": s.get("time_constant", "Second"),
            "system_level": info["level"],
            "complexity_kind": _complexity_kind(s.get("complexity", "Unspecified")),
            "agent_kind": agent["kind"] if agent else None,
            "agency_capacity": agent["agency_capacity"] if agent else None,
            "primitives": agent["primitives"] if agent and "primitives" in agent else [],
        })
    systems_df = pd.DataFrame(sys_rows) if sys_rows else pd.DataFrame()

    if not systems_df.empty:
        dupes = systems_df[systems_df["bert_id"].duplicated(keep=False)]
        if not dupes.empty:
            raise ValueError(f"Duplicate system IDs: {dupes['bert_id'].unique().tolist()}")

    port_map = _build_port_map(model)

    ix_rows = []
    for ix in model.get("interactions", []):
        info = ix["info"]
        source_id = ix["source"]
        sink_id = ix["sink"]

        sink_iface = ix.get("sink_interface")
        if sink_iface and sink_iface in port_map:
            sink_id = port_map[sink_iface]

        source_iface = ix.get("source_interface")
        if source_iface and source_iface in port_map:
            source_id = port_map[source_iface]

        ix_rows.append({
            "bert_id": info["id"],
            "display_name": info["name"],
            "substance_type": ix["substance"]["type"],
            "usability": ix.get("usability", ""),
            "interaction_type": ix.get("type", "Flow"),
            "amount": float(ix.get("amount", "0")),
            "source_id": source_id,
            "sink_id": sink_id,
        })
    interactions_df = pd.DataFrame(ix_rows) if ix_rows else pd.DataFrame()

    if not interactions_df.empty:
        dupes = interactions_df[interactions_df["bert_id"].duplicated(keep=False)]
        if not dupes.empty:
            raise ValueError(f"Duplicate interaction IDs: {dupes['bert_id'].unique().tolist()}")

    return systems_df, interactions_df
