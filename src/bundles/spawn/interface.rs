use crate::components::*;
use crate::constants::*;
use crate::utils::ui_transform_from_button;
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_interface(
    commands: &mut Commands,
    interface_type: InterfaceType,
    flow_entity: Entity,
    transform: &GlobalTransform,
    initial_position: &InitialPosition,
    zoom: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let points = [
        Vec2::new(INTERFACE_WIDTH_HALF, INTERFACE_HEIGHT_HALF), // top right
        Vec2::new(-INTERFACE_WIDTH_HALF, INTERFACE_HEIGHT_HALF), // top left
        Vec2::new(-INTERFACE_WIDTH_HALF, -INTERFACE_HEIGHT_HALF), // bottom left
        Vec2::new(INTERFACE_WIDTH_HALF, -INTERFACE_HEIGHT_HALF), // bottom right
    ];

    let shape = shapes::RoundedPolygon {
        points: points.into_iter().collect(),
        radius: 5.,
        closed: false,
    };

    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 5.0, 0.0, zoom);

    let interface_entity = commands
        .spawn((
            Interface::default(),
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle {
                    transform,
                    ..default()
                },
                ..default()
            },
            Stroke::new(Color::BLACK, 3.0),
            Fill::color(Color::WHITE),
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::Interface,
            Name::new("Interface"),
            initial_position,
            // TODO : this is always going to be the same => make it a resource to re-use
            SimplifiedMesh {
                mesh: meshes
                    .add(Rectangle::new(
                        INTERFACE_WIDTH_HALF * 2.0,
                        INTERFACE_HEIGHT_HALF * 2.0,
                    ))
                    .into(),
            },
            Aabb {
                center: Vec3A::ZERO,
                half_extents: Vec3A::new(INTERFACE_WIDTH_HALF, INTERFACE_HEIGHT_HALF, 0.0),
            },
        ))
        .id();

    let mut entity_commands = commands.entity(flow_entity);

    match interface_type {
        InterfaceType::Import => {
            entity_commands.insert(InflowInterfaceConnection {
                target: interface_entity,
            });
        }
        InterfaceType::Export => {
            entity_commands.insert(OutflowInterfaceConnection {
                target: interface_entity,
            });
        }
    }
}
