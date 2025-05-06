# BERT Architecture Overview

BERT (Bounded Entity Reasoning Toolkit) is a visual modeling tool for systems science that implements the System Language (SL) framework. This document provides a high-level overview of BERT's architecture for systems scientists and developers.

## System Language Implementation

BERT implements the five-layer System Language framework:

```
┌─────────────────────────────────────────────────┐
│ Layer 5: Application Framework (BERT)           │
├─────────────────────────────────────────────────┤
│ Layer 4: Implementation Languages               │
│          (Visual representation)                │
├─────────────────────────────────────────────────┤
│ Layer 3: Knowledge Representation               │
│          (Data model & persistence)             │
├─────────────────────────────────────────────────┤
│ Layer 2: Formal Specification                   │
│          (Component structure & relationships)  │
├─────────────────────────────────────────────────┤
│ Layer 1: Theoretical Framework                  │
│          (Systems science concepts)             │
└─────────────────────────────────────────────────┘
```

## Technical Architecture

BERT is built with a layered architecture using Rust, Leptos, and Bevy:

```
┌─────────────────────────────────────────────────┐
│ User Interface (Leptos Web Components)          │
│ src/leptos_app/                                 │
├─────────────────────────────────────────────────┤
│ Visualization Layer (Bevy Game Engine)          │
│ src/bevy_app/systems/, src/bevy_app/bundles/    │
├─────────────────────────────────────────────────┤
│ System Model (Components & Resources)           │
│ src/bevy_app/components/                        │
├─────────────────────────────────────────────────┤
│ Data Model (Serialization/Persistence)          │
│ src/bevy_app/data_model/                        │
└─────────────────────────────────────────────────┘
```

## Key Components

### 1. System Model (Core Data Structures)

The System Model implements Layer 2 (Formal Specification) by defining components that represent system elements:

- **System**: Main bounded entity (`SystemComponent`)
- **Subsystem**: Nested system within boundaries (`SubsystemComponent`)
- **Flow**: Movement of materials, energy, or information (`FlowComponent`)
- **Interface**: Connection points between systems (`InterfaceComponent`)
- **External Entity**: Outside actors that interact with the system (`ExternalEntityComponent`)

These components are defined in `src/bevy_app/components/`.

### 2. Visualization Layer (Bevy Engine)

The Visualization Layer implements Layer 4 (Implementation Languages) by providing graphical representation:

- **Spawn Bundles**: Create visual entities (`src/bevy_app/bundles/spawn/`)
- **Interaction Systems**: Handle user interaction (`src/bevy_app/systems/ui/`)
- **Camera Systems**: Manage viewport (`src/bevy_app/systems/camera.rs`)

### 3. User Interface (Leptos Components)

The UI Layer provides tools for creating and editing system elements:

- **Property Panels**: Edit element properties (`src/leptos_app/details.rs`)
- **Tree View**: Navigate system hierarchy (`src/leptos_app/tree/`)
- **Input Components**: Reusable UI elements (`src/leptos_app/components/`)

### 4. Data Model (Persistence)

The Data Model implements Layer 3 (Knowledge Representation) by handling persistence:

- **Save Logic**: Serialize system to JSON (`src/bevy_app/data_model/save.rs`)
- **Load Logic**: Deserialize JSON to system (`src/bevy_app/data_model/load.rs`)

## Information Flow

1. **User Interaction**: User interacts with Leptos UI or Bevy canvas
2. **Event Dispatch**: Actions trigger events in `src/bevy_app/events.rs`
3. **System Update**: Bevy systems respond to events, updating components
4. **Visual Update**: Changes to components are reflected in visualization
5. **Data Persistence**: System can be saved to and loaded from JSON

## Extension Points

Systems scientists can extend BERT at several levels:

1. **System Components**: Add new element types in `src/bevy_app/components/`
2. **Visual Representation**: Modify rendering in `src/bevy_app/bundles/spawn/`
3. **UI Components**: Add new editing controls in `src/leptos_app/components/`
4. **Data Model**: Extend persistence format in `src/bevy_app/data_model/`

## Additional Resources

For more detailed information, see:

- [System Language Implementation](docs/architecture/system-language-implementation.md)
- [Getting Started for Systems Scientists](docs/getting-started/for-systems-scientists.md)
- [Code Navigation Guide](docs/getting-started/code-navigation-guide.md)
- [Contributing Guidelines](docs/contributing/guidelines.md)