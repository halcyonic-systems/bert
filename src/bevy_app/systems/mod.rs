//! All app systems

mod camera;
mod egui;
mod removal;
mod setup;
mod subsystem;
mod ui;

use bevy::ecs::system::RunSystemOnce;
pub use camera::*;
pub use egui::*;
pub use removal::*;
pub use setup::*;
pub use subsystem::*;
pub use ui::*;

use crate::bevy_app::data_model::save::serialize_world;
use crate::bevy_app::data_model::WorldModel;
use crate::events::{TreeEvent, TriggerEvent};
use bevy::prelude::*;

pub fn react_to_trigger_event(world: &mut World, mut reader: EventReader<TriggerEvent>) {
    for event in reader.read() {
        match event {
            TriggerEvent::ShowTree => {
                world.run_system_once(serialize_world.pipe(send_world_to_leptos));
            }
        }
    }
}

pub fn send_world_to_leptos(In(world_model): In<WorldModel>, mut writer: EventWriter<TreeEvent>) {
    writer.send(TreeEvent { world_model });
}
