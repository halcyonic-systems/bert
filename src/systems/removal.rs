use crate::components::*;
use crate::plugins::mouse_interaction::PickSelection;
use bevy::prelude::*;

pub fn remove_selected_elements(
    mut commands: Commands,
    selected_query: Query<(Entity, &PickSelection, Option<&Parent>)>,
) {
    for (entity_to_remove, selection, parent) in &selected_query {
        if selection.is_selected {
            if let Some(parent) = parent {
                commands
                    .entity(parent.get())
                    .remove_children(&[entity_to_remove]);
            }
            commands.entity(entity_to_remove).despawn_recursive();
        }
    }
}

pub fn cleanup_external_entity_removal(
    mut commands: Commands,
    mut removed_external_entities: RemovedComponents<ExternalEntity>,
    flow_query: Query<(Entity, &FlowStartConnection, &FlowEndConnection)>,
) {
    for removed_external_entity in removed_external_entities.read() {
        for (flow_entity, flow_start_connection, flow_end_connection) in &flow_query {
            if flow_start_connection.target == removed_external_entity {
                commands.entity(flow_entity).remove::<FlowStartConnection>();
            }
            if flow_end_connection.target == removed_external_entity {
                commands.entity(flow_entity).remove::<FlowEndConnection>();
            }
        }
    }
}
