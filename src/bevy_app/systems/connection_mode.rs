//! Connection mode system for creating flow edges between elements (Phase 2D + Phase 3A)
//!
//! Implements modal workflow for connecting elements with flow edges.
//! This aligns with Mobus formalization where flows are EDGES, not nodes.
//!
//! ## Phase 2D Scope (Complete)
//! - **N network**: Subsystem ↔ Subsystem flows (internal network)
//! - **G network**: EnvironmentalObject ↔ Interface flows (external graph)
//! - Same nesting level validation
//! - Default substance: Material, usability: Resource
//! - Modal mode: stay active after creating flow, ESC to exit
//!
//! ## Phase 3A: Interface as Subsystem Foundation (Complete)
//! - **I ⊆ C**: Treats interfaces as special subsystems per Mobus theory
//! - **Interface ↔ Subsystem flows**: Now valid for proper import/export modeling
//! - **Interface ↔ Interface flows**: Now valid within N network
//! - Implemented via component composition: Interface = Subsystem + InterfaceBehavior
//!
//! ## UX Flow
//! 1. Press 'F' key or click connection button → Enter connection mode
//! 2. Click first element → Select as source, ghost line appears
//! 3. Ghost line follows cursor from source
//! 4. Click second element → Validate and create flow edge
//! 5. Mode stays active for multiple connections, ESC to exit
//!
//! ## Future Phases
//! - Phase 3B+: Substance/usability selection dialog, duplicate detection

use crate::bevy_app::bundles::spawn_interaction_only;
use crate::bevy_app::components::{
    EndTargetType, ExternalEntity, Flow, FlowCurve, FlowEndConnection, FlowEndInterfaceConnection,
    FlowStartConnection, FlowStartInterfaceConnection, InteractionType, InteractionUsability,
    Interface, InterfaceBehavior, NestingLevel, Parameter, StartTargetType, SubstanceType,
    Subsystem, System,
};
use crate::bevy_app::constants::INTERFACE_WIDTH_HALF;
use crate::bevy_app::events::DeselectAllEvent;
use crate::bevy_app::resources::{FocusedSystem, StrokeTessellator, Zoom};
use crate::bevy_app::utils::compute_end_and_direction_from_subsystem;
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
    /// Flag to prevent selection on the frame connection mode exits
    /// (Phase 3C: Prevents final click from opening details panel)
    pub just_exited: bool,
}

