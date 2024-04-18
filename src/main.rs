mod bundles;
mod components;
mod resources;
mod systems;

use crate::resources::Zoom;
use crate::systems::*;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultPickingPlugins, EguiPlugin, ShapePlugin))
        // .insert_resource(DebugPickingMode::Normal)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                egui_selected_context,
                add_outflow_interface_create_button,
                add_inflow_interface_create_button,
                add_source_create_button,
                add_sink_create_button,
                add_consecutive_outflow_create_button,
                add_first_inflow_create_button,
                add_consecutive_inflow_create_button,
                add_interface_subsystem_create_buttons,
                add_first_outflow_create_button,
                zoom_control_system,
                change_focused_system,
                remove_unfocused_system_buttons,
                apply_zoom,
            ),
        )
        .init_resource::<Zoom>()
        .run();
}
