use crate::bundles::{despawn_create_button, spawn_create_button};
use crate::components::{
    CreateButton, CreateButtonType, GeneralUsability, Inflow, InflowInterfaceConnection,
    InflowSourceConnection, InterfaceSubsystemButton, InterfaceSubsystemConnection, Outflow,
    OutflowInterfaceConnection, OutflowSinkConnection,
};
use crate::resources::FocusedSystem;
use bevy::asset::AssetServer;
use bevy::prelude::{Added, Changed, Commands, Entity, Or, Query, Res, Transform, With, Without};
use bevy::utils::HashSet;

pub fn add_interface_subsystem_create_buttons(
    mut commands: Commands,
    changed_query: Query<
        Entity,
        Or<(
            Added<InflowSourceConnection>,
            Added<OutflowSinkConnection>,
            Changed<Inflow>,
            Changed<Outflow>,
        )>,
    >,
    flow_query: Query<
        (Option<&Outflow>, Option<&Inflow>),
        (
            Or<(With<InflowSourceConnection>, With<OutflowSinkConnection>)>,
            Or<(With<Inflow>, With<Outflow>)>,
        ),
    >,
    flow_interface_query: Query<
        (
            Option<&Inflow>,
            Option<&Outflow>,
            Option<&InflowInterfaceConnection>,
            Option<&OutflowInterfaceConnection>,
        ),
        Or<(With<InflowSourceConnection>, With<OutflowSinkConnection>)>,
    >,
    interface_query: Query<
        &Transform,
        (
            Without<InterfaceSubsystemButton>,
            Without<InterfaceSubsystemConnection>,
        ),
    >,
    interface_button_query: Query<&InterfaceSubsystemButton>,
    button_query: Query<&CreateButton>,
    focused_system: Res<FocusedSystem>,
    asset_server: Res<AssetServer>,
) {
    // TODO : also detect removal

    if changed_query.is_empty() {
        return;
    }

    let mut flow_usabilities = HashSet::new();

    for (outflow, inflow) in &flow_query {
        match (inflow, outflow) {
            (Some(inflow), None) => {
                if inflow.system != **focused_system {
                    continue;
                }
                flow_usabilities.insert(GeneralUsability::Inflow(inflow.usability));
            }
            (None, Some(outflow)) => {
                if outflow.system != **focused_system {
                    continue;
                }
                flow_usabilities.insert(GeneralUsability::Outflow(outflow.usability));
            }
            (Some(inflow), Some(outflow)) => {
                if inflow.system == **focused_system {
                    flow_usabilities.insert(GeneralUsability::Inflow(inflow.usability));
                } else if outflow.system == **focused_system {
                    flow_usabilities.insert(GeneralUsability::Outflow(outflow.usability));
                }
            }
            _ => unreachable!("Outflow and inflow can't both be None"),
        }
    }

    for (inflow, outflow, inflow_interface_connection, outflow_interface_connection) in
        &flow_interface_query
    {
        let interface_entity = match (inflow, outflow) {
            (Some(inflow), None) => {
                if inflow.system != **focused_system {
                    continue;
                }

                inflow_interface_connection
                    .expect("Should be there because we have an Inflow")
                    .target
            }
            (None, Some(outflow)) => {
                if outflow.system != **focused_system {
                    continue;
                }

                outflow_interface_connection
                    .expect("Should be there because we have an Outflow")
                    .target
            }
            (Some(inflow), Some(outflow)) => {
                if inflow.system == **focused_system {
                    inflow_interface_connection
                        .expect("Should be there because we have an Inflow")
                        .target
                } else if outflow.system == **focused_system {
                    outflow_interface_connection
                        .expect("Should be there because we have an Outflow")
                        .target
                } else {
                    continue;
                }
            }
            _ => unreachable!("Outflow and inflow can't both be None"),
        };

        if flow_usabilities.len() > 3 {
            if let Ok(transform) = interface_query.get(interface_entity) {
                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: CreateButtonType::InterfaceSubsystem,
                        connection_source: interface_entity,
                        system: **focused_system,
                    },
                    transform.translation.truncate(),
                    0.0,
                    &asset_server,
                );
            }
        } else {
            if let Ok(interface_button) = interface_button_query.get(interface_entity) {
                despawn_create_button(&mut commands, interface_button.button_entity, &button_query);
            }
        }
    }
}
