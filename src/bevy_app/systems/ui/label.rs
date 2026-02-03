use crate::bevy_app::components::*;
use crate::bevy_app::constants::{
    HIDDING_TRANSPARENCY, INTERFACE_HEIGHT_HALF, INTERFACE_WIDTH_HALF,
};
use crate::bevy_app::plugins::label::{CopyPosition, NameLabel};
use crate::bevy_app::resources::Zoom;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use lyon_algorithms::length::approximate_length;
use lyon_algorithms::walk::{walk_along_path, RegularPattern, WalkerEvent};

pub fn update_label_offset_from_interface(
    mut query: Query<
        (&mut CopyPosition, &GlobalTransform, &NestingLevel),
        (Changed<GlobalTransform>, With<Interface>),
    >,
    zoom: Res<Zoom>,
) {
    for (mut copy_position, global_transform, nesting_level) in &mut query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);

        let right = global_transform.right();

        let x = (INTERFACE_WIDTH_HALF + 20.0) * scale;
        let y = -INTERFACE_HEIGHT_HALF * scale * right.y.signum() * right.x.signum();

        copy_position.offset = vec3(x, y, 0.0);
    }
}

pub fn update_label_from_interaction(
    interaction_query: Query<
        (&Shape, &NameLabel, &GlobalTransform),
        (
            With<Flow>,
            Or<(Changed<Shape>, Changed<GlobalTransform>, Added<NameLabel>)>,
        ),
    >,
    parent_query: Query<&ChildOf>,
    mut transform_query: Query<&mut Transform>,
) {
    for (shape, name_label, global_transform) in &interaction_query {
        let sprite_entity = parent_query
            .get(name_label.label)
            .expect("Label should have a Parent")
            .parent();

        let mut transform = transform_query
            .get_mut(sprite_entity)
            .expect("Label should have a Transform");

        let path_len = approximate_length(&shape.path, 0.1);

        let mut pattern = RegularPattern {
            callback: &mut |event: WalkerEvent| {
                let interaction_transform = global_transform.affine();

                let pos = interaction_transform
                    .transform_point3(vec3(event.position.x, event.position.y, 0.0))
                    .truncate();

                transform.translation = pos.extend(transform.translation.z);

                let mut tangent = vec3(event.tangent.x, event.tangent.y, 0.0);
                if tangent.x < 0.0 {
                    tangent = -tangent;
                }
                let tangent = interaction_transform.transform_vector3(tangent).truncate();
                transform.rotation = Quat::from_rotation_z(tangent.to_angle());

                false // Stop walking the path.
            },
            interval: 1.0,
        };

        walk_along_path(&shape.path, path_len * 0.5, 0.1, &mut pattern);
    }
}

pub fn update_text_color(
    source_query: Query<&NameLabel, Added<Hidden>>,
    mut target_query: Query<(Entity, &mut TextColor), With<Text2d>>,
    mut removed_hidden: RemovedComponents<Hidden>,
    name_label_query: Query<&NameLabel>,
    parent_query: Query<&ChildOf>,
) {
    for label in &source_query {
        if let Ok((entity, mut color)) = target_query.get_mut(label.label) {
            if parent_query.get(entity).is_ok() {
                color.0.set_alpha(HIDDING_TRANSPARENCY);
            }
        }
    }

    for hidden_entity in removed_hidden.read() {
        if let Ok(label) = name_label_query.get(hidden_entity) {
            if let Ok((entity, mut color)) = target_query.get_mut(label.label) {
                if parent_query.get(entity).is_ok() {
                    color.0.set_alpha(1.0);
                }
            }
        }
    }
}
