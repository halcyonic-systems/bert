//! # System Elements Module
//!
//! This module defines the core system elements that make up a BERT system model,
//! implementing the System Language (SL) framework for bounded entity reasoning.
//!
//! ## Architecture
//!
//! System elements are the fundamental building blocks that represent different components
//! of a complex adaptive system:
//!
//! - **Systems**: Bounded entities with internal structure and behavior
//! - **Interfaces**: Connection points that define system boundaries
//! - **Flows**: Interactions that move substances between system elements
//! - **External Entities**: Sources and sinks outside the system boundary
//!
//! ## Key Components
//!
//! - [`SystemElement`]: Base enumeration defining all element types
//! - [`System`]: Core system component with boundary and complexity properties
//! - [`Flow`]: Interaction component representing substance movement
//! - [`Interface`]: Boundary component for system connections
//! - [`ExternalEntity`]: Component for sources and sinks
//!
//! ## Usage Patterns
//!
//! System elements are typically created through spawn bundles and manipulated
//! via Bevy's Entity Component System (ECS). Each element type has specific
//! visual representations and behavioral properties defined by their components.
//!
//! ## System Language Implementation
//!
//! This module implements Layer 2 (Formal Specification) of the System Language
//! framework, providing the mathematical foundation for system modeling through
//! structured component definitions.

use crate::bevy_app::data_model::Complexity;

use bevy::prelude::*;
use enum_iterator::Sequence;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use uuid::Uuid;

/// Defines the fundamental types of system elements in the System Language framework.
///
/// Each variant corresponds to a specific mathematical construct in systems theory
/// and has a distinct visual representation in the BERT diagram interface.
///
/// # System Language Mapping
///
/// - **System**: Bounded entities (circles) representing complex adaptive systems
/// - **Interface**: Boundary components (rectangles) defining system interaction points
/// - **Interaction**: Flow connections (lines with arrows) representing substance movement
/// - **External Entity**: Environmental actors (partially enclosed rectangles) as sources/sinks
///
/// # Visual Representation
///
/// Each element type has specific geometric properties and rendering behaviors
/// defined in the corresponding spawn bundles.
///
/// # Examples
///
/// ```rust
/// use bert::SystemElement;
///
/// // Create different system element types
/// let system = SystemElement::System;
/// let interface = SystemElement::Interface;
/// let flow = SystemElement::Interaction;
/// let source = SystemElement::ExternalEntity;
///
/// // Elements can be compared and used in match expressions
/// match system {
///     SystemElement::System => println!("This is a bounded system"),
///     SystemElement::Interface => println!("This is a system boundary"),
///     SystemElement::Interaction => println!("This is a flow between systems"),
///     SystemElement::ExternalEntity => println!("This is an environmental actor"),
/// }
/// ```
#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum SystemElement {
    /// Bounded entity representing a complex adaptive system (rendered as circle).
    ///
    /// Systems are the primary structural elements that contain internal components,
    /// maintain boundaries, and exhibit emergent behaviors.
    System,

    /// Boundary component defining system interaction points (rendered as rectangle).
    ///
    /// Interfaces represent the formal connection points where flows can enter
    /// or exit a system, implementing the System Language boundary concept.
    Interface,

    /// Flow connection representing substance movement (rendered as line with arrow).
    ///
    /// Interactions capture the dynamic aspects of systems by modeling the
    /// movement of energy, material, or information between system elements.
    Interaction,

    /// Environmental actor outside system boundaries (rendered as partially enclosed rectangle).
    ///
    /// External entities represent sources and sinks in the system environment
    /// that provide inputs or receive outputs from the system of interest.
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

