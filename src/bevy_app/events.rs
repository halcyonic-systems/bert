//! Custom defined events
use crate::bevy_app::data_model::WorldModel;
use crate::bevy_app::plugins::mouse_interaction::DragPosition;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Helper macro that defines and implements custom bevy messages for the dragging feature.
macro_rules! impl_drag_event {
    ($name:ident) => {
        #[derive(Message, Debug, Clone)]
        pub struct $name {
            pub target: Entity,
            pub position: Vec2,
        }

        impl $name {
            /// Construct from an observer `On<DragPosition>` event.
            pub fn from_on(on: &On<DragPosition>) -> Self {
                Self {
                    target: on.event().target,
                    position: on.event().local_position,
                }
            }
        }
    };
}

impl_drag_event!(ExternalEntityDrag);
impl_drag_event!(InterfaceDrag);
impl_drag_event!(SubsystemDrag);
impl_drag_event!(PaletteDrag);
impl_drag_event!(FlowEndpointHandleDrag);

/// Fires when an entity is removed from the world. Used in system control flow.
#[derive(Message, Debug, Clone, Copy)]
pub struct RemoveEvent;

/// Fires from leptos to bevy to detach a marker label from an entity.
#[derive(Message, Debug, Clone, Copy)]
pub struct DetachMarkerLabelEvent;

/// Fires from leptos to bevy to load a file.
#[derive(Message, Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoadFileEvent {
    pub file_path: String,
    pub data: Vec<u8>,
}

#[derive(Message, Clone)]
pub struct TreeEvent {
    pub world_model: WorldModel,
}

/// Fires when a model is successfully saved to indicate success to the user
#[derive(Message, Debug, Clone)]
pub struct SaveSuccessEvent {
    pub file_path: Option<String>,
    pub message: String,
}

#[derive(Message, Debug, Clone)]
pub enum TriggerEvent {
    ShowTree,
    ToggleTheme,
}

/// Fires from JavaScript/Leptos to Bevy to control zoom when Bevy keyboard handling fails
#[derive(Message, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ZoomEvent {
    ZoomIn,
    ZoomOut,
}

/// Fires from Leptos to Bevy to deselect all elements (clear selection state)
#[derive(Message, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DeselectAllEvent;

/// Fires from Leptos palette panel to Bevy to enter placement mode
#[derive(Message, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PaletteClickEvent {
    pub element_type: PaletteElementTypeEvent,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PaletteElementTypeEvent {
    Subsystem,
    Interface,
    EnvironmentalObject,
}

/// Fires from Bevy to Leptos when interaction mode changes
#[derive(Message, Debug, Clone, Serialize, Deserialize)]
pub struct ModeChangeEvent {
    pub mode_text: String,
}

/// Fires from Leptos/JavaScript to Bevy to cancel current mode (placement or connection)
#[derive(Message, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CancelModeEvent;
