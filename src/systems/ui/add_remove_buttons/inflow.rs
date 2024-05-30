use crate::bundles::{despawn_create_button_with_component, spawn_create_button};
use crate::components::*;
use crate::events::RemoveEvent;
use crate::resources::{FocusedSystem, Zoom};
use crate::systems::{has_incomplete_interactions, next_inflow_button_transform};
use bevy::prelude::*;
use bevy::utils::HashSet;

pub fn inflow_create_button_needs_update(
    focused_system: Res<FocusedSystem>,
    flow_changed_query: Query<
        (),
        Or<(
            Added<FlowEndConnection>,
            Added<FlowStartConnection>,
            Added<FlowStartInterfaceConnection>,
            Added<FlowEndInterfaceConnection>,
            Changed<Flow>,
        )>,
    >,
    interface_subsystem_changed_query: Query<(), Changed<InterfaceSubsystem>>,
    mut remove_event_reader: EventReader<RemoveEvent>,
) -> bool {
    let needs_update = !flow_changed_query.is_empty()
        || focused_system.is_changed()
        || !interface_subsystem_changed_query.is_empty()
        || !remove_event_reader.is_empty();

    remove_event_reader.clear();

    needs_update
}

pub fn add_inflow_create_button(
    mut commands: Commands,
    outflow_query: Query<(&Flow, &FlowStartConnection), With<FlowEndConnection>>,
    incomplete_outflow_query: Query<
        &FlowStartConnection,
        Or<(
            Without<FlowEndConnection>,
            Without<FlowStartInterfaceConnection>,
        )>,
    >,
    incomplete_inflow_query: Query<
        &FlowEndConnection,
        Or<(
            Without<FlowStartConnection>,
            Without<FlowEndInterfaceConnection>,
        )>,
    >,
    button_query: Query<(Entity, &CreateButton, Option<&Parent>)>,
    flow_interface_query: Query<(&FlowEndConnection, &FlowEndInterfaceConnection)>,
    transform_query: Query<&Transform>,
    system_query: Query<&crate::components::System>,
    export_subsystem_query: Query<&ExportSubsystem>,
    import_subsystem_query: Query<&InterfaceSubsystem, With<ImportSubsystem>>,
    not_interface_subsystem_query: Query<(), (With<Subsystem>, Without<InterfaceSubsystem>)>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    let focused_system = **focused_system;

    let has_incomplete_interactions = has_incomplete_interactions(
        focused_system,
        &incomplete_inflow_query.iter().collect::<Vec<_>>(),
        &incomplete_outflow_query.iter().collect::<Vec<_>>(),
    );

    if !has_incomplete_interactions {
        let is_export_subsystem = export_subsystem_query.get(focused_system).is_ok();

        let mut outflow_usabilities = HashSet::new();

        for (outflow, flow_start_connection) in &outflow_query {
            if matches!(flow_start_connection.target_type, StartTargetType::System) {
                if flow_start_connection.target == focused_system {
                    outflow_usabilities.insert(outflow.usability);
                }
            }
        }

        let is_completed_import_subsystem =
            if let Ok(interface_subsystem) = import_subsystem_query.get(focused_system) {
                interface_subsystem.total_outflow >= interface_subsystem.total_inflow
            } else {
                false
            };

        let is_not_interface_subsystem = not_interface_subsystem_query.get(focused_system).is_ok();

        if outflow_usabilities.len() > 1
            || is_export_subsystem
            || is_completed_import_subsystem
            || is_not_interface_subsystem
        {
            for inflow_end_connection in incomplete_inflow_query.iter() {
                if inflow_end_connection.target == focused_system {
                    return;
                }
            }

            for (_, button, _) in button_query.iter() {
                if matches!(button.ty, CreateButtonType::Inflow)
                    && button.connection_source == focused_system
                {
                    return;
                }
            }

            let (position, angle) = next_inflow_button_transform(
                &flow_interface_query,
                &transform_query,
                &system_query,
                focused_system,
                is_completed_import_subsystem,
            );

            spawn_create_button(
                &mut commands,
                CreateButton {
                    ty: CreateButtonType::Inflow,
                    connection_source: focused_system,
                    system: focused_system,
                    substance_type: None,
                },
                position,
                angle,
                **zoom,
                Some(focused_system),
                &asset_server,
            );

            return;
        }
    }

    for (entity, button, parent) in &button_query {
        if matches!(button.ty, CreateButtonType::Inflow)
            && button.connection_source == focused_system
        {
            despawn_create_button_with_component(&mut commands, entity, button, parent)
        }
    }
}
