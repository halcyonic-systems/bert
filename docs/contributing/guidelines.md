# Contributing to BERT

This guide helps systems scientists and developers understand how to contribute to BERT effectively.

## Contribution Types

There are several ways you can contribute to BERT:

1. **Documentation improvements**: Clarify concepts, add examples, improve tutorials
2. **Bug fixes**: Address issues in existing functionality
3. **Feature enhancements**: Add new capabilities to existing features
4. **New features**: Implement entirely new functionality
5. **Conceptual development**: Extend the System Language framework

## Getting Started

1. **Set up your development environment**:
   - Follow the instructions in the main README.md
   - Install Rust, Tauri, and associated dependencies
   - Run the project with `cargo tauri dev`

2. **Understand the architecture**:
   - Review [Architecture Overview](../architecture/overview.md)
   - Explore the codebase structure in `src/`
   - Run the application and experiment with the existing features

3. **Choose a task**:
   - Look at open issues on GitHub
   - Review the roadmap document
   - Propose a new enhancement

## Development Guidelines

### Code Style

- Follow Rust conventions and use `rustfmt`
- Keep functions small and focused
- Document public interfaces with comments
- Use meaningful variable and function names

### Architecture Principles

1. **Separation of concerns**:
   - UI logic belongs in the Leptos components
   - Visualization logic belongs in Bevy systems
   - System model definitions belong in components

2. **Event-driven design**:
   - Use events for communication between systems
   - Avoid tight coupling between components

3. **Conceptual integrity**:
   - Changes should align with System Language theory
   - Maintain consistent terminology in code and docs

### Testing

- Write tests for new functionality
- Ensure existing tests pass
- Include both unit tests and integration tests

## Common Development Tasks

### Adding a New System Element Type

1. Define the component in `src/bevy_app/components/system_elements.rs`
2. Create a spawn bundle in `src/bevy_app/bundles/spawn/`
3. Add UI for creating and editing in Leptos components
4. Update serialization/deserialization in the data model
5. Add documentation for the new element

### Modifying Visual Representation

1. Locate the relevant spawn bundle
2. Modify the shape parameters
3. Update any associated systems for interaction

### Adding Properties to Elements

1. Update the component definition
2. Extend the UI to edit the properties
3. Update serialization to include the new properties

## Task Framework for Engineers

When assigning tasks to engineers, follow this template:

```markdown
# Task: [Title]

## Objective
Brief description of what needs to be accomplished

## Technical Requirements
1. Specific implementation details
2. Acceptance criteria
3. Performance considerations

## Files to Modify
- Path to file 1
- Path to file 2

## Related Concepts
Reference to system language concepts being implemented

## Dependencies
Any prerequisites or related tasks

## Testing Approach
How the changes should be tested
```

## Submitting Changes

1. Create a feature branch
2. Make your changes
3. Run tests and ensure they pass
4. Submit a pull request
5. Provide a clear description of your changes

## Communication

- Use clear, systems-oriented language
- Reference specific files and components
- Include diagrams when explaining complex changes
- Relate changes back to system language concepts