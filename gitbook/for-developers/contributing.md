# Contributing

Welcome to BERT development! This guide helps systems scientists, engineers, and developers contribute effectively to the BERT system modeling platform while maintaining alignment with systems science principles and professional development standards.

## Quick Start

### Prerequisites

1. **Install Rust**: Follow [rustup.rs](https://rustup.rs/) for the latest stable toolchain
2. **Install Tauri Prerequisites**: Follow [Tauri prerequisites guide](https://tauri.app/v1/guides/getting-started/prerequisites)
3. **Clone the Repository**:
   ```bash
   git clone git@github.com:halcyonic-systems/bert.git
   cd bert
   ```

### Development Setup

1. **Install Dependencies**:
   ```bash
   cargo install cargo-watch
   npm install
   ```

2. **Run Development Server**:
   ```bash
   cargo tauri dev
   ```

3. **Verify Installation**:
   - Application should open with the BERT interface
   - Try creating a simple system model
   - Verify save/load functionality

### First Contribution

1. **Read Architecture Documentation**: Start with the [Architecture](architecture.md) section
2. **Review Coding Standards**: Check the development standards below
3. **Choose Your First Issue**: Look for "good first issue" labels on GitHub

## Development Standards

### Code Quality Requirements

**All contributions must follow our professional development standards:**

- **Documentation**: 100% compliance with documentation templates
- **Code Style**: Use `rustfmt` and `clippy` - no warnings allowed
- **Testing**: Comprehensive test coverage for new functionality
- **Performance**: Maintain 60+ FPS in visualization systems
- **Architecture**: Follow established patterns and Layer 4 System Language implementation

### Mandatory Tools and Checks

```bash
# Before submitting any PR
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo doc --no-deps --quiet
```

### Documentation Requirements

**All new code MUST include comprehensive documentation:**

- **Modules**: Complete module-level documentation with purpose and usage
- **Functions**: Full documentation with Parameters, Returns, Errors, Panics sections
- **Types**: Comprehensive struct/enum documentation with field descriptions
- **Systems**: System function documentation with ECS context
- **Plugins**: Plugin documentation with integration patterns

## Architecture Guidelines

### System Language Implementation

BERT implements a **Layer 4 System Language** architecture with clear separation of concerns:

1. **Layer 1 - Foundation**: Bevy ECS providing entity-component architecture
2. **Layer 2 - System Elements**: Core components representing system entities  
3. **Layer 3 - Knowledge Representation**: Data model and serialization
4. **Layer 4 - System Orchestration**: Event-driven coordination and UI integration

### Architectural Principles

#### 1. Separation of Concerns
- **UI Logic**: Leptos components (`src/leptos_app/`)
- **Visualization Logic**: Bevy systems (`src/bevy_app/systems/`)
- **System Modeling**: Components and data model (`src/bevy_app/components/`, `src/bevy_app/data_model/`)
- **Integration Logic**: Bundles and spawn systems (`src/bevy_app/bundles/`)

#### 2. Event-Driven Design
- Use trigger events for system communication
- Avoid tight coupling between components
- Maintain clear data flow patterns
- Implement proper event scheduling and dependencies

#### 3. Conceptual Integrity
- **Changes must align with System Language theory**
- **Maintain consistent terminology in code and documentation**
- **Reference specific files and components when communicating**
- **Include diagrams when explaining complex changes**
- **Relate changes back to system language concepts**

### File Organization

```
src/
├── bevy_app/                    # Core system implementation
│   ├── components/              # System element definitions
│   ├── bundles/                 # Entity creation patterns
│   ├── systems/                 # System orchestration (Layer 4)
│   ├── data_model/              # Knowledge representation (Layer 3)
│   ├── plugins/                 # System integration
│   └── resources/               # Global state management
├── leptos_app/                  # User interface
└── events.rs                    # Cross-system communication
```

## Systems Science Integration

### Theoretical Foundations

**All contributions should understand and maintain alignment with:**

- **General Systems Theory**: BERT models systems as bounded entities with inputs, outputs, and internal processes
- **Hierarchical Systems**: Support for system decomposition and emergence
- **System Boundaries**: Clear interface definitions and permeability concepts
- **Flow Modeling**: Energy, material, and information exchange patterns
- **Complexity Theory**: Support for atomic, complex, and multiset system types

### Implementation Guidelines

#### 1. System Element Modeling
- **Atomic Systems**: Indivisible entities with simple, predictable behaviors
- **Complex Systems**: Decomposable entities with emergent properties and adaptability
- **Multiset Systems**: Collections of identical components with capacity-based behavior

#### 2. Boundary Management
- **Interfaces**: Formal connection points for system interaction
- **Porosity**: Controlled permeability for flow exchange
- **Protocol Definition**: Structured interaction patterns

#### 3. Flow Characterization
- **Substance Types**: Energy, Material, Message classifications
- **Interaction Types**: Flow vs. Force distinction
- **Usability Patterns**: Directional and utility classifications
- **Parameter Systems**: Extensible property modeling

## Development Workflow

### Branch Strategy

```bash
# Feature development
git checkout -b feature/descriptive-name

# Bug fixes  
git checkout -b fix/issue-description

# Documentation updates
git checkout -b docs/area-being-documented
```

### Commit Standards

**Use conventional commit format:**
```
type(scope): description

feat(data-model): add complexity parameter serialization
fix(mouse): resolve selection state persistence issue  
docs(bundles): add comprehensive spawn system documentation
refactor(systems): optimize flow rendering performance
```

### Pull Request Process

1. **Create Feature Branch**: From latest `main`
2. **Make Changes**: Following all standards and guidelines
3. **Run Tests**: Ensure existing tests pass
4. **Submit Pull Request**: With comprehensive description and testing notes
5. **Address Reviews**: Respond to all feedback promptly
6. **Merge**: Only after approval and all checks passing

## Common Development Tasks

### Adding New System Elements

**Process**:
1. **Component Definition**: Add to `system_elements.rs`
2. **Visual Representation**: Create spawn bundle
3. **UI Integration**: Add Leptos editing components
4. **Serialization**: Update data model
5. **Documentation**: Complete template compliance

**Files to modify**:
- `src/bevy_app/components/system_elements.rs`
- `src/bevy_app/bundles/spawn/[element].rs`
- `src/leptos_app/components/button.rs`
- `src/leptos_app/details.rs`
- `src/bevy_app/data_model/save.rs` and `load.rs`

### Enhancing Visualization

**Focus areas**:
- Rendering performance (maintain 60+ FPS)
- Visual clarity and information density
- Systems science pedagogical value
- Integration with existing visual elements

**Key files**:
- `src/bevy_app/systems/ui/[system].rs`
- `src/bevy_app/bundles/spawn/[element].rs`

### UI Improvements

**Considerations**:
- User experience and workflow efficiency
- Consistency with overall design language
- Accessibility and usability
- Integration with existing UI patterns

**Key files**:
- `src/leptos_app/components/[component].rs`
- `src/leptos_app/details.rs`
- `styles.css`

### Performance Optimization

**Requirements**:
- Performance profiling and bottleneck identification
- Measurable improvement targets
- No functionality regression
- Scalability with complex system models

**Approach**:
- Profile before optimizing
- Implement efficient algorithms and data structures
- Comprehensive before/after testing
- Document performance characteristics

## Testing Requirements

### Test Categories

1. **Unit Tests**: Individual component functionality
2. **Integration Tests**: System interaction patterns
3. **Performance Tests**: Rendering and interaction benchmarks
4. **User Acceptance Tests**: Workflow and usability validation

### Testing Standards

- **Coverage**: Comprehensive test coverage for new functionality
- **Performance**: Maintain 60+ FPS benchmarks
- **Regression**: Ensure existing functionality remains intact
- **Edge Cases**: Handle malformed data and unusual inputs

## Code Review Process

### Review Criteria

1. **Architectural Alignment**: Follows established patterns
2. **Systems Science Integration**: Maintains theoretical consistency
3. **Code Quality**: Meets documentation and style standards
4. **Performance Impact**: No degradation of user experience
5. **Testing Coverage**: Adequate test coverage and validation

### Review Process

1. **Automated Checks**: All CI checks must pass
2. **Peer Review**: At least one developer review required
3. **Systems Science Review**: Conceptual alignment verification
4. **Performance Review**: Impact assessment on visualization performance
5. **Documentation Review**: Template compliance and clarity

## Getting Help

### Resources

- **Architecture Documentation**: Comprehensive technical specifications in `docs/architecture/`
- **Contributing Guide**: Detailed development standards in `docs/contributing/`
- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: Community questions and design discussions

### Communication

- **GitHub Issues**: For bug reports and feature requests
- **Pull Requests**: For code contributions and reviews
- **Discussions**: For design questions and community input
- **Documentation**: For clarification and improvement suggestions

### Common Questions

**Q: How do I understand the codebase architecture?**
A: Start with the [Architecture](architecture.md) documentation, then explore the comprehensive architecture overview in the repository.

**Q: What documentation standards should I follow?**
A: All code must follow the professional documentation templates with 100% compliance requirements.

**Q: How do I ensure my changes align with systems science principles?**
A: Review the systems science integration guidelines and validate theoretical alignment before implementation.

**Q: What performance standards must I maintain?**
A: Visualization systems must maintain 60+ FPS, and all changes should be performance-tested.

---

**Ready to contribute?** Start by exploring the codebase, understanding the architecture, and choosing an issue that matches your interests and expertise level. Remember: BERT is built on solid systems science foundations, so every contribution should enhance our ability to model and understand complex systems! 🚀
