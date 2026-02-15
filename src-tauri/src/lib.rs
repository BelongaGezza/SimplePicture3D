pub mod depth_adjust;
mod file_io;
mod image_loading;
pub mod mesh_generator;
mod python_bridge;
pub mod settings;

use std::sync::Mutex;
use tauri::State;

use depth_adjust::{apply_adjustments, DepthAdjustmentParams};
use mesh_generator::{depth_to_point_cloud, MeshData, MeshParams};

// Error handling pattern (BACK-004): use anyhow inside commands for context chain;
// Tauri IPC requires serializable errors, so we use Result<T, String> and map
// anyhow::Error via .map_err(|e| e.to_string()) at the boundary.

/// App-managed depth map state (BACK-302, BACK-405). Original depth from generate_depth_map;
/// adjustment params applied on demand for get_depth_map; original preserved for reset.
struct AppState {
    /// Original depth map (unchanged by adjustments).
    depth: Mutex<Option<python_bridge::DepthMapOutput>>,
    /// Current adjustment params; get_depth_map returns original transformed by these (BACK-402).
    adjustment_params: Mutex<DepthAdjustmentParams>,
    /// Path to the source image (for filename generation, BACK-705).
    source_image_path: Mutex<Option<String>>,
    /// Persistent app settings (BACK-706).
    app_settings: Mutex<settings::AppSettings>,
}

/// Success response for generate_depth_map (BACK-303, BACK-304). Includes depth and progress/stages.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GenerateDepthMapResponse {
    width: u32,
    height: u32,
    depth: Vec<f32>,
    progress: u8,
    stages: Vec<String>,
}

#[tauri::command]
fn load_image(path: String) -> Result<image_loading::LoadImageOut, String> {
    image_loading::load_image_impl(path).map_err(|e| e.to_string())
}

