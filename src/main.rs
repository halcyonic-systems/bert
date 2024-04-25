mod bundles;
mod components;
mod constants;
mod events;
mod plugins;
mod resources;
mod systems;
mod utils;

use crate::constants::WHITE_COLOR_MATERIAL_HANDLE;
use crate::events::*;
use crate::plugins::lyon_selection::LyonSelectionPlugin;
use crate::resources::*;
use crate::systems::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        WorldInspectorPlugin::new(),
        DefaultPickingPlugins,
        // EguiPlugin,
        ShapePlugin,
        LyonSelectionPlugin,
    ))
    .insert_resource(DebugPickingMode::Disabled)
    .insert_resource(StrokeTessellator::new())
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
            apply_zoom_to_stroke,
            draw_flow_curve,
            apply_zoom_to_flow_curve,
            drag_external_entity,
            drag_interface,
        ),
    )
    .init_resource::<Zoom>()
    .add_event::<ExternalEntityDrag>()
    .add_event::<InterfaceDrag>();

    app.world.resource_mut::<Assets<ColorMaterial>>().insert(
        WHITE_COLOR_MATERIAL_HANDLE,
        ColorMaterial {
            color: Color::WHITE,
            ..default()
        },
    );

    app.run();
}
