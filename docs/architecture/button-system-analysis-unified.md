# Button System Dependency Analysis - Unified
*2025-11-03 | Synthesized from Claude Code + Claude Desktop analyses*

## Executive Summary

**Critical Discovery**: Both analyses converge on the same root cause - `change_focused_system` is a **core selection handler**, not a button-specific system. V1 failure was caused by disabling this critical system along with button UI, breaking FocusedSystem updates cascading to 18+ dependent systems.

**Safe Path Forward**: Keep `change_focused_system` active, implement palette using world-space `Sprite` components (consistent with BERT architecture), parallel implementation with feature flags.

---

## 1. CreateButtonSet Architecture

### System Registration (mod.rs:303-320)

```rust
.in_set(CreateButtonSet)
.after(TransformPropagate)
.after(GeometryUpdateSet)
.run_if(in_state(AppState::Normal))
```

**Schedule**: PostUpdate phase
**Dependencies**: Runs AFTER geometry finalization, ONLY in Normal state (not FlowTerminalSelection)
**Purpose**: Dynamically spawn/despawn UI affordance buttons for adding elements

### 9 Systems in CreateButtonSet (All Button-Specific)

| System | Purpose | Safe to Disable |
|--------|---------|-----------------|
| `add_outflow_interface_create_button` | Spawns button for outflow from interface | ✅ Yes |
| `add_inflow_interface_create_button` | Spawns button for inflow to interface | ✅ Yes |
| `add_source_create_button` | Spawns external entity source button | ✅ Yes |
| `add_sink_create_button` | Spawns external entity sink button | ✅ Yes |
| `add_inflow_create_button` (conditional) | Spawns inflow button on subsystem | ✅ Yes |
| `add_outflow_create_button` (conditional) | Spawns outflow button on subsystem | ✅ Yes |
| `add_interface_subsystem_create_buttons` (conditional) | Spawns interface subsystem buttons | ✅ Yes |
| `add_subsystem_from_external_entities_create_button` | Spawns subsystem conversion button | ✅ Yes |
| `remove_unfocused_system_buttons` | Cleans up buttons when focus changes | ✅ Yes |

**Resources Written**: None directly (only spawns entities with `CreateButton` component)
**Resources Read**: `FocusedSystem`, `Zoom`, `AssetServer`

**Analysis Convergence**: Both analyses agree these 9 systems are purely button lifecycle management with no impact on core rendering.

---

## 2. FocusedSystem Resource Flow

### Definition (resources/mod.rs:9-23)

```rust
#[derive(Debug, Resource, Deref, DerefMut, Copy, Clone, Reflect)]
pub struct FocusedSystem(Entity);

impl Default for FocusedSystem {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)  // ⚠️ Invalid entity initially!
    }
}
```

**Semantics**: Tracks which system entity user is currently focused on
**Initialization**: Set to root system in `setup.rs:50`, updated on model load

### Writers (4 Systems)

| System | Location | Schedule | Purpose | Classification |
|--------|----------|----------|---------|----------------|
| `change_focused_system` | ui/mod.rs:35-61 | **Update** | Updates on system selection | **🔴 CORE** |
| `cleanup_focused_system` | removal.rs:80-95 | RemovalCleanupSet | Resets to root when focused system deleted | Cleanup |
| `setup` | setup.rs:50 | Startup | Initial setup | Initialization |
| `load_world` | data_model/load.rs:279 | Update | During file load | Serialization |

### Readers (18+ Systems)

**Core Systems (Must remain active)**:
1. `spawn_subsystem` (bundles/spawn/subsystem.rs:38) - Creates subsystems as children of focused system
2. `select_flow_terminal` (ui/flow/terminal_selecting.rs:75) - Flow creation on focused system
3. `on_create_button_click` (ui/mod.rs:279) - Element spawning (used by drag-and-drop too)
4. `on_external_entity_create_button_click` (ui/mod.rs:196) - External entity spawning

**Button Systems (Button-specific)**:
5-13. All `add_*_button` systems (9 systems query FocusedSystem for button positioning)

