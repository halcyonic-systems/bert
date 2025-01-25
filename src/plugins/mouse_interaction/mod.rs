#[cfg(feature = "debug_selection")]
pub mod debug;

use crate::events::InterfaceDrag;
use bevy::input::common_conditions::{input_just_pressed, input_just_released, input_pressed};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use bevy_picking::focus::PickingInteraction;
use bevy_picking::mesh_picking::ray_cast::SimplifiedMesh;
use bevy_picking::mesh_picking::update_hits;
use bevy_picking::PickSet;
use bevy_picking::pointer::PointerInteraction;
use bevy_picking::prelude::*;

const DRAG_THRESHOLD_SQUARED: f32 = 4.0;

pub struct MouseInteractionPlugin;

impl Plugin for MouseInteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<PickingPlugin>()
            .init_resource::<Dragging>()
            .init_resource::<Selection>()
            .init_resource::<MouseWorldPosition>()
            .init_resource::<SelectionEnabled>()
            .add_event::<DragPosition>()
            .add_observer(
                |on_drag: Trigger<DragPosition>, mut writer: EventWriter<InterfaceDrag>| {
                    println!("interface dragged");
                    writer.send(on_drag.into());
                },
            )
            .register_type::<PickSelection>()
            .add_systems(PreUpdate, mouse_screen_to_world_position)
            .add_systems(
                Update,
                (
                    (
                        handle_mouse_down.run_if(input_just_pressed(MouseButton::Left)),
                        handle_mouse_up.run_if(input_just_released(MouseButton::Left)),
                        handle_mouse_drag
                            .run_if(input_pressed(MouseButton::Left))
                            .after(handle_mouse_down),
                    )
                        .in_set(MouseInteractionSet),
                    deselect_when_invisible,
                    deselect_all.run_if(input_just_pressed(KeyCode::Escape)),
                ),
            );

        #[cfg(feature = "debug_selection")]
        {
            app.init_resource::<debug::SelectedEntities>()
                .register_type::<debug::SelectedEntities>()
                .add_systems(Update, debug::debug_selection);
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MouseInteractionSet;

#[derive(Resource, Clone, PartialEq, Eq, Reflect, Debug, Deref, DerefMut)]
#[reflect(Resource)]
pub struct SelectionEnabled(bool);

impl Default for SelectionEnabled {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Clone, Event)]
#[allow(dead_code)]
pub struct DragPosition {
    pub target: Entity,
    /// Local to parent coordinates
    pub local_position: Vec2,
    pub world_position: Vec2,
}

#[derive(Resource, Clone, PartialEq, Reflect, Debug, Default)]
pub struct Dragging {
    hovered_entity: Option<Entity>,
    started: bool,
    start_pos: Vec2,
}

#[derive(Resource, Clone, Deref, DerefMut, PartialEq, Eq, Reflect, Debug, Default)]
pub struct Selection(HashSet<Entity>);

#[derive(Resource, Clone, Deref, DerefMut, PartialEq, Reflect, Debug, Default)]
pub struct MouseWorldPosition(Vec2);

#[derive(Component, Default, Copy, Clone, Debug, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct PickSelection {
    pub is_selected: bool,
}

#[derive(Component, Copy, Clone, PartialEq, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct NoDeselect;

#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct PickParent;

#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect, Debug)]
#[reflect(Component)]
pub struct PickTarget {
    pub target: Entity,
}

fn handle_mouse_down(
    interaction_query: Query<(
        Entity,
        &PickingInteraction,
        Option<&PickParent>,
        Option<&PickTarget>,
    )>,
    parent_query: Query<&Parent>,
    mouse_position: Res<MouseWorldPosition>,
    mut dragging: ResMut<Dragging>,
) {
    dragging.hovered_entity = None;
    dragging.started = false;
    dragging.start_pos = **mouse_position;

    for (entity, interaction, pick_parent, pick_target) in &interaction_query {
        if !matches!(interaction, PickingInteraction::None) {
            if pick_parent.is_some() {
                dragging.hovered_entity = Some(
                    parent_query
                        .get(entity)
                        .expect("Parent should exist for components that have PickParent")
                        .get(),
                );
            } else if let Some(target) = pick_target {
                println!("hovered_entity: {target:?}");
                dragging.hovered_entity = Some(target.target);
            } else {
                dragging.hovered_entity = Some(entity);
            }
            break;
        }
    }
}

