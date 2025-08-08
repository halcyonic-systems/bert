# Getting Started - Developer Guide

Welcome to BERT development! This guide covers environment setup, code navigation, and development workflows.

## Prerequisites

1. **Rust**: Install via [rustup.rs](https://rustup.rs/)
2. **Tauri Prerequisites**: Follow the [Tauri guide](https://tauri.app/v1/guides/getting-started/prerequisites)

## Development Setup

```bash
git clone git@github.com:halcyonic-systems/bert.git
cd bert
cargo tauri dev
```

Application opens with BERT interface - try creating a system model to verify everything works.

## Project Structure Overview

Understanding the codebase structure helps you navigate and contribute effectively:

```
bert/
├── src/                    # Main source code
│   ├── bevy_app/           # System model and visualization (Bevy)
│   ├── leptos_app/         # User interface components (Leptos)
│   └── main.rs             # Application entry point
├── src-tauri/              # Tauri desktop integration
├── assets/                 # Static assets (icons, fonts)
├── docs/                   # Technical documentation
└── private-dev/            # Research and development notes
```

### Key Modules and Files

**User Interface (Leptos):**
- `src/leptos_app/mod.rs`: Main UI component definition
- `src/leptos_app/components/`: Reusable UI components (buttons, sliders, etc.)
- `src/leptos_app/details.rs`: Property panel for editing element properties
- `src/leptos_app/tree/`: System hierarchy tree view

**System Model and Visualization (Bevy):**
- `src/bevy_app/mod.rs`: Main Bevy app setup
- `src/bevy_app/components/`: Core data model components
  - `system_elements.rs`: Definitions for systems, subsystems, etc.
  - `connections.rs`: Definitions for flows and interfaces
- `src/bevy_app/bundles/spawn/`: Entity creation logic
- `src/bevy_app/systems/`: Bevy ECS systems for updating the model
- `src/bevy_app/data_model/`: Serialization/deserialization

## Common Development Scenarios

### "I want to add a new property to system elements"

Modify:
1. `src/bevy_app/components/system_elements.rs` to add the property
2. `src/leptos_app/details.rs` to add UI for editing
3. `src/bevy_app/data_model/save.rs` and `load.rs` for persistence

### "I want to change how a system element looks"

Look at:
1. `src/bevy_app/bundles/spawn/main_system.rs` for visual definition
2. `src/bevy_app/systems/ui/color.rs` for color handling

### "I want to change how connections are drawn"

Look at:
1. `src/bevy_app/systems/ui/flow/curve.rs` for flow rendering
2. `src/bevy_app/bundles/spawn/flow.rs` for flow creation

## Development Standards

BERT maintains professional development standards:

- **100% Documentation Compliance** - All code must follow template guidelines
- **No Clippy Warnings** - Code must pass `cargo clippy --all-targets -- -D warnings`
- **Comprehensive Testing** - Unit + integration tests required
- **Feature Documentation** - Use `./scripts/bert.sh feature "Feature Name"`

## First Contribution

1. **Read the [Contributing Guide](contributing.md)** - Complete workflow details
2. **Review [Documentation Standards](https://github.com/halcyonic-systems/bert/blob/main/docs/technical/rust-documentation-guidelines.md)** - Template requirements
3. **Check Current Status** - See [implementation analysis](https://github.com/halcyonic-systems/bert/blob/main/docs/technical/documentation-implementation-analysis.md)
4. **Create Feature Branch** - Follow `feature/descriptive-name` pattern

## Key Workflows

**Adding Features:**
```bash
git checkout -b feature/my-feature
./scripts/bert.sh feature "My Feature"  # Generate documentation
# Implement + document + test
```

**Quality Checks:**
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings  
cargo test --all
```

## Development Learning Path

1. **Start at the model**: Understand the core components in `src/bevy_app/components/`
2. **Explore visualization**: See how components are rendered in `src/bevy_app/bundles/spawn/`
3. **Trace interactions**: Follow user interactions in `src/bevy_app/systems/ui/`
4. **Examine UI components**: Look at the Leptos UI in `src/leptos_app/`

## Next Steps

- Review the [Architecture Overview](architecture/comprehensive-architecture-overview.md)
- Explore [Architecture Decision Records](architecture/decisions/)
- Check out [Current Features](current-features.md) for implementation status