**Critical Cleanup**:
14. `remove_unfocused_system_buttons` (ui/mod.rs:64-82) - Runs every frame when FocusedSystem changes, removes buttons from non-focused systems

**Analysis Convergence**: Both analyses identified the same 18+ readers, with clear separation between core (4) and button-specific (14).

---

## 3. The Critical System: change_focused_system

### Current Implementation (ui/mod.rs:35-61)

```rust
pub fn change_focused_system(
    selected_query: Query<
        (Entity, &PickSelection, Option<&Subsystem>),
        (Changed<PickSelection>, With<System>)
    >,
    button_query: Query<&CreateButton>,  // Only for validation
    mut focused_system: ResMut<FocusedSystem>,
) {
    for (entity, selection, subsystem) in &selected_query {
        if selection.is_selected {
            if let Some(subsystem) = subsystem {
                for button in &button_query {
                    if matches!(button.ty, CreateButtonType::InterfaceSubsystem { .. })
                        && button.system == subsystem.parent_system
                    {
                        // Prevent selecting interface subsystem before all buttons created
                        return;
                    }
                }
            }
            **focused_system = entity;
        }
    }
}
```

### Dependency Analysis

**Core Functionality**: Updates `FocusedSystem` when user clicks on systems (via `PickSelection` component)
**Button Interaction**: Only queries `CreateButton` for validation (lines 49-56), doesn't depend on buttons existing
**Trigger**: `Changed<PickSelection>` - user selection, not button interaction
**Schedule**: Update (NOT in CreateButtonSet)

### Why V1 Failed

**Incorrect Classification**: Assumed button-specific because it's in `ui/mod.rs` and queries buttons
**What Actually Happened**: Disabled along with button systems → `FocusedSystem` never updated after initialization
**Cascading Effects**:
- Buttons spawned on wrong system (Entity::PLACEHOLDER)
- Subsystems created as children of invalid entity
- Flows connected to wrong system
- Label/geometry systems using stale focus data
- `remove_unfocused_system_buttons` deleted buttons incorrectly

### Correct Classification: 🔴 CORE SELECTION HANDLER

| Evidence | Conclusion |
|----------|------------|
| Scheduled in Update, not CreateButtonSet | Not grouped with button systems |
| Triggered by PickSelection changes | User interaction, not button lifecycle |
| Only validation uses buttons | Core logic independent of buttons |
| 18+ systems depend on FocusedSystem | Critical resource for core rendering |
| Named "change_focused_system" not "update_button_focus" | Naming suggests broader scope |

**Analysis Convergence**: Both analyses independently identified this as the root cause of V1 failure.

### Simplified Version (For Drag-and-Drop)

Remove button validation since buttons won't exist:

```rust
pub fn change_focused_system(
    selected_query: Query<
        (Entity, &PickSelection),
        (Changed<PickSelection>, With<System>)
    >,
    mut focused_system: ResMut<FocusedSystem>,
) {
    for (entity, selection) in &selected_query {
        if selection.is_selected {
            **focused_system = entity;
        }
    }
}
```

**Rationale**: Original validation prevented selecting interface subsystems before all buttons created. Without buttons, validation unnecessary.

---

## 4. Bevy 0.15 Image Rendering Patterns

### Critical Discovery: BERT Uses World-Space Sprites, NOT UI Nodes

**Evidence from spawn_create_button (bundles/spawn/create_button.rs:54-66)**:

```rust
commands.spawn((
    create_button,
    Sprite {  // ⚠️ World-space sprite, not UI!
        image: asset_server.load(path),
        custom_size: Some(Vec2::new(button_width, button_width)),
        ..default()
    },
    Transform::from_translation((position * zoom).extend(BUTTON_Z))
        .with_rotation(Quat::from_rotation_z(angle)),
    InitialPosition::new(position),
    Name::new(name),
    PickingBehavior::default(),
));
```

### Architectural Pattern

**Current BERT Architecture**:
- ❌ No UI nodes (`Node`, `ImageNode`) found in codebase
- ✅ World-space sprites (`Sprite` component) for all visual elements
- ✅ Buttons positioned relative to parent systems using `Transform`
- ✅ Buttons parented to system entities (line 82: `commands.entity(parent).add_children(&[button_entity])`)
- ✅ Elements scale with zoom, rotate with system orientation
- ✅ Uses `Text2D` from label plugin for text rendering

