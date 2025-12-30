# Feature: Click-to-Place UI with Modal Connections (V2)

## Overview

**Feature Name**: Click-to-Place UI V2 (formerly "Drag-and-Drop UI V2")
**Branch**: `feature/drag-and-drop-v2`
**Status**: Phase 2E Complete ✅ (Optional Internal Interfaces)
**Contributors**: Claude Code
**Started**: 2025-11-01
**Last Updated**: 2025-12-29

## Description

Replaces sequential button-based workflow with freeform click-to-place UI for creating system diagrams. Users click palette icons then click canvas to place elements, with modal connection mode (F-key) for creating flows. Enables rapid iteration and reduces cognitive load from 5+ prerequisite steps to 2-click operations.

**Motivation**: Baseline measurement showed internal subsystem creation takes 6min 19sec with ~16 clicks and requires complex multi-select sequences. Click-to-place reduces to 2 clicks per element.

**Architectural Foundation**: Based on Mobus 8-tuple formalization where interfaces ∈ C (component set), making interfaces first-class placeable elements. Spatial constraints teach systems theory through interaction design (interfaces snap to boundaries, subsystems inside, environmental objects outside).

**Critical Pivot** (2025-11-03): Abandoned drag-and-drop in favor of click-to-place after discovering Bevy 0.15 sprite picking limitations. Click-to-place provides superior UX (CAD-pattern: select tool → place, vs drag gesture) and aligns with modal interaction design.

## Breakthrough: Spatial Systems Language

**Core Insight**: Theoretical rigor can be expressed through interaction design, not text requirements.

BERT's transformation from enforcement tool → learning environment:
- **Before**: "Fill out 8-tuple notation fields before modeling" (correctness over exploration)
- **After**: "3 icons + spatial constraints teach Mobus ontology through use" (discovery → formalization)

**The Pedagogical Mechanism**:
- **Icons as Vocabulary**: 3-icon palette maps to Mobus structural primitives (C, I, O)
- **Spatial Constraints as Grammar**: Interfaces snap to boundary, subsystems inside, environmental objects outside
- **Modal Workflows as Syntax**: F-key connection mode teaches "flows are edges not nodes"

Users learn Mobus formalization through muscle memory, not paper citations. Same tool supports novices ("fun modeling tool") and experts ("computational Mobus implementation").

## Implemented Functionality

### Phase 0: Preparation (Completed 2025-11-01)
- ✅ Architectural decisions documented (Layered validation + Hybrid hints)
- ✅ Feature branch created (`feature/drag-and-drop-v2`)
- ✅ Button system audit completed (18+ dependent systems mapped)
- ✅ Workflow baseline measured (6min 19sec, ~16 clicks)

### Phase 1: Foundational Refactoring (Completed 2025-11-03)

**Phase 1.1: Button System Disabled**
- ✅ Disabled CreateButtonSet systems (not deleted, preserved for potential Classic Mode)
- ✅ Preserved core selection handler (change_focused_system)
- ✅ Zero breaking changes to rendering/geometry/labels
- ✅ Commit: `b93c2bf` - "session: Phase 2D-Alpha complete, connection mode working"

**Phase 1.2-1.4: Click-to-Place Palette** (Pivot from drag-and-drop)
- ✅ 3-icon Mobus-aligned palette (Subsystem, Interface, EnvironmentalObject)
- ✅ PNG icon assets with clear semantics
- ✅ World-space sprite rendering at z=200
- ✅ Click palette → enter placement mode → click canvas to place
- ✅ Ghost preview follows cursor with ESC cancellation
- ✅ Proper components: PaletteElement, PlacementGhost, PaletteDrag events

**Phase 2A: Subsystem Placement**
- ✅ Click-to-place workflow (2 clicks: palette → canvas)
- ✅ Ghost preview with cyan tint
- ✅ Freeform placement inside system boundary
- ✅ ESC to cancel placement mode

**Phase 2B: Interface Boundary Snapping**
- ✅ Interfaces snap to nearest boundary point
- ✅ Angle-based positioning on system perimeter
- ✅ Spatial constraint teaches "interfaces live on boundaries"
- ✅ Unified Interface type (Import/Export merged per Mobus - direction belongs to flows)

