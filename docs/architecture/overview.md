# BERT Architecture Overview

## High-Level Architecture

BERT is organized around a layered architecture that directly implements the System Language (SL) theoretical framework:

```
┌─────────────────────────────────────────────┐
│                User Interface                │
│           (Leptos Web Components)            │
├─────────────────────────────────────────────┤
│              Visualization Layer             │
│              (Bevy Game Engine)              │
├─────────────────────────────────────────────┤
│                 System Model                 │
│        (Components, Events, Resources)       │
├─────────────────────────────────────────────┤
│                  Data Model                  │
│          (Serialization/Persistence)         │
└─────────────────────────────────────────────┘
```

## Key Components

### 1. User Interface (Leptos Components)
- **Purpose**: Provides interactive UI elements for manipulating system elements
- **Location**: `src/leptos_app/`
- **Key Files**: 
  - Components for editing properties (`components/`)
  - Tree view for system hierarchy (`tree/`)
  - Detail panels for selected elements (`details.rs`)

### 2. Visualization Layer (Bevy Engine)
- **Purpose**: Renders and handles interaction with the system diagram
- **Location**: `src/bevy_app/`
- **Key Files**:
  - Systems for element interaction (`systems/ui/`)
  - Camera and viewport management (`systems/camera.rs`)
  - Element spawning logic (`bundles/spawn/`)

### 3. System Model (Core Data Structures)
- **Purpose**: Defines the fundamental system elements and their relationships
- **Location**: `src/bevy_app/components/`
- **Key Files**:
  - System elements definitions (`system_elements.rs`)
  - Connection definitions (`connections.rs`)
  - UI state components (`ui.rs`)

### 4. Data Model (Persistence)
- **Purpose**: Handles saving and loading system models
- **Location**: `src/bevy_app/data_model/`
- **Key Files**:
  - Save functionality (`save.rs`)
  - Load functionality (`load.rs`)

## Conceptual Mapping

The architecture directly implements the System Language concepts:

| SL Concept | Implementation | Location |
|------------|----------------|----------|
| System | `SystemComponent` | `components/system_elements.rs` |
| Subsystem | `SubsystemComponent` | `components/system_elements.rs` |
| Flow | `FlowComponent` | `components/connections.rs` |
| Interface | `InterfaceComponent` | `components/connections.rs` |
| External Entity | `ExternalEntityComponent` | `components/system_elements.rs` |

## Integration Flow

1. **User Interaction**: Leptos UI components capture user input
2. **Events**: UI events trigger Bevy events (`events.rs`)
3. **System Updates**: Bevy systems react to events and update the model
4. **Rendering**: Changes to the model are rendered by the Bevy engine
5. **Persistence**: System state can be saved/loaded via the data model

## Extension Points

Systems scientists can extend BERT at several levels:

1. **UI Components**: Add new Leptos components for specialized inputs
2. **Visualization**: Enhance or modify the Bevy rendering systems
3. **Model**: Extend the core system elements to support new concepts
4. **Persistence**: Modify the serialization format for additional data