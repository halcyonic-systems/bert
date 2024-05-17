use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};

use crate::{constants::*, ExternalEntity, NestingLevel, SystemElement};

#[derive(Component)]
pub struct TextLabel;

#[derive(Component)]
pub struct LabelBox;

#[derive(Component)]
pub struct Labeled;

pub fn add_external_entity_labels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    name_query: Query<&Name, With<ExternalEntity>>,
    without_label_query: Query<(Entity, &NestingLevel), (With<ExternalEntity>, Without<Labeled>)>,
) {
    for (e, level) in without_label_query.iter() {
        let name = name_query.get(e).expect("external entity should have name");
        spawn_text_label(
            SystemElement::ExternalEntity,
            &name,
            &level,
            &e,
            &mut commands,
            &asset_server,
        );
    }
}

pub fn spawn_text_label(
    element_ty: SystemElement,
    label: &Name,
    level: &NestingLevel,
    parent: &Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Fira_Sans/FiraSans-Bold.ttf");
    let font_size = 16.;
    let text_color = Color::BLACK;
    let justify = JustifyText::Left;
    let text_2d_bundle_transform = Transform::from_translation(Vec3::Z); // z_index should be surface z + 1

    let surface_color = Color::rgba(0., 0., 0., 0.); // Color::ANTIQUE_WHITE
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
        .insert(TextLabel)
        .id();

    let sprite_entity = commands
        .spawn(sprite_bundle)
        .insert(Name::new("Label Sprite"))
        .insert(LabelBox)
        .push_children(&[text_entity])
        .id();

    commands
        .entity(*parent)
        .insert(Labeled)
        .push_children(&[sprite_entity]);
}

pub fn update_label_rotations(
    text_query: Query<Entity, (With<Parent>, With<TextLabel>)>,
    labeled_query: Query<Entity, With<Labeled>>,
    children_query: Query<&Children>,
    mut text_transform_query: Query<&mut Transform, (With<Parent>, With<TextLabel>)>,
    labeled_transform_query: Query<&Transform, (With<Children>, Without<TextLabel>)>,
) {
    for labeled in labeled_query.iter() {
        let labeled_transform = labeled_transform_query
            .get(labeled)
            .expect("labeled should have transform");

        for child in children_query.iter_descendants(labeled) {
            if text_query.get(child).is_err() {
                continue;
            }

            let mut text_transform = text_transform_query
                .get_mut(child)
                .expect("Text should have transform");

            text_transform.rotation = labeled_transform.rotation.inverse();
        }
    }
}

pub fn update_label_text(
    text_query: Query<Entity, (With<Parent>, With<TextLabel>)>,
    labeled_query: Query<Entity, (Changed<Name>, With<Labeled>)>,
    children_query: Query<&Children>,
    mut content_query: Query<&mut Text, (With<Parent>, With<TextLabel>)>,
    labeled_name_query: Query<&Name, (Changed<Name>, With<Children>, With<Labeled>)>,
) {
    for labeled in labeled_query.iter() {
        let labeled_name = labeled_name_query
            .get(labeled)
            .expect("labeled should have name");

        for child in children_query.iter_descendants(labeled) {
            if text_query.get(child).is_err() {
                continue;
            }

            let mut text = content_query
                .get_mut(child)
                .expect("should have Text component");

            for section in &mut text.sections {
                section.value = labeled_name.to_string();
            }
        }
    }
}
