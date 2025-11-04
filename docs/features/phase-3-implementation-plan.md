# Phase 3 Implementation Plan: Decomposition & Refinement

**Date**: 2025-11-04
**Branch**: `feature/drag-and-drop-v2`
**Context**: Phase 2 complete (click-to-place creation working), Phase 3 restores decomposition workflow

---

## Branch Comparison Summary

### Key Finding: Minimal Divergence Between Branches

**spawn/subsystem.rs**: IDENTICAL except return type change (void → Entity)
- Main returns void, feature returns Entity (needed for undo system)
- No changes to sizing logic, radius calculations, or interface spawning

**subsystem.rs** (systems): IDENTICAL across both branches
- `update_subsystem_radius_from_interface_count` unchanged
- `SUBSYSTEM_MIN_SCALING_FACTOR` (4%) and `SUBSYSTEM_SCALING_FACTOR` (30%) constants unchanged
- Interface count scaling formula unchanged

**Conclusion**: "Tiny subsystems" issue is NOT caused by code regression. Issue is likely:
1. **User workflow changed**: Old button system auto-created interfaces during decomposition, new palette requires manual interface creation
2. **Zoom context**: User may be viewing at different zoom level, making 4% subsystems appear smaller
3. **No interfaces**: Subsystems start at 4% radius and grow to 30% as interfaces added (0-8 interfaces)

### Architecture Preserved from Main

**Focus System** (`change_focused_system`):
- Selection → FocusedSystem resource update working correctly
- Subsystem placement depends on FocusedSystem (working in Phase 2)

**Zoom System** (`NestingLevel::compute_scale`):
- Formula: `SUBSYSTEM_SCALING_FACTOR^nesting_level × zoom` (max 1.0)
- Scale affects rendering but NOT base radius calculations
- Subsystem radius = `parent_radius × scaling_factor` (where scaling_factor ∈ [4%, 30%])

**Interface-Driven Sizing** (`update_subsystem_radius_from_interface_count`):
- Regular subsystems: Scale from 4% to 30% based on interface count (0-8)
- Interface subsystems: Fixed at 4% regardless of interfaces
- System runs on `Added<Interface>` and `RemoveEvent`

---

## Root Cause Analysis

### Issue 1: "Subsystems are really tiny and don't change sizes"

**Cause**: User creating subsystems without interfaces
- Subsystems spawn at **4% of parent radius** by design
- Grow to **30% as interfaces added** (linear scaling 0-8 interfaces)
- In old button workflow: Decomposition auto-created interfaces with subsystem
- In new palette workflow: User must manually add interfaces after subsystem creation

**Evidence**:
```rust
// Line 146-154 of subsystem.rs (UNCHANGED between branches)
let mut scaling_factor = if interface_subsystem.is_some() {
    SUBSYSTEM_MIN_SCALING_FACTOR  // 4% - interface subsystems stay tiny
} else {
    // Regular subsystems scale with interface count
    SUBSYSTEM_MIN_SCALING_FACTOR
        + interface_count as f32 * (SUBSYSTEM_SCALING_FACTOR - SUBSYSTEM_MIN_SCALING_FACTOR)
            / SUBSYSTEM_FULL_SIZE_INTERFACE_COUNT
};
```

### Issue 2: Interface ↔ Subsystem Flows Blocked

**Cause**: Intentional validation in connection_mode.rs (lines 231-232)
```rust
} else if (source_is_interface && dest_is_subsystem) || (source_is_subsystem && dest_is_interface) {
    warn!("❌ Interface ↔ Subsystem flows not yet implemented (pending Phase 3 'Interfaces as Subsystems' refactor)");
```

**Context**: Phase 2D explicitly deferred this pending architectural refactor
- Mobus theory: "Interfaces are special subsystems" (I ⊆ C)
- Current architecture: Interface and Subsystem are distinct components
- Required for: Import/export process modeling, proper decomposition

### Issue 3: "Other issues" (Decomposition Workflow)

**Hypothesis**: Missing smooth recursive drilling
- Old workflow: Button on subsystem → decompose → auto-focus child → show interfaces
- New workflow: Click subsystem → focus → manually add elements
- No visual feedback for "this subsystem is ready to decompose"
- No zoom-on-focus behavior to make nested editing comfortable

---

## Architectural Decisions

