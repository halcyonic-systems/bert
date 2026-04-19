# BERT JSON Schema Reference

**Canonical schema reference for BERT model files.** Single source of truth for the BERT JSON format.

## Purpose

This document specifies the exact JSON structure that BERT deserializes. It is derived from the Rust data model at `src/bevy_app/data_model/mod.rs` (the canonical source). Use this reference when:
- Generating BERT models from LLM prompts or external tools
- Writing validators that check model integrity
- Integrating BERT with other systems (TypeDB, Mesa, Facets)
- Authoring models by hand

**For workflow guidance** (how to decompose systems, when to use which archetype, Mobus methodology), see the `bert-json-creation` and `bert` skills.

**For the formal System Language specification** (theoretical grounding, Lean constraints, Mobus mapping), see `system-language-spec.md`.

## Root Structure

```json
{
  "version": 1,
  "environment": Environment,
  "systems": [System, ...],
  "interactions": [Interaction, ...],
  "hidden_entities": []
}
```

### WorldModel

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `version` | `u32` | yes | — | Must be `1`. Triggers migration on mismatch. |
| `environment` | `Environment` | yes | — | Root environmental context. |
| `systems` | `Vec<System>` | yes | — | Flat list; hierarchy via `parent` IDs. |
| `interactions` | `Vec<Interaction>` | yes | — | All flows between entities. |
| `hidden_entities` | `Vec<Id>` | no | `[]` | Visualization state. |

## ID System

IDs are strings combining a type prefix with dot-separated indices: `<prefix><i1>.<i2>...`

| Entity Type | Prefix | Example | Indices | Level |
|---|---|---|---|---|
| Environment | `E` | `E-1` | `[-1]` | -1 |
| Source | `Src` | `Src-1.0` | `[-1, 0]` | -1 |
| Sink | `Snk` | `Snk-1.0` | `[-1, 0]` | -1 |
| Root System | `S` | `S0` | `[0]` | 0 |
| Subsystem | `C` | `C0.1` | `[0, 1]` | 1 |
| Deep Subsystem | `C` | `C0.1.2` | `[0, 1, 2]` | 2 |
| Interface (on S0) | `I` | `I0.0` | `[0, 0]` | 1 |
| Interface (on C0.1) | `I` | `I0.1.0` | `[0, 1, 0]` | 2 |
| Boundary | `B` | `B0.1` | `[0, 1]` | mirrors parent system |
| Flow (env) | `F` | `F-1.0` | `[-1, 0]` | -1 |
| Flow (internal) | `F` | `F0.0` | `[0, 0]` | level of connected entities |

### Level Calculation

- **Normal entities**: `level = len(indices) - 1`
- **Environment and children**: `level = -1`
- **Interfaces**: `level = len(indices) - 1` (matches subsystems at same depth)
- **Flows**: `level` matches the level of the entities being connected

### Assignment Rules

1. Root system is `S0` with `parent: "E-1"`
2. Subsystems of `S0` are `C0.0`, `C0.1`, ... (parent indices + child index)
3. Interfaces belong to their parent system's ID space: `S0` has `I0.0`, `I0.1`; `C0.1` has `I0.1.0`, `I0.1.1`
4. Boundaries mirror their system: `S0` has `B0`, `C0.1` has `B0.1`
5. Environment entities: `Src-1.0`, `Snk-1.0`, `F-1.0`, ...

## Entity Types

### Environment

Root environmental context. Exactly one per model.

| Field | Type | Required | Notes |
|---|---|---|---|
| `info` | `Info` | yes | ID must be `E-1` |
| `sources` | `Vec<ExternalEntity>` | yes | External inputs to the root system |
| `sinks` | `Vec<ExternalEntity>` | yes | External outputs from the root system |

### Info

Metadata block used by most entities.

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | `Id` (string) | yes | Format: `<prefix><i1>.<i2>...` |
| `level` | `i32` | yes | Must match `len(indices) - 1` (except environment = -1) |
| `name` | `String` | yes | Human-readable name |
| `description` | `String` | yes | Description (empty string OK) |

### ExternalEntity (Source / Sink)