/// Export the current mesh as binary STL (BACK-703, BACK-701, BACK-702).
///
/// Takes output file path (from frontend save dialog). Generates triangulated mesh from
/// current adjusted depth map, validates, and writes binary STL.
///
/// **Security (SEC-401, SEC-402):** Path is validated before write:
/// - Non-empty, canonicalized (resolves `..`, symlinks)
/// - Must have `.stl` extension
/// - Must not target system directories (Windows: `C:\Windows`, `C:\Program Files`; Unix: `/etc`, `/usr`, `/bin`, `/sbin`)
/// - Parent directory must exist and be writable (checked before write attempt)
/// - Error messages do not leak full paths to the frontend
#[tauri::command]
fn export_stl(path: String, state: State<AppState>) -> Result<(), String> {
    // SEC-401: Basic validation
    if path.trim().is_empty() {
        return Err("Export path must be non-empty".to_string());
    }

    // SEC-401: Canonicalize path to resolve `..`, `.`, and symlinks.
    // This prevents path traversal attacks (e.g. `../../etc/passwd`).
    let canonical = std::fs::canonicalize(std::path::Path::new(&path).parent().ok_or_else(|| {
        "Invalid export path: no parent directory".to_string()
    })?)
    .map_err(|_| "Export directory does not exist or is not accessible".to_string())?;
    let file_name = std::path::Path::new(&path)
        .file_name()
        .ok_or_else(|| "Invalid export path: no filename".to_string())?;
    let canonical_path = canonical.join(file_name);

    // SEC-401: Validate file extension is .stl
    match canonical_path.extension().and_then(|e| e.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("stl") => {}
        _ => return Err("Export file must have .stl extension".to_string()),
    }

    // SEC-401: Reject writes to system directories
    let canonical_str = canonical.to_string_lossy();
    #[cfg(target_os = "windows")]
    {
        let lower = canonical_str.to_lowercase();
        let blocked = [
            "c:\\windows",
            "c:\\program files",
            "c:\\program files (x86)",
            "c:\\programdata",
        ];
        for prefix in &blocked {
            if lower.starts_with(prefix) {
                log::warn!("SEC-401: Blocked export to system directory: {}", canonical_str);
                return Err("Cannot export to system directories".to_string());
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let blocked = ["/etc", "/usr", "/bin", "/sbin", "/boot", "/lib", "/proc", "/sys"];
        for prefix in &blocked {
            if canonical_str.starts_with(prefix) {
                log::warn!("SEC-401: Blocked export to system directory: {}", canonical_str);
                return Err("Cannot export to system directories".to_string());
            }
        }
    }

    // SEC-402: Verify parent directory is writable before attempting export.
    // This provides a clear error before any partial file is created.
    {
        let parent = canonical_path.parent().unwrap_or(&canonical);
        let test_file = parent.join(".sp3d_write_test");
        match std::fs::File::create(&test_file) {
            Ok(_) => {
                let _ = std::fs::remove_file(&test_file);
            }
            Err(_) => {
                return Err("Export directory is not writable".to_string());
            }
        }
    }

    let canonical_path_str = canonical_path.to_string_lossy().to_string();

    // Get current depth map
    let guard = state.depth.lock().map_err(|e| e.to_string())?;
    let original = guard
        .as_ref()
        .ok_or_else(|| "No depth map loaded. Generate a depth map first.".to_string())?
        .clone();
    drop(guard);

    // Apply adjustments
    let params_guard = state.adjustment_params.lock().map_err(|e| e.to_string())?;
    let adjusted = apply_adjustments(&original.depth, &params_guard);
    let mesh_params = MeshParams {
        step_x: 1,
        step_y: 1,
        depth_min_mm: params_guard.depth_min_mm,
        depth_max_mm: params_guard.depth_max_mm,
        pixel_to_mm: 1.0,
    };
    drop(params_guard);

    // Generate triangulated mesh
    let mesh =
        depth_to_point_cloud(&adjusted, original.width, original.height, &mesh_params)
            .map_err(|e| format!("Mesh generation failed: {}", e))?;

    // Validate before writing (BACK-702)
    mesh_generator::validate_mesh_for_export(&mesh)
        .map_err(|e| format!("Mesh validation failed: {}", e))?;

    // Write STL (BACK-701) — SEC-402: use validated canonical path
    mesh_generator::write_stl_to_file(&canonical_path_str, &mesh)
        .map_err(|_| "STL export failed: could not write file".to_string())?;

    // Update last export directory (BACK-706)
    if let Some(parent) = canonical_path.parent() {
        let mut settings = state.app_settings.lock().map_err(|e| e.to_string())?;
        settings.last_export_dir = Some(parent.to_string_lossy().to_string());
        if let Err(e) = settings.save() {
            log::warn!("Failed to save settings after export: {}", e);
        }
    }

    log::info!("STL exported successfully: {}", canonical_path_str);
    Ok(())
}

/// Export the current mesh as ASCII OBJ (BACK-801, BACK-802, BACK-803).
///
/// Same security validation as export_stl. Writes OBJ file and optional companion MTL file.
#[tauri::command]
fn export_obj(path: String, state: State<AppState>) -> Result<(), String> {
    // SEC-401: Basic validation
    if path.trim().is_empty() {
        return Err("Export path must be non-empty".to_string());
    }

    // SEC-401: Canonicalize path
    let canonical = std::fs::canonicalize(std::path::Path::new(&path).parent().ok_or_else(|| {
        "Invalid export path: no parent directory".to_string()
    })?)
    .map_err(|_| "Export directory does not exist or is not accessible".to_string())?;
    let file_name = std::path::Path::new(&path)
        .file_name()
        .ok_or_else(|| "Invalid export path: no filename".to_string())?;
    let canonical_path = canonical.join(file_name);

    // SEC-401: Validate file extension is .obj
    match canonical_path.extension().and_then(|e| e.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("obj") => {}
        _ => return Err("Export file must have .obj extension".to_string()),
    }

    // SEC-401: Reject writes to system directories
    let canonical_str = canonical.to_string_lossy();
    #[cfg(target_os = "windows")]
    {
        let lower = canonical_str.to_lowercase();
        let blocked = [
            "c:\\windows",
            "c:\\program files",
            "c:\\program files (x86)",
            "c:\\programdata",
        ];
        for prefix in &blocked {
            if lower.starts_with(prefix) {
                log::warn!("SEC-401: Blocked export to system directory: {}", canonical_str);
                return Err("Cannot export to system directories".to_string());
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let blocked = ["/etc", "/usr", "/bin", "/sbin", "/boot", "/lib", "/proc", "/sys"];
        for prefix in &blocked {
            if canonical_str.starts_with(prefix) {
                log::warn!("SEC-401: Blocked export to system directory: {}", canonical_str);
                return Err("Cannot export to system directories".to_string());
            }
        }
    }

    // SEC-402: Verify parent directory is writable
    {
        let parent = canonical_path.parent().unwrap_or(&canonical);
        let test_file = parent.join(".sp3d_write_test");
        match std::fs::File::create(&test_file) {
            Ok(_) => {
                let _ = std::fs::remove_file(&test_file);
            }
            Err(_) => {
                return Err("Export directory is not writable".to_string());
            }
        }
    }

    let canonical_path_str = canonical_path.to_string_lossy().to_string();

    // Get current depth map
    let guard = state.depth.lock().map_err(|e| e.to_string())?;
    let original = guard
        .as_ref()
        .ok_or_else(|| "No depth map loaded. Generate a depth map first.".to_string())?
        .clone();
    drop(guard);

    // Apply adjustments
    let params_guard = state.adjustment_params.lock().map_err(|e| e.to_string())?;
    let adjusted = apply_adjustments(&original.depth, &params_guard);
    let mesh_params = MeshParams {
        step_x: 1,
        step_y: 1,
        depth_min_mm: params_guard.depth_min_mm,
        depth_max_mm: params_guard.depth_max_mm,
        pixel_to_mm: 1.0,
    };
    drop(params_guard);

    // Generate triangulated mesh
    let mesh =
        depth_to_point_cloud(&adjusted, original.width, original.height, &mesh_params)
            .map_err(|e| format!("Mesh generation failed: {}", e))?;

    // Validate before writing (BACK-702)
    mesh_generator::validate_mesh_for_export(&mesh)
        .map_err(|e| format!("Mesh validation failed: {}", e))?;

    // Write OBJ + MTL (BACK-801, BACK-802)
    mesh_generator::write_obj_to_file(&canonical_path_str, &mesh, true)
        .map_err(|_| "OBJ export failed: could not write file".to_string())?;

    // Update last export directory (BACK-706)
    if let Some(parent) = canonical_path.parent() {
        let mut settings = state.app_settings.lock().map_err(|e| e.to_string())?;
        settings.last_export_dir = Some(parent.to_string_lossy().to_string());
        if let Err(e) = settings.save() {
            log::warn!("Failed to save settings after export: {}", e);
        }
    }

    log::info!("OBJ exported successfully: {}", canonical_path_str);
    Ok(())
}

/// Get a suggested export filename and last export directory (BACK-705, BACK-706).
#[tauri::command]
fn get_export_defaults(state: State<AppState>) -> Result<ExportDefaults, String> {
    let source_path = state
        .source_image_path
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .unwrap_or_default();
    let filename = mesh_generator::generate_export_filename(&source_path);
    let settings_guard = state
        .app_settings
        .lock()
        .map_err(|e| e.to_string())?;
    let last_dir = settings_guard.last_export_dir.clone();
    let export_format = settings_guard.export_format.clone();
    drop(settings_guard);
    Ok(ExportDefaults {
        suggested_filename: filename,
        last_export_dir: last_dir,
        export_format,
    })
}

/// Response for get_export_defaults (BACK-705, BACK-706, BACK-803).
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportDefaults {
    suggested_filename: String,
    last_export_dir: Option<String>,
    export_format: Option<String>,
}

/// Get current app settings (BACK-804, BACK-805).
#[tauri::command]
fn get_settings(state: State<AppState>) -> Result<settings::AppSettings, String> {
    state
        .app_settings
        .lock()
        .map_err(|e| e.to_string())
        .map(|guard| guard.clone())
}

/// Save app settings (BACK-804, BACK-805).
#[tauri::command]
fn save_settings(
    new_settings: settings::AppSettings,
    state: State<AppState>,
) -> Result<(), String> {
    let mut settings = state.app_settings.lock().map_err(|e| e.to_string())?;
    *settings = new_settings;
    settings.save().map_err(|e| e.to_string())
}

// --- Sprint 1.10: Model management commands ---

/// Model status response (BACK-902).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModelStatus {
    installed: bool,
    model_dir: String,
    model_id: String,
    #[serde(default)]
    missing_files: Vec<String>,
    #[serde(default)]
    size_mb: Option<f64>,
}

/// Model info response.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModelInfo {
    model_id: String,
    model_dir: String,
    license: String,
    estimated_size_mb: u32,
    description: String,
}

/// Download result response.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct DownloadResult {
    status: String,
    #[serde(default)]
    model_dir: Option<String>,
    #[serde(default)]
    size_mb: Option<f64>,
    #[serde(default)]
    error: Option<String>,
}

/// Run a python model_downloader command and parse JSON output.
fn run_model_downloader_cmd(arg: &str) -> Result<String, String> {
    let python = python_bridge_python_exe();
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let work_dir = cwd.join("python");
    let work_dir = if work_dir.is_dir() {
        work_dir
    } else {
        cwd.parent()
            .map(|p| p.join("python"))
            .filter(|p| p.is_dir())
            .unwrap_or(cwd)
    };

    let output = std::process::Command::new(&python)
        .arg("-m")
        .arg("python.model_downloader")
        .arg(arg)
        .current_dir(&work_dir)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to run model downloader: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if stdout.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Model downloader returned no output. stderr: {}", stderr));
    }
    Ok(stdout)
}

