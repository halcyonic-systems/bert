# BERT Generator Pipeline

**NL-to-model generation: description → LLM extraction → intermediate spec → constraint check → repair → deterministic compilation**

This document describes how BERT generates complete, valid JSON models from natural language descriptions. The pipeline separates what requires intelligence (understanding a system description and identifying its structural elements) from what requires precision (computing IDs, positions, angles, and wiring everything into a schema-compliant WorldModel). An LLM handles the first part. Deterministic Rust code handles the second.

## Architecture Overview

```
                         ┌─────────────────────────────────────────────┐
                         │            bert-generator-core              │
                         │      (Rust crate, source of truth)         │
                         │                                             │
                         │  intermediate.rs  constraints.rs  lib.rs    │
                         │       │                │            │       │
                         │       ▼                ▼            ▼       │
                         │   IntermediateSpec   check()     repair()   │
                         │   validate()         feedback()             │
                         │                                             │
                         │             generator.rs                    │
                         │       BertModelGenerator::generate()        │
                         │                  │                          │
                         │                  ▼                          │
                         │          BERT JSON WorldModel               │
                         └──────────┬──────────────┬──────────────────┘
                                    │              │
                    ┌───────────────┘              └────────────────┐
                    │                                               │
            ┌───────┴───────┐                              ┌───────┴───────┐
            │  bert-tauri   │                              │  bert-python  │
            │  (Rust, FFI)  │                              │  (PyO3 cdylib)│
            │               │                              │               │
            │ pub use       │                              │ fn validate() │
            │   generator;  │                              │ fn repair()   │
            │ pub use       │                              │ fn generate() │
            │   intermediate│                              │               │
            └───────┬───────┘                              └───────┬───────┘
                    │                                               │
        ┌───────────┴───────────┐                      ┌───────────┴───────────┐
        │     BERT Desktop      │                      │   GSR API (Flask)     │
        │  Tauri + Leptos/Bevy  │                      │  serve.py             │
        │                       │                      │  /generate            │
        │  Landing screen input │                      │  /generate-from-      │
        │  Chat panel generate  │                      │    description        │
        │  Ollama extraction    │                      │  Ollama extraction    │
        └───────────────────────┘                      └───────────────────────┘
```

The `bert-generator-core` crate lives at `general-systems-reasoner/core/` and is consumed by both the BERT desktop app (as a Cargo path dependency) and the GSR Python API (via PyO3 bindings at `general-systems-reasoner/python/`).

## Three Generation Paths

### Path 1: BERT GUI (Desktop)

The BERT desktop app (Tauri + Leptos) provides two entry points for generation:

**Landing screen instant creation.** The user types a system name (e.g., "a coffee shop") into the landing screen input. This sends the description to `generate_model_from_conversation`, which runs the full pipeline and loads the result directly onto the canvas.

**Chat panel iterative creation.** The user enters "Creating" mode, has a guided conversation with an LLM (the creation-mode system prompt coaches them through identifying subsystems, sources, sinks, and flows), then clicks "Generate Model". The entire conversation transcript is sent to `generate_model_from_conversation`, giving the extraction LLM richer context.

Both entry points call the same Tauri command:

1. **Try engine first.** POST the description to the GSR API at `http://localhost:5010/generate-from-description`. If the engine is running, it handles the full pipeline and returns a BERT WorldModel.
2. **Fallback to local Ollama.** If the engine is unavailable, BERT runs the pipeline locally: Ollama extraction → constraint check → retry loop → repair → compile.

The local model used for extraction is configured as `gemma4:e2b` (Gemma 4 at a compact quantization level that fits in memory alongside BERT and the Bevy renderer).

### Path 2: GSR API (Server)

The General Systems Reasoner exposes two generation endpoints:

**`POST /generate`** accepts a pre-written intermediate spec and runs repair → validate → compile. Used when the caller (Facets, external tools) has already performed extraction.

**`POST /generate-from-description`** accepts `{"description": "The Federal Reserve"}` and runs the full pipeline: LLM extraction via Ollama → JSON parse → repair → validate → compile. This is the endpoint BERT's Tauri backend tries first.

