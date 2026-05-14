"""
Model-agnostic Mesa agents derived from BERT system archetypes.

Behavior is driven by the typed graph structure:
  - archetype determines the agent class (fallback path)
  - primitives determine T functions when present (primary path)
  - agent_kind (Reactive/Anticipatory/Intentional) modulates step logic
  - agency_capacity scales responsiveness
  - time_constant controls step frequency
  - flows define interaction channels

No domain-specific (Bitcoin, RSC, etc.) logic lives here.
Each T function is a pure state transform: (agent, state, inflows, outflows) -> None
Design intent: portable to Rust ECS via PyO3.
"""

import math
from collections import deque

from mesa import Agent


def _safe_float(val, default):
    if val is None:
        return default
    try:
        f = float(val)
        return default if math.isnan(f) else f
    except (TypeError, ValueError):
        return default

TIME_CONSTANT_TICKS = {
    "Millisecond": 1,
    "Second": 1,
    "Minute": 60,
    "Hour": 3600,
    "Day": 86400,
    "Week": 604800,
    "Month": 2592000,
    "Year": 31536000,
    "Decade": 315360000,
    "Century": 3153600000,
    "Epoch": 31536000000,
}


# ---------------------------------------------------------------------------
# Process primitive T functions (Mobus atomic work processes)
#
# Signature: _t_xxx(agent, state, incoming, outgoing) -> None
# Each mutates `state` in place. `incoming`/`outgoing` are lists of flow dicts
# with keys: amount, substance_type, usability.
# Pure transforms — no Mesa coupling, no framework-dependent patterns.
# ---------------------------------------------------------------------------


def _t_buffering(agent, state, incoming, outgoing):
    """s(t+1) = s(t) + Σ(in) - Σ(out). Affine, persistent state."""
    inflow = sum(f.get("amount", 0) for f in incoming)
    demand = sum(f.get("amount", 0) for f in outgoing)
    storage = state.get("storage", 0.0)
    storage += inflow
    if "_base_demand" not in state:
        state["_base_demand"] = demand
    adjusted = state["_base_demand"] * state.get("_release_factor", 1.0)
    released = min(adjusted, storage)
    storage -= released
    state["storage"] = storage
    state["activity"] = released


def _t_combining(agent, state, incoming, outgoing):
    """activity = Σ(M/E inflows). Linear."""
    state["activity"] = sum(
        f.get("amount", 0) for f in incoming
        if f.get("substance_type") in ("Energy", "Material")
    )


def _t_splitting(agent, state, incoming, outgoing):
    """out_i = total / n. Linear, conserves Material/Energy."""
    total = sum(
        f.get("amount", 0) for f in incoming
        if f.get("substance_type") in ("Energy", "Material")
    )
    n = max(len(outgoing), 1)
    share = total / n
    for f in outgoing:
        f["amount"] = min(share, f.get("capacity", float('inf')))
    state["activity"] = total


def _t_propelling(agent, state, incoming, outgoing):
    """out = in * efficiency. Linear."""
    total_in = sum(f.get("amount", 0) for f in incoming)
    eta = agent.agency_capacity
    state["activity"] = total_in * eta


def _t_impeding(agent, state, incoming, outgoing):
    """out = in * (1 - impedance). Linear, creates back_pressure."""
    total_in = sum(f.get("amount", 0) for f in incoming)
    impedance = 1.0 - agent.agency_capacity
    state["activity"] = total_in * (1.0 - impedance)
    state["back_pressure"] = total_in * impedance


def _t_sensing(agent, state, incoming, outgoing):
    """signal = k * Σ(physical inputs). Crosses substance: Energy/Material -> Message."""
    physical_in = sum(
        f.get("amount", 0) for f in incoming
        if f.get("substance_type") in ("Energy", "Material")
    )
    k = agent.agency_capacity
    state["signal"] = physical_in * k
    state["activity"] = state["signal"]


def _t_modulating(agent, state, incoming, outgoing):
    """out = primary * f(control). Bilinear (nonlinear). Two-input primitive."""
    primary = sum(
        f.get("amount", 0) for f in incoming
        if f.get("substance_type") != "Message"
    )
    control = sum(
        f.get("amount", 0) for f in incoming
        if f.get("substance_type") == "Message"
    )
    mod_factor = max(0.0, min(2.0, control))
    state["activity"] = primary * mod_factor
    state["control_signal"] = control


def _t_inverting(agent, state, incoming, outgoing):
    """out = max_val - in. Affine. Message only."""
    total_in = sum(
        f.get("amount", 0) for f in incoming
        if f.get("substance_type") == "Message"
    )
    max_val = state.get("max_signal", 1.0)
    state["activity"] = max(0.0, max_val - total_in)


def _t_copying(agent, state, incoming, outgoing):
    """out_i = in (replication, NOT conservation). Message only."""
    total_in = sum(
        f.get("amount", 0) for f in incoming
        if f.get("substance_type") == "Message"
    )
    for f in outgoing:
        f["amount"] = min(total_in, f.get("capacity", float('inf')))
    state["activity"] = total_in


