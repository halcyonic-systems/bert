use crate::bevy_app::components::*;
use crate::bevy_app::constants::{
    INTERFACE_SUBSYSTEM_SCALING_FACTOR, SUBSYSTEM_FULL_SIZE_INTERFACE_COUNT,
    SUBSYSTEM_MIN_SCALING_FACTOR, SUBSYSTEM_SCALING_FACTOR,
};
use crate::bevy_app::events::RemoveEvent;
use crate::bevy_app::resources::Zoom;
use crate::bevy_app::utils::{all_flow_end_connected_systems, all_flow_start_connected_systems};
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

pub fn update_sub_system_parent_system(
    mut subsystem_query: Query<(&Subsystem, &mut ParentState)>,
    system_query: Query<
        (Entity, &Name, &ElementDescription),
        (
            With<crate::bevy_app::components::System>,
            Or<(Changed<Name>, Changed<ElementDescription>)>,
        ),
    >,
) {
    for (subsystem, mut parent_state) in &mut subsystem_query {
        if let Ok((system_entity, name, element_description)) = system_query.get_single() {
            if subsystem.parent_system == system_entity {
                parent_state.name = name.as_str().to_owned();
                parent_state.description = element_description.text.clone();
            }
        }
    }
}

pub fn update_subsystem_radius_from_interface_count(
    changed_query: Query<(), Added<Interface>>,
    subsystem_query: Query<(
        Entity,
        &Subsystem,
        Option<&Children>,
        Option<&InterfaceSubsystem>,
        &Parent,
    )>,
    interface_query: Query<&Interface>,
    mut system_query: Query<&mut crate::bevy_app::components::System>,
    mut transform_query: Query<(&mut Transform, &mut InitialPosition)>,
    zoom: Res<Zoom>,
    mut remove_event_reader: EventReader<RemoveEvent>,
) {
    if changed_query.is_empty() && remove_event_reader.is_empty() {
        return;
    }

    remove_event_reader.clear();

    for (subsystem_entity, subsystem, children, interface_subsystem, parent) in &subsystem_query {
        let mut interface_count = 0;

        if interface_subsystem.is_some() {
            interface_count += 1;
        }

        for child in children.map(|c| &**c).unwrap_or_default() {
            if interface_query.get(*child).is_ok() {
                interface_count += 1;
            }
        }

        let mut scaling_factor = if interface_subsystem.is_some() {
            // Interface subsystems stay small (4%) regardless of interface count
            INTERFACE_SUBSYSTEM_SCALING_FACTOR
        } else {
            // Regular subsystems scale with interface count
            SUBSYSTEM_MIN_SCALING_FACTOR
                + interface_count as f32 * (SUBSYSTEM_SCALING_FACTOR - SUBSYSTEM_MIN_SCALING_FACTOR)
                    / SUBSYSTEM_FULL_SIZE_INTERFACE_COUNT
        };

        scaling_factor = scaling_factor.min(SUBSYSTEM_SCALING_FACTOR);

        let parent_radius = system_query
            .get(subsystem.parent_system)
            .expect("Parent system should exist")
            .radius;

        let radius = parent_radius * scaling_factor;

        system_query
            .get_mut(subsystem_entity)
            .expect("System should exist")
            .radius = radius;

        if interface_subsystem.is_some() {
            let (mut transform, mut initial_position) = transform_query
                .get_mut(subsystem_entity)
                .expect("Subsystem should have a Transform");

            // Only recalculate position if it hasn't been loaded/set already.
            // This preserves positions loaded from file (similar to FlowEndpointOffset fix).
            if initial_position.length_squared() < 0.001 {
                if interface_query.get(parent.get()).is_ok() {
                    initial_position.x = radius * transform.translation.x.signum();
                } else {
                    initial_position.x =
                        (parent_radius - radius) * transform.translation.x.signum();
                }

                transform.translation =
                    (**initial_position * **zoom).extend(transform.translation.z);
            }
        }
    }
}

pub fn update_interface_positions_from_system_radius(
    system_query: Query<
        (&crate::bevy_app::components::System, &Children),
        Changed<crate::bevy_app::components::System>,
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