/// Component representing a bounded system with internal structure and emergent properties.
///
/// A `System` implements the mathematical definition of a system in the System Language
/// framework, containing both structural (boundary, complexity) and behavioral
/// (transformation, history) properties.
///
/// # Mathematical Foundation
///
/// This component represents the system tuple: `S = (C, N, G, B, T, H, Δt)`
/// where the boundary, complexity, and transformation properties are explicitly modeled.
///
/// # Invariants
///
/// - `radius` must be positive and finite
/// - `membership` should be in the range [0.0, 1.0] representing degree of belonging
/// - `boundary` defines the system's interaction capabilities with its environment
///
/// # Threading
///
/// This component is not thread-safe and should only be accessed from the main thread
/// within the Bevy ECS context.
///
/// # Examples
///
/// ```rust
/// use bert::{System, SystemBoundary, Complexity};
///
/// // Create a simple atomic system
/// let system = System {
///     radius: 50.0,
///     complexity: Complexity::Atomic,
///     membership: 1.0,
///     equivalence: "Processing Unit".to_string(),
///     transformation: "Input -> Process -> Output".to_string(),
///     history: "Created for manufacturing analysis".to_string(),
///     boundary: SystemBoundary::default(),
///     time_unit: "seconds".to_string(),
/// };
/// ```
///
/// # See Also
///
/// - [`SystemBoundary`]: Defines the system's boundary properties
/// - [`Complexity`]: Categorizes the system's internal structure
/// - [`Subsystem`]: For systems nested within other systems
#[derive(Clone, Debug, Component, Reflect, PartialEq, Default)]
#[reflect(Component)]
pub struct System {
    /// Visual radius of the system in pixels (must be positive).
    ///
    /// Determines the size of the circular representation in the diagram.
    /// Larger radius indicates greater scope or importance in the model.
    pub radius: f32,

    /// Complexity classification of the system's internal structure.
    ///
    /// Determines whether the system contains subsystems, multiple instances,
    /// or represents an atomic (indivisible) entity.
    pub complexity: Complexity,

    /// Degree of membership in the system hierarchy (0.0 to 1.0).
    ///
    /// Represents how strongly this system belongs to its parent system
    /// or the overall system of interest.
    pub membership: f32,

    /// Equivalence class or category that this system represents.
    ///
    /// Used for grouping similar systems and establishing relationships
    /// between systems with comparable functions or structures.
    pub equivalence: String,

    /// Description of the system's transformation process or function.
    ///
    /// Captures the primary purpose or behavioral pattern of the system,
    /// describing how it processes inputs to produce outputs.
    pub transformation: String,

    /// Historical context and evolution of the system.
    ///
    /// Records the system's development over time, including past states,
    /// changes, and contextual information relevant to its current configuration.
    pub history: String,

    /// Boundary definition specifying interaction capabilities.
    ///
    /// Defines how the system interfaces with its environment and what
    /// types of interactions are possible across the system boundary.
    pub boundary: SystemBoundary,

    /// Temporal unit for system operations and measurements.
    ///
    /// Specifies the time scale at which the system operates, enabling
    /// proper temporal modeling and analysis of system behaviors.
    pub time_unit: String,

    /// HCGS archetype classification (Mobus 2022).
    ///
    /// Categorizes the system according to the Hierarchical Cybernetic
    /// Governance System framework: Governance (coordination/control),
    /// Economy (production/exchange), or Agent (autonomous actors).
    pub archetype: HcgsArchetype,
}

/// Defines the boundary properties of a system, controlling interaction capabilities.
///
/// The boundary represents the formal separation between a system and its environment,
/// implementing System Language boundary theory through porosity and perceptive properties.
///
/// # System Language Theory
///
/// Boundaries in systems theory are not simple barriers but complex interfaces that:
/// - Control what can pass through (porosity)
/// - Determine what the system can perceive (perceptive_fuzziness)
/// - Define the system's identity and autonomy
///
/// # Field Descriptions
///
/// - **Porosity**: Measures how easily substances can cross the boundary (0.0 = impermeable, 1.0 = fully open)
/// - **Perceptive Fuzziness**: Represents uncertainty in boundary perception (0.0 = perfectly defined, 1.0 = completely ambiguous)
///
/// # Examples
///
/// ```rust
/// use bert::SystemBoundary;
///
/// // Create a semi-permeable boundary with some uncertainty
/// let boundary = SystemBoundary {
///     porosity: 0.7,
///     perceptive_fuzziness: 0.2,
///     name: "Cell Membrane".to_string(),
///     description: "Selectively permeable biological boundary".to_string(),
/// };
///
/// // Create a rigid boundary for a mechanical system
/// let rigid_boundary = SystemBoundary {
///     porosity: 0.1,
///     perceptive_fuzziness: 0.05,
///     name: "Steel Container".to_string(),
///     description: "Rigid physical containment".to_string(),
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Reflect, Default)]
pub struct SystemBoundary {
    /// Permeability measure controlling substance flow across the boundary (0.0-1.0).
    ///
    /// Higher values indicate more open boundaries that allow easier exchange
    /// with the environment. Lower values represent more controlled or selective boundaries.
    /// TODO: Implement Decimal Type for precise numerical modeling
    pub porosity: f32,

