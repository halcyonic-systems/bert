use crate::resources::Zoom;
use bevy::prelude::*;

pub fn zoom_control_system(input: Res<ButtonInput<KeyCode>>, mut zoom: ResMut<Zoom>) {
    if input.just_pressed(KeyCode::Minus) {
        **zoom += 0.2;
    }

    if input.just_pressed(KeyCode::Equal) {
        **zoom -= 0.2;
    }
}