### Why Icon Rendering Failed in V1

**Analysis Convergence** - Both analyses identified same issues:

1. **SVG Files** ❌
   - Bevy doesn't support SVG textures
   - Must convert to PNG first

2. **ImageNode API Confusion** ❌
   - Attempted to use UI node pattern: `ImageNode::new()`
   - BERT doesn't use UI nodes anywhere - wrong approach

3. **Asset Path Prefix** ❌
   - Used `"assets/icons/system.png"`
   - Should be `"icons/system.png"` (Bevy adds "assets/" automatically)

4. **Gray Boxes Symptom** ❌
   - Asset not loaded yet (async loading)
   - Or Z-index issue (icons behind something)
   - Never diagnosed root cause before abandoning

### Correct Pattern for BERT

```rust
Sprite {
    image: asset_server.load("palette/system.png"),  // No "assets/" prefix!
    custom_size: Some(Vec2::new(40.0, 40.0)),
    ..default()
}
```

---

## 5. Palette Implementation Strategy

### Two Architectural Options

#### Option A: World-Space Palette (Recommended ✅)

```rust
commands.spawn((
    PaletteElement { element_type },
    Sprite {
        image: asset_server.load("palette/system.png"),
        custom_size: Some(Vec2::new(ICON_SIZE, ICON_SIZE)),
        ..default()
    },
    Transform::from_translation(Vec3::new(x, y, Z_PALETTE)),
    Name::new(format!("Palette: {:?}", element_type)),
));
```

**Advantages**:
- ✅ Consistent with existing BERT architecture (world-space sprites)
- ✅ Can use same picking/drag systems as buttons
- ✅ Simpler to implement (proven patterns)
- ✅ Works with existing Transform-based positioning

**Disadvantages**:
- ❌ Doesn't move with camera (need viewport-relative positioning)
- ❌ Not "true" screen-space UI

#### Option B: Screen-Space UI Nodes

```rust
commands.spawn((
    PaletteElement { element_type },
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(x),
        top: Val::Px(y),
        width: Val::Px(ICON_SIZE),
        height: Val::Px(ICON_SIZE),
        ..default()
    },
    ImageNode::new(asset_server.load("palette/system.png")),
));
```

**Advantages**:
- ✅ True screen-space UI (viewport-independent)
- ✅ Doesn't require camera calculations
- ✅ More standard Bevy UI approach

**Disadvantages**:
- ❌ Different from existing BERT patterns (no UI nodes in codebase)
- ❌ Requires different picking system setup
- ❌ Unknown compatibility with BERT's Transform-based architecture

**Both Analyses Recommend**: Option A for Phase 1 - Stay consistent with proven world-space architecture, migrate to UI nodes later if needed.

---

## 6. System Selection and Hierarchy

### Selection Flow

```
User Click
    ↓
MouseInteractionPlugin → PickSelection.is_selected = true
    ↓
change_focused_system (detects Changed<PickSelection>)
    ↓
FocusedSystem resource updated
    ↓
    ├─→ Button systems spawn/remove buttons
    ├─→ GeometryUpdateSet updates based on focus
    ├─→ AutoSpawnLabelSet may use focus context
    └─→ Flow systems create flows on focused system
```

### Key Distinctions

**Selected vs Focused**:
- **Selected** (`PickSelection.is_selected`): Highlighted/marked, can be multiple entities
- **Focused** (`FocusedSystem`): The ONE system whose context is active for button spawning

**Hierarchy Maintenance**:
- Subsystem → Parent: `Subsystem` component stores `parent_system: Entity`
- Interface → System: Entity parent-child relationships via `add_children()`
- Buttons: Just UI affordances, don't maintain hierarchy themselves

---

## 7. Dependency Graph

