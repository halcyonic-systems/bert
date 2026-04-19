# BERT TypeDB Schema Design

**Status**: Design v0.1 (2026-04-19) — schema not yet deployed
**Purpose**: Mapping from BERT's System Language primitives to TypeDB 3.x entity-relation-attribute (PERA) model. Foundation for the BERT→TypeDB transpiler (issue #37) and all downstream integration (Mesa bridge, cross-model queries, Facets, simulation history).

## Positioning

TypeDB is BERT's **live query layer**, not its storage format. BERT models remain JSON on disk (portable, versionable, offline). The transpiler projects JSON into TypeDB as a typed graph so that:

- Mesa reads structured subgraphs instead of parsing raw JSON
- LLMs get precise typed context instead of whole-file blobs
- Cross-model queries become native TypeQL patterns
- Simulation history accumulates as timestamped graph data

JSON ↔ TypeDB is a sync, not a migration. The design below documents the projection.

## Grounding and Authority

This schema is a computational projection of three authoritative sources:

1. **Rust data model** (`src/bevy_app/data_model/mod.rs`) — what BERT actually deserializes. Always canonical for structure.
2. **System Language Specification** (`docs/system-language-spec.md`) — what the structure *means* ontologically. Controlled vocabularies come from here.
3. **OWL/RDF ontology** (`gitbook/for-researchers/bert-systems-ontology.rdf`) — formal ontological grounding. 40 entities, 10 object properties.

**Drift protocol**: If TypeDB schema disagrees with any of the three, the Rust model is authoritative for structure, the SL spec is authoritative for semantics, the OWL ontology is authoritative for ontological claims. The TypeDB schema must be updated to match.

## Design Principles

**Derived from gov-graphs precedent**, with BERT-specific adjustments:

| Principle | Rationale |
|---|---|
| Flat schema — no `sub entity` hierarchy | System and Subsystem share the same Rust struct; distinguish via `system_level` attribute. Matches gov-graphs' `component` pattern. |
| String attributes with `@values` constraints | BERT has formal controlled vocabularies (SubstanceType, HcgsArchetype, ProcessPrimitive, etc.). `@values` enforces spec compliance at insert time. Stricter than gov-graphs, which has open-ended domains. |
| Relations own their metadata | `interaction` owns substance/usability/type directly (not via separate entities). Mirrors gov-graphs' `oversight` pattern. |
| Multi-model namespacing via ID prefix | `bert_id` values prefixed with `{model_name}:{local_id}`. Enables multiple BERT models in one TypeDB instance. |
| `@key` for unique IDs, bare `owns` for everything else | Standard TypeDB 3.x pattern. |
| One example inference function to establish pattern | Full inference catalog deferred to later issue after real query needs emerge. gov-graphs' three rules emerged from civic analysis use cases — BERT's will emerge similarly. |

**BERT-specific departures from gov-graphs**:

| Departure | Why |
|---|---|
| `composition` + `in_environment` instead of single `containment` | Mobus 8-tuple distinguishes C (composition) from E (environment). Conflating them loses the spec's ontological grounding. |
| `@values` constraints on enum-like attributes | Gov-graphs uses raw strings; BERT has formal controlled vocabularies that should be enforced. |
| No `named_flow` / `flow_step` pattern | BERT interactions are graph edges, not ordered process sequences. Temporal ordering lives in the simulation layer (Mesa), not the type system. |
| Deferred inference catalog | Gov-graphs designed three inference functions upfront. BERT defers — inference patterns should emerge from real cross-model query use cases. |

## SL Primitive → TypeDB Mapping

### Entities (7)

