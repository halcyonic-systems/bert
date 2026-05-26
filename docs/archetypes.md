# System Archetypes

Classification framework for systems modeled in BERT. Archetypes determine a subsystem's structural role, behavioral type, and simulation behavior.

## Overview

BERT classifies every subsystem along three complementary axes:

| Axis | Field | Type | Question it answers |
|------|-------|------|---------------------|
| **System level** | (implicit) | Hierarchy position | What kind of system *is* this? |
| **Functional role** | `archetype` | `HcgsArchetype` enum | What role does it play in its parent? |
| **Behavioral type** | `agent.kind` | `AgentKind` enum | How does it make decisions? |
| **Autonomy degree** | `agent.agency_capacity` | `f32` (0.0–1.0) | How much autonomous decision-making? |

The first axis is the archetype hierarchy from Mobus's ontology — a system's position in the natural hierarchy of system types. The remaining three are BERT-specific fields that make archetype classification concrete and simulatable.

---

## 1. Decision Guide

Use this when classifying a subsystem in a BERT model.

### Step 1: Does it make decisions?

- **No** → `archetype: "Unspecified"`, no `agent` field. Done.
- **Yes** → Continue.

### Step 2: What role does it play in its parent system?

- **Regulates other subsystems** → `archetype: "Governance"`. No `agent` field needed (governance acts as agent implicitly through the HCGS framework). If you need detailed behavioral parameters for simulation, add `agent` with appropriate `kind`.
- **Manages resource flows** → `archetype: "Economy"`. No `agent` field. If individual economic actors need modeling, decompose further and classify children as Agents.
- **Autonomous decision-maker** → `archetype: "Agent"`. Continue to Step 3.

### Step 3: How sophisticated is the decision-making?

- **Fixed rules, no learning** → `kind: "Reactive"`, `agency_capacity` ~0.2–0.4
- **Predicts and adapts** → `kind: "Anticipatory"`, `agency_capacity` ~0.4–0.7
- **Sets own goals, plans** → `kind: "Intentional"`, `agency_capacity` ~0.6–0.9

### Step 4: How much autonomy?

Adjust `agency_capacity` within the kind's typical range based on:
- How much external direction does it receive?
- How much discretion does it exercise?
- Would removing governance constraints change its behavior significantly?

The rest of this document explains the theory behind each classification. If you just need to assign archetypes, the guide above is sufficient.

---

## 2. The Archetype Hierarchy

Mobus organizes all systems into a hierarchy of increasing organizational complexity. Each level inherits all properties of levels below it and adds new emergent capabilities.

### Hierarchy of System Types

| Level | Type | Defining Capability | Example Systems |
|-------|------|--------------------:|-----------------|
| 0 | **Physical** | Boundary, energy flow, conservation laws | Star, river, crystal |
| 1 | **Chemical** | Transformation of substances, reaction networks | Combustion engine, chemical reactor |
| 2 | **Biological** | Self-maintenance (autopoiesis), reproduction, adaptation | Cell, organism, ecosystem |
| 3 | **Social** | Communication, shared meaning, institutional structure | Organization, economy, governance body |
| 4 | **Cognitive** | Goal-setting, planning, self-reflection | Human decision-maker, autonomous agent |

This hierarchy is cumulative. A biological system *is* a chemical system (it transforms substances) which *is* a physical system (it has boundaries and energy flow). A social system inherits all of these and adds communication and institutional structure.

### Practical Guidance: Which Level?

When modeling a system, pick the lowest level that captures the capabilities you need to represent.

- Modeling a solar panel's energy conversion? **Physical** (Level 0) — the system transforms energy, no adaptation or goal-setting needed.
- Modeling a cell's metabolism? **Biological** (Level 2) — you need autopoietic self-maintenance, not just chemical transformation.
- Modeling an organization's decision-making? **Social** (Level 3) — departments communicate and coordinate through institutional structure.
- Modeling an autonomous trading agent? **Cognitive** (Level 4) — the agent sets its own goals and plans multi-step actions.