```
User Input (Mouse Click)
    ↓
MouseInteractionPlugin
    ↓
PickSelection.is_selected = true
    ↓
change_focused_system [CORE - Update schedule]
    ↓
FocusedSystem Resource Updated
    ↓
    ├─→ CreateButtonSet (9 button systems) [BUTTON-SPECIFIC - PostUpdate]
    │   ├─→ add_outflow_interface_create_button
    │   ├─→ add_inflow_interface_create_button
    │   ├─→ add_source_create_button
    │   ├─→ add_sink_create_button
    │   ├─→ add_inflow_create_button (conditional)
    │   ├─→ add_outflow_create_button (conditional)
    │   ├─→ add_interface_subsystem_create_buttons (conditional)
    │   ├─→ add_subsystem_from_external_entities_create_button
    │   └─→ remove_unfocused_system_buttons
    │
    ├─→ GeometryUpdateSet [CORE - PostUpdate]
    │   └─→ Update geometry based on focused system
    │
    ├─→ Flow Systems [CORE - Update]
    │   ├─→ select_flow_terminal
    │   └─→ Flow creation on focused system
    │
    └─→ Spawn Systems [CORE - various schedules]
        ├─→ spawn_subsystem
        └─→ on_create_button_click
```

---

## 8. Risk Assessment Matrix

| Action | Systems Affected | Risk Level | Impact | Both Analyses Agree? |
|--------|------------------|------------|--------|----------------------|
| Disable `change_focused_system` | All FocusedSystem readers (18+) | 🔴 **CRITICAL** | Core rendering breaks, buttons/subsystems/flows fail | ✅ YES |
| Disable CreateButtonSet (all 9 systems) | Button spawning only | 🟡 **MEDIUM** | No buttons, but core rendering OK if change_focused_system runs | ✅ YES |
| Disable `remove_unfocused_system_buttons` | Button cleanup | 🟡 **MEDIUM** | Buttons accumulate, performance degrades | ✅ YES |
| Keep buttons + add palette in parallel | None | 🟢 **LOW** | Safe, can coexist | ✅ YES |
| Use world-space sprites for palette | Consistency | 🟢 **LOW** | Proven pattern, minimal risk | ✅ YES |
| Use UI nodes for palette | Architecture | 🟡 **MEDIUM** | Unproven in BERT, different picking system | ✅ YES |

---

## 9. Incremental Implementation Plan

### Phase 0: Preserve Core (✅ Zero Breaking Changes)

**Keep Active**:
- `change_focused_system` (simplified - remove button validation)
- All GeometryUpdateSet systems
- All AutoSpawnLabelSet systems
- Flow creation systems
- Spawn bundles (spawn_subsystem, spawn_interface, etc.)

**Disable Safely**:
- All 9 systems in CreateButtonSet
- CreateButtonSet schedule set registration

**Expected Result**:
- ✅ No buttons spawn (expected)
- ✅ FocusedSystem updates correctly on selection
- ✅ Subsystems/flows work (use correct focused system)
- ✅ Labels render normally
- ✅ Geometry updates normally

**Success Criteria**:
- Core rendering intact
- System selection works
- Subsystem creation works (without buttons)
- Flow creation works (without buttons)

### Phase 1: Static Palette (1-2 days)

**Goal**: Render palette icons without breaking anything

**Step 1.1: Create Palette Types**

```rust
// src/bevy_app/components/palette.rs
#[derive(Component, Clone, Debug)]
pub struct PaletteElement {
    pub element_type: PaletteElementType,
    pub display_name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PaletteElementType {
    System,
    Subsystem,
    Interface(InterfaceType),
    ExternalEntity,
    Interaction,
    Substance(SubstanceType),
    Usability(InteractionUsability),
}
```

**Step 1.2: Create Palette Spawn System**

