use std::{
    path::PathBuf,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

mod chat_service;
use chat_service::{MinimalLLMService, ChatRequest, ChatResponse};

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
            std::fs::write(file_path.unwrap().into_path().unwrap(), data.clone()).unwrap();
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
        .map(|file_path| {
            file_path.into_path().unwrap()
        })
}

#[tauri::command]
fn load_file(_app_handle: AppHandle, pb: PathBuf) -> Result<FileData, String> {
    let data = std::fs::read(&pb).map_err(|err| format!("Error reading file: {:?}", err))?;
    
    // Try to parse and store the model data for chat
    if let Ok(json_str) = String::from_utf8(data.clone()) {
        // Validate it's valid JSON and store it for chat
        if serde_json::from_str::<serde_json::Value>(&json_str).is_ok() {
            update_current_model(json_str);
        }
    }
    
    Ok(FileData {
        data,
        path: pb.to_str().unwrap().to_string(),
    })
}

// Global chat service instance
static CHAT_SERVICE: Mutex<Option<MinimalLLMService>> = Mutex::new(None);

// Global current model data
static CURRENT_MODEL_DATA: Mutex<Option<String>> = Mutex::new(None);

#[tauri::command]
fn update_current_model(model_data: String) {
    let mut data = CURRENT_MODEL_DATA.lock().unwrap();
    *data = Some(model_data);
}

#[tauri::command]
fn get_current_model() -> Option<String> {
    let data = CURRENT_MODEL_DATA.lock().unwrap();
    data.clone()
}

#[tauri::command]
async fn chat_with_model(message: String, model_data: String) -> Result<ChatResponse, String> {
    // Initialize chat service if not already done
    {
        let mut service = CHAT_SERVICE.lock().unwrap();
        if service.is_none() {
            *service = Some(MinimalLLMService::new().map_err(|e| e.to_string())?);
        }
    }
    
    // Try to get the current model data, fallback to the provided model_data
    let current_model = get_current_model().unwrap_or(model_data);
    
    // Create request outside the lock
    let request = ChatRequest {
        message,
        model_context: current_model,
    };
    
    // Get a clone of the service to avoid holding the lock across await
    let service = {
        let guard = CHAT_SERVICE.lock().unwrap();
        guard.as_ref().unwrap().clone()
    };
    
    service.chat(request).await.map_err(|e| e.to_string())
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
            update_current_model,
            get_current_model,
            chat_with_model
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
