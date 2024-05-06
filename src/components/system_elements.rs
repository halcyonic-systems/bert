use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Component, Reflect, PartialEq, Default)]
#[reflect(Component)]
pub struct System {
    pub radius: f32,
    pub adaptable: bool,
    pub evolveable: bool,
    pub boundary: SystemBoundary,
}

#[derive(Clone, Debug, PartialEq, Reflect, Default)]
pub struct SystemBoundary {
    pub porosity: f32,
    pub perceptive_fuzziness: f32,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Reflect, Default, Component)]
#[reflect(Component)]
pub struct SystemEnvironment {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct Interface {
    pub protocol: String,
}

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
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum InflowUsability {
    #[default]
    Resource,
    Disruption,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Default, Serialize, Deserialize)]
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
    pub fn interface_color(&self) -> Color {
        match self {
            SubstanceType::Energy => Color::rgb_u8(233, 182, 178),
            SubstanceType::Material => Color::DARK_GRAY,
            SubstanceType::Message => Color::GRAY,
        }
    }
}

#[derive(Clone, Debug, Component, Reflect, PartialEq, Default)]
#[reflect(Component)]
pub struct ElementDescription {
    pub text: String,
}

impl ElementDescription {
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }
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
