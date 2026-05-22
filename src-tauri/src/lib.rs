use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

mod chat_service;
pub use bert_generator_core::generator;
pub use bert_generator_core::intermediate;
mod simulation;
mod typedb_reader;

#[derive(Serialize, Deserialize)]
struct Data {
    data: String,
}

#[tauri::command]
fn save_to_file(data: &str, path: &str) {
    let data = data.to_string();
    std::thread::spawn({
        let path = path.to_string();
        move || {
            std::fs::write(path, data.clone()).unwrap();
        }
    });
}

#[tauri::command]
fn save_with_dialog(app_handle: AppHandle, data: &str, path: &str) {
    let data = data.to_string();
    app_handle
        .dialog()
        .file()
        .add_filter("valid_formats", &["json"])
        .set_file_name(path)
        .save_file(move |file_path| {
            if let Some(path) = file_path {
                match path.into_path() {
                    Ok(path_buf) => {
                        if let Err(e) = std::fs::write(&path_buf, data.clone()) {
                            eprintln!("Failed to save file: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get file path: {:?}", e);
                    }
                }
            }
        });
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct FileData {
    data: Vec<u8>,
    path: String,
}

#[tauri::command]
async fn pick_file(app_handle: AppHandle) -> Option<PathBuf> {
    app_handle
        .dialog()
        .file()
        .add_filter("valid_formats", &["json"])
        .blocking_pick_file()
        .map(|file_path| file_path.into_path().unwrap())
}

#[tauri::command]
fn load_file(_app_handle: AppHandle, pb: PathBuf) -> Result<FileData, String> {
    let data = std::fs::read(&pb).map_err(|err| format!("Error reading file: {:?}", err))?;
    Ok(FileData {
        data,
        path: pb.to_str().unwrap().to_string(),
    })
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct LocalModelInfo {
    name: String,
    path: String,
    modified: u64,
}

#[tauri::command]
async fn check_ollama_status() -> bool {
    match reqwest::Client::new()
        .get("http://localhost:11434/api/tags")
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
    {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

#[tauri::command]
fn list_local_models() -> Vec<LocalModelInfo> {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let local_dir = project_root.join("assets/models/local");
    let mut models = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&local_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .replace('_', " ");
                let modified = entry
                    .metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                models.push(LocalModelInfo {
                    name,
                    path: path.to_str().unwrap_or("").to_string(),
                    modified,
                });
            }
        }
    }

    models.sort_by(|a, b| b.modified.cmp(&a.modified));
    models
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_to_file,
            save_with_dialog,
            pick_file,
            load_file,
            check_ollama_status,
            list_local_models,
            chat_service::chat_with_model,
            chat_service::generate_model_from_conversation,
            simulation::launch_simulation,
            simulation::poll_run_status,
            simulation::get_run_results,
            simulation::list_runs,
            simulation::poll_json_run_status,
            simulation::get_json_run_results,
            simulation::export_simulation_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
