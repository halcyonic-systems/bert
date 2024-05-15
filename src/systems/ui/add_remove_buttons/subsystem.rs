use crate::bundles::{despawn_create_button, spawn_create_button};
use crate::components::*;
use crate::constants::INTERFACE_WIDTH_HALF;
use crate::resources::{FocusedSystem, Zoom};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub fn add_interface_subsystem_create_buttons(
    mut commands: Commands,
    changed_query: Query<
        Entity,
        Or<(
            Added<FlowStartConnection>,
            Added<FlowEndConnection>,
            Changed<Flow>,
        )>,
    >,
    incomplete_flow_query: Query<
        (),
        (
            With<Flow>,
            Or<(Without<FlowStartConnection>, Without<FlowEndConnection>)>,
        ),
    >,
    complete_flow_query: Query<(&Flow, &FlowStartConnection, &FlowEndConnection)>,
    flow_interface_query: Query<(
        &FlowStartConnection,
        &FlowEndConnection,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
    )>,
    interface_button_query: Query<&HasInterfaceSubsystemButton>,
    interface_subsystem_query: Query<&InterfaceSubsystemConnection>,
    button_query: Query<&CreateButton>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    // TODO : also detect removal

    if changed_query.is_empty() {
        return;
    }

    let incomplete_flows_exist = !incomplete_flow_query.is_empty();

    let mut flow_usabilities = HashSet::new();

    for (flow, flow_start_connection, flow_end_connection) in &complete_flow_query {
        if flow_end_connection.target == **focused_system {
            flow_usabilities.insert(GeneralUsability::Inflow(InflowUsability::from_useful(
                flow.is_useful,
            )));
        } else if flow_start_connection.target == **focused_system {
            flow_usabilities.insert(GeneralUsability::Outflow(OutflowUsability::from_useful(
                flow.is_useful,
            )));
        }
    }

    for (
        flow_start_connection,
        flow_end_connection,
        flow_start_interface_connection,
        flow_end_interface_connection,
    ) in &flow_interface_query
    {
        let interface_entity = if flow_end_connection.target == **focused_system {
            flow_end_interface_connection
                .expect("Should be there because we have an Inflow")
                .target
        } else if flow_start_connection.target == **focused_system {
            flow_start_interface_connection
                .expect("Should be there because we have an Outflow")
                .target
        } else {
            continue;
        };

        let interface_button = interface_button_query.get(interface_entity);

        let usability_conditions = [
            GeneralUsability::Inflow(InflowUsability::from_useful(true)),
            GeneralUsability::Outflow(OutflowUsability::from_useful(true)),
        ];
        let usability_conditions_met = usability_conditions.iter().all(|condition| {
            flow_usabilities.contains(condition)
        });
        if usability_conditions_met && !incomplete_flows_exist {
            if interface_button.is_err() && interface_subsystem_query.get(interface_entity).is_err()
            {
                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: CreateButtonType::InterfaceSubsystem {
                            is_child_of_interface: true, // TODO : compute this
                        },
                        connection_source: interface_entity,
                        system: **focused_system,
                        substance_type: None,
                    },
                    vec2(-INTERFACE_WIDTH_HALF, 0.0),
                    0.0,
                    **zoom,
                    Some(interface_entity),
                    &asset_server,
                );
            }
        } else if let Ok(interface_button) = interface_button {
            despawn_create_button(&mut commands, interface_button.button_entity, &button_query);
        }
    }
}
