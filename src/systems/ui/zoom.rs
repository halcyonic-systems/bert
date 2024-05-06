use crate::bundles::{aabb_from_radius, get_system_geometry_from_radius};
use crate::components::*;
use crate::constants::EXTERNAL_ENTITY_LINE_WIDTH;
use crate::plugins::lyon_selection::HighlightBundles;
use crate::resources::{
    build_external_entity_aabb_half_extents, build_external_entity_path,
    build_interface_aabb_half_extends, build_interface_path, build_interface_simplified_mesh,
    FixedSystemElementGeometriesByNestingLevel, StrokeTessellator, Zoom,
};
use crate::systems::tessellate_simplified_mesh;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::vec3;
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

        simplified_mesh.mesh = meshes.add(mesh);
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
        zoom.mul(1.2);
    }

    if input.just_pressed(KeyCode::Equal) {
        zoom.mul(0.8);
    }
}

pub fn control_zoom_from_mouse_wheel(
    mut scroll_events: EventReader<MouseWheel>,
    mut zoom: ResMut<Zoom>,
) {
    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                zoom.mul(1.0 + event.y * 0.01);
            }
            MouseScrollUnit::Pixel => {
                zoom.mul(1.0 + event.y * 0.001);
            }
        }
    }
}

pub fn apply_zoom_to_system_geometries(
    mut external_entity_query: Query<
        (&NestingLevel, &mut Path, &mut SimplifiedMesh, &mut Aabb),
        With<ExternalEntity>,
    >,
    mut interface_query: Query<
        (&NestingLevel, &mut Path, &mut SimplifiedMesh, &mut Aabb),
        (With<Interface>, Without<ExternalEntity>),
    >,
    zoom: Res<Zoom>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tess: ResMut<StrokeTessellator>,
) {
    let zoom = **zoom;

    for (nesting_level, geometries) in &mut **fixed_system_element_geometries {
        let scale = NestingLevel::compute_scale(*nesting_level, zoom);

        let external_entity_path = build_external_entity_path(scale);
        geometries.external_entity.simplified.mesh =
            tessellate_simplified_mesh(&external_entity_path, &mut meshes, &mut tess);
        geometries.external_entity.path = external_entity_path;
        geometries.external_entity.aabb.half_extents =
            build_external_entity_aabb_half_extents(scale);

        geometries.interface.simplified.mesh = build_interface_simplified_mesh(&mut meshes, scale);
        geometries.interface.path = build_interface_path(scale);
        geometries.interface.aabb.half_extents = build_interface_aabb_half_extends(scale);
    }

    for (nesting_level, mut path, mut simplified_mesh, mut aabb) in &mut external_entity_query {
        let geometries = fixed_system_element_geometries
            .get(&**nesting_level)
            .expect("Geometries have been added in spawn_external_entity");

        let geometry = geometries.external_entity.clone();
        *path = geometry.path;
        simplified_mesh.mesh = geometry.simplified.mesh;
        aabb.half_extents = geometry.aabb.half_extents;
    }

    for (nesting_level, mut path, mut simplified_mesh, mut aabb) in &mut interface_query {
        let geometries = fixed_system_element_geometries
            .get(&**nesting_level)
            .expect("Geometries have been added in spawn_interface");

        let geometry = geometries.interface.clone();
        *path = geometry.path;
        simplified_mesh.mesh = geometry.simplified.mesh;
        aabb.half_extents = geometry.aabb.half_extents;
    }
}

pub fn apply_zoom_to_strokes(
    mut highlight_query: Query<(&NestingLevel, &mut HighlightBundles<Stroke, Stroke>)>,
    mut stroke_query: Query<
        (&NestingLevel, &mut Stroke),
        Without<HighlightBundles<Stroke, Stroke>>,
    >,
    zoom: Res<Zoom>,
) {
    for (nesting_level, mut highlight) in &mut highlight_query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);
        highlight.idle.options.line_width = scale * EXTERNAL_ENTITY_LINE_WIDTH;
        // TODO : this assumes only one line width which is the case right now
        // highlight.selected.options.line_width = (scale * EXTERNAL_ENTITY_SELECTED_LINE_WIDTH);
    }

    for (nesting_level, mut stroke) in &mut stroke_query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);
        let line_width = scale * EXTERNAL_ENTITY_LINE_WIDTH;
        stroke.options.line_width = line_width;
    }
}

pub fn apply_zoom_to_scale(
    mut query: Query<(&mut Transform, &NestingLevel), With<ApplyZoomToScale>>,
    zoom: Res<Zoom>,
) {
    for (mut transform, nesting_level) in &mut query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);
        transform.scale = vec3(scale, scale, 1.0);
    }
}
