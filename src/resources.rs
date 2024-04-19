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

#[derive(Resource, Deref, DerefMut)]
pub struct StrokeTessellator(bevy_prototype_lyon::prelude::tess::StrokeTessellator);

impl StrokeTessellator {
    pub fn new() -> Self {
        StrokeTessellator(bevy_prototype_lyon::prelude::tess::StrokeTessellator::new())
    }
}