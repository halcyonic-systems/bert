"""
Model-agnostic Mesa agents derived from BERT system archetypes.

Behavior is driven by the typed graph structure:
  - archetype determines the agent class
  - agent_kind (Reactive/Anticipatory/Intentional) modulates step logic
  - agency_capacity scales responsiveness
  - time_constant controls step frequency
  - flows define interaction channels

No domain-specific (Bitcoin, RSC, etc.) logic lives here.
"""

from mesa import Agent

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


class BertAgent(Agent):
    """Base agent from a BERT system entity. All behavior derived from graph properties."""

    def __init__(self, model, bert_id, display_name, archetype, time_constant,
                 complexity_kind, agent_kind=None, agency_capacity=None):
        super().__init__(model)
        self.bert_id = bert_id
        self.display_name = display_name
        self.archetype = archetype
        self.time_constant = time_constant
        self.complexity_kind = complexity_kind
        self.agent_kind = agent_kind or "Reactive"
        self.agency_capacity = agency_capacity or 0.5

        self.step_interval = TIME_CONSTANT_TICKS.get(time_constant, 1)

        self.state = {}
        self.incoming_flows = []
        self.outgoing_flows = []

        self._init_state()

    def _init_state(self):
        """Initialize mutable state from graph properties."""
        self.state["activity"] = 0.0
        self.state["throughput"] = 0.0

    def should_step(self, tick: int) -> bool:
        if self.step_interval <= 1:
            return True
        return tick % self.step_interval == 0

    def step(self):
        tick = self.model.current_tick
        if not self.should_step(tick):
            return
        self._process_inputs()
        self._act()
        self._produce_outputs()

    def _process_inputs(self):
        """Accumulate incoming flow amounts."""
        self.state["throughput"] = sum(
            f.get("amount", 0) for f in self.incoming_flows
        )

    def _act(self):
        """Archetype-specific behavior. Override in subclasses."""
        self.state["activity"] = self.state["throughput"] * self.agency_capacity

    def _produce_outputs(self):
        """Scale outgoing flows by activity level."""
        for flow in self.outgoing_flows:
            flow["amount"] = flow.get("amount", 0) * (0.5 + 0.5 * self.agency_capacity)

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
    )
