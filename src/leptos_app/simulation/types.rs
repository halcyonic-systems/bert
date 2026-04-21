use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RunInfo {
    pub run_id: String,
    pub status: String,
    pub tick_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RunStatus {
    pub run_id: String,
    pub status: String,
    pub tick_count: u64,
    pub total_ticks: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LaunchParams {
    pub seed: Option<u64>,
    pub steps: u64,
    pub db: String,
    pub model_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PollParams {
    pub db: String,
    pub run_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowTimeseries {
    pub interaction_id: String,
    pub name: String,
    pub ticks: Vec<u64>,
    pub values: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemTimeseries {
    pub system_id: String,
    pub name: String,
    pub key: String,
    pub ticks: Vec<u64>,
    pub values: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimulationResults {
    pub run_id: String,
    pub flow_timeseries: Vec<FlowTimeseries>,
    pub system_timeseries: Vec<SystemTimeseries>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultsParams {
    pub db: String,
    pub run_id: String,
}