    /// Uncertainty measure in boundary definition and perception (0.0-1.0).
    ///
    /// Represents how clearly the boundary can be distinguished from the environment.
    /// Higher values indicate more ambiguous or fuzzy boundary definitions.
    /// TODO: Implement Decimal Type for precise numerical modeling
    pub perceptive_fuzziness: f32,

    /// Human-readable identifier for the boundary type or category.
    pub name: String,

    /// Detailed description of the boundary's characteristics and behavior.
    pub description: String,
}

/// Attached to the top level System of Interest to hold modeling data related to the environment.
#[derive(Clone, Debug, PartialEq, Reflect, Default, Component)]
#[reflect(Component)]
pub struct SystemEnvironment {
    pub name: String,
    pub description: String,
    /// Milieu (M in Mobus 8-tuple): Ambient environmental properties that "bathe" the system
    /// but don't flow through discrete interfaces. Examples: Temperature, Humidity, Salinity, pH, Pressure.
    /// Stored as key-value pairs (property name, property value with unit).
    pub milieu: Vec<(String, String)>,
}

/// Attached to entities with a SystemElement::Interface component to hold modeling data related to the interface.
#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct Interface {
    pub protocol: String,
}

/// Marks a Subsystem as having Interface behavior (I ⊆ C per Mobus 8-tuple).
///
/// Phase 3A: Enables Interface ↔ Subsystem flows by treating interfaces as special subsystems.
/// Interfaces have dual role: boundary component (Interface) + internal node (Subsystem).
///
/// # Mobus Theory Foundation
///
/// Per Mobus systems theory: "Interfaces are special subsystems" (I ⊆ C).
/// This component implements that principle through composition pattern:
/// - Interface = Subsystem + InterfaceBehavior
///
/// # Usage
///
/// Attach this component to a Subsystem entity to mark it as capable of:
/// - Import/export process modeling (receiving flows from environment)
/// - Internal transformation (processing substances)
/// - Distribution to other subsystems (passing processed substances)
///
/// # Examples
///
/// ```rust
/// // Interface entity that can act as subsystem node in N network
/// commands.spawn((
///     Subsystem { parent_system },
///     InterfaceBehavior {
///         substance_type: SubstanceType::Energy,
///         protocol: "HTTP/2".to_string(),
///     },
///     // ... other components
/// ));
/// ```
#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct InterfaceBehavior {
    /// Type of substance this interface handles (Energy, Material, Message).
    pub substance_type: SubstanceType,
    /// Communication protocol or interaction mechanism.
    pub protocol: String,
}

/// Represents whether an interface imports into or exports flow out of a system. It's to determine control flow in different systems
#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum InterfaceType {
    Import,
    Export,
}

