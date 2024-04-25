use bevy::prelude::*;

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

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Default)]
#[reflect(Component)]
pub struct System {
    pub radius: f32,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct Interface;

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum InterfaceType {
    Import,
    Export,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Inflow {
    pub usability: InflowUsability,
    pub substance_type: SubstanceType,
    pub system: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Outflow {
    pub usability: OutflowUsability,
    pub substance_type: SubstanceType,
    pub system: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ExternalEntity;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Subsystem {
    pub parent_system: Entity,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash, Default)]
pub enum InflowUsability {
    #[default]
    Resource,
    Disruption,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash, Default)]
pub enum OutflowUsability {
    #[default]
    Product,
    Waste,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash)]
pub enum GeneralUsability {
    Inflow(InflowUsability),
    Outflow(OutflowUsability),
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Default)]
pub enum SubstanceType {
    #[default]
    Energy,
    Material,
    Message,
}
