use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct NameLabel {
    pub label: Entity,
}

pub fn copy_name_to_label(
    source_query: Query<(&Name, &NameLabel), Changed<Name>>,
    mut target_query: Query<&mut Text>,
) {
    for (name, label) in &source_query {
        target_query
            .get_mut(label.label)
            .expect("Label should exist")
            .sections[0]
            .value = name.to_string();
    }
}
