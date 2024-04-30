use crate::components::{Connection, HasSubstanceType};
use crate::plugins::lyon_selection::HighlightBundles;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn update_color_from_substance_type<F, C>(
    mut query: Query<
        (&F, &mut HighlightBundles<Stroke, Stroke>, &Children, &C),
        Or<(Added<F>, Changed<F>)>,
    >,
    mut external_entity_query: Query<&mut HighlightBundles<Stroke, Stroke>, Without<F>>,
    mut arrow_query: Query<&mut Fill>,
) where
    F: HasSubstanceType + Component,
    C: Connection + Component,
{
    for (inflow, mut highlight, children, external_entity_connection) in &mut query {
        let color = inflow.substance_type().flow_color();
        highlight.idle.color = color;
        highlight.selected.color = color;

        for child in children.iter() {
            if let Ok(mut fill) = arrow_query.get_mut(*child) {
                fill.color = color;
            }
        }

        let mut external_entity_highlight = external_entity_query
            .get_mut(external_entity_connection.target())
            .expect("External entity should exist");

        external_entity_highlight.idle.color = color;
        external_entity_highlight.selected.color = color;
    }
}
