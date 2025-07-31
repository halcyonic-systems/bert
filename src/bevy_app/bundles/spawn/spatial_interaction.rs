//! # Spatial Interaction Spawn Functions
//!
//! This module provides entity spawning functions for the spatial interaction UX system,
//! enabling boundary rings and environment regions to be clicked separately from system interiors.
//!
//! ## Architecture
//!
//! The spatial interaction system creates three distinct clickable regions for each system:
//!
//! - **System Interior**: The filled circle representing the system core
//! - **Boundary Ring**: The stroke edge representing the system boundary  
//! - **Environment Region**: The surrounding area representing the system environment
//!
//! ## Systems Science Foundation
//!
//! This implementation follows Mobus's Deep Systems Analysis framework where boundaries
//! are first-class entities with distinct properties, not just visual decoration.
//! The spatial interaction model aligns with the theoretical distinction between
//! system interior, boundary interfaces, and environmental context.
//!
//! ## Usage Patterns
//!
//! These spawn functions are called by system creation workflows to establish
//! the complete spatial interaction architecture alongside traditional system entities.
//!
//! ## Integration Points
//!
//! - Integrates with existing `PickSelection` mouse interaction system
//! - Uses `PickTarget` components to redirect clicks to parent system entities
//! - Coordinates with `SpatialDetailPanelMode` resource for UI panel switching

use crate::bevy_app::components::{BoundaryRegion, EnvironmentRegion};
use crate::bevy_app::plugins::mouse_interaction::{PickSelection, PickTarget};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

/// Spawns a clickable boundary ring entity for spatial interaction with system boundaries.
///
/// Creates an invisible stroke ring around a system that can be clicked to display
/// boundary-specific properties, implementing the spatial interaction UX where
/// boundaries are first-class interactive entities.
///
/// # Systems Science Context
///
/// Following Mobus's systems theory, boundaries are not mere visual decoration but
/// fundamental system elements with properties like porosity and perceptive fuzziness.
/// This function makes boundaries clickable to support natural systems thinking workflows.
///
/// # Parameters
///
/// * `commands` - Bevy command buffer for entity spawning
/// * `system_entity` - The parent system entity that owns this boundary
/// * `radius` - The boundary ring radius (typically system radius + click margin)
/// * `position` - World position for the boundary ring
/// * `z_order` - Z-axis position for proper click layering (should be above system but below UI)
///
/// # Returns
///
/// The `Entity` ID of the spawned boundary ring that can be clicked to show boundary properties.
///
/// # Examples
///
/// ```rust
/// use bert::spawn_boundary_ring;
/// use bevy::prelude::*;
///
/// fn create_system_with_spatial_interaction(
///     mut commands: Commands,
///     system_entity: Entity,
///     radius: f32,
///     position: Vec2,
/// ) {
///     // Spawn boundary ring with slight margin for easier clicking
///     let boundary_entity = spawn_boundary_ring(
///         &mut commands,
///         system_entity,
///         radius + 3.0, // 3px margin for click area
///         position,
///         0.1, // Slightly above system
///     );
/// }
/// ```
///
/// # Click Behavior
///
/// When clicked, the boundary ring uses `PickTarget` to redirect selection to the
/// parent system entity, then updates `SpatialDetailPanelMode` to show boundary properties.
///
/// # Visual Design
///
/// The boundary ring is rendered as a transparent stroke to be invisible but clickable.
/// Visual feedback is provided through hover states and panel switching rather than
/// direct visual changes to the boundary ring itself.
///
/// # Performance
///
/// Uses efficient stroke geometry with minimal vertex count for click detection.
/// The invisible stroke approach avoids rendering overhead while maintaining
/// precise click area definition.
///
/// # See Also
///
/// - [`spawn_environment_region`]: Creates environment spatial interaction
/// - [`BoundaryRegion`]: The component attached to identify boundary entities
/// - [`PickTarget`]: Redirects clicks to the parent system entity
pub fn spawn_boundary_ring(
    commands: &mut Commands,
    system_entity: Entity,
    radius: f32,
    position: Vec2,
    z_order: f32,
) -> Entity {
    let ring_path = GeometryBuilder::build_as(&shapes::Circle {
        radius,
        ..default()
    });

    let boundary_entity = commands
        .spawn((
            Name::new("Boundary Ring"),
            ShapeBundle {
                path: ring_path,
                transform: Transform::from_translation(position.extend(z_order)),
                ..default()
            },
            Stroke::new(Color::srgba(0.0, 0.0, 0.0, 0.0), 4.0), // Invisible but clickable stroke
            PickSelection::default(),
            PickTarget {
                target: system_entity,
            },
            BoundaryRegion { system_entity },
        ))
        .id();

    info!("Spawned boundary ring entity {:?} for system {:?} at {:?}", 
          boundary_entity, system_entity, position);
    
    boundary_entity
}

