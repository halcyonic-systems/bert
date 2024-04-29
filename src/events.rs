use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

macro_rules! impl_drag_event {
    ($name:ident) => {
        #[derive(Event)]
        pub struct $name {
            pub target: Entity,
            orig_target: Entity,
            pub delta: Vec2,
        }

        impl $name {
            pub fn has_bubbled(&self) -> bool {
                self.target != self.orig_target
            }
        }

        impl From<ListenerInput<Pointer<Drag>>> for $name {
            fn from(event: ListenerInput<Pointer<Drag>>) -> Self {
                Self {
                    target: event.listener(),
                    orig_target: event.target(),
                    delta: event.delta,
                }
            }
        }
    };
}

impl_drag_event!(ExternalEntityDrag);
impl_drag_event!(InterfaceDrag);