The level you choose constrains what capabilities are available. You don't need to model adaptation for a solar panel, and you don't need to model self-reflection for a cell.

### How Archetype Level Maps to BERT Fields

| Level | Typical `archetype` | Typical `agent.kind` | `agency_capacity` range |
|-------|--------------------:|---------------------:|------------------------:|
| 0 Physical | `Unspecified` | N/A (no agent model) | N/A |
| 1 Chemical | `Unspecified` | N/A | N/A |
| 2 Biological | `Agent` | `Reactive` | 0.0–0.4 |
| 3 Social | `Governance`, `Economy`, or `Agent` | `Reactive` or `Anticipatory` | 0.2–0.7 |
| 4 Cognitive | `Agent` | `Anticipatory` or `Intentional` | 0.5–1.0 |

These are typical mappings, not hard constraints. A governance subsystem within a biological organism (e.g., the nucleus) is valid. The hierarchy describes the *system being modeled*, not the subsystem's role within its parent.

---

## 3. HCGS Functional Archetypes

The Human-Centered General Systems (HCGS) classification from Mobus Ch. 9 identifies three sub-archetypes that constitute the major subsystems of any Complex Adaptive and Evolvable System (CAES):

> "These will be the 'master' archetypes of complex adaptive and evolvable systems (CAS/CAES) and three sub-archetypes that constitute the major subsystems of any CAS/CAES. These are: **agent (with agency), economy, and governance**..." — Mobus Ch. 9, p. 394

### The Three Sub-Archetypes

| Archetype | `HcgsArchetype` value | Role | Key Characteristic |
|-----------|----------------------:|------|-------------------|
| **Governance** | `"Governance"` | Regulatory and control subsystem | Senses, decides, acts *on behalf of the parent system* |
| **Economy** | `"Economy"` | Resource flow and production subsystem | Emerges from agent interactions; manages value flows |
| **Agent** | `"Agent"` | Autonomous decision-making subsystem | Receives information, makes decisions, takes action |

A fourth value, `"Unspecified"`, is used for subsystems that don't fit these categories — typically physical or chemical subsystems without decision-making capability.

### Recursive, Scale-Dependent

The relationship between these archetypes is recursive and scale-dependent:

> "Systems (and systemness) are naturally recursive structures (and concepts)." — Mobus Ch. 9, p. 394

The same entity can function as an **agent** when viewed from a higher level and as a **system composed of agents** when analyzed at its own level. A governance subsystem is simultaneously:
- An agent making decisions for the parent system (macro view)
- A system containing human decision agents at multiple levels (internal view)

This means HCGS archetype assignment depends on the level of analysis. When you decompose a Governance subsystem, its internal components may themselves be Agent, Governance, or Economy subsystems.

### Fuzzy Boundaries

Mobus acknowledges that subsystem membership isn't always crisp:

> "Many components can effectively multiplex or serve roles in multiple subsystems at different times and with different probabilities." — Mobus Ch. 9, p. 404

In BERT, this is handled by `member_autonomy` (0.0–1.0), which captures the degree to which a subsystem is a dedicated member of its parent vs. shared across contexts.

### When to Use Each Archetype

**Governance** — the subsystem regulates other subsystems:
- Executive leadership coordinating departments
- Nuclear control center in a cell
- Protocol governance in a blockchain
- Regulatory bodies in an economy

**Economy** — the subsystem manages resource flows and production:
- Manufacturing and supply chain subsystems
- Metabolic pathways in biological systems
- Token economics in cryptoeconomic systems
- Financial departments in organizations

**Agent** — the subsystem makes autonomous decisions:
- Individual decision-makers (humans, software agents)
- Validator nodes in a blockchain
- Organisms in an ecosystem
- Departments with delegated authority

**Unspecified** — the subsystem doesn't fit the HCGS classification:
- Physical infrastructure (pipes, wires, structural supports)
- Passive storage buffers
- Chemical reactors without feedback control
- Any subsystem where the HCGS distinction isn't analytically useful

### Summary Table

