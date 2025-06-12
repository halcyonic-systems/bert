# Visual System Architecture

## Overview

The visual system in BERT implements a sophisticated hierarchical rendering architecture that supports infinite nesting, dynamic scaling, and automatic curve generation. This document provides comprehensive coverage of the spatial organization, rendering pipeline, and visual behavior patterns based on the original system design.

**Key Concepts:**
- **Scene Graph Hierarchy** with infinite nesting support
- **Dynamic scaling** based on zoom and nesting level
- **Z-coordinate system** for proper draw order at all nesting levels
- **Automatic curve generation** using cubic Bézier curves
- **Context-aware entity rotation** for aesthetic connections

## Table of Contents

- [Scene Graph Hierarchy](#scene-graph-hierarchy)
- [Zoom and Scaling System](#zoom-and-scaling-system)
- [Draw Order and Z-Coordinate System](#draw-order-and-z-coordinate-system)
- [Interaction Curves and Geometry](#interaction-curves-and-geometry)
- [Entity Rotation Logic](#entity-rotation-logic)
- [Visibility Management](#visibility-management)
- [Performance Considerations](#performance-considerations)
- [Integration with Current Codebase](#integration-with-current-codebase)

## Scene Graph Hierarchy

### Hierarchical Organization

BERT uses a **parent-child entity hierarchy** that directly maps to the systems science concept of nested systems:

```
Entity Hierarchy Example:
├── S0 (Root System)
│   ├── Snk0.0 (Internal Sink)
│   ├── F0.0 (Internal Flow)
│   ├── S0.1 (Subsystem)
│   ├── I0.0 (Interface)
│   │   └── S0.0 (Interface Subsystem)
│   │       ├── I0.0.0 (Nested Interface)
│   │       └── S0.0.0 (Nested Interface Subsystem)
│   └── [Other entities...]
├── Snk-1.0 (External Sink)
├── F-1.0 (External Flow)
└── [Labels - Root Level]
```

### Nesting Rules

#### Interface Subsystem Placement
**Important**: Interface subsystems are children of their **containing system**, not their associated interface:
- `S0.0.0` (interface subsystem) is child of `S0.0`, not `I0.0`
- This maintains proper hierarchical relationships for second-level interface subsystems

#### Root Level Entities
- **External entities** (sources, sinks, flows): Always root level
- **Labels**: Always root level entities
- **Root system**: Top-level container for internal entities

### Nesting Level Component

All scalable entities have a `NestingLevel` component:

```rust
struct NestingLevel(u16);
```

**Nesting Level Assignment**:
| Entity Type | Nesting Level | Example |
|-------------|---------------|---------|
| External entities | 0 | `Snk-1.0`, `F-1.0` |
| Root system | 0 (no component) | `S0` |
| First-level internal | 1 | `Snk0.0`, `S0.1`, `I0.0` |
| Second-level internal | 2 | `I0.0.0`, `S0.0.0` |
| N-level internal | N | Continues infinitely |

## Zoom and Scaling System

### Zoom Implementation

**Zoom Storage**: Stored as a Bevy `Resource`, not applied to camera matrix
**Camera Behavior**: Camera translates to maintain center point, does not scale
**Entity Scaling**: Zoom multiplied to entity translations (x, y), z unchanged

```rust
// Conceptual zoom application
fn apply_zoom(entity_position: Vec3, zoom: f32, initial_position: Vec3) -> Vec3 {
    Vec3::new(
        initial_position.x * zoom,
        initial_position.y * zoom,
        entity_position.z  // Z unchanged
    )
}
```

### Initial Position Component

```rust
struct InitialPosition(Vec3);
```

**Purpose**: Provides base position at zoom level 1.0
**Usage**: All entity positions calculated from `InitialPosition * zoom`

### Entity-Specific Scaling Behavior

#### Systems
- **Size scaling**: Circle radius multiplied by zoom
- **Transform unchanged**: Entity transform scale remains 1.0
- **Visual only**: Only visual representation scales

#### Labels, External Entities, Interfaces
- **Maximum size limit**: Have upper bound on visual size
- **Threshold scaling**: Only scale down when zoom falls below threshold
- **Nesting-aware**: Scale calculation includes nesting level

### Scale Calculation Formula

For labels, external entities, and interfaces:

```rust
fn calculate_scale(zoom: f32, nesting_level: u16) -> f32 {
    let nesting_factor = SUBSYSTEM_SCALING_FACTOR.powi(nesting_level as i32);
    zoom * nesting_factor
}
```

**Constants**:
- `SUBSYSTEM_SCALING_FACTOR`: Fixed constant < 1.0 (reduces scale with nesting depth)
- Applied exponentially based on nesting level

### Line Width Scaling

**All entities** (including systems) use the same scale calculation for line width:
- Ensures consistent visual hierarchy
- Maintains readability at all zoom levels
- Scales proportionally with entity size

## Draw Order and Z-Coordinate System

### Infinite Nesting Z-Coordinate Strategy

The visual system implements a **hierarchical Z-coordinate system** that supports infinite nesting while maintaining proper draw order.

#### Z-Scaling Mechanism

Each subsystem applies **0.9 scale** to child Z-coordinates:
- **Local range**: Each system uses Z coordinates 0-100
- **Global mapping**: `global_z = (local_z * parent_scale) + parent_global_z`
- **Scale inheritance**: `child_scale = parent_scale * 0.9`

#### Local Z-Coordinate Assignments

| Entity Type | Local Z | Purpose |
|-------------|---------|---------|
| Root System | 0 | Base layer |
| Interaction | 1 | Above systems |
| External Entity | 1 | Same level as interactions |
| Subsystem | 10 | Above interactions |
| Interface Subsystem | -90 | Below parent system |
| Interface | 100 | Top of local space |
| Button | 200 | Above all entities |
| Label | 150 | Above interfaces, below buttons |

#### Z-Coordinate Calculation Example

```
Entity: S0.0.0 (nested interface subsystem)
├── Parent: S0.0 (interface subsystem, local_z=-90, scale=0.9)
├── Grandparent: S0 (root system, local_z=0, scale=1.0)
└── Calculation:
    S0.0 global_z = (-90 * 1.0) + 0 = -90
    S0.0 scale = 1.0 * 0.9 = 0.9
    S0.0.0 global_z = (10 * 0.9) + (-90) = -81
```

### Button Positioning

**Button Parent**: Buttons have the same parent as the entity they will create
**Z-Coordinate**: Always use local Z = 200 for maximum visibility
**Lifecycle**: Created/destroyed with entity focus changes

## Interaction Curves and Geometry

### Cubic Bézier Curve Implementation

Interactions are rendered as **cubic Bézier curves** with automatically computed control points:

```rust
struct FlowCurve {
    start_point: Vec3,      // P0: Curve start
    end_point: Vec3,        // P3: Curve end
    start_direction: Vec3,  // Direction for C1 computation
    end_direction: Vec3,    // Direction for C2 computation
}

// Control points computed as:
// C1 = start_point + (start_direction * tangent_length)
// C2 = end_point - (end_direction * tangent_length)
// tangent_length = distance(start_point, end_point) / 3.0
```

### Tangent Direction Computation

#### Interface Connections
**Direction Source**: `interface_transform.right()` unit vector
**Purpose**: Ensures curves flow naturally from interface orientation
**Implementation**: Interface rotation determines curve tangent direction

#### External Entity Connections
**Dynamic Rotation**: External entities rotate to create aesthetic curves
**Computation Method**: 
1. Calculate opposite control point position
2. Compute vector from entity to control point
3. Use vector as entity rotation direction

### Curve Smoothness

**Tangent Length**: Set to 1/3 of endpoint distance
**Result**: Produces smooth, aesthetically pleasing curves
**Automatic**: No manual control point adjustment needed

## Entity Rotation Logic

### External Entity Automatic Rotation

External entities (sources/sinks) automatically rotate when:
- **Being dragged** by user
- **Connected to interactions** for aesthetic curve generation

#### Rotation Computation Process

```rust
fn compute_external_entity_rotation(
    entity_position: Vec3,
    connected_curve: &FlowCurve,
    is_start: bool
) -> Vec3 {
    // 1. Compute opposite control point
    let control_point = if is_start {
        compute_control_point_c2(connected_curve)
    } else {
        compute_control_point_c1(connected_curve)
    };
    
    // 2. Compute direction vector
    let direction = (control_point - entity_position).normalize();
    
    // 3. Use as rotation
    direction
}
```

### Mouse Cursor Interaction

**Temporary Connections**: When selecting source/sink with mouse cursor
**Real-time Rotation**: External entity rotates to maintain aesthetic curve
**Same Algorithm**: Uses identical rotation computation as permanent connections

### Subsystem Connection Targeting

#### Target Point Computation

When connecting to subsystems:

```rust
fn compute_subsystem_connection_point(
    subsystem_center: Vec3,
    subsystem_radius: f32,
    approach_direction: Vec3
) -> (Vec3, Vec3) {
    // 1. Compute smooth direction (same as external entity)
    let smooth_direction = compute_smooth_direction(subsystem_center, approach_direction);
    
    // 2. Compute target point on subsystem boundary
    let target_point = subsystem_center + (smooth_direction * subsystem_radius);
    
    // 3. Return point and direction for curve
    (target_point, smooth_direction)
}
```

#### Interface Creation from Subsystem Selection

**Automatic Interface Positioning**: When subsystem selected as target
**Position**: Uses computed target point from connection algorithm
**Orientation**: Uses computed direction for interface rotation
**Consistency**: Ensures visual continuity between selection and final interface

## Visibility Management

### Scale-Based Visibility

Entities become invisible when their computed scale falls below thresholds:

```rust
// Visibility thresholds
const SCALE_VISIBILITY_THRESHOLD: f32 = 0.1;
const LABEL_SCALE_VISIBILITY_THRESHOLD: f32 = 0.05;  // Labels more persistent

fn is_entity_visible(scale: f32, is_label: bool) -> bool {
    let threshold = if is_label {
        LABEL_SCALE_VISIBILITY_THRESHOLD
    } else {
        SCALE_VISIBILITY_THRESHOLD
    };
    scale >= threshold
}
```

### Nesting-Aware Visibility

**Deep Nesting**: Entities at deep nesting levels become invisible sooner
**Zoom Dependency**: Visibility depends on both zoom level and nesting depth
**Label Persistence**: Labels remain visible longer than other entities

## Performance Considerations

### Efficient Scale Computation

**Caching Strategy**: Cache scale calculations per nesting level
**Batch Updates**: Update scales in batches when zoom changes
**Dirty Flagging**: Only recalculate when zoom or nesting changes

### Z-Coordinate Optimization

**Hierarchical Updates**: Only update Z-coordinates when hierarchy changes
**Local Calculations**: Minimize global Z-coordinate recalculation
**Culling**: Use Z-coordinates for efficient rendering culling

### Curve Computation Optimization

**Lazy Evaluation**: Only compute curves when endpoints change
**Incremental Updates**: Update only affected curves when entities move
**LOD System**: Reduce curve complexity at high zoom levels

## Integration with Current Codebase

### Component Definitions

```rust
// Core visual components
struct InitialPosition(Vec3);
struct NestingLevel(u16);
struct FlowCurve {
    start_point: Vec3,
    end_point: Vec3,
    start_direction: Vec3,
    end_direction: Vec3,
}

// Resource for global state
struct ZoomLevel(f32);
```

### System Organization

```
src/bevy_app/systems/
├── camera.rs              # Camera positioning and zoom
├── ui/
│   ├── scaling.rs         # Entity scaling systems
│   ├── visibility.rs      # Visibility management
│   └── curve_update.rs    # Curve computation systems
└── setup.rs               # Initial position setup
```

### Rendering Pipeline Integration

**Bevy Integration**: Works with Bevy's transform and rendering systems
**Custom Shaders**: May use custom shaders for curve rendering
**Batching**: Efficient batching of similar visual elements

### Event System Integration

**Zoom Events**: Triggered by user input, update all visual systems
**Hierarchy Events**: Entity parent/child changes trigger Z-coordinate updates
**Focus Events**: Button creation/destruction based on entity focus

---

## Related Documentation

- [Interaction System Architecture](interaction-system-architecture.md) - Curve integration with interaction lifecycle
- [Component Relationship Guide](component-relationship-guide.md) - Visual component patterns
- [Performance Optimization Guide](performance-optimization-guide.md) - Visual system optimization
- [Contributing Guidelines](../contributing/contributing.md) - Development standards and practices

---

**Note**: This document is based on the original geometry specifications and has been updated to reflect current codebase implementation. The visual system is designed to support infinite nesting while maintaining 60+ FPS performance targets. 