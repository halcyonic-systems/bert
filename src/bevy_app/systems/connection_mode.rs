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
    FlowStartConnection, FlowStartInterfaceConnection, InitialPosition, InteractionType,
    InteractionUsability, Interface, InterfaceBehavior, NestingLevel, Parameter, StartTargetType,
    SubstanceType, Subsystem,
};
use crate::bevy_app::events::DeselectAllEvent;
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
pub fn finalize_connection(
    mut click_events: EventReader<bevy_picking::events::Pointer<bevy_picking::events::Click>>,
    mut connection_mode: ResMut<ConnectionMode>,
    subsystem_query: Query<(&Transform, &InitialPosition, &NestingLevel), With<Subsystem>>,
    interface_query: Query<(&Transform, &InitialPosition, &NestingLevel), With<Interface>>,
    interface_behavior_query: Query<&InterfaceBehavior>,
    external_entity_query: Query<
        (&Transform, &InitialPosition, &NestingLevel),
        With<ExternalEntity>,
    >,
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

        if !is_valid_n_network && !is_valid_g_network {
            // Provide specific error messages for invalid combinations
            if source_is_external && dest_is_subsystem {
                warn!("❌ Cannot connect EnvironmentalObject directly to Subsystem (must connect to Interface per G network)");
            } else if source_is_subsystem && dest_is_external {
                warn!("❌ Cannot connect Subsystem directly to EnvironmentalObject (must connect to Interface per G network)");
            } else if source_is_external && dest_is_external {
                warn!("❌ Cannot connect EnvironmentalObject to EnvironmentalObject (no direct environment-to-environment flows)");
            } else {
                warn!("❌ Invalid connection type");
            }
            continue;
        }

        // Get transforms and nesting levels based on element type
        let (source_transform, source_nesting_level) = if source_is_subsystem {
            let (t, _, n) = subsystem_query.get(source_entity).unwrap();
            (t, n)
        } else if source_is_interface {
            let (t, _, n) = interface_query.get(source_entity).unwrap();
            (t, n)
        } else {
            // source_is_external
            let (t, _, n) = external_entity_query.get(source_entity).unwrap();
            (t, n)
        };

        let (dest_transform, dest_nesting_level) = if dest_is_subsystem {
            let (t, _, n) = subsystem_query.get(destination_entity).unwrap();
            (t, n)
        } else if dest_is_interface {
            let (t, _, n) = interface_query.get(destination_entity).unwrap();
            (t, n)
        } else {
            // dest_is_external
            let (t, _, n) = external_entity_query.get(destination_entity).unwrap();
            (t, n)
        };

        // Validate: Same nesting level (only for N network)
        // G network connections MUST cross levels (interface at N+1, external at N)
        // per Mobus theory - that's the system boundary!
        if source_nesting_level != dest_nesting_level {
            if is_valid_n_network {
                // N network requires same level
                warn!(
                    "❌ Cannot connect elements at different nesting levels ({} vs {})",
                    **source_nesting_level, **dest_nesting_level
                );
                continue;
            }
            // G network: cross-level is expected and correct, allow it
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

        // Determine target types based on element types
        let start_target_type = if source_is_external {
            StartTargetType::Source
        } else {
            StartTargetType::System
        };

        let end_target_type = if dest_is_external {
            EndTargetType::Sink
        } else {
            EndTargetType::System
        };

        // Add connection components to link flow to source and destination
        let mut flow_commands = commands.entity(flow_entity);

        flow_commands.insert((
            FlowStartConnection {
                target: source_entity,
                target_type: start_target_type,
            },
            FlowEndConnection {
                target: destination_entity,
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
        let network_type = if is_valid_n_network { "N" } else { "G" };
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

/// Clear the just_exited flag after one frame to allow normal selection again.
///
/// This system runs after finalize_connection to reset the flag, preventing
/// indefinite selection suppression after exiting connection mode.
pub fn clear_connection_exit_flag(mut connection_mode: ResMut<ConnectionMode>) {
    if connection_mode.just_exited {
        connection_mode.just_exited = false;
    }
}
