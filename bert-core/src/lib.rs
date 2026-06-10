//! bert-core — BERT's System Language kernel.
//!
//! The serialized model contract (`WorldModel` + the 8-tuple element types),
//! the contract enums, smart-parameter data types, and the 4-layer validator —
//! extracted from the app so every surface (Tauri editor, bert-typedb
//! transpiler, bert-compose, generators) builds on one kernel without pulling
//! Bevy/Leptos/Tauri.
//!
//! The `reflect` feature gates `bevy_reflect::Reflect` derives for the app,
//! whose ECS components embed kernel types ([`Complexity`], [`AgentModel`]).

pub mod validate;

use enum_iterator::Sequence;
use std::fmt::Formatter;
use uuid::Uuid;

/// Corresponds to the System Language Interaction types.
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Sequence,
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
/// ```rust,ignore
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
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Serialize, Deserialize, Sequence, Copy, Clone, Eq, PartialEq, Debug, Hash)]
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
    /// ```rust,ignore
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
    /// ```rust,ignore
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
    /// ```rust,ignore
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
/// ```rust,ignore
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
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, Sequence, Hash,
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
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, Sequence, Hash,
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


/// Enhanced parameter value supporting multiple data types (Cliff's framework)
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterValue {
    /// Numeric parameter with unit (traditional quantified measurements)
    Numeric { value: String, unit: String },
    /// Ordinal parameter with total ordering (high/medium/low)
    Ordinal { level: String, options: Vec<String> },
    /// Categorical parameter with discrete options (solid/liquid/gas)
    Categorical { value: String, options: Vec<String> },
    /// Boolean parameter with custom labels (active/inactive)
    Boolean {
        value: bool,
        true_label: String,
        false_label: String,
    },
}


/// Smart parameter with enhanced type system
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmartParameter {
    /// Unique identifier for this parameter (excluded from serialization)
    #[serde(skip)]
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    pub id: Uuid,
    /// Human-readable parameter name
    pub name: String,
    /// Parameter value with type information
    pub value: ParameterValue,
}


/// Parameter suggestion for autocomplete and intelligent input
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterSuggestion {
    /// Display name shown to user
    pub display_name: String,
    /// Parameter type for this suggestion
    pub parameter_type: ParameterType,
    /// Search terms for fuzzy matching
    pub search_terms: Vec<String>,
    /// Default parameter value template
    pub default_value: ParameterValue,
}


/// Parameter type classification
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    Numeric,
    Ordinal,
    Categorical,
    Boolean,
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
/// ```rust,ignore
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
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    /// Unique identifier for this parameter (excluded from serialization).
    ///
    /// Automatically generated to distinguish parameters even when they have
    /// identical names or values. Used internally for parameter management.
    #[serde(skip)]
    #[cfg_attr(feature = "reflect", reflect(ignore))]
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

impl SmartParameter {
    /// Create new smart parameter with generated ID
    pub fn new(name: String, value: ParameterValue) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            value,
        }
    }

    /// Get parameter type from value
    pub fn parameter_type(&self) -> ParameterType {
        match &self.value {
            ParameterValue::Numeric { .. } => ParameterType::Numeric,
            ParameterValue::Ordinal { .. } => ParameterType::Ordinal,
            ParameterValue::Categorical { .. } => ParameterType::Categorical,
            ParameterValue::Boolean { .. } => ParameterType::Boolean,
        }
    }
}





// # Data Model Module
//
// This module implements the persistence layer for BERT system models, providing
// serialization and deserialization capabilities for the System Language framework.
//
// ## Architecture
//
// The data model implements Layer 3 (Knowledge Representation) of the System Language
// by providing structured data formats that can be:
//
// - **Serialized**: Convert live system models to JSON for storage
// - **Deserialized**: Reconstruct system models from saved JSON data
// - **Versioned**: Handle evolution of the data format over time
// - **Validated**: Ensure data integrity and consistency
//
// ## Key Components
//
// - [`WorldModel`]: Root container for complete system models
// - [`Id`]: Hierarchical identification system for all entities
// - [`System`]: Serializable representation of system entities
// - [`Interaction`]: Serializable representation of flows between systems
// - [`Environment`]: Container for external entities and root system context
//
// ## Data Format
//
// The module uses JSON as the primary serialization format with a versioned schema
// to support backward compatibility and data migration. The hierarchical ID system
// ensures proper reconstruction of system relationships during deserialization.
//
// ## Usage Patterns
//
// ```rust,ignore
// use bert::data_model::{WorldModel, System, Environment};
//
// // Serialize a complete world model
// let world_model = WorldModel {
//     version: CURRENT_FILE_VERSION,
//     environment: Environment::default(),
//     systems: vec![],
//     interactions: vec![],
//     hidden_entities: vec![],
// };
//
// let json = serde_json::to_string(&world_model)?;
// ```
//
// ## Version Management
//
// The data format uses semantic versioning to handle schema evolution:
// - Version increments trigger data migration logic
// - Backward compatibility is maintained where possible
// - Breaking changes are clearly documented
//
// ## See Also
//
// - [`load`]: Module for deserializing world models from JSON
// - [`save`]: Module for serializing world models to JSON
// - [`crate::bevy_app::components`]: Live ECS components that this module serializes


use glam::Vec2;
use rust_decimal::Decimal;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;

