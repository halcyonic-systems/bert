use super::PickSelection;
use bevy::prelude::*;

#[derive(Resource, Clone, Deref, DerefMut, PartialEq, Eq, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct SelectedEntities(Vec<Entity>);

pub fn debug_selection(
    selection_query: Query<(Entity, &PickSelection), Changed<PickSelection>>,
    mut selected_entities: ResMut<SelectedEntities>,
) {
    for (entity, selection) in &selection_query {
        if selection.is_selected {
            if !selected_entities.contains(&entity) {
                selected_entities.push(entity);
            }
        } else {
            if let Some(idx) = selected_entities.iter().position(|e| *e == entity) {
                selected_entities.remove(idx);
            }
        }
    }
}
