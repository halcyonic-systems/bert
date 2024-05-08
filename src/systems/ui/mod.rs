mod add_remove_buttons;
mod color;
mod drag;
mod flow;
mod selected_helper;
mod zoom;

use crate::plugins::mouse_interaction::{deselect_all, PickSelection};
pub use add_remove_buttons::*;
pub use color::*;
pub use drag::*;
pub use flow::*;
pub use selected_helper::*;
pub use zoom::*;

use crate::bundles::{
    despawn_create_button, despawn_create_button_with_component, spawn_external_entity,
    spawn_inflow, spawn_interface, spawn_interface_subsystem, spawn_outflow,
};
use crate::components::*;
use crate::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator, Zoom,
};
use crate::utils::combined_transform_of_entity_until_ancestor;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use rust_decimal_macros::dec;

pub fn change_focused_system(
    selected_query: Query<
        (Entity, &PickSelection),
        (
            Changed<PickSelection>,
            Or<(With<crate::components::System>, With<Subsystem>)>,
        ),
    >,
    button_query: Query<&CreateButton>,
    mut focused_system: ResMut<FocusedSystem>,
) {
    for (entity, selection) in &selected_query {
        if selection.is_selected {
            for button in &button_query {
                if button.system == **focused_system
                    && matches!(button.ty, CreateButtonType::InterfaceSubsystem { .. })
                {
                    return;
                }
            }

            **focused_system = entity;
        }
    }
}

pub fn remove_unfocused_system_buttons(
    mut commands: Commands,
    focused_system: Res<FocusedSystem>,
    previous_focused_system: Local<Option<Entity>>,
    button_query: Query<(Entity, &CreateButton)>,
) {
    if !focused_system.is_changed() || Some(**focused_system) == *previous_focused_system {
        return;
    }

    let focused_system = **focused_system;

    for (entity, button) in &button_query {
        if button.system != focused_system {
            despawn_create_button_with_component(&mut commands, entity, button);
        }
    }
}

pub fn on_create_button_click(
    mut commands: Commands,
    mut event: ListenerMut<Pointer<Click>>,
    button_query: Query<(&CreateButton, &Transform)>,
    only_button_query: Query<&CreateButton>,
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

    deselect_all(&mut pick_selection_query);

    let (button, transform) = button_query
        .get(event.target)
        .expect("After on click this has to exist");

    match button.ty {
        CreateButtonType::ImportInterface => spawn_interface(
            &mut commands,
            InterfaceType::Import,
            button
                .substance_type
                .expect("Interface button must have a substance type"),
            button.connection_source,
            transform,
            &nesting_query,
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
            &nesting_query,
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
            Default::default(),
            dec!(1),
            "",
            dec!(1),
            "Inflow",
            "",
        ),
        CreateButtonType::Outflow => spawn_outflow(
            &mut commands,
            &subsystem_query,
            &nesting_query,
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
            Default::default(),
            dec!(1),
            "",
            dec!(1),
            "Outflow",
            "",
        ),
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
        ),
        CreateButtonType::InterfaceSubsystem {
            is_child_of_interface,
        } => spawn_interface_subsystem(
            &mut commands,
            is_child_of_interface,
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
    };

    despawn_create_button(&mut commands, event.target, &only_button_query);
}
