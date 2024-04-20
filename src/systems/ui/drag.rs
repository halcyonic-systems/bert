use bevy::math::vec2;
use crate::components::*;
use crate::events::ExternalEntityDrag;
use bevy::prelude::*;

pub fn drag_external_entity(
    mut events: EventReader<ExternalEntityDrag>,
    mut transform_query: Query<&mut Transform>,
    mut flow_query: Query<(
        &mut FlowCurve,
        Option<&Inflow>,
        Option<&InflowSourceConnection>,
        Option<&Outflow>,
        Option<&OutflowSinkConnection>,
    )>,
    subsystem_query: Query<&crate::components::System, With<Subsystem>>,
) {
    for event in events.read() {
        for (mut flow_curve, inflow, inflow_source_connection, outflow, outflow_sink_connection) in
            &mut flow_query
        {
            let delta = vec2(event.delta.x, -event.delta.y);

            let system = match (
                inflow,
                inflow_source_connection,
                outflow,
                outflow_sink_connection,
            ) {
                (Some(inflow), Some(inflow_source_connection), None, None) => {
                    if inflow_source_connection.target == event.target {
                        flow_curve.start += delta;
                        inflow.system
                    } else {
                        continue;
                    }
                }
                (None, None, Some(outflow), Some(outflow_sink_connection)) => {
                    if outflow_sink_connection.target == event.target {
                        flow_curve.end += delta;
                        outflow.system
                    } else {
                        continue;
                    }
                }
                _ => unreachable!("Either Inflow or Outflow has to be there but not both"),
            };

            let mut transform = transform_query
                .get_mut(event.target)
                .expect("External entity should have a Transform");
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;

            if let Ok(system) = subsystem_query.get(system) {
                transform.translation = transform.translation.clamp_length_max(system.radius);
            }
        }
    }
}
