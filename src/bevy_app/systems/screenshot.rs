//! This module contains the systems for taking and saving screenshots.
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotFinished},
    window::PrimaryWindow,
};
use std::path::PathBuf;

/// A resource to hold the path for the next screenshot.
#[derive(Resource, Deref, DerefMut)]
pub struct ScreenshotFile(PathBuf);

/// Takes a screenshot when the user presses the appropriate key combination.
///
/// This system follows the pattern of other direct keyboard shortcuts in the application.
pub fn take_screenshot(
    input: Res<ButtonInput<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    // Using Command on macs, Control on windows/linux.
    let modifier = if cfg!(target_os = "macos") {
        KeyCode::SuperLeft
    } else {
        KeyCode::ControlLeft
    };

    if input.pressed(modifier) && input.just_pressed(KeyCode::KeyP) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
        let path = PathBuf::from(format!("bert_screenshot_{}.png", timestamp));
        commands.insert_resource(ScreenshotFile(path));
        let screenshot_entity = commands.spawn(Screenshot::window(main_window.single())).id();
        info!("Taking screenshot, saving to entity {:?}", screenshot_entity);
    }
}

/// Saves the screenshot to the path specified in the `ScreenshotFile` resource
/// once the `ScreenshotFinished` event is received.
pub fn save_screenshot_to_disk(
    mut screenshot_events: EventReader<ScreenshotFinished>,
    mut commands: Commands,
    screenshot_file: Option<Res<ScreenshotFile>>,
) {
    if let Some(screenshot_file) = screenshot_file {
        for event in screenshot_events.read() {
            if let Some(image_data) = &event.image_data {
                let path = &screenshot_file.0;
                match std::fs::write(path, image_data) {
                    Ok(_) => {
                        info!("Screenshot saved to {}", path.display());
                    }
                    Err(e) => {
                        error!("Failed to save screenshot: {}", e);
                    }
                }
                // Remove the resource so we don't try to save again.
                commands.remove_resource::<ScreenshotFile>();
            }
        }
    }
} 