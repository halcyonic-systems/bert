//! # Spatial Region Zoom System
//!
//! Applies zoom transformations to spatial interaction regions.
//! Critical for maintaining spatial interaction accuracy during zoom operations.

use bevy::prelude::*;
use crate::bevy_app::components::{BoundaryRegion, EnvironmentRegion, System};
use crate::bevy_app::resources::Zoom;

/// System that synchronizes spatial interaction region positions with their parent systems.
///
/// Runs continuously to ensure that boundary rings and environment regions stay properly
/// aligned with their parent systems, especially during zoom operations and system movement.
///
/// # Behavior
///
/// - Queries all BoundaryRegion entities and updates their Transform to match parent system
/// - Queries all EnvironmentRegion entities and updates their Transform to match parent system  
/// - Only updates when parent system Transform has changed (efficient)
/// - Maintains original radius/scale while syncing position
///
/// # Performance
///
/// This system runs every frame but only performs updates when parent Transform components
/// have been marked as Changed<Transform>, making it efficient for typical usage.
pub fn sync_spatial_regions_with_systems(
    mut boundary_query: Query<(&BoundaryRegion, &mut Transform), Without<System>>,
    mut environment_query: Query<(&EnvironmentRegion, &mut Transform), (Without<System>, Without<BoundaryRegion>)>,
    system_query: Query<&Transform, (With<System>, Changed<Transform>)>,
) {
    // Note: This is the simpler version that syncs all regions when any system changes
    // The more efficient version below uses Changed<Transform> detection
    
    // Update boundary regions when their parent system has moved
    for (boundary_region, mut boundary_transform) in boundary_query.iter_mut() {
        // Skip processing if parent system entity is invalid (during model loading cleanup)
        if let Ok(system_transform) = system_query.get(boundary_region.system_entity) {
            // Defensive coordinate validation to prevent Lyon path panics
            if system_transform.translation.x.is_finite() && system_transform.translation.y.is_finite() {
                // Sync position but preserve the boundary region's Z-order and scale
                let old_z = boundary_transform.translation.z;
                let old_scale = boundary_transform.scale;
                
                boundary_transform.translation = Vec3::new(
                    system_transform.translation.x,
                    system_transform.translation.y,
                    old_z, // Preserve Z-order for proper click layering
                );
                boundary_transform.scale = old_scale; // Preserve original scale
                
                trace!("Synced boundary region to system {:?} at {:?}", 
                       boundary_region.system_entity, boundary_transform.translation);
            } else {
                warn!("Invalid coordinates detected for system {:?}: ({}, {}), skipping boundary sync",
                      boundary_region.system_entity, 
                      system_transform.translation.x, 
                      system_transform.translation.y);
            }
        } else {
            // Parent system entity is invalid - likely during model loading cleanup
            trace!("Skipping boundary sync for system {:?} - entity invalid (probably during cleanup)",
                   boundary_region.system_entity);
        }
    }
    
    // Update environment regions when their parent system has moved  
    for (environment_region, mut environment_transform) in environment_query.iter_mut() {
        // Skip processing if parent system entity is invalid (during model loading cleanup)
        if let Ok(system_transform) = system_query.get(environment_region.system_entity) {
            // Defensive coordinate validation to prevent Lyon path panics
            if system_transform.translation.x.is_finite() && system_transform.translation.y.is_finite() {
                // Sync position but preserve the environment region's Z-order and scale
                let old_z = environment_transform.translation.z;
                let old_scale = environment_transform.scale;
                
                environment_transform.translation = Vec3::new(
                    system_transform.translation.x,
                    system_transform.translation.y,
                    old_z, // Preserve Z-order for proper click layering
                );
                environment_transform.scale = old_scale; // Preserve original scale
                
                trace!("Synced environment region to system {:?} at {:?}", 
                       environment_region.system_entity, environment_transform.translation);
            } else {
                warn!("Invalid coordinates detected for system {:?}: ({}, {}), skipping environment sync",
                      environment_region.system_entity, 
                      system_transform.translation.x, 
                      system_transform.translation.y);
            }
        } else {
            // Parent system entity is invalid - likely during model loading cleanup
            trace!("Skipping environment sync for system {:?} - entity invalid (probably during cleanup)",
                   environment_region.system_entity);
        }
    }
}

