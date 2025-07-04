// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env::current_exe;

use tauri::Manager;
use tauri::Runtime;
use tauri::AppHandle;
use tauri::Window;

#[tauri::command]
fn external_binary_dir<R: Runtime>(app: AppHandle<R>, window: Window<R>) -> String {
    current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_string_lossy().to_string()))
        .unwrap_or_default()
}

#[tauri::command]
fn open_devtools<R: Runtime>(app: AppHandle<R>, window: Window<R>) {
    let window = app.get_webview_window("main").unwrap();
    window.open_devtools();
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![external_binary_dir, open_devtools])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
