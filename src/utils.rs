use crate::components::InitialPosition;
use bevy::prelude::*;

pub fn ui_transform_from_button(
    button_transform: &Transform,
    button_initial_position: &InitialPosition,
    z: f32,
    move_right: f32,
    zoom: f32,
) -> (Transform, InitialPosition) {
    let position = **button_initial_position;
    let right = button_transform.right().truncate();
    let angle = right.to_angle();

    let position = position + right * move_right;
    (
        Transform::from_translation((position * zoom).extend(z))
            .with_rotation(Quat::from_rotation_z(angle)),
        InitialPosition::new(position),
    )
}