/// Current version of the data format schema.
///
/// This version number is incremented whenever breaking changes are made to the
/// serialization format. It enables data migration and backward compatibility
/// handling when loading older saved files.
///
/// # Version History
///
/// - **Version 1**: Initial data format with basic system modeling support
///
/// # Usage
///
/// Always use this constant when creating new [`WorldModel`] instances to ensure
/// compatibility with the current format expectations.
pub const CURRENT_FILE_VERSION: u32 = 1;

/// Root container representing a complete BERT system model for serialization.
///
/// `WorldModel` serves as the top-level data structure that contains all information
/// necessary to fully reconstruct a system model, including the environment context,
/// all system entities, their interactions, and visualization state.
///
/// # Data Model Structure
///
/// The world model follows a hierarchical structure:
/// ```text
/// WorldModel
/// ├── Environment (external context)
/// │   ├── Sources (external inputs)
/// │   └── Sinks (external outputs)
/// ├── Systems (hierarchical system tree)
/// │   ├── Root System
/// │   └── Nested Subsystems (any depth)
/// ├── Interactions (flows between all entities)
/// └── Hidden Entities (visualization state)
/// ```
///
/// # Versioning Strategy
///
/// The version field enables format evolution while maintaining backward compatibility:
/// - Version mismatches trigger migration logic during deserialization
/// - New versions can add optional fields with serde defaults
/// - Breaking changes require version increment and migration code
///
/// # Examples
///
/// Creating a new world model:
/// ```rust,ignore
/// use bert::data_model::{WorldModel, Environment, CURRENT_FILE_VERSION};
///
/// let world_model = WorldModel {
///     version: CURRENT_FILE_VERSION,
///     environment: Environment {
///         info: Info::default(),
///         sources: vec![],
///         sinks: vec![],
///     },
///     systems: vec![],
///     interactions: vec![],
///     hidden_entities: vec![],
/// };
/// ```
///
/// Serializing to JSON:
/// ```rust,ignore
/// let json = serde_json::to_string_pretty(&world_model)?;
/// std::fs::write("model.json", json)?;
/// ```
///
/// # See Also
///
/// - [`Environment`]: Contains external entities and root system context
/// - [`System`]: Individual system entities within the model
/// - [`Interaction`]: Flow connections between system entities
/// - [`CURRENT_FILE_VERSION`]: Current schema version constant
#[derive(Serialize, Deserialize, Clone)]
pub struct WorldModel {
    /// Schema version for format compatibility and migration support.
    ///
    /// Must match [`CURRENT_FILE_VERSION`] for newly created models.
    /// Older versions trigger migration logic during deserialization.
    pub version: u32,

    /// Environmental context containing external entities and root system information.
    ///
    /// The environment represents the broader context in which the system
    /// of interest operates, including all external sources and sinks.
    pub environment: Environment,

    /// Complete collection of all system entities in the model.
    ///
    /// Includes the root system and all nested subsystems at any depth.
    /// Systems are stored in a flat list with hierarchical relationships
    /// maintained through parent ID references.
    pub systems: Vec<System>,

    /// All flow interactions between system entities and external entities.
    ///
    /// Captures both internal flows (between systems) and external flows
    /// (between systems and environment sources/sinks).
    pub interactions: Vec<Interaction>,

    /// List of entity IDs that are currently hidden in the visualization.
    ///
    /// Preserves the user's view state when saving and loading models,
    /// allowing users to maintain their preferred level of detail.
    #[serde(default)]
    pub hidden_entities: Vec<Id>,
}

/// Hierarchical identifier system for all entities in the BERT data model.
///
/// The `Id` structure provides a unique, hierarchical addressing scheme that encodes
/// both the entity type and its position within the system hierarchy. This enables
/// efficient serialization while preserving all relationship information needed
/// for accurate reconstruction.
///
/// # Hierarchical Addressing Scheme
///
/// The ID system uses a path-based approach where each entity's position in the
/// system hierarchy is encoded as a sequence of indices:
///
/// ```text
/// Root System:     [0]           (system 0)
/// Subsystem:       [0, 1]        (system 1, child of system 0)
/// Deep Subsystem:  [0, 1, 2]     (system 2, child of system 1, grandchild of system 0)
/// Interface:       [0, 1, 3]     (interface 3, belonging to system [0, 1])
/// Environment:     [-1]          (special case for environment)
/// Env Children:    [-1, 0]       (child 0 of environment)
/// ```
///
/// # ID Construction Rules
///
/// - **Last index**: Serial number for this entity type within its parent
/// - **Previous indices**: Path from root to parent system
/// - **Environment exception**: Uses -1 as its identifier
/// - **Type safety**: Each ID includes the entity type for validation
///
/// # Examples
///
/// ```rust,ignore
/// use bert::data_model::{Id, IdType};
///
/// // Root system identifier
/// let root_system = Id {
///     ty: IdType::System,
///     indices: vec![0],
/// };
///
/// // Subsystem within the root system
/// let subsystem = Id {
///     ty: IdType::Subsystem,
///     indices: vec![0, 1],  // system 1, child of system 0
/// };
///
/// // Interface belonging to the subsystem
/// let interface = Id {
///     ty: IdType::Interface,
///     indices: vec![0, 1, 0],  // interface 0 of system [0, 1]
/// };
/// ```
///
/// # Serialization Format
///
/// IDs are serialized as strings combining type prefix with dot-separated indices:
/// - `"S0"` → System with indices [0]
/// - `"C0.1"` → Subsystem with indices [0, 1]  
/// - `"I0.1.0"` → Interface with indices [0, 1, 0]
///
/// # See Also
///
/// - [`IdType`]: Enumeration of all entity types that can be identified
/// - [`HasInfo`]: Trait for entities that contain ID information
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id {
    /// Entity type classification for type safety and serialization.
    ///
    /// Determines how the ID should be interpreted and which kind of
    /// entity it references in the data model.
    pub ty: IdType,

    /// Hierarchical path from root to this entity.
    ///
    /// The sequence of indices that uniquely identifies this entity's position
    /// within the system hierarchy. The last index is the entity's serial number
    /// within its parent, while previous indices trace the path from root.
    ///
    /// # Special Cases
    ///
    /// - **Environment**: Uses [-1] as a special identifier
    /// - **Environment children**: Start with [-1, ...] prefix
    /// - **Root system**: Uses [0] as the foundation of all hierarchies
    ///
    /// # Examples
    ///
    /// - `[0]`: Root system
    /// - `[0, 1, 5]`: Entity 5 of its type, child of system 1, grandchild of root system 0
    /// - `[-1, 0]`: First child of the environment
    pub indices: Vec<i64>,
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut str_value = serde_json::to_string(&self.ty).expect("This shouldn't fail");
        str_value = str_value[1..str_value.len() - 1].to_string();

        str_value.push_str(
            &self
                .indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join("."),
        );

        serializer.serialize_str(&str_value)
    }
}

