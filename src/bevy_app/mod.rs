mod bundles;
pub(crate) mod components;
mod constants;
pub(crate) mod data_model;
mod events;
pub(crate) mod plugins;
mod resources;
mod states;
mod systems;
mod utils;

use crate::bevy_app::data_model::load::load_file;
use crate::{
    ExternalEntityFilter, ExternalEntityQuery, InteractionQuery, InterfaceQuery, IsSameAsIdQuery,
    SelectionFilter, SubSystemQuery, SystemQuery,
};
use bevy::asset::AssetMetaCheck;
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;
use bevy::transform::TransformSystem::TransformPropagate;
use bevy_file_dialog::FileDialogPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bundles::*;
use bundles::{auto_spawn_interface_label, auto_spawn_subsystem_label};
pub use components::*;
use constants::WHITE_COLOR_MATERIAL_HANDLE;
use data_model::load::load_world;
use data_model::save::save_world;
pub use events::*;
use leptos_bevy_canvas::prelude::{BevyEventReceiver, BevyQueryDuplex, LeptosBevyApp};
use plugins::label::{
    copy_position, copy_positions, copy_positions_changed, LabelPlugin, MarkerLabel, NameLabel,
};
use plugins::lyon_selection::LyonSelectionPlugin;
use plugins::mouse_interaction::{disable_selection, enable_selection, MouseInteractionPlugin};
use resources::*;
use states::*;
use systems::*;

/// Prevents entity cleanup systems from running until after the user deletes a selected element.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct RemovalCleanupSet;

/// Prevents diagram creation systems from running while file selection is active.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct AllSet;

/// Prevents zoom systems from running until the zoom resource is changed.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ZoomSet;

/// Systems that manipulate the camera transform.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CameraControlSet;

/// Systems that place & update CreateButtons.
/// Prevents buttons from updating until diagram elements child transforms are updated.
/// Prevents buttons from showing while user is in the FlowTerminalSelection State.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CreateButtonSet;

/// Contains the systems that update the geometry of one type of entity from another.
/// For example, interfaces from subsystem radii.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GeometryUpdateSet;

/// Systems that control selection & placement of nested external entities on the diagram
/// Prevents the systems from running while the App is in a Normal State.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct FlowTerminalSelectingSet;

/// Systems that spawns all label types.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct AutoSpawnLabelSet;

/// Used to save and load the world
pub struct JsonWorldData;

