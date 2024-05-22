use crate::components::{
    Connection, CreateButton, Flow, HasFlowOtherEndButton, InterfaceSubsystemConnection,
    TargetTypeConnection,
};
use crate::plugins::lyon_selection::HighlightBundles;
use crate::{Interface, Subsystem};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

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
    mut arrow_query: Query<&mut Fill>,
) where
    C: Connection + TargetTypeConnection + Component,
{
    for (flow, mut highlight, children, external_entity_connection) in &mut query {
        let color = flow.substance_type.flow_color();
        highlight.idle.color = color;
        highlight.selected.color = color;

        for child in children.iter() {
            if let Ok(mut fill) = arrow_query.get_mut(*child) {
                fill.color = color;
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

pub fn update_interface_color_from_flow<C>(
    mut query: Query<(&Flow, &C), Or<(Added<Flow>, Changed<Flow>)>>,
    mut interface_query: Query<&mut Fill, (Without<Flow>, With<Interface>)>,
) where
    C: Connection + Component,
{
    for (flow, interface_connection) in &mut query {
        if let Ok(mut interface_fill) = interface_query.get_mut(interface_connection.target()) {
            interface_fill.color = flow.substance_type.interface_color();
        }
    }
}

pub fn update_interface_subsystem_color_from_interface(
    interface_query: Query<
        (&Fill, &InterfaceSubsystemConnection),
        Or<(Changed<Fill>, Added<InterfaceSubsystemConnection>)>,
    >,
    mut subsystem_query: Query<&mut Fill, Without<InterfaceSubsystemConnection>>,
) {
    for (interface_fill, subsystem_connection) in &interface_query {
        let mut subsystem_fill = subsystem_query
            .get_mut(subsystem_connection.target())
            .expect("Subsystem should exist");
        subsystem_fill.color = interface_fill.color;
    }
}

pub fn update_system_color_from_subsystem(
    subsystem_query: Query<&Subsystem, Added<Subsystem>>,
    mut fill_query: Query<&mut Fill>,
) {
    for subsystem in &subsystem_query {
        let mut system_fill = fill_query
            .get_mut(subsystem.parent_system)
            .expect("System should exist");
        system_fill.color = Color::rgb(235.0, 231.0, 231.0);
    }
}
