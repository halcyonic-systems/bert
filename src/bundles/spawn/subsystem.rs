use crate::bundles::SystemBundle;
use crate::components::*;
use crate::constants::*;
use crate::resources::FocusedSystem;
use bevy::math::vec2;
use bevy::prelude::*;

pub fn spawn_interface_subsystem(
    commands: &mut Commands,
    interface_entity: Entity,
    flow_interface_query: &Query<
        (
            Entity,
            Option<&InflowInterfaceConnection>,
            Option<&OutflowInterfaceConnection>,
        ),
        Or<(With<Inflow>, With<Outflow>)>,
    >,
    system_query: &Query<&crate::components::System>,
    focused_system: &Res<FocusedSystem>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let mut interface_flow_entity = Entity::PLACEHOLDER;

    for (entity, inflow_connection, outflow_connection) in flow_interface_query {
        if let Some(connection) = inflow_connection {
            if connection.target == interface_entity {
                interface_flow_entity = entity;
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

    let radius = system_query
        .get(***focused_system)
        .expect("focused system not found")
        .radius
        * SUBSYSTEM_RADIUS_FRACTION;

    let mut subsystem_entity = Entity::PLACEHOLDER;

    commands
        .entity(interface_entity)
        .with_children(|parent| {
            subsystem_entity = parent
                .spawn((
                    SubsystemParentFlowConnection {
                        target: interface_flow_entity,
                    },
                    Subsystem::default(),
                    SystemBundle::new(vec2(-radius, 0.0), 1.0, radius, meshes),
                ))
                .id();
        })
        .insert(InterfaceSubsystemConnection {
            target: subsystem_entity,
        });
}
