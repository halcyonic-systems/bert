use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Soi {
    id: String,
    name: String,
    #[serde(rename = "type")]
    ty: SoiType,
    environment: Environment,
    // boundary: Boundary
}

#[derive(Serialize, Deserialize)]
pub struct Environment {
    name: String,
    // sources: Vec<Source>,
    // sinks: Vec<Sink>
}

#[derive(Serialize, Deserialize)]
pub enum SoiType {
    PROCESS,
}

pub fn save_to_json(soi: &Soi, file_name: &str) {
    let json = serde_json::to_string(soi).expect("This shouldn't fail");
    std::fs::write(file_name, json).expect("This shouldn't fail");
}