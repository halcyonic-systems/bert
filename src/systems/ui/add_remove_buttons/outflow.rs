use crate::bundles::spawn_create_button;
use crate::components::{System, *};
use crate::resources::{FocusedSystem, Zoom};
use crate::systems::next_outflow_button_transform;
use bevy::prelude::*;

pub fn add_outflow_create_button(
    mut commands: Commands,
    focused_system: Res<FocusedSystem>,
    outflow_finished_query: Query<&FlowStartConnection, Added<FlowEndConnection>>,
    button_query: Query<&CreateButton>,
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
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    if !focused_system.is_changed() && outflow_finished_query.get_single().is_err() {
        return;
    }

    let focused_system = **focused_system;

    for button in &button_query {
        if button.system == focused_system && matches!(button.ty, CreateButtonType::Outflow) {
            return;
        }
    }

    for outflow_connection in &flow_system_query {
        if outflow_connection.target == focused_system {
            return;
        }
    }

    if import_subsystem_query.get(focused_system).is_err() {
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
