use crate::bundles::spawn_create_button;
use crate::components::{System, *};
use crate::resources::{FocusedSystem, Zoom};
use crate::systems::next_outflow_button_transform;
use bevy::prelude::*;

pub fn add_first_outflow_create_button(
    mut commands: Commands,
    focused_system: Res<FocusedSystem>,
    button_query: Query<&CreateButton>,
    flow_system_query: Query<
        &Outflow,
        Or<(
            Without<OutflowSinkConnection>,
            Without<OutflowInterfaceConnection>,
        )>,
    >,
    flow_interface_query: Query<(&Outflow, &OutflowInterfaceConnection)>,
    transform_query: Query<&Transform>,
    system_query: Query<&System>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    if !focused_system.is_changed() {
        return;
    }

    let focused_system = **focused_system;

    for button in &button_query {
        if button.system == focused_system && matches!(button.ty, CreateButtonType::Outflow) {
            return;
        }
    }

    for outflow in &flow_system_query {
        if outflow.system == focused_system {
            return;
        }
    }

    let (position, angle) = next_outflow_button_transform(
        &flow_interface_query,
        &transform_query,
        &system_query,
        focused_system,
    );

    info!("first outflow button: {} {}", position, angle);

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

pub fn add_consecutive_outflow_create_button(
    mut commands: Commands,
    query: Query<&Outflow, Added<OutflowSinkConnection>>,
    flow_interface_query: Query<(&Outflow, &OutflowInterfaceConnection)>,
    transform_query: Query<&Transform>,
    system_query: Query<&crate::components::System>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(outflow) = query.get_single() {
        let system_entity = outflow.system;

        let (position, angle) = next_outflow_button_transform(
            &flow_interface_query,
            &transform_query,
            &system_query,
            **focused_system,
        );
        info!("consecutive outflow button: {} {}", position, angle);

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Outflow,
                connection_source: system_entity,
                system: **focused_system,
                substance_type: None,
            },
            position,
            angle,
            **zoom,
            Some(**focused_system),
            &asset_server,
        );
    }
}