| Archetype | Functions as Agent? | Contains Agents? | Mobus Source |
|-----------|:-------------------:|:----------------:|-------------|
| **Agent** | Yes (by definition) | Can be decomposed | Ch. 11: "special case of an adaptive (and evolvable) system" |
| **Governance** | Yes (explicitly) | Yes (needs decision agents) | Ch. 7, p. 368: "considered as the agent that makes the decisions" |
| **Economy** | Not explicitly | Yes (emerges from agents) | Ch. 9, p. 434: "result of... intentional agents" |

---

## 4. Agent Behavioral Classification

When a subsystem is classified as `archetype: "Agent"`, the `AgentModel` provides two additional classification dimensions: the discrete `AgentKind` and the continuous `agency_capacity`.

### AgentKind: The Decision Architecture

Mobus's decision agent hierarchy (Ch. 11) classifies agents by the sophistication of their decision-making:

| Level | `AgentKind` | Decision Architecture | Capabilities | Example |
|-------|-------------|----------------------|--------------|---------|
| 1 | **Reactive** | Stimulus-response | Fixed input-output mapping; no internal model of the world | Thermostat, simple validator, bypass diode |
| 2 | **Anticipatory** | Predictive model | Predicts future states; adapts behavior based on predictions | Fee-optimizing miner, market maker, MPPT controller |
| 3 | **Intentional** | Goal-directed planning | Sets own goals; plans multi-step actions; reflects on outcomes | Autonomous governance agent, strategic planner |

Each level inherits the capabilities below it. An Anticipatory agent can still do stimulus-response (Reactive behavior) but also predicts. An Intentional agent predicts and plans.

Mobus defines the decision agent as having three core components:

1. **Computational Engine** — processes inputs and generates candidate actions
2. **Decision Model** — evaluates candidates against goals/criteria
3. **Experiential Memory** — stores past outcomes to improve future decisions

A Reactive agent has a minimal computational engine and no experiential memory. An Anticipatory agent adds a predictive model. An Intentional agent adds goal-generation and planning over the predictive model.

### agency_capacity: The Autonomy Scalar

`agency_capacity` (0.0–1.0) is the continuous dimension within the discrete hierarchy. It measures *how much* autonomous decision-making a subsystem exercises, independent of *what type* of decisions it makes.

| Range | Semantic Label | Meaning |
|-------|---------------|---------|
| 0.0–0.2 | Fully directed | Executes instructions with minimal discretion |
| 0.2–0.4 | Low autonomy | Some discretion within tight constraints |
| 0.4–0.6 | Semi-autonomous | Balances external direction with internal judgment |
| 0.6–0.8 | High autonomy | Operates independently within broad parameters |
| 0.8–1.0 | Fully autonomous | Self-directed; sets own operating parameters |

### How AgentKind and agency_capacity Relate

These are independent dimensions, not redundant measures:

- `AgentKind` classifies *what type of decision-making* — the architecture of the agent
- `agency_capacity` measures *how much autonomy* — the degree of independence exercised

A Reactive agent with high agency_capacity (0.8) is a simple stimulus-response system that operates with very little external oversight — like an autonomous thermostat that nobody adjusts. An Intentional agent with low agency_capacity (0.3) is a sophisticated planner that operates under tight constraints — like a strategic analyst who produces recommendations but cannot act on them.

**Default mapping**: When a user selects an `AgentKind` in the BERT UI, `agency_capacity` soft-resets to a kind-appropriate default:

| AgentKind | Default `agency_capacity` |
|-----------|:-------------------------:|
| Reactive | 0.25 |
| Anticipatory | 0.50 |
| Intentional | 0.75 |

Both remain independently editable after the default is set.

---

## 5. How Archetypes Connect to Simulation

BERT is a modeling tool, not a simulation runtime. But the archetype classification directly determines simulation behavior when a BERT model is exported to external frameworks (Mesa, Bevy ECS) or consumed by the GSR pipeline.

### Archetype as Behavioral Contract

The `archetype` field selects the agent behavior class in simulation:

