//! This file holds the systems that control the color of system elements.
use crate::bevy_app::components::{
    Connection, CreateButton, Flow, HasFlowOtherEndButton, InterfaceSubsystem, TargetTypeConnection,
};
use crate::bevy_app::plugins::lyon_selection::HighlightBundles;
use crate::bevy_app::{Hidden, Interface, Subsystem};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

/// Update the color of a flow and it's connected external entities based on the flow substance type.
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
    mut external_entity_query: Query<&mut HighlightBundles<Stroke, Stroke>, Without<Flow>>,
    mut arrow_query: Query<(&mut Fill, Option<&Hidden>)>,
) where
    C: Connection + TargetTypeConnection + Component,
{
    for (flow, mut highlight, children, external_entity_connection) in &mut query {
        let color = flow.substance_type.flow_color();
        highlight.idle.color = color;
        highlight.selected.color = color;

        for child in children.iter() {
            if let Ok((mut fill, hidden)) = arrow_query.get_mut(*child) {
                fill.color = color;
                if hidden.is_some() {
                    fill.color.set_alpha(0.2);
                }
            }
        }

        if let Some(external_entity_connection) = external_entity_connection {
            if external_entity_connection.target_is_external_entity() {
                let mut external_entity_highlight = external_entity_query
                    .get_mut(external_entity_connection.target())
                    .expect("External entity should exist");

                external_entity_highlight.idle.color = color;
                external_entity_highlight.selected.color = color;
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
            interface_fill.color = flow.substance_type.interface_color();
            if hidden.is_some() {
                interface_fill.color.set_alpha(0.2);
            }
        }
    }
}

/// Update the color of an interface subsystem based on the substance type of the parent interface.
/// Also hide (transparent) the interface subsystem if it's marked `Hidden`.
pub fn update_interface_subsystem_color(
    mut interface_subsystem_query: Query<
        (Entity, &mut Fill, &InterfaceSubsystem, Option<&Hidden>),
        Changed<InterfaceSubsystem>,
    >,
    subsystem_query: Query<&Subsystem>,
) {
    'outer: for (system_entity, mut subsystem_fill, interface_subsystem, hidden) in
        &mut interface_subsystem_query
    {
        for subsystem in &subsystem_query {
            if subsystem.parent_system == system_entity {
                continue 'outer;
            }
        }
        subsystem_fill.color = interface_subsystem.substance_type.interface_color();
        if hidden.is_some() {
            subsystem_fill.color.set_alpha(0.2);
        }
    }
}

/// Update the color of a subsystem based on the color of the parent system.
pub fn update_system_color_from_subsystem(
    subsystem_query: Query<&Subsystem, Added<Subsystem>>,
    mut fill_query: Query<(&mut Fill, Option<&Hidden>)>,
) {
    for subsystem in &subsystem_query {
        let (mut system_fill, hidden) = fill_query
            .get_mut(subsystem.parent_system)
            .expect("System should exist");

        if hidden.is_some() {
            system_fill.color = Color::srgba(235.0, 231.0, 231.0, 0.2)
        } else {
            system_fill.color = Color::srgb(235.0, 231.0, 231.0);
        }
    }
}
