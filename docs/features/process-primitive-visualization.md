# Feature: Process Primitive Visualization

## Overview

**Feature Name**: Process Primitive Visualization
**Depends On**: `unified-agent-system` (Phases 1-3 complete, data model exists)
**Status**: Design
**Date**: 2026-04-18

## Core Insight

Process primitives are not independently assigned metadata. They are **read off the flow topology**. Each primitive is defined by the transformation between inflows and outflows at a subsystem boundary:

| Observed flow pattern | Implied primitive |
|---|---|
| Multiple inflows → one outflow | **Combining** |
| One inflow → multiple outflows | **Splitting** |
| Inflow now, outflow later (temporal gap) | **Buffering** |
| Some inflow blocked/filtered | **Impeding** |
| Input moved directionally onward | **Propelling** |
| One input, multiple identical outputs | **Copying** |
| Physical input → information output | **Sensing** |
| One flow controls a different flow's behavior | **Modulating** |
| Output is negation/reversal of input | **Inverting** |

The analyst draws flows first (the core modeling act). The tool infers processes from the flow structure and proposes them. The analyst confirms or corrects.

## Theoretical Grounding

Mobus's 9 atomic work processes (the `ProcessPrimitive` enum already in BERT's data model) are a factored basis set for Troncale's ~45 Systems Processes (SPT). Most SPT processes decompose into compositions of Mobus primitives:

| SPT process | Primitive composition |
|---|---|
| Feedback | Sensing → Modulating → Propelling |
| Cycles | Propelling → Buffering → Propelling (loop) |
| Storage | Buffering |
| Flows | Propelling |
| Hierarchy | Combining + Splitting + Buffering (nested) |
| Oscillations | Inverting + Amplifying + Buffering |

BERT does not need to encode 45 SPT processes. It encodes 9 primitives; higher-order processes emerge as detectable compositions.

**Source**: Friendshuh & Troncale (2014), "Virtual Systems Research II: Using Systems Process Theory," Procedia Computer Science 28, 672-681. Their paper proposes six implementation strategies for making systems process theory executable. BERT combines three: OOP structure (Rust types), ABM dynamics (agent decomposition), and Odum-style flow accounting (substance flows).

## What Exists Today

- `ProcessPrimitive` enum: 9 variants in `data_model/mod.rs:604`
- `AgentModel.primitives: Vec<ProcessPrimitive>`: stored, serialized, round-trips
- `ProcessAssignment` struct: name + flexible params
- Zero UI presence: no panel, no rendering, no inference

## Implementation

### Phase A: Process badges in subsystem circles

**Value**: Instant visual read of functional character without clicking.

**Rendering**: Small letter-badges (C, S, B, I, P, K, N, M, V) or compact icons arranged in the lower half of the subsystem circle. Only shown for Agent-archetype subsystems with non-empty `primitives`.

**Badge legend**:
- **C**ombining, **S**plitting, **B**uffering, **I**mpeding, **P**ropelling
- Copying = **K** (avoids collision with Combining)
- Se**N**sing (avoids collision with Splitting)
- **M**odulating, In**V**erting (avoids collision with Impeding)

Alternative: use 2-letter abbreviations (Co, Sp, Bu, Im, Pr, Cp, Se, Mo, Iv) for clarity over compactness.

**Panel**: Add process primitive multi-select to `SubSystemDetails` (when archetype == Agent). Replaces the "checkboxes" item from unified-agent-system Phase 4.

**Scope**: Leptos details panel widget + Bevy circle label rendering. No data model changes.

### Phase B: Flow-based process inference

**Value**: The tool teaches systems process thinking. Analysts don't need to know the primitive vocabulary upfront.

**Inference rules** (applied per-subsystem by examining connected flows):

```
let inflows = flows where sink == this_subsystem
let outflows = flows where source == this_subsystem

// Structural patterns
if inflows.len() > 1 && outflows.len() == 1 → suggest Combining
if inflows.len() == 1 && outflows.len() > 1 → suggest Splitting
if inflows.len() == 1 && outflows.len() == 1 → suggest Propelling (default passthrough)

// Substance patterns
if any inflow.substance != any outflow.substance → suggest Sensing (physical→info) or Modulating (info controls other)
if inflow.substance == Message && outflow.substance == Energy|Material → suggest Modulating
if inflow.substance == Energy|Material && outflow.substance == Message → suggest Sensing

// Copy detection
if outflows.len() > 1 && all outflows share same substance and magnitude → suggest Copying

// Feedback detection (requires graph analysis)
if subsystem participates in a cycle → flag for Buffering (cycle implies temporal storage)
```

**UX**: When an analyst adds/removes a flow connected to a subsystem, BERT checks the inference rules and displays a subtle suggestion badge or toast: "Flow pattern suggests: Combining + Buffering. Apply?" The analyst accepts, modifies, or dismisses.

**Scope**: New Bevy system that runs on flow topology changes. Writes suggestions to a transient component (not persisted until accepted).

### Phase C: Composition pattern detection

**Value**: Surfaces emergent systems patterns the analyst may not have consciously designed.

**Known compositions** (from Mobus):
- **Feedback loop**: Sensing → Modulating → Propelling (across connected subsystems)
- **Energy cycle**: Combining → Amplifying → Splitting
- **Information processing**: Sensing → Copying → Modulating
- **Self-organization**: Copying → Combining → Buffering

**Detection**: Graph traversal across the subsystem network. When a chain of subsystems' assigned primitives matches a known composition template, highlight the group with a subtle outline or label in both canvas and tree views.

**Scope**: Graph analysis system + visual grouping. Requires Phase A primitives to be populated (manually or via Phase B inference).

## What Does NOT Belong Here

- **Standalone SP network diagrams**: Processes are embedded in the model, not a separate visualization.
- **Mathematical framework overlays**: Belongs in simulation execution (Phase 5 of unified-agent-system), not the visual modeler.
- **Full LP catalogs**: Linkage Propositions emerge as detected compositions, not browsable reference material.
- **Amplifying as 10th primitive**: The `universal-processes.md` reference doc in halcyonic adds Amplifying. BERT's enum follows Mobus's canonical 9. Amplifying can be modeled as a parameterized Propelling (gain > 1). Revisit if real modeling friction demands it.

## Relationship to Unified Agent System Phases

| Unified phase | This feature |
|---|---|
| Phase 4: "Process primitive checkboxes" | Replaced by Phase A (badges + panel) + Phase B (inference) |
| Phase 5: "Process primitive execution engine" | Orthogonal — execution consumes the primitives this feature populates |

---

*Design crystallized 2026-04-18 from SPT analysis session (Friendshuh & Troncale 2014 review).*