Entities in the environment that participate in flows across the system boundary.

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `info` | `Info` | yes | — | ID prefix `Src` or `Snk` |
| `type` | `"Source"` \| `"Sink"` | yes | — | Must match role |
| `transform` | `Option<Transform2d>` | no | `null` | Visual position |
| `equivalence` | `String` | **yes** | `""` | Cross-model entity mapping (empty string OK) |
| `model` | `String` | **yes** | `""` | Reference to external model file (empty string OK) |
| `is_same_as_id` | `Option<usize>` | no | `null` | Links duplicate entities across models |

**⚠ Crash-causing fields**: `equivalence`, `model`, `is_same_as_id` must be present. BERT freezes on load if any are missing.

### System

The primary compositional unit. Used for both root system and all subsystems.

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `info` | `Info` | yes | — | ID prefix `S` (root) or `C` (subsystem) |
| `sources` | `Vec<ExternalEntity>` | **yes** | `[]` | Internal sources (usually empty) |
| `sinks` | `Vec<ExternalEntity>` | **yes** | `[]` | Internal sinks (usually empty) |
| `parent` | `Id` | yes | — | `E-1` for root, `S0` or `C0.x` for subsystems |
| `complexity` | `Complexity` | yes | — | See Complexity section |
| `boundary` | `Boundary` | yes | — | System boundary |
| `radius` | `f32` | **yes** | — | 300 for root, 42 for subsystems, 12 for interface processors |
| `transform` | `Option<Transform2d>` | no | `null` | Visual position |
| `equivalence` | `String` | **yes** | `""` | Cross-model identity tag |
| `history` | `String` | **yes** | `""` | Historical notes |
| `transformation` | `String` | **yes** | `""` | Transformation description |
| `member_autonomy` | `f32` | **yes** | `1.0` | Mobus membership function (0.0–1.0) |
| `time_constant` | `String` | **yes** | `"Second"` | Characteristic timescale |
| `archetype` | `Option<HcgsArchetype>` | no | `null` | Omit for unspecified |
| `agent` | `Option<AgentModel>` | no | `null` | Only when `archetype == "Agent"` |

**⚠ Crash-causing fields**: `sources`, `sinks`, `equivalence`, `history`, `transformation`, `member_autonomy`, `time_constant`, `radius` must all be present. BERT freezes on load if any are missing.

**Invariant**: `agent.is_some()` implies `archetype == "Agent"`.

### Boundary

Defines the formal separation between system and environment.

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `info` | `Info` | yes | — | ID prefix `B` |
| `porosity` | `f32` | **yes** | `0.0` | Boundary permeability (0.0 = impermeable) |
| `perceptive_fuzziness` | `f32` | **yes** | `0.0` | Boundary clarity (0.0 = precise) |
| `interfaces` | `Vec<Interface>` | yes | `[]` | Usually empty for subsystems |
| `parent_interface` | `Option<Id>` | yes | `null` | Set only for interface processor subsystems |

**⚠ Crash-causing fields**: `porosity` and `perceptive_fuzziness` must be present.

### Interface

Connection points where flows cross the system boundary.

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `info` | `Info` | yes | — | ID prefix `I` |
| `protocol` | `String` | **yes** | `""` | Communication protocol (empty string OK) |
| `type` | `"Import"` \| `"Export"` \| `"Hybrid"` | yes | — | Flow direction |
| `exports_to` | `Vec<Id>` | **yes** | `[]` | Target IDs (for Export interfaces) |
| `receives_from` | `Vec<Id>` | **yes** | `[]` | Source IDs (for Import interfaces) |
| `angle` | `Option<f32>` | no | `null` | Rotation in radians on boundary |

**⚠ Crash-causing fields**: `protocol`, `exports_to`, `receives_from` must all be present (even as empty strings/arrays).

**Note on `Hybrid`**: Not implemented in BERT runtime. Use Import + Export pair linked by `is_same_as_id` instead.

### Interaction (Flow / Force)