Both endpoints use the PyO3-compiled `bert_generator` module, which wraps the same `bert-generator-core` Rust crate. The Python bindings expose:
- `bert_generator.validate(spec_json)` → list of error strings
- `bert_generator.repair_spec(spec_json)` → (repaired_json, repair_report)
- `bert_generator.generate(spec_json)` → BERT WorldModel JSON string
- `bert_generator.validate_repair_generate(spec_json)` → full pipeline in one call

### Path 3: CLI (Claude + bert-json-creation skill)

The `bert-json-creation` skill instructs Claude to write an intermediate spec, then compile it using either the Python bindings or the `bert-generate` CLI binary. The binary reads an intermediate spec from stdin and writes a BERT WorldModel to stdout:

```bash
cat my_spec.json | cargo run --bin bert-generate
```

This path gives Claude (or any LLM) the same extraction-then-compile workflow without requiring Ollama or the GSR server. The LLM writes the intermediate spec directly; the deterministic compiler handles everything else.

## The Intermediate Format

The intermediate format is the contract between the creative step (what an LLM or human decides) and the mechanical step (what the compiler computes). It captures structural intent without any of the derived quantities that make manual JSON authoring error-prone.

### What the author specifies

| Section | Content | Example |
|---|---|---|
| `system` | Name and description | `{"name": "Coffee Shop", "description": "..."}` |
| `sources` | External entities that provide inputs | `[{"name": "Suppliers"}, {"name": "Customers"}]` |
| `sinks` | External entities that receive outputs | `[{"name": "Customers"}, {"name": "Landfill"}]` |
| `subsystems` | Internal structural components (nouns, not processes) | `[{"name": "Kitchen"}, {"name": "Service Counter"}]` |
| `routing_table` | How each source/sink connects through a named interface | Import/Export entries with optional processors |
| `external_flows` | What substance crosses each interface | Name, substance type/subtype, usability |
| `internal_flows` | How subsystems connect to each other | Source subsystem → sink subsystem with substance |

### What the generator computes

All of the following are derived deterministically. The author never writes them:

- **IDs**: `S0`, `C0.0`, `C0.1`, `I0.50`, `F-1.0`, `B0.1`, `Src-1.0`, `Snk-1.0`, etc. Assigned by prefix rules and sequential counters (see `bert-schema-reference.md` for the ID system).
- **Levels**: Computed from ID depth (`len(indices) - 1`).
- **Positions**: Subsystems laid out in geometric patterns (line/triangle/diamond/pentagon/N-gon at radius 190 for level-1; smaller N-gon at radius 32 for level-2 children). Sources at x=-520, sinks at x=520, both vertically distributed.
- **Interface angles**: Imports spaced evenly from 2.5 to 3.8 radians (left half of boundary circle). Exports spaced from -0.5 to 0.5 radians (right half). Sorted to match source/sink vertical positions, preventing criss-crossing flow lines.
- **Processor subsystems**: For each interface with `has_processor: true`, a level-1 system is created with `parent_interface` pointing back to the interface, radius 12, positioned at the boundary angle.
- **Processor flows**: Automatically generated to connect each processor to its target subsystem, with directionality determined by Import/Export type.
- **Endpoint offsets**: Flow curve angles computed from `atan2` between source and sink positions.
- **Required field defaults**: `equivalence: ""`, `history: ""`, `transformation: ""`, `member_autonomy: 1.0`, `time_constant: "Second"`, `porosity: 0.0`, `perceptive_fuzziness: 0.0`, and all other fields that BERT requires but that carry no structural information in a first draft.

### Intermediate Spec Type Definition

The canonical types are defined in `general-systems-reasoner/core/src/intermediate.rs` as Rust structs with serde derive. The top-level struct:

```rust
pub struct IntermediateSpec {
    pub system: Option<SystemSpec>,      // or flat name/description at top level
    pub sources: Vec<Source>,            // at least 1
    pub sinks: Vec<Sink>,               // at least 1
    pub subsystems: Vec<Subsystem>,      // at least 1, with optional children
    pub routing_table: Vec<RoutingEntry>,// user-facing input format
    pub interfaces: Vec<InterfaceSpec>,  // post-normalization (replaces routing_table)
    pub external_flows: Vec<ExternalFlow>,
    pub internal_flows: Vec<InternalFlow>,
    pub processor_flows: Vec<ProcessorFlow>, // auto-generated during normalization
}
```

Each `Subsystem` supports:
- `archetype`: `"Agent"`, `"Economy"`, or `"Governance"` (optional)
- `agent`: `{kind, agency_capacity}` when archetype is Agent
- `complexity`: `"Complex"` or `"Atomic"`
- `children`: nested `Vec<Subsystem>` for level-2 decomposition

Each `RoutingEntry` specifies:
- `interface`: unique name for the boundary interface
- `type`: `"Import"` (from source) or `"Export"` (to sink)
- `connected_to`: name of the source or sink entity
- `has_processor`: whether to generate a processor subsystem
- `target_subsystem`: which subsystem the processor routes to

Flows carry `substance: {type, sub_type}` where type is `"Energy"`, `"Material"`, or `"Message"`, and `usability` is `"Resource"`, `"Product"`, `"Waste"`, or `"Disruption"`.

## The Compilation Pipeline

`BertModelGenerator::generate()` runs eight phases in sequence:

### Phase 0a: Normalization

Bridges the user-facing format to the generator's internal representation.

1. **Flatten system metadata.** If the spec has `system: {name, description}`, copy those to top-level `name` and `description` fields.
2. **Convert routing_table to interfaces.** Each routing entry becomes an `InterfaceSpec` with `receives_from` / `exports_to` arrays. If `has_processor` is true, a `ProcessorSpec` is attached and a processor flow is auto-generated.
3. **Annotate external flows with direction.** Import interfaces set `direction: "in"` and populate the `source` field from `connected_to`. Export interfaces set `direction: "out"` and populate `sink`.
4. **Normalize substance fields.** Flatten `substance: {type, sub_type}` into `substance_type` and `substance_subtype` on every flow for uniform access downstream.

### Phase 0b: ID and Position Assignment

Assigns all entity IDs and computes spatial layout:

- Sources get `Src-1.0`, `Src-1.1`, ...
- Sinks get `Snk-1.0`, `Snk-1.1`, ...
- Interfaces: bare interfaces start at `I0.0` counting up; processor-equipped interfaces start at `I0.50` counting up (giving ample room for both pools).
- Processors: `C0.50`, `C0.51`, ... (mirroring their interface IDs)
- Level-1 subsystems: `C0.0`, `C0.1`, ... with positions from hardcoded layouts (1: center, 2: horizontal pair, 3: triangle, 4: diamond, 5+: N-gon at radius 190)
- Level-2 children: `C0.0.0`, `C0.0.1`, ... with positions relative to parent (similar geometric patterns at radius 32)

### Phase 1: Build Environment

Creates the `E-1` environment with positioned sources and sinks. Sources are placed at x=-520 (left side), sinks at x=520 (right side), both vertically distributed with 150px spacing.

### Phase 2: Build Root System S0

Creates the root system with radius 300, positioned at origin. Builds the interface list for S0's boundary, with angles computed to align with their connected sources/sinks.

### Phase 3: Build Subsystems

Creates all level-1 and level-2 subsystem entities with their boundaries, complexity settings, and optional archetypes/agent models.

### Phase 4: Build Interface Processors

For each interface with a processor spec, creates a small (radius 12) subsystem with `parent_interface` pointing to its interface ID. Import processors face right (rotation -pi), export processors face left (rotation 0).

### Phase 5: Build External Flows

Creates `F-1.x` flows for each external flow. Import flows run Source → S0 (with `sink_interface` set). Export flows run S0 → Sink (with `source_interface` set).

### Phase 6: Build Processor Flows

Creates `F0.x` flows connecting each processor to its target subsystem. Direction depends on interface type: Import processors route inward (processor → subsystem), Export processors route outward (subsystem → processor).