**Phase 2C: EnvironmentalObject Placement**
- ✅ Freeform placement (no boundary constraints)
- ✅ Unified Source/Sink into EnvironmentalObject (Mobus set O is unified)
- ✅ Spatial constraint teaches "environment is external to system"
- ✅ Commit: `b894105` - "feat: implement environmental object placement and 3-icon palette"

**Phase 2D-Alpha: N Network (Internal Flows)** (Completed 2025-11-03)
- ✅ Modal connection workflow: Press F → click source → click destination
- ✅ Ghost line preview (cyan Gizmo from source to cursor)
- ✅ Subsystem ↔ Subsystem flow validation
- ✅ Same nesting level enforcement
- ✅ No self-connections
- ✅ ESC to exit connection mode
- ✅ Mode stays active for multiple connections
- ✅ Commit: `98d5afb` - "feat: implement subsystem-to-subsystem flow connections"

**Phase 2D-Beta: G Network (External Flows)** (Completed 2025-11-04)
- ✅ Extended connection mode for EnvironmentalObject ↔ Interface
- ✅ Bidirectional G network validation
- ✅ Invalid connection rejection with specific error messages:
  - EO ↔ EO: "no direct environment-to-environment flows"
  - EO ↔ Subsystem: "must connect to Interface per G network"
- ✅ Proper StartTargetType/EndTargetType mapping (Source/Sink for external, System for internal)
- ✅ Network type logging (N vs G) for debugging
- ✅ Commit: `4f59d1a` - "feat(connection-mode): implement G network validation"

**Phase 2E: Optional Internal Interfaces** (Completed 2025-12-29)
- ✅ Interface ↔ Interface connections enabled (previously blocked)
- ✅ N network flows no longer require interfaces for "completeness"
- ✅ Flow curve visualization fixed for Interface ↔ Interface (directions point toward each other)
- ✅ Users can now model internal flows with OR without explicit interface elements
- ✅ G network flows (external boundary) still require interfaces per Mobus
- ✅ Commit: `b1406d4` - "feat(flows): enable Interface ↔ Interface connections with proper visualization"

### Phase 2: Core Validation & UX ✅ COMPLETE (2025-11-04)
- ✅ **Structural validation** (connection mode): N and G network rules enforced
- ✅ **Keyboard shortcuts**: F-key modal mode, ESC cancellation
- ✅ **Undo system**: Ctrl+Z working for all element types with 50-command history
  - Command pattern with PlaceElementCommand
  - Event-based execution with persistent EventCursor
  - Redo (Ctrl+Shift+Z) deferred - requires spawn function refactoring
- ⏸️ **Extended shortcuts**: Power user keyboard navigation deferred to Phase 5 Polish

### Phase 3-6: Not Started
- **Phase 3**: Decomposition & Refinement (Interfaces as Subsystems, sizing, drilling)
- **Phase 4**: Intelligence & Guidance (Hints system)
- **Phase 5**: Polish & Testing
- **Phase 6**: Advanced Features (Optional)

## Mobus 8-Tuple Implementation Status

**Formal Definition**: S_{i,l} = ⟨C, N, E, G, B, T, H, Δt⟩_{i,l}

**Key Clarifications**:
- **E = ⟨O, M⟩**: Environment contains Objects (O) and Milieu (M)
- **I ⊆ C**: Interfaces are a subset of Components, contained within Boundary (B)
- **I, O, M** are NOT separate tuple elements - they are nested within C, E, and B

### ✅ Complete (6/8 Core Elements)
- **C** (Components): Subsystem + Interface click-to-place. I ⊆ C with boundary snapping.
- **N** (Network): Internal edges ⟨c_i, c_j⟩ with Subsystem ↔ Subsystem flow validation (modal F-key workflow)
- **E** (Environment): Composite element ⟨O, M⟩ fully implemented:
  - **O** (Objects): EnvironmentalObject freeform placement (unified sources/sinks)
  - **M** (Milieu): Editable key-value ambient properties (Temp, Humidity, pH, etc.)