### Decision 1: Interfaces as Subsystems Architecture

**Problem**: Interface and Subsystem are currently distinct, preventing Interface ↔ Subsystem flows

**Mobus Foundation**:
- I ⊆ C (interfaces are special subsystems)
- Interfaces have dual role: boundary component + internal node
- Example: HTTP endpoint is both interface to client AND subsystem handling requests

**Architectural Options**:

**Option A: Component Composition (RECOMMENDED)**
```rust
// Add InterfaceBehavior marker to existing subsystems
struct InterfaceBehavior {
    substance_type: SubstanceType,
    protocol: String,
}

// Interface becomes: Subsystem + InterfaceBehavior
// Query for interfaces: Query<&Subsystem, With<InterfaceBehavior>>
```
**Pros**: Minimal disruption, preserves existing spawn functions, clear separation
**Cons**: Queries need updating, need migration path for existing models

**Option B: Trait-based Polymorphism**
```rust
trait BoundaryComponent { ... }
impl BoundaryComponent for Subsystem { ... }
impl BoundaryComponent for Interface { ... }
```
**Pros**: Theoretically elegant
**Cons**: Bevy ECS doesn't support trait queries, major refactor required

**Option C: Unified Component with Enum**
```rust
struct SystemComponent {
    kind: ComponentKind,  // Subsystem | Interface | Both
    ...
}
```
**Pros**: Single query
**Cons**: Over-abstraction, loses type safety, hard to migrate

**RECOMMENDATION**: **Option A (Component Composition)**
- Incremental migration path: Add InterfaceBehavior to existing Interfaces
- Connection validation becomes: "Allow any two entities with Subsystem component"
- Preserve existing spawn functions with optional InterfaceBehavior parameter
- Data model backward compatible: Load old Interface as Subsystem + InterfaceBehavior

**Migration Strategy**:
1. Phase 3A: Add InterfaceBehavior component, attach to existing Interfaces (additive)
2. Phase 3B: Update connection_mode to allow Subsystem ↔ Subsystem (includes interfaces)
3. Phase 3C: Update spawn_interface to create Subsystem + InterfaceBehavior
4. Phase 3D: Gradual query updates across codebase (as needed)

### Decision 2: Subsystem Sizing Restoration

**Problem**: Users creating subsystems without interfaces (4% radius feels "tiny")

**Options**:

**Option A: Educate Through Hints (RECOMMENDED)**
- Keep existing 4%-30% scaling behavior (correct by design)
- Add visual hints: "Add interfaces to grow subsystem" when hovering tiny subsystem
- Phase 4 intelligence layer will guide: "This subsystem needs interfaces"

**Option B: Change Default Size**
- Start subsystems at 15% instead of 4%
- Pros: Immediate visual improvement
- Cons: Violates Mobus principle (size should reflect interface count)

**Option C: Auto-Create Placeholder Interfaces**
- Spawn subsystem with 2 default interfaces (Import/Export)
- Pros: Subsystems immediately look "right-sized"
- Cons: Creates unwanted interfaces, violates user intent

**RECOMMENDATION**: **Option A (Educate Through Hints)**
- Existing behavior is theoretically correct (size = f(interface_count))
- Issue is pedagogical, not architectural
- Phase 4 will add intelligence layer for guidance
- Quick win: Add tooltip on tiny subsystems in Phase 3

### Decision 3: Zoom-Aware Decomposition Polish

**Problem**: Missing smooth recursive drilling from old workflow

**Current Zoom Behavior**:
- NestingLevel computed: `parent_nesting_level + 1`
- Scale computed: `SUBSYSTEM_SCALING_FACTOR^nesting_level × zoom` (clamped to 1.0)
- Zoom resource global: User manually zooms in/out

**Missing Workflow Elements**:
1. **Auto-zoom on focus**: When focusing nested subsystem, zoom in to comfortable scale
2. **Visual hierarchy**: Nested elements should fade/scale to show depth
3. **Breadcrumb navigation**: Show parent chain for context
4. **Smooth transitions**: Animate zoom + pan when drilling down

**Implementation Priority**:
- **Phase 3B**: Auto-zoom on focus (high impact, low complexity)
- **Phase 3C**: Visual hierarchy (medium impact, medium complexity)
- **Phase 4**: Breadcrumbs + animations (polish, deferred)

