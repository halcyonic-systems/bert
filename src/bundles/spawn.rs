use crate::bundles::SystemBundle;
use crate::components::*;
use crate::constants::{
    BUTTON_WIDTH_HALF, FLOW_END_LENGTH, FLOW_LENGTH, INTERFACE_HEIGHT_HALF, INTERFACE_WIDTH_HALF,
    SUBSYSTEM_RADIUS_FRACTION,
};
use crate::resources::FocusedSystem;
use crate::systems::{create_paths_from_flow_curve, on_create_button_click};
use crate::utils::ui_transform_from_button;
use bevy::math::{vec2, Vec3A};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_create_button(
    commands: &mut Commands,
    create_button: CreateButton,
    position: Vec2,
    angle: f32,
    zoom: f32,
    asset_server: &Res<AssetServer>,
) {
    let path = match create_button.ty {
        CreateButtonType::ImportInterface | CreateButtonType::ExportInterface => {
            "create-button/interface.png"
        }
        CreateButtonType::Inflow => "create-button/inflow.png",
        CreateButtonType::Outflow => "create-button/outflow.png",
        CreateButtonType::Source => "create-button/source.png",
        CreateButtonType::Sink => "create-button/sink.png",
        CreateButtonType::InterfaceSubsystem => "create-button/interface-subsystem.png",
    };

    let button_width = BUTTON_WIDTH_HALF * 2.0;

    let button_entity = commands
        .spawn((
            create_button,
            SpriteBundle {
                texture: asset_server.load(path),
                transform: Transform::from_translation((position * zoom).extend(10.))
                    .with_rotation(Quat::from_rotation_z(angle)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(button_width, button_width)),
                    ..default()
                },
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(on_create_button_click),
            InitialPosition::new(position),
        ))
        .id();

    let mut commands = commands.entity(create_button.connection_source);

    match create_button.ty {
        CreateButtonType::ImportInterface | CreateButtonType::ExportInterface => {
            commands.insert(FlowInterfaceButton);
        }
        CreateButtonType::Source | CreateButtonType::Sink => {
            commands.insert(FlowOtherEndButton);
        }
        CreateButtonType::InterfaceSubsystem => {
            commands.insert(InterfaceSubsystemButton { button_entity });
        }
        CreateButtonType::Inflow | CreateButtonType::Outflow => {
            // do nothing
        }
    }
}

pub fn despawn_create_button(
    commands: &mut Commands,
    entity: Entity,
    query: &Query<&CreateButton>,
) {
    let create_button = query
        .get(entity)
        .expect("Button doesn't have CreateButton component");

    despawn_create_button_with_component(commands, entity, create_button);
}

pub fn despawn_create_button_with_component(
    commands: &mut Commands,
    entity: Entity,
    create_button: &CreateButton,
) {
    let mut entity_commands = commands.entity(create_button.connection_source);

    match create_button.ty {
        CreateButtonType::ImportInterface | CreateButtonType::ExportInterface => {
            entity_commands.remove::<FlowInterfaceButton>();
        }
        CreateButtonType::Source | CreateButtonType::Sink => {
            entity_commands.remove::<FlowOtherEndButton>();
        }
        CreateButtonType::InterfaceSubsystem => {
            entity_commands.remove::<InterfaceSubsystemButton>();
        }
        CreateButtonType::Inflow | CreateButtonType::Outflow => {
            // do nothing
        }
    }

    commands.entity(entity).despawn();
}

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
            }
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

pub fn spawn_outflow(
    commands: &mut Commands,
    system_entity: Entity,
    transform: &GlobalTransform,
    initial_position: &InitialPosition,
    zoom: f32,
) {
    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 6.0, 0.0, zoom);

    let direction = transform.right().truncate();

    let flow_curve = FlowCurve {
        start: *initial_position,
        start_direction: direction * FLOW_END_LENGTH,
        end: *initial_position + direction * FLOW_LENGTH,
        end_direction: direction * -FLOW_END_LENGTH,
    };

    let (curve_path, head_path) = create_paths_from_flow_curve(&flow_curve, zoom);

    commands
        .spawn((
            Outflow {
                system: system_entity,
                substance_type: Default::default(),
                usability: Default::default(),
            },
            flow_curve,
            ShapeBundle {
                path: curve_path,
                ..default()
            },
            Stroke::new(Color::BLACK, 3.0),
            Fill::color(Color::WHITE),
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::Outflow,
            Name::new("Outflow"),
        ))
        .with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: head_path,
                    ..default()
                },
                Fill::color(Color::BLACK),
            ));
        });
}

pub fn spawn_inflow(
    commands: &mut Commands,
    system_entity: Entity,
    transform: &GlobalTransform,
    initial_position: &InitialPosition,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    zoom: f32,
) {
    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 6.0, 64.0, zoom);

    commands.spawn((
        Inflow {
            system: system_entity,
            substance_type: Default::default(),
            usability: Default::default(),
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(32.0, 32.0)).into(),
            transform,
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        },
        PickableBundle {
            selection: PickSelection { is_selected: true },
            ..default()
        },
        SystemElement::Inflow,
        Name::new("Inflow"),
        initial_position,
    ));
}

pub fn spawn_external_entity(
    commands: &mut Commands,
    interface_type: InterfaceType,
    flow_entity: Entity,
    transform: &GlobalTransform,
    initial_position: &InitialPosition,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    zoom: f32,
) {
    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 1.0, 0.0, zoom);

    let external_entity = commands
        .spawn((
            ExternalEntity::default(),
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(32.0, 32.0)).into(),
                transform,
                material: materials.add(ColorMaterial::from(Color::CYAN)),
                ..default()
            },
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::ExternalEntity,
            Name::new("External Entity"),
            initial_position,
        ))
        .id();

    let mut entity_commands = commands.entity(flow_entity);

    match interface_type {
        InterfaceType::Import => {
            entity_commands.insert(InflowSourceConnection {
                target: external_entity,
            });
        }
        InterfaceType::Export => {
            entity_commands.insert(OutflowSinkConnection {
                target: external_entity,
            });
        }
    }
}

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
                    SystemBundle::new(vec2(-radius, 0.0), 1.0, radius),
                ))
                .id();
        })
        .insert(InterfaceSubsystemConnection {
            target: subsystem_entity,
        });
}
