//! Holds all systems for controlling the camera with user input.
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::math::vec3;
use bevy::prelude::*;

/// Allows user to move around the canvas in all directions.
/// Adjusts the Camera's transform based on the Mouse location.
pub fn pan_camera_with_mouse(
    mut motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for event in motion_events.read() {
        camera_query.single_mut().translation += Vec3::new(-event.delta.x, event.delta.y, 0.0);
    }
}

/// Allows user to move around the canvas vertically or horizontally.
/// Adjusts the Camera's transform based on the Mouse Scroll Wheel input.
pub fn pan_camera_with_mouse_wheel(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for event in scroll_events.read() {
        let delta = vec3(-event.x, event.y, 0.0);

        camera_query.single_mut().translation += match event.unit {
            MouseScrollUnit::Line => delta * 10.0,
            MouseScrollUnit::Pixel => delta,
        };
    }
}

/// Allows user to reset the camera's location to the center of the canvas.
pub fn reset_camera_position(mut camera_query: Query<&mut Transform, With<Camera>>) {
    camera_query.single_mut().translation = Vec3::new(0.0, 0.0, 1000.);
}