Connections between entities carrying matter, energy, information, or influence.

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `info` | `Info` | yes | — | ID prefix `F` |
| `substance` | `Substance` | yes | — | What flows |
| `type` | `"Flow"` \| `"Force"` | yes | — | Material transfer vs. influence-without-transfer |
| `usability` | `"Resource"` \| `"Product"` \| `"Waste"` \| `"Disruption"` | yes | — | See Usability section |
| `source` | `Id` | yes | — | Origin entity ID |
| `source_interface` | `Option<Id>` | no | `null` | Required only when source is a system with explicit interface routing |
| `sink` | `Id` | yes | — | Destination entity ID |
| `sink_interface` | `Option<Id>` | no | `null` | Required only when sink is a system with explicit interface routing |
| `amount` | `Decimal` | **yes** | `"1"` | Flow quantity (serialized as string) |
| `unit` | `String` | **yes** | `""` | Measurement unit (empty string OK) |
| `parameters` | `Vec<Parameter>` | **yes** | `[]` | Additional flow metrics |
| `smart_parameters` | `Vec<SmartParameter>` | no | `[]` | Enhanced typed parameters (skipped if empty) |
| `endpoint_offset` | `Option<EndpointOffset>` | no | `null` | Positions flow arrows on subsystem boundaries |

**⚠ Crash-causing fields**: `amount`, `unit`, `parameters` must all be present.

**Flow vs. Force**: `Flow` represents material/energy/information transfer. `Force` represents influence-without-transfer (governance rules, regulatory constraints, protocol parameters that shape behavior without moving substance).

**Internal flows require `endpoint_offset`**: Flows between subsystems within the same parent (including S0↔subsystem) must use `source_interface: null`, `sink_interface: null`, and `endpoint_offset` with angles. Flows with interface routing are excluded from drag handle spawning.

### Substance

What an interaction carries.

| Field | Type | Required | Notes |
|---|---|---|---|
| `sub_type` | `String` | yes | Domain-specific subtype (e.g., "Electricity", "BTC", "Data") |
| `type` | `"Energy"` \| `"Material"` \| `"Message"` | yes | Canonical category |

### AgentModel

Behavioral model for agent-archetype subsystems. Only populated when `archetype == "Agent"`.

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `kind` | `"Reactive"` \| `"Anticipatory"` \| `"Intentional"` | yes | `"Reactive"` | Mobus agent hierarchy |
| `agency_capacity` | `f32` | no | `0.5` | Autonomy scalar (0.0–1.0) |
| `primitives` | `Vec<ProcessPrimitive>` | no | `[]` | Atomic work processes the agent performs |
| `cognitive_params` | `HashMap<String, f64>` | no | `{}` | Domain-specific numeric parameters (skipped if empty) |
| `process_configs` | `Vec<ProcessAssignment>` | no | `[]` | Process behavior configurations (skipped if empty) |
| `initial_state` | `HashMap<String, Value>` | no | `{}` | Arbitrary initial agent state (skipped if empty) |
| `network_config` | `Option<NetworkConfig>` | no | `null` | Multi-agent network behavior (skipped if null) |

### ProcessAssignment

Named process configuration with flexible parameters.

| Field | Type | Required | Default |
|---|---|---|---|
| `name` | `String` | yes | — |
| `params` | `HashMap<String, Value>` | no | `{}` |

### NetworkConfig

Multi-agent network topology and behavior.

| Field | Type | Required | Default |
|---|---|---|---|
| `topology` | `String` | yes | — |
| `connection_params` | `HashMap<String, Value>` | no | `{}` |
| `interaction_rules` | `HashMap<String, Value>` | no | `{}` |

### Parameter

Simple named parameter with string value and unit.

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `name` | `String` | yes | — | Parameter name |
| `value` | `String` | yes | — | Value as string (accommodates numeric, text, boolean) |
| `unit` | `String` | no | `""` | Unit of measurement |

Note: `id` (UUID) is internal and not serialized.

### SmartParameter

Enhanced parameter with typed value.

| Field | Type | Required | Notes |
|---|---|---|---|
| `name` | `String` | yes | Parameter name |
| `value` | `ParameterValue` | yes | Typed value — see below |

Note: `id` (UUID) is internal and not serialized.

