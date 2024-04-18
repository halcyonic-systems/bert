use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Deref, DerefMut)]
#[reflect(Component)]
pub struct InitialPosition(Vec2);

impl InitialPosition {
    pub fn new(pos: Vec2) -> Self {
        Self(pos)
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Deref, DerefMut)]
#[reflect(Component)]
pub struct ScaleWithZoom(Vec2);

// impl ScaleWithZoom {
//     pub fn new(x: f32, y: f32) -> Self {
//         Self(vec2(x, y))
//     }
// }

impl Default for ScaleWithZoom {
    fn default() -> Self {
        Self(Vec2::ONE)
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Deref, DerefMut)]
#[reflect(Component)]
pub struct ZoomIndependentStrokeWidth(f32);

impl ZoomIndependentStrokeWidth {
    pub fn new(width: f32) -> Self {
        Self(width)
    }
}