/// Step 1: Enter connection mode when 'F' key is pressed.
///
/// Initializes ConnectionMode resource to active state.
/// Visual feedback handled by ghost line system.
/// Auto-deselects any selected elements to prevent self-connection errors.
pub fn enter_connection_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut connection_mode: ResMut<ConnectionMode>,
    mut deselect_events: EventWriter<DeselectAllEvent>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        if connection_mode.active {
            // Already in connection mode, do nothing (user might be typing)
            return;
        }
        connection_mode.active = true;
        connection_mode.source_entity = None;

        // Clear any selected elements to avoid "cannot connect element to itself" errors
        // User can now press F and immediately click the same element that was selected
        deselect_events.send(DeselectAllEvent);

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
/// Accepts Subsystem, Interface, or ExternalEntity as source.
/// Valid connections determined in finalize_connection validation.
pub fn select_connection_source(
    mut click_events: EventReader<bevy_picking::events::Pointer<bevy_picking::events::Click>>,
    mut connection_mode: ResMut<ConnectionMode>,
    subsystem_query: Query<&Subsystem>,
    interface_query: Query<&Interface>,
    external_entity_query: Query<&ExternalEntity>,
) {
    if !connection_mode.active {
        return;
    }

    // Only select source if we don't have one yet
    if connection_mode.source_entity.is_some() {
        return;
    }

    for click_event in click_events.read() {
        let target = click_event.target;

        // Check if clicked entity is a valid connection source
        let is_valid = subsystem_query.get(target).is_ok()
            || interface_query.get(target).is_ok()
            || external_entity_query.get(target).is_ok();

        if is_valid {
            connection_mode.source_entity = Some(target);
            info!(
                "✅ Source selected: {:?} - Click destination element",
                target
            );
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
    transform_query: Query<&Transform>,
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

    let Ok(source_transform) = transform_query.get(source_entity) else {
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

            // Draw preview line from source to cursor (matches default flow color)
            gizmos.line_2d(
                source_world_pos,
                cursor_world_pos,
                Color::srgb(0.95, 0.95, 0.95),
            );
        }
    }
}

/// Step 4: Finalize connection on second click - validate and create flow edge.
///
/// # Validation Rules (Phase 3A Complete)
/// **N network** (Internal):
/// - Subsystem ↔ Subsystem
/// - Interface ↔ Interface (Phase 3A: I ⊆ C)
/// - Interface ↔ Subsystem (Phase 3A: enables import/export modeling)
/// - Same nesting level required
///
/// **G network** (External):
/// - ExternalEntity ↔ Interface (bidirectional)
/// - Same nesting level required
///
/// **Invalid combinations**:
/// - ExternalEntity ↔ Subsystem (violates G network definition)
/// - ExternalEntity ↔ ExternalEntity (no direct environment-to-environment flows)
/// - Self-connections (any type)
///
/// # Flow Properties (Hardcoded for MVP)
/// - Substance: Material
/// - Usability: Resource
/// - Amount: Default from spawn_flow
#[allow(clippy::too_many_arguments)]
pub fn finalize_connection(
    mut click_events: EventReader<bevy_picking::events::Pointer<bevy_picking::events::Click>>,
    mut connection_mode: ResMut<ConnectionMode>,
    // Query subsystems with their parent system reference
    subsystem_query: Query<(&Subsystem, &NestingLevel)>,
    // Query interfaces (children of systems)
    interface_query: Query<&NestingLevel, With<Interface>>,
    interface_behavior_query: Query<&InterfaceBehavior>,
    // Query external entities
    external_entity_query: Query<&NestingLevel, With<ExternalEntity>>,
    // Query System components for radius
    system_query: Query<&System>,
    // Query GlobalTransform for world positions and Transform for rotations
    global_transform_query: Query<&GlobalTransform>,
    transform_query: Query<&Transform>,
    parent_query: Query<&Parent>, // Used for finding parent system
    // Query flow connections to determine Source vs Sink for E-network validation
    // Combined query to stay under Bevy's 16-parameter limit
    flow_connections_query: Query<(&FlowStartConnection, &FlowEndConnection)>,
    // FocusedSystem for E-network parent (external entities at SOI level)
    focused_system: Res<FocusedSystem>,
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
            warn!("❌ Cannot connect element to itself");
            continue;
        }

        // Determine element types
        let source_is_subsystem = subsystem_query.get(source_entity).is_ok();
        let source_is_interface = interface_query.get(source_entity).is_ok();
        let source_is_external = external_entity_query.get(source_entity).is_ok();

        let dest_is_subsystem = subsystem_query.get(destination_entity).is_ok();
        let dest_is_interface = interface_query.get(destination_entity).is_ok();
        let dest_is_external = external_entity_query.get(destination_entity).is_ok();

        // Phase 3A: Interface ↔ Interface connections now allowed
        // Interfaces with InterfaceBehavior are treated as subsystem-capable (I ⊆ C per Mobus)
        // Direction logic: N network flows use inward-pointing arrows

        // Phase 3A: Check if entities have InterfaceBehavior (subsystem-capable interfaces)
        let source_has_interface_behavior = interface_behavior_query.get(source_entity).is_ok();
        let dest_has_interface_behavior = interface_behavior_query.get(destination_entity).is_ok();

        // Validate connection types (N network or G network)
        // Phase 3A: Treat interfaces with InterfaceBehavior as subsystem-capable (I ⊆ C per Mobus)
        let source_is_n_network_capable = source_is_subsystem || source_has_interface_behavior;
        let dest_is_n_network_capable = dest_is_subsystem || dest_has_interface_behavior;

        let is_valid_n_network = source_is_n_network_capable && dest_is_n_network_capable;
        let is_valid_g_network =
            (source_is_external && dest_is_interface) || (source_is_interface && dest_is_external);

        // E-network validation: Sink → Source environmental feedback
        // Source external entity is a "Sink" if flows end at it (FlowEndConnection.target)
        // Dest external entity is a "Source" if flows start from it (FlowStartConnection.target)
        let source_is_sink = source_is_external
            && flow_connections_query.iter().any(|(_, end_conn)| {
                end_conn.target == source_entity && end_conn.target_type == EndTargetType::Sink
            });
        let dest_is_source = dest_is_external
            && flow_connections_query.iter().any(|(start_conn, _)| {
                start_conn.target == destination_entity
                    && start_conn.target_type == StartTargetType::Source
            });

        let is_valid_e_network = source_is_sink && dest_is_source;

        if !is_valid_n_network && !is_valid_g_network && !is_valid_e_network {
            // Provide specific error messages for invalid combinations
            if source_is_external && dest_is_subsystem {
                warn!("❌ Cannot connect EnvironmentalObject directly to Subsystem (must connect to Interface per G network)");
            } else if source_is_subsystem && dest_is_external {
                warn!("❌ Cannot connect Subsystem directly to EnvironmentalObject (must connect to Interface per G network)");
            } else if source_is_external && dest_is_external {
                // More specific message based on what's missing
                if !source_is_sink && !dest_is_source {
                    warn!("❌ Cannot connect EnvironmentalObjects - neither has existing flows (Sink→Source requires both to have G-network connections first)");
                } else if !source_is_sink {
                    warn!("❌ Source must be a Sink (must have flows ending at it) for environmental feedback");
                } else if !dest_is_source {
                    warn!("❌ Destination must be a Source (must have flows starting from it) for environmental feedback");
                } else {
                    warn!("❌ Invalid external-to-external connection");
                }
            } else {
                warn!("❌ Invalid connection type");
            }
            continue;
        }

        // Get nesting levels for validation
        let source_nesting_level = if source_is_subsystem {
            *subsystem_query.get(source_entity).unwrap().1
        } else if source_is_interface {
            *interface_query.get(source_entity).unwrap()
        } else {
            *external_entity_query.get(source_entity).unwrap()
        };

        let dest_nesting_level = if dest_is_subsystem {
            *subsystem_query.get(destination_entity).unwrap().1
        } else if dest_is_interface {
            *interface_query.get(destination_entity).unwrap()
        } else {
            *external_entity_query.get(destination_entity).unwrap()
        };

        // Validate: Same nesting level (only for N network)
        // G network connections MUST cross levels (interface at N+1, external at N)
        // per Mobus theory - that's the system boundary!
        if source_nesting_level != dest_nesting_level {
            if is_valid_n_network {
                // N network requires same level
                warn!(
                    "❌ Cannot connect elements at different nesting levels ({} vs {})",
                    *source_nesting_level, *dest_nesting_level
                );
                continue;
            }
            // G network: cross-level is expected and correct, allow it
        }

        // Find the parent system for the flow
        // Flows are parented to the GRANDPARENT system (subsystem.parent_system),
        // matching how spawn_interaction works in flow.rs
        // For N network: find any subsystem among source/dest and use its parent_system
        // For G network: use the interface's parent's parent_system (if interface is on subsystem)
        let flow_parent_entity = if source_is_subsystem {
            // Source is a subsystem - parent flow to its parent_system
            subsystem_query.get(source_entity).unwrap().0.parent_system
        } else if dest_is_subsystem {
            // Dest is a subsystem - parent flow to its parent_system
            subsystem_query
                .get(destination_entity)
                .unwrap()
                .0
                .parent_system
        } else if source_is_interface {
            // Interface's parent might be a subsystem - check and get grandparent
            let interface_parent = parent_query
                .get(source_entity)
                .map(|p| p.get())
                .expect("Interface should have parent");
            if let Ok((subsystem, _)) = subsystem_query.get(interface_parent) {
                subsystem.parent_system
            } else {
                // Interface is on main system - use the main system as parent
                interface_parent
            }
        } else if dest_is_interface {
            let interface_parent = parent_query
                .get(destination_entity)
                .map(|p| p.get())
                .expect("Interface should have parent");
            if let Ok((subsystem, _)) = subsystem_query.get(interface_parent) {
                subsystem.parent_system
            } else {
                interface_parent
            }
        } else if is_valid_e_network {
            // E-network: external entities at SOI level - use FocusedSystem as parent
            // External entities don't have Parent components at the top level
            **focused_system
        } else {
            // External ↔ External without valid E-network (shouldn't reach here)
            warn!("❌ Cannot determine parent system for flow");
            continue;
        };

        let scale = NestingLevel::compute_scale(*source_nesting_level, **zoom);

        // Get GlobalTransform for world positions
        let parent_global = global_transform_query
            .get(flow_parent_entity)
            .expect("Flow parent should have GlobalTransform");
        let parent_inverse = parent_global.affine().inverse();

        // Compute flow endpoints in WORLD space first, then transform to parent's local space
        // This handles cases where source/dest have different immediate parents
        let (start_world, start_dir) = compute_endpoint_for_entity_world(
            source_entity,
            source_is_subsystem,
            source_is_interface,
            source_is_external,
            &global_transform_query,
            &transform_query,
            &system_query,
            is_valid_n_network,
            scale,
            None, // No other end yet
            None,
        );

        // For subsystems, we need the other endpoint to compute direction
        // For interfaces/externals, direction is based on rotation
        let (end_world, end_dir) = compute_endpoint_for_entity_world(
            destination_entity,
            dest_is_subsystem,
            dest_is_interface,
            dest_is_external,
            &global_transform_query,
            &transform_query,
            &system_query,
            is_valid_n_network,
            scale,
            Some(start_world),
            Some(start_dir),
        );

        // If source is a subsystem, recompute with end info for proper direction
        let (start_world, start_dir) = if source_is_subsystem {
            compute_endpoint_for_entity_world(
                source_entity,
                true,
                false,
                false,
                &global_transform_query,
                &transform_query,
                &system_query,
                is_valid_n_network,
                scale,
                Some(end_world),
                Some(end_dir),
            )
        } else {
            (start_world, start_dir)
        };

        // For Interface ↔ Interface connections, fix DIRECTIONS only
        // Positions are correct (outer edges face each other for interfaces on different subsystems)
        // But default N network direction (-right = toward subsystem center) is wrong
        // We need directions pointing toward each other for proper bezier curve
        let (start_world, start_dir, end_world, end_dir) = if source_is_interface
            && dest_is_interface
        {
            info!("🔧 Interface ↔ Interface: fixing directions to point toward each other");

            // Keep original positions (outer edge), just fix directions
            let to_end = (end_world - start_world).normalize_or_zero();
            let to_start = -to_end;

            info!(
                "🔧 start_world: {:?}, end_world: {:?}",
                start_world, end_world
            );
            info!("🔧 to_end: {:?}, to_start: {:?}", to_end, to_start);

            (start_world, to_end, end_world, to_start)
        } else if is_valid_e_network {
            // E-network: Sink → Source environmental feedback
            // Curve should go OUTSIDE the SOI boundary
            // Compute directions that push the bezier control point outward from SOI center
            info!("🔧 E-network: computing directions for external curve (outside SOI)");

            // Get SOI center and radius from the focused system
            let soi_center = parent_global.translation().truncate();
            let soi_radius = system_query
                .get(flow_parent_entity)
                .map(|s| s.radius * **zoom)
                .unwrap_or(100.0);

            // Midpoint between start and end
            let midpoint = (start_world + end_world) / 2.0;

            // Direction from SOI center to midpoint (outward)
            let outward_dir = (midpoint - soi_center).normalize_or_zero();

            // For bezier curve to go outside SOI, both directions should point outward
            let start_to_end = (end_world - start_world).normalize_or_zero();
            let perp = Vec2::new(-start_to_end.y, start_to_end.x);

            // Choose perpendicular that points away from SOI center
            let away_from_soi = if perp.dot(outward_dir) > 0.0 {
                perp
            } else {
                -perp
            };

            // Calculate how much we need to scale the direction vectors
            // The bezier control points are at: endpoint + direction * tangent_len
            // where tangent_len = endpoint_distance * 0.333
            //
            // For a cubic bezier, the apex (at t=0.5) is approximately:
            //   apex ≈ midpoint + 0.75 * outward_direction * tangent_len
            // So the curve only reaches 75% of the control point offset.
            let endpoint_distance = (end_world - start_world).length();
            let standard_tangent_len = endpoint_distance * 0.333;

            // Distance from SOI center to midpoint
            let midpoint_dist_from_center = (midpoint - soi_center).length();

            // Required apex distance from SOI center to clear the boundary
            let clearance_margin = soi_radius * 0.35; // 35% extra clearance
            let required_apex_dist = soi_radius + clearance_margin;
            let required_outward_offset = (required_apex_dist - midpoint_dist_from_center).max(0.0);

            // Scale factor accounting for the 0.75 bezier apex factor
            // apex_offset = 0.75 * direction_scale * tangent_len
            // We need: 0.75 * direction_scale * tangent_len >= required_outward_offset
            // So: direction_scale >= required_outward_offset / (0.75 * tangent_len)
            let bezier_apex_factor = 0.75;
            let effective_tangent = bezier_apex_factor * standard_tangent_len;

            let direction_scale = if effective_tangent > 0.01 {
                (required_outward_offset / effective_tangent).max(1.5) // Minimum 1.5x scale
            } else {
                2.5 // Fallback for very close endpoints
            };

            // Blend outward direction with direction toward the other endpoint
            // Use mostly perpendicular (outward) direction
            let blend_factor = 0.9;
            let start_blend_unit = (start_to_end * (1.0 - blend_factor) + away_from_soi * blend_factor)
                .normalize_or_zero();
            let end_blend_unit = (-start_to_end * (1.0 - blend_factor) + away_from_soi * blend_factor)
                .normalize_or_zero();

            // Scale the directions so control points clear the SOI
            // Non-unit vectors will result in larger control point offsets
            let start_blend = start_blend_unit * direction_scale;
            let end_blend = end_blend_unit * direction_scale;

            info!(
                "🔧 E-network: soi_radius={:.1}, endpoint_dist={:.1}, tangent_len={:.1}, scale={:.2}",
                soi_radius, endpoint_distance, standard_tangent_len, direction_scale
            );
            info!(
                "🔧 E-network: midpoint_dist={:.1}, required_offset={:.1}",
                midpoint_dist_from_center, required_outward_offset
            );

            (start_world, start_blend, end_world, end_blend)
        } else {
            (start_world, start_dir, end_world, end_dir)
        };

        // Transform from world space to parent system's local space
        let start_local = parent_inverse
            .transform_point3(start_world.extend(0.0))
            .truncate();
        let end_local = parent_inverse
            .transform_point3(end_world.extend(0.0))
            .truncate();

        // DEBUG: Log computed positions
        info!(
            "🔍 DEBUG - Source GlobalTransform: {:?}",
            global_transform_query
                .get(source_entity)
                .map(|t| t.translation().truncate())
        );
        info!(
            "🔍 DEBUG - Dest GlobalTransform: {:?}",
            global_transform_query
                .get(destination_entity)
                .map(|t| t.translation().truncate())
        );
        info!(
            "🔍 DEBUG - Parent GlobalTransform: {:?}",
            parent_global.translation().truncate()
        );
        info!("🔍 DEBUG - Zoom: {}, Scale: {}", **zoom, scale);
        info!(
            "🔍 Flow world positions - Start: {:?}, End: {:?}",
            start_world, end_world
        );
        info!(
            "🔍 Flow local positions - Start: {:?}, End: {:?}",
            start_local, end_local
        );

        let flow_curve = FlowCurve {
            start: start_local,
            end: end_local,
            start_direction: start_dir,
            end_direction: end_dir,
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
        let flow_entity = spawn_interaction_only(
            &mut commands,
            flow,
            flow_curve,
            "New Flow",
            "A connection carrying a flow of real substance (matter, energy, or messages) with causal influence.",
            false, // Not selected initially
            *source_nesting_level,
            scale,
            &mut stroke_tess,
            &mut meshes,
        );

        // Add flow as child of flow_parent (matches how spawn_interaction works)
        commands.entity(flow_parent_entity).add_child(flow_entity);

        // Determine target types based on element types and network type
        let start_target_type = if is_valid_e_network {
            // E-network: flow starts from a Sink (environmental feedback)
            StartTargetType::Sink
        } else if source_is_external {
            // G-network: flow starts from a Source
            StartTargetType::Source
        } else {
            // N-network: flow starts from a System
            StartTargetType::System
        };

        let end_target_type = if is_valid_e_network {
            // E-network: flow ends at a Source (environmental feedback)
            EndTargetType::Source
        } else if dest_is_external {
            // G-network: flow ends at a Sink
            EndTargetType::Sink
        } else {
            // N-network: flow ends at a System
            EndTargetType::System
        };

        // Determine the actual targets for FlowStartConnection/FlowEndConnection
        // For interfaces: target should be the interface's parent SYSTEM, not the interface itself
        // This matches how the save system expects flows to be structured
        let start_target = if source_is_interface {
            // Get the interface's parent (the system it's attached to)
            parent_query
                .get(source_entity)
                .map(|p| p.get())
                .expect("Interface should have parent")
        } else {
            source_entity
        };

        let end_target = if dest_is_interface {
            parent_query
                .get(destination_entity)
                .map(|p| p.get())
                .expect("Interface should have parent")
        } else {
            destination_entity
        };

        // Add connection components to link flow to source and destination
        let mut flow_commands = commands.entity(flow_entity);

        flow_commands.insert((
            FlowStartConnection {
                target: start_target,
                target_type: start_target_type,
            },
            FlowEndConnection {
                target: end_target,
                target_type: end_target_type,
            },
        ));

        // Add Interface-specific connection components for proper rendering
        if source_is_interface {
            flow_commands.insert(FlowStartInterfaceConnection {
                target: source_entity,
            });
        }

        if dest_is_interface {
            flow_commands.insert(FlowEndInterfaceConnection {
                target: destination_entity,
            });
        }

        // Log network type for debugging
        let network_type = if is_valid_n_network {
            "N"
        } else if is_valid_e_network {
            "E"
        } else {
            "G"
        };
        info!(
            "✅ Flow created: {:?} → {:?} ({} network, Material/Resource)",
            source_entity, destination_entity, network_type
        );

        // Exit connection mode after successful flow creation
        // Set just_exited flag to prevent this click from triggering selection
        // This provides better UX for new users who expect mode to exit after one action
        // Power users can press F again to create multiple flows
        connection_mode.active = false;
        connection_mode.source_entity = None;
        connection_mode.just_exited = true; // Prevents selection on this frame
        info!("✅ Flow created successfully - Connection mode EXITED (Press F to create another)");

        return; // Only handle first valid click per frame
    }
}

