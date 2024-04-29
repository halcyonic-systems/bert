mod bundles;
mod components;
mod constants;
mod events;
mod plugins;
mod resources;
mod systems;
mod utils;

use crate::components::*;
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
    .init_resource::<Zoom>()
    .add_event::<ExternalEntityDrag>()
    .add_event::<InterfaceDrag>()
    .add_systems(Startup, setup);

    #[cfg(feature = "init_complete_system")]
    app.add_systems(Startup, init_complete_system.after(setup));

    app.add_systems(
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
    .add_systems(
        PostUpdate,
        (update_flow_from_interface, update_flow_from_external_entity),
    )
    .register_type::<OutflowInterfaceConnection>()
    .register_type::<InflowInterfaceConnection>()
    .register_type::<InflowSourceConnection>()
    .register_type::<OutflowSinkConnection>()
    .register_type::<InterfaceSubsystemConnection>()
    .register_type::<SubsystemParentFlowConnection>()
    .register_type::<SystemElement>()
    .register_type::<crate::components::System>()
    .register_type::<Interface>()
    .register_type::<Inflow>()
    .register_type::<Outflow>()
    .register_type::<ExternalEntity>()
    .register_type::<Subsystem>()
    .register_type::<ElementDescription>()
    .register_type::<CreateButton>()
    .register_type::<FlowInterfaceButton>()
    .register_type::<FlowOtherEndButton>()
    .register_type::<InterfaceSubsystemButton>()
    .register_type::<FlowCurve>()
    .register_type::<InitialPosition>()
    .register_type::<ScaleWithZoom>()
    .register_type::<ZoomIndependentStrokeWidth>();

    app.world.resource_mut::<Assets<ColorMaterial>>().insert(
        WHITE_COLOR_MATERIAL_HANDLE,
        ColorMaterial {
            color: Color::WHITE,
            ..default()
        },
    );

    app.run();
}
