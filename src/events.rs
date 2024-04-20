use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct ExternalEntityDrag {
    pub target: Entity,
    pub delta: Vec2,
}

impl From<ListenerInput<Pointer<Drag>>> for ExternalEntityDrag {
    fn from(event: ListenerInput<Pointer<Drag>>) -> Self {
        Self {
            target: event.target,
            delta: event.delta,
        }
    }
}