| TypeDB entity | SL primitive | Rust type | Notes |
|---|---|---|---|
| `bert_model` | Model envelope | `WorldModel` | Namespace root — one entity per loaded model |
| `system` | System (root + subsystem) | `System` | Flat — `system_level` distinguishes root (0) from subsystems (1+) |
| `external_entity` | Source / Sink | `ExternalEntity` | Flat — role derived from `participates_in` direction per SL §1.2 |
| `boundary` | Boundary | `Boundary` | One per system |
| `interface` | Interface | `Interface` | Owns protocol + interface_type |
| `interaction` | Flow / Force | `Interaction` | Primary edge — owns all flow/force metadata |
| `agent_model` | AgentModel | `AgentModel` | Only present when parent system has archetype=Agent |

### Relations (7)

| TypeDB relation | Roles | Represents |
|---|---|---|
| `composition` | `whole`, `part` | system → subsystem (Mobus C) |
| `in_environment` | `environment`, `contained_system` | environment → system (Mobus E) |
| `has_boundary` | `enclosed`, `enclosure` | system → boundary |
| `has_interface` | `boundary`, `interface` | boundary → interface |
| `participates_in` | `entity`, `interaction`, `role` | system/external_entity → interaction (replaces source/sink fields; role attribute distinguishes) |
| `routes_through` | `interaction`, `interface`, `endpoint` | interaction → interface (optional; endpoint attribute: "start"/"end") |
| `is_equivalent_to` | `primary`, `equivalent` | external_entity ↔ external_entity (first-class replacement for `is_same_as_id`) |
| `has_agent_config` | `system`, `config` | system → agent_model (Agent archetype only) |
| `has_primitive` | `agent`, `primitive` | agent_model → one process primitive (normalized from `Vec<ProcessPrimitive>`) |
| `has_cognitive_param` | `agent`, `param` | agent_model → one named param (normalized from `HashMap<String, f64>`) |

*Ten relations total — the `has_primitive` and `has_cognitive_param` entries handle the Vec/HashMap fields on AgentModel.*

### Attributes (organized by concern)

**Identification**:
- `bert_id` (`string`, `@key`) — namespaced ID `{model_name}:{local_id}`
- `model_name` (`string`, `@key` on bert_model)
- `system_level` (`integer`) — -1 for environment-scope, 0 for root, 1+ for subsystems

**Metadata**:
- `display_name` (`string`), `description` (`string`)

**System properties**:
- `radius` (`double`), `equivalence_class` (`string`), `history_note` (`string`), `transformation_note` (`string`)
- `member_autonomy` (`double`) — Mobus's m_{i,k,l} membership function
- `time_constant` (`string`, `@values`) — controlled vocab from SL spec (Millisecond → Epoch)
- `archetype` (`string`, `@values`) — HcgsArchetype: "Unspecified", "Governance", "Economy", "Agent"

**Complexity** (flattened from `Complexity` enum):
- `complexity_kind` (`string`, `@values`) — "Atomic", "Complex", "Multiset"
- `complex_adaptable` (`boolean`) — present iff complexity_kind=Complex
- `complex_evolveable` (`boolean`) — present iff complexity_kind=Complex
- `multiset_count` (`integer`) — present iff complexity_kind=Multiset

**Boundary**:
- `porosity` (`double`), `perceptive_fuzziness` (`double`)

**Interface**:
- `protocol` (`string`) — no `@values` (extensible per SL spec §1.5)
- `interface_type` (`string`, `@values`) — "Import", "Export", "Hybrid"
- `interface_angle` (`double`)

**Interaction**:
- `substance_type` (`string`, `@values`) — "Energy", "Material", "Message"
- `substance_sub_type` (`string`) — no `@values` (extensible per SL spec §1.7)
- `interaction_type` (`string`, `@values`) — "Flow", "Force"
- `usability` (`string`, `@values`) — "Resource", "Disruption", "Product", "Waste"
- `amount` (`string`) — Decimal serialized as string (matches Rust)
- `unit` (`string`)

**Agent**:
- `agent_kind` (`string`, `@values`) — "Reactive", "Anticipatory", "Intentional"
- `agency_capacity` (`double`)
- `process_primitive` (`string`, `@values`) — nine Mobus primitives (on `has_primitive` target)
- `cognitive_param_name` (`string`), `cognitive_param_value` (`double`)