fn handle_mouse_up(
    interaction_query: Query<(Entity, Option<&NoDeselect>)>,
    mut pick_selection_query: Query<&mut PickSelection>,
    mut dragging: ResMut<Dragging>,
    mut selection: ResMut<Selection>,
    selection_enabled: Res<SelectionEnabled>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if dragging.started {
        dragging.started = false;
        dragging.hovered_entity = None;
        return;
    }

    let multi_select = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    if **selection_enabled {
        selection.clear();

        let mut deselect = true;

        if let Some(hovered_entity) = dragging.hovered_entity {
            if let Ok((entity, no_deselect)) = interaction_query.get(hovered_entity) {
                if no_deselect.is_some() {
                    deselect = false;
                } else {
                    selection.insert(entity);
                }
            } else {
                deselect = false;
            }
        }

        if deselect && !multi_select {
            do_deselect_all(&mut pick_selection_query);
        }

        if !selection.is_empty() {
            for entity in &selection.0 {
                if let Ok(mut pick_selection) = pick_selection_query.get_mut(*entity) {
                    if multi_select {
                        pick_selection.is_selected = !pick_selection.is_selected;
                    } else {
                        pick_selection.is_selected = true;
                    }
                }
            }
        }
    }

    dragging.hovered_entity = None;
}

fn handle_mouse_drag(
    mouse_position: Res<MouseWorldPosition>,
    mut writer: EventWriter<DragPosition>,
    mut dragging: ResMut<Dragging>,
    transform_query: Query<&GlobalTransform>,
    parent_query: Query<&Parent>,
) {
    let mouse_position = **mouse_position;

    // println!("dragging: {dragging:?}");
    if !dragging.started
        && mouse_position.distance_squared(dragging.start_pos) > DRAG_THRESHOLD_SQUARED
    {
        dragging.started = true;
    }

    if dragging.started {
        if let Some(entity) = dragging.hovered_entity {
            let position = if let Ok(parent) = parent_query.get(entity) {
                let parent_transform = transform_query
                    .get(parent.get())
                    .expect("Parent should have a Transform");
                parent_transform
                    .affine()
                    .inverse()
                    .transform_point3(mouse_position.extend(0.0))
                    .truncate()
            } else {
                mouse_position
            };

            writer.send(DragPosition {
                target: entity,
                local_position: position,
                world_position: mouse_position,
            });
        }
    }
}

fn mouse_screen_to_world_position(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut mouse_world_position: ResMut<MouseWorldPosition>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = window.cursor_position().and_then(|window_position| {
        camera
            .viewport_to_world_2d(camera_transform, window_position)
            .ok()
    }) {
        **mouse_world_position = world_position;
    }
}

pub fn deselect_all(mut query: Query<&mut PickSelection>) {
    do_deselect_all(&mut query);
}

pub fn do_deselect_all(pick_selection_query: &mut Query<&mut PickSelection>) {
    for mut pick_selection in pick_selection_query {
        pick_selection.is_selected = false;
    }
}

pub fn disable_selection(mut selection_enabled: ResMut<SelectionEnabled>) {
    **selection_enabled = false;
}

pub fn enable_selection(
    mut selection_enabled: ResMut<SelectionEnabled>,
    mut dragging: ResMut<Dragging>,
) {
    **selection_enabled = true;

    dragging.hovered_entity = None;
    dragging.started = false;
}

pub fn deselect_when_invisible(
    mut selection_query: Query<
        (&mut PickSelection, &InheritedVisibility),
        Changed<InheritedVisibility>,
    >,
) {
    for (mut pick_selection, visibility) in &mut selection_query {
        if !visibility.get() {
            pick_selection.is_selected = false;
        }
    }
}