struct IdVisitor;

impl<'de> Visitor<'de> for IdVisitor {
    type Value = Id;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string with format <type><index1>.<index2>...")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Some(index) = v.find(|c: char| c.is_numeric() || c == '-') {
            let ty = serde_json::from_str(&format!("\"{}\"", &v[..index]))
                .map_err(|err| E::custom(format!("Error parsing type prefix: {err:?}")))?;

            let indices = v[index..]
                .split(".")
                .map(|i| i.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| E::custom(format!("Error parsing indices: {err:?}")))?;

            Ok(Id { ty, indices })
        } else {
            Err(E::custom("Didn't find any index"))
        }
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IdVisitor)
    }
}

/// Enumeration of all entity types that can be uniquely identified in the data model.
///
/// `IdType` provides type safety for the hierarchical ID system by ensuring that
/// each ID is properly classified according to the kind of entity it references.
/// This prevents type confusion during serialization and reconstruction.
///
/// # Serialization Mapping
///
/// Each variant uses a compact string representation for efficient JSON serialization:
///
/// | Variant | JSON | Description |
/// |---------|------|-------------|
/// | `System` | `"S"` | Root or parent system entities |
/// | `Subsystem` | `"C"` | Nested system components |
/// | `Interface` | `"I"` | System boundary interfaces |
/// | `Source` | `"Src"` | External input entities |
/// | `Sink` | `"Snk"` | External output entities |
/// | `Environment` | `"E"` | Environmental context |
/// | `Flow` | `"F"` | Interaction connections |
/// | `Boundary` | `"B"` | System boundary definitions |
///
/// # Usage in ID Construction
///
/// ```rust,ignore
/// use bert::data_model::{Id, IdType};
///
/// // Create IDs for different entity types
/// let system_id = Id { ty: IdType::System, indices: vec![0] };
/// let interface_id = Id { ty: IdType::Interface, indices: vec![0, 1] };
/// let flow_id = Id { ty: IdType::Flow, indices: vec![0, 0] };
/// ```
///
/// # See Also
///
/// - [`Id`]: The hierarchical identifier structure that uses these types
/// - [`HasInfo`]: Trait for entities that contain typed ID information
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IdType {
    /// Root system or primary system entity.
    ///
    /// Represents the main bounded entities in the system hierarchy.
    /// Systems can contain subsystems, interfaces, and interact with external entities.
    #[serde(rename = "S")]
    System,

    /// Nested system component within a parent system.
    ///
    /// Subsystems represent decomposed parts of complex systems, enabling
    /// hierarchical modeling and analysis at multiple levels of detail.
    #[serde(rename = "C")]
    Subsystem,

    /// System boundary interface for interaction points.
    ///
    /// Interfaces define formal connection points where flows can enter
    /// or exit system boundaries, implementing boundary management.
    #[serde(rename = "I")]
    Interface,

    /// External source entity providing inputs to the system.
    ///
    /// Sources represent environmental entities that supply resources,
    /// energy, materials, or information to the system of interest.
    #[serde(rename = "Src")]
    Source,

    /// External sink entity receiving outputs from the system.
    ///
    /// Sinks represent environmental entities that consume products,
    /// waste, or other outputs produced by the system of interest.
    #[serde(rename = "Snk")]
    Sink,

    /// Environmental context container for external entities.
    ///
    /// The environment represents the broader context in which the
    /// system operates, containing all external sources and sinks.
    #[serde(rename = "E")]
    Environment,

    /// Flow interaction between system entities.
    ///
    /// Flows represent the dynamic exchange of substances (energy,
    /// material, information) between systems and external entities.
    #[serde(rename = "F")]
    Flow,

    /// System boundary definition and properties.
    ///
    /// Boundaries define the formal separation between systems and
    /// their environment, controlling interaction capabilities.
    #[serde(rename = "B")]
    Boundary,
}

