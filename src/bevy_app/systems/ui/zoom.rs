//! Systems that manipulate the geometry of the diagram.
//! See design/Geometry_High-Level_Overview.pdf for a big picture overview.

use crate::bevy_app::bundles::{
    aabb_from_radius, get_system_geometry_from_radius, FixedSystemElementGeometry,
};
use crate::bevy_app::components::*;
use crate::bevy_app::constants::{
    BUTTON_WIDTH_HALF, EXTERNAL_ENTITY_LINE_WIDTH, FLOW_LINE_WIDTH,
    LABEL_SCALE_VISIBILITY_THRESHOLD, SCALE_VISIBILITY_THRESHOLD,
};
use crate::bevy_app::plugins::label::LabelContainer;
use crate::bevy_app::plugins::lyon_selection::HighlightBundles;
use crate::bevy_app::resources::{
    build_external_entity_aabb_half_extents, build_external_entity_path,
    build_interface_aabb_half_extends, build_interface_path, build_interface_simplified_mesh,
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator, Zoom, ZoomTarget,
};
use crate::bevy_app::systems::tessellate_simplified_mesh;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_picking::mesh_picking::ray_cast::SimplifiedMesh;
use bevy_prototype_lyon::prelude::*;

/// Applies the current zoom value to the x and y translations of all non-camera entities.
/// The z component of the translation remains unchanged.
pub fn apply_zoom(
    mut query: Query<
        (&mut Transform, &InitialPosition),
        (Without<Camera>, Without<PaletteElement>),
    >,
    zoom: Res<Zoom>,
    time: Res<Time>,
) {
    // INTERFACE DRIFT QUICK FIX: Skip first 200ms after startup to ensure Bevy ECS fully initialized
    if time.elapsed_secs() < 0.2 {
        return;
    }

    // Additional safety: validate we have entities with InitialPosition
    let entity_count = query.iter().count();
    if entity_count == 0 {
        return; // No entities to zoom
    }

    if zoom.is_changed() {
        info!(
            "apply_zoom: Zoom changed to {} (applying to {} entities)",
            **zoom, entity_count
        );
    }

    for (mut transform, initial_position) in &mut query {
        // Validate initial_position is reasonable
        if initial_position.is_nan() || initial_position.length() > 10000.0 {
            error!("Invalid InitialPosition detected: {:?}", initial_position);
            continue;
        }

        transform.translation = (**initial_position * **zoom).extend(transform.translation.z);
    }
}