/// Compute endpoint position and direction for an entity in WORLD space.
///
/// Returns (world_position, direction) tuple using GlobalTransform.
/// Caller is responsible for transforming to flow's local space.
fn compute_endpoint_for_entity_world(
    entity: Entity,
    is_subsystem: bool,
    is_interface: bool,
    _is_external: bool,
    global_transform_query: &Query<&GlobalTransform>,
    transform_query: &Query<&Transform>,
    system_query: &Query<&System>,
    is_n_network: bool,
    scale: f32,
    other_end: Option<Vec2>,
    other_end_dir: Option<Vec2>,
) -> (Vec2, Vec2) {
    // Get entity's GlobalTransform (world position)
    let global_transform = global_transform_query
        .get(entity)
        .expect("Entity should have GlobalTransform");

    // Get local Transform for rotation (GlobalTransform.right() includes parent rotations)
    let local_transform = transform_query
        .get(entity)
        .expect("Entity should have Transform");

    // GlobalTransform gives us world position
    let pos = global_transform.translation().truncate();
    // Use local transform's right vector for entity's own orientation
    let right = local_transform.right().truncate();

    if is_subsystem {
        // For subsystems, compute position at boundary facing the other endpoint
        let system = system_query
            .get(entity)
            .expect("Subsystem should have System component");

        if let (Some(other_pos), Some(other_dir)) = (other_end, other_end_dir) {
            // Use zoomed radius (system.radius * scale)
            compute_end_and_direction_from_subsystem(
                pos,
                system.radius * scale,
                other_pos,
                other_dir,
            )
        } else {
            // First pass - use a placeholder direction (will be recomputed)
            (pos, Vec2::X)
        }
    } else if is_interface {
        // For interfaces, position is at edge with direction based on rotation
        // N network: arrows point inward (toward system center)
        // G network: arrows point outward (toward environment)
        let direction = if is_n_network { -right } else { right };

        // Use scaled offset
        (pos + right * INTERFACE_WIDTH_HALF * scale, direction)
    } else {
        // External entity - direction points toward system
        (pos - right * INTERFACE_WIDTH_HALF * scale, -right)
    }
}

