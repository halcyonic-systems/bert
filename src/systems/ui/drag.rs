use crate::components::*;
use crate::constants::{EXTERNAL_ENTITY_WIDTH_HALF, FLOW_END_LENGTH};
use crate::events::*;
use crate::resources::Zoom;
use crate::utils::compute_end_and_direction_from_system_child;
use bevy::prelude::*;
use std::ops::DerefMut;

fn get_system_from_connected_flow<InConn, OutConn>(
    target: Entity,
    flow_query: &Query<
        (
            Option<&Inflow>,
            Option<&InConn>,
            Option<&Outflow>,
            Option<&OutConn>,
        ),
        With<FlowCurve>,
    >,
) -> Entity
where
    InConn: Component + Connection,
    OutConn: Component + Connection,
{
    for (inflow, inflow_connection, outflow, outflow_connection) in flow_query {
        match (inflow, inflow_connection, outflow, outflow_connection) {
            (Some(inflow), Some(inflow_connection), None, None) => {
                if inflow_connection.target() == target {
                    return inflow.system;
                } else {
                    continue;
                }
            }
            (None, None, Some(outflow), Some(outflow_connection)) => {
                if outflow_connection.target() == target {
                    return outflow.system;
                } else {
                    continue;
                }
            }
            _ => unreachable!("Either Inflow or Outflow has to be there but not both"),
        };
    }

    unreachable!("System should exist")
}

pub fn drag_external_entity(
    mut events: EventReader<ExternalEntityDrag>,
    mut transform_query: Query<&mut Transform>,
    flow_query: Query<
        (
            Option<&Inflow>,
            Option<&InflowSourceConnection>,
            Option<&Outflow>,
            Option<&OutflowSinkConnection>,
        ),
        With<FlowCurve>,
    >,
    subsystem_query: Query<&Subsystem>,
    system_query: Query<&crate::components::System>,
    zoom: Res<Zoom>,
) {
    for event in events.read() {
        if event.has_bubbled() {
            continue;
        }

        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");
        transform.translation.x = event.position.x;
        transform.translation.y = event.position.y;

        let system = get_system_from_connected_flow(event.target, &flow_query);

        if let Ok(subsystem) = subsystem_query.get(system) {
            let parent_system = system_query
                .get(subsystem.parent_system)
                .expect("Parent system has to exist");
            transform.translation = transform
                .translation
                .clamp_length_max(parent_system.radius * **zoom);
        }
    }
}

pub fn update_flow_from_external_entity(
    external_entity_query: Query<
        (Entity, &Transform, &NestingLevel),
        (With<ExternalEntity>, Changed<Transform>),
    >,
    mut flow_query: Query<(
        &mut FlowCurve,
        Option<&InflowSourceConnection>,
        Option<&OutflowSinkConnection>,
    )>,
    zoom: Res<Zoom>,
) {
    for (target, transform, nesting_level) in &external_entity_query {
        for (mut flow_curve, inflow_source_connection, outflow_sink_connection) in &mut flow_query {
            let scale = NestingLevel::compute_scale(**nesting_level, **zoom);

            match (inflow_source_connection, outflow_sink_connection) {
                (Some(inflow_source_connection), None) => {
                    if inflow_source_connection.target == target {
                        let right = transform.right().truncate();
                        flow_curve.start = transform.translation.truncate()
                            - right * EXTERNAL_ENTITY_WIDTH_HALF * scale;
                        flow_curve.start_direction = -right * FLOW_END_LENGTH * scale;
                    } else {
                        continue;
                    }
                }
                (None, Some(outflow_sink_connection)) => {
                    if outflow_sink_connection.target == target {
                        let right = transform.right().truncate();
                        flow_curve.end = transform.translation.truncate()
                            - right * EXTERNAL_ENTITY_WIDTH_HALF * scale;
                        flow_curve.end_direction = -right * FLOW_END_LENGTH * scale;
                    } else {
                        continue;
                    }
                }
                _ => {
                    // do nothing
                }
            }
        }
    }
}

