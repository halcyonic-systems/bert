use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowInterfaceConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowOtherEndConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowSystemConnection {
    pub target: Entity,
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

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum SystemElement {
    System,
    Interface,
    Inflow,
    Outflow,
    ExternalEntity,
}

impl std::fmt::Display for SystemElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemElement::System => write!(f, "System"),
            SystemElement::Interface => write!(f, "Interface"),
            SystemElement::Inflow => write!(f, "Inflow"),
            SystemElement::Outflow => write!(f, "Outflow"),
            SystemElement::ExternalEntity => write!(f, "External Entity"),
        }
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct System;

#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Interface {
    pub name: String,
    pub ty: InterfaceType,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum InterfaceType {
    Import,
    Export,
    // TODO : Hybrid,
}

#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Inflow {
    pub name: String,
    pub usability: InflowUsability,
    pub substance_type: SubstanceType,
}

#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Outflow {
    pub name: String,
    pub usability: OutflowUsability,
    pub substance_type: SubstanceType,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct ExternalEntity;

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash)]
pub enum InflowUsability {
    Resource,
    Disruption,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash)]
pub enum OutflowUsability {
    Product,
    Waste,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash)]
pub enum GeneralUsability {
    Inflow(InflowUsability),
    Outflow(OutflowUsability),
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum SubstanceType {
    Material,
    Energy,
    Message,
}
