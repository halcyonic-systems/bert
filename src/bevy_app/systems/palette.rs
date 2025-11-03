//! Palette UI system for drag-and-drop element creation (Phase 1).
//!
//! Spawns a static sidebar with draggable element icons. Users drag elements
//! from the palette to the canvas to create system components.

use crate::bevy_app::components::{PaletteElement, PaletteElementType};
use bevy::prelude::*;

/// Z-level for palette UI elements (above all diagram elements)
const PALETTE_Z: f32 = 1000.0;

/// Icon size for palette elements
const PALETTE_ICON_SIZE: f32 = 40.0;

/// Vertical spacing between palette elements
const PALETTE_SPACING: f32 = 50.0;

/// Left edge position for palette (screen coordinates)
const PALETTE_X: f32 = -900.0;

/// Top position for first palette element
const PALETTE_START_Y: f32 = 400.0;

/// Spawns the static palette UI with all draggable elements on startup.
///
/// Creates world-space sprites for each PaletteElementType, positioned in a
/// vertical sidebar on the left side of the screen. Each element is marked
/// with PaletteElement component for drag detection.
pub fn spawn_palette_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let elements = [
        (PaletteElementType::Subsystem, "palette-icons/subsystem.png"),
        (
            PaletteElementType::InterfaceSubsystem,
            "palette-icons/interface-subsystem.png",
        ),
        (
            PaletteElementType::ImportInterface,
            "palette-icons/interface.png",
        ),
        (
            PaletteElementType::ExportInterface,
            "palette-icons/interface.png",
        ),
        (PaletteElementType::Flow, "palette-icons/flow.png"),
        (PaletteElementType::Inflow, "palette-icons/inflow.png"),
        (PaletteElementType::Outflow, "palette-icons/outflow.png"),
        (PaletteElementType::Source, "palette-icons/source.png"),
        (PaletteElementType::Sink, "palette-icons/sink.png"),
    ];

    for (idx, (element_type, icon_path)) in elements.iter().enumerate() {
        let y_position = PALETTE_START_Y - (idx as f32 * PALETTE_SPACING);

        commands.spawn((
            PaletteElement {
                element_type: *element_type,
            },
            Sprite {
                image: asset_server.load(*icon_path),
                custom_size: Some(Vec2::splat(PALETTE_ICON_SIZE)),
                ..default()
            },
            Transform::from_translation(Vec3::new(PALETTE_X, y_position, PALETTE_Z)),
            Name::new(format!("Palette: {:?}", element_type)),
        ));
    }
}
