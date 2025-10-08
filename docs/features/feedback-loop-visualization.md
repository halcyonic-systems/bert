# Feature: Feedback Loop Visualization

## Overview

**Feature Name**: Feedback Loop Visualization Enhancement
**Branch**: feature/feedback-visualization-simple (archived)
**Status**: Specification Preserved (Not Implemented in Main)
**Contributors**: BERT Development Team
**Date**: 2025-09-06 (Archived: 2025-10-08)

## Description

This feature transforms BERT's equivalence system (numbered badges) into comprehensive visual feedback loop representation. Users can establish equivalence relationships between sources and sinks using the "E" key, and see curved visual connections that clearly show feedback paths around the system boundary, enabling systems scientists to model and analyze cybernetic control systems effectively.

## System Language Integration

Feedback connections implement the "N" (internal interactions) component of System Language theory by visualizing how system outputs feed back as inputs through environmental coupling. This supports modeling of:
- **Negative feedback loops**: Self-regulating, stabilizing systems
- **Positive feedback loops**: Self-reinforcing, amplifying behaviors
- **Complex feedback hierarchies**: Nested control systems and loop-within-loop scenarios

## Implemented Functionality

### Core Features
- **Visual Feedback Connections**: Curved Bezier lines connecting external entities with matching `IsSameAsId` values
- **Smart Curve Routing**: Curves automatically route over/under system boundaries with proper clearance
- **Dynamic Updates**: Connections update in real-time when entities are moved or equivalences change
- **Performance Optimized**: Change-based updates only, avoiding frame-by-frame recreation
- **Visual Distinction**: Cyan color (#00FFFF) and semi-transparent rendering distinguishes feedback from regular flows
- **Automatic Cleanup**: Connections removed when equivalence relationships are deleted

### User Workflow
1. Create source and sink external entities in system environment
2. Select both entities using Shift+click multi-selection
3. Press **'E' key** to establish equivalence relationship
4. Visual feedback connection appears automatically with proper curve routing
5. Drag entities to see connection update dynamically
6. Press 'E' again to remove equivalence and connection

## Technical Implementation

### Architecture Overview

**Design Philosophy**: Minimal integration approach that extends existing `IsSameAsId` system rather than creating complex plugin architecture. Follows BERT's pattern of extending existing systems rather than replacing them.

**Performance Strategy**: Change-based updates triggered only when `IsSameAsId` changes or entities move, preventing unnecessary frame-by-frame overhead.

### Component Structure

```rust
/// Component tracking visual feedback connections between equivalent external entities.
#[derive(Component)]
pub struct FeedbackConnection {
    /// Source entity in the feedback relationship
    pub from: Entity,
    /// Target entity in the feedback relationship
    pub to: Entity,
    /// Equivalence ID linking this connection to IsSameAsId values
    pub equivalence_id: usize,
}
```

### System Architecture (3 Systems)

#### 1. `draw_feedback_connections` System
**Purpose**: Create visual connections when equivalence relationships are established

**Trigger**: Runs when `IsSameAsId` component changes

**Algorithm**:
1. Query all external entities with `Changed<IsSameAsId>`
2. Group entities by their equivalence ID
3. For each group with 2+ entities, create pairwise connections
4. Check existing connections to avoid duplicates
5. Spawn connection entities with curved Bezier paths

**Query Signature**:
```rust
pub fn draw_feedback_connections(
    mut commands: Commands,
    external_entities: Query<
        (Entity, &Transform, &IsSameAsId, &NestingLevel),
        (With<ExternalEntity>, Changed<IsSameAsId>),
    >,
    existing_connections: Query<&FeedbackConnection>,
)
```

#### 2. `update_feedback_connections` System
**Purpose**: Update connection paths when entities are moved

**Trigger**: Runs when `Transform` of entities with feedback connections changes

**Algorithm**:
1. Query feedback connections and their referenced entities
2. Check if either endpoint entity has moved (`Changed<Transform>`)
3. Recalculate Bezier curve path with updated positions
4. Update Lyon path geometry for smooth visual updates

**Query Signature**:
```rust
pub fn update_feedback_connections(
    mut connections: Query<(&FeedbackConnection, &mut Path)>,
    transforms: Query<&Transform, Changed<Transform>>,
)
```

#### 3. `cleanup_feedback_connections` System
**Purpose**: Remove connections when equivalence relationships are deleted

**Trigger**: Runs when entities with `FeedbackConnection` are despawned or equivalence changes

**Algorithm**:
1. Query all feedback connections
2. Check if referenced entities still exist
3. Verify equivalence IDs still match
4. Despawn orphaned connections

### Curve Routing Algorithm

**Clearance-Based Control Points**: Control points calculated from system boundary edges rather than center, ensuring proper visual clearance around system boundaries.

**Routing Logic**:
- **Both entities above system center** → Curve over the top
- **Both entities below system center** → Curve under the bottom
- **Mixed positions** → Default to over the top

**Parameters**:
```rust
const SYSTEM_RADIUS: f32 = 300.0;       // Typical BERT system radius
const BASE_CLEARANCE: f32 = 120.0;      // Minimum clearance above/below system
const DISTANCE_FACTOR: f32 = 0.3;       // Additional curve height based on separation
```

**Control Point Calculation**:
```rust
// Pseudocode for curve routing
let midpoint_y = (from_pos.y + to_pos.y) / 2.0;
let system_center_y = 0.0;
let horizontal_distance = (to_pos.x - from_pos.x).abs();

let curve_over_top = from_pos.y > system_center_y && to_pos.y > system_center_y;
let curve_under_bottom = from_pos.y < system_center_y && to_pos.y < system_center_y;

let control_y = if curve_over_top {
    SYSTEM_RADIUS + BASE_CLEARANCE + (horizontal_distance * DISTANCE_FACTOR)
} else if curve_under_bottom {
    -(SYSTEM_RADIUS + BASE_CLEARANCE + (horizontal_distance * DISTANCE_FACTOR))
} else {
    // Default to over top for mixed positions
    SYSTEM_RADIUS + BASE_CLEARANCE + (horizontal_distance * DISTANCE_FACTOR)
};

// Bezier curve: from_pos -> control_point_1 -> control_point_2 -> to_pos
```

### Visual Styling

```rust
// Feedback connection appearance
let stroke = Stroke::new(CYAN_COLOR, 3.0);  // Cyan distinguishes from flows
let fill = Fill::color(Color::NONE);         // No fill, just stroke
let alpha = 0.6;                              // Semi-transparent

// Color definition
const CYAN_COLOR: Color = Color::srgb(0.0, 1.0, 1.0);  // #00FFFF
```

### File Structure

**Primary Implementation**:
- `src/bevy_app/systems/ui/feedback_connections.rs` (449 lines)

**Integration Points**:
- `src/bevy_app/systems/ui/mod.rs` - Module exports
- `src/bevy_app/mod.rs` - System registration in main app

**Component Dependencies**:
- `ExternalEntity` - Marker component for sources/sinks
- `IsSameAsId` - Equivalence ID component
- `NestingLevel` - System hierarchy level
- `Transform` - Entity position and transform

## Usage Examples

### Basic Feedback Loop

```rust
// User creates feedback loop through UI:
// 1. Create Source entity (e.g., "Sensor Output")
// 2. Create Sink entity (e.g., "Controller Input")
// 3. Select both entities (Shift+click)
// 4. Press 'E' key

// System automatically creates:
let feedback = FeedbackConnection {
    from: source_entity,      // Sensor Output
    to: sink_entity,          // Controller Input
    equivalence_id: 1,        // Matches IsSameAsId value on both entities
};

// Visual: Curved cyan line connecting source → sink,
//         arcing over system boundary with proper clearance
```

### Multiple Feedback Loops

```rust
// User can create multiple independent feedback loops:
// Equivalence ID 1: Source A ↔ Sink A
// Equivalence ID 2: Source B ↔ Sink B
// Equivalence ID 3: Source C ↔ Sink C

// Each equivalence group gets distinct visual connections
// No crosstalk between different equivalence IDs
```

### Complex Feedback Hierarchy

```rust
// Nested feedback loops possible:
// - Outer loop: System output → System input (ID 1)
// - Inner loop: Subsystem output → Subsystem input (ID 2)
// - Cross-level: Subsystem output → System input (ID 3)

// Visual clarity maintained through:
// - Distinct equivalence IDs
// - Smart curve routing avoiding overlaps
// - Cyan color distinguishing from regular flows
```

## Testing Strategy

### Manual Testing Scenarios

**Configuration Testing**:
- ✅ Horizontal source-sink arrangements (curves over top)
- ✅ Vertical arrangements (appropriate curve direction)
- ✅ Mixed arrangements (defaults to over-top routing)
- ✅ Multiple simultaneous feedback loops (no interference)

**Dynamic Behavior Testing**:
- ✅ Entity dragging updates connections in real-time
- ✅ Equivalence removal properly cleans up connections
- ✅ Adding third entity to existing equivalence group creates new connections
- ✅ Connection persistence across model save/load cycles

**Performance Testing**:
- ✅ No frame-by-frame updates (only on change)
- ✅ Large models with 10+ feedback loops maintain smooth rendering
- ✅ No memory leaks from orphaned connections

### Edge Cases Handled

- **Self-equivalence**: Single entity with equivalence ID (no connection drawn)
- **Duplicate equivalences**: Multiple entities with same ID (all pairwise connections)
- **Deleted entities**: Connections cleaned up when endpoint entities despawned
- **Equivalence ID conflicts**: Separate groups maintain independence

## Future Improvements

### Visual Enhancements
- **Arrow Heads**: Add directional indicators to show feedback flow direction
- **Animation**: Subtle animation or dash patterns to show active feedback flows
- **Color Coding**: Different colors for positive vs negative feedback loops
  - Green for negative feedback (stabilizing)
  - Red for positive feedback (amplifying)

### Functional Enhancements
- **Feedback Properties**: Implement gain, delay, and polarity attributes
  - Gain: Amplification factor (K > 1 or K < 1)
  - Delay: Time lag in feedback response
  - Polarity: Positive vs negative feedback classification
- **Nested Loops**: Enhanced support for complex feedback hierarchies
- **Toggle Visibility**: User controls to show/hide feedback connections for complex models
- **Feedback Analysis**: Automatic loop detection and stability analysis

### Integration Opportunities
- **Dynamic Simulations**: Feedback loops drive time-based evolution
- **System Dynamics**: Export feedback structure to SD models
- **Agent Processing**: Agents respond to feedback signals

## Related Documentation

- **System Language Theory**: Feedback implements "N" (internal interactions) component
- **Equivalence System**: Built on existing `IsSameAsId` component workflow
- **Flow Visualization**: Follows established patterns from flow rendering
- **External Entities**: Extends source/sink modeling for environment-system feedback
- **Bevy Lyon Integration**: Uses bevy_prototype_lyon for path rendering

## Re-implementation Guide

### Step 1: Create Component
Copy component definition from above into new file: `src/bevy_app/components/feedback_connection.rs`

### Step 2: Implement Systems
Create `src/bevy_app/systems/ui/feedback_connections.rs` with three systems:
1. `draw_feedback_connections` - Change detection on `IsSameAsId`
2. `update_feedback_connections` - Change detection on `Transform`
3. `cleanup_feedback_connections` - Despawn orphaned connections

### Step 3: Integrate into App
Register systems in `src/bevy_app/mod.rs`:
```rust
app.add_systems(Update, (
    feedback_connections::draw_feedback_connections,
    feedback_connections::update_feedback_connections,
    feedback_connections::cleanup_feedback_connections,
));
```

### Step 4: Test
Follow manual testing scenarios above to verify behavior.

## Notes on Extraction

This specification was extracted from the `feature/feedback-visualization-simple` branch on 2025-10-08 before branch deletion. The branch contained a complete working implementation (449 lines) but was removed during branch consolidation to focus on core agent processing functionality.

The implementation can be fully reconstructed from this documentation. All critical algorithms, parameters, and architectural decisions are preserved above.

---

_This documentation was preserved from the Feedback Loop Visualization Enhancement feature implemented on 2025-09-06 and archived on 2025-10-08._
