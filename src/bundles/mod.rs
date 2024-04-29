mod spawn;

use bevy::math::vec3;
pub use spawn::*;

use crate::components::{System, *};
use bevy::prelude::*;
use bevy::render::mesh::CircleMeshBuilder;
use bevy::render::primitives::Aabb;
use bevy::sprite::Mesh2dHandle;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

const SYSTEM_DEFAULT_FILL_COLOR: Color = Color::GRAY;
const SYSTEM_DEFAULT_STROKE_COLOR: Color = Color::BLACK;
const SYSTEM_DEFAULT_STROKE_SIZE: f32 = 5.0;

pub fn get_system_geometry_from_radius(radius: f32) -> (Mesh, Path) {
    (
        CircleMeshBuilder::new(radius, 16).build(),
        GeometryBuilder::build_as(&shapes::Circle {
            radius,
            ..default()
        }),
    )
}

#[derive(Bundle)]
pub struct SystemBundle {
    pub system: System,
    pub name: Name,
    pub pickable_bundle: PickableBundle,
    pub simplified_mesh: SimplifiedMesh,
    pub aabb: Aabb,
    pub system_shape_bundle: ShapeBundle,
    pub fill: Fill,
    pub stroke: Stroke,
    pub initial_position: InitialPosition,
}

impl SystemBundle {
    pub fn new(
        position: Vec2,
        z: f32,
        radius: f32,
        angle: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        let (simplified_mesh, path) = get_system_geometry_from_radius(radius);

        Self {
            system: System { radius },
            name: Name::new("System"),
            pickable_bundle: PickableBundle::default(),
            simplified_mesh: SimplifiedMesh {
                mesh: meshes.add(simplified_mesh).into(),
            },
            aabb: Aabb::from_min_max(vec3(-radius, -radius, 0.0), vec3(radius, radius, 0.0)),
            system_shape_bundle: ShapeBundle {
                path,
                spatial: SpatialBundle {
                    transform: Transform::from_translation(position.extend(z))
                        .with_rotation(Quat::from_rotation_z(angle))
                        .with_scale(vec3(1.0, 1.0, 0.9)),
                    ..default()
                },
                ..default()
            },
            fill: Fill::color(SYSTEM_DEFAULT_FILL_COLOR),
            stroke: Stroke::new(SYSTEM_DEFAULT_STROKE_COLOR, SYSTEM_DEFAULT_STROKE_SIZE),
            initial_position: InitialPosition::new(position),
        }
    }
}

#[derive(Bundle)]
pub struct FixedSystemElementGeometry {
    pub simplified: SimplifiedMesh,
    pub path: Path,
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
    pub aabb: Aabb,
}

impl Clone for FixedSystemElementGeometry {
    fn clone(&self) -> Self {
        Self {
            simplified: SimplifiedMesh {
                mesh: self.simplified.mesh.clone(),
            },
            path: Path(self.path.0.clone()),
            mesh: self.mesh.clone(),
            material: self.material.clone(),
            aabb: self.aabb.clone(),
        }
    }
}