| `archetype` | Simulation Role | Behavioral Profile |
|-------------|----------------|-------------------|
| `"Governance"` | Controller | Receives state from other subsystems, emits directives/forces |
| `"Economy"` | Network | Mediates resource flows between agents; emergent behavior from agent interactions |
| `"Agent"` | Actor | Reads `AgentModel` for decision parameters, cognitive params, process primitives |
| `"Unspecified"` | Passive | Participates in flows but has no autonomous behavior |

### AgentKind as Simulation Depth

For Agent-archetype subsystems, `AgentKind` determines how much decision-making infrastructure the simulation instantiates:

| `AgentKind` | Simulation Behavior |
|-------------|-------------------|
| **Reactive** | Lookup table or rule set; no state beyond current inputs |
| **Anticipatory** | Maintains internal model; updates predictions each tick; selects action based on predicted outcomes |
| **Intentional** | Maintains goals, plans, and experiential memory; re-plans when predictions diverge from observations |

### agency_capacity as Behavioral Weight

`agency_capacity` modulates how strongly an agent's autonomous behavior influences system dynamics vs. being directed by governance:

- **Low capacity (0.0–0.3)**: Agent behavior largely determined by incoming governance forces/directives
- **Mid capacity (0.3–0.7)**: Agent balances governance directives with internal decision-making
- **High capacity (0.7–1.0)**: Agent acts primarily on internal decisions; governance influence is advisory

### The Full Pipeline

```
BERT Model (JSON)
  archetype: "Agent"           --> simulation framework selects Agent behavior class
  agent.kind: "Anticipatory"   --> instantiates predictive model + adaptive behavior
  agent.agency_capacity: 0.8   --> weights autonomous decisions heavily vs governance
  agent.primitives: [...]      --> declares available process functions
  agent.cognitive_params: {...} --> parameterizes domain-specific behavior
  agent.process_configs: [...]  --> configures process execution
  agent.initial_state: {...}   --> sets starting conditions
```

### Three Integration Paths

| Path | Framework | What it reads | Status |
|------|-----------|--------------|--------|
| External (Python) | Mesa | `agent.kind`, `agent.cognitive_params`, `agent.primitives`, `agent.process_configs`, `agent.initial_state` | Working |
| External (React/Flask) | bitcoin-abm-v2 | `archetype` for behavior profiles, `complexity` for interaction depth | Working |
| Integrated (Rust) | Bevy ECS | Full `AgentModel` + substance buffers + temporal simulation | Experimental |

---

## 6. Process Primitives and Archetypes

The 9 atomic process primitives (Mobus Ch. 3) declare *what transformations* a subsystem performs. They are orthogonal to the archetype classification but constrained by it.

| Category | Primitives | Typical Archetype Context |
|----------|-----------|--------------------------|
| **Material** | Combining, Splitting, Buffering | Economy subsystems, physical/chemical systems |
| **Energetic** | Impeding, Propelling | Physical systems, infrastructure |
| **Informational** | Copying, Sensing, Modulating, Inverting | Governance and Agent subsystems |

Agent subsystems typically declare informational primitives (Sensing, Modulating) because decision-making is fundamentally an information-processing activity. Economy subsystems typically declare material primitives (Combining, Splitting, Buffering) because resource management is fundamentally about material transformation. Governance subsystems span both — they Sense state (informational) and may Modulate or Impede resource flows (energetic/material).

These are tendencies, not rules. A governance subsystem might Buffer resources. An agent might perform Combining. The primitives describe what the subsystem *does*; the archetype describes what role it *plays*.

See [process-primitives.md](process-primitives.md) for the full primitive reference.

---

## 7. Examples Across the Hierarchy

### Physical System: Solar Panel

The photovoltaic solar panel (complexity 15.6) operates at the physical level. Its subsystems are engineered components with no adaptive behavior.

- **Archetype hierarchy level**: Physical (Level 0)
- **Silicon Cell Array**: `archetype: "Unspecified"` — passive energy conversion, no decisions
- **Bypass Diode System**: `archetype: "Governance"` — regulatory protection, but purely reactive
- **Power Conditioning Unit**: `archetype: "Unspecified"` — if modeling basic DC conversion; `archetype: "Agent", kind: "Reactive"` if modeling MPPT adaptive optimization
- **Thermal Management**: `archetype: "Unspecified"` — passive cooling, no decision-making

