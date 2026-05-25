"""
Model-agnostic Mesa Model for any BERT system graph.

Reads system structure from TypeDB, creates agents by archetype,
wires flows as interaction channels. No domain-specific logic —
behavior emerges from the typed graph structure.
"""

import logging

import pandas as pd
from mesa import Model, DataCollector

from agents import agent_from_row

logger = logging.getLogger(__name__)

PRIMITIVE_SUBSTANCE_VALID = {
    "Buffering":  {"Energy", "Material", "Message"},
    "Combining":  {"Energy", "Material"},
    "Splitting":  {"Energy", "Material"},
    "Propelling": {"Energy", "Material", "Message"},
    "Impeding":   {"Energy", "Material", "Message"},
    "Sensing":    {"Energy", "Material"},
    "Modulating": {"Energy", "Material", "Message"},
    "Inverting":  {"Message"},
    "Copying":    {"Message"},
}


class BertModel(Model):
    """
    Generic BERT simulation model. Works with any model in TypeDB.

    Agent behavior is determined by archetype (Agent/Economy/Governance/Unspecified),
    agent_kind (Reactive/Anticipatory/Intentional), and flow topology.
    """

    def __init__(self, systems_df: pd.DataFrame, interactions_df: pd.DataFrame,
                 seed: int = None, perturbations: dict[int, float] | None = None):
        super().__init__(seed=seed)

        self.current_tick = 0
        self.interactions_df = interactions_df
        self.perturbations = perturbations or {}

        self._create_agents(systems_df)
        self._build_flow_adjacency(interactions_df)
        self._validate_substance_compatibility()
        self._build_force_adjacency(interactions_df)
        self._setup_datacollector()

        self.datacollector.collect(self)

    def _create_agents(self, systems_df: pd.DataFrame):
        self._agents_by_bert_id = {}
        for _, row in systems_df.iterrows():
            agent = agent_from_row(self, row.to_dict())
            self._agents_by_bert_id[row["bert_id"]] = agent

    def _build_flow_adjacency(self, interactions_df: pd.DataFrame):
        if interactions_df.empty:
            return

        for _, row in interactions_df.iterrows():
            if row.get("interaction_type", "Flow") == "Force":
                continue
            raw_cap = row.get("capacity")
            capacity = float(raw_cap) if raw_cap is not None else float('inf')
            flow_info = {
                "bert_id": row["bert_id"],
                "substance_type": row.get("substance_type", ""),
                "usability": row.get("usability", ""),
                "amount": float(row.get("amount", 0)),
                "capacity": capacity,
                "_source_id": row.get("source_id", ""),
            }
            src = self._agents_by_bert_id.get(row.get("source_id"))
            snk = self._agents_by_bert_id.get(row.get("sink_id"))
            if src:
                src.outgoing_flows.append(flow_info)
            if snk:
                snk.incoming_flows.append(flow_info)

    def _validate_substance_compatibility(self):
        for agent in self.agents:
            if not agent.primitives:
                continue
            allowed = set()
            for prim in agent.primitives:
                allowed |= PRIMITIVE_SUBSTANCE_VALID.get(prim, set())
            for flow in agent.incoming_flows:
                stype = flow.get("substance_type", "")
                if stype and stype not in allowed:
                    logger.warning(
                        "Invalid substance for %s (%s): flow %s carries %s",
                        agent.display_name, ", ".join(agent.primitives),
                        flow.get("bert_id", "?"), stype,
                    )

    def _build_force_adjacency(self, interactions_df: pd.DataFrame):
        if interactions_df.empty:
            return
        for _, row in interactions_df.iterrows():
            if row.get("interaction_type") != "Force":
                continue
            force_info = {
                "bert_id": row["bert_id"],
                "substance_type": row.get("substance_type", ""),
                "amount": float(row.get("amount", 0)),
                "polarity": row.get("force_polarity", "positive"),
            }
            src = self._agents_by_bert_id.get(row.get("source_id"))
            snk = self._agents_by_bert_id.get(row.get("sink_id"))
            if src and snk:
                src.force_outputs.append(force_info)
                snk.force_inputs.append(force_info)

    def _setup_datacollector(self):
        """Build DataCollector dynamically from whatever agents exist."""
        model_reporters = {
            "agent_count": lambda m: len(list(m.agents)),
            "tick": lambda m: m.current_tick,
        }

        # Add per-archetype counts
        archetypes = set()
        for agent in self.agents:
            archetypes.add(agent.archetype)

        for arch in sorted(archetypes):
            a = arch
            model_reporters[f"count_{a}"] = lambda m, a=a: sum(
                1 for ag in m.agents if ag.archetype == a
            )

        self.datacollector = DataCollector(model_reporters=model_reporters)

    def step(self):
        self.current_tick += 1
        if self.current_tick in self.perturbations:
            self._apply_perturbation(self.perturbations[self.current_tick])
        self.agents.shuffle_do("step")
        self.datacollector.collect(self)

    def _apply_perturbation(self, multiplier: float):
        for agent in self.agents:
            for flow in agent.incoming_flows:
                src_id = flow.get("_source_id", "")
                if src_id not in self._agents_by_bert_id:
                    flow["amount"] *= multiplier
        logger.info("Perturbation at step %d: external source flows scaled by %.2f",
                    self.current_tick, multiplier)

    def collect_all_observations(self) -> tuple[list[dict], list[dict]]:
        flow_obs = []
        for _, row in self.interactions_df.iterrows():
            src = self._agents_by_bert_id.get(row.get("source_id"))
            if src:
                total = sum(
                    f.get("amount", 0) for f in src.outgoing_flows
                    if f["bert_id"] == row["bert_id"]
                )
                flow_obs.append({"interaction_id": row["bert_id"], "amount": float(total)})

        sys_obs = []
        for agent in self.agents:
            sys_obs.extend(agent.collect_observations())

        return flow_obs, sys_obs
