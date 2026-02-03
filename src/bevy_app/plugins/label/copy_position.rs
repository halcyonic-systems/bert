use bevy::camera::primitives::Aabb;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct CopyPosition {
    pub target: Entity,
    pub aabb: Option<Aabb>,
    pub offset: Vec3,
    pub local_offset: bool,
    pub horizontal_alignment: Alignment,
    pub vertical_alignment: Alignment,
    pub horizontal_anchor: HorizontalAttachmentAnchor,
    pub vertical_anchor: VerticalAttachmentAnchor,
}

#[derive(Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct CopyPositions(pub Vec<CopyPosition>);

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Default)]
pub enum Alignment {
    #[default]
    Center,
    /// Switch between start, end and center automatically
    Auto,
    /// Switch between start and end automatically
    AutoStartEnd,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Default)]
pub enum HorizontalAttachmentAnchor {
    #[default]
    Center,
    EastLocal,
    WestLocal,
    EastWorld,
    WestWorld,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Default)]
pub enum VerticalAttachmentAnchor {
    #[default]
    Center,
    NorthLocal,
    SouthLocal,
    NorthWorld,
    SouthWorld,
}

pub fn copy_positions_changed(
    source_query: Query<(&CopyPositions, &GlobalTransform, Option<&Aabb>), Changed<CopyPositions>>,
    mut target_query: Query<&mut Transform>,
) {
    for (copy_positions, transform, aabb) in &source_query {
        for copy_position in &copy_positions.0 {
            apply_copy_position(copy_position, transform, aabb, &mut target_query);
        }
    }
}

pub fn copy_positions(
    source_query: Query<
        (&CopyPositions, &GlobalTransform, Option<&Aabb>),
        Changed<GlobalTransform>,
    >,
    mut target_query: Query<&mut Transform>,
) {
    for (copy_positions, transform, aabb) in &source_query {
        for copy_position in &copy_positions.0 {
            apply_copy_position(copy_position, transform, aabb, &mut target_query);
        }
    }
}

pub fn copy_position(
    source_query: Query<(&CopyPosition, &GlobalTransform, Option<&Aabb>), Changed<GlobalTransform>>,
    mut target_query: Query<&mut Transform>,
) {
    for (copy_position, transform, aabb) in &source_query {
        apply_copy_position(copy_position, transform, aabb, &mut target_query);
    }
}

pub fn apply_copy_position(
    copy_position: &CopyPosition,
    transform: &GlobalTransform,
    aabb: Option<&Aabb>,
    target_query: &mut Query<&mut Transform>,
) {
    let pos = if let Some(aabb) = aabb {
        let x = match copy_position.horizontal_anchor {
            HorizontalAttachmentAnchor::Center => 0.0,
            HorizontalAttachmentAnchor::EastLocal => aabb.max().x,
            HorizontalAttachmentAnchor::WestLocal => aabb.min().x,
            HorizontalAttachmentAnchor::EastWorld => {
                unimplemented!()
            }
            HorizontalAttachmentAnchor::WestWorld => {
                unimplemented!()
            }
        };

        let is_upside_down = transform.affine().transform_vector3(Vec3::Y).y > 0.0;

        let y = match copy_position.vertical_anchor {
            VerticalAttachmentAnchor::Center => 0.0,
            VerticalAttachmentAnchor::NorthLocal => aabb.max().y,
            VerticalAttachmentAnchor::SouthLocal => aabb.min().y,
            VerticalAttachmentAnchor::NorthWorld => {
                if is_upside_down {
                    aabb.max().y
                } else {
                    aabb.min().y
                }
            }
            VerticalAttachmentAnchor::SouthWorld => {
                unimplemented!()
            }
        };

        let offset_x = if copy_position.local_offset {
            match copy_position.horizontal_anchor {
                HorizontalAttachmentAnchor::Center
                | HorizontalAttachmentAnchor::EastLocal
                | HorizontalAttachmentAnchor::WestLocal => copy_position.offset.x,
                HorizontalAttachmentAnchor::EastWorld | HorizontalAttachmentAnchor::WestWorld => {
                    if is_upside_down {
                        -copy_position.offset.x
                    } else {
                        copy_position.offset.x
                    }
                }
            }
        } else {
            0.0
        };

        let offset_y = if copy_position.local_offset {
            match copy_position.vertical_anchor {
                VerticalAttachmentAnchor::Center
                | VerticalAttachmentAnchor::NorthLocal
                | VerticalAttachmentAnchor::SouthLocal => copy_position.offset.y,
                VerticalAttachmentAnchor::NorthWorld | VerticalAttachmentAnchor::SouthWorld => {
                    if is_upside_down {
                        -copy_position.offset.y
                    } else {
                        copy_position.offset.y
                    }
                }
            }
        } else {
            0.0
        };

        let pos = transform.affine().transform_point(vec3(
            x + offset_x,
            y + offset_y,
            copy_position.offset.z,
        ));

        if !copy_position.local_offset {
            pos + copy_position.offset
        } else {
            pos
        }
    } else {
        if copy_position.local_offset {
            transform.transform_point(copy_position.offset)
        } else {
            transform.translation() + copy_position.offset
        }
    };

    let pos = pos.round();

    target_query
        .get_mut(copy_position.target)
        .expect("Target should exist")
        .translation = pos;
}

pub fn text_alignment(
    transform: &GlobalTransform,
    copy_position: &CopyPosition,
    children_query: &Query<&Children>,
    anchor_query: &mut Query<&mut Anchor>,
) {
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
                Alignment::AutoStartEnd => -global_offset.x.signum() * 0.5,
                Alignment::Auto => {
                    if global_offset.y.abs() > global_offset.x.abs() {
                        0.0
                    } else {
                        -global_offset.x.signum() * 0.5
                    }
                }
            };

            let y = match copy_position.vertical_alignment {
                Alignment::Center => 0.0,
                Alignment::AutoStartEnd => -global_offset.y.signum() * 0.5,
                Alignment::Auto => {
                    if global_offset.x.abs() > global_offset.y.abs() {
                        0.0
                    } else {
                        -global_offset.y.signum() * 0.5
                    }
                }
            };

            *anchor = Anchor(vec2(x, y));

            break;
        }
    }
}

pub fn compute_text_alignment(
    source_query: Query<(&CopyPosition, &GlobalTransform), Changed<GlobalTransform>>,
    children_query: Query<&Children>,
    mut anchor_query: Query<&mut Anchor>,
) {
    for (copy_position, transform) in &source_query {
        text_alignment(transform, copy_position, &children_query, &mut anchor_query);
    }
}

pub fn compute_text_alignments(
    source_query: Query<(&CopyPositions, &GlobalTransform), Changed<GlobalTransform>>,
    children_query: Query<&Children>,
    mut anchor_query: Query<&mut Anchor>,
) {
    for (copy_positions, transform) in &source_query {
        for copy_position in &copy_positions.0 {
            text_alignment(transform, copy_position, &children_query, &mut anchor_query);
        }
    }
}
