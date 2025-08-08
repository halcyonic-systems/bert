# Rust Documentation Guidelines

This document outlines the standards and best practices for documenting Rust code in the BERT project. Following these guidelines ensures consistency, maintainability, and helps new contributors understand our codebase more easily.

## Table of Contents

- [General Principles](#general-principles)
- [Documentation Format](#documentation-format)
- [Module-Level Documentation](#module-level-documentation)
- [Type Documentation](#type-documentation)
- [Function Documentation](#function-documentation)
- [Examples](#examples)
- [Documentation Tests](#documentation-tests)
- [Cross-References](#cross-references)
- [Documentation Style](#documentation-style)
- [Tools and Workflows](#tools-and-workflows)

## General Principles

1. **Document as you code**: Write documentation while implementing features, not after.
2. **Audience awareness**: Write documentation with new contributors in mind.
3. **Completeness**: Every public item (module, struct, enum, trait, function, method) must have documentation.
4. **Conciseness**: Be thorough but avoid unnecessary verbosity.
5. **Consistency**: Follow the same patterns throughout the codebase.

## Documentation Format

- Use Rust's native documentation comments:
  - `///` for documenting items (structs, enums, functions, etc.)
  - `//!` for documenting modules (at the top of files or in `mod.rs`)
  - Use Markdown formatting within documentation comments

```rust
//! This module handles system element geometry calculations.

/// Represents a position in 2D space.
pub struct Position {
    pub x: f32,
    pub y: f32,
}
```

## Module-Level Documentation

Every module (`mod.rs` file or top of individual files) should include:

1. **Purpose**: A concise description of the module's responsibility
2. **Architecture**: How this module fits into the larger system
3. **Key Components**: Brief overview of important items defined in the module
4. **Usage Patterns**: Common ways the module is used

Example:

```rust
//! # System Elements Module
//!
//! This module defines the core system elements that make up a BERT model.
//!
//! ## Architecture
//!
//! System elements are the building blocks that represent different parts of a system:
//! - External entities
//! - Interfaces
//! - Flows
//! - Subsystems
//!
//! ## Key Components
//!
//! - [`SystemElement`]: The base trait for all system elements
//! - [`Interface`]: Represents system boundaries
//! - [`Flow`]: Represents movement between system elements
```

## Type Documentation

For structs, enums, and traits:

1. **Purpose**: What the type represents and its role
2. **Invariants**: Any assumptions or constraints that must be maintained
3. **Threading**: Thread-safety properties if relevant
4. **Lifecycle**: Creation, usage, and destruction patterns
5. **Fields**: Document non-obvious fields

Example:

```rust
/// Represents a system element that can be placed in the diagram.
///
/// System elements are the fundamental building blocks of BERT models and can be
/// connected to each other via flows. Each element has a unique ID and position.
///
/// # Invariants
///
/// - The ID must be unique across all system elements
/// - Position coordinates must be finite (not NaN or infinite)
///
/// # Thread Safety
///
/// This type is not thread-safe and should only be accessed from the main thread.
pub struct SystemElement {
    /// Unique identifier for this element
    pub id: Id,
    
    /// Position in 2D space
    pub position: Position,
    
    // ...other fields...
}
```

## Function Documentation

For functions and methods:

1. **Purpose**: What the function does
2. **Parameters**: Explain each parameter's meaning and requirements
3. **Return Value**: What is returned and what it represents
4. **Errors**: All possible error conditions
5. **Panics**: Conditions that cause panics, if any
6. **Side Effects**: Any state changes or external interactions

Example:

```rust
/// Creates a connection between two system elements.
///
/// # Parameters
///
/// - `source_id`: The ID of the source element
/// - `target_id`: The ID of the target element
/// - `flow_type`: The type of flow between elements
///
/// # Returns
///
/// A new [`Connection`] instance representing the established connection.
///
/// # Errors
///
/// Returns `ConnectionError::ElementNotFound` if either element doesn't exist.
/// Returns `ConnectionError::InvalidConnection` if the elements cannot be connected.
///
/// # Panics
///
/// Panics if the connection system is not initialized.
pub fn connect_elements(
    source_id: Id,
    target_id: Id,
    flow_type: FlowType,
) -> Result<Connection, ConnectionError> {
    // Implementation...
}
```

## Examples

Include examples for complex or commonly used items:

1. **Basic Usage**: Show the most common way to use the item
2. **Advanced Usage**: Demonstrate more complex scenarios
3. **Error Handling**: Show how to handle common errors

Example:

```rust
/// # Examples
///
/// Basic usage:
///
/// ```
/// use bert::SystemElement;
///
/// let element = SystemElement::new("Input Source");
/// element.set_position(100.0, 200.0);
/// ```
///
/// Creating with custom attributes:
///
/// ```
/// use bert::{SystemElement, ElementType};
///
/// let element = SystemElement::builder()
///     .name("Processing Unit")
///     .element_type(ElementType::Subsystem)
///     .position(150.0, 300.0)
///     .build();
/// ```
```

## Documentation Tests

Use documentation examples as tests where appropriate:

1. **Verify Examples**: Ensure all examples in documentation compile and run
2. **Test Edge Cases**: Include examples that cover edge cases
3. **Hide Test-Only Code**: Use `# ` for test-only lines (not shown in docs)

Example:

```rust
/// # Examples
///
/// ```
/// # use bert::error::Result;
/// # fn main() -> Result<()> {
/// use bert::{SystemModel, SystemElement};
///
/// let mut model = SystemModel::new("My System");
/// let element_id = model.add_element(SystemElement::new("Input"))?;
/// assert!(model.contains_element(element_id));
/// # Ok(())
/// # }
/// ```
```

## Cross-References

Link related items using Rust's cross-referencing syntax:

1. **Same Crate**: Use `[`ItemName`]`
2. **External Crate**: Use `[`ItemName`](crate_name::ItemName)`
3. **Standard Library**: Use `[`ItemName`](std::ItemName)`

Example:

```rust
/// Creates a new flow between systems.
///
/// See [`Connection`] for more details on how elements are connected.
/// This uses [`bevy::prelude::Entity`] as the underlying identifier.
pub fn create_flow() {
    // Implementation...
}
```

## Documentation Style

1. **Clear Language**: Use simple, direct language
2. **Present Tense**: Describe what functions do, not what they will do
3. **Active Voice**: Use active rather than passive voice
4. **Consistency**: Use consistent terminology throughout
5. **Formatting**:
   - Use backticks for code, types, and variables
   - Use italics for emphasis
   - Use lists for multiple related items

## Tools and Workflows

1. **Documentation Generation**:
   - Run `cargo doc --open` to preview documentation
   - Check documentation coverage with `cargo rustdoc -- -D missing-docs`

2. **Pre-Commit Checks**:
   - Documentation is required for all public items
   - Examples must compile successfully
   - Links must be valid

3. **CI Integration**:
   - Automated checks for documentation presence and correctness
   - Fail builds if documentation standards are not met

## Documentation Review

When reviewing code, ensure documentation:

1. **Exists**: For all public items
2. **Accurate**: Correctly describes behavior
3. **Complete**: Covers all aspects of the item
4. **Consistent**: Follows these guidelines
5. **Helpful**: Actually helps readers understand the code

---

By following these guidelines, we ensure our codebase remains accessible, maintainable, and well-documented. Good documentation is as important as good code!