/// Component representing a flow interaction between system elements.
///
/// Flows model the dynamic exchange of substances (energy, material, or information)
/// between system components, implementing the interaction aspects of System Language theory.
///
/// # System Language Implementation
///
/// A flow captures the "N" (internal interactions) and "G" (external interactions)
/// components of the system tuple, representing how systems exchange substances
/// with their environment and internal components.
///
/// # Flow Characterization
///
/// Each flow is characterized by:
/// - **Type**: Whether it represents flow or force interactions
/// - **Substance**: What is being exchanged (energy, material, message)
/// - **Amount**: Quantitative measure of the interaction strength
/// - **Usability**: Directional and utility classification
/// - **Parameters**: Custom attributes for specific modeling needs
///
/// # Examples
///
/// ```rust
/// use bert::{Flow, InteractionType, SubstanceType, InteractionUsability};
/// use rust_decimal::Decimal;
///
/// // Create an energy flow representing electrical power
/// let power_flow = Flow {
///     interaction_type: InteractionType::Flow,
///     substance_type: SubstanceType::Energy,
///     substance_sub_type: "Electrical".to_string(),
///     amount: Decimal::new(1500, 0), // 1500 watts
///     unit: "watts".to_string(),
///     usability: InteractionUsability::Resource,
///     parameters: vec![],
/// };
/// ```
///
/// # See Also
///
/// - [`InteractionType`]: Defines flow vs. force interactions
/// - [`SubstanceType`]: Categorizes what flows through the interaction
/// - [`InteractionUsability`]: Classifies direction and utility
/// - [`Parameter`]: Provides additional flow characterization
#[derive(Component, Clone, Debug, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct Flow {
    /// Classification of the interaction mechanism (flow vs. force).
    ///
    /// Determines how the interaction behaves and is visualized in the system model.
    pub interaction_type: InteractionType,

    /// Primary substance category being exchanged in this interaction.
    ///
    /// Determines visual representation and helps categorize the nature of the exchange.
    pub substance_type: SubstanceType,

    /// Specific subtype or detailed classification of the substance.
    ///
    /// Provides additional specificity beyond the primary substance type,
    /// allowing for detailed modeling of different varieties within each category.
    pub substance_sub_type: String,

    /// Quantitative measure of the interaction strength or magnitude.
    ///
    /// Uses high-precision decimal arithmetic for accurate numerical modeling.
    /// The amount represents the quantity of substance flowing per time unit.
    #[reflect(ignore)]
    pub amount: Decimal,

    /// Unit of measurement for the amount value.
    ///
    /// Should follow standard unit conventions to ensure consistency
    /// and enable proper analysis across different flows in the system.
    pub unit: String,

    /// Classification of the flow's utility and directional nature.
    ///
    /// Determines whether this flow represents a beneficial or harmful
    /// exchange and whether it imports into or exports from the system.
    pub usability: InteractionUsability,

    /// Additional user-defined parameters that characterize this flow.
    ///
    /// Allows users to specify custom attributes, constraints, or properties
    /// that are specific to their modeling requirements.
    pub parameters: Vec<Parameter>,

    /// Smart parameters with enhanced type system (MVP - runtime only)
    ///
    /// Provides context-aware parameter suggestions and supports multiple data types
    /// including numeric, ordinal, categorical, and boolean parameters.
    pub smart_parameters: Vec<crate::bevy_app::smart_parameters::SmartParameter>,
}

/// Represents a user-defined parameter that provides additional context for flow interactions.
///
/// Parameters allow users to specify custom attributes that characterize the nature,
/// constraints, or properties of substance flows between system elements.
///
/// # Usage Patterns
///
/// Parameters are commonly used to specify:
/// - Physical properties (temperature, pressure, voltage)
/// - Quality measures (purity, efficiency, reliability)
/// - Constraints (maximum flow rate, acceptable ranges)
/// - Contextual information (source location, processing requirements)
///
/// # Serialization
///
/// The `id` field is automatically generated and excluded from serialization to ensure
/// consistent parameter identification across save/load cycles.
///
/// # Examples
///
/// ```rust
/// use bert::Parameter;
///
/// // Create a temperature parameter for an energy flow
/// let temperature = Parameter {
///     id: uuid::Uuid::new_v4(), // Auto-generated
///     name: "Temperature".to_string(),
///     value: "350".to_string(),
///     unit: "Celsius".to_string(),
/// };
///
/// // Create a flow rate constraint for material flow
/// let flow_rate = Parameter {
///     id: uuid::Uuid::new_v4(),
///     name: "Max Flow Rate".to_string(),
///     value: "50".to_string(),
///     unit: "kg/min".to_string(),
/// };
/// ```
///
/// # See Also
///
/// - [`Flow`]: Contains a vector of parameters to characterize interactions
#[derive(Clone, Debug, Reflect, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    /// Unique identifier for this parameter (excluded from serialization).
    ///
    /// Automatically generated to distinguish parameters even when they have
    /// identical names or values. Used internally for parameter management.
    #[serde(skip)]
    #[reflect(ignore)]
    pub id: Uuid,

    /// Human-readable name describing what this parameter represents.
    ///
    /// Should be descriptive and follow consistent naming conventions
    /// within the context of the containing flow.
    pub name: String,

    /// The parameter's value as a string representation.
    ///
    /// Stored as string to accommodate various data types (numeric, text, boolean)
    /// while maintaining flexibility for user input and display.
    pub value: String,

    /// Unit of measurement for the parameter value.
    ///
    /// Should follow standard unit conventions (SI units preferred) to ensure
    /// consistency and enable proper analysis of system interactions.
    #[serde(default = "String::new")]
    pub unit: String,
}

