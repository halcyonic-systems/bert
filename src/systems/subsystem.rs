use crate::components::*;
use crate::constants::{
    SUBSYSTEM_FULL_SIZE_INTERFACE_COUNT, SUBSYSTEM_MIN_SCALING_FACTOR, SUBSYSTEM_SCALING_FACTOR,
};
use crate::events::RemoveEvent;
use crate::resources::Zoom;
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

// TODO : with change detection?
//noinspection RsBorrowChecker
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
                interface_subsystem.is_useful = flow.usability.is_useful();
                interface_subsystem.substance_type = flow.substance_type;
            }
        }

        for system_entity in start_system_entities {
            if let Ok((_, mut interface_subsystem)) =
                interface_subsystem_query.get_mut(system_entity)
            {
                interface_subsystem.total_outflow += flow.amount;
                interface_subsystem.is_useful = flow.usability.is_useful();
                interface_subsystem.substance_type = flow.substance_type;
            }
        }
    }
}

pub fn update_subsystem_radius_from_interface_count(
    changed_query: Query<(), Added<Interface>>,
    subsystem_query: Query<(Entity, &Subsystem, &Children, Option<&InterfaceSubsystem>)>,
    interface_query: Query<&Interface>,
    mut system_query: Query<&mut crate::components::System>,
    mut transform_query: Query<(&mut Transform, &mut InitialPosition)>,
    zoom: Res<Zoom>,
    mut remove_event_reader: EventReader<RemoveEvent>,
) {
    if changed_query.is_empty() && remove_event_reader.is_empty() {
        return;
    }

    remove_event_reader.clear();

    for (subsystem_entity, subsystem, children, interface_subsystem) in &subsystem_query {
        let mut interface_count = 0;

        if interface_subsystem.is_some() {
            interface_count += 1;
        }

        for child in children {
            if interface_query.get(*child).is_ok() {
                interface_count += 1;
            }
        }

        let mut scaling_factor = SUBSYSTEM_MIN_SCALING_FACTOR
            + interface_count as f32 * (SUBSYSTEM_SCALING_FACTOR - SUBSYSTEM_MIN_SCALING_FACTOR)
                / SUBSYSTEM_FULL_SIZE_INTERFACE_COUNT;

        scaling_factor = scaling_factor.min(SUBSYSTEM_SCALING_FACTOR);

        let parent_radius = system_query
            .get(subsystem.parent_system)
            .expect("Parent system should exist")
            .radius;

        let radius = parent_radius * scaling_factor * 0.5;

        system_query
            .get_mut(subsystem_entity)
            .expect("System should exist")
            .radius = radius;

        if interface_subsystem.is_some() {
            let (mut transform, mut initial_position) = transform_query
                .get_mut(subsystem_entity)
                .expect("Subsystem should have a Transform");

            initial_position.x = radius * transform.translation.x.signum();

            transform.translation = (**initial_position * **zoom).extend(transform.translation.z);
        }
    }
}

pub fn update_interface_positions_from_system_radius(
    system_query: Query<
        (&crate::components::System, &Children),
        Changed<crate::components::System>,
    >,
    mut interface_query: Query<&mut Transform, With<Interface>>,
    zoom: Res<Zoom>,
) {
    for (system, children) in &system_query {
        for child in children {
            if let Ok(mut transform) = interface_query.get_mut(*child) {
                transform.translation =
                    (transform.translation.truncate().normalize() * system.radius * **zoom)
                        .extend(transform.translation.z);
            }
        }
    }
}
