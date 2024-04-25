use crate::plugins::lyon_selection::spawn_on_selected::{selection_changed, spawn_on_selected};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod highlight_bundles;
mod spawn_on_selected;

use crate::plugins::lyon_selection::highlight_bundles::apply_highlight_bundles;

pub use highlight_bundles::HighlightBundles;
pub use spawn_on_selected::{SelectedSpawnListener, SpawnOnSelected};

pub struct LyonSelectionPlugin;

impl Plugin for LyonSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_on_selected.run_if(selection_changed),
                apply_highlight_bundles::<Stroke, Stroke>,
                apply_highlight_bundles::<Fill, Fill>,
                apply_highlight_bundles::<(Stroke, Fill), (Stroke, Fill)>,
                apply_highlight_bundles::<(Stroke, Fill), (Fill, Stroke)>,
                apply_highlight_bundles::<(Fill, Stroke), (Stroke, Fill)>,
                apply_highlight_bundles::<(Fill, Stroke), (Fill, Stroke)>,
            ),
        );
    }
}
