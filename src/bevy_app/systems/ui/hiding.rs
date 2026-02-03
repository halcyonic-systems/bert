use bevy::prelude::*;

use crate::bevy_app::constants::HIDDING_TRANSPARENCY;
use crate::bevy_app::plugins::lyon_selection::HighlightBundles;
use crate::{Hidden, SelectedHighlightHelperAdded, SystemElement};
use bevy_prototype_lyon::prelude::*;

pub fn update_hiding_state(
    mut added_hiding_state: Query<
        (Option<&mut Shape>, &mut HighlightBundles, Option<&Children>),
        (Added<Hidden>, With<SystemElement>),
    >,
    mut shape_query: Query<&mut Shape, Without<SystemElement>>,
) {
    for (shape, mut highlight_bundle, children) in &mut added_hiding_state {
        if let Some(mut shape) = shape {
            if let Some(ref mut fill) = shape.fill {
                fill.color.set_alpha(HIDDING_TRANSPARENCY);
            }
        }
        if let Some(ref mut stroke) = highlight_bundle.idle_stroke {
            stroke.color.set_alpha(HIDDING_TRANSPARENCY);
        }
        if let Some(ref mut stroke) = highlight_bundle.selected_stroke {
            stroke.color.set_alpha(HIDDING_TRANSPARENCY);
        }

        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut child_shape) = shape_query.get_mut(child) {
                    if let Some(ref mut fill) = child_shape.fill {
                        fill.color.set_alpha(HIDDING_TRANSPARENCY);
                    }
                }
            }
        }
    }
}

pub fn hide_selected(
    mut commands: Commands,
    mut selected_system_query: Query<
        Entity,
        (
            With<SelectedHighlightHelperAdded>,
            Without<Hidden>,
            With<SystemElement>,
        ),
    >,
) {
    for entity in &mut selected_system_query {
        commands.entity(entity).insert(Hidden);
    }
}

pub fn un_hide_selected(
    mut commands: Commands,
    mut selected_system_query: Query<
        (Entity, Option<&mut Shape>, &mut HighlightBundles, &Children),
        (
            With<SelectedHighlightHelperAdded>,
            With<Hidden>,
            With<SystemElement>,
        ),
    >,
    mut shape_query: Query<&mut Shape, Without<SystemElement>>,
) {
    for (entity, shape, mut highlight_bundle, children) in &mut selected_system_query {
        if let Some(mut shape) = shape {
            if let Some(ref mut fill) = shape.fill {
                fill.color.set_alpha(1.0);
            }
        }
        if let Some(ref mut stroke) = highlight_bundle.idle_stroke {
            stroke.color.set_alpha(1.0);
        }
        if let Some(ref mut stroke) = highlight_bundle.selected_stroke {
            stroke.color.set_alpha(1.0);
        }

        for child in children.iter() {
            if let Ok(mut child_shape) = shape_query.get_mut(child) {
                if let Some(ref mut fill) = child_shape.fill {
                    fill.color.set_alpha(1.0);
                }
            }
        }

        commands.entity(entity).remove::<Hidden>();
    }
}