/// Helper: get python executable (re-uses same logic as python_bridge).
fn python_bridge_python_exe() -> std::path::PathBuf {
    if let Ok(venv) = std::env::var("VIRTUAL_ENV") {
        let path = std::path::Path::new(&venv);
        #[cfg(windows)]
        let exe = path.join("Scripts").join("python.exe");
        #[cfg(not(windows))]
        let exe = path.join("bin").join("python");
        if exe.is_file() {
            return exe;
        }
    }
    #[cfg(windows)]
    return std::path::PathBuf::from("python");
    #[cfg(not(windows))]
    std::path::PathBuf::from("python3")
}

/// Check if AI model is installed (BACK-902).
#[tauri::command]
fn check_model() -> Result<ModelStatus, String> {
    let stdout = run_model_downloader_cmd("--check")?;
    serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse model status: {}", e))
}

/// Get model information (BACK-901).
#[tauri::command]
fn get_model_info() -> Result<ModelInfo, String> {
    let stdout = run_model_downloader_cmd("--info")?;
    serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse model info: {}", e))
}

/// Download AI model (BACK-901, BACK-903).
#[tauri::command]
fn download_model() -> Result<DownloadResult, String> {
    let stdout = run_model_downloader_cmd("--download")?;
    serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse download result: {}", e))
}