/// Spawns a clickable environment region entity for spatial interaction with system environments.
///
/// Creates a large invisible circle around a system that can be clicked to display
/// environment-specific properties, implementing spatial interaction where the area
/// outside systems represents the environmental context.
///
/// # Systems Science Context
///
/// The environment represents the external sources, sinks, and contextual factors
/// that interact with a system. Making this spatially clickable supports systems
/// thinking workflows where environment is a distinct conceptual space.
///
/// # Parameters
///
/// * `commands` - Bevy command buffer for entity spawning
/// * `system_entity` - The parent system entity whose environment this represents
/// * `environment_radius` - The radius of the environment region (typically 3x system radius)
/// * `position` - World position for the environment region (same as system center)
/// * `z_order` - Z-axis position for proper click layering (should be behind system and boundary)
///
/// # Returns
///
/// The `Entity` ID of the spawned environment region that can be clicked to show environment properties.
///
/// # Examples
///
/// ```rust
/// use bert::spawn_environment_region;
/// use bevy::prelude::*;
///
/// fn create_system_with_spatial_interaction(
///     mut commands: Commands,
///     system_entity: Entity,
///     system_radius: f32,
///     position: Vec2,
/// ) {
///     // Spawn environment region with large capture area
///     let environment_entity = spawn_environment_region(
///         &mut commands,
///         system_entity,
///         system_radius * 3.0, // Large area around system
///         position,
///         -0.1, // Behind system for proper click priority
///     );
/// }
/// ```
///
/// # Click Priority
///
/// Environment regions are rendered behind systems and boundaries (negative z-order)
/// to ensure proper click priority: boundary > system > environment.
/// Users clicking near the system will select system/boundary, while clicks
/// in empty space around the system will select the environment.
///
/// # Visual Design
///
/// The environment region is completely invisible (transparent fill) but provides
/// a large clickable area around systems. Visual feedback comes through panel
/// switching rather than direct visual representation.
///
/// # Performance
///
/// Uses simple circle geometry with transparent fill for minimal rendering cost
/// while providing large, easy-to-target click areas for environment interaction.
///
/// # Interaction Behavior
///
/// When clicked, uses `PickTarget` to redirect to the parent system entity,
/// then updates `SpatialDetailPanelMode` to show environment properties rather
/// than system properties.
///
/// # See Also
///
/// - [`spawn_boundary_ring`]: Creates boundary spatial interaction
/// - [`EnvironmentRegion`]: The component attached to identify environment entities  
/// - [`PickTarget`]: Redirects clicks to the parent system entity
pub fn spawn_environment_region(
    commands: &mut Commands,
    system_entity: Entity,
    environment_radius: f32,
    position: Vec2,
    z_order: f32,
) -> Entity {
    let environment_path = GeometryBuilder::build_as(&shapes::Circle {
        radius: environment_radius,
        ..default()
    });

    let environment_entity = commands
        .spawn((
            Name::new("Environment Region"),
            ShapeBundle {
                path: environment_path,
                transform: Transform::from_translation(position.extend(z_order)),
                ..default()
            },
            Fill::color(Color::srgba(0.0, 0.0, 0.0, 0.0)), // Invisible but clickable fill
            PickSelection::default(),
            PickTarget {
                target: system_entity,
            },
            EnvironmentRegion { system_entity },
        ))
        .id();

    info!("Spawned environment region entity {:?} for system {:?} at {:?}", 
          environment_entity, system_entity, position);
    
    environment_entity
}

/// Spawns a complete system with spatial interaction regions for boundary and environment.
///
/// This convenience function creates a system entity along with its associated boundary ring
/// and environment region entities, establishing the complete spatial interaction architecture
/// in a single function call.
///
/// # Systems Science Integration
///
/// Implements the complete spatial interaction model where systems, boundaries, and
/// environments are distinct clickable regions, aligning with Mobus's systems theory
/// and supporting natural systems thinking workflows.
///
/// # Parameters
///
/// * `commands` - Bevy command buffer for entity spawning
/// * `system_entity` - The main system entity (must be pre-spawned with SystemBundle)
/// * `system_radius` - The radius of the main system
/// * `position` - World position for all spatial interaction entities
///
/// # Returns
///
/// A tuple containing `(boundary_entity, environment_entity)` for the spawned spatial regions.
/// The system entity is not returned as it's provided as input.
///
/// # Examples
///
/// ```rust
/// use bert::{SystemBundle, spawn_system_with_spatial_regions};
/// use bevy::prelude::*;
///
/// fn create_complete_system(
///     mut commands: Commands,
///     // ... other parameters
/// ) {
///     let system_radius = 50.0;
///     let position = Vec2::new(100.0, 100.0);
///     
///     // First spawn the main system
///     let system_entity = commands.spawn(SystemBundle::new(
///         position, 0.0, system_radius, // ... other params
///     )).id();
///     
///     // Then add spatial interaction regions
///     let (boundary_entity, environment_entity) = spawn_system_with_spatial_regions(
///         &mut commands,
///         system_entity,
///         system_radius,
///         position,
///     );
/// }
/// ```
///
/// # Z-Order Management
///
/// Automatically manages z-ordering to ensure proper click priority:
/// - Environment: -0.1 (behind everything)
/// - System: 0.0 (middle layer)  
/// - Boundary: 0.1 (front layer)
/// - UI: 1.0+ (above all spatial elements)
///
/// # Click Area Sizing
///
/// Uses intelligent sizing for optimal user experience:
/// - Boundary ring: system radius + 3px margin for easier clicking
/// - Environment region: system radius * 3.0 for generous click area
///
/// # See Also
///
/// - [`spawn_boundary_ring`]: Individual boundary region spawning
/// - [`spawn_environment_region`]: Individual environment region spawning
/// - [`SystemBundle`]: The main system entity bundle
pub fn spawn_system_with_spatial_regions(
    commands: &mut Commands,
    system_entity: Entity,
    system_radius: f32,
    position: Vec2,
) -> (Entity, Entity) {
    // Spawn boundary ring with margin for easier clicking
    let boundary_entity = spawn_boundary_ring(
        commands,
        system_entity,
        system_radius + 3.0, // 3px margin for click area
        position,
        0.1, // Above system for click priority
    );

    // Spawn environment region with large capture area
    let environment_entity = spawn_environment_region(
        commands,
        system_entity,
        system_radius * 3.0, // Large surrounding area
        position,
        -0.1, // Behind system for click priority
    );

    (boundary_entity, environment_entity)
}
