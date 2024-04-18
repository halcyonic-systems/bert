use crate::bundles::spawn_create_button;
use crate::components::{System, *};
use crate::resources::{FocusedSystem, Zoom};
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
    flow_interface_query: Query<(&Outflow, &OutflowInterfaceConnection)>,
    transform_query: Query<&GlobalTransform>,
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

    spawn_create_button(
        &mut commands,
        CreateButton {
            ty: CreateButtonType::Outflow,
            connection_source: focused_system,
            system: focused_system,
        },
        position,
        angle,
        **zoom,
        &asset_server,
    );
}

pub fn add_consecutive_outflow_create_button(
    mut commands: Commands,
    query: Query<(&Transform, &Outflow), Added<OutflowSinkConnection>>,
    flow_interface_query: Query<(&Outflow, &OutflowInterfaceConnection)>,
    transform_query: Query<&GlobalTransform>,
    system_query: Query<&crate::components::System>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, outflow)) = query.get_single() {
        let system_entity = outflow.system;

        let (position, angle) = next_outflow_button_transform(
            &flow_interface_query,
            &transform_query,
            &system_query,
            **focused_system,
        );

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Outflow,
                connection_source: system_entity,
                system: **focused_system,
            },
            position,
            angle,
            **zoom,
            &asset_server,
        );
    }
}

fn next_outflow_button_transform(
    flow_interface_query: &Query<(&Outflow, &OutflowInterfaceConnection)>,
    transform_query: &Query<&GlobalTransform>,
    system_query: &Query<&System>,
    focused_system: Entity,
) -> (Vec2, f32) {
    let system_center = transform_query
        .get(focused_system)
        .expect("System should have a Transform")
        .translation();

    let mut existing_interfaces = HashSet::new();

    for (outflow, flow_interface_connection) in flow_interface_query {
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

    min_angle -= f32::FRAC_PI_8();

    let radius = system_query
        .get(focused_system)
        .expect("Focused system should have a System")
        .radius;

    let position = Mat2::from_angle(min_angle) * vec2(system_center.x + radius, system_center.y);

    (position, min_angle)
}
