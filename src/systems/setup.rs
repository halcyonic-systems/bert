use crate::bundles::{spawn_create_button, FixedSystemElementGeometry, SystemBundle};
use crate::components::{CreateButton, CreateButtonType};
use crate::constants::*;
use crate::resources::{FixedSystemElementGeometries, FocusedSystem, Zoom};
use bevy::math::{vec2, Vec3A};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_prototype_lyon::prelude::*;

const CLEAR_COLOR: Color = Color::ANTIQUE_WHITE;

pub fn setup(
    mut commands: Commands,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(CLEAR_COLOR));

    let radius = 300.0;

    let system_entity = commands
        .spawn(SystemBundle::new(Vec2::ZERO, 0.0, radius, &mut meshes))
        .id();

    commands.insert_resource(FocusedSystem::new(system_entity));
    commands.insert_resource(FixedSystemElementGeometries {
        interface: init_interface_geometry(&mut meshes),
        external_entity: init_external_entity_geometry(&mut meshes),
    });

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

fn init_external_entity_geometry(meshes: &mut ResMut<Assets<Mesh>>) -> FixedSystemElementGeometry {
    let mut external_entity_path_builder = PathBuilder::new();
    external_entity_path_builder.move_to(vec2(
        EXTERNAL_ENTITY_WIDTH_HALF,
        EXTERNAL_ENTITY_HEIGHT_HALF,
    ));
    external_entity_path_builder.line_to(vec2(
        -EXTERNAL_ENTITY_WIDTH_HALF,
        EXTERNAL_ENTITY_HEIGHT_HALF,
    ));
    external_entity_path_builder.line_to(vec2(
        -EXTERNAL_ENTITY_WIDTH_HALF,
        -EXTERNAL_ENTITY_HEIGHT_HALF,
    ));
    external_entity_path_builder.line_to(vec2(
        EXTERNAL_ENTITY_WIDTH_HALF,
        -EXTERNAL_ENTITY_HEIGHT_HALF,
    ));

    FixedSystemElementGeometry {
        simplified: SimplifiedMesh {
            mesh: meshes
                .add(Rectangle::new(
                    EXTERNAL_ENTITY_WIDTH_HALF * 2.0,
                    EXTERNAL_ENTITY_HEIGHT_HALF * 2.0,
                ))
                .into(),
        },
        path: external_entity_path_builder.build(),
        mesh: Default::default(),
        material: WHITE_COLOR_MATERIAL_HANDLE,
        aabb: Aabb {
            center: Vec3A::ZERO,
            half_extents: Vec3A::new(EXTERNAL_ENTITY_WIDTH_HALF, EXTERNAL_ENTITY_HEIGHT_HALF, 0.0),
        },
    }
}

fn init_interface_geometry(meshes: &mut ResMut<Assets<Mesh>>) -> FixedSystemElementGeometry {
    FixedSystemElementGeometry {
        simplified: SimplifiedMesh {
            mesh: meshes
                .add(Rectangle::new(
                    INTERFACE_WIDTH_HALF * 2.0,
                    INTERFACE_HEIGHT_HALF * 2.0,
                ))
                .into(),
        },
        path: GeometryBuilder::build_as(&shapes::RoundedPolygon {
            points: [
                vec2(INTERFACE_WIDTH_HALF, INTERFACE_HEIGHT_HALF), // top right
                vec2(-INTERFACE_WIDTH_HALF, INTERFACE_HEIGHT_HALF), // top left
                vec2(-INTERFACE_WIDTH_HALF, -INTERFACE_HEIGHT_HALF), // bottom left
                vec2(INTERFACE_WIDTH_HALF, -INTERFACE_HEIGHT_HALF), // bottom right
            ]
            .into_iter()
            .collect(),
            radius: 5.,
            closed: false,
        }),
        mesh: Default::default(),
        material: WHITE_COLOR_MATERIAL_HANDLE,
        aabb: Aabb {
            center: Vec3A::ZERO,
            half_extents: Vec3A::new(INTERFACE_WIDTH_HALF, INTERFACE_HEIGHT_HALF, 0.0),
        },
    }
}