/// Common metadata shared by most serializable entities in the data model.
///
/// `Info` provides standard identification and descriptive information that
/// is consistent across different entity types. This enables uniform handling
/// of entity metadata during serialization, search, and reconstruction.
///
/// # Level Calculation Rules
///
/// The level field represents hierarchical depth with specific rules:
/// - **Normal entities**: `level = id.indices.len() - 1`
/// - **Environment**: Always level -1 (special case)
/// - **Environment children**: Also level -1 (inherited from environment)
/// - **Root system**: Level 0 (foundation of hierarchy)
/// - **Subsystems**: Incremental depth (1, 2, 3, ...)
///
/// # Examples
///
/// ```rust,ignore
/// use bert::data_model::{Info, Id, IdType};
///
/// // Root system info
/// let root_info = Info {
///     id: Id { ty: IdType::System, indices: vec![0] },
///     level: 0,  // Root level
///     name: "Manufacturing System".to_string(),
///     description: "Complete manufacturing process".to_string(),
/// };
///
/// // Subsystem info
/// let subsystem_info = Info {
///     id: Id { ty: IdType::Subsystem, indices: vec![0, 1] },
///     level: 1,  // One level below root
///     name: "Assembly Line".to_string(),
///     description: "Product assembly subsystem".to_string(),
/// };
/// ```
///
/// # Usage Patterns
///
/// Most entities in the data model contain an `Info` struct to provide:
/// - Unique identification through the ID system
/// - Hierarchical positioning through level tracking
/// - Human-readable naming and documentation
///
/// # See Also
///
/// - [`Id`]: The hierarchical identifier system
/// - [`HasInfo`]: Trait implemented by entities containing this information
#[derive(Serialize, Deserialize, Clone)]
pub struct Info {
    /// Hierarchical identifier uniquely identifying this entity.
    ///
    /// Provides both type information and positional data within
    /// the system hierarchy for accurate reconstruction.
    pub id: Id,

    /// Hierarchical depth level of this entity within the system tree.
    ///
    /// Represents how many levels deep this entity is nested, with special
    /// handling for environment entities which use level -1.
    ///
    /// # Level Examples
    /// - `-1`: Environment and its direct children
    /// - `0`: Root system
    /// - `1`: First-level subsystems
    /// - `2+`: Deeper nested subsystems
    pub level: i32,

    /// Human-readable name for this entity.
    ///
    /// Should be descriptive and meaningful within the context of the
    /// system model, enabling easy identification and navigation.
    pub name: String,

    /// Detailed description of this entity's purpose and characteristics.
    ///
    /// Provides additional context and documentation that helps users
    /// understand the entity's role within the broader system model.
    pub description: String,
}

/// A system. Either root or subsystem.
#[derive(Serialize, Deserialize, Clone)]
pub struct System {
    pub info: Info,
    /// All sources contained inside this system
    pub sources: Vec<ExternalEntity>,
    /// All sinks contained inside this system
    pub sinks: Vec<ExternalEntity>,
    /// Id of the parent system or the environment if this is the root system.
    pub parent: Id,
    pub complexity: Complexity,
    pub boundary: Boundary,
    /// Radius in pixels when not zoom is 100%
    pub radius: f32,
    pub transform: Option<Transform2d>,
    pub equivalence: String,
    pub history: String,
    pub transformation: String,
    pub member_autonomy: f32,
    pub time_constant: String,
    /// HCGS archetype classification (Governance, Economy, Agent).
    /// None = Unspecified (backward compatible with older models).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype: Option<HcgsArchetype>,
    /// Agent configuration — only present when archetype == Agent.
    /// Contains behavioral parameters for ABM export and agency properties.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentModel>,
}

/// Agent behavioral model for ABM export and agency properties.
/// Only populated when archetype == Agent.
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AgentModel {
    /// Mobus agent hierarchy classification (Reactive/Anticipatory/Intentional)
    pub kind: AgentKind,

    /// Degree of autonomous decision-making capability (0.0 to 1.0)
    /// 0.0 = fully reactive/directed, 0.5 = semi-autonomous, 1.0 = fully autonomous
    #[serde(default = "default_agency_capacity")]
    pub agency_capacity: f32,

    /// Atomic work processes this agent can perform (Mobus primitives)
    #[serde(default)]
    pub primitives: Vec<ProcessPrimitive>,

    /// Domain-agnostic cognitive parameters (e.g., "fee_threshold": 50.0)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub cognitive_params: HashMap<String, f64>,

    /// Process behavior configurations with flexible parameters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process_configs: Vec<ProcessAssignment>,

    /// Initial state for agent instantiation as arbitrary JSON values
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    pub initial_state: HashMap<String, serde_json::Value>,

    /// Optional network behavior configuration for multi-agent interactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_config: Option<NetworkConfig>,
}

fn default_agency_capacity() -> f32 {
    0.5
}

impl Default for AgentModel {
    fn default() -> Self {
        Self {
            kind: AgentKind::default(),
            agency_capacity: default_agency_capacity(),
            primitives: Vec::new(),
            cognitive_params: HashMap::new(),
            process_configs: Vec::new(),
            initial_state: HashMap::new(),
            network_config: None,
        }
    }
}

/// Mobus agent hierarchy classification.
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum AgentKind {
    #[default]
    Reactive,
    Anticipatory,
    Intentional,
}

