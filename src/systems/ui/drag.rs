use crate::components::*;
use crate::constants::EXTERNAL_ENTITY_WIDTH_HALF;
use crate::events::*;
use bevy::math::vec2;
use bevy::prelude::*;

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
    subsystem_query: Query<&crate::components::System, With<Subsystem>>,
) {
    for event in events.read() {
        let delta = vec2(event.delta.x, -event.delta.y);

        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;

        for (inflow, inflow_source_connection, outflow, outflow_sink_connection) in &flow_query {
            let system = match (
                inflow,
                inflow_source_connection,
                outflow,
                outflow_sink_connection,
            ) {
                (Some(inflow), Some(inflow_source_connection), None, None) => {
                    if inflow_source_connection.target == event.target {
                        inflow.system
                    } else {
                        continue;
                    }
                }
                (None, None, Some(outflow), Some(outflow_sink_connection)) => {
                    if outflow_sink_connection.target == event.target {
                        outflow.system
                    } else {
                        continue;
                    }
                }
                _ => unreachable!("Either Inflow or Outflow has to be there but not both"),
            };

            if let Ok(system) = subsystem_query.get(system) {
                transform.translation = transform.translation.clamp_length_max(system.radius);
            }

            break;
        }
    }
}

pub fn update_flow_from_connected_elements(
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

pub fn drag_interface(
    mut events: EventReader<InterfaceDrag>,
    mut transform_query: Query<&mut Transform>,
    mut flow_query: Query<(
        Option<&Inflow>,
        Option<&InflowInterfaceConnection>,
        Option<&Outflow>,
        Option<&OutflowInterfaceConnection>,
    )>,
    system_query: Query<&crate::components::System>,
) {
    for event in events.read() {
        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");

        // let system = update_transform_and_get_system(
        //     transform.as_mut(),
        //     &mut flow_query,
        //     event.delta,
        //     event.target,
        // );
        //
        // drop(transform);
        //
        // let system_transform = transform_query
        //     .get(system)
        //     .expect("System should have a Transform");
        // let system = system_query.get(system).expect("System should exist");
        // let system_pos = system_transform.translation.truncate();
        //
        // let mut transform = transform_query
        //     .get_mut(event.target)
        //     .expect("External entity should have a Transform");
        //
        // let interface_pos = transform.translation.truncate();
        //
        // let mut diff = interface_pos - system_pos;
        // diff *= system.radius / diff.length();
        //
        // transform.translation = (system_pos + diff).extend(transform.translation.z);
    }
}