impl Default for Parameter {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_string(),
            value: "".to_string(),
            unit: "".to_string(),
        }
    }
}

/// Corresponds to the System Language Interaction types.
#[derive(
    Copy, Clone, Debug, Reflect, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Sequence,
)]
pub enum InteractionType {
    #[default]
    Flow,
    Force,
}

impl core::fmt::Display for InteractionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InteractionType::Flow => write!(f, "Flow"),
            InteractionType::Force => write!(f, "Force"),
        }
    }
}

/// Attached to entities with a SystemElement::ExternalEntity component to hold modeling data related to the external entity.
#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ExternalEntity {
    pub equivalence: String,
    pub model: String,
}

/// Attached to entities with a SystemElement::ExternalEntity component to hold the source/sink equivalence id.
#[derive(Clone, Copy, Deref, DerefMut, Debug, Component, Reflect, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct IsSameAsId(pub usize);

impl From<usize> for IsSameAsId {
    fn from(value: usize) -> Self {
        Self(value)
    }
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
    /// Creates a new interface subsystem linked to the specified interface entity.
    ///
    /// # Parameters
    ///
    /// - `interface_entity`: The [`Entity`] reference to the parent interface that created this subsystem
    ///
    /// # Returns
    ///
    /// A new `InterfaceSubsystem` instance with default values for flow calculations
    /// and substance type classification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bert::InterfaceSubsystem;
    /// use bevy::prelude::Entity;
    ///
    /// // Create a new interface subsystem
    /// let interface_entity = Entity::from_raw(123); // Example entity
    /// let subsystem = InterfaceSubsystem::new(interface_entity);
    ///
    /// assert_eq!(subsystem.interface_entity, interface_entity);
    /// assert_eq!(subsystem.total_inflow, rust_decimal::Decimal::default());
    /// assert_eq!(subsystem.total_outflow, rust_decimal::Decimal::default());
    /// assert!(!subsystem.is_useful);
    /// ```
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

/// Categorizes flow interactions based on their utility and directional nature in System Language.
///
/// This enumeration implements the System Language classification of interactions
/// based on their value and directional flow relative to system boundaries.
///
/// # System Language Theory
///
/// Interactions are classified along two dimensions:
/// - **Utility**: Whether the interaction provides value (useful) or reduces it (harmful)
/// - **Direction**: Whether the interaction flows into the system (import) or out of it (export)
///
/// This creates four fundamental interaction types that capture all possible
/// system-environment exchanges.
///
/// # Classification Matrix
///
/// | Direction | Useful | Harmful |
/// |-----------|--------|---------|
/// | Import    | Resource | Disruption |
/// | Export    | Product | Waste |
///
/// # Examples
///
/// ```rust
/// use bert::InteractionUsability;
///
/// // Check if an interaction is beneficial to the system
/// let resource = InteractionUsability::Resource;
/// assert!(resource.is_useful());
/// assert!(resource.is_import());
///
/// let waste = InteractionUsability::Waste;
/// assert!(!waste.is_useful());
/// assert!(waste.is_export());
/// ```
///
/// # See Also
///
/// - [`Flow`]: Uses this enum to classify interaction types
/// - [`SubstanceType`]: Defines what flows through the interaction
#[derive(Serialize, Deserialize, Sequence, Copy, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub enum InteractionUsability {
    /// Useful input that enhances system capabilities or provides needed materials/energy.
    ///
    /// Resources are beneficial imports that the system requires for proper functioning.
    /// Examples: raw materials, energy supply, information inputs, human expertise.
    Resource,

