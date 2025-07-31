# Feature: Spatial Interaction UX

## Overview

**Feature Name**: Spatial Interaction UX  
**Branch**: feature/new-ui-ux  
**Status**: In Progress  
**Contributors**: Joel Beicher,joelbeicher,Joseph Ensminger,Maccesch,Marc-Stefan Cassola,matthias,rsthornton,Shingai Thornton  
**Date**: 2025-07-31

## Description

**Spatial Interaction UX** transforms BERT's system modeling interface from a single overwhelming property panel to intuitive spatial interaction regions that align with Mobus's systems theory. Users can now click on distinct spatial regions (system interior, boundary ring, environment area) to access contextually appropriate property panels, creating a natural mapping between visual representation and interaction model.

This feature addresses critical UX friction points identified through months of BERT usage and applies rigorous systems science theory from Mobus's Deep Systems Analysis framework to create theoretically grounded interaction patterns.

## Systems Science Context

**Theoretical Foundation**: Mobus states that "the biggest single mistake that system scientists make is to ignore or trivialize the concept of an interface (and its protocol)." This feature makes boundaries and interfaces first-class interactive entities rather than buried form fields.

**Boundary Theory Implementation**: 
- Boundaries have "porosity" and "perceptive fuzziness" properties
- "Interfaces embedded in boundary act as pass-ways for inputs and outputs"
- Systems exist in hierarchical spatial organization with clear inside/outside distinctions

**Spatial Cognition Alignment**: Matches interaction affordances with visual spatial relationships, supporting natural systems thinking workflows.

## Implemented Functionality

- **System Interior Clicking**: Click system center (filled circle) → System properties panel
- **Boundary Ring Clicking**: Click system edge (stroke ring) → Boundary properties panel  
- **Environment Region Clicking**: Click outside system → Environment properties panel
- **Context-Sensitive Panels**: Each panel shows only relevant properties for that spatial region
- **Progressive Disclosure**: Complex DSA concepts revealed contextually with Mobus quotes
- **Theoretical Help System**: Hover states and question marks provide theoretical context

## Technical Implementation

### Components Added

- `BoundaryRegion`: Marker component linking boundary entities to parent systems
- `EnvironmentRegion`: Marker component for environment interaction areas  
- `SpatialDetailPanelMode`: Resource enum managing panel selection state
- `BoundaryDetailsPanel`: Dedicated UI component for boundary properties
- `EnvironmentDetailsPanel`: Dedicated UI component for environment properties

### Components Modified

- `SystemBundle`: Enhanced with spatial interaction entity spawning
- `SystemDetails`: Refactored to focus only on system-specific properties
- `MouseInteractionPlugin`: Extended to handle spatial region selection
- `ElementDetails`: Updated with panel switching logic

### Architecture Decisions

**Layer 4 System Language**: Implements proper event-driven coordination between spatial selection and UI panel switching, maintaining separation between visualization (Bevy) and interface (Leptos).

**Entity Composition Pattern**: Each system spawns three entities (system, boundary, environment) with proper `PickTarget` relationships, following BERT's established ECS patterns.

**Progressive Disclosure Strategy**: Rather than overwhelming users with all DSA concepts simultaneously, reveals complexity contextually when clicking relevant spatial regions.

## Usage Examples

**User Workflow:**
1. **System Properties**: Click center of system circle → Access name, description, complexity, time unit, transformation functions
2. **Boundary Properties**: Click system edge/ring → Access boundary name, description, porosity, perceptive fuzziness, interface management  
3. **Environment Properties**: Click outside system → Access environment name, description, source/sink management

**Component Creation:**
```rust
// Spawn spatial interaction entities for a system
let system_entity = spawn_system_with_spatial_regions(
    &mut commands,
    position,
    radius,
    SystemBundle::new(...)
);

// Boundary ring entity with click handling
let boundary_entity = spawn_boundary_ring(
    &mut commands,
    system_entity,
    radius + 2.0, // Slightly larger click area
    position
);

// Environment region entity  
let environment_entity = spawn_environment_region(
    &mut commands,
    system_entity,
    radius * 3.0, // Large surrounding area
    position
);
```

## Testing Strategy

**Phase 1 - Clickable Regions**: 
- Verify boundary ring generates distinct click events
- Test environment region click detection vs system click priority
- Validate `PickTarget` relationships maintain proper parent system references

**Phase 2 - Panel Switching**:
- Confirm panel content changes based on spatial region clicked
- Test state persistence when switching between panels
- Verify form data isolation between different panel types

**Phase 3 - Systems Science Validation**:
- Expert review with systems science practitioners
- Theoretical consistency verification against Mobus DSA framework
- Usability testing with complex system models

## Future Improvements

- **Interface Visualization**: Make individual interfaces within boundary visible and clickable
- **Hierarchical Environment**: Support nested system environments with proper spatial interaction
- **Advanced Tooltips**: Rich contextual help with Mobus quotes and theoretical explanations
- **Accessibility**: Keyboard navigation support for spatial regions
- **Mobile Adaptation**: Touch-friendly interaction patterns for tablet usage

## Related Documentation

- [Mobus Deep Systems Analysis Framework](../../research/foundations/core-texts/the-process-of-deep-systems-analysis.md)
- [System Language Boundary Theory](../../research/foundations/core-texts/a-model-of-system.md)
- [BERT Mouse Interaction Architecture](../architecture/mouse-interaction-system.md)
- [Layer 4 System Language Implementation](../architecture/comprehensive-architecture-overview.md)
- [User Experience Research Synthesis](../research/spatial-interaction-ux-research.md)

---

_This documentation was automatically generated for the Spatial Interaction UX feature on 2025-07-31._
