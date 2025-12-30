mod add_remove_buttons; // DISABLED: Button systems not used in drag-and-drop workflow (Phase 0)
mod color;
mod drag;
mod flow;
mod hiding;
mod label;
mod selected_helper;
mod source_sink_equivalence;
mod zoom;

use crate::bevy_app::plugins::mouse_interaction::{do_deselect_all, PickSelection};
// Button module still needed for helper functions (has_incomplete_interactions, etc.) used by other systems
pub use add_remove_buttons::*;
pub use color::*;
pub use drag::*;
pub use flow::*;
pub use hiding::*;
pub use label::*;
pub use selected_helper::*;
pub use source_sink_equivalence::*;
pub use zoom::*;

use crate::bevy_app::bundles::{
    despawn_create_button, despawn_create_button_with_component, spawn_external_entity,
    spawn_inflow, spawn_interface, spawn_interface_subsystem, spawn_outflow, spawn_subsystem,
};
use crate::bevy_app::components::*;
use crate::bevy_app::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator, Zoom,
};
use crate::bevy_app::states::AppState;
use crate::bevy_app::utils::combined_transform_of_entity_until_ancestor;
use bevy::prelude::*;
use rust_decimal_macros::dec;

/// Updates FocusedSystem resource when user selects a system.
///
/// Core selection handler (not button-specific) - must remain active for correct
/// subsystem placement, flow creation, and other FocusedSystem-dependent systems.
///
/// Button validation removed in drag-and-drop transition - original logic prevented
/// selecting interface subsystems before all buttons created, unnecessary without buttons.
pub fn change_focused_system(
    selected_query: Query<
        (Entity, &PickSelection),
        (
            Changed<PickSelection>,
            With<crate::bevy_app::components::System>,
        ),
    >,
    mut focused_system: ResMut<FocusedSystem>,
) {
    for (entity, selection) in &selected_query {
        if selection.is_selected {
            // PHASE 3D FIX: Only assign if entity actually changed to prevent spurious auto-zoom triggers.
            // Clicking canvas to refocus keyboard was causing FocusedSystem reassignment to same entity,
            // triggering Bevy change detection → auto-zoom → jittery camera during manual zoom.
            if **focused_system != entity {
                **focused_system = entity;
            }
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
    mut trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    transform_query: Query<&Transform>,
    only_button_query: Query<(&CreateButton, Option<&Parent>)>,
    external_entity_query: Query<(Entity, &PickSelection, &Parent), With<ExternalEntity>>,
    flow_connection_query: Query<(Entity, &FlowStartConnection, &FlowEndConnection)>,
    flow_query: Query<(&FlowCurve, &Flow)>,
    system_query: Query<(
        &Transform,
        &crate::bevy_app::components::System,
        &Name,
        &ElementDescription,
    )>,
    nesting_level_query: Query<&NestingLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
) {
    trigger.propagate(false);

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
        .get(trigger.target)
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

    despawn_create_button(&mut commands, trigger.target, &only_button_query);
}

pub fn on_flow_terminal_button_click(
    mut trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    only_button_query: Query<(&CreateButton, Option<&Parent>)>,
    mut pick_selection_query: Query<&mut PickSelection>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    trigger.propagate(false);

    do_deselect_all(&mut pick_selection_query);

    let (button, _) = only_button_query
        .get(trigger.target)
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

    despawn_create_button(&mut commands, trigger.target, &only_button_query);
}

pub fn on_external_entity_create_button_click(
    mut trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
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
    trigger.propagate(false);

    do_deselect_all(&mut pick_selection_query);

    let (button, transform) = button_query
        .get(trigger.target)
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

    despawn_create_button(&mut commands, trigger.target, &only_button_query);
}

pub fn on_create_button_click(
    mut trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    button_query: Query<(&CreateButton, &Transform)>,
    only_button_query: Query<(&CreateButton, Option<&Parent>)>,
    flow_interface_query: Query<(
        Entity,
        &Flow,
        Option<&FlowEndInterfaceConnection>,
        Option<&FlowStartInterfaceConnection>,
    )>,
    system_query: Query<(
        &Transform,
        &crate::bevy_app::components::System,
        &Name,
        &ElementDescription,
    )>,
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
    trigger.propagate(false);

    do_deselect_all(&mut pick_selection_query);

    let (button, transform) = button_query
        .get(trigger.target)
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
                trigger.target,
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
                trigger.target,
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

    despawn_create_button(&mut commands, trigger.target, &only_button_query);
}
