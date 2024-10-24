//! Custom defined events
use crate::plugins::mouse_interaction::DragPosition;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

/// Helper macro that defines and implements custom bevy events for the dragging feature.
macro_rules! impl_drag_event {
    ($name:ident) => {
        #[derive(Event, Debug, Clone)]
        pub struct $name {
            pub target: Entity,
            orig_target: Entity,
            pub position: Vec2,
        }

        impl $name {
            pub fn has_bubbled(&self) -> bool {
                self.target != self.orig_target
            }
        }

        impl From<ListenerInput<DragPosition>> for $name {
            fn from(event: ListenerInput<DragPosition>) -> Self {
                Self {
                    target: event.listener(),
                    orig_target: event.target(),
                    position: event.local_position,
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