**`ParameterValue`** is an internally-tagged enum with four variants:
- `Numeric { value: String, unit: String }`
- `Ordinal { level: String, options: Vec<String> }`
- `Categorical { value: String, options: Vec<String> }`
- `Boolean { value: bool, true_label: String, false_label: String }`

### EndpointOffset

Angular positioning for flow arrows on subsystem boundaries.

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `start_angle` | `Option<f32>` | no | `null` | Radians from source entity center (individually nullable) |
| `end_angle` | `Option<f32>` | no | `null` | Radians from sink entity center (individually nullable) |

### Transform2d

Visual position and rotation.

| Field | Type | Required | Notes |
|---|---|---|---|
| `translation` | `[f32, f32]` | yes | `[x, y]` in pixels at 100% zoom |
| `rotation` | `f32` | yes | Radians |

### Complexity

System complexity classification. Serialized as an adjacently-tagged enum.

| Variant | JSON | Meaning |
|---|---|---|
| Atomic | `"Atomic"` | No internal structure (indivisible) |
| Complex | `{"Complex": {"adaptable": bool, "evolveable": bool}}` | Has internal components with behavioral properties |
| Multiset | `{"Multiset": N}` | N identical component instances |

## Enum Reference

| Enum | JSON Values |
|---|---|
| `SubstanceType` | `"Energy"`, `"Material"`, `"Message"` |
| `InterfaceType` | `"Import"`, `"Export"`, `"Hybrid"` (Hybrid unimplemented — use Import+Export pair instead) |
| `InteractionType` | `"Flow"`, `"Force"` |
| `InteractionUsability` | `"Resource"`, `"Disruption"`, `"Product"`, `"Waste"` |
| `ExternalEntityType` | `"Source"`, `"Sink"` |
| `HcgsArchetype` | `"Unspecified"`, `"Governance"`, `"Economy"`, `"Agent"` (omit field entirely for unspecified) |
| `AgentKind` | `"Reactive"`, `"Anticipatory"`, `"Intentional"` |
| `ProcessPrimitive` | `"Combining"`, `"Splitting"`, `"Buffering"`, `"Impeding"`, `"Propelling"`, `"Copying"`, `"Sensing"`, `"Modulating"`, `"Inverting"` |

## Validation Rules

### L1 — Structural (JSON shape)

1. Top-level: `version`, `environment`, `systems`, `interactions` present
2. All enum values match canonical strings (see Enum Reference)
3. All `info.level` values match `len(indices) - 1` (except environment entities = -1)
4. `complexity` is one of: `"Atomic"`, `{"Complex": {...}}`, `{"Multiset": N}`

### L2 — Referential integrity (IDs resolve)

5. All `source`, `sink`, `source_interface`, `sink_interface` IDs in interactions resolve to existing entities
6. All `parent` IDs resolve to existing systems (or `E-1` for root)
7. All `exports_to` and `receives_from` IDs on interfaces resolve to existing entities
8. No orphan sources/sinks — every external entity must appear in at least one interaction

### L3 — Required field presence (missing = BERT freezes on load)

9. **System**: `sources`, `sinks` (empty arrays OK), `equivalence`, `history`, `transformation`, `member_autonomy`, `time_constant`, `radius`
10. **Boundary**: `porosity`, `perceptive_fuzziness`
11. **Interface**: `protocol`, `exports_to`, `receives_from`
12. **Interaction**: `amount`, `unit`, `parameters`
13. **ExternalEntity**: `equivalence`, `model`, `is_same_as_id`

### L4 — Semantic consistency (future — OWL ontology conformance)

See issue #40. Will check for:
- Archetype consistency with flow patterns
- Substance conservation across connected interactions
- Interface subset constraint (I ⊆ C) satisfied
- Boundary completeness and shielding properties

## Model Structure Summary

```
Environment (E-1)
├── Sources: Src-1.0, Src-1.1, ...
├── Sinks: Snk-1.0, Snk-1.1, ...
└── Root System (S0)
    ├── Boundary (B0) with Interfaces (I0.0, I0.1, ...)
    ├── Independent Subsystem (C0.0)   — parent_interface: null, radius: 42
    ├── Independent Subsystem (C0.1)   — parent_interface: null, radius: 42
    ├── Interface Processor (C0.50)    — parent_interface: "I0.50", radius: 12
    └── ...

Interactions:
├── External: Src-1.0 → S0 via sink_interface: I0.0
├── External: S0 → Snk-1.0 via source_interface: I0.1
├── Internal: C0.0 → C0.1 via endpoint_offset (null interfaces)
└── Internal: C0.50 → C0.0 via endpoint_offset (interface processor to subsystem)
```

