use bevy::prelude::*;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct FocusedSystem(Entity);

impl FocusedSystem {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct Zoom(f32);

impl Default for Zoom {
    fn default() -> Self {
        Self(1.0)
    }
}