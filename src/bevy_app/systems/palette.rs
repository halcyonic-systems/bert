//! Palette UI system for drag-and-drop element creation (Phase 1).
//!
//! Spawns a static sidebar with draggable element icons. Users drag elements
//! from the palette to the canvas to create system components.

use crate::bevy_app::components::{InitialPosition, PaletteElement, PaletteElementType};
use crate::bevy_app::constants::BUTTON_Z;
use bevy::picking::PickingBehavior;
use bevy::prelude::*;

/// Z-level for palette UI elements (same as buttons for consistency)
const PALETTE_Z: f32 = BUTTON_Z;

/// Icon size for palette elements
const PALETTE_ICON_SIZE: f32 = 40.0;

/// Vertical spacing between palette elements
const PALETTE_SPACING: f32 = 60.0;

/// Left edge position for palette (world coordinates, adjusted for main system at origin)
const PALETTE_X: f32 = -550.0;

/// Top position for first palette element
const PALETTE_START_Y: f32 = 300.0;

/// Spawns the static palette UI with all draggable elements on startup.
///
/// Creates world-space sprites for each PaletteElementType, positioned in a
/// vertical sidebar on the left side of the screen. Each element is marked
/// with PaletteElement component for drag detection.
pub fn spawn_palette_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Icons from bert-icon-system-collection (SVG→PNG32 at 40x40, 8-bit RGBA for WASM compatibility)
    let elements = [
        (PaletteElementType::Subsystem, "palette-icons/subsystem.png"),
        (
            PaletteElementType::InterfaceSubsystem,
            "palette-icons/interface-subsystem.png",
        ),
        (
            PaletteElementType::ImportInterface,
            "palette-icons/import.png", // Semantic import arrow
        ),
        (
            PaletteElementType::ExportInterface,
            "palette-icons/export.png", // Semantic export arrow
        ),
        (PaletteElementType::Flow, "palette-icons/flow.png"), // interaction.svg (flows are interactions)
        (PaletteElementType::Inflow, "palette-icons/inflow.png"), // interaction.svg variant
        (PaletteElementType::Outflow, "palette-icons/outflow.png"), // interaction.svg variant
        (PaletteElementType::Source, "palette-icons/source.png"),
        (PaletteElementType::Sink, "palette-icons/sink.png"),
    ];

    for (idx, (element_type, icon_path)) in elements.iter().enumerate() {
        let y_position = PALETTE_START_Y - (idx as f32 * PALETTE_SPACING);
        let position = Vec2::new(PALETTE_X, y_position);

        commands.spawn((
            PaletteElement {
                element_type: *element_type,
            },
            Sprite {
                image: asset_server.load(*icon_path),
                custom_size: Some(Vec2::splat(PALETTE_ICON_SIZE)),
                ..default()
            },
            Transform::from_translation(position.extend(PALETTE_Z)),
            InitialPosition::new(position),
            Name::new(format!("Palette: {:?}", element_type)),
            PickingBehavior::default(),
        ));
    }
}
