# Bevy App - Systems Modeling Engine

The core systems modeling engine built on the Bevy game framework. This directory contains the computational implementation of System Language theory.

## Architecture Overview

**Bevy App** handles:
- System modeling logic and behaviors
- Component definitions and relationships  
- Data persistence and serialization
- System analysis and complexity calculations

## Key Directories

- **`components/`** - Core system modeling components
  - `system_elements.rs` - Fundamental modeling primitives
- **`systems/`** - System behaviors and update logic
- **`data_model/`** - Save/load functionality and serialization
- **`bundles/`** - Component groupings for spawn operations
- **`resources/`** - Global state management
- **`plugins/`** - Bevy plugin organization

## Integration with Leptos UI

The Bevy app runs embedded within the Leptos UI framework:
- Bevy handles systems modeling computation
- Leptos manages user interface and interactions
- Communication via events and shared resources

## Development Patterns

- **Components**: Define what entities have (data)
- **Systems**: Define what entities do (behavior)  
- **Events**: Handle communication between systems
- **Resources**: Manage global state

## Key Files

- **`mod.rs`** - Main Bevy app setup and configuration
- **`constants.rs`** - System-wide constants and configuration
- **`events.rs`** - Custom event definitions
- **`states.rs`** - Application state management