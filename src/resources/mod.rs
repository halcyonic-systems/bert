mod system_element_geometry;

pub use system_element_geometry::*;

use bevy::prelude::*;

#[derive(Debug, Resource, Deref, DerefMut, Copy, Clone, Reflect)]
#[reflect(Resource)]
pub struct FocusedSystem(Entity);

impl FocusedSystem {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}

impl Default for FocusedSystem {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

#[derive(Debug, Resource, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct Zoom(f32);

impl Zoom {
    pub fn mul(&mut self, fac: f32) {
        debug_assert!(fac > 0.0);
        self.0 *= fac;
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
