use crate::bevy_app::components::{Flow, NestingLevel};
use crate::bevy_app::constants::CLEAR_COLOR;
use crate::bevy_app::resources::Theme;
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
pub struct MarkerLabel {
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
            .expect("Label should exist")
            .0 = name.to_string();
    }
}

pub fn apply_text_color_contrast(
    source_query: Query<(&NameLabel, &Shape), Or<(Changed<Shape>, Added<NameLabel>)>>,
    mut target_query: Query<(&mut TextColor, &AutoContrastTextColor), With<Text2d>>,
) {
    for (label, shape) in &source_query {
        if let Some(ref fill) = shape.fill {
            if let Ok((mut color, text_color)) = target_query.get_mut(label.label) {
                if fill.color.alpha() < 1.0 {
                    continue;
                }
                let target_color =
                    if Lcha::from(fill.color).luminance() < text_color.lightness_threshold {
                        text_color.light_color
                    } else {
                        text_color.dark_color
                    };

                color.0 = target_color;
            }
        }
    }
}

pub fn update_background_size_from_label(
    text_query: Query<(&TextLayoutInfo, &ChildOf, &Background), Changed<TextLayoutInfo>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    for (text_layout, parent, background) in &text_query {
        if let Ok(mut sprite) = sprite_query.get_mut(parent.parent()) {
            sprite.custom_size = Some(
                text_layout.size
                    + vec2(background.padding_horizontal, background.padding_vertical) * 2.0,
            );
        }
    }
}

/// Update label background colors when theme changes to maintain contrast and visibility
pub fn update_label_background_on_theme_change(
    theme: Res<Theme>,
    flow_query: Query<(&NameLabel, &NestingLevel), With<Flow>>,
    mut sprite_query: Query<&mut Sprite, With<LabelContainer>>,
) {
    if !theme.is_changed() {
        return;
    }

    for (name_label, nesting_level) in &flow_query {
        if let Ok(mut sprite) = sprite_query.get_mut(name_label.label) {
            sprite.color = match *theme {
                Theme::Normal => {
                    // Original behavior - beige for level 0, white for others
                    if **nesting_level == 0 {
                        CLEAR_COLOR
                    } else {
                        Color::WHITE
                    }
                }
                Theme::White => {
                    // White background theme - use light gray for all labels
                    Color::srgb(0.95, 0.95, 0.95)
                }
            };
        }
    }
}
