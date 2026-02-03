mod spawn;

use bevy::math::vec3;
pub use spawn::*;

use crate::bevy_app::components::{System, *};
use crate::bevy_app::constants::{SYSTEM_LINE_WIDTH, SYSTEM_SELECTED_LINE_WIDTH};
use crate::bevy_app::data_model::Complexity;
use crate::bevy_app::plugins::lyon_selection::HighlightBundles;
use crate::bevy_app::plugins::mouse_interaction::PickSelection;
use bevy::camera::primitives::Aabb;
use bevy::mesh::CircleMeshBuilder;
use bevy::picking::mesh_picking::ray_cast::SimplifiedMesh;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn get_system_geometry_from_radius(radius: f32) -> (Mesh, Shape) {
    (
        CircleMeshBuilder::new(radius, 16).build(),
        ShapeBuilder::with(&shapes::Circle {
            radius,
            ..default()
        })
        .fill(Color::NONE)
        .build(),
    )
}

pub fn aabb_from_radius(radius: f32) -> Aabb {
    Aabb::from_min_max(vec3(-radius, -radius, 0.0), vec3(radius, radius, 0.0))
}

#[derive(Bundle)]
pub struct SystemBundle {
    pub system: System,
    pub name: Name,
    pub description: ElementDescription,
    pub system_element: SystemElement,
    pub pick_selection: PickSelection,
    pub simplified_mesh: SimplifiedMesh,
    pub aabb: Aabb,
    pub shape: Shape,
    pub transform: Transform,
    pub highlight: HighlightBundles,
    pub initial_position: InitialPosition,
}

impl SystemBundle {
    pub fn new(
        position: Vec2,
        z: f32,
        radius: f32,
        angle: f32,
        complexity: Complexity,
        boundary: SystemBoundary,
        meshes: &mut ResMut<Assets<Mesh>>,
        zoom: f32,
        nesting_level: u16,
        name: &str,
        description: &str,
        equivalence: &str,
        time_unit: &str,
        archetype: HcgsArchetype,
    ) -> Self {
        let zoomed_radius = radius * zoom;

        let (simplified_mesh, shape_geom) = get_system_geometry_from_radius(zoomed_radius);
        let scale = NestingLevel::compute_scale(nesting_level, zoom);

        // Build the Shape with fill; stroke is managed by HighlightBundles
        let mut shape = shape_geom;
        shape.fill = Some(Fill::color(Color::srgb_u8(41, 51, 64)));

        Self {
            system: System {
                radius,
                complexity,
                membership: 1.0,
                equivalence: equivalence.to_string(),
                transformation: "".to_string(),
                history: "".to_string(),
                boundary,
                time_unit: time_unit.to_string(),
                archetype,
            },
            name: Name::new(name.to_string()),
            description: ElementDescription::new(description),
            system_element: SystemElement::System,
            pick_selection: PickSelection::default(),
            simplified_mesh: SimplifiedMesh(meshes.add(simplified_mesh)),
            aabb: aabb_from_radius(zoomed_radius),
            shape,
            transform: Transform::from_translation(position.extend(z))
                .with_rotation(Quat::from_rotation_z(angle))
                .with_scale(vec3(1.0, 1.0, 0.9)),
            highlight: HighlightBundles {
                idle_stroke: Some(Stroke::new(Color::BLACK, SYSTEM_LINE_WIDTH * scale)),
                selected_stroke: Some(Stroke::new(Color::BLACK, SYSTEM_SELECTED_LINE_WIDTH)),
                idle_fill: None,
                selected_fill: None,
            },
            initial_position: InitialPosition::new(position),
        }
    }
}

#[derive(Bundle)]
pub struct FixedSystemElementGeometry {
    pub simplified: SimplifiedMesh,
    pub shape: Shape,
    pub aabb: Aabb,
}

impl Clone for FixedSystemElementGeometry {
    fn clone(&self) -> Self {
        Self {
            simplified: self.simplified.clone(),
            shape: self.shape.clone(),
            aabb: self.aabb,
        }
    }
}