- **G** (External Graph): Edges ⟨o_i ∈ O, c_j ∈ I⟩ with EnvironmentalObject ↔ Interface validation (modal F-key workflow)
- **B** (Boundary): Spatial clicking system, properties panel (porosity, perceptive fuzziness), contains set I
- **H** (History): Command history with undo/redo system (Ctrl+Z implemented, 50-command stack)

### 🔴 Deferred (Functional Properties - Future Work)
- **T** (Transformation): Behavioral functions/algorithms. Text field exists but needs computational editor.
- **Δt** (Timescale): Temporal resolution. Text field exists but needs simulation integration.

## Technical Implementation

### Components Added

```rust
// src/bevy_app/components.rs
#[derive(Component)]
pub struct PaletteElement {
    pub element_type: PaletteElementType,
}

#[derive(Component)]
pub struct PlacementGhost;

#[derive(Resource, Default)]
pub struct PlacementMode {
    pub active: bool,
    pub element_type: Option<PaletteElementType>,
    pub ghost_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct ConnectionMode {
    pub active: bool,
    pub source_entity: Option<Entity>,
}
```

### Systems Added

**Palette System** (`src/bevy_app/systems/palette.rs`):
- `spawn_palette_ui`: Creates 3-icon sidebar on Startup
- `enter_placement_mode`: Click palette → spawn ghost + enter mode
- `update_placement_ghost`: Ghost follows cursor
- `finalize_placement`: Click canvas → spawn element at position

**Connection Mode System** (`src/bevy_app/systems/connection_mode.rs`):
- `enter_connection_mode`: F-key → enter modal connection mode
- `select_connection_source`: First click stores source entity
- `update_connection_ghost`: Gizmo line from source to cursor
- `finalize_connection`: Second click validates + creates flow edge

### Architecture Decisions

**1. Click-to-Place over Drag-and-Drop**
- **Rationale**: Bevy 0.15 sprite entities don't support picking events, would require mesh conversion
- **UX Benefit**: CAD-pattern (select tool → place) superior to drag gesture for precision work
- **Pattern**: Tool selection (palette click) → application (canvas click) → ESC to cancel

**2. 3-Icon Palette (Mobus Minimal)**
- **Subsystem**: Component within system (C ∈ C set)
- **Interface**: Boundary component that snaps to perimeter (I ∈ C, lives on B)
- **EnvironmentalObject**: External entity (O ∈ O set), unified Source/Sink per Mobus
- **Flows**: Created via modal connection mode (edges in N and G, not palette items)

**3. Modal Connection Mode (F-Key)**
- **Rationale**: Flows are edges (relationships), not nodes (entities) - can't be "placed"
- **UX**: F → click source → click destination → flow created with validation
- **Advantage**: Clear workflow for directed edges, prevents accidental flow creation
- **Pattern**: Stays active for multiple connections, ESC to exit

**4. Spatial Constraints as Teaching**
- **Interfaces snap to boundary**: Kinesthetic learning "interfaces live on boundaries"
- **Subsystems inside only**: Experience "components exist within system"
- **Environmental objects outside**: Learn "environment is external"
- **Validation during connection**: Real-time Mobus G/N network enforcement

