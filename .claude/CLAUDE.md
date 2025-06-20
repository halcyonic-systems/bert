# Claude's Guide to BERT Development

## What is BERT?

BERT (Bounded Entity Reasoning Toolkit) is a visual system modeling application that helps users understand complex systems through boundary analysis and entity relationship mapping. It's designed to implement the System Language (SL) framework for bounded entity reasoning.

It's built with:
- **Rust** as the primary language
- **Bevy** game engine for visualization
- **Leptos** for reactive UI components
- **Tauri** for desktop integration

Key features:
- System element visualization (systems, interfaces, flows, external entities)
- Boundary analysis tools
- Entity relationship mapping
- Screenshot/image export capabilities
- LLM chat integration

## Project Architecture

BERT uses a hybrid web/desktop architecture with three primary modules:

1. **Bevy App** (`src/bevy_app/`): Core visualization engine using Bevy ECS
   - Components: System elements, connections, UI elements
   - Systems: Setup, camera control, interaction handling
   - Plugins: Selection, labels, mouse interaction

2. **Leptos App** (`src/leptos_app/`): Reactive UI components using Leptos
   - Components: Input controls, panels, tree view
   - Integration with Bevy via signal synchronization

3. **Tauri** (`src-tauri/`): Native desktop integration
   - File system operations
   - Dialog management
   - Native window controls

## System Element Framework

BERT implements a formal System Language framework with these core components:

- **Systems**: Bounded entities with internal structure (rendered as circles)
- **Interfaces**: Connection points defining system boundaries (rendered as rectangles)
- **Flows**: Interactions representing substance movement (rendered as lines with arrows)
- **External Entities**: Sources and sinks outside system boundaries

## Development Workflow

When working on BERT code:

1. **Follow established patterns**:
   - Check `.cursor/rules/patterns.mdc` for code patterns
   - Review similar components before creating new ones
   - Use proper module organization

2. **Leptos component development**:
   - Use Signal-based state management
   - Follow reactive patterns with `move || signal.get()`
   - Use `.run()` method for callbacks (Leptos 0.7+)
   - Follow component file structure in `src/leptos_app/components/`

3. **Bevy system development**:
   - Respect ECS architecture patterns
   - Use appropriate system sets
   - Follow established component organization
   - Use Bevy's resource management correctly

4. **Tauri integration**:
   - File operations should use Tauri commands
   - Dialog management through Tauri plugins
   - Proper error handling for native operations

## Common Commands

- `cargo tauri dev`: Start the application in development mode
- `cargo build`: Build the project
- `cargo clippy`: Run linting checks
- `cargo fmt`: Format code according to standards
- `git checkout -b feature/your-feature-name`: Create a new feature branch

## Debugging Support

When encountering issues:

1. **Leptos-specific issues**:
   - Check callback syntax (use `.run()` for Leptos 0.7+)
   - Verify reactive contexts are properly established
   - Check signal management and component lifecycle

2. **Bevy-specific issues**:
   - Verify entity/component relationships
   - Check system ordering and sets
   - Validate resource initialization

3. **Tauri-specific issues**:
   - Check build pipeline errors
   - Verify command invocation patterns
   - Handle file system errors properly

## Branch Management Guidelines

- Each branch should contain exactly one feature
- Use clear naming conventions:
  - `feature/feature-name` for new features
  - `fix/issue-description` for bug fixes
  - `docs/documentation-change` for documentation updates
- Keep branches up-to-date with main
- Follow cursor rules for consistent development

## Code Style Requirements

1. **Rust conventions**:
   - Follow standard Rust patterns
   - Use proper error handling (avoid `.unwrap()` in production code)
   - Document public interfaces with rustdoc
   - Use descriptive variable and function names

2. **Leptos conventions**:
   - PascalCase for component names
   - Use `#[prop(into)]` for signal props
   - Prefer `Signal<T>` over `ReadSignal<T>` for props
   - Follow established component patterns

3. **Bevy conventions**:
   - Use appropriate component and resource patterns
   - Follow Bevy's established naming conventions
   - Organize systems into appropriate sets
   - Respect entity lifecycle management

## Feature Implementation Process

1. **Analysis**: Understand existing patterns and component relationships
2. **Planning**: Identify files requiring modification and integration points
3. **Implementation**: Make incremental changes, testing compilation frequently
4. **Integration**: Ensure proper styling, documentation, and testing
5. **Review**: Verify changes work as expected with no regressions

## IMPORTANT REMINDERS

1. **NEVER** use `.unwrap()` or `.expect()` in production code
2. Use Leptos 0.7+ patterns (`.run()` method for callbacks)
3. Follow proper module exports in `mod.rs` files
4. Use absolute imports (`crate::module::Component`)
5. Test compilation after each significant change
6. Follow error handling patterns appropriate for context
7. Maintain separation between Bevy, Leptos, and Tauri components
8. Run commands from repository root to ensure proper execution

## Testing Requirements

Before submitting changes:
- Verify code compiles without errors
- Check for regressions in existing functionality
- Test component integration with UI
- Verify responsive behavior
- Test edge cases and error conditions