/// Applies zoom scaling to spatial interaction region transforms during zoom operations.
///
/// This system ensures that boundary rings and environment regions maintain their
/// relative positions and sizes when the zoom level changes, keeping them properly
/// aligned with their parent systems.
pub fn apply_zoom_to_spatial_regions(
    mut boundary_query: Query<(&BoundaryRegion, &mut Transform), (With<BoundaryRegion>, Without<System>)>,
    mut environment_query: Query<(&EnvironmentRegion, &mut Transform), (With<EnvironmentRegion>, Without<System>, Without<BoundaryRegion>)>,
    system_query: Query<&Transform, (With<System>, Without<BoundaryRegion>, Without<EnvironmentRegion>)>,
    zoom: Res<Zoom>,
) {
    let zoom_scale = **zoom;
    
    // Update boundary regions to match their parent system positions and zoom
    for (boundary_region, mut boundary_transform) in boundary_query.iter_mut() {
        // Skip processing if parent system entity is invalid (during model loading cleanup)
        if let Ok(system_transform) = system_query.get(boundary_region.system_entity) {
            // Defensive coordinate validation for zoom operations
            if system_transform.translation.x.is_finite() && system_transform.translation.y.is_finite() && zoom_scale.is_finite() {
                // Apply zoom scaling to position while maintaining original z-order
                let original_z = boundary_transform.translation.z;
                boundary_transform.translation = Vec3::new(
                    system_transform.translation.x,
                    system_transform.translation.y, 
                    original_z,
                );
                
                // Apply zoom scale to the transform
                boundary_transform.scale = Vec3::splat(zoom_scale);
                
                trace!("Applied zoom {:.2} to boundary region of system {:?}", 
                       zoom_scale, boundary_region.system_entity);
            } else {
                warn!("Invalid coordinates/zoom detected for boundary system {:?}: ({}, {}, zoom: {}), skipping zoom sync",
                      boundary_region.system_entity, 
                      system_transform.translation.x, 
                      system_transform.translation.y,
                      zoom_scale);
            }
        } else {
            // Parent system entity is invalid - likely during model loading cleanup
            trace!("Skipping boundary zoom sync for system {:?} - entity invalid (probably during cleanup)",
                   boundary_region.system_entity);
        }
    }
    
    // Update environment regions to match their parent system positions and zoom  
    for (environment_region, mut environment_transform) in environment_query.iter_mut() {
        // Skip processing if parent system entity is invalid (during model loading cleanup)
        if let Ok(system_transform) = system_query.get(environment_region.system_entity) {
            // Defensive coordinate validation for zoom operations
            if system_transform.translation.x.is_finite() && system_transform.translation.y.is_finite() && zoom_scale.is_finite() {
                // Apply zoom scaling to position while maintaining original z-order
                let original_z = environment_transform.translation.z;
                environment_transform.translation = Vec3::new(
                    system_transform.translation.x,
                    system_transform.translation.y,
                    original_z,
                );
                
                // Apply zoom scale to the transform
                environment_transform.scale = Vec3::splat(zoom_scale);
                
                trace!("Applied zoom {:.2} to environment region of system {:?}", 
                       zoom_scale, environment_region.system_entity);
            } else {
                warn!("Invalid coordinates/zoom detected for environment system {:?}: ({}, {}, zoom: {}), skipping zoom sync",
                      environment_region.system_entity, 
                      system_transform.translation.x, 
                      system_transform.translation.y,
                      zoom_scale);
            }
        } else {
            // Parent system entity is invalid - likely during model loading cleanup
            trace!("Skipping environment zoom sync for system {:?} - entity invalid (probably during cleanup)",
                   environment_region.system_entity);
        }
    }
}