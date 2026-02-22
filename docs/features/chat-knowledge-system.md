# Unified Feature: Chat & Knowledge System

## Overview

**Feature Name**: Chat & Knowledge System (concept dictionary + model-aware assistance)
**Target Branch**: `feature/unified-agent-system`
**Status**: Planned (Phase 1 spec complete)
**Source Branches** (archived — see `docs/features/archive/`):
- `feature/concept-dictionary` — Oxigraph-backed concept QA with Leptos chat UI (1,044 lines)
- Archived spec: `docs/features/archive/ontology-concept-qa-oxigraph.md`

## Knowledge Ecosystem

BERT's knowledge capabilities span four interconnected projects, each with one clear role:

```
bert-systems-ontology.rdf (+ mobus, bunge, troncale RDFs)
    |           ^ maintained in Protege, visualized in onto-viz
    |
    |-- onto-viz: Standalone exploration/comparison (shipped v3.1)
    |   +-- Coverage matrix tracks BERT vs Mobus alignment
    |   +-- 5 bundled ontologies: BERT (40), Mobus (62), Bunge (101),
    |   |   Troncale (62), Cognitive Functions (8) = 273 total
    |   +-- Used by developer/researcher, NOT embedded in BERT
    |
    |-- BERT Chat Phase 1: Concept Dictionary (both platforms)
    |   +-- Build step: RDF -> concepts.json (bundled asset)
    |   +-- ConceptService (Rust, WASM-compatible, no Oxigraph)
    |   +-- ChatPanel (Leptos floating toggle)
    |   +-- "What is a boundary?" -> structured ConceptCard
    |
    |-- BERT Chat Phase 2: Model-Aware Assistance (adds LLM)
    |   +-- Current model JSON -> LLM -> modeling guidance
    |   +-- "Does my model follow Mobus principles?" -> validation
    |   +-- Optional: call bert-rag for deeper Mobus knowledge
    |
    +-- bert-rag: Independent research tool (future MCP server)
        +-- Deep textbook knowledge via LlamaIndex + Ollama
        +-- 554 chunks, 734 KG relations from Mobus textbook
        +-- Used from Claude Code, notebooks, etc.
        +-- NOT embedded in BERT — separate process, optional bridge
```

**Key principle:** Each tool has ONE clear role. onto-viz = explore ontologies. BERT chat = in-app concept lookup + guidance. bert-rag = deep research. The RDF ontology is the shared contract.

| Project | Role | Location | Status |
|---------|------|----------|--------|
| **bert-systems-ontology.rdf** | Source of truth (40 concepts, 9 properties) | `bert/gitbook/for-researchers/` + `onto-viz/ontologies/` | Maintained |
| **onto-viz** | Browser-based ontology explorer + comparison | `halcyonic-projects/active/onto-viz/` | Shipped v3.1 |
| **bert-rag** | Python RAG for Mobus textbook | `halcyonic-projects/2026/bert-rag/` | Working prototype |
| **feature/concept-dictionary** | Leptos chat UI + Oxigraph backend | BERT branch (88 commits behind) | Archived reference |

## Phase 1: Concept Dictionary

**Scope**: Ontology-grounded concept lookup in a floating chat panel. Works on both web (trunk serve) and desktop (cargo tauri dev). No external dependencies — pure Rust + bundled JSON.

**Why no Oxigraph?** The ontology is small (~273 concepts across all frameworks). Pre-process RDF to JSON at build time, bundle as a BERT asset. Works in WASM and Tauri identically. Oxigraph adds ~2MB binary size and WASM compatibility complexity for no practical benefit at this scale.

### Build Step: RDF to JSON

Python script converts RDF ontology files to a single `concepts.json`:

**Input**: `bert-systems-ontology.rdf` (required) + optionally mobus, bunge, troncale RDFs from onto-viz
**Output**: `assets/data/concepts.json`

```json
{
  "concepts": [
    {
      "id": "bert:Subsystem",
      "label": "Subsystem",
      "definition": "A system that is a component of a larger system...",
      "math_definition": "S = {C, F, B} where C = components, F = flows, B = boundary",
      "json_mapping": "SubsystemNode in world_model.subsystems[]",
      "implementation_note": "Maps to Bevy Entity with SubsystemBundle",
      "examples": ["Power grid transformer station", "Cell membrane"],
      "relations": [
        { "predicate": "rdfs:subClassOf", "target_label": "System", "target_id": "bert:System" },
        { "predicate": "bert:hasBoundary", "target_label": "Boundary", "target_id": "bert:Boundary" }
      ]
    }
  ],
  "metadata": {
    "source_ontologies": ["bert-systems-ontology.rdf"],
    "generated_at": "2026-02-07T00:00:00Z",
    "concept_count": 40
  }
}
```

The BERT ontology already has rich annotations (`jsonMapping`, `implementationNote`, `bertExample`, `mathematicalDefinition`) that map directly to these fields.

### ConceptService (Rust)

**File**: `src/bevy_app/services/concept_service.rs` (or `data_model/concepts.rs`)

