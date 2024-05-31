//! Systems that manipulate the geometry of the diagram.
//! See design/Geometry_High-Level_Overview.pdf for a big picture overview.

use crate::bundles::{
    aabb_from_radius, get_system_geometry_from_radius, FixedSystemElementGeometry,
};
use crate::components::*;
use crate::constants::{
    EXTERNAL_ENTITY_LINE_WIDTH, LABEL_SCALE_VISIBILITY_THRESHOLD, SCALE_VISIBILITY_THRESHOLD,
};
use crate::plugins::label::LabelContainer;
use crate::plugins::lyon_selection::HighlightBundles;
use crate::resources::{
    build_external_entity_aabb_half_extents, build_external_entity_path,
    build_interface_aabb_half_extends, build_interface_path, build_interface_simplified_mesh,
    FixedSystemElementGeometriesByNestingLevel, StrokeTessellator, Zoom,
};
use crate::systems::tessellate_simplified_mesh;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_prototype_lyon::prelude::*;

/// Applies the current zoom value to the x and y translations of all non-camera entities.
/// The z component of the translation remains unchanged.
pub fn apply_zoom(
    mut query: Query<(&mut Transform, &InitialPosition), Without<Camera>>,
    zoom: Res<Zoom>,
) {
    for (mut transform, initial_position) in &mut query {
        transform.translation = (**initial_position * **zoom).extend(transform.translation.z);
    }
}

/// Adjusts the size of system entities according to the current zoom level.
///
/// This function ensures that only system entities change size by drawing a circle
/// with its base radius multiplied by the current zoom value. Note that the transform's
/// scale is not modified in this process.
pub fn apply_zoom_to_system_radii(
    changed_query: Query<(), Changed<crate::components::System>>,
    mut query: Query<(
        &mut SimplifiedMesh,
        &mut Path,
        &mut Aabb,
        &crate::components::System,
        Option<&SelectedHighlightHelperAdded>,
    )>,
    mut child_query: Query<&mut Path, Without<crate::components::System>>,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if changed_query.is_empty() && !zoom.is_changed() {
        return;
    }

    for (mut simplified_mesh, mut path, mut aabb, system, helper) in &mut query {
        let zoomed_radius = system.radius * **zoom;

        let (mesh, p) = get_system_geometry_from_radius(zoomed_radius);

        simplified_mesh.mesh = meshes.add(mesh);
        *path = Path(p.0.clone());

        *aabb = aabb_from_radius(zoomed_radius);

        if let Some(helper) = helper {
            let mut child_path = child_query
                .get_mut(helper.helper_entity)
                .expect("Helper entity should exist");
            *child_path = p;
        }
    }
}

/// Moves the camera to always be centered on the same point relative to the world entities.
pub fn apply_zoom_to_camera_position(
    mut query: Query<&mut Transform, With<Camera>>,
    zoom: Res<Zoom>,
    mut prev_zoom: Local<Zoom>,
) {
    query.single_mut().translation *= **zoom / **prev_zoom;

    **prev_zoom = **zoom;
}

/// Adjusts the position of flow endpoints that are missing interface connections,
/// when the flow is also missing a start or end connection.
pub fn apply_zoom_to_incomplete_flows(
    mut flow_query: Query<
        (
            &mut FlowCurve,
            Option<&FlowStartInterfaceConnection>,
            Option<&FlowEndInterfaceConnection>,
        ),
        Or<(Without<FlowStartConnection>, Without<FlowEndConnection>)>,
    >,
    zoom: Res<Zoom>,
    mut prev_zoom: Local<Zoom>,
) {
    for (mut flow_curve, flow_start_interface_connection, flow_end_interface_connection) in
        &mut flow_query
    {
        if flow_start_interface_connection.is_none() {
            flow_curve.start *= **zoom / **prev_zoom;
        }

        if flow_end_interface_connection.is_none() {
            flow_curve.end *= **zoom / **prev_zoom;
        }
    }

    **prev_zoom = **zoom;
}

/// Adjusts the position of the flow endpoints connected to a System,
/// when the flow lacks interface connections at both ends.
pub fn apply_zoom_to_flow_without_interface(
    mut no_interface_flow_query: Query<
        (&mut FlowCurve, &FlowStartConnection, &FlowEndConnection),
        (
            Without<FlowStartInterfaceConnection>,
            Without<FlowEndInterfaceConnection>,
        ),
    >,
    zoom: Res<Zoom>,
    mut prev_zoom: Local<Zoom>,
) {
    for (mut flow_curve, start_connection, end_connection) in &mut no_interface_flow_query {
        if matches!(start_connection.target_type, StartTargetType::System) {
            flow_curve.start *= **zoom / **prev_zoom;
        }

        if matches!(end_connection.target_type, EndTargetType::System) {
            flow_curve.end *= **zoom / **prev_zoom;
        }
    }

    **prev_zoom = **zoom;
}

/// Adjusts the zoom level based on keyboard input. 
/// 
/// Press the minus (-) key to zoom in, or press the equals (=) key to zoom out.
pub fn control_zoom_from_keyboard(input: Res<ButtonInput<KeyCode>>, mut zoom: ResMut<Zoom>) {
    if input.just_pressed(KeyCode::Minus) {
        zoom.mul(1.2);
    }

    if input.just_pressed(KeyCode::Equal) {
        zoom.mul(0.8);
    }
}

