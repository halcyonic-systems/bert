//! # Spatial Interaction Components
//!
//! Components for the spatial interaction UX system, enabling boundary rings 
//! and environment regions to be clicked separately from system interiors.

use bevy::prelude::*;

/// Marker component for boundary ring entities that can be clicked to show boundary properties.
///
/// Attached to invisible stroke rings around systems that redirect clicks to the parent
/// system entity but trigger boundary-specific UI panels.
#[derive(Component, Debug, Clone)]
pub struct BoundaryRegion {
    /// The parent system entity that owns this boundary
    pub system_entity: Entity,
}

/// Marker component for environment region entities that can be clicked to show environment properties.
///
/// Attached to large invisible circles around systems that redirect clicks to the parent
/// system entity but trigger environment-specific UI panels.
#[derive(Component, Debug, Clone)]
pub struct EnvironmentRegion {
    /// The parent system entity whose environment this represents
    pub system_entity: Entity,
}

/// Resource enum for managing which spatial detail panel is currently displayed.
///
/// Updated by mouse interaction events when users click on different spatial regions
/// (system interior, boundary ring, environment area) to switch between contextually
/// appropriate property panels.
#[derive(Resource, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Resource)]
pub enum SpatialDetailPanelMode {
    /// Show system-specific properties (name, description, complexity, etc.)
    #[default]
    System,
    /// Show boundary-specific properties (name, description, porosity, perceptive fuzziness)
    Boundary,
    /// Show environment-specific properties (name, description, sources/sinks)
    Environment,
}