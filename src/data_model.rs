use serde::{Deserialize, Serialize};
use bevy::prelude::*;

pub fn save_world(
    world: &World,
    soi_info_query: Query<
        (Entity, &Name, &crate::components::ElementDescription),
        With<crate::components::SystemOfInterest>
    >,
    // subsystem_query: Query<
    //     (Entity, &Name, &crate::components::ElementDescription, &crate::components::SystemElement),
    //     (With<crate::components::Subsystem>, Without<crate::components::SystemOfInterest>)
    // >
) {
    
    let mut system_of_interest = System::default();
    let (entity, name, description) = soi_info_query.get_single().expect("System of interest should exist");
    {
        system_of_interest.info = Info {
            entity,
            name: name.to_string(),
            description: description.text.clone(),
            construct_type: Construct::SYSTEM,
            level: 0,
            parent: None,
        }
    };
    
    let model = WorldModel {
        system_of_interest,
        ..default()
    };
    save_to_json(&model, "world_model.json");
}


fn save_to_json(world_model: &WorldModel, file_name: &str) {
    let json = serde_json::to_string(world_model).expect("This shouldn't fail");
    std::fs::write(file_name, json).expect("This shouldn't fail");
}

#[derive(Serialize, Deserialize, Default)]
pub struct WorldModel {
    pub system_of_interest: System,
    //pub subsystems: Vec<System>
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub entity: Entity,
    pub parent: Option<i32>,
    pub level: i32,
    pub name: String,
    pub description: String,
    pub construct_type: Construct,
}
impl Default for Info {
    fn default() -> Self {
        Info {
            entity: Entity::PLACEHOLDER,
            parent: None,
            level: 0,
            name: "".to_string(),
            description: "".to_string(),
            construct_type: Construct::SYSTEM,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub enum Construct {
    #[default]
    SYSTEM,
    BOUNDARY,
    INTERFACE,
    INTERACTION,
    EXTERNALENTITY,
}

#[derive(Serialize, Deserialize, Default)]
pub struct System {
    // #[serde(rename = "type")]
    pub info: Info,
    pub complexity: Complexity,
    pub environment: Environment,
    pub boundary: Boundary,
    pub internal_interactions: Vec<Interaction>,
    pub external_interactions: Vec<Interaction>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Boundary {
    pub info: Info,
    pub porosity: f32,
    pub perceptive_fuzziness: f32,
    pub interfaces: Vec<Interface>,
}

#[derive(Serialize, Deserialize)]
pub enum Interface {
    EXPORT { info: Info, protocol: String },
    IMPORT { info: Info, protocol: String },
    HYBRID { info: Info, protocol: String },
}

#[derive(Serialize, Deserialize, Default)]
pub struct Environment {
    pub info: Info,
    pub sources: Vec<ExternalEntity>,
    pub sinks: Vec<ExternalEntity>,
}

#[derive(Serialize, Deserialize)]
pub enum ExternalEntity {
    SOURCE { info: Info },
    SINK { info: Info }
}

#[derive(Serialize, Deserialize)]
pub enum Interaction {
    INFLOW {info: Info, substance: Substance, usability: InflowUsability},
    OUTFLOW {info: Info,substance: Substance, usability: OutflowUsability}
}

#[derive(Serialize, Deserialize, Default)]
pub enum InflowUsability {
    #[default]
    RESOURCE,
    DISRUPTION,
}

#[derive(Serialize, Deserialize, Default)]
pub enum OutflowUsability {
    #[default]
    PRODUCT,
    WASTE,
}

#[derive(Serialize, Deserialize)]
pub enum Substance {
    ENERGY  {sub_type: Option<String>},
    MATTER  {sub_type: Option<String>},
    MESSAGE {sub_type: Option<String>},
}

#[derive(Serialize, Deserialize)]
pub enum Complexity {
    /* contains components */
    COMPLEX {adaptable: bool, evolveable: bool},  
    /*  contains no subsystems */
    ATOMIC,
    /*  bounded to hold many instances of that same type of component */
    MULTISET
}

macro_rules! impl_enum_default {
    ($enum_name:ty, $variant_struct:expr) => {
        impl Default for $enum_name {
            fn default() -> Self {
                $variant_struct
            }
        }
    };
}

impl_enum_default!(Complexity, Complexity::COMPLEX { adaptable: false, evolveable: false });
impl_enum_default!(Substance, Substance::ENERGY { sub_type: None });
impl_enum_default!(Interaction, Interaction::OUTFLOW { info: Info::default(), substance: Substance::default(), usability: OutflowUsability::default() });
impl_enum_default!(ExternalEntity, ExternalEntity::SINK { info: Info::default() });
impl_enum_default!(Interface, Interface::EXPORT { info: Info::default(), protocol: default() } );