**Auto-Zoom Strategy**:
```rust
// When FocusedSystem changes to a subsystem:
// 1. Calculate desired zoom: Make focused system ~300px radius on screen
// 2. Calculate pan: Center focused system in viewport
// 3. Animate Zoom resource + Camera transform over 0.3 seconds
```

---

## Implementation Roadmap

### Phase 3A: Interface as Subsystem Foundation (8 hours)
**Goal**: Enable Interface ↔ Subsystem flows through component composition

**Files to Modify**:
- `src/bevy_app/components/mod.rs` or `components/system_elements.rs`: Add `InterfaceBehavior` component
- `src/bevy_app/bundles/spawn/interface.rs`: Attach `InterfaceBehavior` to existing `Interface` spawns
- `src/bevy_app/systems/connection_mode.rs`: Update validation (lines 216-237)

**Approach**:
1. Define `InterfaceBehavior` component:
```rust
#[derive(Component, Debug, Clone, Reflect)]
pub struct InterfaceBehavior {
    pub substance_type: SubstanceType,
    pub protocol: String,
}
```

2. Update `spawn_interface` to add component:
```rust
// After spawning Interface entity, also insert:
commands.entity(interface_entity).insert(InterfaceBehavior {
    substance_type,
    protocol,
});
```

3. Update connection_mode validation:
```rust
// BEFORE: Explicit type checks (source_is_interface, dest_is_subsystem, etc.)
// AFTER: Check for Subsystem component (includes interfaces now)
let source_is_system = subsystem_query.get(source_entity).is_ok();
let dest_is_system = subsystem_query.get(destination_entity).is_ok();

let is_valid_n_network = source_is_system && dest_is_system;
// G network validation unchanged (ExternalEntity ↔ Interface)
```

**Testing**:
1. Load existing model → Verify interfaces still render correctly
2. Create new interface → Verify InterfaceBehavior attached
3. Create flow Interface → Subsystem → Should succeed (no longer blocked)
4. Create flow Subsystem → Interface → Should succeed
5. Verify flow rendering updates correctly

**Success Criteria**:
- ✅ Interface ↔ Subsystem flows create successfully
- ✅ Existing models load without errors
- ✅ New interfaces spawn with both Interface + InterfaceBehavior components
- ✅ Connection mode allows all valid Mobus N-network connections

**Migration Notes**:
- Old models: Interface component exists without InterfaceBehavior (queries handle gracefully)
- Need migration system in Phase 3B to backfill InterfaceBehavior for old interfaces

---

### Phase 3B: Auto-Zoom on Focus (6 hours)
**Goal**: Automatically zoom + pan camera when focusing nested subsystems

**Files to Modify**:
- `src/bevy_app/systems/ui/zoom.rs`: Add `auto_zoom_on_focus_change` system
- `src/bevy_app/resources.rs`: Add `ZoomTarget` resource for animation state
- `src/bevy_app/lib.rs`: Register new system in appropriate set

**Approach**:
1. Detect focus change:
```rust
pub fn auto_zoom_on_focus_change(
    focused_system: Res<FocusedSystem>,
    mut previous_focus: Local<Option<Entity>>,
    system_query: Query<(&Transform, &System, &NestingLevel)>,
    mut zoom_target: ResMut<ZoomTarget>,
) {
    if focused_system.is_changed() && Some(**focused_system) != *previous_focus {
        // Calculate target zoom to make focused system ~300px radius
        let (transform, system, nesting_level) = system_query.get(**focused_system).unwrap();
        let desired_screen_radius = 300.0;
        let target_zoom = desired_screen_radius / system.radius;

        zoom_target.target_zoom = target_zoom.clamp(0.1, 10.0);
        zoom_target.target_pan = transform.translation.truncate();
        zoom_target.animating = true;

        *previous_focus = Some(**focused_system);
    }
}
```

2. Animate zoom resource:
```rust
pub fn animate_zoom_to_target(
    time: Res<Time>,
    mut zoom: ResMut<Zoom>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut zoom_target: ResMut<ZoomTarget>,
) {
    if !zoom_target.animating { return; }

    let t = (time.delta_seconds() / ZOOM_ANIMATION_DURATION).min(1.0);

    // Lerp zoom
    **zoom = zoom.lerp(zoom_target.target_zoom, t);

    // Lerp camera pan
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = camera_transform.translation
        .truncate()
        .lerp(zoom_target.target_pan, t)
        .extend(camera_transform.translation.z);

    if t >= 1.0 {
        zoom_target.animating = false;
    }
}
```

