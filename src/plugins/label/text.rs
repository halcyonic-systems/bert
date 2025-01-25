use bevy::math::vec2;
use bevy::prelude::*;
use bevy::text::TextLayoutInfo;
use bevy_prototype_lyon::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct NameLabel {
    pub label: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct AutoContrastTextColor {
    pub light_color: Color,
    pub dark_color: Color,
    pub lightness_threshold: f32,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct Background {
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
}

impl Default for AutoContrastTextColor {
    fn default() -> Self {
        Self {
            light_color: Color::WHITE,
            dark_color: Color::BLACK,
            lightness_threshold: 0.6,
        }
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct LabelContainer;

pub fn copy_name_to_label(
    source_query: Query<(&Name, &NameLabel), Or<(Changed<Name>, Added<NameLabel>)>>,
    mut target_query: Query<&mut Text2d>,
) {
    for (name, label) in &source_query {
        target_query
            .get_mut(label.label)
            .expect("Label should exist").0 = name.to_string();
    }
}

pub fn apply_text_color_contrast(
    source_query: Query<(&NameLabel, &Fill), Or<(Changed<Fill>, Added<NameLabel>)>>,
    mut target_query: Query<(&mut TextColor, &AutoContrastTextColor)>,
) {
    for (label, fill) in &source_query {
        if let Ok((mut color, text_color)) = target_query.get_mut(label.label) {
            let target_color = if Lcha::from(fill.color).luminance() < text_color.lightness_threshold {
                text_color.light_color
            } else {
                text_color.dark_color
            };

            color.0 = target_color;
        }
    }
}

pub fn update_background_size_from_label(
    text_query: Query<(&TextLayoutInfo, &Parent, &Background), Changed<TextLayoutInfo>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    for (text_layout, parent, background) in &text_query {
        if let Ok(mut sprite) = sprite_query.get_mut(parent.get()) {
            sprite.custom_size = Some(
                text_layout.size
                    + vec2(background.padding_horizontal, background.padding_vertical) * 2.0,
            );
        }
    }
}
