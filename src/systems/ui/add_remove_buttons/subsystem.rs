use crate::bundles::{
    despawn_create_button, despawn_create_button_with_component, spawn_create_button,
};
use crate::components::*;
use crate::constants::INTERFACE_WIDTH_HALF;
use crate::plugins::mouse_interaction::PickSelection;
use crate::resources::{FocusedSystem, Zoom};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub fn add_interface_subsystem_create_buttons(
    mut commands: Commands,
    changed_query: Query<
        Entity,
        Or<(
            Added<FlowStartConnection>,
            Added<FlowEndConnection>,
            Added<FlowStartInterfaceConnection>,
            Added<FlowEndInterfaceConnection>,
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
    button_query: Query<(&CreateButton, Option<&Parent>)>,
    subsystem_query: Query<(Entity, &Subsystem, Option<&SubsystemParentFlowConnection>)>,
    parent_query: Query<&Parent>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    if changed_query.is_empty() && !focused_system.is_changed() {
        return;
    }

    let incomplete_flows_exist = !incomplete_flow_query.is_empty();

    let mut flow_usabilities = HashMap::new();

    let mut systems_at_the_same_nesting_level = vec![];

    if let Ok((_, subsystem, _)) = subsystem_query.get(**focused_system) {
        systems_at_the_same_nesting_level = subsystem_query
            .iter()
            .filter(|(_, subsys, _)| subsys.parent_system == subsystem.parent_system)
            .map(|(entity, _, conn)| (entity, conn))
            .collect();
    } else {
        systems_at_the_same_nesting_level.push((**focused_system, None));
    }

    for (system, _) in &systems_at_the_same_nesting_level {
        flow_usabilities.insert(*system, HashSet::new());
    }

    for (flow, flow_start_connection, flow_end_connection) in &complete_flow_query {
        if systems_at_the_same_nesting_level
            .iter()
            .any(|(entity, _)| *entity == flow_end_connection.target)
        {
            flow_usabilities
                .get_mut(&flow_end_connection.target)
                .map(|set| {
                    set.insert(GeneralUsability::Inflow(InflowUsability::from_useful(
                        flow.is_useful,
                    )))
                });
        }
        if systems_at_the_same_nesting_level
            .iter()
            .any(|(entity, _)| *entity == flow_start_connection.target)
        {
            flow_usabilities
                .get_mut(&flow_start_connection.target)
                .map(|set| {
                    set.insert(GeneralUsability::Outflow(OutflowUsability::from_useful(
                        flow.is_useful,
                    )))
                });
        }
    }

    let mut system_interfaces = vec![];

    for (system_entity, parent_flow_connection) in &systems_at_the_same_nesting_level {
        if let Some(parent_flow_connection) = parent_flow_connection {
            let (_, _, flow_start_interface_connection, flow_end_interface_connection) =
                flow_interface_query
                    .get(parent_flow_connection.target)
                    .expect("Flow should exist");
            let (flow, _, _) = complete_flow_query
                .get(parent_flow_connection.target)
                .expect("Flow should exist");

            let mut interface_entity = Entity::PLACEHOLDER;
            let mut interface_type = InterfaceType::Export;

            let mut parent_entity = *system_entity;
            while let Ok(parent) = parent_query.get(parent_entity) {
                parent_entity = parent.get();

                if let Some(connection) = flow_start_interface_connection {
                    if connection.target == parent_entity {
                        interface_entity = parent_entity;
                        flow_usabilities.get_mut(system_entity).map(|set| {
                            set.insert(GeneralUsability::Outflow(OutflowUsability::from_useful(
                                flow.is_useful,
                            )))
                        });
                        interface_type = InterfaceType::Export;
                        break;
                    }
                } else if let Some(connection) = flow_end_interface_connection {
                    if connection.target == parent_entity {
                        interface_entity = parent_entity;
                        flow_usabilities.get_mut(system_entity).map(|set| {
                            set.insert(GeneralUsability::Inflow(InflowUsability::from_useful(
                                flow.is_useful,
                            )))
                        });
                        interface_type = InterfaceType::Import;
                        break;
                    }
                }
            }

            if *system_entity == **focused_system {
                let mut has_subsystem = false;

                for (_, subsystem, flow_connection) in &subsystem_query {
                    if subsystem.parent_system == *system_entity {
                        if let Some(flow_connection) = flow_connection {
                            if flow_connection.target == parent_flow_connection.target {
                                has_subsystem = true;
                                break;
                            }
                        }
                    }
                }

                if !has_subsystem {
                    system_interfaces.push((interface_entity, false, interface_type));
                }
            }
        }
    }

    let show_interface_buttons = if incomplete_flows_exist {
        false
    } else {
        let mut show_interface_buttons = true;

        for flow_usabilities in flow_usabilities.values() {
            let mut inflow_present = false;
            let mut outflow_present = false;

            for usability in flow_usabilities {
                if matches!(usability, GeneralUsability::Inflow(_)) {
                    inflow_present = true;
                } else if matches!(usability, GeneralUsability::Outflow(_)) {
                    outflow_present = true;
                }
            }

            if !inflow_present || !outflow_present {
                show_interface_buttons = false;
                break;
            }
        }

        show_interface_buttons
    };

    for (
        flow_start_connection,
        flow_end_connection,
        flow_start_interface_connection,
        flow_end_interface_connection,
    ) in &flow_interface_query
    {
        let interface_entity = if flow_end_connection.target == **focused_system {
            flow_end_interface_connection.map(|c| (c.target, InterfaceType::Import))
        } else if flow_start_connection.target == **focused_system {
            flow_start_interface_connection.map(|c| (c.target, InterfaceType::Export))
        } else {
            None
        };

        if let Some((interface_entity, interface_type)) = interface_entity {
            if interface_subsystem_query.get(interface_entity).is_err() {
                system_interfaces.push((interface_entity, true, interface_type));
            }
        }
    }

    for (interface_entity, is_child_of_interface, interface_type) in system_interfaces {
        let interface_button = interface_button_query.get(interface_entity);

        if show_interface_buttons {
            if interface_button.is_err() {
                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: CreateButtonType::InterfaceSubsystem {
                            is_child_of_interface,
                            interface_type,
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

pub fn add_subsystem_from_external_entities_create_button(
    mut commands: Commands,
    external_entity_query: Query<(&PickSelection, &Transform, &Parent), With<ExternalEntity>>,
    selection_changed_query: Query<(), (With<ExternalEntity>, Changed<PickSelection>)>,
    button_query: Query<(Entity, &CreateButton, &Parent)>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    if selection_changed_query.is_empty() {
        return;
    }

    for (entity, button, parent) in &button_query {
        if matches!(button.ty, CreateButtonType::Subsystem) {
            despawn_create_button_with_component(&mut commands, entity, button, Some(parent));
            break;
        }
    }

    let mut selected_count = 0;
    let mut center = Vec3::ZERO;
    let mut current_parent = None;

    for (pick_selection, transform, parent) in &external_entity_query {
        if pick_selection.is_selected {
            if let Some(current_parent) = current_parent {
                if parent.get() != current_parent {
                    return;
                }
            }
            selected_count += 1;
            center += transform.translation;
            current_parent = Some(parent.get());
        }
    }

    if selected_count > 1 {
        center /= selected_count as f32;

        let parent_system_entity = current_parent.expect("No parent");

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Subsystem,
                connection_source: parent_system_entity,
                system: parent_system_entity,
                substance_type: None,
            },
            center.truncate(),
            0.0,
            **zoom,
            current_parent,
            &asset_server,
        );
    }
}