## Spatial Layout Guidelines

These are conventions, not hard constraints — BERT renders whatever valid coordinates you provide. But these conventions produce readable models:

- **Sources**: Left side, x ∈ [-600, -400], spread vertically
- **Sinks**: Right side, x ∈ [400, 600], spread vertically
- **Root system S0**: Centered at `[0, 0]`, radius 300
- **Subsystems**: Inside S0, radius 42, positions in `[-150, 150]` range
- **Interface processors**: On S0 boundary, radius 12
- **Import interface angles**: π to 2π (left half, facing sources)
- **Export interface angles**: 0 to π (right half, facing sinks)

## Troubleshooting

| Symptom | Likely Cause | Fix |
|---|---|---|
| BERT freezes on load | Missing `sources`/`sinks` arrays on systems | Add `"sources": [], "sinks": []` to every system |
| BERT freezes on load | Missing `porosity`/`perceptive_fuzziness` on boundaries | Add `"porosity": 0.0, "perceptive_fuzziness": 0.0` |
| BERT freezes on load | Missing `exports_to`/`receives_from` on interfaces | Add empty arrays or connected entity IDs |
| BERT freezes on load | Missing `amount`/`unit`/`parameters` on flows | Add `"amount": "1", "unit": "", "parameters": []` |
| BERT freezes on load | Missing `equivalence`/`model`/`is_same_as_id` on sources/sinks | Add empty strings and `null` |
| BERT crashes on load | Orphan source/sink with no interaction | Add interaction referencing it, or remove the entity |
| Subsystem stuck at interface | `parent_interface` set to interface ID unexpectedly | Set to `null` for independent subsystems |
| Internal flows don't render | Using `source_interface`/`sink_interface` for internal flows | Use `null` interfaces + `endpoint_offset` instead |
| Subsystems overlapping | Positions too close in `transform.translation` | Spread translations apart |
| Level mismatch error | `info.level` doesn't match ID indices | Set `level` to `len(indices) - 1` |

## Grounding Sources

**Rust data model** — `src/bevy_app/data_model/mod.rs`
: JSON schema definition (1,433 lines, 16 structs, 9 enums). Canonical — what BERT actually deserializes.

**ECS components** — `src/bevy_app/components/system_elements.rs`
: Runtime semantics (1,090 lines). Where `SubstanceType`, `InteractionType`, `HcgsArchetype`, `Parameter` are defined.

**Smart parameters** — `src/bevy_app/smart_parameters.rs`
: `SmartParameter` and `ParameterValue` enum (Numeric/Ordinal/Categorical/Boolean).

**Load logic** — `src/bevy_app/data_model/load.rs`
: ID reconstruction, InterfaceSubsystem detection.

**Save logic** — `src/bevy_app/data_model/save.rs`
: ID generation, serialization.

**System Language Specification** — `docs/system-language-spec.md`
: Formal specification with 11 typed primitives, 4 Lean-verified coherence constraints, execution mapping to Mesa + Bevy ECS.

**OWL/RDF ontology** — `gitbook/for-researchers/bert-systems-ontology.rdf`
: Formal ontology (40 entities, 10 object properties).

**Lean 8-tuple** — `systems-ontology/Systems/Mobus/Tuple.lean`
: Machine-verified coherence constraints.

**Bitcoin model** — `assets/models/examples/bitcoin.json`
: Reference implementation. All four blockchain examples (bitcoin, ethereum, cosmos-hub, solana) share the same 4+3 structural pattern.

**Mobus reference** — `docs/MOBUS_REFERENCE.md`
: DSA quick reference.

---

*Canonical schema reference derived from Rust data model. For questions about evolution or inconsistencies, `mod.rs` is the source of truth.*
