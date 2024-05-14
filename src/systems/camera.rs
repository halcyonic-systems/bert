use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::math::vec3;
use bevy::prelude::*;

pub fn pan_camera_with_mouse(
    mut motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for event in motion_events.read() {
        camera_query.single_mut().translation += Vec3::new(-event.delta.x, event.delta.y, 0.0);
    }
}

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

pub fn reset_camera_position(
    mut camera_query: Query<&mut Transform, With<Camera>>
) {
    camera_query.single_mut().translation = Vec3::new(0.0, 0.0, 1.000);   
}
