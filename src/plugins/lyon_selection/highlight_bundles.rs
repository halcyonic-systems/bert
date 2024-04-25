use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Component, Default)]
pub struct HighlightBundles<IdleB = (), SelB = ()> {
    pub idle: IdleB,
    pub selected: SelB,
}

pub fn apply_highlight_bundles<IdleB, SelB>(
    mut commands: Commands,
    query: Query<
        (Entity, &HighlightBundles<IdleB, SelB>, &PickSelection),
        Or<(
            Changed<HighlightBundles<IdleB, SelB>>,
            Added<HighlightBundles<IdleB, SelB>>,
            Changed<PickSelection>,
        )>,
    >,
) where
    IdleB: Bundle + Clone,
    SelB: Bundle + Clone,
{
    for (entity, bundles, selection) in &query {
        if selection.is_selected {
            commands.entity(entity).insert(bundles.selected.clone());
        } else {
            commands.entity(entity).insert(bundles.idle.clone());
        }
    }
}
