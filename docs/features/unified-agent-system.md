# Unified Feature: Agent System (Reconciled Design)

## Overview

**Feature Name**: Agent System (unified from agent-params-v2 + agent-properties)
**Target Branch**: `feature/unified-agent-system`
**Status**: Phases 1–3 Complete (data model + agency capacity UI + AgentKind), merged to main
**Phase 1 Commit**: `ddec8b2`
**Source Branches** (archived — see `docs/features/archive/`):
- `feature/agent-params-v2` — Backend data model (implemented, Bevy 0.15 / Leptos 0.7)
- `feature/agent-properties` — HCGS archetype + agency capacity UI (partially implemented, Bevy 0.15 / Leptos 0.7)
- `main` — Current baseline (Bevy 0.17 / Leptos 0.8, already has HcgsArchetype)

## Vision & Theoretical Grounding

### User Vision

"Add a consciousness slider for parts. Clearer outputs and possibly a self-actualization score. Feedback from self-actualization back into the system. Treat BERT as a practice mirror, not just a diagram."

Reframed as **Agency Capacity** to align with systems science terminology and avoid the philosophical baggage of "consciousness." The goal: give agent subsystems a continuous measure of autonomous decision-making capability that is both theoretically grounded and practically useful for simulation export.

### Mobus Foundation (Ch. 11: Decision Agents)

Mobus defines decision agents as having three core components:
1. **Computational Engine** — processes inputs and generates candidate actions
2. **Decision Model** — evaluates candidates against goals/criteria
3. **Experiential Memory** — stores past outcomes to improve future decisions

The agent hierarchy classifies agents by sophistication:

| Level | AgentKind | Capability | Example |
|-------|-----------|------------|---------|
| 1 | **Reactive** | Stimulus-response only, no internal model | Thermostat, simple validator |
| 2 | **Anticipatory** | Predicts future states, adapts behavior | Fee-optimizing miner, market maker |
| 3 | **Intentional** | Sets own goals, plans multi-step actions | Autonomous governance agent |

The 9 atomic **process primitives** from General Systems Theory map to `ProcessPrimitive`:
- **Material**: Combining, Splitting, Buffering
- **Energetic**: Impeding, Propelling
- **Informational**: Copying, Sensing, Modulating, Inverting

The **HCGS classification** (Human-Centered General Systems) maps to `HcgsArchetype` (already on main):
- **Governance** — regulatory/control subsystems
- **Economy** — resource flow/production subsystems
- **Agent** — autonomous decision-making subsystems

### How They Connect

`agency_capacity` is the **continuous dimension** within Mobus's discrete hierarchy. A Reactive agent can have low or moderate autonomy; an Intentional agent can range from semi- to fully autonomous. The `AgentKind` classifies *what type of decision-making*, `agency_capacity` measures *how much autonomy*.

Together with `HcgsArchetype`, this gives three complementary classification axes:
- `HcgsArchetype` → *what role* in the system (Governance / Economy / Agent)
- `AgentKind` → *what type* of agent (Reactive / Anticipatory / Intentional)
- `agency_capacity` → *how much* autonomous decision-making (0.0–1.0)

## Critical Context: Framework Version Gap

Both feature branches target **Bevy 0.15 / Leptos 0.7**. Main has been upgraded to **Bevy 0.17 / Leptos 0.8**. This means:
- Code cannot be cherry-picked directly from either branch
- Data model structs (serde types) port cleanly -- they have no Bevy/Leptos API dependencies
- ECS components need adaptation for Bevy 0.17 API changes
- UI code (Leptos) needs signal API migration (0.7 -> 0.8)
- `HcgsArchetype` and `archetype` field are **already on main** -- no porting needed for that

| Dependency | Feature branches | Main |
|---|---|---|
| Bevy | 0.15 | 0.17 |
| Leptos | 0.7 | 0.8 |
| bevy_prototype_lyon | 0.13 | 0.15 |
| bevy-inspector-egui | 0.29 | 0.35 |

## What Already Exists on Main

