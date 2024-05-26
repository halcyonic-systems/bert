use crate::plugins::mouse_interaction::PickTarget;
use bevy::prelude::*;
use bevy::text::{BreakLineOn, Text2dBounds};
use copy_position::copy_position;
use text::{apply_text_color_contrast, copy_name_to_label};

mod copy_position;
mod text;

pub use copy_position::CopyPosition;
pub use text::{LabelContainer, NameLabel, AutoContrastTextColor};

pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (copy_position, copy_name_to_label, apply_text_color_contrast),
        )
        .register_type::<NameLabel>()
        .register_type::<CopyPosition>();
    }
}

#[inline(always)]
pub fn add_name_label<B: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    size: Vec2,
    offset: Vec3,
    name_query: &Query<&Name>,
    asset_server: &Res<AssetServer>,
    additional_bundle: B,
) {
    add_name_label_common(
        commands,
        entity,
        size,
        offset,
        name_query,
        asset_server,
        additional_bundle,
        (),
    );
}

#[inline(always)]
pub fn add_name_label_with_auto_contrast<B: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    size: Vec2,
    offset: Vec3,
    name_query: &Query<&Name>,
    asset_server: &Res<AssetServer>,
    text_color: AutoContrastTextColor,
    additional_bundle: B,
) {
    add_name_label_common(
        commands,
        entity,
        size,
        offset,
        name_query,
        asset_server,
        additional_bundle,
        text_color,
    );
}

fn add_name_label_common<B1: Bundle, B2: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    size: Vec2,
    offset: Vec3,
    name_query: &Query<&Name>,
    asset_server: &Res<AssetServer>,
    additional_sprite_bundle: B1,
    additional_text_bundle: B2,
) {
    let text = Text {
        sections: vec![TextSection::new(
            &name_query
                .get(entity)
                .expect("Entity should have a name")
                .to_string(),
            TextStyle {
                font: asset_server
                    .load("fonts/Fira_Sans/FiraSans-Bold.ttf")
                    .clone(),
                font_size: 16.0,
                color: Color::BLACK,
            }
            .clone(),
        )],
        justify: JustifyText::Left,
        linebreak_behavior: BreakLineOn::WordBoundary,
    };

    let text_entity = commands
        .spawn((
            Text2dBundle {
                text,
                text_2d_bounds: Text2dBounds { size },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            Name::new("Label Text"),
            additional_text_bundle,
        ))
        .id();

    let sprite_entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0., 0., 0., 0.),
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },
            Name::new("Label Sprite"),
            PickTarget { target: entity },
            additional_sprite_bundle,
            LabelContainer,
        ))
        .push_children(&[text_entity])
        .id();

    commands.entity(entity).insert((
        CopyPosition {
            target: sprite_entity,
            offset,
            local_offset: true,
        },
        NameLabel { label: text_entity },
    ));
}
