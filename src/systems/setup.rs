use crate::bundles::{SystemBundle};
use crate::constants::*;
use crate::resources::*;
use bevy::math::{vec2};
use bevy::prelude::*;
use crate::components::NestingLevel;

const CLEAR_COLOR: Color = Color::ANTIQUE_WHITE;

#[cfg_attr(feature = "init_complete_system", allow(unused_variables))]
pub fn setup(
    mut commands: Commands,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
    tess: ResMut<StrokeTessellator>,
    geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(CLEAR_COLOR));

    let system_entity = commands
        .spawn(SystemBundle::new(
            Vec2::ZERO,
            0.0,
            MAIN_SYSTEM_RADIUS,
            0.0,
            &mut meshes,
            **zoom,
            0,
        ))
        .id();

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
            vec2(MAIN_SYSTEM_RADIUS, 0.0),
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
    nesting_query: Query<&NestingLevel>,
    focused_system: Res<FocusedSystem>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
) {
    use crate::bundles::*;
    use crate::components::*;

    spawn_complete_outflow(
        &mut commands,
        &focused_system,
        &subsystem_query,
        &nesting_query,
        &mut meshes,
        &mut stroke_tess,
        &mut fixed_system_element_geometries,
        **zoom,
        vec2(MAIN_SYSTEM_RADIUS, 0.0),
        Default::default(),
        OutflowUsability::Product,
    );

    spawn_complete_outflow(
        &mut commands,
        &focused_system,
        &subsystem_query,
        &nesting_query,
        &mut meshes,
        &mut stroke_tess,
        &mut fixed_system_element_geometries,
        **zoom,
        vec2(1.0, -1.0).normalize() * MAIN_SYSTEM_RADIUS,
        Default::default(),
        OutflowUsability::Waste,
    );

    spawn_complete_inflow(
        &mut commands,
        &focused_system,
        &subsystem_query,
        &nesting_query,
        &mut meshes,
        &mut stroke_tess,
        &mut fixed_system_element_geometries,
        **zoom,
        vec2(-MAIN_SYSTEM_RADIUS, 0.0),
        Default::default(),
        InflowUsability::Resource,
    );

    spawn_complete_inflow(
        &mut commands,
        &focused_system,
        &subsystem_query,
        &nesting_query,
        &mut meshes,
        &mut stroke_tess,
        &mut fixed_system_element_geometries,
        **zoom,
        vec2(-1.0, -1.0).normalize() * MAIN_SYSTEM_RADIUS,
        Default::default(),
        InflowUsability::Disruption,
    );
}
