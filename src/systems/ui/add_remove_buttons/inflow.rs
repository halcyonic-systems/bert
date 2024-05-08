use crate::bundles::{despawn_create_button_with_component, spawn_create_button};
use crate::components::*;
use crate::resources::{FocusedSystem, Zoom};
use crate::systems::next_inflow_button_transform;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub fn add_inflow_create_button(
    mut commands: Commands,
    inflow_finished_query: Query<&FlowEndConnection, Added<FlowStartConnection>>,
    flow_changed_query: Query<Entity, Or<(Added<FlowEndConnection>, Changed<Flow>)>>,
    outflow_query: Query<(&Flow, &FlowStartConnection), With<FlowEndConnection>>,
    incomplete_inflow_query: Query<
        &FlowEndConnection,
        Or<(
            Without<FlowStartConnection>,
            Without<FlowEndInterfaceConnection>,
        )>,
    >,
    button_query: Query<(Entity, &CreateButton)>,
    flow_interface_query: Query<(&FlowEndConnection, &FlowEndInterfaceConnection)>,
    transform_query: Query<&Transform>,
    system_query: Query<&crate::components::System>,
    system_entity_query: Query<Entity, With<crate::components::System>>,
    export_subsystem_query: Query<(), Or<(With<ExportSubsystem>, Without<Subsystem>)>>,
    import_subsystem_query: Query<&InterfaceSubsystem, With<ImportSubsystem>>,
    interface_subsystem_changed_query: Query<(), Changed<InterfaceSubsystem>>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    // TODO : also detect removal

    if flow_changed_query.is_empty()
        && !focused_system.is_changed()
        && inflow_finished_query.get_single().is_err()
        && interface_subsystem_changed_query.is_empty()
    {
        return;
    }

    let focused_system = **focused_system;
    let is_export_subsystem = export_subsystem_query.get(focused_system).is_ok();

    let mut system_to_outflow_usabilities = HashMap::new();

    for entity in &system_entity_query {
        system_to_outflow_usabilities.insert(entity, HashSet::new());
    }

    for (outflow, flow_start_connection) in &outflow_query {
        if matches!(flow_start_connection.target_type, StartTargetType::System) {
            let system_entity = flow_start_connection.target;
            system_to_outflow_usabilities
                .get_mut(&system_entity)
                .expect("Initialized just above")
                .insert(OutflowUsability::from_useful(outflow.is_useful));
        }
    }

    'outer: for (system_entity, outflow_usabilities) in system_to_outflow_usabilities {
        if system_entity == focused_system {
            let is_completed_import_subsystem =
                if let Ok(interface_subsystem) = import_subsystem_query.get(system_entity) {
                    interface_subsystem.total_outflow >= interface_subsystem.total_inflow
                } else {
                    false
                };

            if (outflow_usabilities.len() > 1
                || is_export_subsystem
                || is_completed_import_subsystem)
            {
                for inflow_end_connection in incomplete_inflow_query.iter() {
                    if inflow_end_connection.target == system_entity {
                        continue 'outer;
                    }
                }

                for (_, button) in button_query.iter() {
                    if matches!(button.ty, CreateButtonType::Inflow)
                        && button.connection_source == system_entity
                    {
                        continue 'outer;
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
                        connection_source: system_entity,
                        system: focused_system,
                        substance_type: None,
                    },
                    position,
                    angle,
                    **zoom,
                    Some(focused_system),
                    &asset_server,
                );
            } else {
                for (entity, button) in &button_query {
                    if matches!(button.ty, CreateButtonType::Inflow)
                        && button.connection_source == system_entity
                    {
                        despawn_create_button_with_component(&mut commands, entity, button)
                    }
                }
            }
        } else {
            for (entity, button) in &button_query {
                if matches!(button.ty, CreateButtonType::Inflow)
                    && button.connection_source == system_entity
                {
                    despawn_create_button_with_component(&mut commands, entity, button)
                }
            }
        }
    }
}
