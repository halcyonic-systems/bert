use crate::bundles::{
    despawn_create_button, despawn_create_button_with_component, spawn_create_button,
};
use crate::components::*;
use crate::data_model::Complexity;
use crate::events::RemoveEvent;
use crate::plugins::mouse_interaction::PickSelection;
use crate::resources::{FocusedSystem, Zoom};
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct FlowUsabilities {
    usabilities: HashSet<InteractionUsability>,
    has_inflow: bool,
    has_outflow: bool,
}

pub fn interface_subsystem_create_button_needs_update(
    changed_query: Query<
        (),
        Or<(
            Added<FlowStartConnection>,
            Added<FlowEndConnection>,
            Added<FlowStartInterfaceConnection>,
            Added<FlowEndInterfaceConnection>,
            Changed<Flow>,
            Changed<crate::components::System>,
        )>,
    >,
    focused_system: Res<FocusedSystem>,
    mut remove_event_reader: EventReader<RemoveEvent>,
) -> bool {
    let should_run =
        !changed_query.is_empty() || focused_system.is_changed() || !remove_event_reader.is_empty();

    remove_event_reader.clear();

    should_run
}

pub fn add_interface_subsystem_create_buttons(
    mut commands: Commands,
    incomplete_flow_query: Query<
        (),
        (
            With<Flow>,
            Or<(Without<FlowStartConnection>, Without<FlowEndConnection>)>,
        ),
    >,
    complete_flow_query: Query<(
        &Flow,
        &FlowStartConnection,
        &FlowEndConnection,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
    )>,
    interface_button_query: Query<&HasInterfaceSubsystemButton>,
    interface_subsystem_query: Query<&InterfaceSubsystemConnection>,
    button_query: Query<(&CreateButton, Option<&Parent>)>,
    subsystem_query: Query<(Entity, &Subsystem, Option<&InterfaceSubsystem>)>,
    system_query: Query<&crate::components::System>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    let incomplete_flows_exist = !incomplete_flow_query.is_empty();

    let is_atomic = matches!(
        system_query
            .get(**focused_system)
            .expect("System should exist")
            .complexity,
        Complexity::Atomic | Complexity::Multiset(_)
    );

    let mut flow_usabilities = HashMap::new();

    let mut systems_at_the_same_nesting_level = vec![];

    let is_root_system = if let Ok((_, subsystem, _)) = subsystem_query.get(**focused_system) {
        systems_at_the_same_nesting_level = subsystem_query
            .iter()
            .filter(|(_, subsys, _)| subsys.parent_system == subsystem.parent_system)
            .map(|(entity, _, conn)| (entity, conn))
            .collect();
        false
    } else {
        systems_at_the_same_nesting_level.push((**focused_system, None));
        true
    };

    for (system, _) in &systems_at_the_same_nesting_level {
        flow_usabilities.insert(*system, FlowUsabilities::default());
    }

    for (flow, flow_start_connection, flow_end_connection, _, _) in &complete_flow_query {
        if systems_at_the_same_nesting_level
            .iter()
            .any(|(entity, _)| *entity == flow_end_connection.target)
        {
            flow_usabilities
                .get_mut(&flow_end_connection.target)
                .map(|u| {
                    if flow.usability.is_import() {
                        u.usabilities.insert(flow.usability);
                    }
                    u.has_inflow = true;
                });
        }
        if systems_at_the_same_nesting_level
            .iter()
            .any(|(entity, _)| *entity == flow_start_connection.target)
        {
            flow_usabilities
                .get_mut(&flow_start_connection.target)
                .map(|u| {
                    if flow.usability.is_export() {
                        u.usabilities.insert(flow.usability);
                    }
                    u.has_outflow = true;
                });
        }
    }

    let mut system_interfaces = vec![];

    for (system_entity, interface_subsystem) in &systems_at_the_same_nesting_level {
        if let Some(interface_subsystem) = interface_subsystem {
            let interface_entity = interface_subsystem.interface_entity;
            let mut interface_type = InterfaceType::Export;

            for (flow, _, _, flow_start_interface_connection, flow_end_interface_connection) in
                &complete_flow_query
            {
                if let Some(connection) = flow_start_interface_connection {
                    if connection.target == interface_entity {
                        flow_usabilities.get_mut(system_entity).map(|u| {
                            u.usabilities.insert(flow.usability);
                            u.has_outflow = true;
                        });

                        interface_type = InterfaceType::Export;
                        // TODO : hybrid
                    }
                }
                if let Some(connection) = flow_end_interface_connection {
                    if connection.target == interface_entity {
                        flow_usabilities.get_mut(system_entity).map(|u| {
                            u.usabilities.insert(flow.usability);
                            u.has_inflow = true;
                        });
                        interface_type = InterfaceType::Import;
                        // TODO : hybrid
                    }
                }
            }

            if *system_entity == **focused_system {
                let mut has_subsystem = false;

                for (_, subsystem, interface_subsystem) in &subsystem_query {
                    if subsystem.parent_system == *system_entity {
                        if let Some(interface_subsystem) = interface_subsystem {
                            if interface_subsystem.interface_entity == interface_entity {
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

    let mut show_interface_buttons = if incomplete_flows_exist || is_atomic {
        false
    } else {
        let mut show_interface_buttons = true;

        for flow_usabilities in flow_usabilities.values() {
            let mut inflow_present = false;
            let mut outflow_present = false;

            if is_root_system {
                for usability in &flow_usabilities.usabilities {
                    if matches!(
                        usability,
                        InteractionUsability::Resource | InteractionUsability::Disruption
                    ) {
                        inflow_present = true;
                    } else if matches!(
                        usability,
                        InteractionUsability::Product | InteractionUsability::Waste
                    ) {
                        outflow_present = true;
                    }
                }
            } else {
                inflow_present = flow_usabilities.has_inflow;
                outflow_present = flow_usabilities.has_outflow;
            }

            if !inflow_present || !outflow_present {
                show_interface_buttons = false;
                break;
            }
        }

        show_interface_buttons
    };

    for (
        _,
        flow_start_connection,
        flow_end_connection,
        flow_start_interface_connection,
        flow_end_interface_connection,
    ) in &complete_flow_query
    {
        let interface_entity = if flow_end_connection.target == **focused_system {
            if let Some(conn) = flow_end_interface_connection {
                Some((conn.target, InterfaceType::Import))
            } else {
                // incomplete flow (missing interface)
                show_interface_buttons = false;
                break;
            }
        } else if flow_start_connection.target == **focused_system {
            if let Some(conn) = flow_start_interface_connection {
                Some((conn.target, InterfaceType::Export))
            } else {
                // incomplete flow (missing interface)
                show_interface_buttons = false;
                break;
            }
        } else {
            None
        };

        if let Some((interface_entity, interface_type)) = interface_entity {
            if interface_subsystem_query.get(interface_entity).is_err() {
                system_interfaces.push((interface_entity, true, interface_type));
            }
        }
    }

    if show_interface_buttons {
        for (interface_entity, is_child_of_interface, interface_type) in system_interfaces {
            let interface_button = interface_button_query.get(interface_entity);

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
                    Vec2::ZERO,
                    0.0,
                    **zoom,
                    Some(interface_entity),
                    &asset_server,
                );
            }
        }
    } else {
        for interface_button in &interface_button_query {
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

        if let Some(parent_system_entity) = current_parent {
            spawn_create_button(
                &mut commands,
                CreateButton {
                    ty: CreateButtonType::Subsystem,
                    connection_source: parent_system_entity,
                    system: parent_system_entity,
                    substance_type: None,
                },
                center.truncate() / **zoom,
                0.0,
                **zoom,
                current_parent,
                &asset_server,
            );
        }
    }
}