pub fn init_bevy_app(
    selected_details_query: BevyQueryDuplex<(SystemElement,), With<SelectedHighlightHelperAdded>>,
    interface_details_query: BevyQueryDuplex<InterfaceQuery, SelectionFilter>,
    interaction_details_query: BevyQueryDuplex<InteractionQuery, SelectionFilter>,
    external_entity_details_query: BevyQueryDuplex<ExternalEntityQuery, SelectionFilter>,
    system_details_query: BevyQueryDuplex<SystemQuery, SelectionFilter>,
    sub_system_details_query: BevyQueryDuplex<
        SubSystemQuery,
        (SelectionFilter, crate::SubSystemFilter),
    >,
    is_same_as_id_query: BevyQueryDuplex<IsSameAsIdQuery, (ExternalEntityFilter, SelectionFilter)>,
    detach_event_receiver: BevyEventReceiver<DetachMarkerLabelEvent>,
) -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    focused: false,
                    fit_canvas_to_parent: true,
                    canvas: Some("#bevy_canvas".into()),
                    // resolution: WindowResolution::new(500.0, 300.0),
                    ..default()
                }),
                ..default()
            }),
        MeshPickingPlugin,
        ShapePlugin,
        LyonSelectionPlugin,
        MouseInteractionPlugin,
        LabelPlugin,
        FileDialogPlugin::new()
            .with_save_file::<JsonWorldData>()
            .with_load_file::<JsonWorldData>(),
    ))
    // .add_plugins(WorldInspectorPlugin::new())
    .insert_resource(StrokeTessellator::new())
    .init_resource::<Zoom>()
    .init_resource::<FixedSystemElementGeometriesByNestingLevel>()
    .init_resource::<IsSameAsIdCounter>()
    .add_event::<ExternalEntityDrag>()
    .add_event::<InterfaceDrag>()
    .add_event::<SubsystemDrag>()
    .add_event::<RemoveEvent>()
    .add_event::<DetachMarkerLabelEvent>()
    .init_state::<AppState>()
    .sync_leptos_signal_with_query(selected_details_query)
    .sync_leptos_signal_with_query(interface_details_query)
    .sync_leptos_signal_with_query(interaction_details_query)
    .sync_leptos_signal_with_query(external_entity_details_query)
    .sync_leptos_signal_with_query(system_details_query)
    .sync_leptos_signal_with_query(sub_system_details_query)
    .sync_leptos_signal_with_query(is_same_as_id_query)
    .import_event_from_leptos(detach_event_receiver)
    .add_systems(Startup, (window_setup, setup));
    #[cfg(feature = "init_complete_system")]
    app.add_systems(Startup, init_complete_system.after(setup));

    let wheel_zoom_condition = input_pressed(KeyCode::ControlLeft)
        .or(input_pressed(KeyCode::ControlRight)
            .or(input_pressed(KeyCode::SuperLeft).or(input_pressed(KeyCode::SuperRight))));

    app.add_systems(
        PreUpdate,
        (
            control_zoom_from_keyboard,
            // TODO : for some reason this doesn't always work: control_zoom_from_mouse_wheel.run_if(wheel_zoom_condition.clone()),
        )
            .in_set(AllSet),
    );

    app.add_systems(
        Update,
        (
            (
                change_focused_system,
                draw_flow_curve,
                update_initial_position_from_transform,
                listen_to_remove_marker_label_event,
            ),
            (
                update_selecting_flow_from_mouse,
                select_flow_terminal.after(update_selecting_flow_from_mouse),
            )
                .in_set(FlowTerminalSelectingSet),
            (drag_external_entity, drag_interface, drag_subsystem),
            (
                pan_camera_with_mouse.run_if(input_pressed(MouseButton::Right)),
                pan_camera_with_mouse_wheel.run_if(not(wheel_zoom_condition)),
                reset_camera_position.run_if(
                    input_pressed(KeyCode::SuperLeft).and(input_just_pressed(KeyCode::KeyR)),
                ),
            )
                .in_set(CameraControlSet),
            (
                load_file.run_if(
                    input_pressed(KeyCode::SuperLeft).and(input_just_pressed(KeyCode::KeyL)),
                ),
                load_world,
                save_world.run_if(
                    input_pressed(KeyCode::SuperLeft).and(input_just_pressed(KeyCode::KeyS)),
                ),
            ),
            (
                cleanup_external_entity_removal,
                cleanup_labelled_removal::<NameLabel>,
                cleanup_labelled_removal::<MarkerLabel>,
                update_and_cleanup_source_sink_equivalence,
                cleanup_interface_removal,
                cleanup_subsystem_removal,
                cleanup_flow_removal,
                cleanup_focused_system,
            )
                .in_set(RemovalCleanupSet),
            (
                apply_zoom,
                apply_zoom_to_camera_position,
                apply_zoom_to_incomplete_flows,
                apply_zoom_to_system_geometries,
                apply_zoom_to_strokes,
                apply_zoom_to_scale,
                apply_zoom_to_label,
            )
                .in_set(ZoomSet),
            (
                spawn_selected_system,
                spawn_selected_flow,
                spawn_selected_interface,
                spawn_selected_external_entity,
                update_selected_flow_curve,
                despawn_selected_helper,
                remove_selected_elements.run_if(in_state(AppState::Normal).and(
                    input_just_pressed(KeyCode::Backspace).or(input_just_pressed(KeyCode::Delete)),
                )),
            ),
            (
                update_sub_system_parent_system,
                update_color_from_substance_type::<FlowStartConnection>,
                update_color_from_substance_type::<FlowEndConnection>,
                update_button_substance_type_from_flow,
                update_interface_color_from_flow::<FlowStartInterfaceConnection>,
                update_interface_color_from_flow::<FlowEndInterfaceConnection>,
                update_interface_subsystem_color,
                update_system_color_from_subsystem,
                apply_zoom_to_system_radii, // this is not in ZoomSet on purpose
                update_is_same_as_id_label,
            ),
            (
                hide_selected
                    .run_if(in_state(AppState::Normal).and(input_just_pressed(KeyCode::KeyH))),
                un_hide_selected
                    .run_if(in_state(AppState::Normal).and(input_just_pressed(KeyCode::KeyU))),
            ),
            (apply_sink_and_source_equivalence
                .run_if(in_state(AppState::Normal).and(input_just_pressed(KeyCode::KeyE))),),
        )
            .in_set(AllSet),
    )
    .add_systems(
        PostUpdate,
        (
            (
                auto_spawn_external_entity_label,
                auto_spawn_subsystem_label,
                auto_spawn_system_label,
                auto_spawn_interface_label,
                auto_spawn_flow_label,
                auto_spawn_source_sink_equivalence,
            )
                .in_set(AutoSpawnLabelSet),
            (
                add_outflow_interface_create_button,
                add_inflow_interface_create_button,
                add_source_create_button,
                add_sink_create_button,
                add_inflow_create_button.run_if(inflow_create_button_needs_update),
                add_outflow_create_button.run_if(outflow_create_button_needs_update),
                add_interface_subsystem_create_buttons
                    .run_if(interface_subsystem_create_button_needs_update),
                add_subsystem_from_external_entities_create_button,
                remove_unfocused_system_buttons,
            )
                .in_set(CreateButtonSet),
            (
                update_flow_from_interface,
                update_flow_from_external_entity,
                update_label_offset_from_interface
                    .before(copy_position)
                    .before(copy_positions),
                update_label_from_interaction,
                update_subsystem_radius_from_interface_count,
                update_interface_positions_from_system_radius
                    .after(update_subsystem_radius_from_interface_count),
                update_interface_subsystem_from_flows.run_if(interface_subsystem_should_update),
                update_flow_from_subsystem_without_interface
                    .after(update_subsystem_radius_from_interface_count),
                update_interface_button_from_interaction,
                update_text_color,
            )
                .in_set(GeometryUpdateSet),
            apply_zoom_to_added_label.after(AutoSpawnLabelSet),
        )
            .in_set(AllSet),
    )
    .add_systems(
        OnEnter(AppState::FlowTerminalSelection),
        (disable_selection,),
    )
    .add_systems(
        OnExit(AppState::FlowTerminalSelection),
        (add_inflow_create_button, add_outflow_create_button),
    )
    .add_systems(OnEnter(AppState::Normal), (enable_selection,))
    // .configure_sets(PreUpdate, (AllSet.run_if(in_state(FileState::Inactive)),))
    .configure_sets(
        Update,
        (
            RemovalCleanupSet.after(remove_selected_elements),
            ZoomSet.run_if(resource_changed::<Zoom>),
            FlowTerminalSelectingSet.run_if(in_state(AppState::FlowTerminalSelection)),
            // AllSet.run_if(in_state(FileState::Inactive)),
        ),
    )
    .configure_sets(
        PostUpdate,
        (
            CreateButtonSet
                .after(TransformPropagate)
                .after(GeometryUpdateSet)
                .run_if(in_state(AppState::Normal)),
            AutoSpawnLabelSet.after(copy_positions_changed),
            // AllSet.run_if(in_state(FileState::Inactive)),
        ),
    )
    .register_type::<FlowStartInterfaceConnection>()
    .register_type::<FlowEndInterfaceConnection>()
    .register_type::<FlowStartConnection>()
    .register_type::<FlowEndConnection>()
    .register_type::<InterfaceSubsystemConnection>()
    .register_type::<SystemElement>()
    .register_type::<crate::bevy_app::components::System>()
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
    .register_type::<NestingLevel>()
    .register_type::<FocusedSystem>()
    .register_type::<Zoom>();

    app.world_mut()
        .resource_mut::<Assets<ColorMaterial>>()
        .insert(
            &WHITE_COLOR_MATERIAL_HANDLE,
            ColorMaterial {
                color: Color::WHITE,
                ..default()
            },
        );
    // let settings = bevy_mod_debugdump::schedule_graph::Settings::default();
    // let dot = bevy_mod_debugdump::schedule_graph_dot(&mut app, PreUpdate, &settings);
    // std::fs::write(format!("schedule_preupdate.dot"), dot);

    app.run();
    app
}
