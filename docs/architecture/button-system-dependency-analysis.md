# Button System Dependency Analysis
*2025-11-03 | Pre-Implementation Analysis for Drag-and-Drop V2*

## Executive Summary

Deep analysis of CreateButtonSet and FocusedSystem resource dependencies to inform safe drag-and-drop implementation. **Critical Finding**: `change_focused_system` is NOT button-specific - it's a core selection handler that must remain active.

## CreateButtonSet Architecture

### System Registration (mod.rs:303-315)

```rust
.in_set(CreateButtonSet)
.after(TransformPropagate)
.after(GeometryUpdateSet)
.run_if(in_state(AppState::Normal))
```

**Scheduling**: PostUpdate phase, runs AFTER geometry is finalized, ONLY in Normal state (not during FlowTerminalSelection).

### Systems in CreateButtonSet

1. `add_outflow_interface_create_button` - Creates button for outflow from interface
2. `add_inflow_interface_create_button` - Creates button for inflow to interface
3. `add_source_create_button` - Creates external entity source button
4. `add_sink_create_button` - Creates external entity sink button
5. `add_inflow_create_button` (conditional) - Creates inflow button on subsystem
6. `add_outflow_create_button` (conditional) - Creates outflow button on subsystem
7. `add_interface_subsystem_create_buttons` (conditional) - Creates interface subsystem buttons
8. `add_subsystem_from_external_entities_create_button` - Creates subsystem conversion button
9. `remove_unfocused_system_buttons` - Cleans up buttons when focus changes

**Purpose**: Dynamically spawn/despawn UI buttons that allow users to add elements to diagram. All systems here are strictly about button lifecycle management.

## FocusedSystem Resource Analysis

### Definition (resources/mod.rs:9-23)

```rust
#[derive(Debug, Resource, Deref, DerefMut, Copy, Clone, Reflect)]
pub struct FocusedSystem(Entity);

impl Default for FocusedSystem {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
```

**Semantics**: Tracks which system entity the user is currently focused on. Defaults to PLACEHOLDER (invalid entity).

### Writers (2 locations)

#### 1. change_focused_system (ui/mod.rs:35-61)
```rust
pub fn change_focused_system(
    selected_query: Query<(Entity, &PickSelection, Option<&Subsystem>), ...>,
    button_query: Query<&CreateButton>,
    mut focused_system: ResMut<FocusedSystem>,
)
```

**Triggers**: When user selects a system (PickSelection component changes)
**Logic**: Updates FocusedSystem to selected entity, with validation for interface subsystems
**Classification**: **CORE SELECTION HANDLER** - not button-specific!
**Button interaction**: Only queries buttons for validation, doesn't depend on their existence

#### 2. reset_focused_system_on_removal (removal.rs:88-95)
```rust
**focused_system = root_system_query.single();
```

**Triggers**: When currently focused system is deleted
**Logic**: Resets focus to root system
**Classification**: Cleanup handler, unrelated to buttons

### Readers (16 locations - 11 are button-related)

#### Core Systems (NON-button)

1. **spawn_subsystem** (bundles/spawn/subsystem.rs:38)
   - Creates subsystems as children of focused system
   - **Critical**: Without correct FocusedSystem, subsystems spawn in wrong place

2. **setup** (systems/setup.rs:86)
   - Initializes FocusedSystem to root system on startup
   - **Critical**: Establishes initial focus state

3. **select_flow_terminal** (ui/flow/terminal_selecting.rs:75)
   - Uses focused system during flow terminal selection
   - **Important**: Flow creation depends on correct focus

#### Button Systems (button-specific)

4-16. All `add_*_button` systems (external_entity.rs, outflow.rs, inflow.rs, interface.rs, subsystem.rs, mod.rs)
   - Query FocusedSystem to determine where to spawn buttons
   - **Button-specific**: Only used for button positioning

## Dependency Graph

