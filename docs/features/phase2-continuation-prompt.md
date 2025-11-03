# Phase 2 Continuation Prompt

## Context

**Branch**: `feature/drag-and-drop-v2`
**Status**: Phase 0 + Phase 1 complete, ready for Phase 2 implementation

**Completed**:
- ✅ Phase 0: Button workflow disabled (CreateButtonSet commented out, change_focused_system simplified)
- ✅ Phase 1: Static palette rendering with 9 icons (create-button style, proven quality)

**Current State**: Clean working branch, palette icons render on left sidebar at x=-550, z=200

## Phase 2 Goal

Implement drag-and-drop interaction: drag palette elements to canvas, spawn corresponding BERT components.

## Technical Foundation (Already Researched)

**BERT's Drag Pattern**:
1. `DragPosition` trigger from `mouse_interaction` plugin
2. Custom drag events via `impl_drag_event!` macro (see `src/bevy_app/events.rs:7-29`)
3. Event handlers in systems (see `src/bevy_app/systems/ui/drag.rs` for examples)

**Existing Drag Events**:
- `ExternalEntityDrag` → `drag_external_entity` system
- `InterfaceDrag` → `drag_interface` system
- `SubsystemDrag` → `drag_subsystem` system

**Pattern**:
```rust
// In events.rs
impl_drag_event!(PaletteDrag);

// In palette.rs spawn
commands.spawn((
    PaletteElement { element_type },
    // ... other components
)).observe(|trigger: Trigger<DragPosition>, mut commands: Commands| {
    commands.trigger(PaletteDrag::from(trigger));
});

// New system
pub fn handle_palette_drag(
    mut events: EventReader<PaletteDrag>,
    palette_query: Query<&PaletteElement>,
    // ... other queries
) {
    // Spawn element based on palette_query.get(event.target).element_type
}
```

## Implementation Steps

1. **Add PaletteDrag event** (`src/bevy_app/events.rs`)
2. **Attach drag observer** to palette elements (`src/bevy_app/systems/palette.rs`)
3. **Create spawn system** that reads PaletteDrag events, spawns elements based on PaletteElementType
4. **Register system** in mod.rs
5. **Test**: Drag subsystem icon → subsystem appears at cursor position

## Key Files

- Events: `src/bevy_app/events.rs`
- Palette: `src/bevy_app/systems/palette.rs`
- Drag handlers: `src/bevy_app/systems/ui/drag.rs`
- System registration: `src/bevy_app/mod.rs`
- Spawn functions: `src/bevy_app/bundles/spawn/*.rs`

## PaletteElementType → Spawn Mapping

```rust
match palette_element.element_type {
    Subsystem => spawn_subsystem(...),
    InterfaceSubsystem => spawn_interface_subsystem(...),
    ImportInterface => spawn_interface(InterfaceType::Import, ...),
    ExportInterface => spawn_interface(InterfaceType::Export, ...),
    Flow => spawn_inflow(...), // or create new flow spawn
    Inflow => spawn_inflow(...),
    Outflow => spawn_outflow(...),
    Source => spawn_external_entity(InterfaceType::Import, ...),
    Sink => spawn_external_entity(InterfaceType::Export, ...),
}
```

## Session Reference

See `/Users/home/Desktop/halcyonic/operations/sessions/2025-11-03/bert-phase1-continuation-session.md` for full context.

## Quick Start Prompt

"Continue Phase 2 drag-and-drop implementation on feature/drag-and-drop-v2 branch. Phase 0+1 complete, palette rendering. Need to implement: (1) PaletteDrag event, (2) attach drag observer to palette elements, (3) system to spawn elements based on PaletteElementType. See docs/features/phase2-continuation-prompt.md for technical details."