/// Mobus atomic work process primitives.
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ProcessPrimitive {
    Combining,
    Splitting,
    Buffering,
    Impeding,
    Propelling,
    Copying,
    Sensing,
    Modulating,
    Amplifying,
    Inverting,
}

/// Process behavior configuration with flexible parameters.
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProcessAssignment {
    pub name: String,
    #[serde(default)]
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    pub params: HashMap<String, serde_json::Value>,
}

/// Network behavior configuration for multi-agent interactions.
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NetworkConfig {
    pub topology: String,
    #[serde(default)]
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    pub connection_params: HashMap<String, serde_json::Value>,
    #[serde(default)]
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    pub interaction_rules: HashMap<String, serde_json::Value>,
}

/// Boundary of a system.
#[derive(Serialize, Deserialize, Clone)]
pub struct Boundary {
    pub info: Info,
    pub porosity: f32,
    pub perceptive_fuzziness: f32,
    /// List of all interfaces that are a direct part of this system
    pub interfaces: Vec<Interface>,
    /// In case this is an interface subsystem then this holds the id of that parent subsytem.
    /// This interface is not contained in the field `interfaces`.
    pub parent_interface: Option<Id>,
}

/// Interface of a system
#[derive(Serialize, Deserialize, Clone)]
pub struct Interface {
    pub info: Info,
    pub protocol: String,
    #[serde(rename = "type")]
    pub ty: InterfaceType,
    /// Ids of targets that are connected through interactions from this interface. Can be either a
    /// sink or another subsystem
    pub exports_to: Vec<Id>,
    /// Ids of origins that are connected through interactions with this interface as target.
    /// Can be either a source or another subsystem.
    pub receives_from: Vec<Id>,
    /// Rotation in radians.
    pub angle: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum InterfaceType {
    /// Interface contains only outgoing interactions
    Export,
    /// Interface contains only incoming interactions
    Import,
    /// Interface contains both incoming and outgoing interactions. This is not implemented yet.
    Hybrid,
}


/// Environment of the root system
#[derive(Serialize, Deserialize, Clone)]
pub struct Environment {
    pub info: Info,
    /// All external sources
    pub sources: Vec<ExternalEntity>,
    /// All external sinks
    pub sinks: Vec<ExternalEntity>,
}

/// Source or sink
#[derive(Serialize, Deserialize, Clone)]
pub struct ExternalEntity {
    pub info: Info,
    #[serde(rename = "type")]
    pub ty: ExternalEntityType,
    pub transform: Option<Transform2d>,
    pub equivalence: String,
    pub model: String,
    #[serde(default)]
    pub is_same_as_id: Option<usize>,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum ExternalEntityType {
    Source,
    Sink,
}

/// Interaction between objects. One end is always a system. The other can be a system as well
/// or an external entity.
#[derive(Serialize, Deserialize, Clone)]
pub struct Interaction {
    pub info: Info,
    pub substance: Substance,
    #[serde(rename = "type")]
    pub ty: InteractionType,
    pub usability: InteractionUsability,
    /// Start of the connection. Can be either a system or a source.
    pub source: Id,
    /// If the source is a system, then this holds the id to the interface where this connection
    /// starts from.
    pub source_interface: Option<Id>,
    /// End of the connection. Can be either a system or a sink.
    pub sink: Id,
    /// If the sink is a system, then this holds the id to the interface where this connection
    /// ends at.
    pub sink_interface: Option<Id>,
    pub amount: Decimal,
    pub unit: String,
    /// List of additional parameters
    pub parameters: Vec<Parameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub smart_parameters: Vec<SmartParameter>,
    /// User-defined endpoint offsets for flow positioning.
    /// Only serialized when non-zero to maintain backward compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_offset: Option<EndpointOffset>,
}

/// Serializable representation of flow endpoint offsets.
/// Uses angles (radians) for zoom-independent positioning.
#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, Debug)]
pub struct EndpointOffset {
    /// Angular position (radians) for start endpoint on its subsystem boundary
    pub start_angle: Option<f32>,
    /// Angular position (radians) for end endpoint on its subsystem boundary
    pub end_angle: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Substance {
    pub sub_type: String,
    #[serde(rename = "type")]
    pub ty: SubstanceType,
}

/// Defines the complexity classification of system entities based on their internal structure.
///
/// The `Complexity` enum implements the System Language framework's approach to
/// categorizing systems according to their internal composition and behavioral
/// characteristics. This classification affects how systems are modeled, visualized,
/// and analyzed within BERT.
///
/// # System Language Theory
///
/// Complexity in systems theory represents different modes of organization:
/// - **Atomic**: Indivisible entities with no internal structure
/// - **Complex**: Entities with internal components and emergent properties
/// - **Multiset**: Collections of identical components with scaling behavior
///
/// # Behavioral Properties
///
/// Complex systems can exhibit two key evolutionary characteristics:
/// - **Adaptability**: Can modify behavior in response to environmental changes
/// - **Evolvability**: Can develop new capabilities or structures over time
///
/// # Examples
///
/// ```rust,ignore
/// use bert::data_model::Complexity;
///
/// // Simple atomic system (no internal structure)
/// let sensor = Complexity::Atomic;
///
/// // Complex adaptive system
/// let adaptive_system = Complexity::Complex {
///     adaptable: true,
///     evolveable: false,
/// };
///
/// // Collection of identical processing units
/// let server_farm = Complexity::Multiset(50);
///
/// // Check system characteristics
/// assert!(adaptive_system.is_complex());
/// assert!(adaptive_system.is_adaptable());
/// assert!(!adaptive_system.is_evolveable());
/// ```
///
/// # See Also
///
/// - [`System`]: Uses complexity to define system behavior
/// - [`crate::bevy_app::components::System`]: Runtime system component that includes complexity
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum Complexity {
    /// Complex system containing internal components with emergent properties.
    ///
    /// Represents systems that have decomposable internal structure and can
    /// exhibit sophisticated behaviors through component interactions.
    ///
    /// # Properties
    /// - `adaptable`: Can modify behavior in response to environmental changes
    /// - `evolveable`: Can develop new capabilities or structures over time
    Complex {
        /// Whether the system can adapt its behavior to environmental changes.
        adaptable: bool,
        /// Whether the system can evolve new capabilities or structures.
        evolveable: bool,
    },

    /// Atomic system with no decomposable internal structure.
    ///
    /// Represents indivisible entities that are treated as single units
    /// without internal modeling. These systems exhibit simple, predictable behaviors.
    Atomic,

    /// Multiset system containing multiple instances of identical components.
    ///
    /// Represents bounded collections where the count of identical components
    /// determines the system's capacity or capability. The number indicates
    /// the current or maximum count of component instances.
    ///
    /// # Examples
    /// - Server clusters with N identical nodes
    /// - Production lines with N identical workstations
    /// - Resource pools with N identical units
    Multiset(
        /// Number of identical component instances in the multiset.
        u64,
    ),
}

impl Complexity {
    /// Determines if this complexity represents a complex system with internal structure.
    ///
    /// # Returns
    ///
    /// `true` only for [`Complex`](Self::Complex) variants, `false` for atomic and multiset systems.
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::Complexity;
    ///
    /// let complex = Complexity::Complex { adaptable: true, evolveable: false };
    /// let atomic = Complexity::Atomic;
    /// let multiset = Complexity::Multiset(10);
    ///
    /// assert!(complex.is_complex());
    /// assert!(!atomic.is_complex());
    /// assert!(!multiset.is_complex());
    /// ```
    pub fn is_complex(&self) -> bool {
        matches!(self, Complexity::Complex { .. })
    }