/// Clear the just_exited flag after one frame to allow normal selection again.
///
/// This system runs after finalize_connection to reset the flag, preventing
/// indefinite selection suppression after exiting connection mode.
pub fn clear_connection_exit_flag(mut connection_mode: ResMut<ConnectionMode>) {
    if connection_mode.just_exited {
        connection_mode.just_exited = false;
    }
}

/// Monitors mode changes and sends events to Leptos for UI indicator.
///
/// Tracks ConnectionMode and PlacementMode, sending ModeChangeEvent when state changes.
pub fn send_mode_change_events(
    connection_mode: Res<ConnectionMode>,
    placement_mode: Res<super::palette::PlacementMode>,
    mut mode_event_writer: EventWriter<crate::bevy_app::events::ModeChangeEvent>,
    mut last_mode_text: Local<String>,
) {
    // Determine current mode text
    let current_mode_text = if connection_mode.active {
        if connection_mode.source_entity.is_some() {
            "Connection Mode - Click destination".to_string()
        } else {
            "Connection Mode (F) - Click source".to_string()
        }
    } else if let Some(element_type) = &placement_mode.active_element {
        format!("Placing {:?} - Click to place, ESC to cancel", element_type)
    } else {
        String::new()
    };

    // Only send event if mode changed
    if current_mode_text != *last_mode_text {
        mode_event_writer.send(crate::bevy_app::events::ModeChangeEvent {
            mode_text: current_mode_text.clone(),
        });
        *last_mode_text = current_mode_text;
    }
}
