use crate::bundles::FixedSystemElementGeometry;
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

impl Zoom {
    pub fn add(&mut self, step: f32) {
        self.0 += step;
        self.0 = self.0.max(0.1);
    }
}

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

#[derive(Resource)]
pub struct FixedSystemElementGeometries {
    pub interface: FixedSystemElementGeometry,
    pub external_entity: FixedSystemElementGeometry,
}
