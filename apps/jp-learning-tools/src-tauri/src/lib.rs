// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env::current_exe;

use tauri::AppHandle;
use tauri::Manager;
use tauri::Runtime;
use tauri::Window;

use tauri_plugin_shell::ShellExt;

#[tauri::command]
fn external_binary_dir<R: Runtime>(app: AppHandle<R>, window: Window<R>) -> String {
    current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_string_lossy().to_string()))
        .unwrap_or_default()
}

#[tauri::command]
fn open_tmp_log<R: Runtime>(app: AppHandle<R>, window: Window<R>) -> Result<(), String> {
    let log_file = std::env::temp_dir()
        .join("subs2clipboard-log")
        .to_string_lossy()
        .to_string();

    match tauri_plugin_opener::open_path(&log_file, None::<&str>) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open file with system default: {}", e)),
    }
}

#[tauri::command]
fn open_devtools<R: Runtime>(app: AppHandle<R>, window: Window<R>) {
    let window = app.get_webview_window("main").unwrap();
    window.open_devtools();
}

#[tauri::command]
async fn translate_jp_en<R: Runtime>(text: String, app: AppHandle<R>) -> Result<String, String> {
    let shell = app.shell();
    let output = shell
        .command("gd-tools")
        .args(vec!["translate", "--sentence", &text, "--no-html"])
        .output()
        .await
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format!(
            "Command failed with exit code: {:?}\nStderr: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            external_binary_dir,
            open_devtools,
            open_tmp_log,
            translate_jp_en
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
