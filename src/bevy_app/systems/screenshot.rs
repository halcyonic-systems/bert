//! Screenshot capture and export functionality for BERT diagrams.
//!
//! This module provides systems for capturing screenshots of the BERT canvas
//! and saving them to disk with timestamped filenames. Screenshots are only
//! available on desktop builds (non-wasm).
//!
//! ## Key Features
//!
//! - **Desktop Only**: Gated behind `#[cfg(not(target_arch = "wasm32"))]`
//! - **Timestamped Files**: Automatic filename generation with date/time
//! - **Observer Pattern**: Uses Bevy 0.15's observer-based screenshot API
//! - **Event Integration**: Uses `SaveSuccessEvent` for user feedback
//!
//! ## Architecture
//!
//! The screenshot process uses Bevy 0.15's observer pattern:
//!
//! 1. **Request Phase**: User triggers screenshot via keyboard shortcut
//! 2. **Capture Phase**: Bevy captures screenshot and triggers observer
//! 3. **Save Phase**: Observer saves image to disk with timestamp
//!
//! This ensures the frame is fully rendered before capture, avoiding the
//! blank screenshot issue from previous implementations.

#[cfg(not(target_arch = "wasm32"))]
use bevy::{
    prelude::*,
    render::view::screenshot::{save_to_disk, Screenshot, ScreenshotCaptured},
};

#[cfg(not(target_arch = "wasm32"))]
use crate::events::SaveSuccessEvent;

/// Component to track the screenshot filename on the screenshot entity
#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
struct ScreenshotPath(String);

/// System that initiates screenshot capture when triggered.
///
/// This system spawns a Screenshot component targeting the primary window
/// with an observer that saves the screenshot to disk when ready.
///
/// # Process
///
/// 1. Generate timestamped filename
/// 2. Spawn Screenshot entity with observer and path component
/// 3. Observer saves screenshot when capture completes
/// 4. Send success event for toast notification
///
/// # Note
///
/// This system uses Bevy 0.15's observer pattern which automatically handles
/// the screenshot lifecycle, including entity cleanup after capture.
#[cfg(not(target_arch = "wasm32"))]
pub fn take_screenshot(mut commands: Commands) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let path = format!("bert_screenshot_{}.png", timestamp);
    
    info!("Initiating screenshot capture: {}", path);
    
    // Spawn screenshot entity with save observer and path component
    commands
        .spawn((Screenshot::primary_window(), ScreenshotPath(path.clone())))
        .observe(save_to_disk(path.clone()))
        .observe(screenshot_saved_handler);
    
    info!("Screenshot scheduled for: {}", path);
}

/// Observer function that handles screenshot completion and sends success events.
///
/// This observer is triggered after the screenshot is captured and saved to disk.
/// It sends a `SaveSuccessEvent` to notify the user via toast notification.
///
/// # Parameters
///
/// - `trigger`: Contains the captured screenshot event with image data
/// - `path_query`: Query to get the screenshot path from the entity
/// - `save_events`: Event writer for sending success notifications
#[cfg(not(target_arch = "wasm32"))]
fn screenshot_saved_handler(
    trigger: Trigger<ScreenshotCaptured>,
    path_query: Query<&ScreenshotPath>,
    mut save_events: EventWriter<SaveSuccessEvent>,
) {
    let entity = trigger.entity();
    if let Ok(screenshot_path) = path_query.get(entity) {
        let path = &screenshot_path.0;
        info!("Screenshot saved successfully: {}", path);
        
        save_events.send(SaveSuccessEvent {
            file_path: Some(path.clone()),
            message: format!("Screenshot saved: {}", path),
        });
    }
}


