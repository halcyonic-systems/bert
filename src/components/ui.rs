use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct CreateButton {
    pub ty: CreateButtonType,
    pub connection_source: Entity,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum CreateButtonType {
    Interface,
    Inflow,
    Outflow,
    ExternalEntity,
    InterfaceSubsystem,
}
