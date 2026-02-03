use crate::bevy_app::plugins::mouse_interaction::PickSelection;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Fill, Shape, Stroke};

/// Stores idle and selected visual states for lyon Shape entities.
///
/// In bevy_prototype_lyon 0.15, Fill and Stroke are no longer Components.
/// Instead, they are fields on the Shape component. This struct stores
/// the highlight values and the system applies them to Shape directly.
#[derive(Component, Clone, Default)]
pub struct HighlightBundles {
    pub idle_stroke: Option<Stroke>,
    pub selected_stroke: Option<Stroke>,
    pub idle_fill: Option<Fill>,
    pub selected_fill: Option<Fill>,
}

/// Applies the correct highlight (idle or selected) to the entity's Shape component
/// based on the current PickSelection state.
pub fn apply_highlight_bundles(
    mut query: Query<
        (&HighlightBundles, &PickSelection, &mut Shape),
        Or<(
            Changed<HighlightBundles>,
            Added<HighlightBundles>,
            Changed<PickSelection>,
        )>,
    >,
) {
    for (bundles, selection, mut shape) in &mut query {
        if selection.is_selected {
            if let Some(stroke) = &bundles.selected_stroke {
                shape.stroke = Some(*stroke);
            }
            if let Some(fill) = &bundles.selected_fill {
                shape.fill = Some(*fill);
            }
        } else {
            if let Some(stroke) = &bundles.idle_stroke {
                shape.stroke = Some(*stroke);
            }
            if let Some(fill) = &bundles.idle_fill {
                shape.fill = Some(*fill);
            }
        }
    }
}