```rust
// src/bevy_app/systems/ui/palette.rs
fn spawn_palette(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let icons = vec![
        ("palette/system.png", PaletteElementType::System, "System"),
        ("palette/subsystem.png", PaletteElementType::Subsystem, "Subsystem"),
        ("palette/import.png", PaletteElementType::Interface(InterfaceType::Import), "Import Interface"),
        ("palette/export.png", PaletteElementType::Interface(InterfaceType::Export), "Export Interface"),
        ("palette/external.png", PaletteElementType::ExternalEntity, "External Entity"),
        ("palette/interaction.png", PaletteElementType::Interaction, "Interaction"),
        // ... more icons
    ];

    for (i, (path, element_type, name)) in icons.iter().enumerate() {
        commands.spawn((
            PaletteElement {
                element_type: element_type.clone(),
                display_name: name.to_string(),
                description: "".to_string(),
            },
            Sprite {
                image: asset_server.load(path),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(-400.0, 300.0 - i as f32 * 50.0, 100.0)),
            Name::new(format!("Palette: {}", name)),
        ));
    }
}
```

**Step 1.3: Register System**

```rust
// In src/bevy_app/mod.rs, add to Startup:
.add_systems(Startup, spawn_palette)
```

**Step 1.4: Verification**

- Run BERT
- Look for palette icons on left side of screen
- Verify buttons DON'T spawn (expected after Phase 0)
- Test system selection (should still work)
- Test subsystem creation (should work without buttons - direct spawning)

**Success Criteria**:
- ✅ Icons visible on screen (world-space sprites)
- ✅ No buttons (as expected)
- ✅ Can select systems
- ✅ FocusedSystem updates correctly
- ✅ No rendering errors

### Phase 2: Draggable Palette (3-4 days)

**Goal**: Enable drag-and-drop from palette to canvas

**Step 2.1: Add Picking to Palette**

```rust
fn spawn_palette(/*...*/) {
    // ... existing code ...
    commands.spawn((
        PaletteElement { /*...*/ },
        Sprite { /*...*/ },
        Transform { /*...*/ },
        PickingBehavior::default(),  // Enable picking
        Name::new(format!("Palette: {}", name)),
    ));
}
```

**Step 2.2: Implement Drag State**

```rust
#[derive(Resource, Default)]
pub struct DragState {
    pub dragging: Option<DragData>,
}

pub struct DragData {
    pub element_type: PaletteElementType,
    pub start_position: Vec2,
}
```

**Step 2.3: Handle Drag Start**

```rust
fn handle_palette_drag_start(
    mut drag_state: ResMut<DragState>,
    palette_query: Query<(&PaletteElement, &Transform), With<PaletteElement>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    // ... picking data
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        // Check if mouse over palette element
        // Set drag_state.dragging = Some(...)
    }
}
```

**Step 2.4: Render Drag Preview**

```rust
#[derive(Component)]
pub struct DragPreview;

fn render_drag_preview(
    mut commands: Commands,
    drag_state: Res<DragState>,
    preview_query: Query<Entity, With<DragPreview>>,
    asset_server: Res<AssetServer>,
    // ... mouse position
) {
    // If dragging, spawn semi-transparent preview at mouse position
    // If not dragging, despawn preview
}
```

**Step 2.5: Handle Drop**

```rust
fn handle_palette_drop(
    mut commands: Commands,
    mut drag_state: ResMut<DragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    focused_system: Res<FocusedSystem>,
    // ... spawn bundle resources
) {
    if mouse_button.just_released(MouseButton::Left) {
        if let Some(drag_data) = drag_state.dragging.take() {
            // Spawn actual element using existing spawn bundles
            match drag_data.element_type {
                PaletteElementType::Subsystem => {
                    spawn_subsystem(&mut commands, /*...*/, focused_system, /*...*/);
                }
                PaletteElementType::Interface(interface_type) => {
                    spawn_interface(&mut commands, interface_type, /*...*/, focused_system, /*...*/);
                }
                // ... other element types
            }
        }
    }
}
```

**Success Criteria**:
- ✅ Can drag palette icons
- ✅ Preview follows mouse
- ✅ Drop spawns actual element
- ✅ Elements spawn on focused system
- ✅ Existing spawn logic reused

### Phase 3: Feature Flag (2-3 days)

**Goal**: Allow toggling between button and drag workflows

**Step 3.1: Add Feature Flag**

```toml
# Cargo.toml
[features]
drag_workflow = []
button_workflow = []
default = ["button_workflow"]  # Start with buttons for safety
```