PRIMITIVE_T = {
    "Buffering":  _t_buffering,
    "Combining":  _t_combining,
    "Splitting":  _t_splitting,
    "Propelling": _t_propelling,
    "Impeding":   _t_impeding,
    "Sensing":    _t_sensing,
    "Modulating": _t_modulating,
    "Inverting":  _t_inverting,
    "Copying":    _t_copying,
}


class BertAgent(Agent):
    """Base agent from a BERT system entity. All behavior derived from graph properties."""

    def __init__(self, model, bert_id, display_name, archetype, time_constant,
                 complexity_kind, agent_kind=None, agency_capacity=None,
                 primitives=None):
        super().__init__(model)
        self.bert_id = bert_id
        self.display_name = display_name
        self.archetype = archetype
        self.time_constant = time_constant
        self.complexity_kind = complexity_kind
        self.agent_kind = agent_kind or "Reactive"
        self.agency_capacity = _safe_float(agency_capacity, 0.5)
        self.primitives = primitives or []

        self.step_interval = TIME_CONSTANT_TICKS.get(time_constant, 1)

        self._base_agency_capacity = self.agency_capacity

        self.state = {}
        self.incoming_flows = []
        self.outgoing_flows = []
        self.force_inputs = []
        self.force_outputs = []
        self.history = deque(maxlen=100)

        self._init_state()

    def _init_state(self):
        """Initialize mutable state from graph properties."""
        self.state["activity"] = 0.0
        self.state["throughput"] = 0.0
        self.state["conservation_deficit"] = 0.0
        if "Buffering" in self.primitives:
            self.state["storage"] = 0.0
            self.state["_release_factor"] = 1.0
        if "Impeding" in self.primitives:
            self.state["back_pressure"] = 0.0
        if "Sensing" in self.primitives:
            self.state["signal"] = 0.0
        if "Modulating" in self.primitives:
            self.state["control_signal"] = 0.0
        if "Inverting" in self.primitives:
            self.state["max_signal"] = 1.0

    def should_step(self, tick: int) -> bool:
        if self.step_interval <= 1:
            return True
        return tick % self.step_interval == 0

    def step(self):
        tick = self.model.current_tick
        if not self.should_step(tick):
            return
        self._process_inputs()
        self._apply_forces()
        self._condition_T()
        if self.primitives:
            self._act_by_primitive()
        else:
            self._act()
        self._produce_outputs()
        self._enforce_conservation()
        self._record_history()

    def _apply_forces(self):
        """Force = parameter injection. Source activity modulates sink's agency_capacity."""
        if not self.force_inputs:
            return
        pos_signal = sum(f.get("amount", 0) for f in self.force_inputs if f.get("polarity") != "negative")
        neg_signal = sum(f.get("amount", 0) for f in self.force_inputs if f.get("polarity") == "negative")
        pos_factor = 0.5 + 0.5 * min(pos_signal, 2.0)
        neg_factor = 1.5 - 0.5 * min(neg_signal, 2.0)
        self.agency_capacity = _safe_float(
            self._base_agency_capacity * pos_factor * neg_factor,
            self._base_agency_capacity,
        )

    def _condition_T(self):
        """Read H (history) to set conditioning parameters before T dispatch.
        Implements Mobus's T(t+1) = f(T(t), H(t), Input(t)). Currently Buffering only."""
        if len(self.history) < 2:
            return
        if "Buffering" in self.primitives:
            recent = [h.get("storage", 0) for h in list(self.history)[-10:]]
            if len(recent) >= 2:
                trend = recent[-1] - recent[0]
                norm = max(abs(trend), 1.0)
                self.state["_release_factor"] = max(0.5, min(1.5, 1.0 + 0.3 * (trend / norm)))

    def _act_by_primitive(self):
        """Dispatch through process primitives in sequence."""
        for prim_name in self.primitives:
            t_fn = PRIMITIVE_T.get(prim_name)
            if t_fn is not None:
                t_fn(self, self.state, self.incoming_flows, self.outgoing_flows)

    def _record_history(self):
        """Append current state snapshot to rolling history window."""
        snapshot = {
            k: v for k, v in self.state.items()
            if isinstance(v, (int, float))
        }
        self.history.append(snapshot)

    def _process_inputs(self):
        """Accumulate incoming flow amounts."""
        self.state["throughput"] = sum(
            f.get("amount", 0) for f in self.incoming_flows
        )

    def _act(self):
        """Archetype-specific behavior. Override in subclasses."""
        self.state["activity"] = self.state["throughput"] * self.agency_capacity

    def _produce_outputs(self):
        _SELF_WRITING = {"Splitting", "Copying"}
        if set(self.primitives) & _SELF_WRITING:
            return
        activity = self.state.get("activity", 0.0)
        for flow in self.outgoing_flows:
            flow["amount"] = min(activity, flow.get("capacity", float('inf')))
        for force in self.force_outputs:
            force["amount"] = activity

    def _enforce_conservation(self):
        """Post-step 1st/2nd Law: M/E outflows clamped to inflow budget. Message exempt."""
        _CONSERVED = ("Energy", "Material")
        me_inflow = sum(
            f.get("amount", 0) for f in self.incoming_flows
            if f.get("substance_type") in _CONSERVED
        )
        me_outflow = sum(
            f.get("amount", 0) for f in self.outgoing_flows
            if f.get("substance_type") in _CONSERVED
        )
        inflow_budget = me_inflow + self.state.get("storage", 0.0)

        if me_outflow <= inflow_budget:
            self.state["conservation_deficit"] = 0.0
            return

        self.state["conservation_deficit"] = me_outflow - inflow_budget

        if inflow_budget <= 0:
            for f in self.outgoing_flows:
                if f.get("substance_type") in _CONSERVED:
                    f["amount"] = 0.0
            return

        ratio = inflow_budget / me_outflow
        for f in self.outgoing_flows:
            if f.get("substance_type") in _CONSERVED:
                f["amount"] *= ratio

    def collect_observations(self) -> list[dict]:
        return [
            {"system_id": self.bert_id, "key": k, "value": float(v)}
            for k, v in self.state.items()
            if isinstance(v, (int, float))
        ]


