use crate::components::InitialPosition;
use crate::constants::{FLOW_END_LENGTH, INTERFACE_WIDTH_HALF};
use bevy::prelude::*;

pub fn ui_transform_from_button(
    button_transform: &Transform,
    z: f32,
    move_right: f32,
    zoom: f32,
) -> (Transform, InitialPosition) {
    let position = button_transform.translation.truncate() / zoom;
    let right = button_transform.right().truncate();

    let position = position + right * move_right;
    (
        Transform::from_translation((position * zoom).extend(z))
            .with_rotation(button_transform.rotation),
        InitialPosition::new(position),
    )
}

pub fn compute_end_and_direction_from_system_child(
    system_child: Entity,
    transform_query: &Query<&Transform>,
    parent_query: &Query<&Parent>,
    flow_parent: Option<Entity>,
) -> (Vec2, Vec2) {
    let combined_transform = combined_transform_of_entity_until_common_parent(
        system_child,
        flow_parent,
        transform_query,
        parent_query,
    );

    let right = combined_transform.right().truncate();

    (
        combined_transform.translation.truncate() + right * INTERFACE_WIDTH_HALF,
        right * FLOW_END_LENGTH,
    )
}

pub fn combined_transform_of_entity_until_common_parent(
    entity: Entity,
    common_parent: Option<Entity>,
    transform_query: &Query<&Transform>,
    parent_query: &Query<&Parent>,
) -> Transform {
    let mut combined_transform = *transform_query
        .get(entity)
        .expect("Entity should have a Transform");
    let mut parent_entity = parent_query
        .get(entity)
        .expect("Entity should have a Parent")
        .get();

    loop {
        let parent_transform = transform_query
            .get(parent_entity)
            .expect("Parent should have a Transform");

        combined_transform = parent_transform.mul_transform(combined_transform);

        if let Ok(parent) = parent_query.get(parent_entity) {
            parent_entity = parent.get();
        } else {
            break;
        }

        if Some(parent_entity) == common_parent {
            break;
        }
    }

    combined_transform
}
