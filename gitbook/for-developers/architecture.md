# Architecture

BERT implements a sophisticated **Layer 4 System Language** architecture that bridges systems science theory with modern software engineering practices.

> 💡 **Looking for visual diagrams?** Check out the [Visual Architecture Guide](visual-architecture.md) for simple, illustrated explanations of these concepts.

## Architecture Overview

BERT uses a **component-based ECS (Entity-Component-System) architecture** built on the Bevy game engine, with a reactive web UI layer powered by Leptos. This creates a powerful combination of real-time visualization and intuitive user interaction.

### Technology Stack

```
┌─────────────────────────────────────────────────┐
│ User Interface Layer (Leptos Web Components)    │
│ - Property editing panels                       │
│ - System hierarchy navigation                   │
│ - Interactive controls and forms               │
├─────────────────────────────────────────────────┤
│ Visualization Layer (Bevy Game Engine)          │
│ - Entity-Component-System architecture         │
│ - Real-time rendering and interaction          │
│ - Infinite nesting with automatic scaling      │
├─────────────────────────────────────────────────┤
│ System Model Layer (Component Definitions)      │
│ - System elements and relationships            │
│ - Interaction lifecycle management             │
│ - Event-driven state coordination              │
├─────────────────────────────────────────────────┤
│ Data Model Layer (Persistence & Serialization)  │
│ - JSON-based save/load system                  │
│ - Version management and compatibility         │
│ - Comprehensive data validation                │
├─────────────────────────────────────────────────┤
│ Platform Layer (Tauri Desktop Framework)        │
│ - Cross-platform deployment                    │
│ - Native OS integration                        │
│ - File system and window management            │
└─────────────────────────────────────────────────┘
```

### Core Technologies

- **Rust**: Systems programming language for performance and safety
- **Bevy**: Entity-Component-System game engine for visualization
- **Leptos**: Reactive web framework for UI components
- **Tauri**: Desktop application framework for cross-platform deployment
- **JSON**: Human-readable serialization format for data persistence

## System Language Implementation

BERT implements the complete System Language framework through a five-layer architecture:

### Layer 5: Application Framework
- BERT Application
- Tauri Desktop Integration
- Cross-Platform Deployment

### Layer 4: Implementation Languages
- Visual Representation
- Interactive UI Components
- Event-Driven Coordination

### Layer 3: Knowledge Representation
- Data Model & Serialization
- Persistence Layer
- Version Management

### Layer 2: Formal Specification
- Component Definitions
- Entity Relationships
- System Constraints

### Layer 1: Theoretical Framework
- Systems Science Concepts
- Mathematical Foundations
- Conceptual Integrity

## Core Subsystems

### 1. Interaction System

**Purpose**: Manages the complete lifecycle of connections between system entities

**Key Features**:
- **4-stage lifecycle**: Created → Interface → Other End → Complete
- **Component-based state machine**: Dynamic component addition/removal
- **Context-aware behavior**: Different workflows for root vs. subsystem contexts
- **Automatic state recovery**: Handles entity removal and state reversion

**Components**:
```rust
// Connection tracking
FlowStartConnection, FlowEndConnection
FlowStartInterfaceConnection, FlowEndInterfaceConnection

// UI state management  
HasFlowInterfaceButton, HasFlowOtherEndButton

// Permanent components
Flow, FlowCurve
```

### 2. Visual System

**Purpose**: Provides sophisticated hierarchical rendering with infinite nesting support

**Key Features**:
- **Scene graph hierarchy**: Parent-child entity relationships
- **Dynamic scaling**: Zoom and nesting-level aware scaling
- **Z-coordinate system**: Proper draw order for infinite nesting
- **Automatic curve generation**: Cubic Bézier curves with computed control points
- **Context-aware rotation**: Aesthetic entity positioning

**Components**:
```rust
// Spatial organization
InitialPosition, NestingLevel

// Visual representation
FlowCurve, Transform

// Global state
ZoomLevel (Resource)
```

### 3. Plugin Architecture

**Purpose**: Modular system integration with coordinated lifecycle management

**Current Plugins**:
- **Mouse Interaction**: Selection, dragging, and interaction handling
- **Label System**: Text labeling with entity composition
- **Lyon Selection**: Advanced selection and highlighting

**Plugin Structure**:
```
src/bevy_app/plugins/
├── mod.rs                  # Plugin coordination
├── mouse_interaction/      # Mouse and selection handling
├── label/                  # Text labeling system
└── lyon_selection/         # Advanced selection graphics
```

### 4. Data Model and Persistence

**Purpose**: Comprehensive data representation and persistence with version management

**Key Features**:
- **Hierarchical data model**: Reflects system nesting structure
- **JSON serialization**: Human-readable and version-controllable
- **Backward compatibility**: Handles format evolution
- **Comprehensive validation**: Data integrity and constraint checking

**Implementation**:
```
src/bevy_app/data_model/
├── mod.rs                  # Core data structures and traits
├── save.rs                 # Serialization logic
└── load.rs                 # Deserialization and validation
```

### 5. Event System and Coordination

**Purpose**: Cross-system communication and state synchronization

**Event Categories**:
- **UI Events**: User interactions from Leptos components
- **System Events**: Entity creation, modification, removal
- **Lifecycle Events**: Interaction state transitions
- **Focus Events**: Entity selection and UI state changes