/// Adjusts the size of system entities according to the current zoom level.
///
/// This function ensures that only system entities change size by drawing a circle
/// with its base radius multiplied by the current zoom value. Note that the transform's
/// scale is not modified in this process.
pub fn apply_zoom_to_system_radii(
    changed_query: Query<(), Changed<crate::bevy_app::components::System>>,
    mut query: Query<(
        &mut SimplifiedMesh,
        &mut Path,
        &mut Aabb,
        &crate::bevy_app::components::System,
        Option<&SelectedHighlightHelperAdded>,
    )>,
    mut child_query: Query<&mut Path, Without<crate::bevy_app::components::System>>,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if changed_query.is_empty() && !zoom.is_changed() {
        return;
    }

    for (mut simplified_mesh, mut path, mut aabb, system, helper) in &mut query {
        let zoomed_radius = system.radius * **zoom;

        let (mesh, p) = get_system_geometry_from_radius(zoomed_radius);

        simplified_mesh.0 = meshes.add(mesh);
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
///
/// This maintains zoom centering by scaling camera position with zoom changes.
/// Note: Palette elements are counter-adjusted in `apply_zoom_to_palette_compensation`
/// to remain fixed in screen space despite camera position scaling.
pub fn apply_zoom_to_camera_position(
    mut query: Query<&mut Transform, With<Camera>>,
    zoom: Res<Zoom>,
    mut prev_zoom: Local<Zoom>,
) {
    query.single_mut().translation *= **zoom / **prev_zoom;

    **prev_zoom = **zoom;
}

/// Maintains palette position relative to camera to keep it screen-fixed.
///
/// Palette elements are in world-space but should appear screen-fixed (like UI).
/// This system tracks camera position changes and moves palette elements to compensate,
/// maintaining their fixed screen-space position despite camera panning and zoom scaling.
///
/// Without this: Camera pans to (500, 500) and palette at world (-550, 300) goes off-screen.
/// With this: Palette moves to (450, 800) in world space to stay at same screen position.
pub fn apply_zoom_to_palette_compensation(
    mut palette_query: Query<(&mut Transform, &InitialPosition), With<PaletteElement>>,
    camera_query: Query<&Transform, (With<Camera>, Without<PaletteElement>)>,
    mut prev_camera_pos: Local<Vec2>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    let camera_pos = camera_transform.translation.truncate();

    // Initialize on first run
    if *prev_camera_pos == Vec2::ZERO && camera_pos == Vec2::ZERO {
        *prev_camera_pos = camera_pos;
        return;
    }

    // Calculate camera movement delta
    let camera_delta = camera_pos - *prev_camera_pos;

    if camera_delta.length() > 0.01 {
        // Move palette by same delta to counteract camera pan
        for (mut transform, _initial_position) in &mut palette_query {
            transform.translation += camera_delta.extend(0.0);
        }
    }

    *prev_camera_pos = camera_pos;
}

/// Adjusts the position of flow endpoints that are not connected to anything.
pub fn apply_zoom_to_incomplete_flows(
    mut flow_query: Query<
        (
            &mut FlowCurve,
            Option<&FlowStartConnection>,
            Option<&FlowEndConnection>,
        ),
        Or<(Without<FlowStartConnection>, Without<FlowEndConnection>)>,
    >,
    zoom: Res<Zoom>,
    mut prev_zoom: Local<Zoom>,
) {
    for (mut flow_curve, flow_start_connection, flow_end_connection) in &mut flow_query {
        if flow_start_connection.is_none() {
            let button_offset = flow_curve.start_direction * BUTTON_WIDTH_HALF;
            flow_curve.start -= button_offset;
            flow_curve.start *= **zoom / **prev_zoom;
            flow_curve.start += button_offset;
        }

        if flow_end_connection.is_none() {
            let button_offset = flow_curve.end_direction * BUTTON_WIDTH_HALF;
            flow_curve.end -= button_offset;
            flow_curve.end *= **zoom / **prev_zoom;
            flow_curve.end += button_offset;
        }
    }

    **prev_zoom = **zoom;
}

/// Timer resource to throttle zoom operations
#[derive(Resource, Default)]
pub struct ZoomTimer(pub f32);

/// Adjusts the 'Zoom' level based on keyboard input.
///
/// Press the minus (-) key to zoom in, or press the equals (=) key to zoom out.
/// NOTE: Disabled for web builds to prevent dual zoom event handling.
#[cfg(not(target_arch = "wasm32"))]
pub fn control_zoom_from_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    mut zoom: ResMut<Zoom>,
    time: Res<Time>,
    mut zoom_timer: Local<ZoomTimer>,
) {
    // Update the timer
    zoom_timer.0 -= time.delta_secs();

    // Only process zoom if enough time has passed (throttle to prevent stuck keys)
    if zoom_timer.0 > 0.0 {
        return;
    }

    // Check for zoom in (minus key or numpad minus) - using pressed() instead of just_pressed()
    if input.pressed(KeyCode::Minus) || input.pressed(KeyCode::NumpadSubtract) {
        info!(
            "Zoom in key pressed, current zoom: {}, new zoom: {}",
            **zoom,
            **zoom * 1.2
        );
        zoom.mul(1.2);
        zoom.set_changed();
        zoom_timer.0 = 0.1; // 100ms cooldown
    }

    // Check for zoom out (equal key, plus key, or numpad plus)
    if input.pressed(KeyCode::Equal) || input.pressed(KeyCode::NumpadAdd) {
        info!(
            "Zoom out key pressed, current zoom: {}, new zoom: {}",
            **zoom,
            **zoom * 0.8
        );
        zoom.mul(0.8);
        zoom.set_changed();
        zoom_timer.0 = 0.1; // 100ms cooldown
    }
}

/// Stub version for web builds - zoom handled by JavaScript/Leptos instead
#[cfg(target_arch = "wasm32")]
pub fn control_zoom_from_keyboard(
    _input: Res<ButtonInput<KeyCode>>,
    _zoom: ResMut<Zoom>,
    _time: Res<Time>,
    _zoom_timer: Local<ZoomTimer>,
) {
    // No-op: Let Leptos JavaScript handler manage zoom on web to prevent dual events
}

/// Handles zoom events sent from JavaScript/Leptos
pub fn handle_zoom_events(
    mut zoom_events: EventReader<crate::bevy_app::events::ZoomEvent>,
    mut zoom: ResMut<Zoom>,
) {
    for event in zoom_events.read() {
        match event {
            crate::bevy_app::events::ZoomEvent::ZoomIn => {
                info!(
                    "JavaScript zoom in event received, current zoom: {}, new zoom: {}",
                    **zoom,
                    **zoom * 1.2
                );
                zoom.mul(1.2);
                zoom.set_changed();
            }
            crate::bevy_app::events::ZoomEvent::ZoomOut => {
                info!(
                    "JavaScript zoom out event received, current zoom: {}, new zoom: {}",
                    **zoom,
                    **zoom * 0.8
                );
                zoom.mul(0.8);
                zoom.set_changed();
            }
        }
    }
}

/// Handles deselect events sent from Leptos (e.g., close button clicks)
pub fn handle_deselect_events(
    mut deselect_events: EventReader<crate::bevy_app::events::DeselectAllEvent>,
    mut pick_selection_query: Query<
        &mut crate::bevy_app::plugins::mouse_interaction::PickSelection,
    >,
) {
    for _event in deselect_events.read() {
        info!("Deselect event received - clearing all Bevy selection state");
        crate::bevy_app::plugins::mouse_interaction::do_deselect_all(&mut pick_selection_query);
    }
}

/// Controls zoom using the mouse wheel.
pub fn control_zoom_from_mouse_wheel(
    mut scroll_events: EventReader<MouseWheel>,
    mut zoom: ResMut<Zoom>,
) {
    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                // Invert the direction: positive y (scroll up) = zoom in
                zoom.mul(1.0 - event.y * 0.1);
            }
            MouseScrollUnit::Pixel => {
                // Invert the direction: positive y (scroll up) = zoom in
                zoom.mul(1.0 - event.y * 0.001);
            }
        }
    }
}

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
        geometries.external_entity.simplified.0 =
            tessellate_simplified_mesh(&external_entity_path, &mut meshes, &mut tess);
        geometries.external_entity.path = external_entity_path;
        geometries.external_entity.aabb.half_extents =
            build_external_entity_aabb_half_extents(scale);

        geometries.interface.simplified.0 = build_interface_simplified_mesh(&mut meshes, scale);
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

