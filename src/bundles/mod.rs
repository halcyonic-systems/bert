mod spawn;

pub use spawn::*;

use crate::components::{System, *};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

const SYSTEM_DEFAULT_FILL_COLOR: Color = Color::DARK_GRAY;
const SYSTEM_DEFAULT_STROKE_COLOR: Color = Color::BLACK;
const SYSTEM_DEFAULT_STROKE_SIZE: f32 = 5.0;

#[derive(Bundle)]
pub struct SystemBundle {
    pub system: System,
    pub pickable_bundle: PickableBundle,
    pub scale_with_zoom: ScaleWithZoom,
    pub system_shape_bundle: ShapeBundle,
    pub fill: Fill,
    pub stroke: Stroke,
    pub zoom_independent_stroke_width: ZoomIndependentStrokeWidth,
    pub initial_position: InitialPosition,
}

impl SystemBundle {
    pub fn new(position: Vec2, z: f32, radius: f32) -> Self {
        Self {
            system: System { radius },
            pickable_bundle: PickableBundle::default(),
            scale_with_zoom: ScaleWithZoom::default(),
            system_shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(position.extend(z)),
                    ..default()
                },
                ..default()
            },
            fill: Fill::color(SYSTEM_DEFAULT_FILL_COLOR),
            stroke: Stroke::new(SYSTEM_DEFAULT_STROKE_COLOR, SYSTEM_DEFAULT_STROKE_SIZE),
            zoom_independent_stroke_width: ZoomIndependentStrokeWidth::new(
                SYSTEM_DEFAULT_STROKE_SIZE,
            ),
            initial_position: InitialPosition::new(position),
        }
    }
}