    /// Harmful input that degrades system performance or introduces unwanted effects.
    ///
    /// Disruptions are negative imports that interfere with system operations.
    /// Examples: noise, contamination, system attacks, resource shortages.
    Disruption,

    /// Useful output that fulfills the system's intended purpose or provides value.
    ///
    /// Products are beneficial exports that represent successful system transformation.
    /// Examples: manufactured goods, processed information, services, value creation.
    Product,

    /// Harmful output that represents unwanted byproducts or system inefficiency.
    ///
    /// Waste represents negative exports that may require management or disposal.
    /// Examples: pollution, heat loss, defective products, information leakage.
    Waste,
}

impl InteractionUsability {
    /// Determines if this interaction type provides value to the system.
    ///
    /// # Returns
    ///
    /// `true` for [`Resource`](Self::Resource) and [`Product`](Self::Product),
    /// `false` for [`Disruption`](Self::Disruption) and [`Waste`](Self::Waste).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bert::InteractionUsability;
    ///
    /// assert!(InteractionUsability::Resource.is_useful());
    /// assert!(InteractionUsability::Product.is_useful());
    /// assert!(!InteractionUsability::Disruption.is_useful());
    /// assert!(!InteractionUsability::Waste.is_useful());
    /// ```
    #[inline(always)]
    pub fn is_useful(&self) -> bool {
        matches!(self, Self::Resource | Self::Product)
    }

    /// Determines if this interaction flows out of the system (export direction).
    ///
    /// # Returns
    ///
    /// `true` for [`Product`](Self::Product) and [`Waste`](Self::Waste),
    /// `false` for [`Resource`](Self::Resource) and [`Disruption`](Self::Disruption).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bert::InteractionUsability;
    ///
    /// assert!(InteractionUsability::Product.is_export());
    /// assert!(InteractionUsability::Waste.is_export());
    /// assert!(!InteractionUsability::Resource.is_export());
    /// assert!(!InteractionUsability::Disruption.is_export());
    /// ```
    #[inline(always)]
    pub fn is_export(&self) -> bool {
        matches!(self, Self::Product | Self::Waste)
    }

    /// Determines if this interaction flows into the system (import direction).
    ///
    /// # Returns
    ///
    /// `true` for [`Resource`](Self::Resource) and [`Disruption`](Self::Disruption),
    /// `false` for [`Product`](Self::Product) and [`Waste`](Self::Waste).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bert::InteractionUsability;
    ///
    /// assert!(InteractionUsability::Resource.is_import());
    /// assert!(InteractionUsability::Disruption.is_import());
    /// assert!(!InteractionUsability::Product.is_import());
    /// assert!(!InteractionUsability::Waste.is_import());
    /// ```
    #[inline(always)]
    pub fn is_import(&self) -> bool {
        !self.is_export()
    }
}

impl core::fmt::Display for InteractionUsability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InteractionUsability::Resource => write!(f, "Resource"),
            InteractionUsability::Disruption => write!(f, "Disruption"),
            InteractionUsability::Product => write!(f, "Product"),
            InteractionUsability::Waste => write!(f, "Waste"),
        }
    }
}