**5. Layered Validation Philosophy**
- **Structural (Strict)**: Prevents breaking operations (e.g., can't place interface in space)
- **Mobus (Advisory)**: Educates on systems science principles (future hints system)
- **Pattern**: Hard constraints via spatial/connection rules, soft guidance via future hints

## Usage Examples

### Basic Element Placement
```rust
// User workflow:
// 1. Click Subsystem icon in palette
// 2. Click inside system boundary on canvas
// 3. Subsystem spawned at click position

// System: enter_placement_mode (triggered on palette click)
// System: update_placement_ghost (ghost follows cursor)
// System: finalize_placement (spawns element on canvas click)
```

### Creating Flows (N Network)
```rust
// User workflow:
// 1. Press F key (enter connection mode)
// 2. Click first subsystem (source)
// 3. Click second subsystem (destination)
// 4. Flow created with validation (N network rules)

// Validation checks:
// - Both must be Subsystems
// - Same nesting level
// - No self-connections
```

### Creating Flows (G Network)
```rust
// User workflow:
// 1. Press F key
// 2. Click EnvironmentalObject (source)
// 3. Click Interface (destination)
// 4. Flow created with G network validation

// Validation checks:
// - EnvironmentalObject ↔ Interface only (bidirectional)
// - Rejects EO ↔ Subsystem (violates G network definition)
// - Rejects EO ↔ EO (no environment-to-environment flows)
```

## Testing Strategy

### Manual Testing Completed
- ✅ Palette icon rendering (3 icons visible, correct positioning)
- ✅ Click-to-place workflow (all 3 element types)
- ✅ Ghost preview following cursor
- ✅ Interface boundary snapping (angles calculated correctly)
- ✅ Connection mode entry/exit (F key and ESC)
- ✅ N network flows (Subsystem ↔ Subsystem)
- ✅ G network flows (EnvironmentalObject ↔ Interface bidirectional)
- ✅ Invalid connection rejection (EO↔EO, EO↔Subsystem)
- ✅ Interface↔Interface connections (enabled Phase 2E)
- ✅ Self-connection prevention
- ✅ Nesting level validation

### Known Issues
- ⚠️ **Interface ↔ Subsystem flows not yet implemented**: Pending Phase 3 "Interfaces as Subsystems" refactor. Workaround: use Subsystem ↔ Subsystem flows. Theoretically valid (both are components in C), but requires architectural work to handle Interface's dual role as boundary component and internal node.

### Automated Testing (Future)
- [ ] Unit tests for placement validation
- [ ] Unit tests for connection validation (N and G networks)
- [ ] Integration tests for full workflow
- [ ] Performance tests with 50+ elements
- [ ] Regression tests for existing model loading

## Future Improvements

### Immediate (Mobus 8-Tuple Completion)
- Add M (Milieu) panel for ambient environmental properties
- Add H (History/Memory) panel for state variables per subsystem
- Make boundary (B) explicitly selectable with properties panel
- Implement algorithmic protocol editor (φ)

### Phase 2 Completion Notes
- ✅ Undo system (Ctrl+Z) fully functional
- 🔜 Redo (Ctrl+Shift+Z) deferred - keyboard detection works, respawn requires architecture refactoring
- ⏸️ Extended keyboard shortcuts deferred to Phase 4 Polish
- ⏸️ Visual error indicators deferred to Phase 4 Polish

### Phase 3-5 (Intelligence & Polish)
- Model state analysis system (detect incomplete structures)
- Completion hint generator (AI-style suggestions)
- Hint display panel with actionable suggestions
- Mobus validation feedback (advisory warnings)
- Smart parameter templates
- Animation & transitions
- Performance optimization for 100+ element models

### Advanced Features (Optional)
- Classic Mode toggle (restore button-based UI for legacy users)
- Model templates (common patterns library)
- Collaborative hints with LLM integration

## Complete 5-Phase Roadmap

### Phase 0: Preparation (2-3 hours) ✅ COMPLETE
- Architectural decisions
- Feature branch creation
- Button system audit
- Workflow baseline measurement

### Phase 1: Foundational Refactoring (23 hours) ✅ COMPLETE
- Button system deletion
- Palette component creation → **PIVOT: 3-icon Mobus palette**
- Drag-from-palette → **PIVOT: Click-to-place with ghost**
- Canvas drop zones → **PIVOT: Modal placement mode**
- Details panel redesign → **DEFERRED**
- **BONUS**: Phase 2A-D (Subsystem, Interface, EnvironmentalObject placement + N/G network flows)

### Phase 2: Core Validation & UX (16 hours) ✅ COMPLETE
- ✅ Structural validation (connection mode)
- ⏸️ Highlight valid drop targets (N/A for click-to-place)
- ✅ Undo/redo foundation (Ctrl+Z working, redo deferred)
- ✅ Keyboard shortcuts (F-key, ESC)

### Phase 3: Decomposition & Refinement (TBD hours) ⏸️ NOT STARTED
**Rationale**: Phase 2 optimized creation workflow but regressed refinement workflow.
Old main had smooth recursive decomposition; new click-to-place needs decomposition polish.

Priority issues:
- **Interfaces as Subsystems**: Enable Interface ↔ Subsystem flows (Mobus: "interfaces are special subsystems")
  - Architectural challenge: Interface's dual role (boundary component + internal node)
  - Required for proper import/export process modeling
- **System Decomposition Polish**:
  - Subsystem sizing (currently too small, no dynamic resize)
  - Zoom-aware rendering and interaction
  - Smooth recursive drilling (focus → decompose → focus child)
  - Visual hierarchy (nested systems need clearer parent/child relationships)

### Phase 4: Intelligence & Guidance (24 hours) ⏸️ NOT STARTED
- Model state analysis system
- Completion hint generator
- Hint display panel
- Mobus validation feedback
- Smart parameter templates

### Phase 5: Polish & Testing (25 hours) ⏸️ NOT STARTED
- Animation & transitions
- Error visualization
- Comprehensive testing
- Documentation updates
- Performance optimization

### Phase 6: Advanced Features (Optional) ⏸️ NOT STARTED
- Classic mode toggle
- Model templates

### 2.6+ Backlog (Deferred from 2.5)
- **Cross-level Interface connections**: Parent boundary interface → child interface/subsystem
  - Currently blocked by nesting level validation (N network same-level requirement)
  - Theoretically valid per Mobus (boundary interface visible to internal elements)
  - Implementation complex: flow parenting, curve rendering across levels
- **InterfaceSubsystem button**: Reactivate convenience feature from old workflow
  - Spawns subsystem "behind" interface automatically
  - Workaround exists: manually place subsystem and connect
- Collaborative hints (AI)

## Related Documentation

- **Session Files** (halcyonic workspace):
  - `operations/sessions/2025-11-01/bert-transition-planning-session.md` - Original 5-phase roadmap
  - `operations/sessions/2025-11-03/bert-phase2-reference.md` - Detailed implementation notes (Phase 2A-D)
  - `operations/sessions/2025-11-03/daily-recap.md` - Spatial systems language breakthrough
  - `operations/sessions/2025-11-04/bert-phase2d-beta-session.md` - G network implementation
- **Architecture Docs**:
  - `docs/architecture/button-system-analysis-unified.md` - Button system dependency analysis
  - `docs/architecture/phase1-lessons-learned.md` - V1 branch failure lessons
  - `docs/architecture/icon-integration-lessons.md` - Icon rendering learnings
- **BERT Documentation**:
  - [System Language](https://bert.gitbook.io/bert-documentation/system-language) - Mobus 8-tuple formalization
  - [Contributing Guide](CONTRIBUTING.md) - Development workflow

## Commits

**Phase 0-1**:
- `b93c2bf` - session: Phase 2D-Alpha complete, connection mode working
- `b894105` - feat: implement environmental object placement and 3-icon palette
- `98d5afb` - feat: implement subsystem-to-subsystem flow connections

**Phase 2D-Beta (G Network)**:
- `4f59d1a` - feat(connection-mode): implement G network validation for EnvironmentalObject ↔ Interface flows

**Phase 2 (Undo/Redo)**:
- `d123d8f` - feat(undo-redo): implement command pattern foundation
- `2a1f872` - feat(undo-redo): integrate PlaceElementCommand in palette placement workflow
- `a7b760f` - feat(undo-redo): implement event-based undo/redo execution
- `c5d66a4` - fix(undo-redo): use persistent EventCursor to prevent multiple undos per keypress
- `dbecde2` - docs(undo-redo): document redo implementation challenge

---

**Last Updated**: 2025-11-04
**Branch Status**: Phase 2 Complete ✅ | 6/8 Mobus elements implemented | Next: Phase 3 (Decomposition & Refinement - Interfaces as Subsystems)
