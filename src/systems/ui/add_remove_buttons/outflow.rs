use crate::bundles::{despawn_create_button_with_component, spawn_create_button};
use crate::components::{System, *};
use crate::resources::{FocusedSystem, Zoom};
use crate::systems::next_outflow_button_transform;
use bevy::prelude::*;

pub fn add_outflow_create_button(
    mut commands: Commands,
    focused_system: Res<FocusedSystem>,
    outflow_finished_query: Query<&FlowStartConnection, Added<FlowEndConnection>>,
    button_query: Query<(Entity, &CreateButton, Option<&Parent>)>,
    flow_system_query: Query<
        &FlowStartConnection,
        Or<(
            Without<FlowEndConnection>,
            Without<FlowStartInterfaceConnection>,
        )>,
    >,
    flow_interface_query: Query<(&FlowStartConnection, &FlowStartInterfaceConnection)>,
    import_subsystem_query: Query<(), Or<(With<ImportSubsystem>, Without<Subsystem>)>>,
    transform_query: Query<&Transform>,
    system_query: Query<&System>,
    removed_end_connections: RemovedComponents<FlowEndConnection>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    if !focused_system.is_changed()
        && outflow_finished_query.get_single().is_err()
        && removed_end_connections.is_empty()
    {
        return;
    }

    let focused_system = **focused_system;

    if import_subsystem_query.get(focused_system).is_err() {
        return;
    }

    let mut button_entities = vec![];
    for (button_entity, button, parent) in &button_query {
        if button.system == focused_system && matches!(button.ty, CreateButtonType::Outflow) {
            button_entities.push((button_entity, button, parent));
        }
    }

    for outflow_connection in &flow_system_query {
        if outflow_connection.target == focused_system {
            for (entity, button, parent) in button_entities {
                despawn_create_button_with_component(&mut commands, entity, button, parent)
            }
            return;
        }
    }

    if !button_entities.is_empty() {
        return;
    }

    let (position, angle) = next_outflow_button_transform(
        &flow_interface_query,
        &transform_query,
        &system_query,
        focused_system,
        false, // TODO
    );

    spawn_create_button(
        &mut commands,
        CreateButton {
            ty: CreateButtonType::Outflow,
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
}
