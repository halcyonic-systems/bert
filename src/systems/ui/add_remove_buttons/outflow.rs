use crate::bundles::spawn_create_button;
use crate::components::*;
use crate::resources::FocusedSystem;
use bevy::log::info;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::HashSet;
use num_traits::FloatConst;

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
    flow_interface_query: Query<
        (&Outflow, &OutflowInterfaceConnection),
        With<InflowSourceConnection>,
    >,
    transform_query: Query<&GlobalTransform>,
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

    // find button angle

    let system_center = transform_query
        .get(focused_system)
        .expect("System should have a Transform")
        .translation();

    let mut existing_interfaces = HashSet::new();

    for (outflow, flow_interface_connection) in &flow_interface_query {
        if outflow.system == focused_system {
            existing_interfaces.insert(flow_interface_connection.target);
        }
    }

    let mut min_angle = f32::PI();

    for interface in existing_interfaces {
        let interface_pos = transform_query
            .get(interface)
            .expect("Interface should have a Transform")
            .translation();

        let angle = (interface_pos - system_center).truncate().to_angle();

        min_angle = min_angle.min(angle);
    }

    info!("System center: {}", system_center);
    info!("Min angle: {}", min_angle);

    min_angle -= f32::FRAC_PI_8();

    spawn_create_button(
        &mut commands,
        CreateButton {
            ty: CreateButtonType::Outflow,
            connection_source: focused_system,
            system: focused_system,
        },
        vec2(system_center.x + 32.0, system_center.y),
        min_angle,
        &asset_server,
    );
}

pub fn add_consecutive_outflow_create_button(
    mut commands: Commands,
    query: Query<(&Transform, &Outflow), Added<OutflowSinkConnection>>,
    focused_system: Res<FocusedSystem>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, outflow)) = query.get_single() {
        let system_entity = outflow.system;

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Outflow,
                connection_source: system_entity,
                system: **focused_system,
            },
            vec2(128.0, transform.translation.y - 70.0),
            0.0,
            &asset_server,
        );
    }
}
