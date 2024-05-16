use bevy::{
    ecs::component,
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};

use crate::{constants::*, ExternalEntity, NestingLevel, SystemElement};

#[derive(Component)]
pub struct TextLabel;

pub fn update_external_entity_label(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    name_query: Query<&Name, With<ExternalEntity>>,
    without_label_query: Query<
        (Entity, &NestingLevel, &Transform),
        (With<ExternalEntity>, Without<TextLabel>),
    >,
) {
    without_label_query
        .iter()
        .for_each(|(e, level, transform)| {
            let name = name_query.get(e).expect("external entity should have name");
            let position = transform.translation;

            spawn_text_label(
                SystemElement::ExternalEntity,
                &name,
                &level,
                //&position,
                &e,
                &mut commands,
                &asset_server,
            );
        })
}

pub fn spawn_text_label(
    element_ty: SystemElement,
    label: &Name,
    level: &NestingLevel,
    //position: &Vec3,
    parent: &Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Fira_Sans/FiraSans-Bold.ttf");
    let font_size = 16.;
    let text_color = Color::WHITE;
    let justify = JustifyText::Left;
    let text_2d_bundle_transform = Transform::from_translation(Vec3::Z); // z_index should be surface z + 1

    let surface_color = Color::rgb(0.25, 0.25, 0.75);
    let mut surface_size = Vec2::new(150.0, 50.0);
    let mut surface_custom_size = Vec2::new(surface_size.x, surface_size.y);
    let surface_position = Vec2::ZERO;
    let mut surface_transform = Transform::from_translation(surface_position.extend(3.0)); // z_index should be 3.0 minimum

    type SE = SystemElement;
    match element_ty {
        SE::ExternalEntity => {
            surface_size = Vec2::new(70.0, 100.0);
            surface_custom_size = Vec2::new(surface_size.x, surface_size.y);
            surface_transform.translation.x += EXTERNAL_ENTITY_WIDTH_HALF;
        }
        _ => {}
    };

    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        color: text_color,
    };

    let text = Text {
        sections: vec![TextSection::new(label, text_style.clone())],
        justify,
        linebreak_behavior: BreakLineOn::WordBoundary,
    };

    let text_2d_bounds = Text2dBounds {
        // Wrap text in the rectangle
        size: surface_size,
    };

    let text_2d_bundle = Text2dBundle {
        text,
        text_2d_bounds,
        transform: text_2d_bundle_transform,
        ..default()
    };

    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: surface_color,
            custom_size: Some(surface_custom_size),
            ..default()
        },
        transform: surface_transform,
        ..default()
    };

    let text_entity = commands
        .spawn(text_2d_bundle)
        .insert(Name::new("Label Text"))
        .id();

    let sprite_entity = commands
        .spawn(sprite_bundle)
        .insert(Name::new("Label Sprite"))
        .push_children(&[text_entity])
        .id();

    commands
        .entity(*parent)
        .insert(TextLabel)
        .push_children(&[sprite_entity]);
}