Main already has (from properties branch or independent work):
- `HcgsArchetype` enum (Unspecified/Governance/Economy/Agent) with stroke colors
- `archetype: HcgsArchetype` on ECS `System` component
- `archetype: Option<HcgsArchetype>` on serialization `System` struct
- Save/load for archetype (Unspecified serializes as None)
- All Bevy 0.17 / Leptos 0.8 APIs

Main does NOT have:
- `AgentModel` or any agent data model structs
- `agent` field on either System struct
- Agency capacity slider or stroke modulation
- Any ABM export capabilities

## Contradiction Analysis

### 1. AgentKind vs agency_capacity -- COMPLEMENTARY

| Concept | Source | Type | Semantics |
|---------|--------|------|-----------|
| `AgentKind` | v2 | Enum (Reactive/Anticipatory/Intentional) | Mobus categorical agent hierarchy |
| `agency_capacity` | properties | f32 (0.0-1.0) | Continuous autonomy scalar |

**Resolution**: Both kept. `AgentKind` is the discrete Mobus classification. `agency_capacity` is a continuous property within that classification. A Reactive agent might have agency_capacity 0.2, an Intentional agent might have 0.9 -- they're independent dimensions. Both go on `AgentModel`.

### 2. System.agent vs System.archetype -- COMPLEMENTARY

| Field | Source | Scope | On Main? |
|-------|--------|-------|----------|
| `archetype: Option<HcgsArchetype>` | properties | All subsystems | YES |
| `agent: Option<AgentModel>` | v2 | Agent subsystems only | NO |

**Resolution**: Both kept. `archetype` classifies ALL subsystems (Governance/Economy/Agent). `agent` holds detailed agent model data, only populated when archetype == Agent.

**Invariant**: `agent.is_some()` implies `archetype == Some(Agent)`. Changing archetype away from Agent clears the agent field.

### 3. HashMap import -- TRIVIAL

v2 uses `std::collections::HashMap` (for cognitive_params, initial_state). Main doesn't import it. Add it back when adding AgentModel.

### 4. ECS System component field -- STRUCTURAL

v2 stores `agent: Option<AgentModel>` directly on the ECS System component. This is a heavyweight approach (the full data model struct lives in ECS).

**Recommendation**: Keep this for Phase 1 (simplest porting path). In Phase 2, consider splitting into a separate `AgentProperties` ECS component for the UI-editable fields (agency_capacity) and keeping the full AgentModel only in the serialization layer.

### 5. No actual code conflicts

The properties branch's main code contribution (HcgsArchetype) is already on main. The v2 branch's code (AgentModel) doesn't exist on main at all. There are zero merge conflicts -- this is purely additive work.

## Unified Data Model

### Serialization struct: System (data_model/mod.rs)

Add to the existing main System struct:

```rust
pub struct System {
    // ... existing fields (info, sources, sinks, parent, complexity, boundary,
    //     radius, transform, equivalence, history, transformation,
    //     member_autonomy, time_constant, archetype) ...

    /// Agent configuration — only present when archetype == Agent.
    /// Contains behavioral parameters for ABM export and agency properties.
    /// PORT FROM: feature/agent-params-v2, EXTENDED with agency_capacity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentModel>,
}
```

### AgentModel (from v2, extended)

Port from v2 and add `agency_capacity`:

```rust
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Reflect)]
pub struct AgentModel {
    /// Mobus agent hierarchy classification (Reactive/Anticipatory/Intentional)
    pub kind: AgentKind,

    /// Degree of autonomous decision-making capability (0.0 to 1.0)
    /// 0.0 = fully reactive/directed, 0.5 = semi-autonomous, 1.0 = fully autonomous
    /// FROM: feature/agent-properties (was AgentPropertiesData.agency_capacity)
    #[serde(default = "default_agency_capacity")]
    pub agency_capacity: f32,

    /// Atomic work processes this agent can perform (Mobus primitives)
    #[serde(default)]
    pub primitives: Vec<ProcessPrimitive>,

    /// Domain-agnostic cognitive parameters (e.g., "fee_threshold": 50.0)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub cognitive_params: HashMap<String, f64>,

    /// Process behavior configurations with flexible parameters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process_configs: Vec<ProcessAssignment>,

    /// Initial state for agent instantiation as arbitrary JSON values
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[reflect(ignore)]
    pub initial_state: HashMap<String, serde_json::Value>,

    /// Optional network behavior configuration for multi-agent interactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_config: Option<NetworkConfig>,
}

fn default_agency_capacity() -> f32 { 0.5 }
```

