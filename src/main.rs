mod bundles;
mod components;
mod constants;
mod data_model;
mod events;
mod plugins;
mod resources;
mod states;
mod systems;
mod utils;

use crate::bundles::auto_spawn_external_entity_label;
use crate::components::*;
use crate::constants::WHITE_COLOR_MATERIAL_HANDLE;
use crate::data_model::load::load_world;
use crate::data_model::save::save_world;
use crate::events::*;
use crate::plugins::label::LabelPlugin;
use crate::plugins::lyon_selection::LyonSelectionPlugin;
use crate::plugins::mouse_interaction::{
    disable_selection, enable_selection, MouseInteractionPlugin,
};
use crate::resources::*;
use crate::states::*;
use crate::systems::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bundles::{
    auto_spawn_interface_label, auto_spawn_interface_subsystem_label, auto_spawn_subsystem_label,
};
use data_model::{export_file_dialog::*, import_file_dialog::*};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct RemovalCleanupSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ZoomSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CameraControlSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CreateButtonSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct FlowTerminalSelectingSet;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        WorldInspectorPlugin::new(),
        DefaultPickingPlugins,
        ShapePlugin,
        LyonSelectionPlugin,
        MouseInteractionPlugin,
        LabelPlugin,
    ))
    .init_state::<FileImportState>()
    .init_state::<FileExportState>()
    .insert_resource(DebugPickingMode::Disabled)
    .insert_resource(StrokeTessellator::new())
    .init_resource::<Zoom>()
    .init_resource::<FixedSystemElementGeometriesByNestingLevel>()
    .add_event::<ExternalEntityDrag>()
    .add_event::<InterfaceDrag>()
    .add_event::<SubsystemDrag>()
    .init_state::<AppState>()
    .add_systems(Startup, (window_setup, setup).chain());

    #[cfg(feature = "init_complete_system")]
    app.add_systems(Startup, init_complete_system.after(setup));

    let wheel_zoom_condition = input_pressed(KeyCode::ControlLeft)
        .or_else(input_pressed(KeyCode::ControlRight).or_else(
            input_pressed(KeyCode::SuperLeft).or_else(input_pressed(KeyCode::SuperRight)),
        ));

    app.add_systems(
        PreUpdate,
        (absorb_egui_inputs,)
            .after(bevy_egui::systems::process_input_system)
            .before(bevy_egui::EguiSet::BeginFrame),
    );

    app.add_systems(
        Update,
        (
            (
                egui_selected_context.after(bevy_egui::EguiSet::InitContexts),
                change_focused_system,
                draw_flow_curve,
                update_initial_position_from_transform,
            ),
            (
                update_selecting_flow_from_mouse,
                select_flow_terminal.after(update_selecting_flow_from_mouse),
            )
                .in_set(FlowTerminalSelectingSet),
            (drag_external_entity, drag_interface, drag_subsystem),
            (
                add_outflow_interface_create_button,
                add_inflow_interface_create_button,
                add_source_create_button,
                add_sink_create_button,
                add_inflow_create_button.run_if(inflow_create_button_needs_update),
                add_interface_subsystem_create_buttons,
                add_subsystem_from_external_entities_create_button,
                add_outflow_create_button,
                remove_unfocused_system_buttons,
                // update_unpinned_pinnables,
            )
                .in_set(CreateButtonSet),
            (
                pan_camera_with_mouse.run_if(input_pressed(MouseButton::Right)),
                pan_camera_with_mouse_wheel.run_if(not(wheel_zoom_condition.clone())),
                control_zoom_from_keyboard,
                control_zoom_from_mouse_wheel.run_if(wheel_zoom_condition),
                reset_camera_position.run_if(
                    input_pressed(KeyCode::SuperLeft).and_then(input_just_pressed(KeyCode::KeyR)),
                ),
            )
                .in_set(CameraControlSet),
            (
                import_file.run_if(
                    in_state(FileImportState::Inactive)
                        .and_then(input_pressed(KeyCode::SuperLeft))
                        .and_then(input_just_pressed(KeyCode::KeyL)),
                ),
                open_import_dialog_selection.run_if(in_state(FileImportState::Select)),
                poll_for_selected_file.run_if(in_state(FileImportState::Poll)),
                load_world.run_if(in_state(FileImportState::Load)),
                import_clean_up.run_if(in_state(FileImportState::CleanUp)),
                export_file.run_if(
                    in_state(FileExportState::Inactive)
                        .and_then(input_pressed(KeyCode::SuperLeft))
                        .and_then(input_just_pressed(KeyCode::KeyS)),
                ),
                open_export_dialog.run_if(in_state(FileExportState::Select)),
                poll_for_export_file.run_if(in_state(FileExportState::Poll)),
                save_world.run_if(in_state(FileExportState::Save)),
                export_clean_up.run_if(in_state(FileExportState::CleanUp)),
            ),
            (
                cleanup_external_entity_removal,
                cleanup_labelled_removal,
                cleanup_interface_removal,
                cleanup_subsystem_removal,
                cleanup_flow_removal,
            )
                .in_set(RemovalCleanupSet),
            (
                apply_zoom,
                apply_zoom_to_system_radii,
                apply_zoom_to_camera_position,
                apply_zoom_to_incomplete_flows,
                apply_zoom_to_flow_without_interface,
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
                remove_selected_elements.run_if(
                    in_state(AppState::Normal).and_then(
                        input_just_pressed(KeyCode::Backspace)
                            .or_else(input_just_pressed(KeyCode::Delete)),
                    ),
                ),
            ),
            (
                update_color_from_substance_type::<FlowStartConnection>,
                update_color_from_substance_type::<FlowEndConnection>,
                update_button_substance_type_from_flow,
                update_interface_color_from_flow::<FlowStartInterfaceConnection>,
                update_interface_color_from_flow::<FlowEndInterfaceConnection>,
                update_interface_subsystem_color_from_interface,
                update_system_color_from_subsystem,
            ),
        ),
    )
    .add_systems(
        PostUpdate,
        (
            update_flow_from_interface,
            update_flow_from_external_entity,
            update_interface_subsystem_from_flows.run_if(interface_subsystem_should_update),
            update_flow_from_subsystem_without_interface,
            auto_spawn_external_entity_label,
            auto_spawn_interface_subsystem_label,
            auto_spawn_subsystem_label,
            auto_spawn_interface_label, //update_pin_rotation,
        ),
    )
    .add_systems(
        OnEnter(AppState::FlowTerminalSelection),
        (disable_selection,),
    )
    .add_systems(OnEnter(AppState::Normal), (enable_selection,))
    .configure_sets(
        Update,
        (
            RemovalCleanupSet.after(remove_selected_elements),
            CreateButtonSet
                .after(RemovalCleanupSet)
                .run_if(in_state(AppState::Normal)),
            ZoomSet.run_if(resource_changed::<Zoom>),
            FlowTerminalSelectingSet.run_if(in_state(AppState::FlowTerminalSelection)),
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
    .register_type::<HasFlowInterfaceButton>()
    .register_type::<HasFlowOtherEndButton>()
    .register_type::<HasInterfaceSubsystemButton>()
    .register_type::<FlowCurve>()
    .register_type::<InitialPosition>()
    .register_type::<SelectedHighlightHelperAdded>()
    .register_type::<Pinnable>()
    .register_type::<Pin>()
    .register_type::<NestingLevel>()
    .register_type::<FocusedSystem>()
    .register_type::<Zoom>();

    app.world.resource_mut::<Assets<ColorMaterial>>().insert(
        WHITE_COLOR_MATERIAL_HANDLE,
        ColorMaterial {
            color: Color::WHITE,
            ..default()
        },
    );

    app.run();
}
