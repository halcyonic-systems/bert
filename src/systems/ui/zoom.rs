use crate::bundles::{aabb_from_radius, get_system_geometry_from_radius};
use crate::components::*;
use crate::resources::Zoom;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn apply_zoom(
    mut query: Query<(&mut Transform, &InitialPosition), Without<Camera>>,
    zoom: Res<Zoom>,
) {
    for (mut transform, initial_position) in &mut query {
        transform.translation = (**initial_position * **zoom).extend(transform.translation.z);
    }
}

pub fn apply_zoom_to_system_radii(
    mut query: Query<(
        &mut SimplifiedMesh,
        &mut Path,
        &mut Aabb,
        &crate::components::System,
    )>,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut simplified_mesh, mut path, mut aabb, system) in &mut query {
        let zoomed_radius = system.radius * **zoom;

        let (mesh, p) = get_system_geometry_from_radius(zoomed_radius);

        simplified_mesh.mesh = meshes.add(mesh).into();
        *path = p;

        *aabb = aabb_from_radius(zoomed_radius);
    }
}

pub fn apply_zoom_to_camera_position(
    mut query: Query<&mut Transform, With<Camera>>,
    zoom: Res<Zoom>,
    mut prev_zoom: Local<Zoom>,
) {
    query.single_mut().translation *= **zoom / **prev_zoom;

    **prev_zoom = **zoom;
}

pub fn apply_zoom_to_incomplete_flows(
    mut inflow_query: Query<
        (&mut FlowCurve, Option<&InflowInterfaceConnection>),
        (With<Inflow>, Without<InflowSourceConnection>),
    >,
    mut outflow_query: Query<
        (&mut FlowCurve, Option<&OutflowInterfaceConnection>),
        (
            With<Outflow>,
            Without<OutflowSinkConnection>,
            Without<Inflow>,
        ),
    >,
    zoom: Res<Zoom>,
    mut prev_zoom: Local<Zoom>,
) {
    for (mut flow_curve, inflow_interface_connection) in &mut inflow_query {
        flow_curve.start *= **zoom / **prev_zoom;

        if inflow_interface_connection.is_none() {
            flow_curve.end *= **zoom / **prev_zoom;
        }
    }

    for (mut flow_curve, outflow_interface_connection) in &mut outflow_query {
        flow_curve.end *= **zoom / **prev_zoom;

        if outflow_interface_connection.is_none() {
            flow_curve.start *= **zoom / **prev_zoom;
        }
    }

    **prev_zoom = **zoom;
}

pub fn control_zoom_from_keyboard(input: Res<ButtonInput<KeyCode>>, mut zoom: ResMut<Zoom>) {
    if input.just_pressed(KeyCode::Minus) {
        zoom.add(0.2);
    }

    if input.just_pressed(KeyCode::Equal) {
        zoom.add(-0.2);
    }
}

pub fn control_zoom_from_mouse_wheel(
    mut scroll_events: EventReader<MouseWheel>,
    mut zoom: ResMut<Zoom>,
) {
    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                zoom.add(event.y * 0.01);
            }
            MouseScrollUnit::Pixel => {
                zoom.add(event.y * 0.001);
            }
        }
    }
}
