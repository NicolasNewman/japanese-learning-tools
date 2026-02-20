// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env::current_exe;

use image::buffer::ConvertBuffer;
use image::{DynamicImage, ImageBuffer, Rgb, Rgba, RgbaImage};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Runtime;
use tauri::Window;
use tauri_plugin_shell::ShellExt;
use tesseract_rs::TesseractAPI;
use xcap::Monitor;

use oar_ocr::prelude::*;

struct AppData {
    monitor: Mutex<Monitor>,
    ocr: Mutex<OAROCR>,
}

#[tauri::command]
fn capture<R: Runtime>(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    app: AppHandle<R>,
    window: Window<R>,
    state: tauri::State<'_, AppData>,
) -> Result<String, String> {
    let monitor = state.monitor.lock().map_err(|e| e.to_string())?;
    let image = monitor
        .capture_region(x, y, width, height)
        .map_err(|e| e.to_string())?;
    drop(monitor);

    let image = RgbaImage::from_raw(image.width(), image.height(), image.into_raw())
        .ok_or_else(|| "Failed to convert captured image to RGBA format".to_string())
        .and_then(|rgba_image| Ok(rgba_image.convert()))?;

    let ocr = state.ocr.lock().map_err(|e| e.to_string())?;
    let result = ocr
        .predict(vec![image])
        .map_err(|e| format!("OCR prediction failed: {}", e))?;
    // tess.set_image(
    //     &image,
    //     image.width() as i32,
    //     image.height() as i32,
    //     4,
    //     4 * image.width() as i32,
    // )
    // .map_err(|e| e.to_string())?;

    // let text = tess.get_utf8_text().map_err(|e| e.to_string())?;
    println!("Recognized text: {}", result.len());
    println!(
        "Region: x={}, y={}, width={}, height={}",
        x, y, width, height
    );

    Ok("".to_string())
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

            // let tess = TesseractAPI::new();

            let ocr_dir = app
                .path()
                .resource_dir()
                .expect("Failed to get resource dir")
                .join("resources")
                .join("ocr");

            // let tessdata_path = app
            //     .path()
            //     .resource_dir()
            //     .expect("Failed to get resource dir")
            //     .join("resources")
            //     .join("tessdata");
            // tess.init(tessdata_path, "jpn")
            //     .expect("Failed to initialize TesseractAPI");

            let ocr = OAROCRBuilder::new(
                ocr_dir.join("japan_pp-ocrv3_mobile_det.onnx"),
                ocr_dir.join("pp-ocrv5_mobile_rec.onnx"),
                ocr_dir.join("ppocrv5_dict.txt"),
            )
            .build()?;
            app.manage(AppData {
                monitor: Mutex::new(monitor),
                ocr: Mutex::new(ocr),
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
