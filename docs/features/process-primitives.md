# Feature: Process Primitives

## Overview

**Feature Name**: Process Primitives
**Branch**: feature/process-primitives
**Status**: Research Complete — Ready for Implementation
**Date Created**: 2026-04-27
**GitHub Issues**: #13 (parent), #34 (Phase A), #35 (Phase B), #36 (Phase C)

## Description

Lift Mobus's 10 universal atomic work processes from `AgentModel` to all subsystems. Every subsystem performs process work — not just Agent-archetype ones. Process primitives are the operational verbs of what subsystems *do*: the Mobus transforms τ.

The 10 primitives: Amplifying, Buffering, Combining, Copying, Impeding, Inverting, Modulating, Propelling, Sensing, Splitting.

**Source reference**: `operations/systems-science/system-language/universal-processes.md`

**Note**: The existing SL spec §1.10 lists 9 primitives (missing Amplifying). The universal-processes.md doc lists 10. Reconcile during implementation — Amplifying may have been excluded from §1.10 or added later.

## Research Findings (2026-04-27)

### Current State: ProcessPrimitive Already Exists

`ProcessPrimitive` is already defined as an enum in `src/bevy_app/data_model/mod.rs`:

```rust
pub enum ProcessPrimitive {
    Combining, Splitting, Buffering, Impeding,
    Propelling, Copying, Sensing, Modulating, Inverting,
}
```

**Problem**: It's buried inside `AgentModel`, only accessible when `archetype == Agent`:
```
system.agent.primitives: Vec<ProcessPrimitive>
```

Economy, Governance, and Unspecified subsystems can't have primitives — despite all subsystems performing process work.

### The Fix: Lift to System Struct

Add `process_primitives: Vec<ProcessPrimitive>` directly to the `System` struct in the data model.

**Pattern to follow** (backward-compatible optional Vec):
```rust
#[serde(default, skip_serializing_if = "Vec::is_empty")]
pub process_primitives: Vec<ProcessPrimitive>,
```

This matches the existing pattern on `AgentModel.primitives`. Empty vec = no primitives assigned (yet). Old JSON files without the field load cleanly via `#[serde(default)]`.

### Files to Modify

| File | Change | Effort |
|------|--------|--------|
| `src/bevy_app/data_model/mod.rs` | Add `process_primitives: Vec<ProcessPrimitive>` to `System` struct | Small |
| `src/bevy_app/components/system_elements.rs` | Add matching field to ECS `System` component | Small |
| `src/bevy_app/data_model/load.rs` | Map data model field → ECS component (serde handles deserialization) | Small |
| `src/bevy_app/data_model/save.rs` | Map ECS component → data model field (serde handles serialization) | Small |
| `src/leptos_app/details.rs` | Display primitives in details panel | Medium |
| `src/bevy_app/systems/ui/` | Render badges on subsystem circles | Medium |
| `tools/bert-typedb/src/main.rs` | Emit `has_primitive` for all systems (schema already supports it) | Small |
| `docs/system-language-spec.md` | Update §1.10 or add §1.8 for top-level primitives | Small |

### TypeDB Schema Already Supports This

The TypeDB schema already has `entity primitive_assignment` and `relation has_primitive` designed to receive primitives independently of agent models. The transpiler just needs to emit them for all systems.

### Design Decisions

**Single vs. multiple primitives per subsystem**: Multiple. A subsystem can perform several atomic processes simultaneously (e.g., Mempool = Buffering + Sensing). The existing `Vec<ProcessPrimitive>` type is correct.

**Keep AgentModel.primitives or remove?** Keep for now — AgentModel carries additional behavioral configuration (dynamics, parameters) that couples with primitives. The top-level `process_primitives` is the universal annotation; `agent.primitives` is the behavioral wiring. Revisit if redundancy causes drift.

**Primitive count**: Reconcile 9 (SL spec §1.10) vs 10 (universal-processes.md). Add Amplifying to the enum if missing from spec.

### Visual Design (Phase A)

Small letter badges in subsystem circles showing which primitives the subsystem performs:
- `B` = Buffering, `S` = Sensing, `M` = Modulating, etc.
- Multiple badges arranged in top-right quadrant of circle
- Details panel shows full primitive name + mathematical framework + Mobus reference when selected

### Inference Logic (Phase B)

Each primitive has a flow signature derivable from the subsystem's flow topology:
- Count incoming/outgoing flows
- Compare input vs output substance types
- Match against primitive flow patterns
- Report: confirmed (matches label), inferred (unlabeled), conflicts (label ≠ topology)

### Composition Detection (Phase C)

Detect higher-level behavioral patterns from primitive chains across connected subsystems:
- Sensing → Modulating → Propelling = cybernetic control
- Combining → Amplifying → Splitting = thermodynamic work
- Sensing → Copying → Modulating = knowledge construction

## Connection to Systems-Ontology Work

Process primitives are the Mobus transforms τ — currently the opaque parametric field in the 8-tuple. The Foundations doc Frontier chapter identifies this as the formalization's current limit. Making primitives first-class in BERT is the first step toward computationalizing what Joslyn's rule/law distinction demands: knowing *what kind of operation* a subsystem performs.

In the Lean→CQL→TypeDB→Quint→BERT stack:
- Lean proves the structural constraints
- TypeDB stores and enforces the primitive assignments
- Quint could verify behavioral properties of primitive compositions
- BERT visualizes and lets modelers assign/verify primitives

---

_Feature doc created 2026-04-27. Research complete. Ready for Phase A implementation._
