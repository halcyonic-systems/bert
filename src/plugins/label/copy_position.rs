use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct CopyPosition {
    pub target: Entity,
    pub offset: Vec3,
    pub local_offset: bool,
    pub horizontal_alignment: Alignment,
    pub vertical_alignment: Alignment,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Default)]
pub enum Alignment {
    #[default]
    Center,
    Auto,
}

pub fn copy_position(
    source_query: Query<(&CopyPosition, &GlobalTransform), Changed<GlobalTransform>>,
    mut target_query: Query<&mut Transform>,
) {
    for (copy_position, transform) in &source_query {
        let pos = if copy_position.local_offset {
            transform.transform_point(copy_position.offset)
        } else {
            transform.translation() + copy_position.offset
        };

        let pos = pos.round();

        target_query
            .get_mut(copy_position.target)
            .expect("Target should exist")
            .translation = pos;
    }
}

pub fn compute_text_alignment(
    source_query: Query<(&CopyPosition, &GlobalTransform), Changed<GlobalTransform>>,
    children_query: Query<&Children>,
    mut anchor_query: Query<&mut Anchor>,
) {
    for (copy_position, transform) in &source_query {
        for child in children_query
            .get(copy_position.target)
            .expect("Target should have children")
        {
            if let Ok(mut anchor) = anchor_query.get_mut(*child) {
                let global_offset = if copy_position.local_offset {
                    transform.affine().transform_vector3(copy_position.offset)
                } else {
                    copy_position.offset
                };

                let x = match copy_position.horizontal_alignment {
                    Alignment::Center => 0.0,
                    Alignment::Auto => -global_offset.x.signum() * 0.5,
                };

                let y = match copy_position.vertical_alignment {
                    Alignment::Center => 0.0,
                    Alignment::Auto => -global_offset.y.signum() * 0.5,
                };

                *anchor = Anchor::Custom(vec2(x, y));

                break;
            }
        }
    }
}
