use crate::constants::SUBSYSTEM_SCALING_FACTOR;
use bevy::prelude::*;
use num_traits::Pow;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Deref, DerefMut)]
#[reflect(Component)]
pub struct InitialPosition(Vec2);

impl InitialPosition {
    pub fn new(pos: Vec2) -> Self {
        Self(pos)
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq, Deref, DerefMut, Default)]
#[reflect(Component)]
// would be 0 for the root system (which doesn't have this component)
pub struct NestingLevel(u16);

impl NestingLevel {
    pub fn new(nesting_level: u16) -> Self {
        Self(nesting_level)
    }

    pub fn current(parent_system_entity: Entity, nesting_query: &Query<&Self>) -> u16 {
        if let Ok(nesting) = nesting_query.get(parent_system_entity) {
            **nesting
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn compute_scale(nesting_level: u16, zoom: f32) -> f32 {
        (SUBSYSTEM_SCALING_FACTOR.pow(nesting_level) * zoom).min(1.0)
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct ApplyZoomToScale;
