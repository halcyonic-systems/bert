mod add_remove_buttons;
mod zoom;

pub use add_remove_buttons::*;
use bevy::math::vec2;
pub use zoom::*;

use crate::bundles::{
    despawn_create_button, despawn_create_button_with_component, spawn_external_entity,
    spawn_inflow, spawn_interface, spawn_interface_subsystem, spawn_outflow,
};
use crate::components::*;
use crate::constants::{ARROW_HEAD_LENGTH, ARROW_HEAD_WIDTH_HALF};
use crate::resources::{FocusedSystem, Zoom};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

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
    system_query: Query<&crate::components::System>,
    focused_system: Res<FocusedSystem>,
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
            &system_query,
            &focused_system,
        ),
    }

    despawn_create_button(&mut commands, event.target, &only_button_query);
}

pub fn draw_flow_curve(
    mut query: Query<(&FlowCurve, &mut Path, &Children), Changed<FlowCurve>>,
    mut path_query: Query<&mut Path, Without<FlowCurve>>,
    zoom: Res<Zoom>,
) {
    for (flow_curve, path, children) in &mut query {
        update_flow_curve(&mut path_query, flow_curve, path, children, **zoom);
    }
}

pub fn update_flow_curve(
    path_query: &mut Query<&mut Path, Without<FlowCurve>>,
    flow_curve: &FlowCurve,
    mut path: Mut<Path>,
    children: &Children,
    zoom: f32,
) {
    let (curve_path, head_path) = create_paths_from_flow_curve(flow_curve, zoom);
    *path = curve_path;

    if let Some(child) = children.iter().next() {
        if let Ok(mut path) = path_query.get_mut(*child) {
            *path = head_path;
        }
    }
}

pub fn create_paths_from_flow_curve(flow_curve: &FlowCurve, zoom: f32) -> (Path, Path) {
    let mut curve_path_builder = PathBuilder::new();

    let zoomed_start = flow_curve.start * zoom;
    let zoomed_end = flow_curve.end * zoom;

    curve_path_builder.move_to(zoomed_start);

    let end_direction = flow_curve.end_direction.normalize();
    let end = zoomed_end + end_direction * (ARROW_HEAD_LENGTH - 2.0);

    curve_path_builder.cubic_bezier_to(
        zoomed_start + flow_curve.start_direction,
        end + flow_curve.end_direction,
        end,
    );

    let mut head_path_builder = PathBuilder::new();

    let head_width_direction = vec2(end_direction.y, -end_direction.x);

    head_path_builder.move_to(zoomed_end);
    head_path_builder.line_to(
        zoomed_end
            + end_direction * ARROW_HEAD_LENGTH
            + head_width_direction * ARROW_HEAD_WIDTH_HALF,
    );
    head_path_builder.line_to(
        zoomed_end + end_direction * ARROW_HEAD_LENGTH
            - head_width_direction * ARROW_HEAD_WIDTH_HALF,
    );
    head_path_builder.close();

    (curve_path_builder.build(), head_path_builder.build())
}
