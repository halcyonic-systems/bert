use bevy::prelude::*;
use rust_decimal::Decimal;
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

#[derive(Component, Clone, Debug, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Flow {
    pub substance_type: SubstanceType,
    #[reflect(ignore)]
    pub amount: Decimal,
    pub unit: String,
    #[reflect(ignore)]
    pub time_unit: Decimal,
    pub is_useful: bool,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ExternalEntity;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Subsystem {
    pub parent_system: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ImportSubsystem;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ExportSubsystem;

#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct InterfaceSubsystem {
    #[reflect(ignore)]
    total_inflow: Decimal,
    #[reflect(ignore)]
    total_outflow: Decimal,
    substance_type: SubstanceType,
    is_useful: bool,
}

pub trait Usability: Sized {
    fn is_useful(&self) -> bool;

    fn from_useful(is_useful: bool) -> Self;

    #[inline(always)]
    fn mutate<F: FnOnce(&mut Self)>(value: &mut bool, f: F) {
        let mut usability = Self::from_useful(*value);
        f(&mut usability);
        *value = usability.is_useful();
    }
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum InflowUsability {
    #[default]
    Resource,
    Disruption,
}

impl Usability for InflowUsability {
    fn is_useful(&self) -> bool {
        matches!(self, InflowUsability::Resource)
    }

    fn from_useful(is_useful: bool) -> Self {
        match is_useful {
            true => InflowUsability::Resource,
            false => InflowUsability::Disruption,
        }
    }
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum OutflowUsability {
    #[default]
    Product,
    Waste,
}

impl Usability for OutflowUsability {
    fn is_useful(&self) -> bool {
        matches!(self, OutflowUsability::Product)
    }

    fn from_useful(is_useful: bool) -> Self {
        match is_useful {
            true => OutflowUsability::Product,
            false => OutflowUsability::Waste,
        }
    }
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