**Integration Pattern**:
```
// Event flow: Leptos UI → Events → Bevy Systems → State Updates → Rendering
src/events.rs               # Cross-system event definitions
```

## Systems Science Mapping

The architecture directly implements core systems science concepts:

| Systems Concept | Implementation | Location |
|----------------|----------------|----------|
| **System** | `SystemComponent` with boundaries | `components/system_elements.rs` |
| **Subsystem** | Nested `SubsystemComponent` | `components/system_elements.rs` |
| **Flow** | `FlowComponent` with lifecycle | `components/connections.rs` |
| **Interface** | `InterfaceComponent` with positioning | `components/connections.rs` |
| **External Entity** | `ExternalEntityComponent` | `components/system_elements.rs` |
| **Hierarchy** | Scene graph with infinite nesting | Visual system architecture |
| **Boundaries** | Visual containers with permeability | Bundle spawn system |

## Integration Patterns

### Leptos-Bevy Communication

**Challenge**: Integrate web-based UI with game engine visualization

**Solution**: Event-driven architecture with shared state management

**Flow**:
1. Leptos UI captures user interaction
2. Event System triggers appropriate events
3. Bevy Systems respond to events
4. Component State updates
5. Rendering reflects changes
6. UI receives state feedback

### Component Lifecycle Management

**Pattern**: Dynamic component addition/removal based on entity state

**Implementation**: Specialized systems for each lifecycle stage

**Benefits**: Clean state management and automatic cleanup

### Cross-System Data Flow

**Principle**: Unidirectional data flow with event-driven updates

**Implementation**: Events trigger system updates, systems modify components, rendering reflects changes

**Benefits**: Predictable state management and debugging

## Performance Architecture

### Rendering Performance

**Target**: 60+ FPS with complex system models

**Strategies**:
- **Efficient ECS queries**: Optimized component access patterns
- **Batched updates**: Minimize individual entity operations
- **LOD system**: Level-of-detail for complex visualizations
- **Culling**: Z-coordinate based rendering optimization

### Memory Management

**Approach**: Rust's ownership system for memory safety

**Optimization**: 
- **Component pooling**: Reuse components when possible
- **Efficient data structures**: Minimize allocation overhead
- **Lazy evaluation**: Compute only when necessary

### Scalability Considerations

- **Infinite Nesting**: Hierarchical algorithms that scale with depth
- **Large Models**: Efficient handling of complex system representations
- **Real-time Interaction**: Responsive UI even with large datasets

## Development Standards

### Documentation Requirements

**Requirement**: 100% compliance with professional documentation templates

**Implementation**: 
- **6 comprehensive templates** for all code element types
- **Complete function documentation** with Parameters, Returns, Errors, Panics
- **Systems science context** for all architectural decisions

### Code Quality Standards

**Requirements**:
- **No clippy warnings**: All code passes strict linting
- **Comprehensive testing**: Unit, integration, and systems tests
- **Performance targets**: 60+ FPS maintained in visualization
- **Architecture compliance**: Follows established patterns

## Extension Points

### Adding New System Elements

**Process**:
1. **Component Definition**: Add to `system_elements.rs`
2. **Visual Representation**: Create spawn bundle
3. **UI Integration**: Add Leptos editing components
4. **Serialization**: Update data model
5. **Documentation**: Complete template compliance

### Plugin Development

**Framework**: Bevy plugin system with coordinated lifecycle

**Integration**: Event system for cross-plugin communication

**Standards**: Follow established plugin patterns

### Visual System Extensions

- **Rendering**: Custom shaders and visual effects
- **Interaction**: New interaction patterns and behaviors
- **Scaling**: Enhanced scaling and visibility algorithms

## Current Implementation Status

### Fully Implemented Systems

✅ **Core Architecture**: Complete ECS foundation with Bevy  
✅ **Interaction Lifecycle**: 4-stage state machine fully implemented  
✅ **Visual System**: Infinite nesting with automatic scaling  
✅ **Data Model**: Comprehensive persistence with version management  
✅ **Plugin System**: Mouse interaction, labeling, selection  
✅ **Documentation**: Professional standards with template compliance  

### Areas for Enhancement

🔄 **Bundle System**: Needs comprehensive documentation  
🔄 **Event System**: Requires detailed architectural documentation  
🔄 **Resource Management**: Global state patterns need documentation  
🔄 **Performance Optimization**: Systematic optimization documentation  

## Future Architecture Evolution

### Planned Enhancements

1. **Complete SysXML Support**: Generate XML representations of systems
2. **Transformation Rules**: Add computational capabilities for system simulation
3. **Memory/History**: Track system state changes over time
4. **Time Interval Handling**: Support multi-scale modeling
5. **Knowledge Base Integration**: Connect with external systems databases

### Architectural Principles for Evolution

1. **Maintain Conceptual Integrity**: All changes align with systems science theory
2. **Preserve Performance**: 60+ FPS target maintained
3. **Extend Documentation**: 100% compliance for all new code
4. **Follow Established Patterns**: Use proven architectural approaches

---

**For detailed technical specifications**, see the comprehensive architecture documentation in the project repository at `docs/architecture/comprehensive-architecture-overview.md`.

