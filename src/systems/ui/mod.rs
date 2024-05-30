mod add_remove_buttons;
mod color;
mod drag;
mod flow;
mod label;
mod pin;
mod selected_helper;
mod zoom;

use crate::plugins::mouse_interaction::{do_deselect_all, PickSelection};
pub use add_remove_buttons::*;
pub use color::*;
pub use drag::*;
pub use flow::*;
// pub use pin::*;
pub use label::*;
pub use selected_helper::*;
pub use zoom::*;

use crate::bundles::{
    despawn_create_button, despawn_create_button_with_component, spawn_external_entity,
    spawn_inflow, spawn_interface, spawn_interface_subsystem, spawn_outflow, spawn_subsystem,
};
use crate::components::*;
use crate::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator, Zoom,
};
use crate::states::AppState;
use crate::utils::combined_transform_of_entity_until_ancestor;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use rust_decimal_macros::dec;

pub fn change_focused_system(
    selected_query: Query<
        (Entity, &PickSelection, Option<&Subsystem>),
        (Changed<PickSelection>, With<crate::components::System>),
    >,
    button_query: Query<&CreateButton>,
    mut focused_system: ResMut<FocusedSystem>,
) {
    for (entity, selection, subsystem) in &selected_query {
        if selection.is_selected {
            if let Some(subsystem) = subsystem {
                for button in &button_query {
                    if matches!(button.ty, CreateButtonType::InterfaceSubsystem { .. })
                        && button.system == subsystem.parent_system
                    {
                        // Do not allow selecting an interface subsystem while not all interface subsystems are created for the parent
                        return;
                    }
                }
            }

            **focused_system = entity;
        }
    }
}

pub fn remove_unfocused_system_buttons(
    mut commands: Commands,
    focused_system: Res<FocusedSystem>,
    mut previous_focused_system: Local<Option<Entity>>,
    button_query: Query<(Entity, &CreateButton, Option<&Parent>)>,
) {
    if !focused_system.is_changed() || Some(**focused_system) == *previous_focused_system {
        return;
    }

    let focused_system = **focused_system;
    *previous_focused_system = Some(focused_system);

    for (entity, button, parent) in &button_query {
        if button.system != focused_system {
            despawn_create_button_with_component(&mut commands, entity, button, parent);
        }
    }
}

pub fn on_subsystem_button_click(
    mut commands: Commands,
    mut event: ListenerMut<Pointer<Click>>,
    transform_query: Query<&Transform>,
    only_button_query: Query<(&CreateButton, Option<&Parent>)>,
    external_entity_query: Query<(Entity, &PickSelection, &Parent), With<ExternalEntity>>,
    flow_connection_query: Query<(Entity, &FlowStartConnection, &FlowEndConnection)>,
    flow_query: Query<(&FlowCurve, &Flow)>,
    system_query: Query<(&Transform, &crate::components::System)>,
    nesting_level_query: Query<&NestingLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
) {
    event.stop_propagation();

    let mut inflows = vec![];
    let mut outflows = vec![];

    let mut parent_system = Entity::PLACEHOLDER;

    for (external_entity, selection, parent) in &external_entity_query {
        if selection.is_selected {
            parent_system = parent.get();

            for (flow_entity, start_connection, end_connection) in &flow_connection_query {
                if start_connection.target == external_entity {
                    outflows.push(flow_entity);
                } else if end_connection.target == external_entity {
                    inflows.push(flow_entity);
                }
            }

            commands.entity(external_entity).despawn_recursive();
            commands
                .entity(parent_system)
                .remove_children(&[external_entity]);
        }
    }

    let transform = transform_query
        .get(event.target)
        .expect("After on click this has to exist");

    spawn_subsystem(
        &mut commands,
        parent_system,
        &system_query,
        &nesting_level_query,
        &flow_query,
        &inflows,
        &outflows,
        &mut fixed_system_element_geometries,
        &mut meshes,
        &mut stroke_tess,
        **zoom,
        "Subsystem",
        "",
        transform.translation.truncate(),
    );

    despawn_create_button(&mut commands, event.target, &only_button_query);
}

pub fn on_flow_terminal_button_click(
    mut commands: Commands,
    mut event: ListenerMut<Pointer<Click>>,
    only_button_query: Query<(&CreateButton, Option<&Parent>)>,
    mut pick_selection_query: Query<&mut PickSelection>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    event.stop_propagation();

    do_deselect_all(&mut pick_selection_query);

    let (button, _) = only_button_query
        .get(event.target)
        .expect("After on click this has to exist");

    match button.ty {
        CreateButtonType::FlowTerminalStart => {
            commands
                .entity(button.connection_source)
                .insert(FlowTerminalSelecting::Start);
            next_state.set(AppState::FlowTerminalSelection);
        }
        CreateButtonType::FlowTerminalEnd => {
            commands
                .entity(button.connection_source)
                .insert(FlowTerminalSelecting::End);
            next_state.set(AppState::FlowTerminalSelection);
        }
        _ => unreachable!("The other types are handled in other event listeners"),
    }

    despawn_create_button(&mut commands, event.target, &only_button_query);
}

