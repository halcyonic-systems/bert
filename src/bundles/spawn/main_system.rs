use crate::bundles::SystemBundle;
use crate::components::{SystemBoundary, SystemEnvironment};
use crate::constants::MAIN_SYSTEM_RADIUS;
use crate::Pinnable;
use bevy::math::Vec2;
use bevy::prelude::*;
use crate::data_model::Complexity;

pub fn spawn_main_system(
    commands: &mut Commands,
    center: Vec2,
    angle: f32,
    complexity: Complexity,
    boundary: SystemBoundary,
    zoom: f32,
    name: &str,
    description: &str,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> Entity {
    commands
        .spawn((
            SystemBundle::new(
                center,
                0.0,
                MAIN_SYSTEM_RADIUS,
                angle,
                complexity,
                boundary,
                meshes,
                zoom,
                0,
                name,
                description,
            ),
            SystemEnvironment::default(),
            Pinnable { has_pins: false },
        ))
        .id()
}
