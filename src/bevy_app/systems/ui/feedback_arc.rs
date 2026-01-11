//! Feedback loop visualization system.
//!
//! Renders dashed bezier arcs connecting matched Source/Sink pairs to visualize
//! external feedback loops around the System of Interest (SOI).
//!
//! # Mobus Theory Foundation
//!
//! In systems theory, feedback occurs when outputs to sinks feed back as inputs
//! from sources. This creates a cycle that passes through the environment:
//!
//! ```text
//!     [Source]                              [Sink]
//!         │                                    ▲
//!         ▼                                    │
//!     ┌───────────────────────────────────────────┐
//!     │                   SOI                     │
//!     └───────────────────────────────────────────┘
//!         ╰─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─╯
//!                     dashed arc
//! ```
//!
//! Source and Sink are matched by having the same `IsSameAsId` component value.

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_prototype_lyon::prelude::*;

use crate::bevy_app::components::{
    FlowEndConnection, FlowStartConnection, IsSameAsId, NestingLevel,
};
use crate::bevy_app::resources::{FocusedSystem, Zoom};

/// Marker component for feedback arc entities
#[derive(Component, Reflect, Debug, Clone, Copy, Default)]
#[reflect(Component)]
pub struct FeedbackArc {
    /// The IsSameAsId value this arc represents
    pub equivalence_id: usize,
}

/// Z-level for feedback arcs (behind flows, above background)
const FEEDBACK_ARC_Z: f32 = -5.0;
/// Line width for feedback arcs
const FEEDBACK_ARC_LINE_WIDTH: f32 = 2.0;
/// Dash pattern: [dash_length, gap_length]
const FEEDBACK_ARC_DASH_ON: f32 = 8.0;
const FEEDBACK_ARC_DASH_OFF: f32 = 4.0;
/// Control point offset multiplier (how far outside SOI the arc curves)
const ARC_CONTROL_OFFSET_MULTIPLIER: f32 = 0.4;

/// System to spawn/update feedback arc visualizations.
///
/// Detects matched Source/Sink pairs via `IsSameAsId` and renders dashed arcs
/// connecting them, curving outside the SOI boundary.
pub fn update_feedback_arcs(
    mut commands: Commands,
    // External entities with IsSameAsId
    external_entity_query: Query<(Entity, &GlobalTransform, &IsSameAsId, &NestingLevel)>,
    // Flow connections to determine Source vs Sink
    flow_start_query: Query<&FlowStartConnection>,
    flow_end_query: Query<&FlowEndConnection>,
    // SOI radius from focused system
    focused_system: Res<FocusedSystem>,
    system_query: Query<(&GlobalTransform, &crate::bevy_app::components::System)>,
    // Existing feedback arcs
    existing_arcs: Query<(Entity, &FeedbackArc)>,
    zoom: Res<Zoom>,
) {
    // Group external entities by IsSameAsId
    let mut id_to_entities: HashMap<usize, Vec<(Entity, Vec2, u16)>> = HashMap::new();

    for (entity, transform, is_same_as_id, nesting_level) in external_entity_query.iter() {
        let pos = transform.translation().truncate();
        id_to_entities
            .entry(**is_same_as_id)
            .or_default()
            .push((entity, pos, **nesting_level));
    }

    // Get SOI center and radius for control point calculation
    let (soi_center, soi_radius) =
        if let Ok((transform, system)) = system_query.get(**focused_system) {
            (transform.translation().truncate(), system.radius * **zoom)
        } else {
            (Vec2::ZERO, 100.0) // Fallback
        };

    // Track which equivalence IDs still have valid pairs
    let mut valid_ids: Vec<usize> = Vec::new();

    // Process each matched pair
    for (equivalence_id, entities) in id_to_entities.iter() {
        // Only process pairs (exactly 2 entities with same ID)
        if entities.len() != 2 {
            continue;
        }

        let (entity_a, pos_a, _nesting_a) = entities[0];
        let (entity_b, pos_b, _nesting_b) = entities[1];

        // Determine which is Source (flow starts from it) and which is Sink (flow ends at it)
        let a_is_source = flow_start_query.iter().any(|conn| conn.target == entity_a);
        let b_is_source = flow_start_query.iter().any(|conn| conn.target == entity_b);
        let a_is_sink = flow_end_query.iter().any(|conn| conn.target == entity_a);
        let b_is_sink = flow_end_query.iter().any(|conn| conn.target == entity_b);

        // Find Source and Sink positions
        // If flows exist, use them to determine direction
        // Otherwise, use position (leftmost = source convention for visual clarity)
        let (source_pos, sink_pos) = if a_is_source && b_is_sink {
            (pos_a, pos_b)
        } else if b_is_source && a_is_sink {
            (pos_b, pos_a)
        } else {
            // No flows yet - use position-based convention (leftmost = source)
            if pos_a.x < pos_b.x {
                (pos_a, pos_b)
            } else {
                (pos_b, pos_a)
            }
        };

        valid_ids.push(*equivalence_id);

        // Check if arc already exists for this ID
        let existing_arc = existing_arcs
            .iter()
            .find(|(_, arc)| arc.equivalence_id == *equivalence_id);

        if let Some((arc_entity, _)) = existing_arc {
            // Update existing arc path
            update_arc_path(
                &mut commands,
                arc_entity,
                source_pos,
                sink_pos,
                soi_center,
                soi_radius,
            );
        } else {
            // Spawn new arc
            spawn_feedback_arc(
                &mut commands,
                *equivalence_id,
                source_pos,
                sink_pos,
                soi_center,
                soi_radius,
            );
        }
    }

    // Remove arcs for pairs that no longer exist
    for (arc_entity, arc) in existing_arcs.iter() {
        if !valid_ids.contains(&arc.equivalence_id) {
            commands.entity(arc_entity).despawn_recursive();
        }
    }
}