/// Applies the given geometry to the specified entity.
///
/// This function updates the `Path`, `SimplifiedMesh`, and `Aabb` components of the specified
/// entity with the values from the provided `FixedSystemElementGeometry`.
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
    simplified_mesh.0 = geometry.simplified.0;
    aabb.half_extents = geometry.aabb.half_extents;
}

/// Adjusts the line width and `Visibility` state of a `Stroke` based on the current `Zoom` level.
pub fn apply_zoom_to_strokes(
    mut highlight_query: Query<(
        &NestingLevel,
        &SystemElement,
        &mut HighlightBundles<Stroke, Stroke>,
        &mut Visibility,
    )>,
    mut stroke_query: Query<
        (&NestingLevel, &mut Stroke, &mut Visibility),
        Without<HighlightBundles<Stroke, Stroke>>,
    >,
    zoom: Res<Zoom>,
) {
    for (nesting_level, system_element, mut highlight, mut visibility) in &mut highlight_query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);

        // Use appropriate line width based on element type
        let base_line_width = match system_element {
            SystemElement::Interaction => FLOW_LINE_WIDTH,
            _ => EXTERNAL_ENTITY_LINE_WIDTH,
        };
        highlight.idle.options.line_width = scale * base_line_width;
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

/// Adjusts the scale and `Visibility` state of entities with a `ApplyZoomToScale` component
/// based on the current `Zoom` level.
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

/// Adjusts the scale and `Visibility` state of entities with a `LabelContainer` component
/// based on the current `Zoom` level.
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

/// Adjusts the scale of a `Transform` and the `Visibility` state based on
/// the `NestingLevel`, zoom, and a threshold.
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

