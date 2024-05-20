use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct CopyPosition {
    pub target: Entity,
    pub offset: Vec3,
    pub local_offset: bool,
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

        target_query
            .get_mut(copy_position.target)
            .expect("Target should exist")
            .translation = pos;
    }
}