/// Defines the fundamental types of substances that can flow between system elements.
///
/// The System Language recognizes three primary categories of substances that can
/// be exchanged across system boundaries, each with distinct properties and behaviors.
///
/// # System Language Foundation
///
/// These substance types represent the fundamental forms of exchange in complex systems:
/// - **Energy**: The capacity to do work or create change
/// - **Material**: Physical matter or tangible resources
/// - **Message**: Information, data, or symbolic content
///
/// # Visual Coding
///
/// Each substance type has associated colors for visual distinction in the interface:
/// - Energy: Red tones (representing power and transformation)
/// - Material: Gray tones (representing physical matter)
/// - Message: Light gray tones (representing information flow)
///
/// # Examples
///
/// ```rust
/// use bert::{SubstanceType, Theme};
/// use bevy::prelude::Color;
///
/// // Different substance types for modeling flows
/// let electricity = SubstanceType::Energy;
/// let raw_materials = SubstanceType::Material;
/// let control_signals = SubstanceType::Message;
///
/// // Each type has distinct visual representation
/// assert_ne!(electricity.flow_color(Theme::Normal), raw_materials.flow_color(Theme::Normal));
/// assert_ne!(electricity.interface_color(Theme::Normal), control_signals.interface_color(Theme::Normal));
/// ```
///
/// # See Also
///
/// - [`Flow`]: Uses substance types to categorize interactions
/// - [`InteractionUsability`]: Defines the directional and utility aspects of flows
#[derive(
    Copy, Clone, Debug, Reflect, PartialEq, Eq, Default, Serialize, Deserialize, Sequence, Hash,
)]
pub enum SubstanceType {
    /// Energy flows representing power, work capacity, or transformative potential.
    ///
    /// Examples: electrical power, thermal energy, mechanical work, chemical potential,
    /// kinetic energy, or any form of energy transfer between system components.
    #[default]
    Energy,

    /// Material flows representing physical matter or tangible resources.
    ///
    /// Examples: raw materials, manufactured components, fluids, gases, solid objects,
    /// or any physical substance that moves between system elements.
    Material,

    /// Information flows representing data, signals, or symbolic content.
    ///
    /// Examples: control signals, data transmissions, communication protocols,
    /// sensor readings, commands, or any form of information exchange.
    Message,
}

/// HCGS (Hierarchical Cybernetic Governance System) archetype classification.
///
/// Classifies subsystems according to their functional role in a complex adaptive system,
/// based on the Mobus framework from "Systems Science: Theory, Analysis, Modeling, and Design" (2022).
///
/// # Archetype Definitions
///
/// - **Governance**: Policy, rules, coordination, and control mechanisms
/// - **Economy**: Resource allocation, production, and value flows
/// - **Agent**: Active actors that make decisions and take actions
/// - **Unspecified**: Default classification for subsystems not yet categorized
///
/// # Visual Representation
///
/// Each archetype has a distinct stroke color for visual identification:
/// - Governance: Blue (#3B82F6)
/// - Economy: Green (#22C55E)
/// - Agent: Orange (#F97316)
/// - Unspecified: Black (default)
#[derive(
    Copy, Clone, Debug, Reflect, PartialEq, Eq, Default, Serialize, Deserialize, Sequence, Hash,
)]
pub enum HcgsArchetype {
    /// Default classification for subsystems not yet categorized.
    #[default]
    Unspecified,

    /// Governance subsystems: policy, rules, coordination, control mechanisms.
    ///
    /// Examples: consensus protocols, voting mechanisms, access control,
    /// rule enforcement, coordination layers.
    Governance,

    /// Economy subsystems: resource allocation, production, value flows.
    ///
    /// Examples: token economics, fee markets, resource pools,
    /// transaction processing, value transfer mechanisms.
    Economy,

    /// Agent subsystems: active actors that make decisions and take actions.
    ///
    /// Examples: validators, miners, users, smart contracts,
    /// autonomous processes, decision-making entities.
    Agent,
}

impl HcgsArchetype {
    /// Returns the stroke color for visual representation of this archetype.
    ///
    /// Colors follow Tailwind CSS conventions for consistency:
    /// - Governance: Blue-500 (#3B82F6)
    /// - Economy: Green-500 (#22C55E)
    /// - Agent: Orange-500 (#F97316)
    /// - Unspecified: Black (default system stroke)
    pub fn stroke_color(&self) -> Color {
        match self {
            HcgsArchetype::Unspecified => Color::BLACK,
            HcgsArchetype::Governance => Color::srgb_u8(59, 130, 246), // Blue-500
            HcgsArchetype::Economy => Color::srgb_u8(34, 197, 94),     // Green-500
            HcgsArchetype::Agent => Color::srgb_u8(249, 115, 22),      // Orange-500
        }
    }
}