**Testing**:
1. Focus root system → Verify zoomed out to show full system
2. Focus nested subsystem → Camera zooms + pans to center it
3. Focus another subsystem at same level → Camera pans smoothly
4. Focus parent again → Zoom out to show parent context
5. Rapid focus changes → Verify no animation conflicts

**Success Criteria**:
- ✅ Focusing nested subsystem zooms to comfortable editing scale
- ✅ Camera pans to center focused system
- ✅ Smooth animation (no jarring jumps)
- ✅ Doesn't interfere with manual zoom/pan controls

**Edge Cases**:
- User zooms manually during animation → Cancel animation, respect user control
- Focus change while animating → Retarget to new focus
- Subsystem at extreme nesting level → Clamp zoom to reasonable bounds

---

### Phase 3C: Subsystem Sizing Polish + Visual Feedback (6 hours)
**Goal**: Add visual hints for tiny subsystems + improve hierarchy visibility

**Files to Modify**:
- `src/bevy_app/systems/ui/hints.rs`: New system for hover hints (create file)
- `src/leptos_app/components/status_bar.rs`: Add subsystem stats display
- `src/bevy_app/systems/ui/zoom.rs`: Add depth-based opacity scaling

**Approach**:

**Part 1: Hover Hints for Tiny Subsystems**
```rust
pub fn show_subsystem_hints(
    subsystem_query: Query<(Entity, &System, &Children, &GlobalTransform), With<Subsystem>>,
    interface_query: Query<&Interface>,
    hover_query: Query<Entity, With<HoveredEntity>>,  // Assume hover detection exists
    mut hint_display: ResMut<HintDisplay>,
) {
    for hovered in hover_query.iter() {
        if let Ok((_, system, children, transform)) = subsystem_query.get(hovered) {
            let interface_count = children.iter()
                .filter(|c| interface_query.get(**c).is_ok())
                .count();

            // Show hint if subsystem is small (< 10% parent radius)
            if interface_count < 3 && system.radius < parent_radius * 0.1 {
                hint_display.show(
                    "Add interfaces to grow this subsystem (4% → 30% for 8 interfaces)",
                    transform.translation()
                );
            }
        }
    }
}
```

**Part 2: Status Bar Subsystem Stats**
```rust
// In Leptos status bar component:
"Selected: {subsystem_name} | Interfaces: {interface_count}/8 | Size: {size_percent}%"
```

**Part 3: Depth-Based Visual Hierarchy**
```rust
pub fn update_nesting_visual_hierarchy(
    mut element_query: Query<(&mut Sprite, &NestingLevel)>,
    focused_system: Res<FocusedSystem>,
    nesting_query: Query<&NestingLevel>,
) {
    let focused_nesting = nesting_query.get(**focused_system)
        .map(|n| **n)
        .unwrap_or(0);

    for (mut sprite, nesting_level) in element_query.iter_mut() {
        let depth_delta = (**nesting_level as i16) - (focused_nesting as i16);

        // Fade elements not at focused depth
        let opacity = match depth_delta {
            0 => 1.0,      // Focused level: full opacity
            1 => 0.7,      // One level deeper: slightly faded
            -1 => 0.5,     // Parent level: more faded
            _ => 0.3,      // Further away: heavily faded
        };

        sprite.color.set_alpha(opacity);
    }
}
```

**Testing**:
1. Create subsystem without interfaces → Hover shows hint
2. Add 1 interface → Subsystem grows, hint updates
3. Add 8 interfaces → Subsystem reaches 30%, hint disappears
4. Focus nested subsystem → Parents fade, children fade, focused clear
5. Status bar shows correct interface count and size percentage

**Success Criteria**:
- ✅ Hover hint explains why subsystem is small
- ✅ Status bar shows subsystem stats
- ✅ Depth-based fading makes hierarchy visible
- ✅ No performance impact on large models

---

### Phase 3D: Data Model Migration for InterfaceBehavior (4 hours)
**Goal**: Ensure old saved models load correctly with new component architecture