class EconomySystem(BertAgent):
    """Economy archetype: resource-transforming, throughput-maximizing."""

    def _init_state(self):
        super()._init_state()
        self.state["resource_level"] = 1.0
        self.state["efficiency"] = self.agency_capacity

    def _act(self):
        input_energy = sum(
            f.get("amount", 0) for f in self.incoming_flows
            if f.get("substance_type") == "Energy"
        )
        input_material = sum(
            f.get("amount", 0) for f in self.incoming_flows
            if f.get("substance_type") == "Material"
        )

        self.state["resource_level"] += (input_energy + input_material) * self.state["efficiency"]
        self.state["resource_level"] *= 0.95  # decay
        self.state["activity"] = self.state["resource_level"]


class GovernanceSystem(BertAgent):
    """Governance archetype: consensus-seeking, rule-enforcing."""

    def _init_state(self):
        super()._init_state()
        self.state["consensus"] = 0.5
        self.state["rule_strength"] = self.agency_capacity

    def _act(self):
        messages = sum(
            f.get("amount", 0) for f in self.incoming_flows
            if f.get("substance_type") == "Message"
        )
        self.state["consensus"] += 0.01 * (messages - self.state["consensus"])
        self.state["consensus"] = max(0.0, min(1.0, self.state["consensus"]))
        self.state["rule_strength"] = self.state["consensus"] * self.agency_capacity
        self.state["activity"] = self.state["rule_strength"]


class AgentSystem(BertAgent):
    """Agent archetype: autonomous, adaptive. Behavior modulated by agent_kind."""

    def _init_state(self):
        super()._init_state()
        self.state["belief"] = 0.5
        self.state["adaptation_rate"] = self.agency_capacity

        if self.agent_kind == "Anticipatory":
            self.state["prediction"] = 0.5
        if self.agent_kind == "Intentional":
            self.state["goal"] = 1.0

    def _act(self):
        signal = self.state["throughput"]

        if self.agent_kind == "Reactive":
            self.state["belief"] += self.state["adaptation_rate"] * (signal - self.state["belief"])

        elif self.agent_kind == "Anticipatory":
            error = signal - self.state["prediction"]
            self.state["prediction"] += 0.1 * error
            self.state["belief"] += self.state["adaptation_rate"] * (self.state["prediction"] - self.state["belief"])

        elif self.agent_kind == "Intentional":
            gap = self.state["goal"] - self.state["belief"]
            self.state["belief"] += self.state["adaptation_rate"] * gap * 0.1
            self.state["belief"] += self.state["adaptation_rate"] * (signal - self.state["belief"]) * 0.5

        self.state["belief"] = max(0.0, min(1.0, self.state["belief"]))
        self.state["activity"] = self.state["belief"]


class PassiveSystem(BertAgent):
    """Unspecified archetype: pass-through relay, no autonomous behavior."""

    def _act(self):
        self.state["activity"] = self.state["throughput"]


ARCHETYPE_TO_CLASS = {
    "Economy": EconomySystem,
    "Agent": AgentSystem,
    "Governance": GovernanceSystem,
    "Unspecified": PassiveSystem,
}


def agent_from_row(model, row: dict) -> BertAgent:
    archetype = row.get("archetype", "Unspecified")
    cls = ARCHETYPE_TO_CLASS.get(archetype, PassiveSystem)

    return cls(
        model,
        bert_id=row["bert_id"],
        display_name=row.get("display_name", ""),
        archetype=archetype,
        time_constant=row.get("time_constant", "Second"),
        complexity_kind=row.get("complexity_kind", "Atomic"),
        agent_kind=row.get("agent_kind"),
        agency_capacity=row.get("agency_capacity"),
        primitives=row.get("primitives"),
    )
