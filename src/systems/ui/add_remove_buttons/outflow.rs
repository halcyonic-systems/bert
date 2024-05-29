use crate::bundles::{despawn_create_button_with_component, spawn_create_button};
use crate::components::{System, *};
use crate::events::RemoveEvent;
use crate::resources::{FocusedSystem, Zoom};
use crate::systems::next_outflow_button_transform;
use bevy::prelude::*;

pub fn outflow_create_button_needs_update(
    flow_finished_query: Query<
        (),
        Or<(
            Added<FlowEndConnection>,
            Added<FlowStartConnection>,
            Added<FlowStartInterfaceConnection>,
            Added<FlowEndInterfaceConnection>,
        )>,
    >,
    focused_system: Res<FocusedSystem>,
    mut remove_event_reader: EventReader<RemoveEvent>,
) -> bool {
    let needs_update = if !focused_system.is_changed()
        && flow_finished_query.get_single().is_err()
        && remove_event_reader.is_empty()
    {
        false
    } else {
        true
    };
    remove_event_reader.clear();

    needs_update
}

pub fn add_outflow_create_button(
    mut commands: Commands,
    button_query: Query<(Entity, &CreateButton, Option<&Parent>)>,
    incomplete_outflow_query: Query<
        &FlowStartConnection,
        Or<(
            Without<FlowEndConnection>,
            Without<FlowStartInterfaceConnection>,
        )>,
    >,
    incomplete_inflow_query: Query<
        &FlowEndConnection,
        Or<(
            Without<FlowStartConnection>,
            Without<FlowEndInterfaceConnection>,
        )>,
    >,
    flow_interface_query: Query<(&FlowStartConnection, &FlowStartInterfaceConnection)>,
    export_subsystem_query: Query<&InterfaceSubsystem, With<ExportSubsystem>>,
    transform_query: Query<&Transform>,
    system_query: Query<&System>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    asset_server: Res<AssetServer>,
) {
    let focused_system = **focused_system;

    let mut is_export_subsystem = true;

    let is_completed_export_subsystem =
        if let Ok(interface_subsystem) = export_subsystem_query.get(focused_system) {
            interface_subsystem.total_outflow <= interface_subsystem.total_inflow
        } else {
            is_export_subsystem = false;
            false
        };

    if is_export_subsystem && !is_completed_export_subsystem {
        return;
    }

    if !despawn_existing_buttons(
        &mut commands,
        focused_system,
        CreateButtonType::Outflow,
        &button_query,
        &incomplete_inflow_query.iter().collect::<Vec<_>>(),
        &incomplete_outflow_query.iter().collect::<Vec<_>>(),
    ) {
        return;
    }

    let (position, angle) = next_outflow_button_transform(
        &flow_interface_query,
        &transform_query,
        &system_query,
        focused_system,
        is_completed_export_subsystem,
    );

    spawn_create_button(
        &mut commands,
        CreateButton {
            ty: CreateButtonType::Outflow,
            connection_source: focused_system,
            system: focused_system,
            substance_type: None,
        },
        position,
        angle,
        **zoom,
        Some(focused_system),
        &asset_server,
    );
}

pub fn despawn_existing_buttons(
    mut commands: &mut Commands,
    focused_system: Entity,
    button_type: CreateButtonType,
    button_query: &Query<(Entity, &CreateButton, Option<&Parent>)>,
    incomplete_inflow_connections: &[&FlowEndConnection],
    incomplete_outflow_connections: &[&FlowStartConnection],
) -> bool {
    let mut button_entities = vec![];

    for (button_entity, button, parent) in button_query {
        if button.system == focused_system && button.ty == button_type {
            button_entities.push((button_entity, button, parent));
        }
    }

    for target in incomplete_inflow_connections
        .into_iter()
        .map(|c| c.target)
        .chain(incomplete_outflow_connections.into_iter().map(|c| c.target))
    {
        if target == focused_system {
            for (entity, button, parent) in button_entities {
                despawn_create_button_with_component(&mut commands, entity, button, parent)
            }

            return false;
        }
    }

    button_entities.is_empty()
}
