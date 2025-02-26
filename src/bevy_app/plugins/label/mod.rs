use crate::bevy_app::plugins::mouse_interaction::PickTarget;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::text::{update_text2d_layout, TextBounds};

mod copy_position;
mod text;

use crate::bevy_app::constants::LABEL_Z;
pub use copy_position::*;
pub use text::*;

pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                copy_position,
                copy_positions,
                copy_positions_changed,
                compute_text_alignment,
                compute_text_alignments,
                copy_name_to_label,
                apply_text_color_contrast,
                update_background_size_from_label.after(update_text2d_layout),
            ),
        )
        .register_type::<NameLabel>()
        .register_type::<CopyPosition>()
        .register_type::<CopyPositions>()
        .register_type::<AutoContrastTextColor>()
        .register_type::<LabelContainer>()
        .register_type::<Background>();
    }
}

pub struct CopyPositionArgs {
    pub offset: Vec3,
    pub horizontal_alignment: Alignment,
    pub vertical_alignment: Alignment,
    pub horizontal_anchor: HorizontalAttachmentAnchor,
    pub vertical_anchor: VerticalAttachmentAnchor,
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
    multiple_copy_positions: bool,
    name_query: &Query<&Name>,
    asset_server: &Res<AssetServer>,
    text_color: Option<AutoContrastTextColor>,
    additional_bundle: B,
) {
    let mut text_commands = commands.spawn((
        Text2d::new(
            &name_query
                .get(entity)
                .expect("Entity should have a name")
                .to_string(),
        ),
        TextFont {
            font: asset_server
                .load("fonts/Fira_Sans/FiraSans-Bold.ttf")
                .clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor::BLACK,
        TextLayout {
            justify: JustifyText::Left,
            linebreak: LineBreak::WordBoundary,
        },
        TextBounds::from(text_bounds_size),
        Transform::from_xyz(0.0, 0.0, 1.0),
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
            Sprite {
                color: background_color,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, LABEL_Z),
            Name::new("Label Sprite"),
            PickTarget { target: entity },
            additional_bundle,
            LabelContainer,
        ))
        .add_children(&[text_entity])
        .id();

    let mut entity_commands = commands.entity(entity);

    entity_commands.insert((NameLabel { label: text_entity },));

    if let Some(CopyPositionArgs {
        offset,
        horizontal_alignment,
        vertical_alignment,
        vertical_anchor,
        horizontal_anchor,
    }) = copy_position
    {
        let copy_position = CopyPosition {
            target: sprite_entity,
            aabb: None,
            offset,
            local_offset: true,
            horizontal_alignment,
            vertical_alignment,
            horizontal_anchor,
            vertical_anchor,
        };

        if multiple_copy_positions {
            entity_commands.insert(CopyPositions(vec![copy_position]));
        } else {
            entity_commands.insert(copy_position);
        }
    }
}

pub fn add_marker_with_text<B: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    copy_positions: &mut CopyPositions,
    aabb: &Aabb,
    sprite_size: Vec2,
    copy_position: Option<CopyPositionArgs>,
    text: &str,
    asset_path: &str,
    asset_server: &Res<AssetServer>,
    text_color: Option<AutoContrastTextColor>,
    additional_bundle: B,
) {
    let mut text_commands = commands.spawn((
        Text2d::new(text),
        TextFont {
            font: asset_server
                .load("fonts/Fira_Sans/FiraSans-Bold.ttf")
                .clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor::BLACK,
        TextLayout {
            justify: JustifyText::Center,
            linebreak: LineBreak::WordBoundary,
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Name::new("Label Text"),
    ));

    if let Some(text_color) = text_color {
        text_commands.insert(text_color);
    }

    let text_entity = text_commands.id();

    let sprite_entity = commands
        .spawn((
            Sprite {
                image: asset_server.load(asset_path),
                custom_size: Some(sprite_size),
                ..default()
            },
            Name::new("Named Indicator"),
            PickTarget { target: entity },
            additional_bundle,
            LabelContainer,
        ))
        .add_children(&[text_entity])
        .id();

    let mut entity_commands = commands.entity(entity);

    entity_commands.insert((MarkerLabel { label: text_entity },));

    if let Some(CopyPositionArgs {
        offset,
        horizontal_alignment,
        vertical_alignment,
        vertical_anchor,
        horizontal_anchor,
    }) = copy_position
    {
        copy_positions.0.push(CopyPosition {
            target: sprite_entity,
            aabb: Some(*aabb),
            offset,
            local_offset: true,
            horizontal_alignment,
            vertical_alignment,
            horizontal_anchor,
            vertical_anchor,
        });
    }
}