### Supporting types (port from v2, unchanged)

```rust
// All from feature/agent-params-v2, no changes needed

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq, Eq, Reflect)]
#[serde(rename_all = "PascalCase")]
pub enum AgentKind {
    #[default]
    Reactive,
    Anticipatory,
    Intentional,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[serde(rename_all = "PascalCase")]
pub enum ProcessPrimitive {
    Combining, Splitting, Buffering, Impeding, Propelling,
    Copying, Sensing, Modulating, Inverting,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct ProcessAssignment {
    pub name: String,
    #[serde(default)]
    #[reflect(ignore)]
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct NetworkConfig {
    pub topology: String,
    #[serde(default)]
    #[reflect(ignore)]
    pub connection_params: HashMap<String, serde_json::Value>,
    #[serde(default)]
    #[reflect(ignore)]
    pub interaction_rules: HashMap<String, serde_json::Value>,
}
```

### ECS System component (system_elements.rs)

Add `agent` field to the existing main component:

```rust
pub struct System {
    // ... existing fields (radius, complexity, membership, equivalence,
    //     transformation, history, boundary, time_unit, archetype) ...

    /// Optional agent configuration for agent-based modeling.
    /// Only populated when archetype == Agent.
    pub agent: Option<crate::bevy_app::data_model::AgentModel>,
}
```

### SystemBundle::new (bundles/mod.rs)

The signature needs to accept both `archetype` (already there) AND `agent`:

```rust
pub fn new(
    // ... existing params ...
    equivalence: &str,
    time_unit: &str,
    archetype: HcgsArchetype,          // already on main
    agent: Option<AgentModel>,          // NEW: add this parameter
) -> Self {
    // ...
    Self {
        system: System {
            // ... existing fields ...
            archetype,
            agent,    // NEW
        },
        // ...
    }
}
```

## Data Invariants

The following invariants must hold at all times:

1. **Agent implies archetype**: `agent.is_some()` → `archetype == Some(HcgsArchetype::Agent)`
2. **Lifecycle: Agent → other**: Changing archetype away from Agent clears the `agent` field (set to `None`)
3. **Lifecycle: other → Agent**: Changing archetype to Agent creates a default `AgentModel` (kind: Reactive, agency_capacity: 0.5, empty collections)
4. **Agency capacity range**: `agency_capacity` is clamped to `[0.0, 1.0]`
5. **Backward compatibility**: Missing `agent` field deserializes as `None`; missing `agency_capacity` defaults to 0.5

These invariants are enforced at the UI layer (archetype change handlers) and validated in tests (serialization round-trip).

## UI Design

### Phase 1 UI: Agency Capacity Slider

From properties spec, implemented with Leptos 0.8 signals:
- Appears in SubSystem details panel only when `archetype == Agent`
- Slider range: 0.0 to 1.0, default 0.5
- Semantic labels: "Reactive" (low end), "Semi-autonomous" (mid), "Fully autonomous" (high end)
- Modifies `system.agent.agency_capacity` on the ECS component

### Phase 1 UI: Stroke Modulation

