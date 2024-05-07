use crate::bundles::{despawn_create_button_with_component, spawn_create_button};
use crate::components::*;
use crate::resources::{FocusedSystem, Zoom};
use crate::systems::next_inflow_button_transform;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub fn add_first_inflow_create_button(
    mut commands: Commands,
    changed_query: Query<Entity, Or<(Added<FlowEndConnection>, Changed<Flow>)>>,
    outflow_query: Query<(&Flow, &FlowStartConnection), With<FlowEndConnection>>,
    inflow_query: Query<&FlowEndConnection>,
    button_query: Query<(Entity, &CreateButton)>,
    flow_interface_query: Query<(&FlowEndConnection, &FlowEndInterfaceConnection)>,
    transform_query: Query<&Transform>,
    system_query: Query<&crate::components::System>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    // TODO : also detect removal

    if changed_query.is_empty() {
        return;
    }

    let mut system_to_outflow_usabilities = HashMap::new();

    for (outflow, flow_start_connection) in &outflow_query {
        if matches!(flow_start_connection.target_type, StartTargetType::System) {
            system_to_outflow_usabilities
                .entry(flow_start_connection.target)
                .or_insert_with(HashSet::new)
                .insert(OutflowUsability::from_useful(outflow.is_useful));
        }
    }

    'outer: for (system_entity, outflow_usabilities) in system_to_outflow_usabilities {
        if outflow_usabilities.len() > 1 {
            for inflow in inflow_query.iter() {
                if inflow.target == system_entity {
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

            let (position, angle) = next_inflow_button_transform(
                &flow_interface_query,
                &transform_query,
                &system_query,
                **focused_system,
            );

            spawn_create_button(
                &mut commands,
                CreateButton {
                    ty: CreateButtonType::Inflow,
                    connection_source: system_entity,
                    system: **focused_system,
                    substance_type: None,
                },
                position,
                angle,
                **zoom,
                Some(**focused_system),
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
    inflow_query: Query<&FlowEndConnection, Added<FlowStartConnection>>,
    flow_interface_query: Query<(&FlowEndConnection, &FlowEndInterfaceConnection)>,
    transform_query: Query<&Transform>,
    system_query: Query<&crate::components::System>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(inflow_connection) = inflow_query.get_single() {
        let system_entity = inflow_connection.target;

        let (position, angle) = next_inflow_button_transform(
            &flow_interface_query,
            &transform_query,
            &system_query,
            **focused_system,
        );

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Inflow,
                connection_source: system_entity,
                system: **focused_system,
                substance_type: None,
            },
            position,
            angle,
            **zoom,
            Some(**focused_system),
            &asset_server,
        );
    }
}
