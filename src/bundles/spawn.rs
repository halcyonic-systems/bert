use crate::components::*;
use crate::systems::on_create_button_click;
use crate::utils::ui_transform_from_button;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
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

    let button_entity = commands
        .spawn((
            create_button,
            SpriteBundle {
                texture: asset_server.load(path),
                transform: Transform::from_translation((position * zoom).extend(10.))
                    .with_rotation(Quat::from_rotation_z(angle)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32., 32.)),
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
) {
    let points = [
        Vec2::new(12.0, 30.0),   // top right
        Vec2::new(-12.0, 30.0),  // top left
        Vec2::new(-12.0, -30.0), // bottom left
        Vec2::new(12.0, -30.0),  // bottom right
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
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    zoom: f32,
) {
    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 6.0, 64.0, zoom);

    commands.spawn((
        Outflow {
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
        SystemElement::Outflow,
        Name::new("Outflow"),
        initial_position,
    ));
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
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
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
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle { radius: 32. }).into(), // TODO : compute radius from parent system
                        transform: Transform::from_translation(Vec3::new(-32.0, 0.0, 1.0)), // TODO : compute z from parent system
                        material: materials.add(ColorMaterial::from(Color::CYAN)),
                        ..default()
                    },
                    PickableBundle {
                        selection: PickSelection { is_selected: true },
                        ..default()
                    },
                ))
                .id();
        })
        .insert(InterfaceSubsystemConnection {
            target: subsystem_entity,
        });
}
