//! Connection mode system for creating flow edges between elements (Phase 2D-Alpha)
//!
//! Implements modal workflow for connecting subsystems with flow edges.
//! This aligns with Mobus formalization where flows are EDGES in N (internal network),
//! not nodes - you can't "place" an edge, you create it by connecting vertices.
//!
//! ## Phase 2D-Alpha Scope
//! - Subsystem ↔ Subsystem flows ONLY (N network edges)
//! - Same nesting level validation
//! - Default substance: Material, usability: Resource
//! - Modal mode: stay active after creating flow, ESC to exit
//!
//! ## UX Flow
//! 1. Press 'F' key or click connection button → Enter connection mode
//! 2. Click first subsystem → Select as source, ghost line appears
//! 3. Ghost line follows cursor from source
//! 4. Click second subsystem → Validate and create flow edge
//! 5. Mode stays active for multiple connections, ESC to exit
//!
//! ## Future Phases
//! - Phase 2D-Beta: Add EnvironmentalObject ↔ Interface support (G network)
//! - Phase 3: Substance/usability selection dialog, duplicate detection

use crate::bevy_app::bundles::spawn_interaction_only;
use crate::bevy_app::components::{
    EndTargetType, Flow, FlowCurve, FlowEndConnection, FlowStartConnection, InitialPosition,
    InteractionType, InteractionUsability, NestingLevel, Parameter, StartTargetType, SubstanceType,
    Subsystem,
};
use crate::bevy_app::resources::{StrokeTessellator, Zoom};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rust_decimal::Decimal;

/// Connection mode state - tracks source selection and ghost line visibility.
///
/// Modal workflow: Enter mode with 'F' key, stays active until ESC pressed.
/// This allows creating multiple flow connections without re-entering mode.
#[derive(Resource, Default)]
pub struct ConnectionMode {
    /// Whether connection mode is currently active
    pub active: bool,
    /// Entity of the first-clicked element (source of flow)
    pub source_entity: Option<Entity>,
}

/// Step 1: Enter connection mode when 'F' key is pressed.
///
/// Initializes ConnectionMode resource to active state.
/// Visual feedback handled by ghost line system.
pub fn enter_connection_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut connection_mode: ResMut<ConnectionMode>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        if connection_mode.active {
            // Already in connection mode, do nothing (user might be typing)
            return;
        }
        connection_mode.active = true;
        connection_mode.source_entity = None;
        info!("🔗 Connection mode ACTIVE - Click first subsystem");
    }

    // ESC to exit connection mode
    if keys.just_pressed(KeyCode::Escape) && connection_mode.active {
        *connection_mode = ConnectionMode::default();
        info!("❌ Connection mode EXITED");
    }
}

/// Step 2: Select connection source on first click.
///
/// When user clicks a subsystem while in connection mode (and no source selected),
/// that subsystem becomes the source of the flow edge.
pub fn select_connection_source(
    mut click_events: EventReader<bevy_picking::events::Pointer<bevy_picking::events::Click>>,
    mut connection_mode: ResMut<ConnectionMode>,
    subsystem_query: Query<&Subsystem>,
) {
    if !connection_mode.active {
        return;
    }

    // Only select source if we don't have one yet
    if connection_mode.source_entity.is_some() {
        return;
    }

    for click_event in click_events.read() {
        // Check if clicked entity is a subsystem
        if subsystem_query.get(click_event.target).is_ok() {
            connection_mode.source_entity = Some(click_event.target);
            info!("✅ Source selected: {:?} - Click destination subsystem", click_event.target);
            return; // Only select first valid click
        }
    }
}

/// Step 3: Render ghost line from source to cursor using Bevy Gizmos.
///
/// Shows visual preview of the flow edge being created.
/// Line follows cursor in world space.
pub fn update_connection_ghost(
    connection_mode: Res<ConnectionMode>,
    subsystem_query: Query<&Transform, With<Subsystem>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
) {
    if !connection_mode.active {
        return;
    }

    let Some(source_entity) = connection_mode.source_entity else {
        return; // No source selected yet, nothing to draw
    };

    let Ok(source_transform) = subsystem_query.get(source_entity) else {
        return; // Source entity invalid (deleted?), skip drawing
    };

    // Get cursor world position
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return; // Camera not ready yet
    };
    let Ok(window) = window_query.get_single() else {
        return; // Window not ready yet
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            let source_world_pos = source_transform.translation.truncate();

            // Draw cyan line from source to cursor
            gizmos.line_2d(source_world_pos, cursor_world_pos, Color::srgb(0.0, 1.0, 1.0));
        }
    }
}