/// Adjusts the `Visibility` state based on a scale value and a threshold.
fn apply_visibility(visibility: &mut Mut<Visibility>, scale: f32, threshold: f32) {
    **visibility = if scale > threshold {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    }
}

/// Adjusts the `Visibility` state of entities with newly added `LabelContainers` based on the current `Zoom` level.
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

/// Phase 3B: Auto-zoom on focus change - calculates target zoom to make focused system ~300px radius.
///
/// When user focuses a nested subsystem (double-click), this system:
/// 1. Detects the focus change
/// 2. Calculates zoom to make focused system 300px screen radius
/// 3. Sets ZoomTarget for smooth animation
///
/// Solves "subsystems too tiny at deep nesting" UX issue.
///
/// IMPORTANT: Only triggers on actual FocusedSystem entity changes, NOT on manual zoom changes.
/// Uses change detection on FocusedSystem resource to detect when user focuses a different system.
pub fn auto_zoom_on_focus_change(
    focused_system: Res<FocusedSystem>,
    mut previous_focus: Local<Option<Entity>>,
    system_query: Query<(
        &GlobalTransform,
        &crate::bevy_app::components::System,
        &NestingLevel,
    )>,
    mut zoom_target: ResMut<ZoomTarget>,
) {
    // Only trigger if FocusedSystem resource changed AND the entity is different
    if !focused_system.is_changed() {
        return;
    }

    let current_focus = **focused_system;

    // Check if this is actually a NEW focus (entity changed, not just resource marked changed)
    if Some(current_focus) == *previous_focus {
        return; // Same entity, no actual focus change
    }

    // Skip placeholder entity (no real system focused)
    if current_focus == Entity::PLACEHOLDER {
        *previous_focus = None;
        return;
    }

    let Ok((global_transform, system, _nesting_level)) = system_query.get(current_focus) else {
        warn!("Failed to get system data for focused entity");
        return;
    };

    // Target: make focused system appear as 300px radius on screen
    let desired_screen_radius = 300.0;
    let target_zoom: f32 = desired_screen_radius / system.radius;

    // Clamp zoom to reasonable bounds
    zoom_target.target_zoom = target_zoom.clamp(0.1, 10.0);
    // CRITICAL FIX: Use GlobalTransform for world position, not local Transform
    zoom_target.target_pan = global_transform.translation().truncate();
    zoom_target.animating = true;
    zoom_target.progress = 0.0;

    info!(
        "🔍 Auto-zoom triggered: entity {:?}, target zoom {:.2}x (system radius {:.1} → screen radius {:.1}px), world pos {:?}",
        current_focus, target_zoom, system.radius, desired_screen_radius, global_transform.translation().truncate()
    );

    *previous_focus = Some(current_focus);
}

/// Phase 3B: Animate zoom and camera pan toward target over ~300ms.
///
/// Smoothly lerps Zoom resource and Camera transform from current to target values.
/// Uses ease-out interpolation for natural feel.
pub fn animate_zoom_to_target(
    mut zoom: ResMut<Zoom>,
    mut zoom_target: ResMut<ZoomTarget>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    if !zoom_target.animating {
        return;
    }

    // Animation duration: 300ms
    let animation_speed = 3.33; // 1.0 / 0.3 seconds
    zoom_target.progress += time.delta_secs() * animation_speed;

    // Ease-out cubic interpolation for smooth deceleration
    let t = zoom_target.progress.min(1.0);
    let ease_t = 1.0 - (1.0 - t).powi(3);

    // Lerp zoom
    let start_zoom = **zoom;
    let new_zoom = start_zoom + (zoom_target.target_zoom - start_zoom) * ease_t;
    **zoom = new_zoom;

    // Lerp camera pan
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let start_pan = camera_transform.translation.truncate();
        let new_pan = start_pan + (zoom_target.target_pan - start_pan) * ease_t;
        camera_transform.translation = new_pan.extend(camera_transform.translation.z);
    }

    // Complete animation when progress reaches 1.0
    if zoom_target.progress >= 1.0 {
        zoom_target.animating = false;
        info!("✅ Auto-zoom animation complete at {:.2}x", **zoom);
    }
}
