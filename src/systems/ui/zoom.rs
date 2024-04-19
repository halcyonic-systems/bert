use crate::components::{FlowCurve, InitialPosition, ScaleWithZoom, ZoomIndependentStrokeWidth};
use crate::resources::{StrokeTessellator, Zoom};
use crate::systems::update_flow_curve;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn apply_zoom(
    mut query: Query<(&mut Transform, Option<&ScaleWithZoom>, &InitialPosition), Without<Camera>>,
    zoom: Res<Zoom>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (mut transform, scale_with_zoom, initial_position) in &mut query {
        transform.translation = (**initial_position * **zoom).extend(transform.translation.z);

        if let Some(scale_with_zoom) = scale_with_zoom {
            transform.scale = (**scale_with_zoom * **zoom).extend(transform.scale.z);
        }
    }
}

pub fn apply_zoom_to_stroke(
    mut query: Query<(&mut Stroke, &ZoomIndependentStrokeWidth)>,
    zoom: Res<Zoom>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (mut stroke, width) in &mut query {
        stroke.options.line_width = **width / **zoom;
    }
}

pub fn apply_zoom_to_flow_curve(
    mut query: Query<(
        &FlowCurve,
        &mut Path,
        &mut SimplifiedMesh,
        &mut Aabb,
        &Children,
    )>,
    mut path_query: Query<&mut Path, Without<FlowCurve>>,
    zoom: Res<Zoom>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (flow_curve, path, simplified_mesh, aabb, children) in &mut query {
        update_flow_curve(
            &mut path_query,
            flow_curve,
            path,
            simplified_mesh,
            aabb,
            children,
            **zoom,
            &mut stroke_tess,
            &mut meshes,
        );
    }
}
