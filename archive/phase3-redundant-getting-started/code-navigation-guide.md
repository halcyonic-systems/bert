# BERT Code Navigation Guide

This guide helps systems scientists and new contributors navigate BERT's codebase effectively.

## Project Structure Overview

```
bert-main/
├── src/                    # Main source code
│   ├── bevy_app/           # System model and visualization (Bevy)
│   ├── leptos_app/         # User interface components (Leptos)
│   └── main.rs             # Application entry point
├── src-tauri/              # Tauri desktop integration
├── assets/                 # Static assets (icons, fonts)
├── docs/                   # Documentation
└── research/               # Theoretical background materials
```

## Key Modules and Files

### Main Application

- `src/main.rs`: Application entry point, mounts the Leptos app

### User Interface (Leptos)

- `src/leptos_app/mod.rs`: Main UI component definition
- `src/leptos_app/components/`: Reusable UI components (buttons, sliders, etc.)
- `src/leptos_app/details.rs`: Property panel for editing element properties
- `src/leptos_app/tree/`: System hierarchy tree view

### System Model and Visualization (Bevy)

- `src/bevy_app/mod.rs`: Main Bevy app setup
- `src/bevy_app/components/`: Core data model components
  - `system_elements.rs`: Definitions for systems, subsystems, etc.
  - `connections.rs`: Definitions for flows and interfaces
- `src/bevy_app/bundles/spawn/`: Entity creation logic
- `src/bevy_app/systems/`: Bevy ECS systems for updating the model
  - `ui/`: User interaction handling
  - `camera.rs`: Viewport management
- `src/bevy_app/data_model/`: Serialization/deserialization

## Following the Code Flow

### System Creation Flow

1. User clicks "Create System" button:
   - Defined in: `src/leptos_app/components/button.rs`
   - Triggers event: `CreateSystemEvent`

2. Bevy system handles the event:
   - Event defined in: `src/bevy_app/events.rs`
   - Handler in: `src/bevy_app/systems/`

3. System entity is spawned:
   - Spawn logic in: `src/bevy_app/bundles/spawn/main_system.rs`
   - Creates entity with components from: `src/bevy_app/components/system_elements.rs`

### Property Editing Flow

1. User selects an element:
   - Selection handling in: `src/bevy_app/systems/ui/`

2. Property panel updates:
   - Panel logic in: `src/leptos_app/details.rs`
   - Displays fields based on selected entity type

3. User edits properties:
   - Input components in: `src/leptos_app/components/`
   - Changes trigger events to update the Bevy model

### Save/Load Flow

1. User triggers save/load:
   - Save/load events defined in: `src/bevy_app/events.rs`

2. Data is serialized/deserialized:
   - Save logic in: `src/bevy_app/data_model/save.rs`
   - Load logic in: `src/bevy_app/data_model/load.rs`

## Common Code Navigation Scenarios

### "I want to change how a system element looks"

Look at:
1. `src/bevy_app/bundles/spawn/main_system.rs` for visual definition
2. `src/bevy_app/systems/ui/color.rs` for color handling

### "I want to add a new property to system elements"

Modify:
1. `src/bevy_app/components/system_elements.rs` to add the property
2. `src/leptos_app/details.rs` to add UI for editing
3. `src/bevy_app/data_model/save.rs` and `load.rs` for persistence

### "I want to change how connections are drawn"

Look at:
1. `src/bevy_app/systems/ui/flow/curve.rs` for flow rendering
2. `src/bevy_app/bundles/spawn/flow.rs` for flow creation

## Using LLM Tools for Code Navigation

When exploring unfamiliar parts of the codebase, you can use Claude or similar LLMs:

1. **Find relevant files**: "Where is the system element defined in BERT?"
2. **Understand components**: "Explain how the FlowComponent works"
3. **Trace execution**: "What happens when a user creates a new subsystem?"
4. **Find extension points**: "How would I add a new type of connection?"

## Recommended Workflow

1. **Start at the model**: Understand the core components in `src/bevy_app/components/`
2. **Explore visualization**: See how components are rendered in `src/bevy_app/bundles/spawn/`
3. **Trace interactions**: Follow user interactions in `src/bevy_app/systems/ui/`
4. **Examine UI components**: Look at the Leptos UI in `src/leptos_app/`