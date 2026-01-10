//! Keyboard shortcut system for creating interface subsystems.
//!
//! Implements Mobus 8-tuple principle I ⊆ C: interfaces ARE subsystems.
//! Interface subsystems are small circular entities positioned at interfaces
//! that process boundary-crossing flows.
//!
//! ## UX Flow
//! 1. Select an interface (click on it)
//! 2. Press 'I' key → Interface subsystem created
//! 3. Interface subsystem appears small and unobtrusive at the interface
//! 4. Internal flows (N-network) can now connect to the interface subsystem
//!
//! ## Theoretical Foundation
//! Per Mobus, interfaces are a subset of subsystems (I ⊆ C).
//! Interface subsystems make this relationship explicit and visual:
//! - G-network flows terminate at the interface (rectangle)
//! - N-network flows connect to the interface subsystem (circle)

use crate::bevy_app::bundles::spawn_interface_subsystem;
use crate::bevy_app::components::SelectedHighlightHelperAdded;
use crate::bevy_app::components::{
    ElementDescription, Flow, FlowEndInterfaceConnection, FlowStartInterfaceConnection, Interface,
    InterfaceSubsystemConnection, InterfaceType, NestingLevel,
};
use crate::bevy_app::resources::{FocusedSystem, Zoom};
use bevy::prelude::*;

/// Create interface subsystem when 'I' key is pressed with an interface selected.
///
/// Only creates if the interface doesn't already have an interface subsystem attached.
/// Determines interface type (Import/Export) from connected flows.
#[allow(clippy::too_many_arguments)]
pub fn create_interface_subsystem_on_keypress(
    keys: Res<ButtonInput<KeyCode>>,
    // Query selected interfaces (have both Interface and SelectedHighlightHelperAdded)
    selected_interface_query: Query<
        (Entity, Option<&InterfaceSubsystemConnection>),
        (With<Interface>, With<SelectedHighlightHelperAdded>),
    >,
    // Flow connections to determine interface type
    flow_interface_query: Query<(
        Entity,
        &Flow,
        Option<&FlowEndInterfaceConnection>,
        Option<&FlowStartInterfaceConnection>,
    )>,
    // Required for spawn_interface_subsystem
    system_query: Query<(
        &Transform,
        &crate::bevy_app::components::System,
        &Name,
        &ElementDescription,
    )>,
    nesting_level_query: Query<&NestingLevel>,
    focused_system: Res<FocusedSystem>,
    zoom: Res<Zoom>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Only trigger on 'I' key press
    if !keys.just_pressed(KeyCode::KeyI) {
        return;
    }

    // Find selected interface(s)
    for (interface_entity, existing_connection) in selected_interface_query.iter() {
        // Debug: log what we found
        info!(
            "🔍 Checking interface {:?}, has_connection: {:?}",
            interface_entity,
            existing_connection.map(|c| c.target)
        );

        // Skip if interface already has an interface subsystem
        if existing_connection.is_some() {
            info!(
                "⚠️ Interface {:?} already has an interface subsystem pointing to {:?}",
                interface_entity,
                existing_connection.unwrap().target
            );
            continue;
        }

        // Determine interface type from connected flows
        let interface_type = determine_interface_type(interface_entity, &flow_interface_query);

        info!(
            "🔧 Creating {:?} interface subsystem for interface {:?}",
            interface_type, interface_entity
        );

        // Spawn interface subsystem with is_child_of_interface: true
        // This keeps it small (14% of parent) and positions it properly
        spawn_interface_subsystem(
            &mut commands,
            true, // is_child_of_interface - keeps it small and unobtrusive
            interface_type,
            interface_entity,
            &flow_interface_query,
            &system_query,
            &nesting_level_query,
            &focused_system,
            &mut meshes,
            **zoom,
            "Interface Processor", // Default descriptive name
            "",                    // Empty description for minimal clutter
        );

        info!("✅ Interface subsystem created successfully");
    }
}

/// Determine whether an interface is Import or Export based on connected flows.
///
/// - Import: flow ends at this interface (FlowEndInterfaceConnection)
/// - Export: flow starts from this interface (FlowStartInterfaceConnection)
///
/// Defaults to Import if no flows are connected (can be changed later when flows added).
fn determine_interface_type(
    interface_entity: Entity,
    flow_query: &Query<(
        Entity,
        &Flow,
        Option<&FlowEndInterfaceConnection>,
        Option<&FlowStartInterfaceConnection>,
    )>,
) -> InterfaceType {
    for (_, _, end_connection, start_connection) in flow_query.iter() {
        // Check if this flow ends at the interface (import)
        if let Some(connection) = end_connection {
            if connection.target == interface_entity {
                return InterfaceType::Import;
            }
        }
        // Check if this flow starts from the interface (export)
        if let Some(connection) = start_connection {
            if connection.target == interface_entity {
                return InterfaceType::Export;
            }
        }
    }

    // Default to Import if no flows connected yet
    InterfaceType::Import
}
