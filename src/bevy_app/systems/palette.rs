//! Palette UI system for click-to-place element creation (Phase 2A).
//!
//! Spawns a static sidebar with clickable element icons. Users click an icon
//! to enter placement mode, then click the canvas to place the element.
//!
//! Phase 2A: Subsystem placement with ghost preview (other elements warn gracefully).
//!
//! ## UX Flow
//! 1. Click palette icon → Enters placement mode with semi-transparent ghost
//! 2. Ghost follows cursor
//! 3. Click canvas → Element spawns at cursor position
//! 4. ESC → Cancel placement

use crate::bevy_app::bundles::{spawn_external_entity_only, spawn_interface_only, spawn_subsystem};
use crate::bevy_app::components::{
    ElementDescription, Flow, FlowCurve, InitialPosition, NestingLevel, PaletteElementType,
    SubstanceType, System,
};
use crate::bevy_app::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator, Zoom,
};
use crate::bevy_app::systems::{CommandHistory, PlaceElementCommand};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Z-level for placement ghost (above everything else)
const GHOST_Z: f32 = 200.0;

/// Placement mode state - tracks active element type and ghost entity.
///
/// When a palette icon is clicked, this resource stores which element type
/// is being placed and the entity ID of the semi-transparent ghost preview.
#[derive(Resource, Default)]
pub struct PlacementMode {
    /// The element type being placed (None = not in placement mode)
    pub active_element: Option<PaletteElementType>,
    /// Entity ID of the ghost preview sprite
    pub ghost_entity: Option<Entity>,
}

/// Component marker for the placement ghost sprite.
///
/// The ghost is a semi-transparent preview that follows the cursor to show
/// where the element will be placed.
#[derive(Component)]
pub struct PlacementGhost;

/// Icon size for palette ghost preview
const PALETTE_ICON_SIZE: f32 = 40.0;

/// Maps PaletteElementType to its icon asset path.
fn icon_path_for(element_type: PaletteElementType) -> &'static str {
    match element_type {
        PaletteElementType::Subsystem => "palette-icons/subsystem-icon.png",
        PaletteElementType::Interface => "palette-icons/interface-icon.png",
        PaletteElementType::EnvironmentalObject => "palette-icons/source.png", // Reuse source icon for now
    }
}

// DEPRECATED: Replaced by Leptos fixed panel (see src/leptos_app/components/palette.rs)
// /// Spawns the static palette UI with all draggable elements on startup.
// ///
// /// Creates world-space sprites for each PaletteElementType, positioned in a
// /// vertical sidebar on the left side of the screen. Each element is marked
// /// with PaletteElement component for drag detection.
// pub fn spawn_palette_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let elements = [
//         (PaletteElementType::Subsystem, "palette-icons/subsystem.png"),
//         (PaletteElementType::Interface, "palette-icons/interface.png"),
//         (
//             PaletteElementType::EnvironmentalObject,
//             "palette-icons/source.png",
//         ),
//     ];
//
//     for (idx, (element_type, icon_path)) in elements.iter().enumerate() {
//         let y_position = PALETTE_START_Y - (idx as f32 * PALETTE_SPACING);
//         let position = Vec2::new(PALETTE_X, y_position);
//
//         commands
//             .spawn((
//                 PaletteElement {
//                     element_type: *element_type,
//                 },
//                 Sprite {
//                     image: asset_server.load(*icon_path),
//                     custom_size: Some(Vec2::splat(PALETTE_ICON_SIZE)),
//                     ..default()
//                 },
//                 Transform::from_translation(position.extend(PALETTE_Z)),
//                 InitialPosition::new(position),
//                 Name::new(format!("Palette: {:?}", element_type)),
//                 // Enable picking and dragging
//                 PickingBehavior {
//                     should_block_lower: true, // Block clicks from passing through
//                     is_hoverable: true,
//                 },
//                 PickSelection::default(), // Required for mouse interaction system
//                 NoDeselect,               // Don't participate in selection system
//             ))
//             .observe(|trigger: Trigger<DragPosition>, mut commands: Commands| {
//                 commands.trigger(PaletteDrag::from(trigger));
//             });
//     }
// }