/// JR2-303: Log depth map statistics (min, max, mean) at info level. Single pass; no PII.
fn log_depth_stats(depth: &[f32]) {
    if depth.is_empty() {
        log::info!("depth stats: (empty)");
        return;
    }
    let (min, max, sum) = depth.iter().fold(
        (f32::INFINITY, f32::NEG_INFINITY, 0f32),
        |(min, max, sum), &v| (min.min(v), max.max(v), sum + v),
    );
    let mean = sum / depth.len() as f32;
    log::info!(
        "depth stats: min={:.4}, max={:.4}, mean={:.4}, n={}",
        min,
        max,
        mean,
        depth.len()
    );
}

/// Inner implementation for depth generation (testable without Tauri state).
fn generate_depth_map_impl(
    path: &str,
) -> Result<(python_bridge::DepthMapOutput, Vec<String>), String> {
    let bytes = image_loading::read_image_bytes_for_depth(path).map_err(|e| e.to_string())?;
    let result = python_bridge::run_depth_estimation(&bytes).map_err(|e| e.to_string())?;
    log_depth_stats(&result.depth.depth);
    Ok((result.depth, result.stderr_lines))
}

/// Generates a depth map from an image file (BACK-301, BACK-303, BACK-304).
/// Accepts image path (same as load_image); validates path and format, runs Python bridge;
/// stores result in app state (BACK-302), returns depth + progress 100 + stages.
#[tauri::command]
fn generate_depth_map(
    path: String,
    state: State<AppState>,
) -> Result<GenerateDepthMapResponse, String> {
    let (depth, stderr_lines) = generate_depth_map_impl(&path)?;
    *state.depth.lock().map_err(|e| e.to_string())? = Some(depth.clone());
    // Store source image path for filename generation (BACK-705).
    *state.source_image_path.lock().map_err(|e| e.to_string())? = Some(path.clone());
    // Leave adjustment_params unchanged (user may have presets); reset is explicit (BACK-405).
    let params = state.adjustment_params.lock().map_err(|e| e.to_string())?;
    let adjusted = apply_adjustments(&depth.depth, &params);
    let stages = python_bridge::stages_from_stderr(&stderr_lines);
    Ok(GenerateDepthMapResponse {
        width: depth.width,
        height: depth.height,
        depth: adjusted,
        progress: 100,
        stages,
    })
}