**Step 3.2: Conditional System Registration**

```rust
// In mod.rs:
#[cfg(feature = "button_workflow")]
app.add_systems(PostUpdate, (
    add_outflow_interface_create_button,
    // ... rest of button systems
).in_set(CreateButtonSet));

#[cfg(feature = "drag_workflow")]
app.add_systems(Startup, spawn_palette);

#[cfg(feature = "drag_workflow")]
app.add_systems(Update, (
    handle_palette_drag_start,
    render_drag_preview,
    handle_palette_drop,
));
```

**Step 3.3: Parallel Testing**

- Enable both features for 1-2 weeks
- Let users test drag workflow
- Collect feedback
- Ensure drag achieves same functionality as buttons

**Success Criteria**:
- ✅ Both workflows functional
- ✅ Can switch via feature flag
- ✅ No interference between workflows

### Phase 4: Deprecate Buttons (Final Step)

**When**: Drag workflow proven stable for 2+ weeks

**Actions**:
1. Change default feature to `drag_workflow`
2. Keep button workflow available via flag for 1 release
3. Remove button workflow in next major release
4. Clean up button components, assets, and systems

---

## 10. Critical Recommendations

### ✅ DO

1. **Keep `change_focused_system` running** - Simplified without button validation, but MUST remain active
2. **Implement palette alongside buttons** - Parallel development minimizes risk
3. **Use world-space sprites** - Consistent with BERT architecture, proven pattern
4. **Test incrementally** - Verify each phase before proceeding to next
5. **Convert icons to PNG** - Bevy doesn't support SVG textures
6. **Remove "assets/" prefix** - Asset paths relative to assets/ directory
7. **Feature flag workflows** - Enable gradual transition and rollback if needed
8. **Reuse existing spawn bundles** - Don't reinvent element creation logic

### ❌ DON'T

1. **Don't disable `change_focused_system`** - This breaks core rendering (root cause of V1)
2. **Don't use UI nodes yet** - BERT uses world-space, stay consistent for Phase 1
3. **Don't remove buttons until drag proven** - Keep fallback workflow active
4. **Don't guess at asset loading** - Test explicitly, check Z-index if gray boxes appear
5. **Don't skip verification** - Each phase must work before proceeding
6. **Don't modify spawn bundles** - Reuse existing, well-tested element creation
7. **Don't rush implementation** - V1 failed due to insufficient analysis

---

## 11. Key Metrics

| Metric | Value | Significance |
|--------|-------|--------------|
| FocusedSystem readers | 18+ systems | High dependency, critical resource |
| Button systems | 9 systems | All safe to disable |
| Core systems depending on FocusedSystem | 4 systems | Must preserve |
| Render pipeline depth | 3 schedules | Update → PostUpdate → RenderApp |
| Critical path length | 4 systems | Click → PickSelection → change_focused_system → FocusedSystem → Spawn |

---

## 12. Conclusion

**Analysis Convergence**: Both analyses independently identified:
- `change_focused_system` as core selection handler (not button-specific)
- 18+ systems reading FocusedSystem with clear core/button separation
- World-space sprites as BERT's architectural pattern
- V1 icon rendering failed due to wrong approach (UI nodes vs sprites)
- Safe implementation path through parallel development

**Key Takeaway**: `change_focused_system` is **misnamed** - should be `update_focused_system_on_selection`. The name suggests button association, but it's actually a core selection handler required for all FocusedSystem-dependent systems.

**Safe Path Forward**:
1. Phase 0: Disable CreateButtonSet, keep change_focused_system active (simplified)
2. Phase 1: Add static palette with world-space sprites (test icon rendering)
3. Phase 2: Add drag-and-drop behavior (reuse spawn bundles)
4. Phase 3: Feature flag both workflows (parallel testing)
5. Phase 4: Deprecate buttons only after drag proven stable

**Risk Level**: 🟢 **LOW** - With proper phasing and change_focused_system preserved, this approach is low-risk and reversible at each step.

---

*This unified analysis synthesizes findings from two independent architectural reviews to provide comprehensive understanding for drag-and-drop V2 implementation.*
