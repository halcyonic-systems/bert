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

pub trait HasSubstanceType {
    fn substance_type(&self) -> SubstanceType;
}

impl HasSubstanceType for Inflow {
    fn substance_type(&self) -> SubstanceType {
        self.substance_type
    }
}

impl HasSubstanceType for Outflow {
    fn substance_type(&self) -> SubstanceType {
        self.substance_type
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ExternalEntity;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Subsystem {
    pub parent_system: Entity,

    // would be 0 for the root system (which doesn't have this component)
    pub nesting_level: usize,
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

impl SubstanceType {
    pub fn flow_color(&self) -> Color {
        match self {
            SubstanceType::Energy => Color::rgb_u8(181, 27, 27),
            SubstanceType::Material => Color::BLACK,
            SubstanceType::Message => Color::GRAY,
        }
    }
}

#[derive(Clone, Debug, Component, Reflect, PartialEq, Default)]
#[reflect(Component)]
pub struct ElementDescription {
    pub text: String,
}

impl From<&str> for ElementDescription {
    fn from(text: &str) -> Self {
        Self { text: text.into() }
    }
}

impl From<String> for ElementDescription {
    fn from(text: String) -> Self {
        Self { text }
    }
}
