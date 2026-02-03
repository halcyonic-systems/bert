use crate::bevy_app::bundles::FixedSystemElementGeometry;
use crate::bevy_app::components::NestingLevel;
use crate::bevy_app::constants::{
    EXTERNAL_ENTITY_HEIGHT_HALF, EXTERNAL_ENTITY_WIDTH_HALF, FLOW_CLICK_WIDTH,
    INTERFACE_HEIGHT_HALF, INTERFACE_LINE_WIDTH, INTERFACE_WIDTH_HALF,
};
use crate::bevy_app::resources::StrokeTessellator;
use crate::bevy_app::systems::tessellate_simplified_mesh;
use bevy::camera::primitives::Aabb;
use bevy::math::{vec2, Vec3A};
use bevy::picking::mesh_picking::ray_cast::SimplifiedMesh;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Deref, DerefMut, Default, Clone, Copy)]
pub struct IsSameAsIdCounter(pub usize);

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
    let shape = build_external_entity_shape(scale);

    FixedSystemElementGeometry {
        simplified: SimplifiedMesh(tessellate_simplified_mesh(&shape, meshes, tess)),
        shape,
        aabb: Aabb {
            center: Vec3A::ZERO,
            half_extents: build_external_entity_aabb_half_extents(scale),
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

pub fn build_external_entity_shape(scale: f32) -> Shape {
    let width_half = EXTERNAL_ENTITY_WIDTH_HALF * scale;
    let height_half = EXTERNAL_ENTITY_HEIGHT_HALF * scale;

    let path = ShapePath::new()
        .move_to(vec2(width_half, height_half))
        .line_to(vec2(-width_half, height_half))
        .line_to(vec2(-width_half, -height_half))
        .line_to(vec2(width_half, -height_half));

    ShapeBuilder::with(&path).fill(Color::NONE).build()
}

fn init_interface_geometry(
    meshes: &mut ResMut<Assets<Mesh>>,
    scale: f32,
) -> FixedSystemElementGeometry {
    FixedSystemElementGeometry {
        simplified: SimplifiedMesh(build_interface_simplified_mesh(meshes, scale)),
        shape: build_interface_shape(scale),
        aabb: Aabb {
            center: Vec3A::ZERO,
            half_extents: build_interface_aabb_half_extends(scale),
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

pub fn build_interface_shape(scale: f32) -> Shape {
    let interface_width_half = INTERFACE_WIDTH_HALF * scale;
    let interface_height_half = INTERFACE_HEIGHT_HALF * scale;

    ShapeBuilder::with(&shapes::RoundedPolygon {
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
    .fill(Color::NONE)
    .build()
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
