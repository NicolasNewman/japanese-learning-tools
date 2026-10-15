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

#[cfg(windows)]
mod win_mpv {
    use std::ffi::c_void;

    type Hwnd = *mut c_void;

    #[link(name = "user32")]
    extern "system" {
        fn EnumChildWindows(
            parent: Hwnd,
            func: unsafe extern "system" fn(Hwnd, isize) -> i32,
            lparam: isize,
        ) -> i32;
        fn GetClassNameW(hwnd: Hwnd, buf: *mut u16, max: i32) -> i32;
        fn EnableWindow(hwnd: Hwnd, enable: i32) -> i32;
        fn SetFocus(hwnd: Hwnd) -> Hwnd;
        fn GetFocus() -> Hwnd;
        fn GetCurrentThreadId() -> u32;
        fn GetForegroundWindow() -> Hwnd;
        fn IsWindow(hwnd: Hwnd) -> i32;
        fn GetWindowThreadProcessId(hwnd: Hwnd, pid: *mut u32) -> u32;
        fn AttachThreadInput(attach: u32, attach_to: u32, enable: i32) -> i32;
        fn SetWindowPos(
            hwnd: Hwnd,
            after: Hwnd,
            x: i32,
            y: i32,
            cx: i32,
            cy: i32,
            flags: u32,
        ) -> i32;
    }

    const SWP_NOSIZE: u32 = 0x0001;
    const SWP_NOMOVE: u32 = 0x0002;
    const SWP_NOACTIVATE: u32 = 0x0010;
    const HWND_TOP: Hwnd = std::ptr::null_mut();

    unsafe extern "system" fn find_mpv(hwnd: Hwnd, out: isize) -> i32 {
        let mut buf = [0u16; 16];
        let len = GetClassNameW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
        if len > 0 && String::from_utf16_lossy(&buf[..len as usize]) == "mpv" {
            *(out as *mut Hwnd) = hwnd;
            return 0;
        }
        1
    }

    pub fn is_window(hwnd: isize) -> bool {
        unsafe { IsWindow(hwnd as Hwnd) != 0 }
    }

    /// Enables and raises mpv's child window, and gives it keyboard focus while
    /// the parent window is in the foreground.
    pub fn prepare_mpv_child(parent: isize) {
        unsafe {
            let mut mpv: Hwnd = std::ptr::null_mut();
            EnumChildWindows(parent as Hwnd, find_mpv, &mut mpv as *mut Hwnd as isize);
            if mpv.is_null() {
                return;
            }

            // mpv disables its own child window in --wid mode, so it never receives input.
            EnableWindow(mpv, 1);
            SetWindowPos(
                mpv,
                HWND_TOP,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );

            if GetForegroundWindow() != parent as Hwnd {
                return;
            }

            // mpv runs its window on another thread, and WebView2 keeps keyboard
            // focus; SetFocus only works across threads with attached input.
            let mpv_thread = GetWindowThreadProcessId(mpv, std::ptr::null_mut());
            let this_thread = GetCurrentThreadId();
            AttachThreadInput(this_thread, mpv_thread, 1);
            if GetFocus() != mpv {
                SetFocus(mpv);
            }
            AttachThreadInput(this_thread, mpv_thread, 0);
        }
    }
}

/// On Windows, mpv disables its embedded child window (`--wid` mode) and
/// WebView2 keeps keyboard focus, so mpv scripts (ModernZ, mpvacious) get no
/// input. Watch the window and keep mpv's child enabled, raised and focused
/// until the window is closed.
#[tauri::command]
fn raise_mpv_window<R: Runtime>(app: AppHandle<R>, label: String) -> Result<(), String> {
    #[cfg(windows)]
    {
        static WATCHED: Mutex<Vec<isize>> = Mutex::new(Vec::new());

        let window = app
            .get_webview_window(&label)
            .ok_or_else(|| format!("Window '{}' not found", label))?;
        let hwnd = window.hwnd().map_err(|e| e.to_string())?.0 as isize;

        let mut watched = WATCHED.lock().unwrap();
        if !watched.contains(&hwnd) {
            watched.push(hwnd);
            std::thread::spawn(move || {
                while win_mpv::is_window(hwnd) {
                    win_mpv::prepare_mpv_child(hwnd);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                WATCHED.lock().unwrap().retain(|h| *h != hwnd);
            });
        }
    }
    #[cfg(not(windows))]
    let _ = (app, label);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_libmpv::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_cors_fetch::init())
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
            capture,
            raise_mpv_window
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
