mod bundles;
mod components;
mod constants;
mod data_model;
mod events;
mod plugins;
mod resources;
mod systems;
mod utils;

use crate::components::*;
use crate::constants::WHITE_COLOR_MATERIAL_HANDLE;
use crate::data_model::load::load_world;
use crate::data_model::save::save_world;
use crate::events::*;
use crate::plugins::lyon_selection::LyonSelectionPlugin;
use crate::plugins::mouse_interaction::MouseInteractionPlugin;
use crate::resources::*;
use crate::systems::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::common_conditions::input_pressed;
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
        MouseInteractionPlugin,
    ))
    .insert_resource(DebugPickingMode::Disabled)
    .insert_resource(StrokeTessellator::new())
    .init_resource::<Zoom>()
    .init_resource::<FixedSystemElementGeometriesByNestingLevel>()
    .add_event::<ExternalEntityDrag>()
    .add_event::<InterfaceDrag>()
    .add_systems(Startup, setup);

    #[cfg(feature = "init_complete_system")]
    app.add_systems(Startup, init_complete_system.after(setup));

    let wheel_zoom_condition = input_pressed(KeyCode::ControlLeft)
        .or_else(input_pressed(KeyCode::ControlRight).or_else(
            input_pressed(KeyCode::SuperLeft).or_else(input_pressed(KeyCode::SuperRight)),
        ));

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
            change_focused_system,
            remove_unfocused_system_buttons,
            draw_flow_curve,
            drag_external_entity,
            drag_interface,
            update_initial_position_from_transform,
        ),
    )
    .add_systems(
        Update,
        (
            pan_camera_with_mouse.run_if(input_pressed(MouseButton::Right)),
            pan_camera_with_mouse_wheel.run_if(not(wheel_zoom_condition.clone())),
            control_zoom_from_keyboard,
            control_zoom_from_mouse_wheel.run_if(wheel_zoom_condition),
        ),
    )
    .add_systems(
        Update,
        (
            save_world.run_if(
                input_pressed(KeyCode::SuperLeft).and_then(input_just_pressed(KeyCode::KeyS)),
            ),
            load_world.run_if(
                input_pressed(KeyCode::SuperLeft).and_then(input_just_pressed(KeyCode::KeyL)),
            ),
        ),
    )
    .add_systems(
        Update,
        (
            apply_zoom,
            apply_zoom_to_system_radii,
            apply_zoom_to_camera_position,
            apply_zoom_to_incomplete_flows,
            apply_zoom_to_system_geometries,
            apply_zoom_to_strokes,
            apply_zoom_to_scale,
        )
            .run_if(resource_changed::<Zoom>),
    )
    .add_systems(
        Update,
        (
            spawn_selected_system,
            spawn_selected_flow,
            spawn_selected_interface,
            spawn_selected_external_entity,
            update_selected_flow_curve,
            despawn_selected_helper,
        ),
    )
    .add_systems(
        Update,
        (
            update_color_from_substance_type::<Inflow, InflowSourceConnection>,
            update_color_from_substance_type::<Outflow, OutflowSinkConnection>,
            update_button_substance_type_from_flow::<Inflow>,
            update_button_substance_type_from_flow::<Outflow>,
            update_interface_color_from_flow::<Inflow, InflowInterfaceConnection>,
            update_interface_color_from_flow::<Outflow, OutflowInterfaceConnection>,
            update_interface_subsystem_color_from_interface,
            update_system_color_from_subsystem,
        ),
    )
    .add_systems(
        PostUpdate,
        (
            update_flow_from_interface_subsystem.before(update_flow_from_system),
            update_flow_from_system.before(update_flow_from_interface),
            update_flow_from_interface,
            update_flow_from_external_entity,
        ),
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
    .register_type::<SelectedHighlightHelperAdded>();

    app.world.resource_mut::<Assets<ColorMaterial>>().insert(
        WHITE_COLOR_MATERIAL_HANDLE,
        ColorMaterial {
            color: Color::WHITE,
            ..default()
        },
    );

    app.run();
}
