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
    "Amplifying": {"Energy", "Message"},
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
                 seed: int = None, perturbations: dict[int, float] | None = None,
                 update_mode: str = "async"):
        super().__init__(seed=seed)

        from collections import deque
        self.current_tick = 0
        self.interactions_df = interactions_df
        self.perturbations = perturbations or {}
        self.system_history = deque(maxlen=100)
        # "async" (default): push-based, shuffled, correct for regulation circuits.
        # "synchronous": two-phase compute->commit, order-independent, exact mass
        # conservation for stock-to-stock transfer (SIR, Lotka-Volterra).
        self.update_mode = update_mode
        # Pre-tick storage levels, frozen at the top of each synchronous step for
        # observation flows to read. Initialized unconditionally so the async path
        # and any observation read can .get() it without an AttributeError (B5).
        self._level_snapshot = {}

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
            raw_obs = row.get("observation", False)
            _obs = bool(raw_obs) if raw_obs == raw_obs else False  # NaN != NaN -> False
            flow_info = {
                "bert_id": row["bert_id"],
                "substance_type": row.get("substance_type", ""),
                "usability": row.get("usability", ""),
                "amount": float(row.get("amount", 0)),
                "capacity": capacity,
                "_source_id": row.get("source_id", ""),
                # Observation tap (carried on interaction.parameters observation:true,
                # parsed in json_bridge): a non-draining level read. Set on EVERY flow
                # so downstream code can read flow["observation"] unambiguously.
                # _obs is NaN-safe: pandas fills absent keys with NaN when a model mixes
                # rows with and without the column, and bool(NaN) is True in Python.
                "observation": _obs,
            }
            src = self._agents_by_bert_id.get(row.get("source_id"))
            snk = self._agents_by_bert_id.get(row.get("sink_id"))
            if flow_info["observation"] and (src is None or "Buffering" not in (src.primitives or [])):
                logger.warning(
                    "Observation flow %s does not originate from a Buffering agent; "
                    "its level-snapshot read will be 0.0", row.get("bert_id", "?"),
                )
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
        if self.update_mode == "synchronous":
            self._step_synchronous()
        else:
            self.agents.shuffle_do("step")
        self._record_system_history()
        self.datacollector.collect(self)

    def _step_synchronous(self):
        """Order-independent two-phase tick for conservative compartmental models.

        1. Snapshot pre-tick storage levels (observation flows read these, never the
           live flow amount, so sensing a stock can't drain it).
        2. Every agent's compute() — reads incoming flow amounts that are still last
           tick's committed values (no agent has written yet), runs T-functions into
           self.state only.
        3. Every agent's commit() — writes activity to shared outgoing flows.

        No shuffle: when no compute() reads another agent's fresh write, agent order
        cannot affect the result. One tick's transferred mass is "on the wire" and
        credited to the sink at T+1, so the ledger is exact."""
        self._level_snapshot = {
            a.bert_id: a.state.get("storage", 0.0) for a in self.agents
        }
        for a in self.agents:
            a.compute()
        for a in self.agents:
            a.commit()

    def _record_system_history(self):
        agents = list(self.agents)
        if not agents:
            return
        activities = [a.state.get("activity", 0.0) for a in agents]
        throughputs = [a.state.get("throughput", 0.0) for a in agents]
        deficits = [a.state.get("conservation_deficit", 0.0) for a in agents]
        n = len(agents)
        self.system_history.append({
            "tick": self.current_tick,
            "mean_activity": sum(activities) / n,
            "total_throughput": sum(throughputs),
            "conservation_deficit_sum": sum(deficits),
            "agent_count": n,
        })

    def _apply_perturbation(self, multiplier: float):
        for agent in self.agents:
            for flow in agent.incoming_flows:
                src_id = flow.get("_source_id", "")
                if src_id not in self._agents_by_bert_id:
                    flow["amount"] *= multiplier
        logger.info("Perturbation at step %d: external source flows scaled by %.2f",
                    self.current_tick, multiplier)

    def total_conserved_mass(self) -> float:
        """Σ buffer storage + Σ in-flight transfer-flow amount (Energy/Material only,
        observation taps excluded, each flow dict counted once).

        In synchronous transfer one tick's released mass sits 'on the wire' (in the
        flow amount) between the source's debit and the sink's credit, so storage
        alone is not conserved tick-to-tick — this sum is. The closed-loop test asserts
        it holds to 1e-9 every tick."""
        total = sum(a.state.get("storage", 0.0) for a in self.agents)
        seen = set()
        for a in self.agents:
            for f in a.outgoing_flows:
                if f.get("observation", False):
                    continue
                if f.get("substance_type") not in ("Energy", "Material"):
                    continue
                if id(f) in seen:
                    continue
                seen.add(id(f))
                total += f.get("amount", 0.0)
        return total

    def classify_openness(self) -> dict:
        """Classify the model as 'open' or 'closed' with respect to mass.

        A system is *closed* when no Energy/Material flow crosses its boundary — every
        mass flow runs agent↔agent. It is *open* when some mass flow has an external
        (non-agent) source feeding it or an external sink draining it. Message flows and
        observation taps are ignored (information isn't conserved; a tap carries no mass).

        Closed ⟹ total mass is the invariant (`total_conserved_mass`). Open ⟹ internal
        mass changes by net boundary flux (inflow − outflow); the conserved statement is
        internal(t) = internal(0) + Σ inflow − Σ outflow.

        This is a structural property (load-time, no simulation needed). An empty
        environment with no boundary-crossing flows is not malformed — it is the
        *signature* of a closed system.
        """
        agent_ids = set(self._agents_by_bert_id)
        inflow_sources, outflow_sinks = set(), set()
        if not self.interactions_df.empty:
            for _, row in self.interactions_df.iterrows():
                if row.get("interaction_type", "Flow") == "Force":
                    continue
                if row.get("substance_type") not in ("Energy", "Material"):
                    continue
                obs = row.get("observation", False)
                if (obs == obs) and bool(obs):  # NaN-safe; skip observation taps
                    continue
                src, snk = row.get("source_id"), row.get("sink_id")
                if src and src not in agent_ids:
                    inflow_sources.add(src)
                if snk and snk not in agent_ids:
                    outflow_sinks.add(snk)
        is_open = bool(inflow_sources or outflow_sinks)
        return {
            "class": "open" if is_open else "closed",
            "boundary_inflow_sources": sorted(inflow_sources),
            "boundary_outflow_sinks": sorted(outflow_sinks),
            "invariant": "internal mass changes by net boundary flux"
            if is_open else "total mass conserved (total_conserved_mass)",
        }

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
