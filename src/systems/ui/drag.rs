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
        let system = match (inflow, inflow_connection, outflow, outflow_connection) {
            (Some(inflow), Some(inflow_connection), None, None) => {
                if inflow_connection.target() == target {
                    inflow.system
                } else {
                    continue;
                }
            }
            (None, None, Some(outflow), Some(outflow_connection)) => {
                if outflow_connection.target() == target {
                    outflow.system
                } else {
                    continue;
                }
            }
            _ => unreachable!("Either Inflow or Outflow has to be there but not both"),
        };

        return system;
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
                _ => unreachable!("Either Inflow or Outflow has to be there but not both"),
            }
        }
    }
}

pub fn update_flow_from_interface(
    interface_query: Query<(Entity, &Transform), (With<Interface>, Changed<Transform>)>,
    mut flow_query: Query<(
        &mut FlowCurve,
        Option<&InflowInterfaceConnection>,
        Option<&OutflowInterfaceConnection>,
    )>,
) {
    for (target, transform) in &interface_query {
        for (mut flow_curve, inflow_interface_connection, outflow_interface_connection) in
            &mut flow_query
        {
            match (inflow_interface_connection, outflow_interface_connection) {
                (Some(inflow_interface_connection), None) => {
                    if inflow_interface_connection.target == target {
                        let right = transform.right().truncate();
                        flow_curve.end =
                            transform.translation.truncate() + right * INTERFACE_WIDTH_HALF;
                        flow_curve.end_direction = right * FLOW_END_LENGTH;
                    } else {
                        continue;
                    }
                }
                (None, Some(outflow_interface_connection)) => {
                    if outflow_interface_connection.target == target {
                        let right = transform.right().truncate();
                        flow_curve.start =
                            transform.translation.truncate() + right * INTERFACE_WIDTH_HALF;
                        flow_curve.start_direction = right * FLOW_END_LENGTH;
                    } else {
                        continue;
                    }
                }
                _ => unreachable!("Either Inflow or Outflow has to be there but not both"),
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
    system_query: Query<(&Transform, &crate::components::System)>,
) {
    for event in events.read() {
        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");
        transform.translation.x += event.delta.x;
        transform.translation.y -= event.delta.y;

        let system = get_system_from_connected_flow(event.target, &flow_query);

        let (system_transform, system) = system_query
            .get(system)
            .expect("System should have a Transform");

        let system_pos = system_transform.translation.truncate();

        let interface_pos = transform.translation.truncate();

        let mut diff = interface_pos - system_pos;
        diff *= system.radius / diff.length();

        transform.rotation = Quat::from_rotation_z(diff.to_angle());

        transform.translation = (system_pos + diff).extend(transform.translation.z);
    }
}