### Phase 7: Build Internal Flows

Creates `F0.x` flows between subsystems. Resolves names to IDs (supporting direct names, `Parent/Child` slash notation, and processor name lookups). Computes endpoint angles from position geometry. Detects level-2 sibling flows and sets their level accordingly.

### Assembly

Wraps everything into the final WorldModel:

```json
{
  "version": 1,
  "environment": { ... },
  "systems": [ S0, C0.0, C0.1, C0.50, ... ],
  "interactions": [ F-1.0, F-1.1, F0.0, ... ],
  "hidden_entities": []
}
```

## Constraint Checking

Before repair runs, the pipeline can optionally check the intermediate spec against seven structural constraints (`constraints.rs`). These are systems-science invariants, not just schema rules:

| Constraint | What it checks |
|---|---|
| `all_flows_connected` | Every internal flow source/sink names an existing subsystem. Every routing entry references an existing source/sink. |
| `no_isolated_entities` | Every subsystem participates in at least one internal flow or routing entry. No structural orphans. |
| `has_feedback_loop` | Internal flows contain at least one cycle (A→B→A or A→B→C→A). Systems without feedback cannot self-regulate. |
| `substance_valid` | All substance types are `Energy`, `Material`, or `Message`. |
| `boundary_coherent` | Every source has an Import routing entry. Every sink has an Export routing entry. No unrouted boundary crossings. |
| `descriptions_present` | Subsystems, sources, and sinks all have descriptions of at least 10 characters. |
| `transforms_described` | Subsystem descriptions are not generic boilerplate ("processes inputs", "handles data") but describe actual transformations. |

### Check-Feedback-Retry Loop

When BERT runs the pipeline locally (Ollama fallback), it uses constraint failures as structured feedback to the LLM:

```
Attempt 1: extract → check → 3 failures
  ↓ format failures as numbered correction list
Attempt 2: re-extract with feedback in conversation → check → 0 failures
  ↓ proceed to repair and compile
```

The retry loop runs up to 2 iterations. The feedback message includes the original extraction as the assistant's prior response, followed by a user message listing each failure with the constraint name and actionable fix description. This gives the LLM explicit, machine-generated correction signals rather than relying on prompt engineering alone.

## Repair Pipeline

After constraint checking (or instead of it, for speed), the repair pipeline (`lib.rs::repair()`) applies 16 deterministic fixups organized in three categories:

### Structural Repairs

| Repair | What it fixes |
|---|---|
| `repair_empty_names` | Names empty entries "Source 1", "Subsystem 2", etc. |
| `repair_missing_sinks` | Adds a default sink with routing and flow if none exist |
| `repair_missing_sources` | Adds a default source with routing and flow if none exist |
| `repair_routing_types` | Corrects Import/Export when `connected_to` points to the wrong entity type |
| `repair_missing_routing_entries` | Adds routing entries for sources/sinks that lack them |
| `repair_missing_external_flows` | Adds external flows for routing entries that lack them |
| `repair_dangling_internal_flows` | Removes internal flows that reference non-existent subsystems |
| `repair_self_loop_flows` | Removes internal flows where source == sink |
| `repair_dangling_routing_entries` | Removes routing entries pointing to non-existent sources/sinks |
| `repair_orphaned_external_flows` | Removes external flows referencing non-existent interfaces |
| `repair_duplicate_interface_names` | Renames duplicate interface names to be unique |

### Semantic Repairs

| Repair | What it fixes |
|---|---|
| `repair_substance_types` | Reclassifies substance types based on keyword signals (e.g., "water" → Material, "electricity" → Energy). Uses curated keyword lists for Energy (~20 terms) and Material (~40 terms). |
| `repair_generic_subtypes` | Replaces generic sub_types ("data", "output", "signal") with the flow name when available |
| `repair_waste_usability` | Sets usability to "Waste" when flow names/subtypes contain waste-indicating terms ("exhaust", "effluent", "spent", etc.) |
| `repair_empty_substance_subtypes` | Fills missing sub_types with type-appropriate defaults ("Kinetic" for Energy, "Solid" for Material, "Data" for Message) |
| `repair_invalid_enums` | Corrects any enum value not in the valid set (usability, substance type, interaction type, complexity) |

