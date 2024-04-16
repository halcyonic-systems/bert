use crate::bundles::spawn_create_button;
use crate::components::{CreateButton, CreateButtonType, System, SystemElement};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::prelude::*;
use crate::resources::FocusedSystem;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let system_entity = commands
        .spawn((
            System,
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default().with_scale(Vec3::splat(256.)),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            },
            PickableBundle::default(),
            SystemElement::System,
        ))
        .id();

    commands.insert_resource(FocusedSystem::new(system_entity));

    spawn_create_button(
        &mut commands,
        CreateButton {
            ty: CreateButtonType::Outflow,
            connection_source: system_entity,
            system: system_entity,
        },
        vec2(128.0, 100.0),
        0.0,
        &asset_server,
    );
}
