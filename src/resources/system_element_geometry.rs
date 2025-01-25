use crate::bundles::FixedSystemElementGeometry;
use crate::components::NestingLevel;
use crate::constants::{
    EXTERNAL_ENTITY_HEIGHT_HALF, EXTERNAL_ENTITY_WIDTH_HALF, FLOW_CLICK_WIDTH,
    INTERFACE_HEIGHT_HALF, INTERFACE_LINE_WIDTH, INTERFACE_WIDTH_HALF, WHITE_COLOR_MATERIAL_HANDLE,
};
use crate::resources::StrokeTessellator;
use crate::systems::tessellate_simplified_mesh;
use bevy::math::{vec2, Vec3A};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::utils::HashMap;
use bevy_picking::mesh_picking::ray_cast::SimplifiedMesh;
use bevy_prototype_lyon::prelude::*;

#[derive(Clone)]
pub struct FixedSystemElementGeometries {
    pub interface: FixedSystemElementGeometry,
    pub external_entity: FixedSystemElementGeometry,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct FixedSystemElementGeometriesByNestingLevel(HashMap<u16, FixedSystemElementGeometries>);

impl FixedSystemElementGeometriesByNestingLevel {
    pub fn get_or_create(
        &mut self,
        nesting_level: u16,
        zoom: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        tess: &mut ResMut<StrokeTessellator>,
    ) -> FixedSystemElementGeometries {
        let scale = NestingLevel::compute_scale(nesting_level, zoom);

        self.0
            .entry(nesting_level)
            .or_insert_with(|| FixedSystemElementGeometries {
                interface: init_interface_geometry(meshes, scale),
                external_entity: init_external_entity_geometry(meshes, tess, scale),
            })
            .clone()
    }
}

fn init_external_entity_geometry(
    meshes: &mut ResMut<Assets<Mesh>>,
    tess: &mut ResMut<StrokeTessellator>,
    scale: f32,
) -> FixedSystemElementGeometry {
    let path = build_external_entity_path(scale);

    FixedSystemElementGeometry {
        simplified: SimplifiedMesh(tessellate_simplified_mesh(&path, meshes, tess)),
        path,
        mesh: Default::default(),
        material: WHITE_COLOR_MATERIAL_HANDLE,
        aabb: Aabb {
            half_extents: build_external_entity_aabb_half_extents(scale),
            ..default()
        },
    }
}

pub fn build_external_entity_aabb_half_extents(scale: f32) -> Vec3A {
    Vec3A::new(
        EXTERNAL_ENTITY_WIDTH_HALF * scale + FLOW_CLICK_WIDTH * 0.5,
        EXTERNAL_ENTITY_HEIGHT_HALF * scale + FLOW_CLICK_WIDTH * 0.5,
        0.0,
    )
}

pub fn build_external_entity_path(scale: f32) -> Path {
    let mut external_entity_path_builder = PathBuilder::new();

    let width_half = EXTERNAL_ENTITY_WIDTH_HALF * scale;
    let height_half = EXTERNAL_ENTITY_HEIGHT_HALF * scale;

    external_entity_path_builder.move_to(vec2(width_half, height_half));
    external_entity_path_builder.line_to(vec2(-width_half, height_half));
    external_entity_path_builder.line_to(vec2(-width_half, -height_half));
    external_entity_path_builder.line_to(vec2(width_half, -height_half));

    external_entity_path_builder.build()
}

fn init_interface_geometry(
    meshes: &mut ResMut<Assets<Mesh>>,
    scale: f32,
) -> FixedSystemElementGeometry {
    FixedSystemElementGeometry {
        simplified: SimplifiedMesh(build_interface_simplified_mesh(meshes, scale)),
        path: build_interface_path(scale),
        mesh: Default::default(),
        material: WHITE_COLOR_MATERIAL_HANDLE,
        aabb: Aabb {
            half_extents: build_interface_aabb_half_extends(scale),
            ..default()
        },
    }
}

pub fn build_interface_aabb_half_extends(scale: f32) -> Vec3A {
    Vec3A::new(
        (INTERFACE_WIDTH_HALF + INTERFACE_LINE_WIDTH) * scale,
        (INTERFACE_HEIGHT_HALF + INTERFACE_LINE_WIDTH) * scale,
        0.0,
    )
}

pub fn build_interface_path(scale: f32) -> Path {
    let interface_width_half = INTERFACE_WIDTH_HALF * scale;
    let interface_height_half = INTERFACE_HEIGHT_HALF * scale;

    GeometryBuilder::build_as(&shapes::RoundedPolygon {
        points: [
            vec2(interface_width_half, interface_height_half), // top right
            vec2(-interface_width_half, interface_height_half), // top left
            vec2(-interface_width_half, -interface_height_half), // bottom left
            vec2(interface_width_half, -interface_height_half), // bottom right
        ]
        .into_iter()
        .collect(),
        radius: 5.0 * scale,
        closed: false,
    })
}

pub fn build_interface_simplified_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    scale: f32,
) -> Handle<Mesh> {
    meshes.add(Rectangle::new(
        (INTERFACE_WIDTH_HALF + INTERFACE_LINE_WIDTH) * scale * 2.0,
        (INTERFACE_HEIGHT_HALF + INTERFACE_LINE_WIDTH) * scale * 2.0,
    ))
}