// DEPRECATED: Replaced by handle_leptos_palette_click
// /// Step 1: Detects palette icon clicks and enters placement mode with ghost preview.
// ///
// /// When a palette icon is clicked:
// /// 1. Stores the element type in PlacementMode resource
// /// 2. Spawns a semi-transparent ghost sprite
// /// 3. Ghost will follow cursor (handled by update_placement_ghost)
// pub fn enter_placement_mode(
//     mut click_events: EventReader<bevy_picking::events::Pointer<bevy_picking::events::Click>>,
//     palette_query: Query<&PaletteElement>,
//     mut placement_mode: ResMut<PlacementMode>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     for click_event in click_events.read() {
//         if let Ok(palette_element) = palette_query.get(click_event.target) {
//             // Clean up previous ghost if it exists
//             if let Some(old_ghost) = placement_mode.ghost_entity {
//                 commands.entity(old_ghost).despawn();
//             }
//
//             // Enter placement mode
//             placement_mode.active_element = Some(palette_element.element_type);
//
//             // Spawn semi-transparent preview ghost
//             let ghost = commands
//                 .spawn((
//                     PlacementGhost,
//                     Sprite {
//                         image: asset_server.load(icon_path_for(palette_element.element_type)),
//                         color: Color::srgba(1.0, 1.0, 1.0, 0.5), // Semi-transparent
//                         custom_size: Some(Vec2::splat(PALETTE_ICON_SIZE)),
//                         ..default()
//                     },
//                     Transform::from_xyz(0.0, 0.0, GHOST_Z),
//                     Name::new("Placement Ghost"),
//                 ))
//                 .id();
//
//             placement_mode.ghost_entity = Some(ghost);
//             info!(
//                 "✨ Entered placement mode for {:?}",
//                 palette_element.element_type
//             );
//         }
//     }
// }

/// Handle palette clicks from Leptos UI panel
pub fn handle_leptos_palette_click(
    mut palette_events: EventReader<crate::events::PaletteClickEvent>,
    mut placement_mode: ResMut<PlacementMode>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in palette_events.read() {
        // Convert PaletteElementTypeEvent to PaletteElementType
        let element_type = match event.element_type {
            crate::events::PaletteElementTypeEvent::Subsystem => PaletteElementType::Subsystem,
            crate::events::PaletteElementTypeEvent::Interface => PaletteElementType::Interface,
            crate::events::PaletteElementTypeEvent::EnvironmentalObject => {
                PaletteElementType::EnvironmentalObject
            }
        };

        // Clean up previous ghost if it exists
        if let Some(old_ghost) = placement_mode.ghost_entity {
            commands.entity(old_ghost).despawn();
        }

        // Enter placement mode
        placement_mode.active_element = Some(element_type);

        // Spawn semi-transparent preview ghost
        let ghost = commands
            .spawn((
                PlacementGhost,
                Sprite {
                    image: asset_server.load(icon_path_for(element_type)),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.5), // Semi-transparent
                    custom_size: Some(Vec2::splat(PALETTE_ICON_SIZE)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, GHOST_Z),
                Name::new("Placement Ghost"),
            ))
            .id();

        placement_mode.ghost_entity = Some(ghost);
        info!(
            "✨ Entered placement mode for {:?} (from Leptos)",
            element_type
        );
    }
}

/// Step 2: Updates ghost sprite position to follow cursor in world space.
///
/// Runs every frame while in placement mode. Converts cursor screen coordinates
/// to world coordinates and updates the ghost transform.
///
/// For interfaces, snaps ghost to boundary of focused system.
pub fn update_placement_ghost(
    placement_mode: Res<PlacementMode>,
    mut ghost_query: Query<&mut Transform, With<PlacementGhost>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    focused_system: Res<FocusedSystem>,
    system_query: Query<(&Transform, &System), Without<PlacementGhost>>,
    zoom: Res<Zoom>,
) {
    let Some(element_type) = placement_mode.active_element else {
        return;
    };

    let Some(ghost_entity) = placement_mode.ghost_entity else {
        return;
    };
    let Ok(mut ghost_transform) = ghost_query.get_mut(ghost_entity) else {
        return;
    };

    // Get cursor world position
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Snap to boundary for interfaces only
            let ghost_world_pos = match element_type {
                PaletteElementType::Interface => {
                    if **focused_system != Entity::PLACEHOLDER {
                        if let Ok((system_transform, system)) = system_query.get(**focused_system) {
                            let system_center = system_transform.translation.truncate();
                            let cursor_direction =
                                (cursor_world_pos - system_center).normalize_or_zero();
                            system_center + cursor_direction * system.radius * **zoom
                        } else {
                            cursor_world_pos // Fallback to cursor if system query fails
                        }
                    } else {
                        cursor_world_pos // No focused system, follow cursor
                    }
                }
                // Subsystems and EnvironmentalObjects follow cursor freely
                PaletteElementType::Subsystem | PaletteElementType::EnvironmentalObject => {
                    cursor_world_pos
                }
            };

            ghost_transform.translation = ghost_world_pos.extend(GHOST_Z);
        }
    }
}

