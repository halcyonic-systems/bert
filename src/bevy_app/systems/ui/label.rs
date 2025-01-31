use crate::bevy_app::components::*;
use crate::bevy_app::constants::{INTERFACE_HEIGHT_HALF, INTERFACE_WIDTH_HALF};
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
        (&Path, &NameLabel, &GlobalTransform),
        (
            With<Flow>,
            Or<(Changed<Path>, Changed<GlobalTransform>, Added<NameLabel>)>,
        ),
    >,
    parent_query: Query<&Parent>,
    mut transform_query: Query<&mut Transform>,
) {
    for (path, name_label, global_transform) in &interaction_query {
        let sprite_entity = parent_query
            .get(name_label.label)
            .expect("Label should have a Parent")
            .get();

        let mut transform = transform_query
            .get_mut(sprite_entity)
            .expect("Label should have a Transform");

        let path_len = approximate_length(&path.0, 0.1);

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

        walk_along_path(&path.0, path_len * 0.5, 0.1, &mut pattern);
    }
}
