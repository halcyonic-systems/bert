use crate::bundles::{spawn_create_button, SystemBundle};
use crate::components::{CreateButton, CreateButtonType};
use crate::resources::{FocusedSystem, Zoom};
use bevy::math::vec2;
use bevy::prelude::*;

const CLEAR_COLOR: Color = Color::ANTIQUE_WHITE;

pub fn setup(mut commands: Commands, zoom: Res<Zoom>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(CLEAR_COLOR));

    let radius = 300.0;

    let system_entity = commands
        .spawn(SystemBundle::new(Vec2::ZERO, 0.0, radius))
        .id();

    commands.insert_resource(FocusedSystem::new(system_entity));

    spawn_create_button(
        &mut commands,
        CreateButton {
            ty: CreateButtonType::Outflow,
            connection_source: system_entity,
            system: system_entity,
        },
        vec2(radius, 0.0),
        0.0,
        **zoom,
        &asset_server,
    );
}
