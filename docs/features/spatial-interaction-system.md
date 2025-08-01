# Spatial Interaction System Implementation Guide
*Complete technical implementation analysis and lessons learned*

## Overview

The Spatial Interaction System enables users to "click WHERE to edit WHAT" - clicking different spatial regions of a system (interior, boundary, environment) shows contextually appropriate property panels.

**Status**: ✅ **Fully Functional** (August 1, 2025)  
**Architecture**: Bevy ECS → Leptos Reactive UI → CSS Conditional Rendering

## Core Architecture

### 1. Spatial Click Detection (Bevy ECS)
```rust
// Z-order layered clickable regions
Environment: z = -0.2  // Background layer
Boundary:    z = -0.1  // Middle layer  
System:      z = 0.0   // Foreground layer
```

### 2. Resource Bridging (Bevy ↔ Leptos)
```rust
// In leptos_app/mod.rs
let (spatial_mode, spatial_mode_duplex) = 
    signal_synced(SpatialDetailPanelMode::default());

// In bevy_app/mod.rs  
.sync_leptos_signal_with_resource(spatial_mode_duplex)
```

### 3. Reactive Signal Management (Leptos)
```rust
// Use Memo::new() for persistent signals
let system_name = Memo::new(move |_| system_query
    .read()
    .as_ref()
    .map(|(name, _, _, _)| name.to_string())
    .unwrap_or_default());
```

### 4. Conditional Rendering (CSS-based)
```rust
// CSS visibility instead of Show components
<div class:hidden=move || !matches!(spatial_mode.get(), SpatialDetailPanelMode::System)>
    <InputGroup value=system_name />
</div>
```

## Critical Implementation Lessons

### ❌ Avoid: Show Components for Persistent State
```rust
// DON'T DO THIS - causes signal disposal
<Show when=move || condition>
    <InputGroup value=Signal::derive(move || /* ... */) />
</Show>
```

### ✅ Use: CSS Visibility + Memoization  
```rust
// DO THIS - signals persist across visibility changes
let persistent_signal = Memo::new(move |_| /* ... */);
<div class:hidden=move || !condition>
    <InputGroup value=persistent_signal />
</div>
```

## Key Files

- **Spatial Regions**: `src/bevy_app/bundles/spawn/spatial_interaction.rs`
- **Click Handling**: `src/bevy_app/plugins/mouse_interaction/mod.rs`
- **UI Integration**: `src/leptos_app/details.rs` (SystemDetails component)
- **Resource Bridge**: `src/leptos_app/mod.rs` + `src/bevy_app/mod.rs`

## Implementation Checklist

### Phase 1: Spatial Detection
- [ ] Create spatial region components (BoundaryRegion, EnvironmentRegion)
- [ ] Implement z-order layered spawning
- [ ] Add click detection system with debug logging
- [ ] Test spatial region detection in console

### Phase 2: Resource Bridging
- [ ] Define spatial mode resource/enum
- [ ] Set up signal_synced bridge between Bevy and Leptos
- [ ] Create click handler that updates spatial mode
- [ ] Verify mode changes sync to UI

### Phase 3: Conditional Rendering
- [ ] Create persistent signals using Memo::new()
- [ ] Replace Show components with CSS class:hidden
- [ ] Test mode switching without crashes
- [ ] Validate all property fields work correctly

## Performance Considerations

- **DOM Elements**: Stay mounted, only CSS visibility changes
- **Signal Lifecycle**: Memo::new() prevents disposal during conditional rendering
- **Z-order Rendering**: Minimal performance impact, proper layering essential
- **Event Handling**: Direct Bevy event processing, no intermediate layers

## Future Extensions

This architecture enables:
- **Complexity Metrics**: Click system parts → show different complexity calculations
- **Flow Properties**: Click flows → edit flow-specific properties  
- **Nested Systems**: Click subsystems → hierarchical property editing
- **Any Spatial Context**: Template for any "click region → show properties" features

## Troubleshooting

### "Reactive value has already been disposed"
- **Cause**: Using Signal::derive inside Show components
- **Fix**: Use Memo::new() + CSS class:hidden approach

### Spatial Regions Not Detecting Clicks
- **Cause**: Z-order conflicts or incorrect layering
- **Fix**: Verify z-order: Environment (-0.2) < Boundary (-0.1) < System (0.0)

### UI Not Updating on Mode Switch
- **Cause**: Resource bridge not properly configured
- **Fix**: Verify signal_synced setup in both Bevy and Leptos initialization

---

*Last Updated: August 1, 2025*  
*Implementation Status: ✅ Complete and Functional*