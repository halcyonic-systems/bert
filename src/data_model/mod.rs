pub mod load;
pub mod save;
pub mod import_file_dialog;
pub mod export_file_dialog;

use crate::components::*;
use import_file_dialog::*;
use export_file_dialog::*;
use bevy::prelude::*;
use rust_decimal::Decimal;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct WorldModel {
    pub system_of_interest: System,
    //pub subsystems: Vec<System>
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

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub id: Id,
    pub level: i32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct System {
    // #[serde(rename = "type")]
    pub info: Info,
    pub parent: Option<i32>,
    pub complexity: Complexity,
    pub environment: Environment,
    pub boundary: Boundary,
    pub internal_interactions: Vec<Interaction>,
    pub external_interactions: Vec<Interaction>,
    pub components: Vec<Id>,
    pub transform: Option<Transform2d>,
}

#[derive(Serialize, Deserialize)]
pub struct Boundary {
    pub info: Info,
    pub porosity: f32,
    pub perceptive_fuzziness: f32,
    pub interfaces: Vec<Interface>,
}

#[derive(Serialize, Deserialize)]
pub struct Interface {
    info: Info,
    protocol: String,
    #[serde(rename = "type")]
    ty: InterfaceType,
    exports_to: Vec<Id>,
    receives_from: Vec<Id>,
    angle: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub enum InterfaceType {
    Export,
    Import,
    Hybrid,
}

#[derive(Serialize, Deserialize)]
pub struct Environment {
    pub info: Info,
    pub sources: Vec<ExternalEntity>,
    pub sinks: Vec<ExternalEntity>,
}

#[derive(Serialize, Deserialize)]
pub struct ExternalEntity {
    info: Info,
    #[serde(rename = "type")]
    ty: ExternalEntityType,
    interactions: Vec<Id>,
    transform: Option<Transform2d>,
}

#[derive(Serialize, Deserialize)]
pub enum ExternalEntityType {
    Source,
    Sink,
}

#[derive(Serialize, Deserialize)]
pub struct Interaction {
    info: Info,
    substance: Substance,
    #[serde(rename = "type")]
    ty: InteractionType,
    external_entity: Id,
    amount: Decimal,
    unit: String,
    time_unit: Decimal,
}

#[derive(Serialize, Deserialize)]
pub enum InteractionType {
    Inflow { usability: InflowUsability },
    Outflow { usability: OutflowUsability },
}

#[derive(Serialize, Deserialize)]
pub struct Substance {
    sub_type: Option<String>,
    #[serde(rename = "type")]
    ty: SubstanceType,
}

#[derive(Serialize, Deserialize)]
pub enum Complexity {
    /* contains components */
    Complex { adaptable: bool, evolveable: bool },
    /*  contains no subsystems */
    Atomic,
    /*  bounded to hold many instances of that same type of component */
    Multiset,
}

#[derive(Serialize, Deserialize, Clone)]
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