**Provenance** (for relations):
- `role` (`string`, `@values` — "source", "sink") — on `participates_in`
- `endpoint` (`string`, `@values` — "start", "end") — on `routes_through`

## The TypeQL Schema

```typeql
define

# =============================================================================
# ATTRIBUTES
# =============================================================================

# Identification
attribute bert_id value string;
attribute model_name value string;
attribute system_level value integer;

# Metadata
attribute display_name value string;
attribute description value string;

# System properties
attribute radius value double;
attribute equivalence_class value string;
attribute history_note value string;
attribute transformation_note value string;
attribute member_autonomy value double;
attribute time_constant value string
    @values("Millisecond", "Second", "Minute", "Hour", "Day", "Week", "Month", "Year", "Decade", "Century", "Epoch");
attribute archetype value string
    @values("Unspecified", "Governance", "Economy", "Agent");

# Complexity (flattened from enum)
attribute complexity_kind value string
    @values("Atomic", "Complex", "Multiset");
attribute complex_adaptable value boolean;
attribute complex_evolveable value boolean;
attribute multiset_count value integer;

# Boundary
attribute porosity value double;
attribute perceptive_fuzziness value double;

# Interface
attribute protocol value string;
attribute interface_type value string
    @values("Import", "Export", "Hybrid");
attribute interface_angle value double;

# Interaction
attribute substance_type value string
    @values("Energy", "Material", "Message");
attribute substance_sub_type value string;
attribute interaction_type value string
    @values("Flow", "Force");
attribute usability value string
    @values("Resource", "Disruption", "Product", "Waste");
attribute amount value string;
attribute unit value string;

# Agent
attribute agent_kind value string
    @values("Reactive", "Anticipatory", "Intentional");
attribute agency_capacity value double;
attribute process_primitive value string
    @values("Combining", "Splitting", "Buffering", "Impeding", "Propelling",
            "Copying", "Sensing", "Modulating", "Inverting");
attribute cognitive_param_name value string;
attribute cognitive_param_value value double;

# Provenance
attribute participation_role value string
    @values("source", "sink");
attribute endpoint value string
    @values("start", "end");

# =============================================================================
# ENTITIES
# =============================================================================

entity bert_model,
    owns model_name @key,
    owns description,
    plays in_environment:environment;

entity system,
    owns bert_id @key,
    owns display_name,
    owns description,
    owns system_level,
    owns radius,
    owns equivalence_class,
    owns history_note,
    owns transformation_note,
    owns member_autonomy,
    owns time_constant,
    owns archetype,
    owns complexity_kind,
    owns complex_adaptable,
    owns complex_evolveable,
    owns multiset_count,
    plays composition:whole,
    plays composition:part,
    plays in_environment:contained_system,
    plays has_boundary:enclosed,
    plays participates_in:participant,
    plays has_agent_config:system;

entity external_entity,
    owns bert_id @key,
    owns display_name,
    owns description,
    owns equivalence_class,
    plays in_environment:contained_system,
    plays participates_in:participant,
    plays is_equivalent_to:primary,
    plays is_equivalent_to:equivalent;

entity boundary,
    owns bert_id @key,
    owns display_name,
    owns description,
    owns porosity,
    owns perceptive_fuzziness,
    plays has_boundary:enclosure,
    plays has_interface:boundary;

entity interface,
    owns bert_id @key,
    owns display_name,
    owns description,
    owns protocol,
    owns interface_type,
    owns interface_angle,
    plays has_interface:interface,
    plays routes_through:interface;

entity interaction,
    owns bert_id @key,
    owns display_name,
    owns description,
    owns substance_type,
    owns substance_sub_type,
    owns interaction_type,
    owns usability,
    owns amount,
    owns unit,
    plays participates_in:interaction,
    plays routes_through:interaction;

entity agent_model,
    owns agent_kind,
    owns agency_capacity,
    plays has_agent_config:config,
    plays has_primitive:agent,
    plays has_cognitive_param:agent;

entity primitive_assignment,
    owns process_primitive,
    plays has_primitive:primitive;

entity cognitive_parameter,
    owns cognitive_param_name,
    owns cognitive_param_value,
    plays has_cognitive_param:param;

# =============================================================================
# RELATIONS
# =============================================================================

# Mobus C: system → subsystem
relation composition,
    relates whole,
    relates part;

# Mobus E: environment → system
relation in_environment,
    relates environment,
    relates contained_system;

# system → boundary (one-to-one)
relation has_boundary,
    relates enclosed,
    relates enclosure;

# boundary → interface (one-to-many)
relation has_interface,
    relates boundary,
    relates interface;

# entity → interaction (replaces source/sink fields, with role)
relation participates_in,
    owns participation_role,
    relates participant,
    relates interaction;

# interaction → interface (optional explicit routing, with endpoint)
relation routes_through,
    owns endpoint,
    relates interaction,
    relates interface;

# external_entity ↔ external_entity (replaces is_same_as_id)
relation is_equivalent_to,
    relates primary,
    relates equivalent;

# system → agent_model (Agent archetype only)
relation has_agent_config,
    relates system,
    relates config;

# agent_model → primitive_assignment (normalized Vec<ProcessPrimitive>)
relation has_primitive,
    relates agent,
    relates primitive;

# agent_model → cognitive_parameter (normalized HashMap<String, f64>)
relation has_cognitive_param,
    relates agent,
    relates param;

# =============================================================================
# INFERENCE FUNCTIONS (starter set — full catalog deferred)
# =============================================================================

# Identifies cross-model structural equivalents: external entities in
# different BERT models that have been declared equivalent. Useful for
# categorical cryptoeconomics queries ("all chains where Users is the
# same real-world entity").
fun cross_model_equivalents() -> { external_entity, external_entity }:
    match
        $a isa external_entity, has bert_id $aid;
        $b isa external_entity, has bert_id $bid;
        (primary: $a, equivalent: $b) isa is_equivalent_to;
        $aid != $bid;
    return { $a, $b };
```

