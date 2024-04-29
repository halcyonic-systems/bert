use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Deref, DerefMut)]
#[reflect(Component)]
pub struct InitialPosition(Vec2);

impl InitialPosition {
    pub fn new(pos: Vec2) -> Self {
        Self(pos)
    }
}