```rust
pub struct Concept {
    pub id: String,
    pub label: String,
    pub definition: Option<String>,
    pub math_definition: Option<String>,
    pub json_mapping: Option<String>,
    pub implementation_note: Option<String>,
    pub examples: Vec<String>,
    pub relations: Vec<ConceptRelation>,
}

pub struct ConceptRelation {
    pub predicate: String,
    pub target_label: String,
    pub target_id: String,
}

pub struct ConceptMatch {
    pub concept: Concept,
    pub score: f32,       // 0.0-1.0, higher = better match
    pub match_type: MatchType,  // Exact, Prefix, Contains, Fuzzy
}

pub enum MatchType {
    Exact,
    Prefix,
    Contains,
    Fuzzy,
}
```

**API**:
- `ConceptService::new() -> Self` — loads `concepts.json` via `include_str!`
- `lookup(query: &str) -> Vec<ConceptMatch>` — fuzzy search, returns ranked matches
- `get(id: &str) -> Option<&Concept>` — exact ID lookup

**Matching logic** (ported from concept-dictionary's `ontology_service.rs` scoring):
1. Exact label match (score: 1.0)
2. Case-insensitive exact (score: 0.95)
3. Prefix match (score: 0.8)
4. Word boundary contains (score: 0.6)
5. Substring contains (score: 0.4)

Pure Rust, no Oxigraph — works in WASM identically to native.

### ChatPanel (Leptos Component)

**File**: `src/leptos_app/components/chat_panel.rs`

**Layout**: Floating toggle button (bottom-right, z-20), follows existing Palette button pattern.

**Message model**:
```rust
pub enum ChatMessage {
    User(String),
    ConceptCard(Concept),
    Info(String),
    Error(String),
}
```

**Behavior**:
1. User types query in input field
2. Intent detection extracts concept name from patterns:
   - "what is X?" / "what's X?" / "define X" / "explain X"
   - Bare term "boundary" also works (direct lookup)
3. ConceptService.lookup(query) returns matches
4. Single match → render ConceptCard immediately
5. Multiple matches → disambiguation list (clickable)
6. No match → "No concept found for 'X'. Try: [suggest similar]"

**ConceptCard rendering**:
- Label (h3, bold)
- Definition (paragraph)
- Relations (grouped by predicate, clickable targets)
- JSON Mapping (code block, if present)
- Math Definition (if present, formatted)
- Implementation Note (if present, collapsible)

**Reference patterns**: concept-dictionary branch's `chat.rs` has proven Leptos 0.8-style signals for the floating panel, message history, and ConceptCard rendering.

### Platform Strategy

The JSON-as-bundled-asset approach eliminates the web vs desktop split entirely:

| Concern | Web (trunk serve) | Desktop (cargo tauri dev) |
|---------|-------------------|--------------------------|
| Asset loading | `include_str!` at compile time | Same — WASM in webview |
| Search | In-memory Rust | Same |
| UI | Leptos in browser | Leptos in Tauri webview |

No `#[cfg]` gates needed. No Tauri commands needed. Everything runs in WASM.

## Phase 2: Model-Aware Assistance (Future)

**Adds**: LLM integration for contextual modeling help.

**Architecture**:
1. Extract current WorldModel JSON as context
2. Send to LLM (local Ollama or API) with constrained system prompt
3. LLM answers questions about the user's specific model

**Capabilities**:
- **Validation**: "Does my model follow Mobus principles?" — check for missing boundaries, isolated subsystems, disconnected flows
- **Suggestions**: Coverage matrix logic (adapted from onto-viz) identifies missing concepts — "Your model has no Sink, but has Sources and Flows. Consider adding a Sink."
- **Guided modeling**: "Help me decompose this system" — structured prompting workflow

**Optional bridge to bert-rag**: For deeper Mobus textbook knowledge, Phase 2 can call bert-rag as an external service. bert-rag is NOT embedded — it runs as a separate process (future MCP server).

## Phase 3: Deep Integration (Future)

- Multi-framework responses (cite Mobus, Bunge, Troncale simultaneously)
- bert-rag MCP server integration
- Model generation from chat ("create a model of X")
- Conversation memory across sessions

## Data Model

Core types (referenced above, consolidated here):

```rust
// concept_service.rs
pub struct ConceptStore {
    pub concepts: Vec<Concept>,
    pub metadata: StoreMetadata,
}

pub struct StoreMetadata {
    pub source_ontologies: Vec<String>,
    pub generated_at: String,
    pub concept_count: usize,
}

pub struct Concept { /* see Phase 1 section */ }
pub struct ConceptRelation { /* see Phase 1 section */ }
pub struct ConceptMatch { /* see Phase 1 section */ }
pub enum MatchType { /* see Phase 1 section */ }

// chat_panel.rs
pub enum ChatMessage { /* see Phase 1 section */ }
```

## Testing Strategy

### Unit Tests
- JSON deserialization round-trip (serialize ConceptStore, deserialize, compare)
- Concept lookup accuracy: exact match, case-insensitive, prefix, substring, no-match
- Scoring: verify ranking order for ambiguous queries
- Edge cases: empty query, very long query, special characters

### Integration Tests
- ChatPanel intent detection: "what is a boundary?" extracts "boundary"
- Disambiguation: query matching 3+ concepts shows list
- ConceptCard rendering: all optional fields handled (present and absent)

### Platform Tests
- `trunk serve` — chat panel opens, concept lookup works
- `cargo tauri dev` — identical behavior in desktop webview
- `cargo test -p bert` — all unit tests pass

### Quality Gates
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test -p bert
```
