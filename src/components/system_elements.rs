//! This file contains all the Bevy components and data structures related to System Elements and their associated helper methods.
use crate::data_model::Complexity;
use bevy::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Corresponds to the System Language elements and their visual representation in the diagram.
#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum SystemElement {
    /// Is a circle
    System,
    /// Is a rectangle
    Interface,
    // Is a line + arrow head
    Interaction,
    // Is a partially enclosed rectangle
    ExternalEntity,
}

impl std::fmt::Display for SystemElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemElement::System => write!(f, "System"),
            SystemElement::Interface => write!(f, "Interface"),
            SystemElement::Interaction => write!(f, "Interaction"),
            SystemElement::ExternalEntity => write!(f, "External Entity"),
        }
    }
}

/// Attached to entities with a SystemElement::System component to hold graphical and modeling data.
#[derive(Clone, Debug, Component, Reflect, PartialEq, Default)]
#[reflect(Component)]
pub struct System {
    pub radius: f32,
    pub complexity: Complexity,
    pub membership: f32,
    pub equivalence: String,
    pub transformation: String,
    pub history: String,
    pub boundary: SystemBoundary,
    pub time_unit: String,
}

/// Attached to entities with a SystemElement::System component to hold modeling data related to the system's boundary.
#[derive(Clone, Debug, PartialEq, Reflect, Default)]
pub struct SystemBoundary {
    pub porosity: f32,             // TODO: Implement Decimal Type
    pub perceptive_fuzziness: f32, // TODO: Implement Decimal Type
    pub name: String,
    pub description: String,
}

/// Attached to the top level System of Interest to hold modeling data related to the environment.
#[derive(Clone, Debug, PartialEq, Reflect, Default, Component)]
#[reflect(Component)]
pub struct SystemEnvironment {
    pub name: String,
    pub description: String,
}

/// Attached to entities with a SystemElement::Interface component to hold modeling data related to the interface.
#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct Interface {
    pub protocol: String,
}

/// Represents whether an interface imports into or exports flow out of a system. It's to determine control flow in different systems
#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum InterfaceType {
    Import,
    Export,
}

/// Attached to entities with a SystemElement::Interaction component to hold modeling data related to the interaction.
#[derive(Component, Clone, Debug, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Flow {
    pub interaction_type: InteractionType,
    pub substance_type: SubstanceType,
    pub substance_sub_type: String,
    #[reflect(ignore)]
    pub amount: Decimal,
    pub unit: String,
    pub usability: InteractionUsability,
    pub parameters: Vec<Parameter>,
}

/// Represents a user-defined parameter stored in a Flow.
#[derive(Clone, Debug, Reflect, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

/// Corresponds to the System Language Interaction types.
#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum InteractionType {
    #[default]
    Flow,
    Force,
}

/// Attached to entities with a SystemElement::ExternalEntity component to hold modeling data related to the external entity.
#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ExternalEntity {
    pub equivalence: String,
    pub model: String,
}

/// Attached to entities with a System component that are nested inside of a parent system.
/// Used to define the system-subsystem relation from the subsystem's perspective.
/// All subsystems are systems. Not all systems are subsystems.
#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Subsystem {
    pub parent_system: Entity,
}

/// Marker component for Interface Subsystems that import flows into a system.
#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ImportSubsystem;

/// Marker component for Interface Subsystems that export flows out of a system.
#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ExportSubsystem;

/// Used to establish a relation to the associated Interface the InterfaceSubsystem is created by.
/// Holds modeling data.
#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct InterfaceSubsystem {
    pub interface_entity: Entity,
    #[reflect(ignore)]
    pub total_inflow: Decimal,
    #[reflect(ignore)]
    pub total_outflow: Decimal,
    pub substance_type: SubstanceType,
    pub is_useful: bool,
}

impl InterfaceSubsystem {
    pub fn new(interface_entity: Entity) -> Self {
        Self {
            interface_entity,
            total_inflow: Default::default(),
            total_outflow: Default::default(),
            substance_type: Default::default(),
            is_useful: false,
        }
    }
}

/// Corresponds to System Language interaction types. Used to determine to app control flow.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub enum InteractionUsability {
    Resource,
    Disruption,
    Product,
    Waste,
}

impl InteractionUsability {
    #[inline(always)]
    /// Helper method to determine generically if an interaction is "usable"
    pub fn is_useful(&self) -> bool {
        matches!(self, Self::Resource | Self::Product)
    }

    #[inline(always)]
    pub fn is_export(&self) -> bool {
        matches!(self, Self::Product | Self::Waste)
    }

    #[inline(always)]
    pub fn is_import(&self) -> bool {
        !self.is_export()
    }
}

/// Corresponds to System Language fundemental substance types. Used to determine graphical representation and app control flow.
#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SubstanceType {
    #[default]
    Energy,
    Material,
    Message,
}

impl SubstanceType {
    // Helper method to determine a the color of a flow from it's substance type.
    pub fn flow_color(&self) -> Color {
        match self {
            SubstanceType::Energy => Color::rgb_u8(181, 27, 27),
            SubstanceType::Material => Color::GRAY,
            SubstanceType::Message => Color::SILVER,
        }
    }
    // Helper method to determine a the color of an interface from it's substance type.
    pub fn interface_color(&self) -> Color {
        match self {
            SubstanceType::Energy => Color::rgb_u8(233, 182, 178),
            SubstanceType::Material => Color::GRAY,
            SubstanceType::Message => Color::SILVER,
        }
    }
}

/// Attached to all entities with a SystemElement component. Stores user input.
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
