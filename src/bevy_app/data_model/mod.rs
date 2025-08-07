//! # Data Model Module
//!
//! This module implements the persistence layer for BERT system models, providing
//! serialization and deserialization capabilities for the System Language framework.
//!
//! ## Architecture
//!
//! The data model implements Layer 3 (Knowledge Representation) of the System Language
//! by providing structured data formats that can be:
//!
//! - **Serialized**: Convert live system models to JSON for storage
//! - **Deserialized**: Reconstruct system models from saved JSON data
//! - **Versioned**: Handle evolution of the data format over time
//! - **Validated**: Ensure data integrity and consistency
//!
//! ## Key Components
//!
//! - [`WorldModel`]: Root container for complete system models
//! - [`Id`]: Hierarchical identification system for all entities
//! - [`System`]: Serializable representation of system entities
//! - [`Interaction`]: Serializable representation of flows between systems
//! - [`Environment`]: Container for external entities and root system context
//!
//! ## Data Format
//!
//! The module uses JSON as the primary serialization format with a versioned schema
//! to support backward compatibility and data migration. The hierarchical ID system
//! ensures proper reconstruction of system relationships during deserialization.
//!
//! ## Usage Patterns
//!
//! ```rust
//! use bert::data_model::{WorldModel, System, Environment};
//!
//! // Serialize a complete world model
//! let world_model = WorldModel {
//!     version: CURRENT_FILE_VERSION,
//!     environment: Environment::default(),
//!     systems: vec![],
//!     interactions: vec![],
//!     hidden_entities: vec![],
//! };
//!
//! let json = serde_json::to_string(&world_model)?;
//! ```
//!
//! ## Version Management
//!
//! The data format uses semantic versioning to handle schema evolution:
//! - Version increments trigger data migration logic
//! - Backward compatibility is maintained where possible
//! - Breaking changes are clearly documented
//!
//! ## See Also
//!
//! - [`load`]: Module for deserializing world models from JSON
//! - [`save`]: Module for serializing world models to JSON
//! - [`crate::bevy_app::components`]: Live ECS components that this module serializes

pub mod complexity_calculator;
pub mod load;
pub mod save;

use crate::bevy_app::components::*;
use bevy::prelude::*;
use rust_decimal::Decimal;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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
/// ```
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
/// ```rust
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
/// ```rust
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
/// ```
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
/// ```rust
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
                .map_err(|err| E::custom(format!("Error parsing type prefix: {:?}", err)))?;

            let indices = v[index..]
                .split(".")
                .map(|i| i.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| E::custom(format!("Error parsing indices: {:?}", err)))?;

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
/// ```rust
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
/// ```rust
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

impl From<crate::bevy_app::components::InterfaceType> for InterfaceType {
    fn from(ty: crate::bevy_app::components::InterfaceType) -> Self {
        match ty {
            crate::bevy_app::components::InterfaceType::Export => Self::Export,
            crate::bevy_app::components::InterfaceType::Import => Self::Import,
        }
    }
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
/// ```rust
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
#[derive(Copy, Clone, Serialize, Deserialize, Reflect, PartialEq, Eq, Hash, Debug)]
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
        evolveable: bool 
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
        u64
    ),
}

impl Complexity {
    /// Determines if this complexity represents a complex system with internal structure.
    ///
    /// # Returns
    ///
    /// `true` only for [`Complex`](Self::Complex) variants, `false` for atomic and multiset systems.
    ///
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Examples
    ///
    /// ```rust
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
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Examples
    ///
    /// ```rust
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
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Examples
    ///
    /// ```rust
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
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Examples
    ///
    /// ```rust
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
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Examples
    ///
    /// ```rust
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
    /// # Returns
    ///
    /// This function returns `()` (unit type) and operates through side effects.
    ///
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Behavior
    ///
    /// Only affects [`Complex`](Self::Complex) variants. Has no effect on
    /// [`Atomic`](Self::Atomic) or [`Multiset`](Self::Multiset) systems.
    ///
    /// # Examples
    ///
    /// ```rust
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
        match self {
            Complexity::Complex { adaptable, .. } => *adaptable = adapt,
            _ => (),
        }
    }

    /// Sets the evolvability property for complex systems.
    ///
    /// # Parameters
    ///
    /// - `evolve`: New evolvability setting
    ///
    /// # Returns
    ///
    /// This function returns `()` (unit type) and operates through side effects.
    ///
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Behavior
    ///
    /// Only affects [`Complex`](Self::Complex) variants. Has no effect on
    /// [`Atomic`](Self::Atomic) or [`Multiset`](Self::Multiset) systems.
    ///
    /// # Examples
    ///
    /// ```rust
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
        match self {
            Complexity::Complex { evolveable, .. } => *evolveable = evolve,
            _ => (),
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

impl From<(&Transform, &InitialPosition)> for Transform2d {
    fn from((t, ip): (&Transform, &InitialPosition)) -> Self {
        Transform2d {
            translation: **ip,
            rotation: t.right().truncate().to_angle(),
        }
    }
}

impl Transform2d {
    pub fn as_components(&self, z: f32, zoom: f32) -> (Transform, InitialPosition) {
        (
            Transform::from_translation((self.translation * zoom).extend(z))
                .with_rotation(Quat::from_rotation_z(self.rotation)),
            InitialPosition::new(self.translation),
        )
    }
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
/// ```rust
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
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Examples
    ///
    /// ```rust
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
/// ```rust
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
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Examples
    ///
    /// ```rust
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
    /// # Errors
    ///
    /// This function does not return errors.
    ///
    /// # Panics
    ///
    /// Does not panic under normal operation.
    ///
    /// # Examples
    ///
    /// ```rust
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
    /// ```rust
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
    /// ```rust
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