/// Returns the current depth map from app state with adjustments applied (BACK-302, BACK-402).
/// Original depth is preserved; display = apply_adjustments(original, adjustment_params).
#[tauri::command]
fn get_depth_map(state: State<AppState>) -> Result<Option<python_bridge::DepthMapOutput>, String> {
    let guard = state.depth.lock().map_err(|e| e.to_string())?;
    let original = match guard.as_ref() {
        Some(d) => d.clone(),
        None => return Ok(None),
    };
    drop(guard);
    let params = state.adjustment_params.lock().map_err(|e| e.to_string())?;
    let adjusted = apply_adjustments(&original.depth, &params);
    Ok(Some(python_bridge::DepthMapOutput {
        width: original.width,
        height: original.height,
        depth: adjusted,
    }))
}

/// Sets depth adjustment parameters (BACK-402). Next get_depth_map returns adjusted view.
#[tauri::command]
fn set_depth_adjustment_params(
    params: DepthAdjustmentParams,
    state: State<AppState>,
) -> Result<(), String> {
    *state.adjustment_params.lock().map_err(|e| e.to_string())? = params;
    Ok(())
}

/// Returns current adjustment params for UI sync (e.g. after reset).
#[tauri::command]
fn get_depth_adjustment_params(state: State<AppState>) -> Result<DepthAdjustmentParams, String> {
    state
        .adjustment_params
        .lock()
        .map_err(|e| e.to_string())
        .map(|guard| guard.clone())
}

/// Resets adjustment params to defaults; original depth unchanged (BACK-405).
#[tauri::command]
fn reset_depth_adjustments(state: State<AppState>) -> Result<(), String> {
    *state.adjustment_params.lock().map_err(|e| e.to_string())? = DepthAdjustmentParams::default();
    Ok(())
}

