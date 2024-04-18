use crate::bundles::{despawn_create_button_with_component, spawn_create_button};
use crate::components::{
    CreateButton, CreateButtonType, Inflow, InflowSourceConnection, Outflow, OutflowSinkConnection,
};
use crate::resources::FocusedSystem;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub fn add_first_inflow_create_button(
    mut commands: Commands,
    changed_query: Query<Entity, Or<(Added<OutflowSinkConnection>, Changed<Outflow>)>>,
    query: Query<&Outflow, With<OutflowSinkConnection>>,
    inflow_query: Query<&Inflow>,
    button_query: Query<(Entity, &CreateButton)>,
    focused_system: Res<FocusedSystem>,
    asset_server: Res<AssetServer>,
) {
    // TODO : also detect removal

    if changed_query.is_empty() {
        return;
    }

    let mut system_to_outflow_usabilities = HashMap::new();

    for outflow in &query {
        system_to_outflow_usabilities
            .entry(outflow.system)
            .or_insert_with(HashSet::new)
            .insert(outflow.usability);
    }

    'outer: for (system_entity, outflow_usabilities) in system_to_outflow_usabilities {
        if outflow_usabilities.len() > 1 {
            for inflow in inflow_query.iter() {
                if inflow.system == system_entity {
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
                    system: **focused_system,
                },
                vec2(-128.0, 100.0),
                0.0,
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
    query: Query<(&Transform, &Inflow), Added<InflowSourceConnection>>,
    focused_system: Res<FocusedSystem>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, inflow)) = query.get_single() {
        let system_entity = inflow.system;

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Inflow,
                connection_source: system_entity,
                system: **focused_system,
            },
            vec2(-128.0, transform.translation.y - 70.0),
            0.0,
            &asset_server,
        );
    }
}
