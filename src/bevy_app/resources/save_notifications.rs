use crate::events::SaveSuccessEvent;
use bevy::prelude::*;
use std::sync::{mpsc, Arc};

/// Resource that holds a channel receiver for save success events from async contexts
#[derive(Resource)]
pub struct SaveNotificationChannel {
    pub receiver: Arc<std::sync::Mutex<mpsc::Receiver<SaveSuccessEvent>>>,
}

/// Global sender that async tasks can use to send save success events
static SAVE_NOTIFICATION_SENDER: std::sync::OnceLock<mpsc::Sender<SaveSuccessEvent>> = std::sync::OnceLock::new();

/// Initialize the save notification channel system
pub fn init_save_notification_channel(commands: &mut Commands) {
    let (sender, receiver) = mpsc::channel();
    
    // Store the sender globally for async tasks to use
    SAVE_NOTIFICATION_SENDER.set(sender).expect("Save notification channel should only be initialized once");
    
    // Insert the receiver as a Bevy resource
    commands.insert_resource(SaveNotificationChannel { 
        receiver: Arc::new(std::sync::Mutex::new(receiver))
    });
}

/// Send a save success event from an async context
pub fn send_save_success_event(event: SaveSuccessEvent) {
    if let Some(sender) = SAVE_NOTIFICATION_SENDER.get() {
        if let Err(e) = sender.send(event) {
            eprintln!("Failed to send save success event: {:?}", e);
        }
    }
}