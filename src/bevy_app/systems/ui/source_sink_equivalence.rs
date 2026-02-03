use bevy::prelude::*;
use std::collections::HashMap;

use crate::{
    bevy_app::resources::IsSameAsIdCounter,
    plugins::label::{CopyPositions, MarkerLabel},
    ExternalEntity, IsSameAsId, NestingLevel, SelectedHighlightHelperAdded,
};

/// Conditions:
/// - Making two external entities equal only works if they are in the same system.
/// - `IsSameAsIdCounter` will only be incremented if non of the selected entities have been assigned to a specific `IsSameAsId` yet.
/// - `IsSameAsIdCounter` will be added as `IsSameAsId` to selected external entites.
pub fn apply_sink_and_source_equivalence(
    mut commands: Commands,
    selected_external_entities: Query<
        (Entity, &NestingLevel, Option<&IsSameAsId>),
        (With<SelectedHighlightHelperAdded>, With<ExternalEntity>),
    >,
    mut equivalence_counter: ResMut<IsSameAsIdCounter>,
) {
    let (mut entities_with_id, mut entities_without_id) = (Vec::new(), Vec::new());

    for (entity, nesting_level, source_sink_equivalence_id) in selected_external_entities.iter() {
        if let Some(source_sink_equivalence_id) = source_sink_equivalence_id {
            entities_with_id.push((entity, nesting_level, source_sink_equivalence_id));
        } else {
            entities_without_id.push((entity, nesting_level));
        }
    }

    entities_with_id.sort_by_key(|(_, nesting_level, _)| ***nesting_level);
    entities_without_id.sort_by_key(|(_, nesting_level)| ***nesting_level);

    let mut nesting_level_of_interest = entities_with_id
        .first()
        .map(|(_, nesting_level, _)| *nesting_level);

    if nesting_level_of_interest.is_none() {
        nesting_level_of_interest = entities_without_id
            .first()
            .map(|(_, nesting_level)| *nesting_level);
    }

    if nesting_level_of_interest.is_none()
        || entities_with_id.len() + entities_without_id.len() < 2
        || entities_with_id
            .iter()
            .find(|(_, nesting_level, _)| *nesting_level != nesting_level_of_interest.unwrap())
            .is_some()
        || entities_without_id
            .iter()
            .find(|(_, nesting_level)| *nesting_level != nesting_level_of_interest.unwrap())
            .is_some()
    {
        return;
    }

    let id_of_interest = if let Some(id) = entities_with_id.first().map(|(_, _, id)| *id) {
        **id
    } else {
        **equivalence_counter += 1;
        **equivalence_counter
    };

    if entities_with_id
        .iter()
        .find(|(_, _, id)| ***id != id_of_interest)
        .is_some()
    {
        return;
    }

    for (entity, _) in entities_without_id {
        commands.entity(entity).insert(IsSameAsId(id_of_interest));
    }
}

pub fn update_and_cleanup_source_sink_equivalence(
    mut commands: Commands,
    removed_is_same_as_id: RemovedComponents<IsSameAsId>,
    mut copy_positions: Query<(&MarkerLabel, &mut CopyPositions)>,
    parent_query: Query<&ChildOf>,
    mut current_ids_query: Query<(Entity, &mut IsSameAsId)>,
    mut equivalence_counter: ResMut<IsSameAsIdCounter>,
) {
    if removed_is_same_as_id.len() > 0 {
        let mut id_to_entities: HashMap<usize, Vec<Entity>> = HashMap::new();
        for (entity, id) in current_ids_query.iter() {
            id_to_entities.entry(**id).or_insert(vec![]).push(entity);
        }

        for (id, entities) in id_to_entities.clone() {
            if entities.len() <= 1 {
                for entity in entities {
                    commands
                        .entity(entity)
                        .remove::<IsSameAsId>()
                        .remove::<MarkerLabel>();
                    if let Ok((marker_label, mut copy_positions)) = copy_positions.get_mut(entity) {
                        if let Ok(parent) = parent_query.get(marker_label.label) {
                            copy_positions
                                .0
                                .retain(|copy_position| copy_position.target != parent.parent());
                        }
                    }
                    id_to_entities.remove(&id);
                }
            }
        }

        let mut ids = id_to_entities.iter().collect::<Vec<_>>();

        ids.sort_by_key(|&(id, _)| *id);

        for (i, (_, entities)) in ids.iter().enumerate() {
            for entity in entities.iter() {
                if let Ok((_, mut same_as_id)) = current_ids_query.get_mut(*entity) {
                    **same_as_id = i + 1;
                }
            }
        }

        **equivalence_counter = ids.len();
    }
}
