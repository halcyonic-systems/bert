# Rust Documentation Guidelines

Standards for documenting Rust code in BERT. Following these ensures consistency and helps new contributors understand the codebase.

## Quick Reference

| Item Type | Template |
|-----------|----------|
| Modules | Purpose, Architecture, Key Components |
| Functions | Purpose, Parameters, Returns, Errors, Panics |
| Types | Purpose, Invariants, Fields |
| Examples | Basic usage, Error handling |

## General Principles

1. **Document as you code** - Write docs while implementing, not after
2. **Audience awareness** - Write for new contributors
3. **Completeness** - Every public item needs documentation
4. **Conciseness** - Thorough but not verbose

## Documentation Format

```rust
//! Module-level documentation (top of file)

/// Item documentation (structs, functions, etc.)
pub struct Example {}
```

## Module Template

```rust
//! # Module Name
//!
//! Brief description of what this module does.
//!
//! ## Architecture
//!
//! How this module fits into the larger system.
//!
//! ## Key Components
//!
//! - [`MainType`]: Primary type in this module
//! - [`helper_function`]: Common utility
```

## Function Template

```rust
/// Brief description of what the function does.
///
/// # Parameters
///
/// - `param1`: What this parameter means
/// - `param2`: What this parameter means
///
/// # Returns
///
/// What is returned and what it represents.
///
/// # Errors
///
/// - `ErrorType::Variant` - When this error occurs
///
/// # Panics
///
/// Conditions that cause panics (if any).
///
/// # Examples
///
/// ```rust
/// let result = my_function(arg1, arg2)?;
/// ```
pub fn my_function(param1: Type1, param2: Type2) -> Result<Output, Error> {
    // ...
}
```

## Type Template

```rust
/// Brief description of what this type represents.
///
/// # Invariants
///
/// - Constraint that must always hold
/// - Another constraint
///
/// # Thread Safety
///
/// Thread-safety properties (if relevant).
pub struct MyType {
    /// Description of this field
    pub field1: Type,

    /// Description of this field
    pub field2: Type,
}
```

## Examples

Include examples for complex or commonly used items:

```rust
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use bert::SystemElement;
///
/// let element = SystemElement::new("Input Source");
/// element.set_position(100.0, 200.0);
/// ```
```

## Documentation Tests

Examples in docs are compiled and tested. Hide test-only code with `# `:

```rust
/// ```rust
/// # use bert::error::Result;
/// # fn main() -> Result<()> {
/// let model = SystemModel::new("My System");
/// # Ok(())
/// # }
/// ```
```

## Cross-References

Link to related items:

```rust
/// See [`Connection`] for details on element connections.
/// Uses [`bevy::prelude::Entity`] as the underlying ID.
```

## Leptos-Specific: Signal Pattern

Use `Memo::new()` for reactive computations (not `Signal::derive()`):

```rust
// Correct - avoids disposal issues with Show components
let system_name = Memo::new(move |_| {
    system_query.read()
        .as_ref()
        .map(|(name, _, _, _)| name.to_string())
        .unwrap_or_default()
});
```

See [ADR-001: Signal Pattern Decision](../gitbook/for-developers/architecture/decisions/adr-001-signal-pattern.md) for context.

## Tooling

```bash
# Preview documentation
cargo doc --open

# Check for missing docs
cargo rustdoc -- -D missing-docs
```

## Review Checklist

- [ ] All public items documented
- [ ] Descriptions are accurate
- [ ] Parameters/Returns/Errors covered
- [ ] Examples compile
- [ ] Links are valid
