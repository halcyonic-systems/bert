# Feature: Ontology-Grounded Concept QA (Oxigraph + Constrained LLM)

> **Archived**: This spec is from `feature/concept-dictionary` (Oct 2025, 88 commits behind main).
> Superseded by `docs/features/chat-knowledge-system.md` on `feature/unified-agent-system`.
> Kept as reference for patterns: ConceptCard model, disambiguation UX, floating panel.

## Metadata

- Feature Name: Ontology-Grounded Concept QA
- Branch: feature/ontology-concept-qa
- Status: Archived (superseded)
- Owners: You (review), Claude (implementation)
- Last Updated: 2025-09-19

## 1) Overview

Enable users to reliably ask "What is X?" and related basic systems questions in the chat, with answers grounded strictly in the ontology (no hallucinations). The ontology is loaded into an embedded Oxigraph store. The chat routes concept queries to an ontology lookup that returns a structured ConceptCard. Optionally, a constrained LLM paraphrases the ConceptCard without adding facts, and always cites the concept IRI.

## 2) Goals / Non-Goals

### Goals
- Rust-native, offline, ontology-grounded concept definitions and relations
- Strict mode: zero LLM, facts directly from ontology
- Friendly mode: LLM restates ConceptCard with hard constraints, no new facts
- Disambiguation for multi-hit label matches and fuzzy search
- Clear IRI citation in every answer

### Non-Goals (Phase 1)
- Model validation/suggestions (tracked separately)
- Full OWL reasoning; use SPARQL over asserted triples
- Large-scale persistence; in-memory store is sufficient initially

## 3) Architecture

```
Leptos ChatPanel -- intent -> Tauri Commands -> OntologyService (Oxigraph)
       ^                    |               |-- load ontology (once)
       |                    |               |-- label/definition lookup
       |                    |               |-- altLabel + fuzzy search
       |                    |               +-- related relations
       +-(optional) constrained LLM <------ ConceptCard json
```

## 4) Data Model

- ConceptCard
```json
{
  "label": "Sink",
  "iri": "http://example.org/bert#Sink",
  "definition": "...",
  "altLabels": ["..."],
  "relations": [{ "predicate": "broader", "objectLabel": "...", "objectIri": "..." }]
}
```

## 5) Public API (Tauri Commands)

All commands return `Result<..., String>`.

1. init_ontology(path: String) -> ()
   - Load ontology from RDF/XML or Turtle once.

2. concept_info(concept: String) -> ConceptCard
   - Case-insensitive match on `rdfs:label`, `skos:altLabel`.
   - Fallbacks: IRI local-name, contains/starts-with fuzzy search.
   - Include label, definition (`skos:definition` or `rdfs:comment`), altLabels, and a few relations (e.g., `skos:broader`, `skos:narrower`, `rdfs:subClassOf`).

3. search_concepts(query: String) -> [ConceptHit]
   - Lightweight list for disambiguation (label, iri, score).

4. answer_concept_question(question: String, mode: String) -> String
   - Router: extract concept candidate -> call `concept_info`.
   - If mode = "strict": format a factual answer with IRI and no LLM.
   - If mode = "friendly": pass ConceptCard to ChatService with a hard system prompt that forbids adding facts; return paraphrase with IRI.

## 6) SPARQL Lookups (Sketches)

Label/definition by exact case-insensitive label
```sparql
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX skos: <http://www.w3.org/2004/02/skos/core#>
SELECT ?c ?label ?def WHERE {
  ?c rdfs:label ?label .
  FILTER(LCASE(STR(?label)) = LCASE(?q))
  OPTIONAL { ?c skos:definition ?def }
}
LIMIT 10
```

Alt labels and fuzzy search (contains)
```sparql
SELECT ?c ?label WHERE {
  ?c rdfs:label ?label .
  FILTER(CONTAINS(LCASE(STR(?label)), LCASE(?q)))
}
LIMIT 20
```

Relations (examples)
```sparql
SELECT ?p ?o ?oLabel WHERE {
  VALUES ?c { <IRI_OF_CONCEPT> }
  ?c ?p ?o .
  OPTIONAL { ?o rdfs:label ?oLabel }
  FILTER(?p IN (skos:broader, skos:narrower, rdfs:subClassOf))
}
LIMIT 20
```

## 7) Backend Implementation

### Files
- src-tauri/src/ontology_service.rs (extend existing or create if new)
- src-tauri/src/lib.rs (register new commands)

### OntologyService additions
- `fn concept_info(&self, q: &str) -> Result<ConceptCard>`
- `fn search_concepts(&self, q: &str) -> Result<Vec<ConceptHit>>`
- Helpers: exact label, altLabel, iri local-name, contains search; relation fetch

### LLM Constraint (Friendly Mode)
Use existing ChatService; system prompt must:
- State: "Only restate fields from the provided ConceptCard JSON. Do not add any information. Include the IRI verbatim."
- Reject unknown fields; if `definition` missing -> answer: "No definition available. IRI: ...".

## 8) Frontend (Leptos) Integration

File: `src/leptos_app/components/chat.rs`
- Intent detection: if message matches `^(what\s+is|define|explain)\s+(.+)\??$` -> route to concept QA.
- Strict/Friendly toggle in chat header (default: Strict for zero-hallucination).
- Disambiguation: if `search_concepts` returns >1, show a short selectable list; on selection, show ConceptCard.
- Always render IRI as a copyable/linkable citation.

## 9) Acceptance Criteria

- Asking "What is a sink?" returns a definition from the ontology with the concept IRI, no extra content.
- Friendly mode uses LLM but never introduces facts not present in ConceptCard; IRI is present.
- Ambiguous terms prompt a disambiguation list.
- Works offline; errors are user-friendly.

## 10) Testing Strategy

### Unit
- concept_info: exact match, altLabel match, iri local-name match, fuzzy match
- No-definition cases handled cleanly
- Relations included when present

### Integration
- answer_concept_question: strict and friendly paths
- Disambiguation flow

### Systems
- Chat UX: toggle behavior, latency, citations present

Required commands (per CONTRIBUTING.md):
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo doc --no-deps --quiet
cargo tauri build
```

## 11) Performance & Privacy

- Ontology load once; single-digit ms lookups for typical queries
- All local processing; no external APIs required

## 12) Timeline (Estimate)

- Backend lookups + commands: 0.5-0.75 day
- Chat intent + UI toggle + disambiguation: 0.5 day
- Tests + polish: 0.25-0.5 day
-> Total: ~1.25-1.75 days

## 13) Claude Implementation Checklist

1. Add commands: `init_ontology`, `concept_info`, `search_concepts`, `answer_concept_question`.
2. Implement Oxigraph SPARQL lookups and relation gathering.
3. Update ChatPanel intent detection and add Strict/Friendly toggle.
4. Implement disambiguation UI flow.
5. Add constrained system prompt in ChatService for Friendly mode.
6. Tests for lookups, routing, and UI behaviors.

---

This feature directly targets ontology-grounded concept QA with a constrained LLM wrapper, ensuring reliable definitions and citations with minimal UI changes.
