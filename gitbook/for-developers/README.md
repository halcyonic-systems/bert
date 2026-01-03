# For Developers

Welcome to BERT development! BERT is built with Rust + Leptos + Bevy using professional development standards and comprehensive documentation requirements.

## Quick Setup

### Prerequisites
1. **Rust**: Install via [rustup.rs](https://rustup.rs/)
2. **Tauri Prerequisites**: Follow the [Tauri guide](https://tauri.app/v1/guides/getting-started/prerequisites)

### Get Started
```bash
git clone git@github.com:halcyonic-systems/bert.git
cd bert
cargo tauri dev
```

Application opens with BERT interface - try creating a system model to verify everything works.

## Project Structure

Understanding the codebase structure helps you navigate and contribute effectively:

```
bert/
├── src/                    # Main source code
│   ├── bevy_app/           # System model and visualization (Bevy)
│   ├── leptos_app/         # User interface components (Leptos)
│   └── main.rs             # Application entry point
├── src-tauri/              # Tauri desktop integration
├── assets/                 # Static assets (icons, fonts)
└── docs/                   # Technical documentation
```

### Key Modules

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

## Common Development Tasks

### Add New Property to System Elements
1. `src/bevy_app/components/system_elements.rs` - Add the property
2. `src/leptos_app/details.rs` - Add UI for editing
3. `src/bevy_app/data_model/save.rs` and `load.rs` - Update persistence

### Change System Element Appearance
1. `src/bevy_app/bundles/spawn/main_system.rs` - Visual definition
2. `src/bevy_app/systems/ui/color.rs` - Color handling

### Modify Connection Rendering
1. `src/bevy_app/systems/ui/flow/curve.rs` - Flow rendering
2. `src/bevy_app/bundles/spawn/flow.rs` - Flow creation

## Development Standards

BERT maintains professional development standards:

- **100% Documentation Compliance** - All code must follow template guidelines
- **No Clippy Warnings** - Code must pass `cargo clippy --all-targets -- -D warnings`
- **Comprehensive Testing** - Unit + integration tests required
- **Feature Documentation** - Use `./scripts/bert.sh feature "Feature Name"`

## Development Workflows

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

## Learning Path

1. **Start at the model**: Understand core components in `src/bevy_app/components/`
2. **Explore visualization**: See how components render in `src/bevy_app/bundles/spawn/`
3. **Trace interactions**: Follow user interactions in `src/bevy_app/systems/ui/`
4. **Examine UI components**: Look at Leptos UI in `src/leptos_app/`

## Essential Resources

- [Contributing Guide](bert/bert/gitbook/for-developers/contributing.md) - Complete development workflow
- [Architecture Overview](bert/bert/gitbook/for-developers/architecture.md) - High-level technical overview  
- [Current Capabilities](../releases/README.md#current-capabilities-v020) - See what's available in v0.2.0

**External Documentation:**
- [Architecture Overview](https://github.com/halcyonic-systems/bert/blob/main/docs/architecture/comprehensive-architecture-overview.md)
- [Feature Documentation](https://github.com/halcyonic-systems/bert/tree/main/docs/features)
- [Documentation Standards](https://github.com/halcyonic-systems/bert/blob/main/docs/technical/rust-documentation-guidelines.md)

## First Contribution

1. **Read the [Contributing Guide](bert/bert/gitbook/for-developers/contributing.md)** - Complete workflow details
2. **Review Documentation Standards** - Template requirements (link above)
3. **Create Feature Branch** - Follow `feature/descriptive-name` pattern
4. **Generate Documentation** - Use `./scripts/bert.sh feature "Feature Name"`