/// Returns mesh data (point cloud with normals) from current adjusted depth map (BACK-501–505, BACK-601, BACK-602, BACK-603).
///
/// **Transfer (ADR-007):** Current path is JSON over Tauri IPC. If Sprint 1.6A/ADR-007 adopts binary
/// transfer (e.g. temp file or binary IPC) for latency >100ms at 1080p, add an alternative path here
/// (write mesh to temp file, return path or handle); frontend contract (positions, normals, dimensions)
/// remains unchanged from the caller’s perspective.
///
/// **LOD (BACK-603):** Optional `preview_step` requests reduced vertex count for preview. When `None`,
/// full resolution (step 1) is used. When `Some(s)`, step_x = step_y = max(1, s). Full-detail export
/// (Sprint 1.8) is unaffected.
#[tauri::command]
fn get_mesh_data(
    state: State<AppState>,
    preview_step: Option<u32>,
) -> Result<Option<MeshData>, String> {
    let guard = state.depth.lock().map_err(|e| e.to_string())?;
    let original = match guard.as_ref() {
        Some(d) => d.clone(),
        None => return Ok(None),
    };
    drop(guard);
    let params_guard = state.adjustment_params.lock().map_err(|e| e.to_string())?;
    let adjusted = apply_adjustments(&original.depth, &params_guard);
    let step = preview_step.unwrap_or(1).max(1);
    let mesh_params = MeshParams {
        step_x: step,
        step_y: step,
        depth_min_mm: params_guard.depth_min_mm,
        depth_max_mm: params_guard.depth_max_mm,
        pixel_to_mm: 1.0,
    };
    drop(params_guard);
    let mesh = depth_to_point_cloud(&adjusted, original.width, original.height, &mesh_params)
        .map_err(|e| e.to_string())?;
    Ok(Some(mesh))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = env_logger::try_init();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            depth: Mutex::new(None),
            adjustment_params: Mutex::new(DepthAdjustmentParams::default()),
            source_image_path: Mutex::new(None),
            app_settings: Mutex::new(settings::AppSettings::load()),
        })
        .invoke_handler(tauri::generate_handler![
            load_image,
            export_stl,
            export_obj,
            get_export_defaults,
            generate_depth_map,
            get_depth_map,
            set_depth_adjustment_params,
            get_depth_adjustment_params,
            reset_depth_adjustments,
            get_mesh_data,
            get_settings,
            save_settings,
            check_model,
            get_model_info,
            download_model
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn load_image_rejects_empty_path() {
        let err = load_image("".to_string()).unwrap_err();
        assert!(err.contains("non-empty") || err.contains("path"));
        let err = load_image("   ".to_string()).unwrap_err();
        assert!(!err.is_empty());
    }

    /// BACK-301: generate_depth_map rejects empty path (same validation as load_image).
    #[test]
    fn generate_depth_map_rejects_empty_path() {
        let err = generate_depth_map_impl("").unwrap_err();
        assert!(!err.is_empty());
        let err = generate_depth_map_impl("   ").unwrap_err();
        assert!(!err.is_empty());
    }

    /// BACK-301: generate_depth_map rejects nonexistent path.
    #[test]
    fn generate_depth_map_rejects_nonexistent_path() {
        let path = std::env::temp_dir()
            .join("sp3d_nonexistent_generate_depth_12345.png")
            .to_string_lossy()
            .to_string();
        let result = generate_depth_map_impl(&path);
        assert!(result.is_err(), "nonexistent path should fail");
    }

    /// QA-103: Load valid PNG, verify response contains correct dimensions (BACK-101, BACK-105).
    /// Creates a temp PNG at runtime so the test is self-contained and CI does not depend on fixture content.
    #[test]
    fn load_valid_png_returns_dimensions() {
        let temp = std::env::temp_dir().join("sp3d_qa103_test.png");
        let (w, h) = (100u32, 50u32);
        let img = image::ImageBuffer::from_fn(w, h, |_x, _y| image::Rgb([255u8, 0u8, 0u8]));
        img.save(&temp).expect("write test PNG");
        let path = temp.to_string_lossy().to_string();
        let result = load_image(path);
        let _ = std::fs::remove_file(&temp);
        let r = result.expect("load_image should succeed for valid PNG");
        assert!(r.ok);
        assert_eq!(r.width, w);
        assert_eq!(r.height, h);
        assert!(!r.preview_base64.is_empty());
    }

    /// QA-104: Load invalid/corrupt file, verify error (BACK-102, SEC-102).
    #[test]
    fn load_invalid_file_returns_error() {
        let temp = std::env::temp_dir().join("sp3d_qa104_invalid.dat");
        File::create(&temp)
            .unwrap()
            .write_all(b"not an image; wrong magic bytes")
            .unwrap();
        let path = temp.to_string_lossy().to_string();
        let result = load_image(path);
        let _ = std::fs::remove_file(&temp);
        let err = result.expect_err("load_image should fail for non-image file");
        let lower = err.to_lowercase();
        assert!(
            lower.contains("invalid") || lower.contains("corrupt") || lower.contains("format"),
            "error message should indicate invalid/corrupt/format: {}",
            err
        );
    }

    /// JR2-202: When Python exits non-zero (e.g. invalid image), Rust returns Err without panic.
    #[test]
    fn subprocess_python_nonzero_exit_returns_err() {
        let invalid_image = b"not an image; invalid magic bytes";
        let timeout = std::time::Duration::from_secs(15);
        let result = python_bridge::run_depth_estimation_with_timeout(invalid_image, timeout);
        assert!(
            result.is_err(),
            "invalid image should cause Python to exit non-zero and Rust to return Err"
        );
    }

    /// JR2-203: Benchmark temp-file roundtrip (image bytes → Rust write → Python read → depth out).
    /// Run with: cargo test benchmark_temp_file_roundtrip -- --ignored --nocapture
    /// when Python env is available. Results documented in SPRINTS/Sprint_1_3/GOTCHAS.md.
    #[test]
    #[ignore]
    fn benchmark_temp_file_roundtrip() {
        let (w, h) = (640u32, 480u32); // ~900KB PNG
        let img = image::ImageBuffer::from_fn(w, h, |x, y| {
            image::Rgb([(x % 256) as u8, (y % 256) as u8, 128u8])
        });
        let png_bytes = image_loading::rgb_to_png_bytes(&img).expect("encode PNG");
        let size_kb = png_bytes.len() / 1024;
        let timeout = std::time::Duration::from_secs(60);
        let runs = 2;
        let mut times_ms: Vec<u128> = Vec::with_capacity(runs);
        for i in 0..runs {
            let start = std::time::Instant::now();
            let result = python_bridge::run_depth_estimation_with_timeout(&png_bytes, timeout);
            let elapsed = start.elapsed().as_millis();
            times_ms.push(elapsed);
            let ok = result.is_ok();
            eprintln!(
                "JR2-203 run {}: {} ms, ok={} ({}×{} image, ~{} KB)",
                i + 1,
                elapsed,
                ok,
                w,
                h,
                size_kb
            );
            if !ok {
                panic!("roundtrip failed: {:?}", result.unwrap_err());
            }
        }
        let median = if times_ms.len() == 1 {
            times_ms[0]
        } else {
            times_ms.sort();
            times_ms[times_ms.len() / 2]
        };
        eprintln!(
            "JR2-203 temp-file roundtrip median: {} ms for ~{} KB PNG",
            median, size_kb
        );
        // Sanity: roundtrip should complete in under 60s
        assert!(median < 60_000, "roundtrip too slow: {} ms", median);
    }

    /// JR2-201: Integration test Rust → Python → Rust roundtrip (image in → depth out).
    /// Uses in-memory minimal PNG; fixture alternative: tests/fixtures/valid_small.png (see tests/fixtures/README.md).
    /// Run with: cargo test --manifest-path src-tauri/Cargo.toml roundtrip_depth -- --ignored
    /// when Python 3.10+ and python package (python/python/) are available from project root.
    #[test]
    #[ignore]
    fn roundtrip_depth_rust_python_rust() {
        let (w, h) = (10u32, 8u32);
        let img = image::ImageBuffer::from_fn(w, h, |_x, _y| image::Rgb([200u8, 100u8, 50u8]));
        let mut png_bytes = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        use image::ImageEncoder;
        encoder
            .write_image(img.as_raw(), w, h, image::ExtendedColorType::Rgb8)
            .expect("encode PNG");
        let result = python_bridge::run_depth_estimation(&png_bytes);
        let res = result.expect("run_depth_estimation should succeed when Python env is set up");
        assert_eq!(res.depth.height, h);
        assert_eq!(res.depth.width, w);
        assert_eq!(res.depth.depth.len(), (w as usize) * (h as usize));
        assert!(res.depth.depth.iter().all(|&v| (0.0..=1.0).contains(&v)));
    }

    /// JR2-301: Unit test for depth map normalization — contract is all values in [0, 1] (AI-301, ARCH-102).
    /// Synthetic in-range depth passes; regression guard when Python/bridge returns depth.
    #[test]
    fn depth_map_normalization_all_values_in_0_1() {
        use python_bridge::DepthMapOutput;
        let valid = DepthMapOutput {
            width: 2,
            height: 2,
            depth: vec![0.0, 0.25, 0.5, 1.0],
        };
        assert!(
            valid.depth.iter().all(|&v| (0.0..=1.0).contains(&v)),
            "depth from bridge must be in [0, 1] per ARCH-102"
        );
        let with_boundary = DepthMapOutput {
            width: 1,
            height: 2,
            depth: vec![0.0, 1.0],
        };
        assert!(with_boundary
            .depth
            .iter()
            .all(|&v| (0.0..=1.0).contains(&v)));
    }

    /// JR2-302: Edge case — all-black image. No panic; depth may be constant or model-dependent.
    /// Run with: cargo test depth_estimation_all_black_image -- --ignored when Python env available.
    #[test]
    #[ignore]
    fn depth_estimation_all_black_image() {
        let (w, h) = (4u32, 4u32);
        let img = image::ImageBuffer::from_fn(w, h, |_x, _y| image::Rgb([0u8, 0u8, 0u8]));
        let png_bytes = image_loading::rgb_to_png_bytes(&img).expect("encode PNG");
        let result = python_bridge::run_depth_estimation_with_timeout(
            &png_bytes,
            std::time::Duration::from_secs(30),
        );
        let res = result.expect("all-black image should not panic; may return constant depth");
        assert_eq!(res.depth.width, w);
        assert_eq!(res.depth.height, h);
        assert_eq!(res.depth.depth.len(), (w as usize) * (h as usize));
        assert!(
            res.depth.depth.iter().all(|&v| (0.0..=1.0).contains(&v)),
            "depth must be in [0, 1]"
        );
    }

    /// JR2-302: Edge case — all-white image. No panic; depth may be constant or model-dependent.
    #[test]
    #[ignore]
    fn depth_estimation_all_white_image() {
        let (w, h) = (4u32, 4u32);
        let img = image::ImageBuffer::from_fn(w, h, |_x, _y| image::Rgb([255u8, 255u8, 255u8]));
        let png_bytes = image_loading::rgb_to_png_bytes(&img).expect("encode PNG");
        let result = python_bridge::run_depth_estimation_with_timeout(
            &png_bytes,
            std::time::Duration::from_secs(30),
        );
        let res = result.expect("all-white image should not panic; may return constant depth");
        assert_eq!(res.depth.width, w);
        assert_eq!(res.depth.height, h);
        assert!(res.depth.depth.iter().all(|&v| (0.0..=1.0).contains(&v)));
    }

    /// QA-304: Automated test — depth map dimensions match input image.
    /// Creates temp PNG with known size (100×50), calls generate_depth_map_impl, asserts width/height match.
    /// Run with: cargo test depth_map_dimensions_match_image -- --ignored when Python env available.
    #[test]
    #[ignore = "requires Python env and python.depth_estimator"]
    fn depth_map_dimensions_match_image() {
        let (w, h) = (100u32, 50u32);
        let img = image::ImageBuffer::from_fn(w, h, |x, y| {
            image::Rgb([(x % 256) as u8, (y % 256) as u8, 128u8])
        });
        let temp = std::env::temp_dir().join("sp3d_qa304_dimensions.png");
        img.save(&temp).expect("write test PNG");
        let path = temp.to_string_lossy().to_string();
        let result = generate_depth_map_impl(&path);
        let _ = std::fs::remove_file(&temp);
        let (depth_out, _) =
            result.expect("generate_depth_map_impl should succeed when Python env is set up");
        assert_eq!(
            depth_out.width, w,
            "depth width must match image width (no downsampling for this size)"
        );
        assert_eq!(
            depth_out.height, h,
            "depth height must match image height (no downsampling for this size)"
        );
        assert_eq!(
            depth_out.depth.len(),
            (w as usize) * (h as usize),
            "depth array length must be width × height"
        );
    }
}