/// Step 4: Finalize connection on second click - validate and create flow edge.
///
/// # Validation Rules (Phase 2D-Alpha)
/// - Both entities must be Subsystems
/// - Must be at same nesting level
/// - Source and destination must be different entities
///
/// # Flow Properties (Hardcoded for MVP)
/// - Substance: Material
/// - Usability: Resource
/// - Amount: Default from spawn_flow
pub fn finalize_connection(
    mut click_events: EventReader<bevy_picking::events::Pointer<bevy_picking::events::Click>>,
    mut connection_mode: ResMut<ConnectionMode>,
    subsystem_query: Query<(&Transform, &InitialPosition, &NestingLevel), With<Subsystem>>,
    mut commands: Commands,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
) {
    if !connection_mode.active {
        return;
    }

    let Some(source_entity) = connection_mode.source_entity else {
        return; // No source selected yet, this click will be handled by select_connection_source
    };

    for click_event in click_events.read() {
        let destination_entity = click_event.target;

        // Validate: Don't connect to self
        if source_entity == destination_entity {
            warn!("❌ Cannot connect subsystem to itself");
            continue;
        }

        // Validate: Both must be subsystems
        let Ok((source_transform, _source_initial_pos, source_nesting_level)) =
            subsystem_query.get(source_entity)
        else {
            warn!("❌ Source entity is not a subsystem or was deleted");
            connection_mode.source_entity = None; // Clear invalid source
            continue;
        };

        let Ok((dest_transform, _dest_initial_pos, dest_nesting_level)) =
            subsystem_query.get(destination_entity)
        else {
            warn!("❌ Destination entity is not a subsystem");
            continue; // Don't clear source, user might click wrong element
        };

        // Validate: Same nesting level
        if source_nesting_level != dest_nesting_level {
            warn!(
                "❌ Cannot connect subsystems at different nesting levels ({} vs {})",
                **source_nesting_level, **dest_nesting_level
            );
            continue;
        }

        // All validation passed - create flow edge
        let source_world_pos = source_transform.translation.truncate();
        let dest_world_pos = dest_transform.translation.truncate();

        // Calculate flow curve (simple straight line for now)
        let direction = (dest_world_pos - source_world_pos).normalize_or_zero();
        let flow_curve = FlowCurve {
            start: source_world_pos,
            end: dest_world_pos,
            start_direction: direction,
            end_direction: -direction,
        };

        // Create Flow component with default properties
        let flow = Flow {
            interaction_type: InteractionType::Flow,
            substance_type: SubstanceType::Material,
            substance_sub_type: String::new(),
            amount: Decimal::ONE,
            unit: "unit".to_string(),
            usability: InteractionUsability::Resource,
            parameters: Vec::<Parameter>::new(),
            smart_parameters: Vec::new(),
        };

        // Spawn flow with spawn_interaction_only
        let scale = NestingLevel::compute_scale(**source_nesting_level, **zoom);
        let flow_entity = spawn_interaction_only(
            &mut commands,
            flow,
            flow_curve,
            "New Flow",
            "",
            false, // Not selected initially
            **source_nesting_level,
            scale,
            &mut stroke_tess,
            &mut meshes,
        );

        // Add connection components to link flow to source and destination subsystems
        commands.entity(flow_entity).insert((
            FlowStartConnection {
                target: source_entity,
                target_type: StartTargetType::System,
            },
            FlowEndConnection {
                target: destination_entity,
                target_type: EndTargetType::System,
            },
        ));

        info!(
            "✅ Flow created: {:?} → {:?} (Material/Resource)",
            source_entity, destination_entity
        );

        // Clear source to allow creating another flow
        connection_mode.source_entity = None;
        info!("🔗 Connection mode ACTIVE - Click first subsystem (or ESC to exit)");

        return; // Only handle first valid click per frame
    }
}