    /// Determines if this complexity represents an atomic (indivisible) system.
    ///
    /// # Returns
    ///
    /// `true` only for [`Atomic`](Self::Atomic) variants.
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::Complexity;
    ///
    /// assert!(Complexity::Atomic.is_atomic());
    /// assert!(!Complexity::Complex { adaptable: false, evolveable: false }.is_atomic());
    /// assert!(!Complexity::Multiset(5).is_atomic());
    /// ```
    pub fn is_atomic(&self) -> bool {
        matches!(self, Complexity::Atomic)
    }

    /// Determines if this complexity represents a multiset system.
    ///
    /// # Returns
    ///
    /// `true` only for [`Multiset`](Self::Multiset) variants.
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::Complexity;
    ///
    /// assert!(Complexity::Multiset(100).is_multiset());
    /// assert!(!Complexity::Atomic.is_multiset());
    /// assert!(!Complexity::Complex { adaptable: true, evolveable: true }.is_multiset());
    /// ```
    pub fn is_multiset(&self) -> bool {
        matches!(self, Complexity::Multiset(_))
    }

    /// Determines if this system can adapt its behavior to environmental changes.
    ///
    /// # Returns
    ///
    /// `true` only for [`Complex`](Self::Complex) systems with `adaptable: true`.
    /// All other complexity types return `false`.
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::Complexity;
    ///
    /// let adaptive = Complexity::Complex { adaptable: true, evolveable: false };
    /// let rigid = Complexity::Complex { adaptable: false, evolveable: true };
    ///
    /// assert!(adaptive.is_adaptable());
    /// assert!(!rigid.is_adaptable());
    /// assert!(!Complexity::Atomic.is_adaptable());
    /// ```
    pub fn is_adaptable(&self) -> bool {
        match self {
            Complexity::Complex { adaptable, .. } => *adaptable,
            _ => false,
        }
    }

    /// Determines if this system can evolve new capabilities or structures over time.
    ///
    /// # Returns
    ///
    /// `true` only for [`Complex`](Self::Complex) systems with `evolveable: true`.
    /// All other complexity types return `false`.
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::Complexity;
    ///
    /// let evolveable = Complexity::Complex { adaptable: false, evolveable: true };
    /// let static_system = Complexity::Complex { adaptable: true, evolveable: false };
    ///
    /// assert!(evolveable.is_evolveable());
    /// assert!(!static_system.is_evolveable());
    /// assert!(!Complexity::Atomic.is_evolveable());
    /// ```
    pub fn is_evolveable(&self) -> bool {
        match self {
            Complexity::Complex { evolveable, .. } => *evolveable,
            _ => false,
        }
    }

