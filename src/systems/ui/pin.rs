use bevy::prelude::*;
use crate::components::*;
use crate::bundles::*;

pub fn update_pins(
    mut commands: Commands,
    mut pinnable_query: Query<(Entity, &mut Pinnable)>,
    asset_server: Res<AssetServer>
) {
    for (pinnable_entity, mut pinnable) in &mut pinnable_query {
        if !pinnable.has_pins {
            spawn_pin(
                &pinnable_entity,
                &mut commands, 
                &asset_server
            );
            pinnable.has_pins = true;
        }
    }
}