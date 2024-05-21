use crate::components::*;
use crate::constants::INTERFACE_WIDTH_HALF;
use crate::plugins::label::NameLabel;
use crate::plugins::mouse_interaction::{PickSelection, PickTarget};
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

pub fn cleanup_labelled_removal(
    mut commands: Commands,
    mut removed: RemovedComponents<NameLabel>,
    label_query: Query<(Entity, &PickTarget, Option<&Parent>)>,
) {
    for removed in removed.read() {
        for (label_entity, pick_target, parent) in &label_query {
            if pick_target.target == removed {
                if let Some(parent) = parent {
                    commands
                        .entity(parent.get())
                        .remove_children(&[label_entity]);
                }

                commands.entity(label_entity).despawn_recursive();
            }
        }
    }
}

pub fn cleanup_interface_removal(
    mut commands: Commands,
    mut removed_interfaces: RemovedComponents<Interface>,
    mut flow_query: Query<(
        Entity,
        &mut FlowCurve,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
    )>,
) {
    for removed_interface in removed_interfaces.read() {
        for (flow_entity, mut flow_curve, flow_start_connection, flow_end_connection) in
            &mut flow_query
        {
            if flow_start_connection
                .map(|connection| connection.target == removed_interface)
                .unwrap_or(false)
            {
                commands
                    .entity(flow_entity)
                    .remove::<FlowStartInterfaceConnection>();

                let offset = flow_curve.start_direction * INTERFACE_WIDTH_HALF;
                flow_curve.start -= offset;
            }

            if flow_end_connection
                .map(|connection| connection.target == removed_interface)
                .unwrap_or(false)
            {
                commands
                    .entity(flow_entity)
                    .remove::<FlowEndInterfaceConnection>();

                let offset = flow_curve.end_direction * INTERFACE_WIDTH_HALF;
                flow_curve.end -= offset;
            }
        }
    }
}

pub fn cleanup_subsystem_removal(
    mut commands: Commands,
    mut removed_subsystems: RemovedComponents<Subsystem>,
    flow_query: Query<(
        Entity,
        Option<&FlowStartConnection>,
        Option<&FlowEndConnection>,
    )>,
) {
    for removed_subsystem in removed_subsystems.read() {
        for (flow_entity, flow_start_connection, flow_end_connection) in &flow_query {
            if flow_start_connection
                .map(|connection| connection.target == removed_subsystem)
                .unwrap_or(false)
            {
                commands.entity(flow_entity).remove::<FlowStartConnection>();
            }
            if flow_end_connection
                .map(|connection| connection.target == removed_subsystem)
                .unwrap_or(false)
            {
                commands.entity(flow_entity).remove::<FlowEndConnection>();
            }
        }
    }
}
