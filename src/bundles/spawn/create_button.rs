use crate::components::*;
use crate::constants::{BUTTON_WIDTH_HALF, BUTTON_Z};
use crate::systems::{
    on_create_button_click, on_external_entity_create_button_click, on_flow_terminal_button_click,
    on_subsystem_button_click,
};
use bevy::prelude::*;

pub fn spawn_create_button(
    commands: &mut Commands,
    create_button: CreateButton,
    position: Vec2,
    angle: f32,
    zoom: f32,
    parent: Option<Entity>,
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
        CreateButtonType::InterfaceSubsystem { .. } => "create-button/interface-subsystem.png",
        CreateButtonType::FlowTerminalStart => "create-button/source.png", // TODO
        CreateButtonType::FlowTerminalEnd => "create-button/sink.png",     // TODO
        CreateButtonType::Subsystem => "create-button/interface-subsystem.png", // TODO
    };

    let name = match create_button.ty {
        CreateButtonType::ImportInterface => "Import Interface Button",
        CreateButtonType::ExportInterface => "Export Interface Button",
        CreateButtonType::Inflow => "Inflow Button",
        CreateButtonType::Outflow => "Outflow Button",
        CreateButtonType::Source => "Source Button",
        CreateButtonType::Sink => "Sink Button",
        CreateButtonType::InterfaceSubsystem { .. } => "Interface Subsystem Button",
        CreateButtonType::FlowTerminalStart => "Flow Target Start Button",
        CreateButtonType::FlowTerminalEnd => "Flow Target End Button",
        CreateButtonType::Subsystem => "Subsystem Button",
    };

    let button_width = BUTTON_WIDTH_HALF * 2.0;

    let mut spawn_image = &mut commands.spawn((
        create_button,
        Sprite {
            image: asset_server.load(path),
            custom_size: Some(Vec2::new(button_width, button_width)),
            ..default()
        },
        Transform::from_translation((position * zoom).extend(BUTTON_Z))
            .with_rotation(Quat::from_rotation_z(angle)),
        InitialPosition::new(position),
        Name::new(name),
        PickingBehavior::default(),
    ));

    spawn_image = match create_button.ty {
        CreateButtonType::Subsystem => spawn_image.observe(on_subsystem_button_click),
        CreateButtonType::FlowTerminalStart | CreateButtonType::FlowTerminalEnd => {
            spawn_image.observe(on_flow_terminal_button_click)
        }
        CreateButtonType::Source | CreateButtonType::Sink => {
            spawn_image.observe(on_external_entity_create_button_click)
        }
        _ => spawn_image.observe(on_create_button_click),
    };

    let button_entity = spawn_image.id();

    if let Some(parent) = parent {
        commands.entity(parent).add_children(&[button_entity]);
    }

    let mut commands = commands.entity(create_button.connection_source);

    match create_button.ty {
        CreateButtonType::ImportInterface | CreateButtonType::ExportInterface => {
            commands.insert(HasFlowInterfaceButton { button_entity });
        }
        CreateButtonType::Source
        | CreateButtonType::Sink
        | CreateButtonType::FlowTerminalStart
        | CreateButtonType::FlowTerminalEnd => {
            commands.insert(HasFlowOtherEndButton);
        }
        CreateButtonType::InterfaceSubsystem { .. } => {
            commands.insert(HasInterfaceSubsystemButton { button_entity });
        }
        CreateButtonType::Inflow | CreateButtonType::Outflow | CreateButtonType::Subsystem => {
            // do nothing
        }
    }
}

pub fn despawn_create_button(
    commands: &mut Commands,
    entity: Entity,
    query: &Query<(&CreateButton, Option<&Parent>)>,
) {
    let (create_button, parent) = query
        .get(entity)
        .expect("Button doesn't have CreateButton component");

    despawn_create_button_with_component(commands, entity, create_button, parent);
}

pub fn despawn_create_button_with_component(
    commands: &mut Commands,
    entity: Entity,
    create_button: &CreateButton,
    parent: Option<&Parent>,
) {
    let mut entity_commands = commands.entity(create_button.connection_source);

    match create_button.ty {
        CreateButtonType::ImportInterface | CreateButtonType::ExportInterface => {
            entity_commands.remove::<HasFlowInterfaceButton>();
        }
        CreateButtonType::Source
        | CreateButtonType::Sink
        | CreateButtonType::FlowTerminalStart
        | CreateButtonType::FlowTerminalEnd => {
            entity_commands.remove::<HasFlowOtherEndButton>();
        }
        CreateButtonType::InterfaceSubsystem { .. } => {
            entity_commands.remove::<HasInterfaceSubsystemButton>();
        }
        CreateButtonType::Inflow | CreateButtonType::Outflow | CreateButtonType::Subsystem => {
            // do nothing
        }
    }

    if let Some(parent) = parent {
        commands.entity(parent.get()).remove_children(&[entity]);
    }

    commands.entity(entity).despawn();
}
