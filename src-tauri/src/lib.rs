use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

#[derive(Serialize, Deserialize)]
struct Data {
    data: String,
}
#[tauri::command]
fn save_to_file(app_handle: AppHandle, data: &str, path: &str) {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![save_to_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
