use crate::components::{
    CreateButton, CreateButtonType, ExternalEntity, FlowInterfaceButton, FlowInterfaceConnection,
    FlowOtherEndButton, FlowOtherEndConnection, FlowSystemConnection, Inflow, Interface,
    InterfaceSubsystem, InterfaceSubsystemButton, Outflow, SystemElement,
};
use crate::systems::on_create_button_click;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::prelude::*;

pub fn spawn_create_button(
    commands: &mut Commands,
    create_button: CreateButton,
    position: Vec2,
    asset_server: &Res<AssetServer>,
) {
    let path = match create_button.ty {
        CreateButtonType::Interface => "create-button/interface.png",
        CreateButtonType::Inflow => "create-button/inflow.png",
        CreateButtonType::Outflow => "create-button/outflow.png",
        CreateButtonType::ExternalEntity => "create-button/sink.png",
        CreateButtonType::InterfaceSubsystem => "create-button/interface-subsystem.png",
    };

    let button_entity = commands
        .spawn((
            create_button,
            SpriteBundle {
                texture: asset_server.load(path),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 10.)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32., 32.)),
                    ..default()
                },
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(on_create_button_click),
        ))
        .id();

    let mut commands = commands.entity(create_button.connection_source);

    match create_button.ty {
        CreateButtonType::Interface => {
            commands.insert(FlowInterfaceButton);
        }
        CreateButtonType::ExternalEntity => {
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

pub fn despawn_create_button_with_component(commands: &mut Commands, entity: Entity, create_button: &CreateButton) {
    let mut entity_commands = commands.entity(create_button.connection_source);

    match create_button.ty {
        CreateButtonType::Interface => {
            entity_commands.remove::<FlowInterfaceButton>();
        }
        CreateButtonType::ExternalEntity => {
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
    flow_entity: Entity,
    position: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let interface_entity = commands
        .spawn((
            Interface::default(),
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(32.0, 64.0)).into(),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 5.)),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            },
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::Interface,
            Name::new("Interface"),
        ))
        .id();

    commands
        .entity(flow_entity)
        .insert(FlowInterfaceConnection {
            target: interface_entity,
        });
}

pub fn spawn_outflow(
    commands: &mut Commands,
    system_entity: Entity,
    position: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Outflow::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(32.0, 32.0)).into(),
            transform: Transform::from_translation(Vec3::new(position.x + 64.0, position.y, 5.)),
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        },
        PickableBundle {
            selection: PickSelection { is_selected: true },
            ..default()
        },
        SystemElement::Outflow,
        FlowSystemConnection {
            target: system_entity,
        },
        Name::new("Outflow"),
    ));
}

pub fn spawn_inflow(
    commands: &mut Commands,
    system_entity: Entity,
    position: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Inflow::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(32.0, 32.0)).into(),
            transform: Transform::from_translation(Vec3::new(position.x - 64.0, position.y, 5.)),
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        },
        PickableBundle {
            selection: PickSelection { is_selected: true },
            ..default()
        },
        SystemElement::Inflow,
        FlowSystemConnection {
            target: system_entity,
        },
        Name::new("Inflow"),
    ));
}

pub fn spawn_external_entity(
    commands: &mut Commands,
    flow_entity: Entity,
    position: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let external_entity = commands
        .spawn((
            ExternalEntity::default(),
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(32.0, 32.0)).into(),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
                material: materials.add(ColorMaterial::from(Color::CYAN)),
                ..default()
            },
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::ExternalEntity,
            Name::new("External Entity"),
        ))
        .id();

    commands.entity(flow_entity).insert(FlowOtherEndConnection {
        target: external_entity,
    });
}

pub fn spawn_interface_subsystem(
    commands: &mut Commands,
    interface_entity: Entity,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.entity(interface_entity).with_children(|parent| {
        parent.spawn((
            InterfaceSubsystem::default(),
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle { radius: 16. }).into(), // TODO : compute radius from parent system
                transform: Transform::from_translation(Vec3::new(0.0, 10.0, 1.0)), // TODO : compute z from parent system
                material: materials.add(ColorMaterial::from(Color::CYAN)),
                ..default()
            },
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
        ));
    });
}
