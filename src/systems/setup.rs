use crate::bundles::spawn_create_button;
use crate::components::{CreateButton, CreateButtonType, ScaleWithZoom, System, SystemElement};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::prelude::*;
use crate::resources::FocusedSystem;
use bevy_prototype_lyon::{shapes, entity::ShapeBundle, geometry::GeometryBuilder, draw::{Fill, Stroke}};

const CLEAR_COLOR: Color = Color::ANTIQUE_WHITE;
const SOI_DEFAULT_FILL_COLOR: Color = Color::DARK_GRAY;
const SOI_DEFAULT_STROKE_COLOR: Color = Color::BLACK;
const SOI_DEFAULT_STROKE_SIZE: f32 = 5.0;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(CLEAR_COLOR));
    
    // SPAWN SOI SYSTEM
    let system_entity = commands
        .spawn((
            System,
            PickableBundle::default(),
            ScaleWithZoom,
        ))
        .id();
    
    // DRAW SOI SYSTEM
    let system_shape = shapes::Circle {
        radius: 300.0,
        center: vec2(0.0, 0.0),
    };
    let system_shape_bundle = ShapeBundle {
        path: GeometryBuilder::build_as(&system_shape),
        ..default()
    };
    commands.entity(system_entity)
        .insert(system_shape_bundle)
        .insert(Fill::color(SOI_DEFAULT_FILL_COLOR))
        .insert(Stroke::new(SOI_DEFAULT_STROKE_COLOR, SOI_DEFAULT_STROKE_SIZE));

    commands.insert_resource(FocusedSystem::new(system_entity));
    
    // ADD OUTFLOW BUTTON TO RIGHT SIDE OF SYSTEM
    let button_position = vec2(
        system_shape.center.x + system_shape.radius,
        system_shape.center.y
    );
    let button_angle = (button_position - system_shape.center).to_angle();
    spawn_create_button(
        &mut commands,
        CreateButton {
            ty: CreateButtonType::Outflow,
            connection_source: system_entity,
            system: system_entity,
        },
        button_position,
        button_angle,
        &asset_server,
    );
}
