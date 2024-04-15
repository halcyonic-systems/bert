use crate::components::{
    CreateButton, CreateButtonType, ExternalEntity, FlowInterfaceButton, FlowInterfaceConnection,
    FlowOtherEndButton, FlowOtherEndConnection, FlowSystemConnection, Inflow, Interface,
    InterfaceSubsystemButton, Outflow, SystemElement,
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

pub fn spawn_interface(
    commands: &mut Commands,
    flow_entity: Entity,
    interface: Interface,
    position: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let interface_entity = commands
        .spawn((
            interface,
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 5.))
                    .with_scale(vec3(32., 64., 1.)),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            },
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::Interface,
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
    outflow: Outflow,
    position: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        outflow,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::from_translation(Vec3::new(position.x + 64.0, position.y, 5.))
                .with_scale(vec3(32., 32., 1.)),
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
    ));
}

pub fn spawn_inflow(
    commands: &mut Commands,
    system_entity: Entity,
    inflow: Inflow,
    position: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        inflow,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::from_translation(Vec3::new(position.x - 64.0, position.y, 5.))
                .with_scale(vec3(32., 32., 1.)),
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
    ));
}

pub fn spawn_external_entity(
    commands: &mut Commands,
    flow_entity: Entity,
    external_entity: ExternalEntity,
    position: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let external_entity = commands
        .spawn((
            external_entity,
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.))
                    .with_scale(vec3(32., 32., 1.)),
                material: materials.add(ColorMaterial::from(Color::CYAN)),
                ..default()
            },
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::ExternalEntity,
        ))
        .id();

    commands.entity(flow_entity).insert(FlowOtherEndConnection {
        target: external_entity,
    });
}
