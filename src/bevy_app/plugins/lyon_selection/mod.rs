use bevy::prelude::*;

mod highlight_bundles;

use crate::bevy_app::plugins::lyon_selection::highlight_bundles::apply_highlight_bundles;

pub use highlight_bundles::HighlightBundles;

pub struct LyonSelectionPlugin;

impl Plugin for LyonSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_highlight_bundles);
    }
}
