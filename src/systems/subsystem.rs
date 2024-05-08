use crate::components::*;
use crate::utils::{all_flow_end_connected_systems, all_flow_start_connected_systems};
use bevy::prelude::*;
use rust_decimal_macros::dec;

pub fn interface_subsystem_should_update(
    flow_changed_query: Query<
        (),
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
    removed_end_connections: RemovedComponents<FlowEndConnection>,
    removed_start_connections: RemovedComponents<FlowStartConnection>,
) -> bool {
    !flow_changed_query.is_empty()
        || !removed_end_connections.is_empty()
        || !removed_start_connections.is_empty()
}

pub fn update_interface_subsystem_from_flows(
    flow_query: Query<(
        &Flow,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
        &FlowStartConnection,
        &FlowEndConnection,
    )>,
    mut interface_subsystem_query: Query<(Entity, &mut InterfaceSubsystem)>,
) {
    for (_, mut interface_subsystem) in &mut interface_subsystem_query {
        interface_subsystem.total_inflow = dec!(0);
        interface_subsystem.total_outflow = dec!(0);
    }

    for (
        flow,
        flow_start_interface_connection,
        flow_end_interface_connection,
        flow_start_connection,
        flow_end_connection,
    ) in &flow_query
    {
        let mut lens = interface_subsystem_query.transmute_lens::<(Entity, &InterfaceSubsystem)>();
        let query = lens.query();

        let end_system_entities = all_flow_end_connected_systems(
            (Some(flow_end_connection), flow_end_interface_connection),
            &query,
        );

        let start_system_entities = all_flow_start_connected_systems(
            (Some(flow_start_connection), flow_start_interface_connection),
            &query,
        );

        for system_entity in end_system_entities {
            if let Ok((_, mut interface_subsystem)) =
                interface_subsystem_query.get_mut(system_entity)
            {
                interface_subsystem.total_inflow += flow.amount;
            }
        }

        for system_entity in start_system_entities {
            if let Ok((_, mut interface_subsystem)) =
                interface_subsystem_query.get_mut(system_entity)
            {
                interface_subsystem.total_outflow += flow.amount;
            }
        }
    }
}