```
User Selection (PickSelection changes)
    ↓
change_focused_system (CORE - not button-specific)
    ↓
FocusedSystem Resource Updated
    ↓
    ├─→ spawn_subsystem (uses for parent system)
    ├─→ select_flow_terminal (uses for flow creation)
    ├─→ Button systems (uses for button positioning) ← BUTTON-SPECIFIC
    └─→ reset_focused_system_on_removal (cleanup)
```

## Critical Insight: change_focused_system Classification

### V1 Mistake
- Disabled `change_focused_system` along with button systems
- Assumed it was button-specific because it's in ui/mod.rs
- Result: FocusedSystem never updated after initialization → subsystems/flows created in wrong place

### Correct Classification
`change_focused_system` is a **core selection handler** that:
- Updates FocusedSystem when user clicks on systems
- Has no inherent dependency on buttons (only queries for validation)
- Is required for correct subsystem/flow placement
- Should remain active even when buttons are disabled

**Button query in function**: Only checks if interface subsystem buttons exist to prevent early selection. This is validation logic, not core functionality.

## What Breaks If CreateButtonSet is Disabled

### Safe to Disable (Button-specific)
- `add_*_button` systems → No buttons spawn (expected)
- `remove_unfocused_system_buttons` → No cleanup needed (no buttons exist)

### MUST Remain Active (Core systems)
- `change_focused_system` → Required for FocusedSystem updates
- `spawn_subsystem` → Uses FocusedSystem for parent
- `select_flow_terminal` → Uses FocusedSystem for flow creation

## Parallel Implementation Strategy

### Phase 0: Preserve Core (✅ No Breaking Changes)

**Keep Active**:
- `change_focused_system` (simplified - remove button validation)
- All geometry/label systems
- Flow creation systems

**Disable Safely**:
- All `add_*_button` systems in CreateButtonSet
- `remove_unfocused_system_buttons`

**Result**: No buttons spawn, but FocusedSystem updates correctly, subsystems/flows work

### Phase 1: Add Palette (✅ Parallel UI)

**New Systems**:
- `spawn_palette_ui` (Startup) - Static palette sidebar
- No interaction with button systems
- Uses own component hierarchy

**Coexistence**: Button system disabled, palette visible, both can be toggled with feature flag

### Phase 2: Add Drag Behavior (✅ Incremental)

**New Systems**:
- `handle_palette_drag_start` (Update)
- `handle_palette_drag_update` (Update)
- `handle_palette_drop` (Update)

**Integration**: Drop handling spawns elements using existing spawn bundles (spawn_subsystem, etc.)

## Simplified change_focused_system

Remove button validation since buttons won't exist:

```rust
pub fn change_focused_system(
    selected_query: Query<(Entity, &PickSelection), (Changed<PickSelection>, With<System>)>,
    mut focused_system: ResMut<FocusedSystem>,
) {
    for (entity, selection) in &selected_query {
        if selection.is_selected {
            **focused_system = entity;
        }
    }
}
```

**Rationale**: Original button validation prevented selecting interface subsystems before all buttons created. Without buttons, this validation is unnecessary.

## Risk Assessment

### Low Risk
- Disabling CreateButtonSet systems ✅
- Adding palette UI alongside ✅
- Keeping change_focused_system active ✅

### Medium Risk
- Icon rendering (Bevy 0.15 ImageNode requirements unclear)
- Drag-and-drop state management (needs careful design)

### High Risk
- Modifying spawn bundles (affects serialization/loading)
- Changing FocusedSystem semantics (cascading effects)

## Next Steps

1. ✅ Keep change_focused_system active (simplified)
2. ✅ Disable CreateButtonSet systems only
3. ⏭️ Research Bevy 0.15 UI ImageNode patterns
4. ⏭️ Implement static palette without drag
5. ⏭️ Add drag behavior incrementally
6. ⏭️ Feature flag to toggle button/drag workflows

## Conclusion

**Key Takeaway**: `change_focused_system` is misnamed - it should be `update_focused_system_on_selection`. It's a core selection handler, not a button system. The V1 failure was caused by incorrect dependency classification.

**Safe Path Forward**: Keep change_focused_system active, disable only button spawning systems, add palette in parallel.