pub fn on_external_entity_create_button_click(
    mut commands: Commands,
    mut event: ListenerMut<Pointer<Click>>,
    button_query: Query<(&CreateButton, &Transform)>,
    only_button_query: Query<(&CreateButton, Option<&Parent>)>,
    mut pick_selection_query: Query<&mut PickSelection>,
    subsystem_query: Query<&Subsystem>,
    nesting_query: Query<&NestingLevel>,
    focused_system: Res<FocusedSystem>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
) {
    event.stop_propagation();

    do_deselect_all(&mut pick_selection_query);

    let (button, transform) = button_query
        .get(event.target)
        .expect("After on click this has to exist");

    match button.ty {
        CreateButtonType::Source => spawn_external_entity(
            &mut commands,
            &subsystem_query,
            &nesting_query,
            **focused_system,
            InterfaceType::Import,
            button
                .substance_type
                .expect("Source button must have a substance type"),
            button.connection_source,
            transform,
            &mut fixed_system_element_geometries,
            **zoom,
            true,
            &mut meshes,
            &mut stroke_tess,
            "Source",
            "",
            true,
        ),
        CreateButtonType::Sink => spawn_external_entity(
            &mut commands,
            &subsystem_query,
            &nesting_query,
            **focused_system,
            InterfaceType::Export,
            button
                .substance_type
                .expect("Sink button must have a substance type"),
            button.connection_source,
            transform,
            &mut fixed_system_element_geometries,
            **zoom,
            true,
            &mut meshes,
            &mut stroke_tess,
            "Sink",
            "",
            true,
        ),
        _ => unreachable!("The other types are handled in other event listeners"),
    };

    despawn_create_button(&mut commands, event.target, &only_button_query);
}

pub fn on_create_button_click(
    mut commands: Commands,
    mut event: ListenerMut<Pointer<Click>>,
    button_query: Query<(&CreateButton, &Transform)>,
    only_button_query: Query<(&CreateButton, Option<&Parent>)>,
    flow_interface_query: Query<(
        Entity,
        &Flow,
        Option<&FlowEndInterfaceConnection>,
        Option<&FlowStartInterfaceConnection>,
    )>,
    system_query: Query<(&Transform, &crate::components::System)>,
    transform_query: Query<&Transform>,
    parent_query: Query<&Parent>,
    subsystem_query: Query<&Subsystem>,
    nesting_query: Query<&NestingLevel>,
    mut pick_selection_query: Query<&mut PickSelection>,
    focused_system: Res<FocusedSystem>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
) {
    event.stop_propagation();

    do_deselect_all(&mut pick_selection_query);

    let (button, transform) = button_query
        .get(event.target)
        .expect("After on click this has to exist");

    let nesting_level = NestingLevel::current(**focused_system, &nesting_query);

    match button.ty {
        CreateButtonType::ImportInterface => spawn_interface(
            &mut commands,
            InterfaceType::Import,
            button
                .substance_type
                .expect("Interface button must have a substance type"),
            button.connection_source,
            transform,
            nesting_level,
            **focused_system,
            &mut fixed_system_element_geometries,
            **zoom,
            true,
            &mut meshes,
            &mut stroke_tess,
            "Interface",
            "",
        ),
        CreateButtonType::ExportInterface => spawn_interface(
            &mut commands,
            InterfaceType::Export,
            button
                .substance_type
                .expect("Interface button must have a substance type"),
            button.connection_source,
            transform,
            nesting_level,
            **focused_system,
            &mut fixed_system_element_geometries,
            **zoom,
            true,
            &mut meshes,
            &mut stroke_tess,
            "Interface",
            "",
        ),
        CreateButtonType::Inflow => spawn_inflow(
            &mut commands,
            &subsystem_query,
            &nesting_query,
            &system_query,
            button.connection_source,
            &combined_transform_of_entity_until_ancestor(
                event.target,
                subsystem_query
                    .get(button.connection_source)
                    .ok()
                    .map(|s| s.parent_system),
                &transform_query,
                &parent_query,
            ),
            &mut stroke_tess,
            &mut meshes,
            **zoom,
            true,
            Default::default(),
            InteractionUsability::Resource,
            dec!(1),
            "",
            "Flow",
            "",
        ),
        CreateButtonType::Outflow => spawn_outflow(
            &mut commands,
            &subsystem_query,
            &nesting_query,
            &system_query,
            button.connection_source,
            &combined_transform_of_entity_until_ancestor(
                event.target,
                subsystem_query
                    .get(button.connection_source)
                    .ok()
                    .map(|s| s.parent_system),
                &transform_query,
                &parent_query,
            ),
            &mut stroke_tess,
            &mut meshes,
            **zoom,
            true,
            Default::default(),
            InteractionUsability::Product,
            dec!(1),
            "",
            "Flow",
            "",
        ),
        CreateButtonType::InterfaceSubsystem {
            is_child_of_interface,
            interface_type,
        } => spawn_interface_subsystem(
            &mut commands,
            is_child_of_interface,
            interface_type,
            button.connection_source,
            &flow_interface_query,
            &system_query,
            &nesting_query,
            &focused_system,
            &mut meshes,
            **zoom,
            "Subsystem",
            "",
        ),

        _ => unreachable!("The other types are handled in other event listeners"),
    };

    despawn_create_button(&mut commands, event.target, &only_button_query);
}
