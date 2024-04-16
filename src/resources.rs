use bevy::prelude::*;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct FocusedSystem(Entity);

impl FocusedSystem {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}