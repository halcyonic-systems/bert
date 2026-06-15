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
use rust_decimal::Decimal;

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

    /// Optional agent configuration for agent-based modeling.
    /// Only populated when archetype == Agent.
    pub agent: Option<crate::bevy_app::data_model::AgentModel>,
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

// ── Kernel contract types, extracted to bert-core ────────────────────────────
// The data enums below are part of the serialized model contract and now live
// in the bert-core kernel crate. Re-exported here so existing paths
// (`components::SubstanceType`, glob imports) keep working unchanged.
pub use bert_core::{
    HcgsArchetype, InteractionType, InteractionUsability, Parameter, SubstanceType,
};

/// Bevy-coupled view helpers for the kernel's [`SubstanceType`] — inherent
/// methods can't live outside the defining crate, so they're an extension
/// trait here. In scope wherever `components::*` is glob-imported.
pub trait SubstanceTypeExt {
    fn flow_color(&self) -> Color;
    fn interface_color(&self) -> Color;
    fn to_rgb_string(&self) -> String;
    fn to_rgb_string_default(&self) -> String;
    fn flow_color_default(&self) -> Color;
    fn interface_color_default(&self) -> Color;
}

impl SubstanceTypeExt for SubstanceType {
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
    fn flow_color(&self) -> Color {
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
    fn interface_color(&self) -> Color {
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
    fn to_rgb_string(&self) -> String {
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
    fn to_rgb_string_default(&self) -> String {
        self.to_rgb_string()
    }

    /// Convenience method for getting flow color - same as flow_color().
    ///
    /// # Returns
    ///
    /// A [`Color`] value.
    fn flow_color_default(&self) -> Color {
        self.flow_color()
    }

    /// Convenience method for getting interface color - same as interface_color().
    ///
    /// # Returns
    ///
    /// A [`Color`] value.
    fn interface_color_default(&self) -> Color {
        self.interface_color()
    }
}

/// Bevy-coupled view helper for the kernel's [`HcgsArchetype`].
pub trait HcgsArchetypeExt {
    fn stroke_color(&self) -> Color;
}

impl HcgsArchetypeExt for HcgsArchetype {
    fn stroke_color(&self) -> Color {
        match self {
            HcgsArchetype::Unspecified => Color::BLACK,
            HcgsArchetype::Governance => Color::srgb_u8(59, 130, 246), // Blue-500
            HcgsArchetype::Economy => Color::srgb_u8(34, 197, 94),     // Green-500
            HcgsArchetype::Agent => Color::srgb_u8(249, 115, 22),      // Orange-500
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

/// Stores the original ID from the loaded file to preserve ID stability across save/load cycles.
///
/// When an entity is loaded from a file, its original ID is stored in this component.
/// During save, entities with this component use their original ID instead of generating
/// a new one, ensuring that IDs remain stable across sessions.
///
/// Entities created during runtime (not loaded from file) won't have this component,
/// so they'll get new sequential IDs as expected.
#[derive(Clone, Debug, Component)]
pub struct OriginalId(pub crate::bevy_app::data_model::Id);

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
