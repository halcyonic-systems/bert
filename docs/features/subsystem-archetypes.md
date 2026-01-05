# Feature: Subsystem Archetypes

## Overview

**Feature Name**: Subsystem Archetypes (HCGS Classification)
**Branch**: feature/subsystem-archetypes
**Status**: In Progress
**Contributors**: Claude Code
**Date**: 2026-01-02

## Description

Mark subsystems with HCGS (Hierarchical Cybernetic Governance System) archetype classifications per Mobus framework: **Governance**, **Economy**, or **Agent**. This enables visual distinction of subsystem roles and supports pedagogical use for teaching systems thinking.

**Motivation**: Showcase blockchain system models (Bitcoin, Ethereum, Cosmos, Solana) through a cryptoeconomics lens by making HCGS decomposition visible in the model.

**Archetype Definitions**:
- **Governance**: Policy, rules, coordination, control mechanisms
- **Economy**: Resource allocation, production, value flows
- **Agent**: Active actors that make decisions and take actions
- **Unspecified**: Default, backward compatible

## Implemented Functionality

- [ ] Add `HcgsArchetype` enum with 4 variants
- [ ] Add `SubsystemArchetype` component (composition pattern)
- [ ] Radio buttons in subsystem details panel
- [ ] Color-coded stroke outlines (governance=blue, economy=green, agent=orange)
- [ ] Save/load with backward compatibility
- [ ] Update blockchain models with archetype designations

## Technical Implementation

### Components Added

**`HcgsArchetype` enum** (`components/system_elements.rs`):
```rust
#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Default, Serialize, Deserialize, Sequence)]
pub enum HcgsArchetype {
    #[default]
    Unspecified,
    Governance,
    Economy,
    Agent,
}
```

**`SubsystemArchetype` component** (`components/system_elements.rs`):
```rust
#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct SubsystemArchetype {
    pub archetype: HcgsArchetype,
}
```

### Components Modified

- `data_model/mod.rs`: Add optional `archetype` field to `System` struct
- `data_model/save.rs`: Query and serialize archetype in `build_system`
- `data_model/load.rs`: Insert component if archetype present
- `bundles/` or `spawn/subsystem.rs`: Stroke color based on archetype
- `leptos_app/details.rs`: RadioGroup for archetype selection

### Architecture Decisions

**Composition over modification**: Using separate `SubsystemArchetype` component rather than adding field to `Subsystem` component. Rationale:
- Follows existing patterns (`InterfaceBehavior`, `InterfaceSubsystem`)
- Query-friendly: `Query<(Entity, &Subsystem, Option<&SubsystemArchetype>)>`
- Backward compatible: old subsystems simply lack the component
- Extensible: can add more archetype metadata later

**Stroke modification over outline layer**: Modifying existing stroke color rather than adding separate outline. Rationale:
- Simpler rendering pipeline
- No z-fighting issues
- Consistent with substance type coloring pattern

**Color scheme**:
- Governance: Blue (`#3B82F6` / Blue-500)
- Economy: Green (`#22C55E` / Green-500)
- Agent: Orange (`#F97316` / Orange-500)
- Unspecified: Black (default)

## Usage Examples

```rust
// Color helper method
impl HcgsArchetype {
    pub fn stroke_color(&self) -> Color {
        match self {
            HcgsArchetype::Unspecified => Color::BLACK,
            HcgsArchetype::Governance => Color::srgb_u8(59, 130, 246),
            HcgsArchetype::Economy => Color::srgb_u8(34, 197, 94),
            HcgsArchetype::Agent => Color::srgb_u8(249, 115, 22),
        }
    }
}
```

## Testing Strategy

- [ ] Unit tests for archetype color mapping
- [ ] Save/load round-trip with archetype data
- [ ] Backward compatibility: load model without archetype field
- [ ] Visual verification of stroke colors
- [ ] RadioGroup UI interaction

## Future Improvements

- Archetype-specific validation rules (e.g., governance subsystems must have certain interface patterns)
- Archetype statistics/summary view
- Filter/highlight by archetype
- Legend showing archetype color meanings

## Related Documentation

- Mobus "Systems Science: Theory, Analysis, Modeling, and Design" (2022) - HCGS framework
- `docs/features/drag-and-drop-ui-v2.md` - Recent UI patterns
- `examples/bitcoin-interface-patterns.md` - Blockchain modeling context

---

_This documentation was automatically generated for the Subsystem Archetypes feature on 2026-01-02._
