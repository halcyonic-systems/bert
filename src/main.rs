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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct RemovalCleanupSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ZoomSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CameraControlSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CreateButtonSet;

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
            (
                egui_selected_context,
                change_focused_system,
                draw_flow_curve,
                drag_external_entity,
                drag_interface,
                update_initial_position_from_transform,
            ),
            (
                add_outflow_interface_create_button,
                add_inflow_interface_create_button,
                add_source_create_button,
                add_sink_create_button,
                add_inflow_create_button.run_if(inflow_create_button_needs_update),
                add_interface_subsystem_create_buttons,
                add_outflow_create_button,
                remove_unfocused_system_buttons,
            )
                .in_set(CreateButtonSet),
            (
                pan_camera_with_mouse.run_if(input_pressed(MouseButton::Right)),
                pan_camera_with_mouse_wheel.run_if(not(wheel_zoom_condition.clone())),
                control_zoom_from_keyboard,
                control_zoom_from_mouse_wheel.run_if(wheel_zoom_condition),
            )
                .in_set(CameraControlSet),
            (
                save_world.run_if(
                    input_pressed(KeyCode::SuperLeft).and_then(input_just_pressed(KeyCode::KeyS)),
                ),
                load_world.run_if(
                    input_pressed(KeyCode::SuperLeft).and_then(input_just_pressed(KeyCode::KeyL)),
                ),
                remove_selected_elements.run_if(
                    input_just_pressed(KeyCode::Delete)
                        .or_else(input_just_pressed(KeyCode::Backspace)),
                ),
            ),
            (cleanup_external_entity_removal,).in_set(RemovalCleanupSet),
            (
                apply_zoom,
                apply_zoom_to_system_radii,
                apply_zoom_to_camera_position,
                apply_zoom_to_incomplete_flows,
                apply_zoom_to_system_geometries,
                apply_zoom_to_strokes,
                apply_zoom_to_scale,
            )
                .in_set(ZoomSet),
            (
                spawn_selected_system,
                spawn_selected_flow,
                spawn_selected_interface,
                spawn_selected_external_entity,
                update_selected_flow_curve,
                despawn_selected_helper,
            ),
            (
                update_color_from_substance_type::<FlowStartConnection>,
                update_color_from_substance_type::<FlowEndConnection>,
                update_button_substance_type_from_flow,
                update_interface_color_from_flow::<FlowStartInterfaceConnection>,
                update_interface_color_from_flow::<FlowEndInterfaceConnection>,
                update_interface_subsystem_color_from_interface,
                update_system_color_from_subsystem,
                update_pins
            ),
        ),
    )
    .add_systems(
        PostUpdate,
        (
            update_flow_from_interface_subsystem.before(update_flow_from_system),
            update_flow_from_system.before(update_flow_from_interface),
            update_flow_from_interface,
            update_flow_from_external_entity,
            update_external_entity_from_flow,
            update_interface_subsystem_from_flows.run_if(interface_subsystem_should_update),
        ),
    )
    .configure_sets(
        Update,
        (
            RemovalCleanupSet.after(remove_selected_elements),
            CreateButtonSet.after(RemovalCleanupSet),
            ZoomSet.run_if(resource_changed::<Zoom>),
        ),
    )
    .register_type::<FlowStartInterfaceConnection>()
    .register_type::<FlowEndInterfaceConnection>()
    .register_type::<FlowStartConnection>()
    .register_type::<FlowEndConnection>()
    .register_type::<InterfaceSubsystemConnection>()
    .register_type::<SubsystemParentFlowConnection>()
    .register_type::<SystemElement>()
    .register_type::<crate::components::System>()
    .register_type::<ImportSubsystem>()
    .register_type::<ExportSubsystem>()
    .register_type::<InterfaceSubsystem>()
    .register_type::<Interface>()
    .register_type::<Flow>()
    .register_type::<ExternalEntity>()
    .register_type::<Subsystem>()
    .register_type::<ElementDescription>()
    .register_type::<CreateButton>()
    .register_type::<FlowInterfaceButton>()
    .register_type::<FlowOtherEndButton>()
    .register_type::<InterfaceSubsystemButton>()
    .register_type::<FlowCurve>()
    .register_type::<InitialPosition>()
    .register_type::<SelectedHighlightHelperAdded>()
    .register_type::<Pinnable>()
    .register_type::<Pin>();

    app.world.resource_mut::<Assets<ColorMaterial>>().insert(
        WHITE_COLOR_MATERIAL_HANDLE,
        ColorMaterial {
            color: Color::WHITE,
            ..default()
        },
    );

    app.run();
}
