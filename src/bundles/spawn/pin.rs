use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

pub fn spawn_pin(parent: &Entity, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let path = "todo-pin.png";
    let name = "Pin";
    let pin_width = BUTTON_WIDTH_HALF * 2.0;
    let pin_entity = commands
        .spawn((
            Pin { target: *parent },
            Name::new(name),
            SpriteBundle {
                texture: asset_server.load(path),
                transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(pin_width, pin_width)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    commands.entity(*parent).push_children(&[pin_entity]);
}