### Biological System: Cell

The cell (complexity 16.2) operates at the biological level. Self-maintaining with feedback control but no intentional goal-setting.

- **Archetype hierarchy level**: Biological (Level 2)
- **Nucleus**: `archetype: "Governance"` — central regulatory control, receives feedback from all subsystems
- **Mitochondria**: `archetype: "Economy"` — ATP production, resource transformation
- **Endoplasmic Reticulum**: `archetype: "Economy"` — protein manufacturing
- **Golgi Apparatus**: `archetype: "Economy"` — packaging and distribution
- **Peroxisomes**: `archetype: "Economy"` — waste processing

No subsystem requires `archetype: "Agent"` because no subsystem exercises autonomous decision-making independent of the nuclear governance system. If you decomposed the nucleus further and modeled individual transcription factors, *those* might warrant agent classification.

### Social System: Organization

The organization (complexity 21.9) operates at the social level. Communication, shared meaning, and institutional structure enable coordination.

- **Archetype hierarchy level**: Social (Level 3)
- **Executive Leadership**: `archetype: "Governance"` — strategic control, resource allocation directives
- **Sales & Customer Success**: `archetype: "Agent", kind: "Anticipatory", agency_capacity: 0.6` — predicts market conditions, exercises significant autonomy
- **Human Resources**: `archetype: "Agent", kind: "Anticipatory", agency_capacity: 0.5` — predicts workforce needs, moderate autonomy
- **Finance & Accounting**: `archetype: "Economy"` — manages financial flows, reporting
- **Innovation & Strategy**: `archetype: "Agent", kind: "Intentional", agency_capacity: 0.7` — sets research goals, plans multi-step initiatives
- **Operations & Administration**: `archetype: "Economy"` — administrative process management

### Cognitive System: Autonomous Trading Agent

A trading agent operating at the cognitive level. Self-reflective, goal-setting, planning.

- **Archetype hierarchy level**: Cognitive (Level 4)
- **Market Sensor**: `archetype: "Agent", kind: "Reactive", agency_capacity: 0.2` — senses price signals, low autonomy
- **Prediction Engine**: `archetype: "Agent", kind: "Anticipatory", agency_capacity: 0.7` — forecasts market states, high autonomy in prediction
- **Strategy Planner**: `archetype: "Agent", kind: "Intentional", agency_capacity: 0.9` — generates and evaluates trading plans
- **Risk Governor**: `archetype: "Governance"` — constrains agent behavior within risk parameters
- **Portfolio Manager**: `archetype: "Economy"` — manages asset allocation flows

---

## 8. Data Invariants

The following invariants must hold for archetype-related fields:

1. **Agent implies archetype**: `agent.is_some()` requires `archetype == "Agent"`
2. **Archetype lifecycle**: Changing archetype away from Agent clears the `agent` field. Changing archetype to Agent creates a default `AgentModel` (kind: Reactive, agency_capacity: 0.5).
3. **agency_capacity range**: Clamped to [0.0, 1.0]
4. **Backward compatibility**: Missing `agent` field deserializes as `None`. Missing `agency_capacity` defaults to 0.5.

See [bert-schema-reference.md](bert-schema-reference.md) for the full JSON schema and [features/unified-agent-system.md](features/unified-agent-system.md) for the implementation specification.

---

## Formal Foundations

- **8-tuple system definition**: [mobus-reference.md](mobus-reference.md)
- **System Language specification**: [system-language-spec.md](system-language-spec.md)
- **JSON schema**: [bert-schema-reference.md](bert-schema-reference.md)
- **Agent system feature spec**: [features/unified-agent-system.md](features/unified-agent-system.md)
- **Process primitives**: [process-primitives.md](process-primitives.md)
- **Mobus source**: *Systems Science: Theory, Analysis, Modeling, and Design* (2022), Ch. 3, 7, 9, 10, 11, 12
