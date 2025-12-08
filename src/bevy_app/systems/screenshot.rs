//! Screenshot capture system for BERT diagrams.
//!
//! Provides cross-platform screenshot functionality using browser download APIs.
//! Works in both web browser and Tauri desktop contexts since BERT compiles to WASM
//! in both cases (Tauri wraps a WASM webview).
//!
//! ## Usage
//!
//! Press `Ctrl/Cmd+P` to capture a screenshot. The image will be downloaded as a
//! timestamped PNG file (e.g., `bert_screenshot_2025-01-15_14-30-00.png`).
//!
//! ## Architecture Note
//!
//! BERT's desktop build uses Tauri which wraps a WASM webview, meaning
//! `target_arch` is always `wasm32` even on desktop. This implementation uses
//! web APIs (Blob, anchor element download) rather than native file I/O to ensure
//! compatibility across all deployment contexts.

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::Extent3d,
        view::screenshot::{Screenshot, ScreenshotCaptured},
    },
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

use crate::events::SaveSuccessEvent;

/// Component to store the intended filename for a screenshot.
#[derive(Component)]
struct ScreenshotFilename(String);

/// Initiates a screenshot capture with a timestamped filename.
///
/// This system spawns a screenshot request using Bevy's `Screenshot` component
/// and attaches an observer to handle the captured image data.
pub fn take_screenshot(mut commands: Commands) {
    // Use JS Date API for WASM-compatible timestamps
    let js_date = js_sys::Date::new_0();
    let year = js_date.get_full_year() as u32;
    let month = js_date.get_month() as u32 + 1; // JS months are 0-indexed
    let day = js_date.get_date() as u32;
    let hour = js_date.get_hours() as u32;
    let minute = js_date.get_minutes() as u32;
    let second = js_date.get_seconds() as u32;

    let timestamp = format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        year, month, day, hour, minute, second
    );

    let filename = format!("bert_screenshot_{}.png", timestamp);
    info!("Initiating screenshot capture: {}", filename);

    commands
        .spawn((
            Screenshot::primary_window(),
            ScreenshotFilename(filename.clone()),
        ))
        .observe(screenshot_download_handler);
}

/// Observer handler that processes captured screenshot data and triggers a browser download.
///
/// This function:
/// 1. Extracts raw RGBA image data from Bevy's screenshot event
/// 2. Converts it to PNG format using the `image` crate
/// 3. Triggers a browser download via Blob and anchor element
fn screenshot_download_handler(
    trigger: Trigger<ScreenshotCaptured>,
    filename_query: Query<&ScreenshotFilename>,
    mut save_events: EventWriter<SaveSuccessEvent>,
) {
    let entity = trigger.entity();

    let Ok(screenshot_filename) = filename_query.get(entity) else {
        error!("Screenshot entity missing ScreenshotFilename component");
        return;
    };

    let filename = &screenshot_filename.0;
    let screenshot_data = &trigger.event().0;

    // Create Bevy Image from raw screenshot data
    let image = Image::new(
        Extent3d {
            width: screenshot_data.texture_descriptor.size.width,
            height: screenshot_data.texture_descriptor.size.height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        screenshot_data.data.clone(),
        screenshot_data.texture_descriptor.format,
        RenderAssetUsages::default(),
    );

    // Convert to PNG bytes
    let png_bytes = match image.clone().try_into_dynamic() {
        Ok(dynamic_image) => {
            let mut bytes: Vec<u8> = Vec::new();
            if let Err(e) = dynamic_image.write_to(
                &mut std::io::Cursor::new(&mut bytes),
                image::ImageFormat::Png,
            ) {
                error!("Failed to encode screenshot as PNG: {}", e);
                return;
            }
            bytes
        }
        Err(e) => {
            error!("Failed to convert screenshot to dynamic image: {}", e);
            return;
        }
    };

    // Trigger browser download
    if let Err(e) = trigger_browser_download(&png_bytes, filename) {
        error!("Failed to trigger browser download: {:?}", e);
        return;
    }

    info!("Screenshot saved to Downloads: {}", filename);

    save_events.send(SaveSuccessEvent {
        file_path: Some(filename.clone()),
        message: format!("Screenshot saved to Downloads: {}", filename),
    });
}

/// Triggers a browser download by creating a Blob URL and clicking a hidden anchor element.
///
/// This approach works in both web browsers and Tauri's webview context.
fn trigger_browser_download(png_bytes: &[u8], filename: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window available"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document available"))?;

    // Create Uint8Array from PNG bytes
    let uint8_array = js_sys::Uint8Array::from(png_bytes);
    let array = js_sys::Array::new();
    array.push(&uint8_array);

    // Create Blob with PNG MIME type
    let blob_options = BlobPropertyBag::new();
    blob_options.set_type("image/png");
    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &blob_options)?;

    // Create object URL for the blob
    let url = Url::create_object_url_with_blob(&blob)?;

    // Create hidden anchor element and trigger download
    let anchor = document
        .create_element("a")?
        .dyn_into::<HtmlAnchorElement>()?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.style().set_property("display", "none")?;

    // Append to body, click, and cleanup
    let body = document
        .body()
        .ok_or_else(|| JsValue::from_str("No body element"))?;

    body.append_child(&anchor)?;
    anchor.click();
    body.remove_child(&anchor)?;

    // Revoke the object URL to free memory
    Url::revoke_object_url(&url)?;

    Ok(())
}
