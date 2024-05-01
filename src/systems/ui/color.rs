use crate::components::{
    Connection, CreateButton, FlowOtherEndButton, HasSubstanceType, InterfaceSubsystemConnection,
};
use crate::plugins::lyon_selection::HighlightBundles;
use crate::Interface;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn update_color_from_substance_type<F, C>(
    mut query: Query<
        (
            &F,
            &mut HighlightBundles<Stroke, Stroke>,
            &Children,
            Option<&C>,
        ),
        Or<(Added<F>, Changed<F>)>,
    >,
    mut external_entity_query: Query<&mut HighlightBundles<Stroke, Stroke>, Without<F>>,
    mut arrow_query: Query<&mut Fill>,
) where
    F: HasSubstanceType + Component,
    C: Connection + Component,
{
    for (flow, mut highlight, children, external_entity_connection) in &mut query {
        let color = flow.substance_type().flow_color();
        highlight.idle.color = color;
        highlight.selected.color = color;

        for child in children.iter() {
            if let Ok(mut fill) = arrow_query.get_mut(*child) {
                fill.color = color;
            }
        }

        if let Some(external_entity_connection) = external_entity_connection {
            let mut external_entity_highlight = external_entity_query
                .get_mut(external_entity_connection.target())
                .expect("External entity should exist");

            external_entity_highlight.idle.color = color;
            external_entity_highlight.selected.color = color;
        }
    }
}

pub fn update_button_substance_type_from_flow<F>(
    flow_query: Query<(Entity, &F), (Changed<F>, With<FlowOtherEndButton>)>,
    mut button_query: Query<&mut CreateButton>,
) where
    F: HasSubstanceType + Component,
{
    for (flow_entity, flow) in &flow_query {
        for mut button in &mut button_query {
            if button.connection_source == flow_entity {
                button.substance_type = Some(flow.substance_type());
            }
        }
    }
}

pub fn update_interface_color_from_flow<F, C>(
    mut query: Query<(&F, &C), Or<(Added<F>, Changed<F>)>>,
    mut interface_query: Query<&mut Fill, (Without<F>, With<Interface>)>,
) where
    F: HasSubstanceType + Component,
    C: Connection + Component,
{
    for (flow, interface_connection) in &mut query {
        let color = flow.substance_type().interface_color();

        let mut interface_fill = interface_query
            .get_mut(interface_connection.target())
            .expect("Interface should exist");
        interface_fill.color = color;
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
