# Contributing to BERT

Welcome to BERT development! This guide helps systems scientists, engineers, and developers contribute effectively to the BERT system modeling platform while maintaining alignment with systems science principles and professional development standards.

## Table of Contents

- [Quick Start](#quick-start)
- [Development Standards](#development-standards)
- [Architecture Guidelines](#architecture-guidelines)
- [Systems Science Integration](#systems-science-integration)
- [Development Workflow](#development-workflow)
- [Task Assignment Framework](#task-assignment-framework)
- [Common Development Tasks](#common-development-tasks)
- [Code Review Process](#code-review-process)
- [Testing Requirements](#testing-requirements)
- [Documentation Requirements](#documentation-requirements)
  - [Feature Documentation Process](#feature-documentation-process)

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

1. **Read Architecture Documentation**: Start with [`docs/architecture/comprehensive-architecture-overview.md`](../architecture/comprehensive-architecture-overview.md)
2. **Review Coding Standards**: Read [`rust-documentation-guidelines.md`](rust-documentation-guidelines.md)
3. **Check Current Status**: Review [`documentation-implementation-analysis.md`](documentation-implementation-analysis.md)
4. **Choose Your First Issue**: Look for "good first issue" labels on GitHub

### Contribution Types

There are several ways you can contribute to BERT:

1. **Documentation improvements**: Clarify concepts, add examples, improve tutorials
2. **Bug fixes**: Address issues in existing functionality
3. **Feature enhancements**: Add new capabilities to existing features
4. **New features**: Implement entirely new functionality
5. **Conceptual development**: Extend the System Language framework

## Development Standards

### Code Quality Standards

**All contributions must follow our professional development standards:**

- **Documentation**: 100% compliance with [`rust-documentation-guidelines.md`](rust-documentation-guidelines.md)
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

### Documentation Template Usage

**All new code MUST use templates from [`.cursor/rules/commenting-guidelines.mdc`](../../.cursor/rules/commenting-guidelines.mdc):**

- **Modules**: Use Module Template
- **Functions**: Use Function Template with complete Parameters, Returns, Errors, Panics sections
- **Types**: Use Struct/Enum Template
- **Traits**: Use Trait Template
- **Systems**: Use System Function Template
- **Plugins**: Use Plugin Template

## Architecture Guidelines

### System Language Implementation

BERT implements a **Layer 4 System Language** architecture with clear separation of concerns:

1. **Layer 1 - Foundation**: Bevy ECS providing entity-component architecture
2. **Layer 2 - System Elements**: Core components representing system entities  
3. **Layer 3 - Knowledge Representation**: Data model and serialization (extensively documented)
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

### File Organization Patterns

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

### Conceptual Validation

**Before implementing new features, validate:**
1. **Theoretical Alignment**: Does this fit established systems science principles?
2. **Modeling Capability**: Does this enhance system representation capabilities?
3. **Integration Impact**: How does this affect existing system modeling patterns?
4. **User Mental Model**: Does this align with how systems scientists think about problems?

## Development Workflow

### Branch Strategy

```bash
# Feature development
git checkout -b feature/descriptive-name
# Then generate feature documentation
./scripts/bert.sh feature "Feature Name"

# Bug fixes  
git checkout -b fix/issue-description

# Documentation updates
git checkout -b docs/area-being-documented
```

> **Note:** After creating a feature branch, immediately generate feature documentation using `./scripts/bert.sh feature "Feature Name"` to establish the documentation for your work.

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
2. **Generate Documentation**: Run `./scripts/bert.sh feature "Feature Name"`
3. **Make Changes**: Following all standards and guidelines
4. **Update Documentation**: Complete all sections of feature documentation
5. **Run Tests**: Ensure existing tests pass
6. **Submit Pull Request**: With comprehensive description and link to feature documentation
7. **Address Reviews**: Respond to all feedback promptly
8. **Merge**: Only after approval and all checks passing

## Task Assignment Framework

This framework provides structured formats for assigning development tasks while maintaining conceptual alignment with systems science principles.

### Task Categories and Templates

#### 1. System Element Enhancement
**Objective**: Add or modify system modeling capabilities

```markdown
# System Element: [Element Name]

## Objective
Add a new type of system element representing [concept description]

## Systems Science Context
[Explanation of the element's role in systems theory]
[Reference to relevant literature or principles]

## Technical Requirements
1. Create component definition with properties and constraints
2. Implement visual representation with appropriate symbology  
3. Add creation and editing UI controls
4. Implement serialization support for persistence
5. Define interaction patterns with existing elements

## Implementation Approach
- Component: `src/bevy_app/components/system_elements.rs`
- Bundle: `src/bevy_app/bundles/spawn/[element].rs`  
- UI Integration: `src/leptos_app/components/button.rs` and `src/leptos_app/details.rs`
- Serialization: `src/bevy_app/data_model/save.rs` and `load.rs`
- Documentation: Full template compliance required

## Validation Criteria
1. Theoretical alignment with systems science principles
2. Integration with existing system modeling patterns
3. UI functionality and user experience evaluation
4. Save/load cycle integrity testing
5. Performance impact assessment
```

#### 2. Visualization Enhancement
**Objective**: Improve system representation and user comprehension

```markdown
# Visualization: [Feature Name]

## Objective
Improve the visual representation of [system aspect] to better convey [concept]

## Systems Science Context
[Explanation of visual representation's role in understanding systems]

## Technical Requirements
1. Rendering performance targets (60+ FPS maintained)
2. Visual design alignment with systems concepts
3. Interactive behavior patterns and feedback
4. Accessibility considerations
5. Scalability with complex system models

## Implementation Approach
- Rendering: `src/bevy_app/systems/ui/[system].rs`
- Visual Assets: Modify `src/bevy_app/bundles/spawn/[element].rs`
- Performance: Efficient rendering patterns and optimization
- Documentation: System function template compliance

## Validation Criteria
1. Performance benchmarks met (60+ FPS)
2. Visual clarity and information density optimization
3. Systems science pedagogical value assessment
4. User testing feedback and iteration
5. Integration with existing visual elements
```

#### 3. UI Enhancement
**Objective**: Improve user interface and interaction patterns

```markdown
# UI Enhancement: [Feature Name]

## Objective
[Brief description of the UI feature to be enhanced]

## Systems Science Context
[Explanation of how this relates to systems concepts and user mental models]

## Technical Requirements
1. Specific UI component modifications
2. Interaction behavior changes and feedback
3. Visual design considerations and consistency
4. Accessibility and usability requirements
5. Integration with existing UI patterns

## Implementation Approach
- Components: `src/leptos_app/components/[component].rs`
- Properties: `src/leptos_app/details.rs` (if modifying property panels)
- Styling: Update `styles.css` for visual consistency
- Documentation: Complete function template compliance

## Validation Criteria
1. Specific UI interactions function correctly
2. Expected outcomes verified through testing
3. User experience improvements measured
4. Consistency with overall UI design language
5. Performance impact on UI responsiveness
```

#### 4. Integration Feature
**Objective**: Connect BERT with external systems or standards

```markdown
# Integration: [External System/Standard]

## Objective
Enable integration with [external system/tool/framework]

## Systems Science Context
[Explanation of how this integration extends system analysis capabilities]

## Technical Requirements
1. Data format compatibility and conversion algorithms
2. Import/export functionality with comprehensive validation
3. Error handling for malformed data and edge cases
4. Performance considerations for large datasets
5. User interface for managing integration operations

## Implementation Approach
- Integration Module: Create `src/bevy_app/integrations/[name]/`
- Data Validation: Comprehensive error checking and reporting
- UI Components: User-friendly import/export interface
- Documentation: Complete API and usage documentation

## Validation Criteria
1. Successful data round-trip testing
2. Error handling for edge cases and malformed inputs
3. Performance with realistic dataset sizes
4. Integration with existing workflow patterns
5. User testing with target external systems
```

#### 5. Performance Optimization
**Objective**: Maintain responsiveness with complex system models

```markdown
# Optimization: [Performance Area]

## Objective
Improve performance of [specific functionality] by [target improvement]

## Systems Science Context
[Explanation of how performance relates to handling complex systems]

## Technical Requirements
1. Performance profiling and bottleneck identification
2. Optimization implementation with specific techniques
3. Benchmark suite development and measurement
4. Regression testing for functionality preservation
5. Scalability testing with large system models

## Implementation Approach
- Profiling: Identify specific performance bottlenecks
- Optimization: Implement efficient algorithms and data structures
- Testing: Before/after performance measurements
- Documentation: Performance characteristics documentation

## Validation Criteria
1. Measurable performance improvement (specific targets)
2. No functionality regression verification
3. Scalability with increasing system complexity
4. Memory usage optimization
5. Real-world scenario testing
```

#### 6. Data Model Extension
**Objective**: Extend system representation capabilities

```markdown
# Data Model Extension: [Feature Name]

## Objective
Extend the system representation to support [new capability]

## Systems Science Context
[Explanation of how this extension relates to system knowledge representation]

## Technical Requirements
1. Add new properties/relationships to the data model
2. Implement backward compatibility with existing saved files
3. Add validation for specific constraints
4. Update serialization/deserialization logic
5. Maintain data integrity across operations

## Implementation Approach
- Data Model: `src/bevy_app/data_model/save.rs` and `load.rs`
- Components: Update associated component definitions
- Validation: Implement constraint checking and error reporting
- Documentation: Complete data model documentation

## Validation Criteria
1. Test fixtures with new and old format data
2. Correct loading of both formats verified
3. Edge cases for validation testing
4. Data integrity across save/load cycles
5. Performance impact assessment
```

#### 7. Conceptual Framework Extension
**Objective**: Extend BERT's theoretical foundation

```markdown
# Conceptual Framework Extension: [Feature Name]

## Objective
Extend BERT's theoretical foundation to support [new systems concept]

## Systems Science Context
[Detailed explanation of the concept and its theoretical basis]
[References to relevant literature and established theory]

## Technical Requirements
1. Define formal representation of the new concept
2. Implement component structure in code
3. Create appropriate visual representation
4. Add user interface for concept manipulation
5. Update comprehensive documentation

## Implementation Approach
- Documentation: Create concept explanation in `docs/`
- Components: Define core model components
- Visualization: Implement visual representation
- UI Controls: Add interface for manipulation
- Integration: Connect with existing system elements

## Validation Criteria
1. Validation with systems science experts
2. Example models demonstrating the concept
3. Integration testing with existing system elements
4. Theoretical consistency verification
5. Pedagogical value assessment
```

## Common Development Tasks

### Adding a New System Element Type

**Step-by-step process:**

1. **Define the Component**: Add to `src/bevy_app/components/system_elements.rs`
   - Use proper documentation templates
   - Include all required properties
   - Add complexity and adaptability support

2. **Create Spawn Bundle**: New file in `src/bevy_app/bundles/spawn/`
   - Implement visual representation
   - Add interaction capabilities
   - Include proper documentation

3. **Add UI Integration**: Update Leptos components
   - Creation button in toolbar
   - Property editing in details panel
   - Form validation and user feedback

4. **Update Data Model**: Modify serialization
   - Add to save/load functions
   - Maintain backward compatibility
   - Include comprehensive error handling

5. **Add Documentation**: Complete template compliance
   - All functions fully documented
   - Examples and usage patterns
   - Integration with existing elements

### Modifying Visual Representation

1. **Locate Spawn Bundle**: Find relevant file in `src/bevy_app/bundles/spawn/`
2. **Modify Shape Parameters**: Update geometry and styling
3. **Update Interaction Systems**: Modify associated systems for new visuals
4. **Test Performance**: Ensure 60+ FPS maintained
5. **Document Changes**: Update function documentation

### Adding Properties to Elements

1. **Update Component Definition**: Add new properties with proper types
2. **Extend UI Controls**: Add editing capabilities in details panel
3. **Update Serialization**: Include properties in save/load operations
4. **Add Validation**: Implement constraint checking
5. **Document Properties**: Complete template compliance

## Code Review Process

### Review Criteria

**All pull requests must meet:**

#### Technical Standards
- [ ] **Feature Documentation**: Complete documentation in docs/features/
- [ ] **Code Documentation**: 100% template compliance
- [ ] **Testing**: Comprehensive coverage and passing tests
- [ ] **Performance**: No regression, meets targets
- [ ] **Architecture**: Follows established patterns
- [ ] **Code Quality**: No clippy warnings, proper formatting

#### Systems Science Alignment
- [ ] **Conceptual Integrity**: Aligns with systems theory
- [ ] **Terminology**: Consistent systems science language
- [ ] **Modeling Capability**: Enhances system representation
- [ ] **User Experience**: Supports systems thinking workflows

#### Integration Quality
- [ ] **Cross-System**: Proper integration with existing components
- [ ] **Event Patterns**: Correct event-driven implementation
- [ ] **Data Flow**: Clear and maintainable data patterns
- [ ] **Boundary Management**: Proper interface definitions

### Review Process

1. **Automated Checks**: CI/CD pipeline verification
2. **Technical Review**: Code quality and architecture assessment  
3. **Domain Review**: Systems science alignment verification
4. **Integration Testing**: End-to-end functionality validation
5. **Documentation Review**: Template compliance and clarity
6. **Performance Review**: Benchmark and scalability assessment

## Testing Requirements

### Test Categories

#### Unit Tests
- **Component Logic**: Individual component behavior
- **Data Model**: Serialization/deserialization accuracy
- **Utility Functions**: Pure function correctness
- **Performance**: Critical path benchmarking

#### Integration Tests  
- **System Coordination**: Multi-system interaction patterns
- **UI Integration**: Leptos-Bevy communication
- **Data Flow**: End-to-end data processing
- **Event Systems**: Trigger event coordination

#### Systems Tests
- **Complete Workflows**: Full user interaction scenarios
- **Performance**: Large-scale system model handling
- **Persistence**: Save/load cycle integrity
- **Cross-Platform**: Tauri deployment verification

### Testing Standards

```bash
# Required test commands
cargo test --all                    # All unit and integration tests
cargo test --doc                    # Documentation example tests  
cargo bench                        # Performance benchmarks
cargo tauri build                   # Full application build test
```

### Testing Approach Templates

**For each task category, include specific testing requirements:**

- **Element Addition**: Creation, editing, persistence, interaction testing
- **Visualization**: Performance measurement, visual inspection, usability testing
- **UI Enhancement**: Interaction testing, accessibility verification, user experience
- **Integration**: Round-trip testing, error handling, performance with large datasets
- **Optimization**: Before/after benchmarks, regression testing, scalability
- **Data Model**: Format compatibility, validation, backward compatibility

## Documentation Requirements

### Mandatory Documentation

**Every contribution MUST include:**

1. **Feature Documentation**: Complete feature documentation in docs/features/
2. **Code Documentation**: 100% template compliance
3. **Architecture Documentation**: System integration patterns
4. **Usage Examples**: Practical application demonstrations
5. **Performance Documentation**: Characteristics and limitations
6. **Systems Science Context**: Theoretical alignment explanation

### Feature Documentation Process

1. **Generate Template**: Use `./scripts/bert.sh feature "Feature Name"` at the start of development
2. **Fill Initial Sections**: Complete overview, goals, and requirements sections
3. **Update Throughout Development**: Document design decisions and implementation details as you go
4. **Complete Before PR**: Ensure documentation is complete with examples and testing approach
5. **Reference in Commits**: Include references to feature documentation in commit messages

### Documentation Workflow

1. **Template Selection**: Choose appropriate template from commenting guidelines
2. **Content Creation**: Complete all required sections
3. **Integration**: Cross-reference with existing documentation
4. **Validation**: Compile and accuracy verification
5. **Review**: Technical and pedagogical assessment

### Quality Standards

- **Clarity**: Clear, jargon-free explanations
- **Completeness**: All template sections fully completed
- **Accuracy**: Technical and theoretical correctness
- **Examples**: Practical, compilable code examples
- **Integration**: Proper cross-referencing and linking

---

## Communication Guidelines

### Language and Terminology

- **Use clear, systems-oriented language**
- **Reference specific files and components**
- **Include diagrams when explaining complex changes**
- **Relate changes back to system language concepts**
- **Maintain consistency with established terminology**

### Documentation Integration

- **Cross-reference related components and systems**
- **Link to relevant theoretical concepts**
- **Include practical examples and use cases**
- **Update related documentation when making changes**

---

## Getting Help

### Resources

- **Architecture Documentation**: [`docs/architecture/`](../architecture/)
- **Development Standards**: [`rust-documentation-guidelines.md`](rust-documentation-guidelines.md)
- **Feature Documentation**: [`docs/contributing/feature-documentation-process.md`](feature-documentation-process.md)
- **Feature Template**: [`docs/contributing/feature-template.md`](feature-template.md)
- **Current Status**: [`documentation-implementation-analysis.md`](documentation-implementation-analysis.md)
- **Comprehensive Architecture**: [`docs/architecture/comprehensive-architecture-overview.md`](../architecture/comprehensive-architecture-overview.md)

### Communication Channels

- **GitHub Issues**: Technical problems and feature requests
- **Pull Request Comments**: Code-specific discussions
- **Architecture Questions**: Reference existing documentation first

### Escalation Process

1. **Documentation Review**: Check existing guides and specifications
2. **Issue Search**: Look for existing GitHub issues
3. **Create Issue**: Detailed problem description with context
4. **Discussion**: Engage with maintainers and community

---

**Welcome to BERT development!** By following these guidelines, you're contributing to a professional, theoretically-grounded system modeling platform that advances both software engineering and systems science. Thank you for maintaining our high standards and helping build the future of system modeling tools. 