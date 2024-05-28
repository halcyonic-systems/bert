use crate::bundles::SystemBundle;
use crate::components::{NestingLevel, Subsystem, SystemBoundary, SystemEnvironment};
use crate::constants::MAIN_SYSTEM_RADIUS;
use crate::data_model::Complexity;
use crate::plugins::label::{add_name_label, Alignment, AutoContrastTextColor, CopyPositionArgs};
use crate::Pinnable;
use bevy::math::{vec2, vec3, Vec2};
use bevy::prelude::*;

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

pub fn auto_spawn_system_label(
    mut commands: Commands,
    system_query: Query<Entity, (Added<crate::components::System>, Without<Subsystem>)>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
) {
    for entity in system_query.iter() {
        add_name_label(
            &mut commands,
            entity,
            vec2(100.0, 100.0),
            None,
            Some(CopyPositionArgs {
                offset: vec3(0.0, 0.0, 0.0),
                horizontal_alignment: Alignment::Center,
                vertical_alignment: Alignment::Center,
            }),
            &name_query,
            &asset_server,
            Some(AutoContrastTextColor::default()),
            NestingLevel::new(0),
        );
    }
}
