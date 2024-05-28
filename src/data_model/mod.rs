pub mod load;
pub mod save;

use crate::components::*;
use bevy::prelude::*;
use rust_decimal::Decimal;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub const CURRENT_FILE_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct WorldModel {
    pub version: u32,
    pub environment: Environment,
    pub systems: Vec<System>,
    pub interactions: Vec<Interaction>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id {
    pub ty: IdType,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Info {
    pub id: Id,
    pub level: i32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct System {
    pub info: Info,
    pub sources: Vec<ExternalEntity>,
    pub sinks: Vec<ExternalEntity>,
    pub parent: Id,
    pub complexity: Complexity,
    pub boundary: Boundary,
    pub radius: f32,
    pub transform: Option<Transform2d>,
    pub equivalence: String,
    pub history: String,
    pub transformation: String,
    pub member_autonomy: f32,
    pub time_constant: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Boundary {
    pub info: Info,
    pub porosity: f32,
    pub perceptive_fuzziness: f32,
    pub interfaces: Vec<Interface>,
    pub parent_interface: Option<Id>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Interface {
    pub info: Info,
    pub protocol: String,
    #[serde(rename = "type")]
    pub ty: InterfaceType,
    pub exports_to: Vec<Id>,
    pub receives_from: Vec<Id>,
    pub angle: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum InterfaceType {
    Export,
    Import,
    Hybrid,
}

impl From<crate::components::InterfaceType> for InterfaceType {
    fn from(ty: crate::components::InterfaceType) -> Self {
        match ty {
            crate::components::InterfaceType::Export => Self::Export,
            crate::components::InterfaceType::Import => Self::Import,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Environment {
    pub info: Info,
    pub sources: Vec<ExternalEntity>,
    pub sinks: Vec<ExternalEntity>,
}

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

#[derive(Serialize, Deserialize, Clone)]
pub struct Interaction {
    pub info: Info,
    pub substance: Substance,
    #[serde(rename = "type")]
    pub ty: InteractionType,
    pub usability: InteractionUsability,
    pub source: Id,
    pub source_interface: Option<Id>,
    pub sink: Id,
    pub sink_interface: Option<Id>,
    pub amount: Decimal,
    pub unit: String,
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

#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, Debug)]
pub struct Transform2d {
    pub translation: Vec2,
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
