use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct CreateButton {
    pub ty: CreateButtonType,
    pub connection_source: Entity,
    pub system: Entity,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum CreateButtonType {
    ImportInterface,
    ExportInterface,
    Inflow,
    Outflow,
    Source,
    Sink,
    InterfaceSubsystem { is_child_of_interface: bool },
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowInterfaceButton;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowOtherEndButton;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct InterfaceSubsystemButton {
    pub button_entity: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct FlowCurve {
    pub start: Vec2,
    pub end: Vec2,
    pub start_direction: Vec2,
    pub end_direction: Vec2,
}
