use crate::bundles::spawn_main_system;
use crate::constants::*;
use crate::resources::*;
use bevy::prelude::*;
use bevy::render::deterministic::DeterministicRenderingConfig;
use bevy::window::PrimaryWindow;

pub fn window_setup(mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut w = primary_window_query
        .get_single_mut()
        .expect("Should only be one primary window.");

    w.title = "Deep System Analysis".to_string();
    w.position = WindowPosition::Centered(MonitorSelection::Current);
    w.set_maximized(true);    
}

const CLEAR_COLOR: Color = Color::ANTIQUE_WHITE;

#[allow(unused_variables)]
pub fn setup(
    mut commands: Commands,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
    tess: ResMut<StrokeTessellator>,
    geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    mut deterministic_rendering_config: ResMut<DeterministicRenderingConfig>,
    asset_server: Res<AssetServer>,
) {
    deterministic_rendering_config.stable_sort_z_fighting = true;

    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(CLEAR_COLOR));

    let system_entity = spawn_main_system(
        &mut commands,
        Vec2::ZERO,
        0.0,
        false,
        false,
        Default::default(),
        **zoom,
        "System",
        "",
        &mut meshes,
    );

    commands.insert_resource(FocusedSystem::new(system_entity));

    #[cfg(not(feature = "init_complete_system"))]
    {
        use crate::bundles::spawn_create_button;
        use crate::components::{CreateButton, CreateButtonType};

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Outflow,
                connection_source: system_entity,
                system: system_entity,
                substance_type: None,
            },
            bevy::math::vec2(MAIN_SYSTEM_RADIUS, 0.0),
            0.0,
            **zoom,
            Some(system_entity),
            &asset_server,
        );
    }
}

#[cfg(feature = "init_complete_system")]
pub fn init_complete_system(
    mut commands: Commands,
    subsystem_query: Query<&crate::components::Subsystem>,
    nesting_query: Query<&crate::components::NestingLevel>,
    focused_system: Res<FocusedSystem>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
) {
    use crate::bundles::*;
    use crate::components::*;
    use rust_decimal_macros::dec;

    spawn_complete_outflow(
        &mut commands,
        *focused_system,
        &subsystem_query,
        &nesting_query,
        &mut meshes,
        &mut stroke_tess,
        &mut fixed_system_element_geometries,
        **zoom,
        0.0,
        MAIN_SYSTEM_RADIUS,
        Default::default(),
        OutflowUsability::Product,
        dec!(1),
        "",
        "Second",
        "Interface",
        "",
        "Outflow",
        "",
        "Sink",
        "",
        None,
    );

    spawn_complete_outflow(
        &mut commands,
        *focused_system,
        &subsystem_query,
        &nesting_query,
        &mut meshes,
        &mut stroke_tess,
        &mut fixed_system_element_geometries,
        **zoom,
        -std::f32::consts::FRAC_PI_4,
        MAIN_SYSTEM_RADIUS,
        Default::default(),
        OutflowUsability::Waste,
        dec!(1),
        "",
        "Second",
        "Interface",
        "",
        "Outflow",
        "",
        "Sink",
        "",
        None,
    );

    spawn_complete_inflow(
        &mut commands,
        *focused_system,
        &subsystem_query,
        &nesting_query,
        &mut meshes,
        &mut stroke_tess,
        &mut fixed_system_element_geometries,
        **zoom,
        std::f32::consts::PI,
        MAIN_SYSTEM_RADIUS,
        Default::default(),
        InflowUsability::Resource,
        dec!(1),
        "",
        "Second",
        "Interface",
        "",
        "Inflow",
        "",
        "Source",
        "",
        None,
    );

    spawn_complete_inflow(
        &mut commands,
        *focused_system,
        &subsystem_query,
        &nesting_query,
        &mut meshes,
        &mut stroke_tess,
        &mut fixed_system_element_geometries,
        **zoom,
        std::f32::consts::FRAC_PI_4 * 3.0,
        MAIN_SYSTEM_RADIUS,
        Default::default(),
        InflowUsability::Disruption,
        dec!(1),
        "",
        "Second",
        "Interface",
        "",
        "Inflow",
        "",
        "Source",
        "",
        None,
    );
}
