# Signal Pattern Decision - Signal::derive vs Memo::new

## Decision Made: 2025-08-01

**Chosen Pattern**: `Memo::new()` for reactive signal creation  
**Previous Pattern**: `Signal::derive()` (original implementation)

## Context

During BERT v0.2.0 development, we encountered lifecycle bugs where signals were being disposed when using `Show` components, causing blank screen issues. The original engineer likely chose `Signal::derive` for good technical reasons, but we needed a working solution.

## The Problem

```rust
// Original pattern - caused disposal issues with Show components
let system_name = Signal::derive(move || {
    system_query.read().as_ref()
        .map(|(name, _, _, _)| name.to_string())
        .unwrap_or_default()
});
```

When used with Leptos `Show` components, signals created with `Signal::derive` were being disposed when components unmounted, leading to "reactive value has already been disposed" errors.

## The Solution

```rust
// New pattern - prevents disposal issues
let system_name = Memo::new(move |_| system_query
    .read()
    .as_ref()
    .map(|(name, _, _, _)| name.to_string())
    .unwrap_or_default());
```

Combined with CSS `class:hidden` instead of `Show` components for conditional rendering.

## Implementation

- **SystemDetails**: Updated to `Memo::new` during spatial interaction development
- **SubSystemDetails**: Updated to `Memo::new` during v0.2.0 polish for consistency
- **Future Components**: Should use `Memo::new` pattern for consistency

## Trade-offs

**Pros**:
- ✅ Eliminates signal disposal bugs
- ✅ Works reliably with conditional rendering
- ✅ Consistent pattern across codebase

**Cons**:
- ❓ May not be the "ideal" Leptos pattern (original engineer's choice unclear)
- ❓ Slightly more verbose syntax

## Future Considerations

If signal disposal issues are resolved in future Leptos versions, or if we discover the original technical reasoning for `Signal::derive`, this decision can be revisited. For now, `Memo::new` provides a stable, working solution.

## Affected Components

- `SystemDetails` - Updated during spatial interaction development
- `SubSystemDetails` - Updated during v0.2.0 consistency work
- Future reactive components should follow this pattern