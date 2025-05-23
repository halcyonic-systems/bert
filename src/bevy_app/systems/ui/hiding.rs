use bevy::prelude::*;

use crate::bevy_app::constants::HIDDING_TRANSPARENCY;
use crate::bevy_app::plugins::lyon_selection::HighlightBundles;
use crate::{Hidden, SelectedHighlightHelperAdded, SystemElement};
use bevy_prototype_lyon::prelude::*;

pub fn update_hiding_state(
    mut added_hiding_state: Query<
        (
            Option<&mut Fill>,
            &mut HighlightBundles<Stroke, Stroke>,
            Option<&Children>,
        ),
        (Added<Hidden>, With<SystemElement>),
    >,
    mut fill_query: Query<&mut Fill, Without<SystemElement>>,
) {
    for (fill, mut highlight_bundle, children) in &mut added_hiding_state {
        if let Some(mut fill) = fill {
            fill.color.set_alpha(HIDDING_TRANSPARENCY);
        }
        highlight_bundle.idle.color.set_alpha(HIDDING_TRANSPARENCY);
        highlight_bundle
            .selected
            .color
            .set_alpha(HIDDING_TRANSPARENCY);

        if let Some(children) = children {
            for &child in children.iter() {
                if let Ok(mut fill) = fill_query.get_mut(child) {
                    fill.color.set_alpha(HIDDING_TRANSPARENCY);
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
        (
            Entity,
            Option<&mut Fill>,
            &mut HighlightBundles<Stroke, Stroke>,
            &Children,
        ),
        (
            With<SelectedHighlightHelperAdded>,
            With<Hidden>,
            With<SystemElement>,
        ),
    >,
    mut fill_query: Query<&mut Fill, Without<SystemElement>>,
) {
    for (entity, fill, mut highlight_bundle, children) in &mut selected_system_query {
        if let Some(mut fill) = fill {
            fill.color.set_alpha(1.0);
        }
        highlight_bundle.idle.color.set_alpha(1.0);
        highlight_bundle.selected.color.set_alpha(1.0);

        for &child in children.iter() {
            if let Ok(mut fill) = fill_query.get_mut(child) {
                fill.color.set_alpha(1.0);
            }
        }

        commands.entity(entity).remove::<Hidden>();
    }
}
