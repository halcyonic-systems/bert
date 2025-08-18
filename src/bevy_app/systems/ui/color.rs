//! This file holds the systems that control the color of system elements.
use crate::bevy_app::components::{
    Connection, CreateButton, Flow, HasFlowOtherEndButton, TargetTypeConnection,
};
use crate::bevy_app::constants::theme::*;
use crate::bevy_app::constants::HIDDING_TRANSPARENCY;
use crate::bevy_app::plugins::lyon_selection::HighlightBundles;
use crate::bevy_app::resources::Theme;
use crate::bevy_app::{Hidden, Interface, Subsystem};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

/// Update the color of a flow and it's connected external entities based on the flow substance type.
/// Uses original BERT colors regardless of background theme.
pub fn update_color_from_substance_type<C>(
    mut query: Query<
        (
            &Flow,
            &mut HighlightBundles<Stroke, Stroke>,
            &Children,
            Option<&C>,
        ),
        Or<(Added<Flow>, Changed<Flow>)>,
    >,
    mut external_entity_query: Query<
        (&mut HighlightBundles<Stroke, Stroke>, Option<&Hidden>),
        Without<Flow>,
    >,
    mut arrow_query: Query<(&mut Fill, Option<&Hidden>)>,
) where
    C: Connection + TargetTypeConnection + Component,
{
    for (flow, mut highlight, children, external_entity_connection) in &mut query {
        // Use original BERT colors - no theme dependency
        let color = flow.substance_type.flow_color_default();
        highlight.idle.color = color;
        highlight.selected.color = color;

        for child in children.iter() {
            if let Ok((mut fill, hidden)) = arrow_query.get_mut(*child) {
                fill.color = color;
                if hidden.is_some() {
                    fill.color.set_alpha(HIDDING_TRANSPARENCY);
                }
            }
        }

        if let Some(external_entity_connection) = external_entity_connection {
            if external_entity_connection.target_is_external_entity() {
                let (mut external_entity_highlight, hidden) = external_entity_query
                    .get_mut(external_entity_connection.target())
                    .expect("External entity should exist");

                external_entity_highlight.idle.color = color;
                external_entity_highlight.selected.color = color;

                if hidden.is_some() {
                    external_entity_highlight
                        .idle
                        .color
                        .set_alpha(HIDDING_TRANSPARENCY);
                    external_entity_highlight
                        .selected
                        .color
                        .set_alpha(HIDDING_TRANSPARENCY);
                }
            }
        }
    }
}

pub fn update_button_substance_type_from_flow(
    flow_query: Query<(Entity, &Flow), (Changed<Flow>, With<HasFlowOtherEndButton>)>,
    mut button_query: Query<&mut CreateButton>,
) {
    for (flow_entity, flow) in &flow_query {
        for mut button in &mut button_query {
            if button.connection_source == flow_entity {
                button.substance_type = Some(flow.substance_type);
            }
        }
    }
}

/// Update the color of an interface based the flow substance type.
/// Uses original BERT colors regardless of background theme.
pub fn update_interface_color_from_flow<C>(
    mut query: Query<(&Flow, &C), Or<(Added<Flow>, Changed<Flow>)>>,
    mut interface_query: Query<(&mut Fill, Option<&Hidden>), (Without<Flow>, With<Interface>)>,
) where
    C: Connection + Component,
{
    for (flow, interface_connection) in &mut query {
        if let Ok((mut interface_fill, hidden)) =
            interface_query.get_mut(interface_connection.target())
        {
            // Use original BERT colors - no theme dependency
            interface_fill.color = flow.substance_type.interface_color_default();
            if hidden.is_some() {
                interface_fill.color.set_alpha(HIDDING_TRANSPARENCY);
            }
        }
    }
}

/// Update the color of a subsystem based on the color of the parent system.
/// Uses original BERT colors regardless of background theme.
pub fn update_system_color_from_subsystem(
    subsystem_query: Query<&Subsystem, Added<Subsystem>>,
    mut fill_query: Query<(&mut Fill, Option<&Hidden>)>,
) {
    for subsystem in &subsystem_query {
        let (mut system_fill, hidden) = fill_query
            .get_mut(subsystem.parent_system)
            .expect("System should exist");

        // Use original BERT system color - no theme dependency
        system_fill.color = Color::srgb(0.92, 0.91, 0.91);

        if hidden.is_some() {
            system_fill.color.set_alpha(HIDDING_TRANSPARENCY);
        }
    }
}

/// System to update background color when theme changes
pub fn update_background_color_on_theme_change(
    theme: Res<Theme>,
    mut clear_color: ResMut<ClearColor>,
) {
    if !theme.is_changed() {
        return;
    }

    // Update only the background color
    clear_color.0 = match *theme {
        Theme::Normal => NORMAL_BACKGROUND,
        Theme::White => WHITE_BACKGROUND,
    };
}