impl std::fmt::Display for HcgsArchetype {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HcgsArchetype::Unspecified => write!(f, "Unspecified"),
            HcgsArchetype::Governance => write!(f, "Governance"),
            HcgsArchetype::Economy => write!(f, "Economy"),
            HcgsArchetype::Agent => write!(f, "Agent"),
        }
    }
}

impl SubstanceType {
    /// Returns the color used for visual representation of flows of this substance type.
    ///
    /// Uses the original BERT colors regardless of background theme.
    ///
    /// # Returns
    ///
    /// A [`Color`] value appropriate for rendering flow connections in the diagram.
    /// Colors are chosen to provide intuitive visual distinction between substance types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bert::SubstanceType;
    /// use bevy::prelude::Color;
    ///
    /// let energy_color = SubstanceType::Energy.flow_color();
    /// let material_color = SubstanceType::Material.flow_color();
    ///
    /// // Colors are different for each substance type
    /// assert_ne!(energy_color, material_color);
    /// ```
    pub fn flow_color(&self) -> Color {
        match self {
            SubstanceType::Energy => Color::srgb_u8(181, 27, 27), // Deep red for energy
            SubstanceType::Material => Color::srgb(0.5, 0.5, 0.5), // Medium gray for material
            SubstanceType::Message => Color::srgb(0.75, 0.75, 0.75), // Light gray for information
        }
    }

    /// Returns the color used for visual representation of interfaces handling this substance type.
    ///
    /// Interface colors are typically lighter variants of the corresponding flow colors
    /// to provide visual consistency while maintaining distinction between flows and boundaries.
    ///
    /// # Returns
    ///
    /// A [`Color`] value appropriate for rendering interface boundaries in the diagram.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bert::SubstanceType;
    ///
    /// let energy_interface = SubstanceType::Energy.interface_color();
    /// let energy_flow = SubstanceType::Energy.flow_color();
    ///
    /// // Interface colors are related but distinct from flow colors
    /// assert_ne!(energy_interface, energy_flow);
    /// ```
    pub fn interface_color(&self) -> Color {
        match self {
            SubstanceType::Energy => Color::srgb_u8(233, 182, 178), // Light red for energy interfaces
            SubstanceType::Material => Color::srgb(0.5, 0.5, 0.5), // Medium gray for material interfaces
            SubstanceType::Message => Color::srgb(0.75, 0.75, 0.75), // Light gray for message interfaces
        }
    }

    /// Converts the flow color to an RGB string representation for web/CSS usage.
    ///
    /// # Returns
    ///
    /// A string in the format "rgb(r, g, b)" where r, g, b are integers 0-255.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bert::SubstanceType;
    ///
    /// let energy_rgb = SubstanceType::Energy.to_rgb_string();
    /// assert!(energy_rgb.starts_with("rgb("));
    /// assert!(energy_rgb.ends_with(")"));
    /// ```
    pub fn to_rgb_string(&self) -> String {
        let srgb = self.flow_color().to_srgba();
        let r = srgb.red;
        let g = srgb.green;
        let b = srgb.blue;
        format!("rgb({}, {}, {})", r * 255.0, g * 255.0, b * 255.0)
    }

    /// Convenience method for frontend usage - same as to_rgb_string().
    ///
    /// # Returns
    ///
    /// A string in the format "rgb(r, g, b)".
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bert::SubstanceType;
    ///
    /// let energy_rgb = SubstanceType::Energy.to_rgb_string_default();
    /// assert!(energy_rgb.starts_with("rgb("));
    /// ```
    pub fn to_rgb_string_default(&self) -> String {
        self.to_rgb_string()
    }

    /// Convenience method for getting flow color - same as flow_color().
    ///
    /// # Returns
    ///
    /// A [`Color`] value.
    pub fn flow_color_default(&self) -> Color {
        self.flow_color()
    }

    /// Convenience method for getting interface color - same as interface_color().
    ///
    /// # Returns
    ///
    /// A [`Color`] value.
    pub fn interface_color_default(&self) -> Color {
        self.interface_color()
    }
}

impl std::fmt::Display for SubstanceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubstanceType::Energy => write!(f, "Energy"),
            SubstanceType::Material => write!(f, "Material"),
            SubstanceType::Message => write!(f, "Message"),
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