From properties spec:
- Agent-archetype subsystems modulate stroke intensity by agency_capacity
- Formula: base_alpha = 0.5 + agency_capacity (range 0.5 to 1.5)
- Builds on existing HcgsArchetype::Agent orange (#F97316) stroke color

### Phase 2 UI: Agent Configuration Panel

- AgentKind selector dropdown (Reactive/Anticipatory/Intentional)
- Cognitive params key-value editor
- Process configs structured editor
- Network config editor

## Save/Load Changes

### save.rs

In the System construction (where `archetype` is already serialized), add:

```rust
let root_system = crate::bevy_app::data_model::System {
    // ... existing fields ...
    archetype,                          // already there
    agent: system.agent.clone(),        // NEW: port from v2
};
```

### load.rs

When spawning loaded subsystems, pass agent through:

```rust
// For root system spawn
system.agent.clone(),  // NEW parameter to spawn function

// For subsystem spawn (in SystemBundle::new call)
system.archetype.unwrap_or_default(),  // already there
system.agent.clone(),                   // NEW
```

## JSON Example (Unified)

```json
{
  "info": { "id": "C0.1", "level": 1, "name": "Validator Node" },
  "sources": [],
  "sinks": [],
  "parent": "C0",
  "complexity": { "type": "Atomic" },
  "boundary": {
    "info": { "id": "C0.1B", "level": 1, "name": "Validator Boundary" },
    "porosity": 0.5,
    "perceptive_fuzziness": 0.3,
    "interfaces": []
  },
  "radius": 50,
  "archetype": "Agent",
  "agent": {
    "kind": "Anticipatory",
    "agency_capacity": 0.8,
    "primitives": ["Sensing", "Modulating", "Propelling"],
    "cognitive_params": {
      "base_hashrate": 500.0,
      "fee_threshold": 50.0
    },
    "process_configs": [
      {
        "name": "fee_optimization",
        "params": { "lookback_window": 10 }
      }
    ],
    "initial_state": {
      "wallet": 0,
      "blocks_mined": 0
    }
  }
}
```

Backward-compatible examples (all valid):

```json
// Old model (no archetype, no agent) -- still loads fine
{ "info": {...}, "radius": 50, ... }

// Governance subsystem (archetype only, no agent)
{ "info": {...}, "archetype": "Governance", "radius": 50, ... }

// Agent with minimal config
{ "info": {...}, "archetype": "Agent", "agent": { "kind": "Reactive", "agency_capacity": 0.3 }, "radius": 50, ... }
```

## Simulation Integration

BERT produces the model; external and integrated simulations consume it. The JSON schema is the contract.

### Three Integration Paths

| Path | Project | How it uses BERT JSON | Status |
|------|---------|----------------------|--------|
| **External (Python/Mesa)** | `bitcoin-abm` (v1) | `bert_mesa_loader.py` reads `agent.cognitive_params`, `agent.kind`, `agent.primitives` | Working — reads v2 schema |
| **External (React/Flask)** | `bitcoin-abm-v2` | `bert_loader.py` maps `archetype` → behavior profiles, reads `complexity` | Working — reads archetype schema |
| **Integrated (Rust/Bevy)** | `feature/agent-dynamics` | 5600+ lines: substance processing, temporal sim, process primitive execution | Experimental — separate branch |

### External Path: Python/Mesa (bitcoin-abm v1)

The `bert_mesa_loader.py` already reads the `AgentModel` schema:
- `agent.cognitive_params` → maps to Mesa agent attributes (e.g., `fee_threshold`, `base_hashrate`)
- `agent.kind` → determines agent behavior class (Reactive/Anticipatory/Intentional)
- `agent.primitives` → declares which process primitives the agent can execute
- `agent.process_configs` → parameterizes specific process behaviors
- `agent.initial_state` → sets initial agent state for simulation start

The `HashMap<String, f64>` for cognitive_params and `Vec<ProcessAssignment>` with flexible params are exactly what the Mesa loader consumes. No schema changes needed.

### External Path: React/Flask (bitcoin-abm v2)

The `bert_loader.py` uses a higher-level mapping:
- `archetype` → maps to behavior profiles (Agent archetype triggers agent behavior)
- `complexity` → determines interaction depth
- Does not (yet) read the detailed `agent` field — future enhancement

### Integrated Path: Rust/Bevy (agent-dynamics)

The `feature/agent-dynamics` branch contains an ambitious integrated simulation engine:
- Substance buffers for material/energy/information flow
- Temporal simulation with configurable time steps
- Process primitive execution (the 9 Mobus primitives running inside Bevy)
- Simulation engine specification

This is a **separate future effort**. The current `AgentModel` data model does not block it — the `ProcessPrimitive` enum maps 1:1 to what agent-dynamics expects, and `cognitive_params` provides the parameterization layer.

### Key Principle

BERT is a **modeling tool**, not a simulation runtime. The JSON schema defines what an agent *is* (structure, parameters, capabilities). External frameworks define what an agent *does* (behavior, interaction, dynamics). The integrated path blurs this boundary intentionally, but the data model serves both use cases.

## Testing Strategy

1. **Backward compatibility**: Load models without `agent` field -- defaults to None
2. **Archetype-only**: Models with `archetype` but no `agent` -- valid for Governance/Economy
3. **Agent models**: v2-style JSON with `agent` but no `archetype` -- loads, agent preserved
4. **Unified models**: Both `archetype: "Agent"` and `agent: {...}` -- full feature
5. **Serialization round-trip**: Save and reload preserves all agent fields
6. **Empty collections**: Empty cognitive_params, process_configs, initial_state -- no errors
7. **UI**: Slider appears only for Agent archetype, updates agency_capacity
8. **Stroke**: Visual modulation changes with agency_capacity value

## Recommended Implementation Sequence

### Phase 1: Data Model — COMPLETE (`ddec8b2`)

1. ~~Create `feature/unified-agent-system` from `main`~~
2. ~~Add `use std::collections::HashMap` to `data_model/mod.rs`~~
3. ~~Port `AgentModel`, `AgentKind`, `ProcessPrimitive`, `ProcessAssignment`, `NetworkConfig` structs from v2 to `data_model/mod.rs`~~
4. ~~Add `agency_capacity: f32` field to `AgentModel` with default 0.5~~
5. ~~Add `agent: Option<AgentModel>` field to serialization `System` struct~~
6. ~~Add `agent: Option<AgentModel>` field to ECS `System` component~~
7. ~~Update `SystemBundle::new` to accept and store `agent`~~
8. ~~Update `save.rs` to serialize `system.agent.clone()`~~
9. ~~Update `load.rs` to pass `system.agent.clone()` through spawn paths~~
10. ~~Update all `SystemBundle::new` call sites to pass `None` for agent (or loaded value)~~
11. ~~Run `cargo test --all` and `cargo clippy`~~

All 99 lines of data model code passing tests.

### Phase 2: Agency Capacity UI — COMPLETE (`d8ccf83`–`0144e37`)

- ~~Add Agency Capacity slider to Leptos details panel (Leptos 0.8 signals)~~
  - ~~Slider range 0.0–1.0, step 0.01, default 0.5~~
- ~~Show slider only when `archetype == HcgsArchetype::Agent`~~
- ~~Wire slider to `system.agent.agency_capacity` (create default AgentModel if needed)~~
- ~~Add stroke intensity modulation by agency_capacity (stroke width: 0.5×–1.5× base)~~
- ~~Handle archetype change lifecycle (Agent→other clears agent, other→Agent creates default)~~
- ~~Serialization round-trip tests (4 tests, all passing)~~

**Known limitation — Slider filler on model load (deferred to Phase 3)**

The slider's red filler gradient (`style:background`) is driven by `internal_value`
(a local `RwSignal<f64>` set synchronously on every `on_input` event) rather than
the external `value: Signal<f64>` that tracks `sub_system_query → agency_capacity`.

**Why**: `value.get()` inside a `style:background` closure on a Slider mounted inside
`<Show>` failed to reactively update on `sub_system_query` writes during dragging —
root cause unresolved after multiple attempts (Effect sync, Memo::new, Show condition
refactor). `internal_value` is the source that provably works (same as `prop:value` on
the number input).

**Effect**: If a model is loaded from file while the Agent details panel is already
open, the filler position won't update to reflect the loaded `agency_capacity`. The
knob (`prop:value=value`) and number input will correctly show the loaded value; only
the filler gradient lags until the user next interacts with the slider.

**Fix path**: Investigate whether `RwSignalSynced::write()` propagates reactivity
synchronously or requires a separate read to trigger re-evaluation. Compare B2L update
path for `porosity` (which works) vs `agency_capacity` (which doesn't) — specifically
whether the difference is the `<Show>` boundary or the nested `system.agent.?` access.
An `Effect::new(move |_| { internal_value.set(value.get()); })` would fix the sync
but previously caused interactivity regressions — needs further isolation.

### Phase 3: AgentKind Classification — COMPLETE (`64f07f8`)

- ~~`AgentKind` RadioGroup (Reactive / Anticipatory / Intentional)~~
- ~~Soft-resets `agency_capacity` to kind default on selection (0.25 / 0.50 / 0.75)~~
- ~~Per-option tooltips referencing Mobus decision agent hierarchy levels~~

**Design decision**: AgentKind and agency_capacity are complementary, not redundant.
Kind = decision architecture tier (structural); capacity = degree of autonomy exercised
(continuous). Selecting a kind soft-defaults the slider but both remain independently editable.

**Deferred to Phase 4:**
- Visual differentiation by AgentKind (color/shape) — deferred pending real-model usage feedback
- Full `cognitive_params` key-value editor
- Process primitive checkboxes (9 Mobus primitives)
- Process configs structured editor
- Self-actualization score (computed from agent configuration completeness)
- Feedback loop visualization (agency → system behavior → agency feedback)

### Phase 4: Full Agent Configuration Panel + Simulation Prep (future)

- AgentKind visual encoding (color modulation within Agent orange, or shape — shape deferred, complex)
- Full `cognitive_params` key-value editor (add/remove/edit f64 parameters)
- Process primitive checkboxes (select from 9 Mobus primitives)
- Process configs structured editor (name + JSON params)
- Network config editor (topology, connection_params, interaction_rules)
- Self-actualization score (computed metric from agent configuration completeness)
- Feedback loop visualization (agency → system behavior → agency feedback)

### Phase 5: Integrated Simulation Execution (future)

- Draw from `feature/agent-dynamics` branch vision (5600+ lines)
- Substance buffer processing within Bevy ECS
- Temporal simulation with configurable time steps
- Process primitive execution engine
- Simulation playback and state inspection
- **Note**: This is a separate feature effort; the data model from Phase 1 already supports it

## Files to Modify (Phase 1)

| File | Change |
|------|--------|
| `src/bevy_app/data_model/mod.rs` | Add AgentModel + supporting types, add `agent` field to System |
| `src/bevy_app/components/system_elements.rs` | Add `agent` field to ECS System component |
| `src/bevy_app/bundles/mod.rs` | Add `agent` param to SystemBundle::new |
| `src/bevy_app/bundles/spawn/subsystem.rs` | Pass agent through spawn_subsystem_common |
| `src/bevy_app/data_model/save.rs` | Serialize agent field |
| `src/bevy_app/data_model/load.rs` | Deserialize and pass agent through spawn |
| `docs/features/agent-system.md` | This unified spec |

## Relationship to Mobus Theory

The unified design preserves both branches' Mobus grounding:

- **v2's contribution**: Process primitives (9 atomic work processes from GST), AgentKind hierarchy (Reactive -> Anticipatory -> Intentional), flexible cognitive params for ABM export
- **properties' contribution**: HCGS archetype classification (Governance/Economy/Agent), agency_capacity as continuous decision-agent property (Ch. 11), visual feedback through stroke modulation
- **Unified**: A subsystem classified as `HcgsArchetype::Agent` has an `AgentModel` that specifies its `AgentKind` (categorical autonomy level), `agency_capacity` (continuous autonomy measure), process primitives, and domain-specific parameters for simulation export

---

*Unified spec finalized 2026-02-07. Updated 2026-02-07 with Vision, Simulation Integration, Data Invariants.*
*Sources: feature/agent-params-v2 spec + code, feature/agent-properties spec + code, main branch analysis.*
*Archived specs: `docs/features/archive/agent-parameters-v2.md`, `docs/features/archive/agent-properties.md`*
