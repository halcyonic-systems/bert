use crate::bundles::{
    despawn_create_button, despawn_create_button_with_component, spawn_create_button,
    spawn_external_entity, spawn_inflow, spawn_interface, spawn_interface_subsystem, spawn_outflow,
};
use crate::components::*;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_mod_picking::events::{Click, Pointer};
use bevy_mod_picking::prelude::Listener;

pub fn add_interface_create_button(
    mut commands: Commands,
    query: Query<
        (Entity, &Transform),
        (
            Or<(With<Outflow>, With<Inflow>)>,
            Without<FlowInterfaceConnection>,
            Without<FlowOtherEndConnection>,
            Without<FlowInterfaceButton>,
        ),
    >,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform) in &query {
        let direction = transform.right().truncate();

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Interface,
                connection_source: entity,
            },
            transform.translation.truncate() - direction * 64.0,
            &asset_server,
        );
    }
}

pub fn add_external_entity_create_button(
    mut commands: Commands,
    query: Query<
        (Entity, &Transform),
        (
            // Or<(With<Outflow>, With<Inflow>)>,
            With<FlowInterfaceConnection>,
            Without<FlowOtherEndConnection>,
            Without<FlowOtherEndButton>,
        ),
    >,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform) in &query {
        let direction = transform.right().truncate();

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::ExternalEntity,
                connection_source: entity,
            },
            transform.translation.truncate() + direction * 64.0,
            &asset_server,
        );
    }
}

pub fn add_outflow_create_button(
    mut commands: Commands,
    query: Query<
        (&Transform, &FlowSystemConnection),
        (Added<FlowOtherEndConnection>, With<Outflow>),
    >,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, system_connection)) = query.get_single() {
        let system_entity = system_connection.target;

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Outflow,
                connection_source: system_entity,
            },
            vec2(128.0, transform.translation.y - 70.0),
            &asset_server,
        );
    }
}

pub fn add_first_inflow_create_button(
    mut commands: Commands,
    changed_query: Query<Entity, Or<(Added<FlowOtherEndConnection>, Changed<Outflow>)>>,
    query: Query<(&FlowSystemConnection, &Outflow), With<FlowOtherEndConnection>>,
    inflow_query: Query<&FlowSystemConnection, With<Inflow>>,
    button_query: Query<(Entity, &CreateButton)>,
    asset_server: Res<AssetServer>,
) {
    // TODO : also detect removal

    if changed_query.is_empty() {
        return;
    }

    let mut system_to_outflow_usabilities = HashMap::new();

    for (system_connection, outflow) in &query {
        system_to_outflow_usabilities
            .entry(system_connection.target)
            .or_insert_with(HashSet::new)
            .insert(outflow.usability);
    }

    'outer: for (system_entity, outflow_usabilities) in system_to_outflow_usabilities {
        if outflow_usabilities.len() > 1 {
            for inflow_connection in inflow_query.iter() {
                if inflow_connection.target == system_entity {
                    continue 'outer;
                }
            }

            for (_, button) in button_query.iter() {
                if matches!(button.ty, CreateButtonType::Inflow)
                    && button.connection_source == system_entity
                {
                    continue 'outer;
                }
            }

            spawn_create_button(
                &mut commands,
                CreateButton {
                    ty: CreateButtonType::Inflow,
                    connection_source: system_entity,
                },
                vec2(-128.0, 100.0),
                &asset_server,
            );
        } else {
            for (entity, button) in &button_query {
                if matches!(button.ty, CreateButtonType::Inflow)
                    && button.connection_source == system_entity
                {
                    despawn_create_button_with_component(&mut commands, entity, button)
                }
            }
        }
    }
}

pub fn add_consecutive_inflow_create_button(
    mut commands: Commands,
    query: Query<
        (&Transform, &FlowSystemConnection),
        (Added<FlowOtherEndConnection>, With<Inflow>),
    >,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, system_connection)) = query.get_single() {
        let system_entity = system_connection.target;

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Inflow,
                connection_source: system_entity,
            },
            vec2(-128.0, transform.translation.y - 70.0),
            &asset_server,
        );
    }
}

pub fn add_interface_subsystem_create_buttons(
    mut commands: Commands,
    changed_query: Query<
        Entity,
        Or<(
            Added<FlowOtherEndConnection>,
            Changed<Outflow>,
            Changed<Inflow>,
        )>,
    >,
    flow_query: Query<
        (&FlowSystemConnection, Option<&Outflow>, Option<&Inflow>),
        With<FlowOtherEndConnection>,
    >,
    flow_interface_query: Query<
        (&FlowSystemConnection, &FlowInterfaceConnection),
        With<FlowOtherEndConnection>,
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
    asset_server: Res<AssetServer>,
) {
    // TODO : also detect removal

    if changed_query.is_empty() {
        return;
    }

    let mut system_to_flow_usabilities = HashMap::new();

    for (system_connection, outflow, inflow) in &flow_query {
        let system_entity = system_connection.target;

        match (inflow, outflow) {
            (Some(inflow), None) => {
                system_to_flow_usabilities
                    .entry(system_entity)
                    .or_insert_with(HashSet::new)
                    .insert(GeneralUsability::Inflow(inflow.usability));
            }
            (None, Some(outflow)) => {
                system_to_flow_usabilities
                    .entry(system_entity)
                    .or_insert_with(HashSet::new)
                    .insert(GeneralUsability::Outflow(outflow.usability));
            }
            _ => unreachable!("Outflow and inflow can't both be None"),
        }
    }

    for (system_connection, interface_connection) in &flow_interface_query {
        let flow_usabilities = system_to_flow_usabilities
            .get(&system_connection.target)
            .expect("We just added this above");
        {
            if flow_usabilities.len() > 3 {
                let interface_entity = interface_connection.target;
                if let Ok(transform) = interface_query.get(interface_entity) {
                    spawn_create_button(
                        &mut commands,
                        CreateButton {
                            ty: CreateButtonType::InterfaceSubsystem,
                            connection_source: interface_entity,
                        },
                        transform.translation.truncate(),
                        &asset_server,
                    );
                }
            } else {
                if let Ok(interface_button) =
                    interface_button_query.get(interface_connection.target)
                {
                    despawn_create_button(
                        &mut commands,
                        interface_button.button_entity,
                        &button_query,
                    );
                }
            }
        }
    }
}

pub fn on_create_button_click(
    mut commands: Commands,
    event: Listener<Pointer<Click>>,
    button_query: Query<(&CreateButton, &GlobalTransform)>,
    only_button_query: Query<&CreateButton>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (button, transform) = button_query
        .get(event.target)
        .expect("After on click this has to exist");

    match button.ty {
        CreateButtonType::Interface => spawn_interface(
            &mut commands,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::Inflow => spawn_inflow(
            &mut commands,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::Outflow => spawn_outflow(
            &mut commands,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::ExternalEntity => spawn_external_entity(
            &mut commands,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::InterfaceSubsystem => spawn_interface_subsystem(
            &mut commands,
            button.connection_source,
            &mut meshes,
            &mut materials,
        ),
    }

    despawn_create_button(&mut commands, event.target, &only_button_query);
}
