use crate::bundles::SystemBundle;
use crate::components::*;
use crate::constants::*;
use crate::resources::FocusedSystem;
use bevy::math::vec2;
use bevy::prelude::*;

pub fn spawn_interface_subsystem(
    commands: &mut Commands,
    is_child_of_interface: bool,
    interface_entity: Entity,
    flow_interface_query: &Query<
        (
            Entity,
            Option<&FlowEndInterfaceConnection>,
            Option<&FlowStartInterfaceConnection>,
        ),
        With<Flow>,
    >,
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

    for (entity, inflow_connection, outflow_connection) in flow_interface_query {
        if let Some(connection) = inflow_connection {
            if connection.target == interface_entity {
                interface_flow_entity = entity;
                angle = std::f32::consts::PI;
                break;
            }
        }
        if let Some(connection) = outflow_connection {
            if connection.target == interface_entity {
                interface_flow_entity = entity;
                break;
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

    let subsystem_entity = commands
        .spawn((
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
        ))
        .id();

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
