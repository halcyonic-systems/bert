use crate::bevy_app::bundles::SystemBundle;
use crate::bevy_app::bundles::spawn::spatial_interaction::spawn_system_with_spatial_regions;
use crate::bevy_app::components::{NestingLevel, Subsystem, SystemBoundary, SystemEnvironment};
use crate::bevy_app::constants::MAIN_SYSTEM_RADIUS;
use crate::bevy_app::data_model::Complexity;
use crate::bevy_app::plugins::label::{
    add_name_label, Alignment, AutoContrastTextColor, CopyPositionArgs,
};
use crate::plugins::label::{HorizontalAttachmentAnchor, VerticalAttachmentAnchor};
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
    equivalence: &str,
    time_unit: &str,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> Entity {
    let system_entity = commands
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
                equivalence,
                time_unit,
            ),
            SystemEnvironment::default(),
        ))
        .id();
    
    // Add spatial interaction regions for boundary and environment clicking
    let (_boundary_entity, _environment_entity) = spawn_system_with_spatial_regions(
        commands,
        system_entity,
        MAIN_SYSTEM_RADIUS,
        center,
    );
    
    system_entity
}

pub fn auto_spawn_system_label(
    mut commands: Commands,
    system_query: Query<
        Entity,
        (
            Added<crate::bevy_app::components::System>,
            Without<Subsystem>,
        ),
    >,
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
                horizontal_anchor: HorizontalAttachmentAnchor::default(),
                vertical_anchor: VerticalAttachmentAnchor::default(),
            }),
            false,
            &name_query,
            &asset_server,
            Some(AutoContrastTextColor::default()),
            NestingLevel::new(0),
        );
    }
}
