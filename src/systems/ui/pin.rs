// use crate::bundles::*;
// use crate::components::*;
// use bevy::prelude::*;
// 
// pub fn update_unpinned_pinnables(
//     mut commands: Commands,
//     mut pinnable_query: Query<(Entity, &mut Pinnable), Added<Pinnable>>,
//     asset_server: Res<AssetServer>,
// ) {
//     for (pinnable_entity, mut pinnable) in &mut pinnable_query {
//         if !pinnable.has_pins {
//             spawn_pin(&pinnable_entity, &mut commands, &asset_server);
//             pinnable.has_pins = true;
//         }
//     }
// }
// 
// pub fn update_pin_rotation(mut pin_transform_query: Query<&mut Transform, With<Pin>>) {
//     for mut transform in &mut pin_transform_query {
//         transform.rotation = Quat::IDENTITY;
//     }
// }
