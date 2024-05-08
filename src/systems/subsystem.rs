use crate::components::*;
use bevy::prelude::*;
use rust_decimal_macros::dec;

pub fn update_interface_subsystem_from_flows(
    flow_changed_query: Query<
        Entity,
        (
            With<FlowStartConnection>,
            With<FlowEndConnection>,
            Or<(
                Changed<Flow>,
                Added<FlowStartConnection>,
                Added<FlowEndConnection>,
            )>,
        ),
    >,
    flow_query: Query<(
        &Flow,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
        &FlowStartConnection,
        &FlowEndConnection,
    )>,
    mut subsystem_query: Query<(Entity, &mut InterfaceSubsystem, &Parent)>,
) {
    for flow_changed_entity in &flow_changed_query {
        let (
            _,
            flow_start_interface_connection,
            flow_end_interface_connection,
            flow_start_connection,
            flow_end_connection,
        ) = flow_query
            .get(flow_changed_entity)
            .expect("This entity must be a flow");

        for (subsystem_entity, mut interface_subsystem, subsystem_parent) in &mut subsystem_query {
            // TODO : recurse up until interface is found

            let interface_entity = subsystem_parent.get();

            if flow_start_interface_connection.map(|c| c.target) == Some(interface_entity)
                || flow_end_interface_connection.map(|c| c.target) == Some(interface_entity)
                || flow_start_connection.target == subsystem_entity
                || flow_end_connection.target == subsystem_entity
            {
                interface_subsystem.total_inflow = dec!(0);
                interface_subsystem.total_outflow = dec!(0);

                for (
                    flow,
                    flow_start_interface_connection,
                    flow_end_interface_connection,
                    flow_start_connection,
                    flow_end_connection,
                ) in &flow_query
                {
                    if flow_start_interface_connection.map(|c| c.target) == Some(interface_entity)
                        || flow_start_connection.target == subsystem_entity
                    {
                        interface_subsystem.total_outflow += flow.amount;
                    }

                    if flow_end_interface_connection.map(|c| c.target) == Some(interface_entity)
                        || flow_end_connection.target == subsystem_entity
                    {
                        interface_subsystem.total_inflow += flow.amount;
                    }
                }
            }
        }
    }
}