### Connectivity Repairs

| Repair | What it fixes |
|---|---|
| `repair_isolated_subsystems` | Connects orphaned subsystems to the first connected subsystem via a "Coordination" message flow |

Every repair returns a human-readable report string (e.g., `"Reclassified 'Heat Output': Message → Energy"`). The BERT GUI logs these to the browser console; the GSR API returns them in the response body.

## Validation

After repair, the intermediate spec is validated against the typed `IntermediateSpec` struct (`intermediate.rs::validate_intermediate()`). This catches issues that repair cannot fix:

- Missing required fields (system name, at least 1 source, at least 1 sink, at least 1 subsystem)
- Duplicate names within the same entity class
- Invalid enum values that repair missed
- Routing entries with `has_processor: true` but no `target_subsystem`
- External flows referencing interfaces not in the routing table
- Internal flows referencing subsystems that do not exist
- Invalid archetypes, agent kinds, or complexity values

Validation returns a list of error strings. An empty list means the spec is safe to compile.

## Extraction Prompt Design

The LLM extraction prompt (used by both BERT's local fallback and the GSR API) is carefully structured to minimize repair work:

1. **Schema template.** The exact JSON structure is shown with placeholder values. The LLM fills in the blanks rather than inventing a format.
2. **Naming rules.** Explicit guidance: subsystems are nouns not verbs, sources/sinks are entities not substances, sub_types describe what flows not where it goes.
3. **Cycling rule.** The prompt requires at least one feedback loop, matching the `has_feedback_loop` constraint.
4. **Valid enum values.** Listed explicitly so the LLM does not invent types like "Information" or "Chemical".
5. **Worked example.** A complete coffee shop model demonstrates every field and naming convention.
6. **Output discipline.** "Output ONLY valid JSON. No markdown fences, no explanation, no thinking tags." With a strip-fences fallback in the parser for models that ignore this.

The LLM model is configurable per deployment (`FACETS_MODEL` env var for GSR, hardcoded `gemma4:e2b` for BERT desktop).

## Crate Structure

```
general-systems-reasoner/
├── Cargo.toml              # workspace: [core, python, rag]
├── core/
│   ├── Cargo.toml          # bert-generator-core, deps: serde, serde_json
│   ├── src/
│   │   ├── lib.rs          # pub fn check(), pub fn repair()
│   │   ├── intermediate.rs # IntermediateSpec types + validate_intermediate()
│   │   ├── constraints.rs  # 7 structural constraint checks + format_feedback()
│   │   └── generator.rs    # BertModelGenerator: normalize → assign_ids → build → assemble
│   └── tests/
│       └── constraint_pipeline.rs  # End-to-end test: description → extract → check → retry → repair
├── python/
│   ├── Cargo.toml          # bert-generator-python, cdylib via PyO3
│   └── src/
│       └── lib.rs          # validate(), repair_spec(), generate(), validate_repair_generate()
├── rag/                    # RAG engine (separate from generation)
└── serve.py                # Flask API: /generate, /generate-from-description
```

BERT consumes the core crate as a path dependency in `src-tauri/Cargo.toml`:

```toml
bert-generator-core = { path = "../../general-systems-reasoner/core" }
```

The BERT Tauri backend re-exports the generator and intermediate modules:

```rust
pub use bert_generator_core::generator;
pub use bert_generator_core::intermediate;
```

## Related Documents

- **`bert-schema-reference.md`** -- Canonical BERT JSON schema (what the generator outputs). ID system, entity types, required fields, crash-causing fields.
- **`system-language-spec.md`** -- Formal System Language specification grounded in Mobus 8-tuple and Lean 4 proofs. Theoretical foundation for why the intermediate format has the fields it does.
- **`bert-json-creation` skill** -- Claude Code skill that wraps the generator for CLI-driven model creation. Documents Path A (intermediate → generator) and Path B (manual JSON).
