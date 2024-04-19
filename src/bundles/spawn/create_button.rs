use crate::components::*;
use crate::constants::BUTTON_WIDTH_HALF;
use crate::systems::on_create_button_click;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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
