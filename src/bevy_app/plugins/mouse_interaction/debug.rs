use super::PickSelection;
use crate::bevy_app::components::{BoundaryRegion, EnvironmentRegion};
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;

#[derive(Resource, Clone, Deref, DerefMut, PartialEq, Eq, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct SelectedEntities(Vec<Entity>);

pub fn debug_selection(
    selection_query: Query<(Entity, &PickSelection), Changed<PickSelection>>,
    mut selected_entities: ResMut<SelectedEntities>,
) {
    for (entity, selection) in &selection_query {
        if selection.is_selected {
            if !selected_entities.contains(&entity) {
                selected_entities.push(entity);
            }
        } else {
            if let Some(idx) = selected_entities.iter().position(|e| *e == entity) {
                selected_entities.remove(idx);
            }
        }
    }
}

/// Debug system to verify that spatial interaction clicks are being detected.
///
/// This system logs when BoundaryRegion or EnvironmentRegion entities are clicked,
/// helping verify that the spatial interaction infrastructure is working correctly.
///
/// Enable by adding this system to the debug schedule and watching console output
/// when clicking on boundary rings or environment areas around systems.
pub fn debug_spatial_clicks(
    mut click_events: MessageReader<Pointer<Click>>,
    boundary_query: Query<&BoundaryRegion>,
    environment_query: Query<&EnvironmentRegion>,
) {
    for event in click_events.read() {
        if let Ok(boundary) = boundary_query.get(event.entity) {
            info!(
                "🎯 BOUNDARY CLICKED for system {:?}",
                boundary.system_entity
            );
        } else if let Ok(environment) = environment_query.get(event.entity) {
            info!(
                "🌍 ENVIRONMENT CLICKED for system {:?}",
                environment.system_entity
            );
        } else {
            // Log regular system clicks too for comparison
            debug!("Regular entity clicked: {:?}", event.entity);
        }
    }
}
