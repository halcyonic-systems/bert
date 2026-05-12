use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::typedb_reader;

#[derive(Serialize, Deserialize)]
pub struct LaunchParams {
    pub seed: Option<u64>,
    pub steps: u64,
    pub db: String,
    pub model_name: String,
    #[serde(default)]
    pub json_path: Option<String>,
    #[serde(default)]
    pub params: Option<HashMap<String, f64>>,
}

fn resolve_model_path(python_dir: &std::path::Path, model_name: &str) -> Option<String> {
    let project_root = python_dir.parent()?;
    let asset_dirs = [
        project_root.join("assets/models/examples"),
        project_root.join("assets/models/local/test-primitives"),
        project_root.join("dist/assets/models/examples"),
        project_root.join("dist/assets/models/local/test-primitives"),
    ];
    for dir in &asset_dirs {
        let path = dir.join(format!("{model_name}.json"));
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
    }
    None
}

#[tauri::command]
pub async fn launch_simulation(params: LaunchParams) -> Result<typedb_reader::RunInfo, String> {
    let run_id = uuid::Uuid::new_v4().to_string();
    let seed = params.seed.unwrap_or(42);

    let cwd = std::env::current_dir().map_err(|e| format!("cwd: {e}"))?;
    // Tauri cwd is src-tauri/, python/ is at project root
    let python_dir = if cwd.join("python").exists() {
        cwd.join("python")
    } else {
        cwd.parent().unwrap_or(&cwd).join("python")
    };

    let venv_python = python_dir.join("venv/bin/python3");
    let python_bin = if venv_python.exists() {
        venv_python.to_string_lossy().to_string()
    } else {
        "python3".to_string()
    };

    let runner = python_dir.join("mesa_runner.py");
    if !runner.exists() {
        return Err(
            "Simulation requires running BERT from source (cargo tauri dev) \
             with Python and TypeDB installed. \
             See https://bert.gitbook.io for setup instructions."
                .into(),
        );
    }

    let mut cmd = Command::new(&python_bin);
    cmd.arg(runner.to_string_lossy().as_ref())
        .args(["--seed", &seed.to_string()])
        .args(["--steps", &params.steps.to_string()])
        .args(["--run-id", &run_id]);

    if let Some(ref json_path) = params.json_path {
        cmd.args(["--json-path", json_path]);
    } else if !params.model_name.is_empty() {
        let resolved = resolve_model_path(&python_dir, &params.model_name);
        if let Some(path) = resolved {
            cmd.args(["--json-path", &path]);
        } else {
            cmd.args(["--db", &params.db])
                .args(["--model-name", &params.model_name]);
        }
    } else {
        return Err("No model loaded".into());
    }

    if let Some(ref overrides) = params.params {
        if !overrides.is_empty() {
            let json_str =
                serde_json::to_string(overrides).map_err(|e| format!("params serialize: {e}"))?;
            cmd.args(["--params", &json_str]);
        }
    }

    cmd.spawn()
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
pub async fn get_run_results(
    params: ResultsParams,
) -> Result<typedb_reader::SimulationResults, String> {
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

#[derive(Serialize, Deserialize)]
pub struct JsonPollParams {
    pub run_id: String,
}

#[tauri::command]
pub async fn poll_json_run_status(
    params: JsonPollParams,
) -> Result<typedb_reader::RunStatus, String> {
    let temp_dir = std::env::temp_dir();
    let results_path = temp_dir.join(format!("{}_results.json", params.run_id));
    let status_path = temp_dir.join(format!("{}_status.json", params.run_id));

    if results_path.exists() {
        let mut tick_count = 0u64;
        let mut total_ticks = 0u64;
        if let Ok(data) = std::fs::read_to_string(&status_path) {
            if let Ok(st) = serde_json::from_str::<typedb_reader::RunStatus>(&data) {
                tick_count = st.tick_count;
                total_ticks = st.total_ticks;
            }
        }
        return Ok(typedb_reader::RunStatus {
            run_id: params.run_id,
            status: "Complete".into(),
            tick_count,
            total_ticks,
        });
    }

    if status_path.exists() {
        let data = std::fs::read_to_string(&status_path)
            .map_err(|e| format!("Failed to read status file: {e}"))?;
        let status: typedb_reader::RunStatus =
            serde_json::from_str(&data).map_err(|e| format!("Failed to parse status: {e}"))?;
        return Ok(status);
    }

    Ok(typedb_reader::RunStatus {
        run_id: params.run_id,
        status: "Running".into(),
        tick_count: 0,
        total_ticks: 0,
    })
}

#[derive(Serialize, Deserialize)]
pub struct JsonResultsParams {
    pub run_id: String,
}

#[tauri::command]
pub async fn get_json_run_results(
    params: JsonResultsParams,
) -> Result<typedb_reader::SimulationResults, String> {
    let temp_dir = std::env::temp_dir();
    let results_path = temp_dir.join(format!("{}_results.json", params.run_id));
    let status_path = temp_dir.join(format!("{}_status.json", params.run_id));

    let data = std::fs::read_to_string(&results_path)
        .map_err(|e| format!("Failed to read results file: {e}"))?;
    let results: typedb_reader::SimulationResults =
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse results: {e}"))?;

    let _ = std::fs::remove_file(&results_path);
    let _ = std::fs::remove_file(&status_path);

    Ok(results)
}
