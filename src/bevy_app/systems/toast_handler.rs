use crate::bevy_app::resources::SaveNotificationChannel;
use crate::events::SaveSuccessEvent;
use bevy::prelude::*;

/// System that handles save success events from both sync and async contexts
/// Events are automatically forwarded to Leptos via the export_event_to_leptos setup
pub fn handle_save_success_events(
    mut events: EventReader<SaveSuccessEvent>,
    mut save_events: EventWriter<SaveSuccessEvent>,
    save_channel: Option<Res<SaveNotificationChannel>>,
) {
    // Handle events from sync contexts (like web saves)
    for event in events.read() {
        info!("Save success: {}", event.message);
        if let Some(path) = &event.file_path {
            info!("File saved to: {}", path);
        } else {
            info!("File downloaded successfully");
        }
        // Event forwarding to Leptos happens automatically via export_event_to_leptos
    }

    // Handle events from async contexts (like desktop saves)
    if let Some(channel) = save_channel {
        if let Ok(receiver_guard) = channel.receiver.lock() {
            while let Ok(event) = receiver_guard.try_recv() {
                info!("Async save success: {}", event.message);
                if let Some(path) = &event.file_path {
                    info!("File saved to: {}", path);
                } else {
                    info!("File downloaded successfully");
                }

                // Re-emit the event so it gets forwarded to Leptos
                save_events.send(event);
            }
        }
    }
}
