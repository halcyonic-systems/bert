use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::typedb_reader;

#[derive(Serialize, Deserialize)]
pub struct LaunchParams {
    pub seed: Option<u64>,
    pub steps: u64,
    pub db: String,
    pub model_name: String,
}

#[tauri::command]
pub async fn launch_simulation(params: LaunchParams) -> Result<typedb_reader::RunInfo, String> {
    let run_id = uuid::Uuid::new_v4().to_string();
    let seed = params.seed.unwrap_or(42);

    let python_dir = std::env::current_dir()
        .map_err(|e| format!("cwd: {e}"))?
        .join("python");

    let venv_python = python_dir.join("venv/bin/python3");
    let python_bin = if venv_python.exists() {
        venv_python.to_string_lossy().to_string()
    } else {
        "python3".to_string()
    };

    let runner = python_dir.join("mesa_runner.py");
    if !runner.exists() {
        return Err(format!("mesa_runner.py not found at {}", runner.display()));
    }

    Command::new(&python_bin)
        .arg(runner.to_string_lossy().as_ref())
        .args([
            "--seed", &seed.to_string(),
            "--steps", &params.steps.to_string(),
            "--db", &params.db,
            "--model-name", &params.model_name,
            "--run-id", &run_id,
        ])
        .spawn()
        .map_err(|e| format!("Failed to spawn mesa_runner.py: {e}"))?;

    Ok(typedb_reader::RunInfo {
        run_id,
        status: "Pending".into(),
        tick_count: 0,
    })
}

#[derive(Serialize, Deserialize)]
pub struct PollParams {
    pub db: String,
    pub run_id: String,
}

#[tauri::command]
pub async fn poll_run_status(params: PollParams) -> Result<typedb_reader::RunStatus, String> {
    let driver = typedb_reader::connect().await?;
    typedb_reader::query_run_status(&driver, &params.db, &params.run_id).await
}

#[derive(Serialize, Deserialize)]
pub struct ResultsParams {
    pub db: String,
    pub run_id: String,
}

#[tauri::command]
pub async fn get_run_results(params: ResultsParams) -> Result<typedb_reader::SimulationResults, String> {
    let driver = typedb_reader::connect().await?;
    typedb_reader::query_results(&driver, &params.db, &params.run_id).await
}

#[derive(Serialize, Deserialize)]
pub struct ListRunsParams {
    pub db: String,
    pub model_name: String,
}

#[tauri::command]
pub async fn list_runs(params: ListRunsParams) -> Result<Vec<typedb_reader::RunInfo>, String> {
    let driver = typedb_reader::connect().await?;
    typedb_reader::query_runs(&driver, &params.db, &params.model_name).await
}
