# Feature: Agent Properties — Agency Capacity (ARCHIVED)

> **Archived from**: `feature/agent-properties` branch
> **Archive date**: 2026-02-07
> **Superseded by**: `docs/features/unified-agent-system.md` on `feature/unified-agent-system`
> **Reason**: Reconciled with agent-params-v2 branch into unified agent system design

---

## Overview

**Feature Name**: Agent Properties
**Branch**: feature/agent-properties
**Status**: Design Complete, Implementation Pending
**Date Created**: 2026-01-24
**Date Completed**: —

## Description

Add agency capacity property to Agent-archetype subsystems, enabling characterization of autonomous decision-making capability. This follows Mobus's decision agent framework (Ch. 11) and provides visual feedback through stroke modulation.

**User Vision**: "Add a consciousness slider for parts. Clearer outputs and possibly a self-actualization score. Feedback from self-actualization back into the system (e.g., narrowing 'difference of potential' with higher self). Treat BERT as a practice mirror, not just a diagram."

**Systems Science Grounding**: Rather than "consciousness" (which carries philosophical baggage), we use **Agency Capacity** as the property name, aligning with:
- Mobus's Agent Archetype: Agents are "special cases of an adaptive (and evolvable) system that relies on having a storehouse of experientially based, implicit knowledge"
- Decision Agent Framework (Ch. 11): Computational Engine, Decision Model, Experiential Memory
- HCGS Classification: Agents already distinguished by orange stroke color

## Implemented Functionality

### Phase 1 (This PR)
- [ ] `AgentProperties` component with `agency_capacity: f32` field
- [ ] Slider in SubSystem details panel (Agent archetype only)
- [ ] Serialization support (backward compatible)
- [ ] Visual feedback via stroke intensity modulation

### Phase 2 (Follow-up)
- [ ] Auto-attach/detach when archetype changes
- [ ] Enhanced visual indicators

### Phase 3 (Future)
- [ ] Multi-dimensional breakdown (autonomy, adaptability, sapience)
- [ ] Goals/objectives system for agents
- [ ] Self-actualization score calculation
- [ ] Feedback loop visualization

## Technical Implementation

### Component Definition

```rust
#[derive(Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct AgentProperties {
    pub agency_capacity: f32,
}

impl Default for AgentProperties {
    fn default() -> Self {
        Self { agency_capacity: 0.5 }
    }
}
```

### Architecture Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Property name | "Agency Capacity" | Aligns with Mobus terminology, avoids "consciousness" debate |
| Component location | New `AgentProperties` component | Clean separation, ECS pattern, Agent-specific |
| Scope | Agent-archetype only | Semantic correctness - agency is agent property |
| Initial structure | Single f32 field | MVP simplicity, future expandable |
| Visual feedback | Stroke intensity | Low implementation cost, clear feedback |
| Serialization | Optional field | Backward compatible with existing models |

## Agency Capacity Scale

- **0.0**: Fully reactive/directed - acts only on external commands
- **0.5**: Semi-autonomous - follows rules but adapts to context (default)
- **1.0**: Fully autonomous - independent goal-setting and decision-making

## Visual Feedback

```rust
impl AgentProperties {
    pub fn stroke_modifier(&self) -> f32 {
        0.5 + self.agency_capacity
    }
}
```

## Related Documentation

- Mobus Ch. 11 - Decision Agent Framework
- HCGS archetype documentation
- Existing slider examples: porosity, perceptive_fuzziness in `details.rs`

---

_Originally created 2026-01-24. Archived 2026-02-07._