// TODO
// pub fn control_zoom_from_mouse_wheel(
//     mut scroll_events: EventReader<MouseWheel>,
//     mut zoom: ResMut<Zoom>,
// ) {
//     for event in scroll_events.read() {
//         match event.unit {
//             MouseScrollUnit::Line => {
//                 zoom.mul(1.0 + event.y * 0.01);
//             }
//             MouseScrollUnit::Pixel => {
//                 zoom.mul(1.0 + event.y * 0.001);
//             }
//         }
//     }
// }

//noinspection ALL
pub fn apply_zoom_to_system_geometries(
    external_entity_query: Query<
        (Entity, &NestingLevel, Option<&SelectedHighlightHelperAdded>),
        With<ExternalEntity>,
    >,
    interface_query: Query<
        (Entity, &NestingLevel, Option<&SelectedHighlightHelperAdded>),
        (With<Interface>, Without<ExternalEntity>),
    >,
    mut geometry_query: Query<(&mut Path, &mut SimplifiedMesh, &mut Aabb)>,
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

    macro_rules! apply_geometries {
        ($query:ident, $field:ident) => {
            for (entity, nesting_level, highlight_helper) in &$query {
                let geometries = fixed_system_element_geometries
                    .get(&**nesting_level)
                    .expect("Geometries have been added in spawn_external_entity");

                let geometry = &geometries.$field;

                apply_geometry(entity, geometry, &mut geometry_query);

                if let Some(highlight_helper) = highlight_helper {
                    apply_geometry(
                        highlight_helper.helper_entity,
                        &geometry,
                        &mut geometry_query,
                    );
                }
            }
        };
    }

    apply_geometries!(external_entity_query, external_entity);
    apply_geometries!(interface_query, interface);
}

fn apply_geometry(
    entity: Entity,
    geometry: &FixedSystemElementGeometry,
    geometry_query: &mut Query<(&mut Path, &mut SimplifiedMesh, &mut Aabb)>,
) {
    let (mut path, mut simplified_mesh, mut aabb) = geometry_query
        .get_mut(entity)
        .expect("Entity should have geometry");

    let geometry = geometry.clone();

    *path = geometry.path;
    simplified_mesh.mesh = geometry.simplified.mesh;
    aabb.half_extents = geometry.aabb.half_extents;
}

pub fn apply_zoom_to_strokes(
    mut highlight_query: Query<(
        &NestingLevel,
        &mut HighlightBundles<Stroke, Stroke>,
        &mut Visibility,
    )>,
    mut stroke_query: Query<
        (&NestingLevel, &mut Stroke, &mut Visibility),
        Without<HighlightBundles<Stroke, Stroke>>,
    >,
    zoom: Res<Zoom>,
) {
    for (nesting_level, mut highlight, mut visibility) in &mut highlight_query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);
        highlight.idle.options.line_width = scale * EXTERNAL_ENTITY_LINE_WIDTH;
        // TODO : this assumes only one line width which is the case right now
        // highlight.selected.options.line_width = (scale * EXTERNAL_ENTITY_SELECTED_LINE_WIDTH);

        apply_visibility(&mut visibility, scale, SCALE_VISIBILITY_THRESHOLD);
    }

    for (nesting_level, mut stroke, mut visibility) in &mut stroke_query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);
        let line_width = scale * EXTERNAL_ENTITY_LINE_WIDTH;
        stroke.options.line_width = line_width;

        apply_visibility(&mut visibility, scale, SCALE_VISIBILITY_THRESHOLD);
    }
}

pub fn apply_zoom_to_scale(
    mut query: Query<(&mut Transform, &mut Visibility, &NestingLevel), With<ApplyZoomToScale>>,
    zoom: Res<Zoom>,
) {
    for (mut transform, mut visibility, nesting_level) in &mut query {
        apply_scale_and_visibility(
            nesting_level,
            &mut transform,
            &mut visibility,
            **zoom,
            SCALE_VISIBILITY_THRESHOLD,
        );
    }
}

pub fn apply_zoom_to_label(
    mut query: Query<(&mut Transform, &mut Visibility, &NestingLevel), With<LabelContainer>>,
    zoom: Res<Zoom>,
) {
    for (mut transform, mut visibility, nesting_level) in &mut query {
        apply_scale_and_visibility(
            nesting_level,
            &mut transform,
            &mut visibility,
            **zoom,
            LABEL_SCALE_VISIBILITY_THRESHOLD,
        );
    }
}

fn apply_scale_and_visibility(
    nesting_level: &NestingLevel,
    transform: &mut Mut<Transform>,
    visibility: &mut Mut<Visibility>,
    zoom: f32,
    threshold: f32,
) {
    let scale = NestingLevel::compute_scale(**nesting_level, zoom);
    transform.scale = vec3(scale, scale, 1.0);

    apply_visibility(visibility, scale, threshold);
}

fn apply_visibility(visibility: &mut Mut<Visibility>, scale: f32, threshold: f32) {
    **visibility = if scale > threshold {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    }
}

pub fn apply_zoom_to_added_label(
    mut query: Query<(&mut Transform, &mut Visibility, &NestingLevel), Added<LabelContainer>>,
    zoom: Res<Zoom>,
) {
    for (mut transform, mut visibility, nesting_level) in &mut query {
        apply_scale_and_visibility(
            nesting_level,
            &mut transform,
            &mut visibility,
            **zoom,
            LABEL_SCALE_VISIBILITY_THRESHOLD,
        );
    }
}
