//! Custom defined events
use crate::plugins::mouse_interaction::DragPosition;
use bevy::prelude::*;

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
