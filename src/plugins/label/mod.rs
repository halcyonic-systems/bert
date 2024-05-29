use crate::plugins::mouse_interaction::PickTarget;
use bevy::prelude::*;
use bevy::text::{update_text2d_layout, BreakLineOn, Text2dBounds};
mod copy_position;
mod text;

use crate::constants::LABEL_Z;
pub use copy_position::*;
pub use text::*;

pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                copy_position,
                compute_text_alignment,
                copy_name_to_label,
                apply_text_color_contrast,
                update_background_size_from_label.after(update_text2d_layout),
            ),
        )
        .register_type::<NameLabel>()
        .register_type::<CopyPosition>()
        .register_type::<AutoContrastTextColor>()
        .register_type::<LabelContainer>()
        .register_type::<Background>();
    }
}

pub struct CopyPositionArgs {
    pub offset: Vec3,
    pub horizontal_alignment: Alignment,
    pub vertical_alignment: Alignment,
}

pub struct BackgroundArgs {
    pub color: Color,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
}

impl Default for BackgroundArgs {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            padding_horizontal: 7.0,
            padding_vertical: 3.0,
        }
    }
}

pub fn add_name_label<B: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    text_bounds_size: Vec2,
    background: Option<BackgroundArgs>,
    copy_position: Option<CopyPositionArgs>,
    name_query: &Query<&Name>,
    asset_server: &Res<AssetServer>,
    text_color: Option<AutoContrastTextColor>,
    additional_bundle: B,
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

    let mut text_commands = commands.spawn((
        Text2dBundle {
            text,
            text_2d_bounds: Text2dBounds {
                size: text_bounds_size,
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Name::new("Label Text"),
    ));

    if let Some(text_color) = text_color {
        text_commands.insert(text_color);
    }

    let background_color = if let Some(background) = background {
        text_commands.insert(Background {
            padding_horizontal: background.padding_horizontal,
            padding_vertical: background.padding_vertical,
        });

        background.color
    } else {
        Color::NONE
    };

    let text_entity = text_commands.id();

    let sprite_entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: background_color,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, LABEL_Z),
                ..default()
            },
            Name::new("Label Sprite"),
            PickTarget { target: entity },
            additional_bundle,
            LabelContainer,
        ))
        .push_children(&[text_entity])
        .id();

    let mut entity_commands = commands.entity(entity);

    entity_commands.insert((NameLabel { label: text_entity },));

    if let Some(CopyPositionArgs {
        offset,
        horizontal_alignment,
        vertical_alignment,
    }) = copy_position
    {
        entity_commands.insert(CopyPosition {
            target: sprite_entity,
            offset,
            local_offset: true,
            horizontal_alignment,
            vertical_alignment,
        });
    }
}