## Multi-Model Namespacing

All `bert_id` values carry a model prefix: `{model_name}:{local_id}`.

Example: Bitcoin's S0 becomes `"bitcoin:S0"`; Ethereum's S0 becomes `"ethereum:S0"`. Both coexist in the same TypeDB instance.

**Why**: Enables the cross-model query use cases the TypeDB pivot was motivated by. Without namespacing, loading a second model would collide with the first.

**Convention**: `model_name` is derived from the JSON file stem (e.g., `bitcoin.json` → `bitcoin`). The transpiler (#37) enforces this.

**Querying a single model**: Filter by `bert_id` prefix using regex: `$id like "^bitcoin:.*"`.

## OWL Alignment Stance

The BERT OWL ontology at `gitbook/for-researchers/bert-systems-ontology.rdf` is the formal ontological grounding for BERT's concept taxonomy (40 entities, 10 object properties).

**Declared authority order**:
1. Rust data model (structural truth)
2. SL spec (semantic truth)
3. OWL ontology (ontological truth)

**Alignment obligations**:
- TypeDB entity names should correspond to OWL class labels where a 1:1 mapping is meaningful
- TypeDB `@values` constraints for attributes should match OWL-declared individual enumerations
- If the OWL ontology defines an entity or property that is *not* represented in TypeDB, that's a deliberate coverage gap (document it) rather than a drift

**Not obligated**:
- TypeDB schema does not need to mirror OWL's full class hierarchy — TypeDB's flat pattern is a deliberate simplification for query ergonomics
- TypeDB does not need to represent every OWL property (some may be metadata-only)

**Drift detection**: when the OWL ontology or SL spec changes, audit this schema. When this schema changes, audit against OWL. The three sources stay in sync through explicit cross-reference, not automation (yet).

## Open Design Questions

### Q1: Cross-model equivalence scope

Currently the schema allows `is_equivalent_to` between any two `external_entity` instances, regardless of model. This enables the categorical cryptoeconomics use case ("bitcoin:Users is structurally equivalent to ethereum:Users").

**Alternative**: Constrain `is_equivalent_to` to within-model pairs (mirroring the JSON `is_same_as_id` scope). Cross-model equivalence would require a separate relation (e.g., `cross_model_equivalent`).

**Decision pending**: defer until #37 transpiler design. The transpiler will have opinions about how to populate this relation from JSON.

### Q2: Environment as entity or attribute

Currently `bert_model` has a `plays in_environment:environment` role. Alternative: represent the environment implicitly via namespace (all entities with `system_level = -1` are in the environment).

**Decision pending**: current design keeps it explicit for queryability. Revisit if queries become awkward.

### Q3: Interface routing ownership

The `routes_through` relation owns an `endpoint` attribute distinguishing "start" vs "end". This is cleaner than two separate relations (`starts_at` + `ends_at`) but requires cardinality discipline — an interaction should have at most one `routes_through` with endpoint=start and at most one with endpoint=end.

TypeDB doesn't enforce this natively; validator or application logic must.

**Decision pending**: if this becomes a frequent error source, split into two relations.

## Integration with the Transpiler (#37)

This schema is consumed by the BERT→TypeDB transpiler (issue #37), which will:

1. Load this schema into a TypeDB database at setup time
2. Parse BERT JSON models and emit TypeQL `insert` queries against this schema
3. Enforce the `{model_name}:{local_id}` namespace convention
4. Validate that all required attributes are populated before insert (cross-referencing `bert/docs/bert-schema-reference.md` §L3 crash-causing fields)

The transpiler does not need to know about inference functions — those are consumed at query time by downstream code (Mesa simulation init, Facets, analysis tools).

## Verification Plan

Before this schema is considered stable:

1. **Syntactic check**: load schema.tql into a local TypeDB 3.x instance, verify no parse errors
2. **Bitcoin round-trip**: transpile `bitcoin.json` (spec-compliant as of #14) into TypeDB, verify all entities/relations/attributes populate without @values violations
3. **Cross-model sanity**: load bitcoin + ethereum + cosmos + solana + llm into the same DB, confirm no ID collisions, confirm `is_equivalent_to` relations populate across models
4. **Inference test**: `cross_model_equivalents()` should return expected pairs (e.g., Users across bitcoin/ethereum/cosmos/solana if we populate cross-model equivalences)
5. **Mesa bridge test**: run a TypeQL query that extracts "all Economy-archetype subsystems at level 1 with their inbound flow substance types" — this is the simulation init use case; should return structured results directly usable by Mesa

All of these happen under #37, not #21. #21 closes when the design is approved and committed.

## Related Issues

- **#38** (closed) — Canonical BERT JSON schema reference. This schema is derived from that reference.
- **#14** (closed) — bitcoin.json now SL spec v0.1 compliant. Bitcoin is the reference test case for the transpiler.
- **#37** (open) — BERT→TypeDB transpiler. Consumes this schema.
- **#41** (open) — Extend SL compliance to ethereum/cosmos/solana/llm. Ensures all reference models are transpilable.
- **#11** (open) — ABM Mesa bridge. Consumes TypeDB as substitute for `bert_loader.py` parsing JSON.

---

*Design derived from gov-graphs TypeDB schema pattern (Botetourt County / NYC governance graphs), BERT SL Specification v0.1, and the Rust canonical data model. Adjusted for BERT's formal controlled vocabularies, Mobus 8-tuple ontological distinctions, and deferred inference catalog design.*
