use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowEndConnection {
    pub target: Entity,
    pub target_type: EndTargetType,
}

pub trait TargetTypeConnection {
    fn target_is_external_entity(&self) -> bool;
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum EndTargetType {
    Sink,
    System,
}

impl TargetTypeConnection for FlowEndConnection {
    fn target_is_external_entity(&self) -> bool {
        matches!(self.target_type, EndTargetType::Sink)
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowStartConnection {
    pub target: Entity,
    pub target_type: StartTargetType,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum StartTargetType {
    Source,
    System,
}

impl TargetTypeConnection for FlowStartConnection {
    fn target_is_external_entity(&self) -> bool {
        matches!(self.target_type, StartTargetType::Source)
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowEndInterfaceConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowStartInterfaceConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct InterfaceSubsystemConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct SubsystemParentFlowConnection {
    pub target: Entity,
}

pub trait Connection {
    fn target(&self) -> Entity;
}

macro_rules! impl_connection {
    ($name:ident) => {
        impl Connection for $name {
            #[inline(always)]
            fn target(&self) -> Entity {
                self.target
            }
        }
    };
}

impl_connection!(FlowStartConnection);
impl_connection!(FlowEndConnection);
impl_connection!(FlowStartInterfaceConnection);
impl_connection!(FlowEndInterfaceConnection);
impl_connection!(InterfaceSubsystemConnection);
impl_connection!(SubsystemParentFlowConnection);
