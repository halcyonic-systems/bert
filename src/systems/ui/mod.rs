mod add_remove_buttons;
mod zoom;

pub use add_remove_buttons::*;
pub use zoom::*;

use crate::bundles::{
    despawn_create_button, despawn_create_button_with_component, spawn_external_entity,
    spawn_inflow, spawn_interface, spawn_interface_subsystem, spawn_outflow,
};
use crate::components::*;
use crate::resources::{FocusedSystem, Zoom};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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
                    && matches!(button.ty, CreateButtonType::InterfaceSubsystem)
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
    event: Listener<Pointer<Click>>,
    button_query: Query<(&CreateButton, &GlobalTransform, &InitialPosition)>,
    only_button_query: Query<&CreateButton>,
    flow_interface_query: Query<
        (
            Entity,
            Option<&InflowInterfaceConnection>,
            Option<&OutflowInterfaceConnection>,
        ),
        Or<(With<Inflow>, With<Outflow>)>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    zoom: Res<Zoom>,
) {
    let (button, transform, initial_position) = button_query
        .get(event.target)
        .expect("After on click this has to exist");

    match button.ty {
        CreateButtonType::ImportInterface => spawn_interface(
            &mut commands,
            InterfaceType::Import,
            button.connection_source,
            transform,
            initial_position,
            **zoom,
        ),
        CreateButtonType::ExportInterface => spawn_interface(
            &mut commands,
            InterfaceType::Export,
            button.connection_source,
            transform,
            initial_position,
            **zoom,
        ),
        CreateButtonType::Inflow => spawn_inflow(
            &mut commands,
            button.connection_source,
            &transform,
            initial_position,
            &mut meshes,
            &mut materials,
            **zoom,
        ),
        CreateButtonType::Outflow => spawn_outflow(
            &mut commands,
            button.connection_source,
            &transform,
            initial_position,
            &mut meshes,
            &mut materials,
            **zoom,
        ),
        CreateButtonType::Source => spawn_external_entity(
            &mut commands,
            InterfaceType::Import,
            button.connection_source,
            &transform,
            initial_position,
            &mut meshes,
            &mut materials,
            **zoom,
        ),
        CreateButtonType::Sink => spawn_external_entity(
            &mut commands,
            InterfaceType::Export,
            button.connection_source,
            &transform,
            initial_position,
            &mut meshes,
            &mut materials,
            **zoom,
        ),
        CreateButtonType::InterfaceSubsystem => spawn_interface_subsystem(
            &mut commands,
            button.connection_source,
            &flow_interface_query,
            &mut meshes,
            &mut materials,
        ),
    }

    despawn_create_button(&mut commands, event.target, &only_button_query);
}
