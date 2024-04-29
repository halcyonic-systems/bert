use crate::components::*;
use crate::constants::{EXTERNAL_ENTITY_WIDTH_HALF, FLOW_END_LENGTH, INTERFACE_WIDTH_HALF};
use crate::events::*;
use bevy::prelude::*;

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
) {
    for event in events.read() {
        if event.has_bubbled() {
            continue;
        }

        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");
        transform.translation.x += event.delta.x;
        transform.translation.y -= event.delta.y;

        let system = get_system_from_connected_flow(event.target, &flow_query);

        if let Ok(subsystem) = subsystem_query.get(system) {
            let parent_system = system_query
                .get(subsystem.parent_system)
                .expect("Parent system has to exist");
            transform.translation = transform.translation.clamp_length_max(parent_system.radius);
        }
    }
}

pub fn update_flow_from_external_entity(
    external_entity_query: Query<(Entity, &Transform), (With<ExternalEntity>, Changed<Transform>)>,
    mut flow_query: Query<(
        &mut FlowCurve,
        Option<&InflowSourceConnection>,
        Option<&OutflowSinkConnection>,
    )>,
) {
    for (target, transform) in &external_entity_query {
        for (mut flow_curve, inflow_source_connection, outflow_sink_connection) in &mut flow_query {
            match (inflow_source_connection, outflow_sink_connection) {
                (Some(inflow_source_connection), None) => {
                    if inflow_source_connection.target == target {
                        flow_curve.start = transform.translation.truncate()
                            - transform.right().truncate() * EXTERNAL_ENTITY_WIDTH_HALF;
                    } else {
                        continue;
                    }
                }
                (None, Some(outflow_sink_connection)) => {
                    if outflow_sink_connection.target == target {
                        flow_curve.end = transform.translation.truncate()
                            - transform.right().truncate() * EXTERNAL_ENTITY_WIDTH_HALF;
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
    interface_query: Query<(Entity, &Transform, &Parent), (With<Interface>, Changed<Transform>)>,
    transform_query: Query<&Transform>,
    parent_query: Query<&Parent>,
    mut flow_query: Query<(
        &mut FlowCurve,
        &Parent,
        Option<&InflowInterfaceConnection>,
        Option<&OutflowInterfaceConnection>,
    )>,
) {
    for (target, transform, parent) in &interface_query {
        for (
            mut flow_curve,
            flow_parent,
            inflow_interface_connection,
            outflow_interface_connection,
        ) in &mut flow_query
        {
            match (inflow_interface_connection, outflow_interface_connection) {
                (Some(inflow_interface_connection), None) => {
                    if inflow_interface_connection.target == target {
                        let (end, dir) = compute_end_and_direction(
                            parent,
                            transform,
                            &transform_query,
                            &parent_query,
                            flow_parent.get(),
                        );
                        flow_curve.end = end;
                        flow_curve.end_direction = dir;
                    } else {
                        continue;
                    }
                }
                (None, Some(outflow_interface_connection)) => {
                    if outflow_interface_connection.target == target {
                        let (start, dir) = compute_end_and_direction(
                            parent,
                            transform,
                            &transform_query,
                            &parent_query,
                            flow_parent.get(),
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

fn compute_end_and_direction(
    parent: &Parent,
    interface_transform: &Transform,
    transform_query: &Query<&Transform>,
    parent_query: &Query<&Parent>,
    flow_parent: Entity,
) -> (Vec2, Vec2) {
    let mut combined_transform = *interface_transform;
    let mut parent_entity = parent.get();

    loop {
        let parent_transform = transform_query
            .get(parent_entity)
            .expect("Parent should have a Transform");

        combined_transform = parent_transform.mul_transform(combined_transform);

        parent_entity = parent_query
            .get(parent_entity)
            .expect("There has to be a System some time")
            .get();

        if parent_entity == flow_parent {
            break;
        }
    }

    let right = combined_transform.right().truncate();

    (
        combined_transform.translation.truncate() + right * INTERFACE_WIDTH_HALF,
        right * FLOW_END_LENGTH,
    )
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
) {
    for event in events.read() {
        if event.has_bubbled() {
            continue;
        }

        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");
        transform.translation.x += event.delta.x;
        transform.translation.y -= event.delta.y;

        let system = get_system_from_connected_flow(event.target, &flow_query);

        let system = system_query
            .get(system)
            .expect("System should have a Transform");

        let interface_pos = transform.translation.truncate();

        let mut pos = interface_pos;
        pos *= system.radius / pos.length();

        transform.rotation = Quat::from_rotation_z(pos.to_angle());

        transform.translation = pos.extend(transform.translation.z);
    }
}