**Files to Modify**:
- `src/bevy_app/data_model/mod.rs`: Add migration logic
- `src/bevy_app/systems/load.rs`: Run migration on model load

**Approach**:
```rust
pub fn migrate_interfaces_to_subsystems(
    mut commands: Commands,
    interface_query: Query<(Entity, &Interface), Without<InterfaceBehavior>>,
) {
    for (entity, interface) in interface_query.iter() {
        // Backfill InterfaceBehavior for old interfaces
        commands.entity(entity).insert(InterfaceBehavior {
            substance_type: SubstanceType::default(),  // Or infer from connected flows
            protocol: interface.protocol.clone(),
        });

        info!("Migrated Interface entity {:?} to new architecture", entity);
    }
}
```

**Testing**:
1. Load pre-Phase-3 model → All interfaces gain InterfaceBehavior
2. Save model → InterfaceBehavior persists
3. Load saved model → No migration needed (already has component)
4. Mixed model (some migrated, some new) → All interfaces work correctly

**Success Criteria**:
- ✅ All old models load without errors
- ✅ Interfaces in old models gain InterfaceBehavior automatically
- ✅ No data loss during migration
- ✅ Resave doesn't corrupt models

---

## Risk Analysis

### High Risk: Interface Component Architecture Change

**Risk**: Queries throughout codebase expect Interface component, may break
**Mitigation**:
- Phase 3A is additive (adds InterfaceBehavior, doesn't remove Interface)
- Gradual query migration over Phase 3B-D as issues discovered
- Comprehensive testing with existing models before merge

**Rollback Plan**: Remove InterfaceBehavior component, revert connection_mode changes

---

### Medium Risk: Auto-Zoom Conflicts with User Control

**Risk**: Auto-zoom animates while user is manually zooming/panning
**Mitigation**:
- Detect user input (scroll, drag) → Cancel animation immediately
- Add toggle in settings: "Auto-zoom on focus change" (default: ON)
- Animation duration short (0.3s) to minimize interference

**Rollback Plan**: Disable auto_zoom_on_focus_change system if issues persist

---

### Low Risk: Hover Hints Performance

**Risk**: Hover detection + hint calculation on every frame
**Mitigation**:
- Only run when mouse moves (change detection on cursor position)
- Cache hint text, only recalculate on subsystem changes
- Limit to one hint at a time (closest to cursor)

**Rollback Plan**: Disable hover hint system if performance degrades

---

## Phase 3 Success Criteria

**Functional Requirements**:
- ✅ Interface ↔ Subsystem flows work (create + render correctly)
- ✅ Auto-zoom on focus change provides comfortable editing scale
- ✅ Hover hints explain subsystem sizing behavior
- ✅ Status bar shows subsystem statistics
- ✅ Depth-based visual hierarchy shows nesting clearly

**Non-Functional Requirements**:
- ✅ Old models load and migrate automatically
- ✅ No performance regression on large models (100+ elements)
- ✅ Phase 2 click-to-place UX preserved (no regressions)

**User Experience**:
- ✅ Decomposition workflow feels smooth and intuitive
- ✅ Visual feedback guides user toward correct modeling
- ✅ Nested editing comfortable (auto-zoom + visual hierarchy)

---

## Next Steps After Phase 3

**Phase 4: Intelligence & Guidance** (24 hours) - Deferred
- Model state analysis system
- Completion hint generator (leveraging Phase 3C hover system)
- Mobus validation feedback
- Smart decomposition suggestions

**Phase 5: Polish & Performance** (TBD)
- Undo/redo for all operations (Phase 3A creates foundation)
- Keyboard shortcuts for palette + connection mode
- Animation polish (smooth zoom curves, element spawn/despawn)
- Large model optimization (100+ elements)

---

## Notes

**Design Philosophy**: Phase 3 restores **refinement workflow** degraded during Phase 2 creation workflow optimization. Goal is balanced UX: fast creation (Phase 2) + smooth refinement (Phase 3).

**Theoretical Alignment**: Interface as Subsystem (Phase 3A) is critical for computational Mobus implementation. Enables proper import/export process modeling per Mobus 8-tuple formalization.

**Pedagogical Focus**: Phase 3C hints teach users "subsystem size = f(interface count)" through interaction, not documentation. Spatial constraints continue teaching systems theory through use.