    /// Sets the adaptability property for complex systems.
    ///
    /// # Parameters
    ///
    /// - `adapt`: New adaptability setting
    ///
    ///
    ///
    ///
    /// # Behavior
    ///
    /// Only affects [`Complex`](Self::Complex) variants. Has no effect on
    /// [`Atomic`](Self::Atomic) or [`Multiset`](Self::Multiset) systems.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::Complexity;
    ///
    /// let mut system = Complexity::Complex { adaptable: false, evolveable: true };
    /// system.set_adaptable(true);
    /// assert!(system.is_adaptable());
    ///
    /// let mut atomic = Complexity::Atomic;
    /// atomic.set_adaptable(true);  // No effect
    /// assert!(!atomic.is_adaptable());
    /// ```
    pub fn set_adaptable(&mut self, adapt: bool) {
        if let Complexity::Complex { adaptable, .. } = self {
            *adaptable = adapt
        }
    }

    /// Sets the evolvability property for complex systems.
    ///
    /// # Parameters
    ///
    /// - `evolve`: New evolvability setting
    ///
    ///
    ///
    ///
    /// # Behavior
    ///
    /// Only affects [`Complex`](Self::Complex) variants. Has no effect on
    /// [`Atomic`](Self::Atomic) or [`Multiset`](Self::Multiset) systems.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::Complexity;
    ///
    /// let mut system = Complexity::Complex { adaptable: true, evolveable: false };
    /// system.set_evolveable(true);
    /// assert!(system.is_evolveable());
    ///
    /// let mut multiset = Complexity::Multiset(10);
    /// multiset.set_evolveable(true);  // No effect
    /// assert!(!multiset.is_evolveable());
    /// ```
    pub fn set_evolveable(&mut self, evolve: bool) {
        if let Complexity::Complex { evolveable, .. } = self {
            *evolveable = evolve
        }
    }
}

impl std::fmt::Display for Complexity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type T = Complexity;
        match self {
            T::Complex { .. } => write!(f, "Complex"),
            T::Atomic => write!(f, "Atomic"),
            T::Multiset(_) => write!(f, "Multiset"),
        }
    }
}

impl Default for Complexity {
    fn default() -> Self {
        Complexity::Complex {
            adaptable: false,
            evolveable: false,
        }
    }
}

/// Position and rotation of the object
#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, Debug)]
pub struct Transform2d {
    /// position of the object relative to it's parent (as defined by the bevy scene graph, not
    /// by this data model). This is in pixels if zoom is at 100%.
    pub translation: Vec2,
    /// Rotation in radians.
    pub rotation: f32,
}



/// Trait for entities that contain standard identification and metadata information.
///
/// `HasInfo` provides a consistent interface for accessing entity metadata across
/// different types in the data model. This enables generic operations on entities
/// regardless of their specific type.
///
/// # Usage Patterns
///
/// The trait is typically used for:
/// - Generic entity processing during serialization/deserialization
/// - Search and filtering operations across entity types
/// - Hierarchical navigation and relationship building
/// - Consistent metadata access patterns
///
/// # Examples
///
/// ```rust,ignore
/// use bert::data_model::{HasInfo, System, Environment};
///
/// fn print_entity_info<T: HasInfo>(entity: &T) {
///     let info = entity.info();
///     println!("Entity: {} (Level {})", info.name, info.level);
/// }
///
/// // Works with any entity that implements HasInfo
/// let system = System { /* ... */ };
/// let environment = Environment { /* ... */ };
///
/// print_entity_info(&system);
/// print_entity_info(&environment);
/// ```
///
/// # Implementing Types
///
/// The following data model types implement `HasInfo`:
/// - [`System`]: System entities at all hierarchical levels
/// - [`Boundary`]: System boundary definitions
/// - [`Environment`]: Environmental context container
/// - [`ExternalEntity`]: External sources and sinks
/// - [`Interaction`]: Flow connections between entities
///
/// # See Also
///
/// - [`Info`]: The metadata structure returned by this trait
/// - [`Id`]: The hierarchical identification system used in Info
pub trait HasInfo {
    /// Returns a reference to the entity's metadata information.
    ///
    /// # Returns
    ///
    /// A reference to the [`Info`] struct containing the entity's ID,
    /// hierarchical level, name, and description.
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::{HasInfo, System};
    ///
    /// let system = System { /* ... */ };
    /// let info = system.info();
    /// println!("System ID: {:?}", info.id);
    /// println!("System level: {}", info.level);
    /// ```
    fn info(&self) -> &Info;
}

macro_rules! impl_has_info {
    ($($ty:ty),*) => {
        $(
            impl HasInfo for $ty {
                #[inline(always)]
                fn info(&self) -> &Info {
                    &self.info
                }
            }
        )*
    }
}

impl_has_info!(System, Boundary, Environment, ExternalEntity, Interaction);

/// Trait for entities that can contain external sources and sinks.
///
/// `HasSourcesAndSinks` provides a consistent interface for managing collections
/// of external entities (sources and sinks) within container entities like systems
/// and environments. This enables uniform handling of external entity management
/// across different container types.
///
/// # System Language Context
///
/// Sources and sinks represent the environmental context of a system:
/// - **Sources**: External entities that provide inputs (resources, disruptions)
/// - **Sinks**: External entities that receive outputs (products, waste)
///
/// These external entities define the system's interaction capabilities with
/// its broader environment.
///
/// # Usage Patterns
///
/// ```rust,ignore
/// use bert::data_model::{HasSourcesAndSinks, ExternalEntity, System, Environment};
///
/// fn add_external_input<T: HasSourcesAndSinks>(container: &mut T, source: ExternalEntity) {
///     container.sources_mut().push(source);
/// }
///
/// fn count_external_entities<T: HasSourcesAndSinks>(container: &T) -> usize {
///     container.sources().len() + container.sinks().len()
/// }
///
/// // Works with both systems and environments
/// let mut system = System { /* ... */ };
/// let mut environment = Environment { /* ... */ };
///
/// let source = ExternalEntity { /* ... */ };
/// add_external_input(&mut system, source.clone());
/// add_external_input(&mut environment, source);
/// ```
///
/// # Implementing Types
///
/// The following data model types implement `HasSourcesAndSinks`:
/// - [`System`]: Systems can contain internal sources and sinks
/// - [`Environment`]: The environment contains all external sources and sinks
///
/// # See Also
///
/// - [`ExternalEntity`]: The entities managed by this trait
/// - [`ExternalEntityType`]: Classification of sources vs. sinks
pub trait HasSourcesAndSinks {
    /// Returns a read-only slice of source entities.
    ///
    /// # Returns
    ///
    /// A slice containing all [`ExternalEntity`] instances classified as sources
    /// that are contained within this entity.
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::{HasSourcesAndSinks, System};
    ///
    /// let system = System { /* ... */ };
    /// let sources = system.sources();
    /// println!("System has {} source entities", sources.len());
    /// ```
    fn sources(&self) -> &[ExternalEntity];

