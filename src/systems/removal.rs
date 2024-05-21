use crate::components::*;
use crate::constants::INTERFACE_WIDTH_HALF;
use crate::plugins::label::NameLabel;
use crate::plugins::mouse_interaction::{PickSelection, PickTarget};
use bevy::prelude::*;

pub fn remove_selected_elements(
    mut commands: Commands,
    selected_query: Query<(Entity, &PickSelection, Option<&Parent>)>,
    flow_query: Query<
        (
            Option<&FlowStartConnection>,
            Option<&FlowEndConnection>,
            Option<&FlowStartInterfaceConnection>,
            Option<&FlowEndInterfaceConnection>,
        ),
        With<Flow>,
    >,
    parent_query: Query<&Parent>,
) {
    for (entity_to_remove, selection, parent) in &selected_query {
        if selection.is_selected {
            if let Ok((
                start_connection,
                end_connection,
                start_interface_connection,
                end_interface_connection,
            )) = flow_query.get(entity_to_remove)
            {
                if let Some(start_connection) = start_connection {
                    if matches!(start_connection.target_type, StartTargetType::Source) {
                        remove_entity(&mut commands, start_connection.target, &parent_query);
                    }
                }

                if let Some(end_connection) = end_connection {
                    if matches!(end_connection.target_type, EndTargetType::Sink) {
                        remove_entity(&mut commands, end_connection.target, &parent_query);
                    }
                }

                if let Some(start_interface_connection) = start_interface_connection {
                    remove_entity(
                        &mut commands,
                        start_interface_connection.target,
                        &parent_query,
                    );
                }

                if let Some(end_interface_connection) = end_interface_connection {
                    remove_entity(
                        &mut commands,
                        end_interface_connection.target,
                        &parent_query,
                    );
                }
            }

            if let Some(parent) = parent {
                commands
                    .entity(parent.get())
                    .remove_children(&[entity_to_remove]);
            }
            commands.entity(entity_to_remove).despawn_recursive();
        }
    }
}

fn remove_entity(commands: &mut Commands, entity_to_remove: Entity, parent_query: &Query<&Parent>) {
    if let Ok(parent) = parent_query.get(entity_to_remove) {
        commands
            .entity(parent.get())
            .remove_children(&[entity_to_remove]);
    }

    commands.entity(entity_to_remove).despawn_recursive();
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
    interface_query: Query<(Entity, &InterfaceSubsystemConnection)>,
) {
    for removed_subsystem in removed_subsystems.read() {
        for (flow_entity, flow_start_connection, flow_end_connection) in &flow_query {
            if flow_start_connection
                .map(|connection| connection.target == removed_subsystem)
                .unwrap_or(false)
            {
                commands.entity(flow_entity).remove::<FlowStartConnection>();

                if let Some(connection) = flow_end_connection {
                    if matches!(connection.target_type, EndTargetType::Sink) {
                        commands.entity(connection.target).despawn_recursive();
                        commands.entity(flow_entity).despawn_recursive();
                    }
                }
            }
            if flow_end_connection
                .map(|connection| connection.target == removed_subsystem)
                .unwrap_or(false)
            {
                commands.entity(flow_entity).remove::<FlowEndConnection>();

                if let Some(connection) = flow_start_connection {
                    if matches!(connection.target_type, StartTargetType::Source) {
                        commands.entity(connection.target).despawn_recursive();
                        commands.entity(flow_entity).despawn_recursive();
                    }
                }
            }
        }

        for (interface_entity, interface_subsystem_connection) in &interface_query {
            if interface_subsystem_connection.target == removed_subsystem {
                commands
                    .entity(interface_entity)
                    .remove::<InterfaceSubsystemConnection>();
            }
        }
    }
}
