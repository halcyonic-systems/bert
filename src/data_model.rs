use crate::components::*;
use bevy::prelude::*;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub fn save_world(
    soi_info_query: Query<
        (
            Entity,
            &Name,
            &ElementDescription,
            &crate::components::System,
            &SystemEnvironment,
        ),
        Without<Subsystem>,
    >,
    inflow_query: Query<(
        &Name,
        &ElementDescription,
        &Inflow,
        &InflowSourceConnection,
        &InflowInterfaceConnection,
    )>,
    outflow_query: Query<(
        &Name,
        &ElementDescription,
        &Outflow,
        &OutflowSinkConnection,
        &OutflowInterfaceConnection,
    )>,
    interface_query: Query<(&Name, &ElementDescription, &crate::components::Interface)>,
    external_entity_query: Query<(&Name, &ElementDescription)>,
    // subsystem_query: Query<
    //     (Entity, &Name, &crate::components::ElementDescription, &crate::components::SystemElement),
    //     (With<crate::components::Subsystem>, Without<crate::components::SystemOfInterest>)
    // >
) {
    let (system_entity, name, description, system, environment) = soi_info_query
        .get_single()
        .expect("System of interest should exist");

    let mut interfaces = vec![];
    let mut external_interactions = vec![];
    let mut sources = vec![];
    let mut sinks = vec![];

    for (name, description, flow, source_connection, interface_connection) in &inflow_query {
        process_external_inflow(
            &interface_query,
            &external_entity_query,
            system_entity,
            &mut interfaces,
            &mut external_interactions,
            &mut sources,
            name,
            description,
            flow,
            source_connection,
            interface_connection,
        );
    }

    for (name, description, flow, sink_connection, interface_connection) in &outflow_query {
        process_external_outflow(
            &interface_query,
            &external_entity_query,
            system_entity,
            &mut interfaces,
            &mut external_interactions,
            &mut sinks,
            name,
            description,
            flow,
            sink_connection,
            interface_connection,
        );
    }

    let boundary = Boundary {
        info: Info {
            id: Id {
                ty: IdType::Boundary,
                indices: vec![0],
            },
            level: 0,
            name: system.boundary.name.clone(),
            description: system.boundary.description.clone(),
        },
        porosity: system.boundary.porosity,
        perceptive_fuzziness: system.boundary.perceptive_fuzziness,
        interfaces,
    };

    let environment = Environment {
        info: Info {
            id: Id {
                ty: IdType::Environment,
                indices: vec![-1],
            },
            name: environment.name.clone(),
            description: environment.description.clone(),
            level: -1,
        },
        sources,
        sinks,
    };

    let system_of_interest = System {
        info: Info {
            id: Id {
                ty: IdType::System,
                indices: vec![0],
            },
            level: 0,
            name: name.to_string(),
            description: description.text.clone(),
        },
        parent: None,
        complexity: Complexity::Complex {
            adaptable: system.adaptable,
            evolveable: system.evolveable,
        },
        environment,
        boundary,
        internal_interactions: vec![], // TODO
        external_interactions,
        components: vec![],
    };

    let model = WorldModel { system_of_interest };
    save_to_json(&model, "world_model.json");
}

macro_rules! process_external_flow {
    (
        $fn_name:ident,
        $flow_ty:ty,
        $interface_connection_ty:ty,
        $external_entity_connection_ty:ty,
        $interaction_ty:tt,
        $interface_ty:expr,
        $external_entity_ty:expr,
        $id_ty:expr,
        $external_entity_field:tt
    ) => {
        fn $fn_name(
            interface_query: &Query<(&Name, &ElementDescription, &crate::components::Interface)>,
            external_entity_query: &Query<(&Name, &ElementDescription)>,
            system_entity: Entity,
            interfaces: &mut Vec<Interface>,
            interactions: &mut Vec<Interaction>,
            external_entities: &mut Vec<ExternalEntity>,
            name: &Name,
            description: &ElementDescription,
            flow: &$flow_ty,
            source_connection: &$external_entity_connection_ty,
            interface_connection: &$interface_connection_ty,
        ) {
            if flow.system == system_entity {
                let interaction_id = Id {
                    ty: IdType::Flow,
                    indices: vec![-1, interactions.len() as i64],
                };
                let external_entity_id = Id {
                    ty: $id_ty,
                    indices: vec![-1, external_entities.len() as i64],
                };

                interactions.push(Interaction {
                    info: Info {
                        id: interaction_id.clone(),
                        name: name.to_string(),
                        description: description.text.clone(),
                        level: -1,
                    },
                    substance: Substance {
                        sub_type: None, // TODO
                        ty: flow.substance_type,
                    },
                    ty: InteractionType::$interaction_ty {
                        usability: flow.usability,
                    },
                    external_entity: external_entity_id.clone(),
                });

                let (interface_name, interface_description, interface) = interface_query
                    .get(interface_connection.target)
                    .expect("Interface should exist");

                let mut interface = Interface {
                    info: Info {
                        id: Id {
                            ty: IdType::Interface,
                            indices: vec![0, interfaces.len() as i64],
                        },
                        name: interface_name.to_string(),
                        description: interface_description.text.clone(),
                        level: 1,
                    },
                    protocol: interface.protocol.clone(),
                    ty: $interface_ty,                               // TODO: hybrid
                    receives_from: vec![], // TODO : multiple
                    exports_to: vec![],                              // TODO
                };
                
                interface.$external_entity_field.push(external_entity_id.clone());
                
                interfaces.push(interface);

                let (external_entity_name, external_entity_description) = external_entity_query
                    .get(source_connection.target)
                    .expect("External entity should exist");

                external_entities.push(ExternalEntity {
                    info: Info {
                        id: external_entity_id,
                        name: external_entity_name.to_string(),
                        description: external_entity_description.text.clone(),
                        level: -1,
                    },
                    ty: $external_entity_ty,
                    interactions: vec![interaction_id], // TODO : multiple
                });
            }
        }
    };
}

process_external_flow!(
    process_external_inflow,
    Inflow,
    InflowInterfaceConnection,
    InflowSourceConnection,
    Inflow,
    InterfaceType::Import,
    ExternalEntityType::Source,
    IdType::Source,
    receives_from
);

process_external_flow!(
    process_external_outflow,
    Outflow,
    OutflowInterfaceConnection,
    OutflowSinkConnection,
    Outflow,
    InterfaceType::Export,
    ExternalEntityType::Sink,
    IdType::Sink,
    exports_to
);

fn save_to_json(world_model: &WorldModel, file_name: &str) {
    let json = serde_json::to_string(world_model).expect("This shouldn't fail");
    std::fs::write(file_name, json).expect("This shouldn't fail");
}

#[derive(Serialize, Deserialize)]
pub struct WorldModel {
    pub system_of_interest: System,
    //pub subsystems: Vec<System>
}

#[derive(Clone, Debug)]
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
        if let Some(index) = v.find(|c: char| c.is_numeric()) {
            let ty = serde_json::from_str(&v[..index])
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

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
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