pub fn update_flow_from_interface(
    interface_query: Query<(Entity, &NestingLevel), (With<Interface>, Changed<Transform>)>,
    transform_query: Query<&Transform>,
    parent_query: Query<&Parent>,
    mut flow_query: Query<(
        &mut FlowCurve,
        Option<&Parent>,
        Option<&InflowInterfaceConnection>,
        Option<&OutflowInterfaceConnection>,
    )>,
    zoom: Res<Zoom>,
) {
    for (target, nesting_level) in &interface_query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);

        for (
            mut flow_curve,
            flow_parent,
            inflow_interface_connection,
            outflow_interface_connection,
        ) in &mut flow_query
        {
            let flow_parent = flow_parent.map(|p| p.get());

            match (inflow_interface_connection, outflow_interface_connection) {
                (Some(inflow_interface_connection), None) => {
                    if inflow_interface_connection.target == target {
                        let (end, dir) = compute_end_and_direction_from_system_child(
                            target,
                            &transform_query,
                            &parent_query,
                            flow_parent,
                            **zoom,
                            scale,
                        );
                        flow_curve.end = end;
                        flow_curve.end_direction = dir;
                    } else {
                        continue;
                    }
                }
                (None, Some(outflow_interface_connection)) => {
                    if outflow_interface_connection.target == target {
                        let (start, dir) = compute_end_and_direction_from_system_child(
                            target,
                            &transform_query,
                            &parent_query,
                            flow_parent,
                            **zoom,
                            scale,
                        );
                        flow_curve.start = start;
                        flow_curve.start_direction = dir;
                    } else {
                        continue;
                    }
                }
                _ => {
                    // do nothing
                }
            }
        }
    }
}

pub fn drag_interface(
    mut events: EventReader<InterfaceDrag>,
    mut transform_query: Query<&mut Transform, Without<crate::components::System>>,
    flow_query: Query<
        (
            Option<&Inflow>,
            Option<&InflowInterfaceConnection>,
            Option<&Outflow>,
            Option<&OutflowInterfaceConnection>,
        ),
        With<FlowCurve>,
    >,
    system_query: Query<&crate::components::System>,
    zoom: Res<Zoom>,
) {
    for event in events.read() {
        if event.has_bubbled() {
            continue;
        }

        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");
        transform.translation.x = event.position.x;
        transform.translation.y = event.position.y;

        let system = get_system_from_connected_flow(event.target, &flow_query);

        let system = system_query
            .get(system)
            .expect("System should have a Transform");

        let interface_pos = transform.translation.truncate();

        let mut pos = interface_pos;
        pos *= system.radius * **zoom / pos.length();

        transform.rotation = Quat::from_rotation_z(pos.to_angle());

        transform.translation = pos.extend(transform.translation.z);
    }
}

pub fn update_initial_position_from_transform(
    mut query: Query<(&mut InitialPosition, &Transform), Changed<Transform>>,
    zoom: Res<Zoom>,
) {
    for (mut initial_position, transform) in &mut query {
        **initial_position = transform.translation.truncate() / **zoom;
    }
}

pub fn update_flow_from_system(
    system_query: Query<&Children, (With<crate::components::System>, Changed<Transform>)>,
    mut interface_query: Query<
        &mut Transform,
        (With<Interface>, Without<crate::components::System>),
    >,
) {
    for children in &system_query {
        for child in children.iter() {
            if let Ok(mut transform) = interface_query.get_mut(*child) {
                // touch to trigger flow updates
                let _ = transform.deref_mut();
            }
        }
    }
}

pub fn update_flow_from_interface_subsystem(
    interface_query: Query<&InterfaceSubsystemConnection, Changed<Transform>>,
    mut system_query: Query<
        &mut Transform,
        (
            With<crate::components::System>,
            Without<InterfaceSubsystemConnection>,
        ),
    >,
) {
    for interface_subsystem_connection in &interface_query {
        if let Ok(mut transform) = system_query.get_mut(interface_subsystem_connection.target) {
            // touch to trigger flow updates
            let _ = transform.deref_mut();
        }
    }
}