/// Spawn a new feedback arc entity
fn spawn_feedback_arc(
    commands: &mut Commands,
    equivalence_id: usize,
    source_pos: Vec2,
    sink_pos: Vec2,
    soi_center: Vec2,
    soi_radius: f32,
) {
    let path = create_arc_path(source_pos, sink_pos, soi_center, soi_radius);

    // Dashed stroke with gray color
    let stroke = Stroke {
        color: Color::srgba(0.5, 0.5, 0.5, 0.7),
        options: StrokeOptions::default()
            .with_line_width(FEEDBACK_ARC_LINE_WIDTH)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round),
    };

    commands.spawn((
        FeedbackArc { equivalence_id },
        ShapeBundle {
            path,
            transform: Transform::from_xyz(0.0, 0.0, FEEDBACK_ARC_Z),
            ..default()
        },
        stroke,
    ));
}

/// Update an existing arc's path
fn update_arc_path(
    commands: &mut Commands,
    arc_entity: Entity,
    source_pos: Vec2,
    sink_pos: Vec2,
    soi_center: Vec2,
    soi_radius: f32,
) {
    let path = create_arc_path(source_pos, sink_pos, soi_center, soi_radius);
    commands.entity(arc_entity).insert(path);
}

/// Create the bezier path for a feedback arc.
///
/// The arc curves OUTSIDE the SOI boundary, connecting Sink → Source
/// to complete the external feedback loop visualization.
fn create_arc_path(source_pos: Vec2, sink_pos: Vec2, soi_center: Vec2, soi_radius: f32) -> Path {
    let mut path_builder = PathBuilder::new();

    // Arc goes from Sink to Source (completing the external loop)
    let start = sink_pos;
    let end = source_pos;

    path_builder.move_to(start);

    // Calculate midpoint between Source and Sink
    let midpoint = (start + end) / 2.0;

    // Direction from SOI center to midpoint
    let outward_dir = (midpoint - soi_center).normalize_or_zero();

    // If entities are on opposite sides of SOI, we need a different approach
    let start_dir = (start - soi_center).normalize_or_zero();
    let end_dir = (end - soi_center).normalize_or_zero();

    // Control point: push outward from SOI center, past the boundary
    let control_distance = soi_radius * (1.0 + ARC_CONTROL_OFFSET_MULTIPLIER);

    // For entities roughly opposite each other, use the midpoint direction
    // For entities on same side, curve away from SOI
    let control_offset = if start_dir.dot(end_dir) < 0.3 {
        // Entities on opposite-ish sides: use midpoint direction
        outward_dir * control_distance
    } else {
        // Entities on same side: curve outward perpendicular to line
        let perp = Vec2::new(-(end - start).y, (end - start).x).normalize_or_zero();
        let away_from_center = if perp.dot(outward_dir) > 0.0 {
            perp
        } else {
            -perp
        };
        away_from_center * control_distance
    };

    let control_point = soi_center + control_offset;

    // Use quadratic bezier for smooth arc
    path_builder.quadratic_bezier_to(control_point, end);

    path_builder.build()
}
