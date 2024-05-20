use crate::bundles::SystemBundle;
use crate::components::*;
use crate::constants::*;
use crate::plugins::label::add_name_label;
use crate::resources::FocusedSystem;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;

pub fn spawn_interface_subsystem(
    commands: &mut Commands,
    is_child_of_interface: bool,
    interface_entity: Entity,
    flow_interface_query: &Query<(
        Entity,
        &Flow,
        Option<&FlowEndInterfaceConnection>,
        Option<&FlowStartInterfaceConnection>,
    )>,
    system_query: &Query<(&Transform, &crate::components::System)>,
    nesting_level_query: &Query<&NestingLevel>,
    focused_system: &Res<FocusedSystem>,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
    name: &str,
    description: &str,
) -> Entity {
    let mut interface_flow_entity = Entity::PLACEHOLDER;
    let mut angle = 0.0;
    let mut is_import_subsystem = false;
    let mut is_export_subsystem = false;
    let mut interface_subsystem = InterfaceSubsystem::new(interface_entity);

    for (entity, flow, inflow_connection, outflow_connection) in flow_interface_query {
        if let Some(connection) = inflow_connection {
            if connection.target == interface_entity {
                interface_flow_entity = entity;
                angle = std::f32::consts::PI;
                is_import_subsystem = true;
                interface_subsystem.total_inflow += flow.amount;
                interface_subsystem.substance_type = flow.substance_type;
                interface_subsystem.is_useful = flow.is_useful;
            }
        }
        if let Some(connection) = outflow_connection {
            if connection.target == interface_entity {
                interface_flow_entity = entity;
                is_export_subsystem = true;
                interface_subsystem.total_outflow += flow.amount;
                interface_subsystem.substance_type = flow.substance_type;
                interface_subsystem.is_useful = flow.is_useful;
            }
        }
    }

    let parent_system = ***focused_system;

    let radius = system_query
        .get(parent_system)
        .expect("focused system not found")
        .1
        .radius
        * SUBSYSTEM_RADIUS_FRACTION;

    let z = if is_child_of_interface {
        SUBSYSTEM_Z - INTERFACE_Z
    } else {
        SUBSYSTEM_Z
    };

    let nesting_level = NestingLevel::current(parent_system, nesting_level_query) + 1;

    let mut subsystem_commands = commands.spawn((
        SubsystemParentFlowConnection {
            target: interface_flow_entity,
        },
        Subsystem { parent_system },
        NestingLevel::new(nesting_level),
        SystemBundle::new(
            vec2(-radius * zoom, 0.0),
            z,
            radius,
            angle,
            false,
            false,
            Default::default(),
            meshes,
            zoom,
            nesting_level,
            name,
            description,
        ),
        interface_subsystem,
        Pinnable { has_pins: false },
    ));

    if is_import_subsystem {
        subsystem_commands.insert(ImportSubsystem);
    }
    if is_export_subsystem {
        subsystem_commands.insert(ExportSubsystem);
    }

    let subsystem_entity = subsystem_commands.id();

    let mut interface_commands = commands.entity(interface_entity);
    interface_commands.insert(InterfaceSubsystemConnection {
        target: subsystem_entity,
    });

    if is_child_of_interface {
        interface_commands.add_child(subsystem_entity);
    } else {
        commands.entity(parent_system).add_child(subsystem_entity);
    }

    subsystem_entity
}

pub fn auto_spawn_interface_subsystem_label(
    mut commands: Commands,
    interface_subsystem_query: Query<Entity, Added<InterfaceSubsystem>>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
) {
    for interface_subsystem in interface_subsystem_query.iter() {
        add_name_label(
            &mut commands,
            interface_subsystem,
            vec2(100.0, 100.0),
            vec3(0.0, 0.0, 0.0),
            &name_query,
            &asset_server,
        );
    }
}
