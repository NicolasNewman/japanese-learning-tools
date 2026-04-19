// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env::current_exe;

use image::{DynamicImage, RgbImage};
use oar_ocr::core::config::{OrtExecutionProvider, OrtSessionConfig};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri::Manager;
use tauri::Runtime;
use tauri::Window;
use tauri_plugin_shell::ShellExt;
use xcap::Monitor;

use oar_ocr::prelude::*;

use tokio::runtime::Runtime as TokioRuntime;

struct AppData {
    monitor_id: Mutex<u32>,
    ocr: Arc<OAROCR>,
    ocr_runtime: TokioRuntime,
}

#[tauri::command]
async fn capture<R: Runtime>(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    app: AppHandle<R>,
    window: Window<R>,
    state: tauri::State<'_, AppData>,
) -> Result<Vec<String>, String> {
    let image = {
        let monitor_id = *state.monitor_id.lock().map_err(|e| e.to_string())?;
        let monitor = Monitor::all()
            .map_err(|e| e.to_string())?
            .into_iter()
            .find(|m| m.id().ok() == Some(monitor_id))
            .ok_or_else(|| "Monitor not found".to_string())?;

        monitor
            .capture_region(x, y, width, height)
            .map_err(|e| e.to_string())?
    };

    let mut image: RgbImage = DynamicImage::ImageRgba8(image).into_rgb8();
    let scale_factor = 0.5;
    if width > 800 || height > 600 {
        let new_width = (width as f32 * scale_factor) as u32;
        let new_height = (height as f32 * scale_factor) as u32;
        image = image::imageops::resize(
            &image,
            new_width,
            new_height,
            image::imageops::FilterType::Triangle,
        );
    }

    let ocr = state.ocr.clone();
    let results = state
        .ocr_runtime
        .spawn_blocking(move || ocr.predict(vec![image]))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("OCR prediction failed: {}", e))?;

    for text_region in &results[0].text_regions {
        if let Some((text, confidence)) = text_region.text_with_confidence() {
            println!("Text: {} ({:.2})", text, confidence);
        }
    }
    println!("Recognized text: {}", results.len());
    println!(
        "Region: x={}, y={}, width={}, height={}",
        x, y, width, height
    );
    Ok(results[0]
        .text_regions
        .iter()
        .filter_map(|r| r.text_with_confidence().map(|(t, _)| t.to_string()))
        .collect::<Vec<_>>())
}

#[tauri::command]
async fn start_region_select<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    tauri::WebviewWindowBuilder::new(
        &app,
        "region-selector",
        tauri::WebviewUrl::App("region-selector".into()),
    )
    .title("Select Region")
    .fullscreen(true)
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

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
            translate_jp_en,
            start_region_select,
            capture
        ])
        .setup(|app| {
            let monitor = Monitor::all()
                .unwrap()
                .into_iter()
                .find(|m| m.is_primary().unwrap_or(false))
                .expect("No primary monitor found");

            let monitor_id = monitor.id()?;

            let ocr_dir = app
                .path()
                .resource_dir()
                .expect("Failed to get resource dir")
                .join("resources")
                .join("ocr");

            let ort_config = OrtSessionConfig::new().with_execution_providers(vec![
                OrtExecutionProvider::CUDA {
                    device_id: Some(0),
                    gpu_mem_limit: None,
                    arena_extend_strategy: None,
                    cudnn_conv_algo_search: None,
                    cudnn_conv_use_max_workspace: None,
                },
                OrtExecutionProvider::CPU,
            ]);

            let ocr = OAROCRBuilder::new(
                ocr_dir.join("pp-ocrv5_mobile_det.onnx"),
                ocr_dir.join("pp-ocrv5_mobile_rec.onnx"),
                ocr_dir.join("ppocrv5_dict.txt"),
            )
            .ort_session(ort_config)
            .build()?;

            app.manage(AppData {
                monitor_id: Mutex::new(monitor_id),
                ocr: Arc::new(ocr),
                ocr_runtime: tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(2)
                    .thread_name("ocr-worker")
                    .build()
                    .expect("Failed to create OCR runtime"),
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