    /// Returns a mutable reference to the sources collection.
    ///
    /// # Returns
    ///
    /// A mutable reference to the vector containing source entities,
    /// enabling addition, removal, and modification of sources.
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::{HasSourcesAndSinks, ExternalEntity, System};
    ///
    /// let mut system = System { /* ... */ };
    /// let new_source = ExternalEntity { /* ... */ };
    ///
    /// system.sources_mut().push(new_source);
    /// system.sources_mut().clear(); // Remove all sources
    /// ```
    fn sources_mut(&mut self) -> &mut Vec<ExternalEntity>;

    /// Returns a read-only slice of sink entities.
    ///
    /// # Returns
    ///
    /// A slice containing all [`ExternalEntity`] instances classified as sinks
    /// that are contained within this entity.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::{HasSourcesAndSinks, System};
    ///
    /// let system = System { /* ... */ };
    /// let sinks = system.sinks();
    /// println!("System has {} sink entities", sinks.len());
    /// ```
    fn sinks(&self) -> &[ExternalEntity];

    /// Returns a mutable reference to the sinks collection.
    ///
    /// # Returns
    ///
    /// A mutable reference to the vector containing sink entities,
    /// enabling addition, removal, and modification of sinks.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use bert::data_model::{HasSourcesAndSinks, ExternalEntity, System};
    ///
    /// let mut system = System { /* ... */ };
    /// let new_sink = ExternalEntity { /* ... */ };
    ///
    /// system.sinks_mut().push(new_sink);
    /// system.sinks_mut().retain(|sink| sink.info().name != "obsolete");
    /// ```
    fn sinks_mut(&mut self) -> &mut Vec<ExternalEntity>;
}

macro_rules! impl_has_sources_and_sinks {
    ($($ty:ty),*) => {
        $(
            impl HasSourcesAndSinks for $ty {
                #[inline(always)]
                fn sources(&self) -> &[ExternalEntity] {
                    &self.sources
                }
                #[inline(always)]
                fn sources_mut(&mut self) -> &mut Vec<ExternalEntity> {
                    &mut self.sources
                }
                #[inline(always)]
                fn sinks(&self) -> &[ExternalEntity] {
                    &self.sinks
                }
                #[inline(always)]
                fn sinks_mut(&mut self) -> &mut Vec<ExternalEntity> {
                    &mut self.sinks
                }
            }
        )*
    }
}

impl_has_sources_and_sinks!(System, Environment);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_model_roundtrip_with_agent() {
        let agent = AgentModel {
            agency_capacity: 0.8,
            ..AgentModel::default()
        };
        let json = serde_json::to_string(&agent).unwrap();
        let restored: AgentModel = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.agency_capacity, 0.8);
    }

    #[test]
    fn agent_model_partial_json_roundtrip() {
        // Only required field is `kind`; agency_capacity uses serde default = 0.5
        let json = r#"{"kind": "Reactive"}"#;
        let agent: AgentModel = serde_json::from_str(json).unwrap();
        assert_eq!(agent.agency_capacity, 0.5);
    }

    #[test]
    fn agent_default_agency_capacity() {
        let agent = AgentModel::default();
        assert_eq!(agent.agency_capacity, 0.5);
    }

    #[test]
    fn system_backward_compat_no_agent_field() {
        // Old JSON without agent field should deserialize with agent: None
        let json = r#"{"id": {"kind": "system", "index": 0}, "label": "test", "archetype": null}"#;
        let result = serde_json::from_str::<serde_json::Value>(json);
        assert!(result.is_ok());
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

impl std::fmt::Display for SubstanceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubstanceType::Energy => write!(f, "Energy"),
            SubstanceType::Material => write!(f, "Material"),
            SubstanceType::Message => write!(f, "Message"),
        }
    }
}

impl std::fmt::Display for AgentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentKind::Reactive => write!(f, "Reactive"),
            AgentKind::Anticipatory => write!(f, "Anticipatory"),
            AgentKind::Intentional => write!(f, "Intentional"),
        }
    }
}
