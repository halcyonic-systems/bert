# Contributing to BERT

Thanks for your interest in contributing! BERT is a visual toolkit for modeling complex systems, built with Rust + Leptos + Bevy.

## Quick Start

### Prerequisites

1. **Rust**: Install via [rustup.rs](https://rustup.rs/)
2. **Tauri Prerequisites**: Follow the [Tauri guide](https://tauri.app/v1/guides/getting-started/prerequisites)
3. **Node.js**: For Tailwind CSS processing

### Setup

```bash
git clone git@github.com:halcyonic-systems/bert.git
cd bert
npm install
cargo tauri dev        # Desktop app
# OR
trunk serve            # Web at localhost:1320
```

**Verify**: App window opens, toolbar visible, you can create and save a model.

**Alternative**: Open in VS Code and select "Reopen in Container" for a pre-configured dev environment.

## Ways to Contribute

1. **Bug fixes** - Pick an issue labeled `good first issue`
2. **Documentation** - Improve clarity, fix errors, add examples
3. **Features** - Implement new functionality (discuss in an issue first)
4. **Testing** - Add test coverage for existing code

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/my-feature   # New feature
git checkout -b fix/issue-description  # Bug fix
git checkout -b docs/what-you-changed  # Documentation
```

### 2. Make Changes

**Project structure:**
```
src/
├── bevy_app/           # System model and visualization
│   ├── components/     # Core data types
│   ├── bundles/spawn/  # Entity creation
│   ├── systems/        # ECS systems
│   └── data_model/     # Save/load
└── leptos_app/         # UI components
```

### 3. Run Quality Checks

```bash
cargo fmt --all                           # Format code
cargo clippy --all-targets -- -D warnings # Lint (must pass with no warnings)
cargo test --all                          # Run tests
```

### 4. Commit

Use [conventional commits](https://www.conventionalcommits.org/):

```
feat(data-model): add complexity parameter
fix(mouse): resolve selection persistence issue
docs(readme): clarify setup instructions
refactor(systems): optimize flow rendering
```

### 5. Submit a Pull Request

- Clear title describing the change
- Reference any related issues
- Describe what changed and why
- Include screenshots for UI changes

## Common Tasks

### Add a Property to System Elements

1. `src/bevy_app/components/system_elements.rs` - Add the field
2. `src/leptos_app/details.rs` - Add UI for editing
3. `src/bevy_app/data_model/save.rs` and `load.rs` - Update serialization

### Change Visual Appearance

1. `src/bevy_app/bundles/spawn/main_system.rs` - Visual definition
2. `src/bevy_app/systems/ui/color.rs` - Color handling

### Modify Flow Rendering

1. `src/bevy_app/systems/ui/flow/curve.rs` - Curve rendering
2. `src/bevy_app/bundles/spawn/flow.rs` - Flow creation

## Code Standards

- **Format**: Run `cargo fmt` before committing
- **Linting**: No clippy warnings allowed
- **Documentation**: Document public functions (see [docs/DOCUMENTATION_GUIDELINES.md](docs/DOCUMENTATION_GUIDELINES.md))
- **Performance**: Maintain 60+ FPS in visualization

## Architecture Overview

BERT uses a layered architecture:

| Layer | Purpose | Location |
|-------|---------|----------|
| UI | User interface | `src/leptos_app/` |
| Visualization | Rendering system models | `src/bevy_app/systems/` |
| Data Model | Core types and persistence | `src/bevy_app/components/`, `data_model/` |
| Desktop | Native app wrapper | `src-tauri/` |

For deeper architectural details, see [gitbook/for-developers/architecture/](gitbook/for-developers/architecture/).

## Exploration Branches

These branches contain experimental work kept for reference. They're 80+ commits behind `main` - don't merge directly, but reference for architectural ideas:

| Branch | Purpose |
|--------|---------|
| `feature/concept-dictionary` | Chat UI + ontology exploration |
| `feature/agent-dynamics` | Temporal simulation engine |
| `feature/agent-params-v2` | Agent configuration UI |

## Getting Help

- **GitHub Issues** - Bug reports, feature requests, and questions
- **Pull Request Comments** - Code-specific discussions

Don't hesitate to open an issue to ask questions! We'd rather help you succeed than have you give up.

## Review Checklist

Before submitting, ensure:

- [ ] Code compiles without warnings
- [ ] `cargo fmt` and `cargo clippy` pass
- [ ] Tests pass
- [ ] Public functions are documented
- [ ] UI changes include screenshots

---

Thank you for contributing to BERT!
