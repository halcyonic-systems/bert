//! Custom defined events
use crate::bevy_app::data_model::WorldModel;
use crate::bevy_app::plugins::mouse_interaction::DragPosition;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Helper macro that defines and implements custom bevy events for the dragging feature.
macro_rules! impl_drag_event {
    ($name:ident) => {
        #[derive(Event, Debug, Clone)]
        pub struct $name {
            pub target: Entity,
            pub position: Vec2,
        }

        impl From<Trigger<'_, DragPosition>> for $name {
            fn from(trigger: Trigger<'_, DragPosition>) -> Self {
                Self {
                    target: trigger.entity(),
                    position: trigger.local_position,
                }
            }
        }
    };
}

impl_drag_event!(ExternalEntityDrag);
impl_drag_event!(InterfaceDrag);
impl_drag_event!(SubsystemDrag);

/// Fires when an entity is removed from the world. Used in system control flow.
#[derive(Event, Debug, Clone, Copy)]
pub struct RemoveEvent;

/// Fires from leptos to bevy to detach a marker label from an entity.
#[derive(Event, Debug, Clone, Copy)]
pub struct DetachMarkerLabelEvent;

/// Fires from leptos to bevy to load a file.
#[derive(Event, Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoadFileEvent {
    pub file_path: String,
    pub data: Vec<u8>,
}

#[derive(Event, Clone)]
pub struct TreeEvent {
    pub world_model: WorldModel,
}

#[derive(Event, Debug, Clone)]
pub enum TriggerEvent {
    ShowTree,
}