/// Step 3: Finalizes element placement on canvas click or cancels on ESC.
///
/// # Controls
/// - **Left Click**: Spawns element at ghost position, exits placement mode
/// - **ESC**: Cancels placement, despawns ghost, exits placement mode
///
/// # Phase 2A Implementation
/// - Subsystem: Spawns at ghost position with no flows
/// - Other elements: Log warning (not yet implemented)
pub fn finalize_placement(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut placement_mode: ResMut<PlacementMode>,
    ghost_query: Query<&Transform, With<PlacementGhost>>,
    mut commands: Commands,
    focused_system: Res<FocusedSystem>,
    system_query: Query<(&Transform, &System, &Name, &ElementDescription)>,
    nesting_level_query: Query<&NestingLevel>,
    flow_query: Query<(&FlowCurve, &Flow)>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    zoom: Res<Zoom>,
    mut command_history: ResMut<CommandHistory>,
) {
    let Some(element_type) = placement_mode.active_element else {
        return;
    };

    // ESC to cancel
    if keys.just_pressed(KeyCode::Escape) {
        if let Some(ghost) = placement_mode.ghost_entity {
            commands.entity(ghost).despawn();
        }
        *placement_mode = PlacementMode::default();
        info!("❌ Placement cancelled");
        return;
    }

    // Left click to place
    if mouse_button.just_pressed(MouseButton::Left) {
        // Get ghost world position
        let ghost_world_pos = if let Some(ghost_entity) = placement_mode.ghost_entity {
            let ghost_transform = ghost_query.get(ghost_entity).unwrap();
            let world_pos = ghost_transform.translation.truncate();
            commands.entity(ghost_entity).despawn();
            world_pos
        } else {
            Vec2::ZERO
        };

        // Validate focused system
        if **focused_system == Entity::PLACEHOLDER {
            warn!("Cannot spawn element: no system focused");
            *placement_mode = PlacementMode::default();
            return;
        }

        // Get focused system data
        let (focused_transform, _focused_system_component, _, _) =
            system_query.get(**focused_system).unwrap();
        let system_center = focused_transform.translation.truncate();

        // Spawn element based on type
        match element_type {
            PaletteElementType::Subsystem => {
                // Subsystems: Place at cursor position (freeform)
                let local_pos = (ghost_world_pos - system_center) / **zoom;
                let entity = spawn_subsystem(
                    &mut commands,
                    **focused_system,
                    &system_query,
                    &nesting_level_query,
                    &flow_query,
                    &[], // Empty inflows
                    &[], // Empty outflows
                    &mut fixed_system_element_geometries,
                    &mut meshes,
                    &mut stroke_tess,
                    **zoom,
                    "New Subsystem",
                    "", // Empty description
                    local_pos,
                );
                info!("✅ Subsystem placed at {:?}", local_pos);

                // Push undo command
                command_history.push(Box::new(PlaceElementCommand {
                    entity,
                    element_type,
                    position: local_pos,
                    parent_system: Some(**focused_system),
                }));
            }
            PaletteElementType::Interface => {
                // Interfaces: Already snapped to boundary by update_placement_ghost
                // The ghost position IS the boundary position

                // Calculate angle for interface rotation (faces outward from center)
                let cursor_direction = (ghost_world_pos - system_center).normalize_or_zero();
                let angle = cursor_direction.to_angle();

                let transform = Transform::from_translation(ghost_world_pos.extend(0.0))
                    .with_rotation(Quat::from_rotation_z(angle));
                let initial_position = InitialPosition::new(ghost_world_pos);

                // Phase 3A: Interfaces must be at same nesting level as internal subsystems
                // for N network flows to work (same-level validation requirement)
                let nesting_level =
                    NestingLevel::current(**focused_system, &nesting_level_query) + 1;

                let entity = spawn_interface_only(
                    &mut commands,
                    SubstanceType::default(), // Default substance, user can change later
                    nesting_level,
                    **focused_system,
                    **zoom,
                    false, // Not selected initially
                    "New Interface",
                    "",                    // Empty description
                    "default".to_string(), // Default protocol
                    transform,
                    initial_position,
                    &mut stroke_tess,
                    &mut meshes,
                    &mut fixed_system_element_geometries,
                );
                info!(
                    "✅ Interface placed on boundary at angle {:.2}°",
                    angle.to_degrees()
                );

                // Push undo command
                command_history.push(Box::new(PlaceElementCommand {
                    entity,
                    element_type,
                    position: ghost_world_pos,
                    parent_system: Some(**focused_system),
                }));
            }
            PaletteElementType::EnvironmentalObject => {
                // Environmental objects: Freeform placement in environment
                // These are objects in O (environment set) per Mobus 8-tuple

                let transform = Transform::from_translation(ghost_world_pos.extend(0.0));
                let initial_position = InitialPosition::new(ghost_world_pos);

                let nesting_level = NestingLevel::current(**focused_system, &nesting_level_query);

                let entity = spawn_external_entity_only(
                    &mut commands,
                    SubstanceType::default(), // Default substance, user can change later
                    false,                    // Not selected initially
                    "New Environmental Object",
                    "", // Empty description
                    "", // Empty equivalence
                    "", // Empty model
                    transform,
                    initial_position,
                    nesting_level,
                    **zoom,
                    &mut fixed_system_element_geometries,
                    &mut meshes,
                    &mut stroke_tess,
                );
                info!("✅ Environmental object placed at {:?}", ghost_world_pos);

                // Push undo command
                command_history.push(Box::new(PlaceElementCommand {
                    entity,
                    element_type,
                    position: ghost_world_pos,
                    parent_system: Some(**focused_system),
                }));
            }
        }

        *placement_mode = PlacementMode::default();
    }
}
