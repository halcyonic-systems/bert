pub mod load;
pub mod save;

use crate::bevy_app::components::*;
use bevy::prelude::*;
use rust_decimal::Decimal;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub const CURRENT_FILE_VERSION: u32 = 1;

/// Root object
#[derive(Serialize, Deserialize)]
pub struct WorldModel {
    /// File format version. When changes are made to the structure then this needs to be increased
    /// so files saved with previous versions can be converted when they are loaded.
    pub version: u32,
    /// This is kind of the parent of the root system
    pub environment: Environment,
    /// All systems including the root system and all subsystems at all nesting levels.
    pub systems: Vec<System>,
    /// All interactions at all nesting levels.
    pub interactions: Vec<Interaction>,
}

/// Unique identifier for any kind of object.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id {
    /// Type of the object
    pub ty: IdType,
    /// List of indices. The last is a serial number (starting at 0) for this type of object
    /// (same `IdType`). The penultimate number is the serial number of the parent system. The third to
    /// last is the number of the grandparent system and so on until the root (which is always 0)
    /// is reached. The environment is the only object that has serial number -1 and it's children
    /// are `[-1, 0]`, `[-1, 1]`, ...
    ///
    /// For example `[0, 1, 5]` means that this is object number `5` of this type and is the child
    /// of system number `1` which is a subsystem of the root system.
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

/// Type of an [`Id`]. Self explanatory.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IdType {
    #[serde(rename = "S")]
    System,
    #[serde(rename = "C")]
    Subsystem,
    #[serde(rename = "I")]
    Interface,
    #[serde(rename = "Src")]
    Source,
    #[serde(rename = "Snk")]
    Sink,
    #[serde(rename = "E")]
    Environment,
    #[serde(rename = "F")]
    Flow,
    #[serde(rename = "B")]
    Boundary,
}

/// Common data. Most serialized entities have this.
#[derive(Serialize, Deserialize, Clone)]
pub struct Info {
    /// The entities id
    pub id: Id,
    /// The nesting level of the entity. It is usually the parent level + 1 and equals
    /// `id.indices.len() - 1`. The exception is the environment and all interactions and
    /// external entities in it. They all have level -1.
    pub level: i32,
    pub name: String,
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

#[derive(Copy, Clone, Serialize, Deserialize, Reflect, PartialEq, Eq, Hash, Debug)]
pub enum Complexity {
    /// contains components
    Complex { adaptable: bool, evolveable: bool },
    /// contains no subsystems
    Atomic,
    /// bounded to hold many instances of that same type of component
    Multiset(u64),
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

pub trait HasInfo {
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

pub trait HasSourcesAndSinks {
    fn sources(&self) -> &[ExternalEntity];
    fn sources_mut(&mut self) -> &mut Vec<ExternalEntity>;

    fn sinks(&self) -> &[ExternalEntity];
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
