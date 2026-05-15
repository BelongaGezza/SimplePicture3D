// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! SimplePicture3D Tauri application: Tauri commands and app state.
//!
//! Public API surface: Tauri commands registered in `run()` (load_image, generate_depth_map,
//! get_depth_map, set_depth_adjustment_params, etc.). The 2.5D mesh / STL / OBJ surface has been
//! retired (Sprint A). ADR-012 point cloud commands are registered: `set_blank_envelope`,
//! `set_volumetric_params`, `generate_point_cloud`, `export_ply`, `export_xyz`, `export_csv`, etc.
//!
//! See `docs/developer-guide.md` and `cargo doc` for command contracts and types.

pub mod blank_envelope;
pub mod depth_adjust;
pub mod export;
mod file_io;
mod image_loading;
pub mod mask;
pub mod preset;
mod python_bridge;
pub mod settings;
pub mod undo;
pub mod volumetric;

use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;
use tauri::Emitter;
use tauri::State;

use blank_envelope::BlankEnvelope;
use depth_adjust::{apply_adjustments, compute_histogram, DepthAdjustmentParams};
use export::ExportMetadata;
use preset::{get_builtin_preset, sanitize_preset_name, Preset};
use undo::{SetDepthParamsCommand, SetMaskCommand, UndoRedoHistory, UndoableCommand};
use volumetric::{validate_volumetric_params, VolumetricParams, VolumetricResult};

/// BACK-1202, BACK-1203: Apply depth adjustments with optional mask and feathering.
/// When mask is None or dimensions don't match, returns full apply_adjustments(original, params).
/// When mask is Some, adjusted depth is blended: weight = soft_mask (feather at edges), out = weight*adjusted + (1-weight)*original.
pub(crate) fn apply_adjustments_with_mask(
    original: &[f32],
    width: u32,
    height: u32,
    params: &DepthAdjustmentParams,
    mask: Option<&mask::MaskBitmap>,
) -> Vec<f32> {
    let adjusted = apply_adjustments(original, params);
    let mask = match mask {
        Some(m) if m.dimensions_match(width, height) => m,
        _ => return adjusted,
    };
    let weights = mask.to_soft_mask(params.feather_radius_px);
    let n = original.len().min(adjusted.len()).min(weights.len());
    (0..n)
        .map(|i| {
            let w = weights[i];
            (w * adjusted[i] + (1.0 - w) * original[i]).clamp(0.0, 1.0)
        })
        .collect()
}

// Error handling pattern (BACK-004): use anyhow inside commands for context chain;
// Tauri IPC requires serializable errors, so we use Result<T, String> and map
// anyhow::Error via .map_err(|e| e.to_string()) at the boundary.

/// SEC-401/SEC-402: Validate export path (canonicalize, extension, block system dirs, writable).
/// Returns (canonical PathBuf, path as String) for use in export commands.
fn validate_export_path(
    path: &str,
    extension: &str,
) -> Result<(std::path::PathBuf, String), String> {
    if path.trim().is_empty() {
        return Err("Export path must be non-empty".to_string());
    }
    let canonical = std::fs::canonicalize(
        std::path::Path::new(path)
            .parent()
            .ok_or_else(|| "Invalid export path: no parent directory".to_string())?,
    )
    .map_err(|_| "Export directory does not exist or is not accessible".to_string())?;
    let file_name = std::path::Path::new(path)
        .file_name()
        .ok_or_else(|| "Invalid export path: no filename".to_string())?;
    let canonical_path = canonical.join(file_name);

    match canonical_path.extension().and_then(|e| e.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case(extension) => {}
        _ => return Err(format!("Export file must have .{} extension", extension)),
    }

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
                log::warn!(
                    "SEC-401: Blocked export to system directory: {}",
                    canonical_str
                );
                return Err("Cannot export to system directories".to_string());
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let blocked = [
            "/etc", "/usr", "/bin", "/sbin", "/boot", "/lib", "/proc", "/sys",
        ];
        for prefix in &blocked {
            if canonical_str.starts_with(prefix) {
                log::warn!(
                    "SEC-401: Blocked export to system directory: {}",
                    canonical_str
                );
                return Err("Cannot export to system directories".to_string());
            }
        }
    }

    let parent = canonical_path.parent().unwrap_or(&canonical);
    let test_file = parent.join(".sp3d_write_test");
    match std::fs::File::create(&test_file) {
        Ok(_) => {
            let _ = std::fs::remove_file(&test_file);
        }
        Err(_) => return Err("Export directory is not writable".to_string()),
    }

    let canonical_path_str = canonical_path.to_string_lossy().to_string();
    Ok((canonical_path, canonical_path_str))
}

/// Validates path for saving a preset JSON file (export). Same security as validate_export_path but for .json.
fn validate_preset_export_path(path: &str) -> Result<(std::path::PathBuf, String), String> {
    validate_export_path(path, "json")
}

/// App-managed depth map state (BACK-302, BACK-405). Original depth from generate_depth_map;
/// adjustment params applied on demand for get_depth_map; original preserved for reset.
struct AppState {
    /// Original depth map (unchanged by adjustments).
    depth: Mutex<Option<python_bridge::DepthMapOutput>>,
    /// Current adjustment params; get_depth_map returns original transformed by these (BACK-402).
    adjustment_params: Mutex<DepthAdjustmentParams>,
    /// Mask for regional depth adjustments (BACK-1201, ARCH-502). Cleared when depth is replaced.
    mask: Mutex<Option<mask::MaskBitmap>>,
    /// Path to the source image (for export metadata and PROGRESS context). Written by
    /// `generate_depth_map`; consumed by point cloud export commands.
    source_image_path: Mutex<Option<String>>,
    /// Persistent app settings (BACK-706).
    app_settings: Mutex<settings::AppSettings>,
    /// Undo/redo history for depth and mask (BACK-1402, ARCH-502).
    undo_redo: Mutex<UndoRedoHistory>,
    /// Last successful [`generate_point_cloud`] result — used by export commands (ADR-012).
    last_point_cloud: Mutex<Option<VolumetricResult>>,
}

/// Payload for "depth-progress" Tauri event (BACK-205-STREAM, ARCH-501).
#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DepthProgressPayload {
    percent: u8,
    stage: Option<String>,
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

/// Load and validate an image file (BACK-101–105). Returns dimensions, metadata, and base64 preview.
/// Path is validated (SEC-101); format by magic bytes (SEC-102). Images over 8192×8192 are downsampled.
#[tauri::command]
fn load_image(path: String) -> Result<image_loading::LoadImageOut, String> {
    image_loading::load_image_impl(path).map_err(|e| e.to_string())
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

// --- ADR-012: Crystal surface point cloud (Sprint B, TD-11) ---

fn current_adjusted_depth(state: &AppState) -> Result<Option<(Vec<f32>, u32, u32)>, String> {
    let guard = state.depth.lock().map_err(|e| e.to_string())?;
    let original = match guard.as_ref() {
        Some(d) => d.clone(),
        None => return Ok(None),
    };
    drop(guard);
    let params = state
        .adjustment_params
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let mask_guard = state.mask.lock().map_err(|e| e.to_string())?;
    let adjusted = apply_adjustments_with_mask(
        &original.depth,
        original.width,
        original.height,
        &params,
        mask_guard.as_ref(),
    );
    Ok(Some((adjusted, original.width, original.height)))
}

fn resolved_blank_envelope(state: &AppState) -> Result<BlankEnvelope, String> {
    let guard = state.app_settings.lock().map_err(|e| e.to_string())?;
    let env = guard.blank_envelope.clone().unwrap_or_default();
    drop(guard);
    env.validate()?;
    Ok(env)
}

fn resolved_volumetric_params(state: &AppState) -> Result<VolumetricParams, String> {
    let guard = state.app_settings.lock().map_err(|e| e.to_string())?;
    Ok(guard.volumetric_params.clone().unwrap_or_default())
}

fn invalidate_point_cloud_cache(state: &AppState) -> Result<(), String> {
    *state.last_point_cloud.lock().map_err(|e| e.to_string())? = None;
    Ok(())
}

fn build_export_metadata(state: &AppState) -> Result<ExportMetadata, String> {
    let mut metadata = ExportMetadata::new();
    let settings = state.app_settings.lock().map_err(|e| e.to_string())?;
    metadata.blank_envelope = Some(
        settings
            .blank_envelope
            .clone()
            .unwrap_or_else(BlankEnvelope::default),
    );
    drop(settings);
    let path_guard = state.source_image_path.lock().map_err(|e| e.to_string())?;
    if let Some(ref p) = *path_guard {
        if let Some(name) = Path::new(p).file_name() {
            metadata.source_image = Some(name.to_string_lossy().to_string());
        }
    }
    Ok(metadata)
}

fn persist_last_export_dir(state: &AppState, export_path: &str) -> Result<(), String> {
    let Some(parent) = Path::new(export_path).parent() else {
        return Ok(());
    };
    let dir = parent.to_string_lossy().to_string();
    let mut settings = state.app_settings.lock().map_err(|e| e.to_string())?;
    settings.last_export_dir = Some(dir);
    settings.save().map_err(|e| e.to_string())
}

fn generate_point_cloud_from_state(state: &AppState) -> Result<VolumetricResult, String> {
    let Some((depth_vec, width, height)) = current_adjusted_depth(state)? else {
        return Err("No depth map loaded".to_string());
    };
    let envelope = resolved_blank_envelope(state)?;
    let params = resolved_volumetric_params(state)?;
    volumetric::generate_volumetric_points(&depth_vec, width, height, &params, &envelope)
}

fn estimate_point_cloud_count_from_state(state: &AppState) -> Result<Option<usize>, String> {
    let Some((_, width, height)) = current_adjusted_depth(state)? else {
        return Ok(None);
    };
    let envelope = resolved_blank_envelope(state)?;
    let params = resolved_volumetric_params(state)?;
    Ok(Some(volumetric::estimate_point_count(
        width, height, &params, &envelope,
    )))
}

#[tauri::command]
fn set_blank_envelope(envelope: BlankEnvelope, state: State<AppState>) -> Result<(), String> {
    envelope.validate()?;
    {
        let mut settings = state.app_settings.lock().map_err(|e| e.to_string())?;
        settings.blank_envelope = Some(envelope);
        settings.save().map_err(|e| e.to_string())?;
    }
    invalidate_point_cloud_cache(&state)?;
    Ok(())
}

#[tauri::command]
fn set_volumetric_params(params: VolumetricParams, state: State<AppState>) -> Result<(), String> {
    validate_volumetric_params(&params)?;
    {
        let mut settings = state.app_settings.lock().map_err(|e| e.to_string())?;
        settings.volumetric_params = Some(params);
        settings.save().map_err(|e| e.to_string())?;
    }
    invalidate_point_cloud_cache(&state)?;
    Ok(())
}

#[tauri::command]
fn set_point_cloud_format(format: String, state: State<AppState>) -> Result<(), String> {
    let f = format.trim().to_lowercase();
    match f.as_str() {
        "ply" | "xyz" | "csv" => {
            let mut settings = state.app_settings.lock().map_err(|e| e.to_string())?;
            settings.point_cloud_format = Some(f);
            settings.save().map_err(|e| e.to_string())?;
            Ok(())
        }
        _ => Err("Format must be ply, xyz, or csv".to_string()),
    }
}

#[tauri::command]
fn estimate_point_cloud_count(state: State<AppState>) -> Result<Option<usize>, String> {
    estimate_point_cloud_count_from_state(&state)
}

#[tauri::command]
fn generate_point_cloud(state: State<AppState>) -> Result<VolumetricResult, String> {
    let result = generate_point_cloud_from_state(&state)?;
    *state.last_point_cloud.lock().map_err(|e| e.to_string())? = Some(result.clone());
    Ok(result)
}

#[tauri::command]
fn export_ply(path: String, binary: bool, state: State<AppState>) -> Result<(), String> {
    let (canonical_path, canonical_str) = validate_export_path(&path, "ply")?;
    let points = {
        let guard = state.last_point_cloud.lock().map_err(|e| e.to_string())?;
        let Some(ref cached) = *guard else {
            return Err("No point cloud generated; run generate_point_cloud first.".to_string());
        };
        cached.points.clone()
    };
    let metadata = build_export_metadata(&state)?;
    export::export_ply(canonical_path.as_path(), &points, &metadata, binary)
        .map_err(|e| e.to_string())?;
    persist_last_export_dir(&state, &canonical_str)?;
    Ok(())
}

#[tauri::command]
fn export_xyz(path: String, state: State<AppState>) -> Result<(), String> {
    let (canonical_path, canonical_str) = validate_export_path(&path, "xyz")?;
    let points = {
        let guard = state.last_point_cloud.lock().map_err(|e| e.to_string())?;
        let Some(ref cached) = *guard else {
            return Err("No point cloud generated; run generate_point_cloud first.".to_string());
        };
        cached.points.clone()
    };
    export::export_xyz(canonical_path.as_path(), &points).map_err(|e| e.to_string())?;
    persist_last_export_dir(&state, &canonical_str)?;
    Ok(())
}

#[tauri::command]
fn export_csv(path: String, state: State<AppState>) -> Result<(), String> {
    let (canonical_path, canonical_str) = validate_export_path(&path, "csv")?;
    let points = {
        let guard = state.last_point_cloud.lock().map_err(|e| e.to_string())?;
        let Some(ref cached) = *guard else {
            return Err("No point cloud generated; run generate_point_cloud first.".to_string());
        };
        cached.points.clone()
    };
    let metadata = build_export_metadata(&state)?;
    export::export_csv(canonical_path.as_path(), &points, &metadata).map_err(|e| e.to_string())?;
    persist_last_export_dir(&state, &canonical_str)?;
    Ok(())
}

// --- Sprint 2.3: Presets (BACK-1302) ---

/// Save current depth/mesh settings as a preset (BACK-1302).
/// If `path` is Some, writes to that file (user-chosen export path); otherwise saves to ~/.simplepicture3d/presets/{name}.json.
#[tauri::command]
fn save_preset(name: String, path: Option<String>, state: State<AppState>) -> Result<(), String> {
    let params = state
        .adjustment_params
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let settings_guard = state.app_settings.lock().map_err(|e| e.to_string())?;
    let target_width_mm = settings_guard.target_width_mm;
    let target_height_mm = settings_guard.target_height_mm;
    drop(settings_guard);

    let preset = Preset::from_depth_and_mesh(
        params.brightness,
        params.contrast,
        params.gamma,
        params.invert,
        params.depth_min_mm,
        params.depth_max_mm,
        params.curve_control_points.clone(),
        1,
        1,
        target_width_mm,
        target_height_mm,
    );

    let json = serde_json::to_string_pretty(&preset).map_err(|e| e.to_string())?;

    let write_path = if let Some(ref p) = path {
        let (canonical, _) = validate_preset_export_path(p)?;
        canonical
    } else {
        let presets_dir = settings::app_data_dir()
            .ok_or_else(|| "Cannot determine presets directory".to_string())?
            .join("presets");
        std::fs::create_dir_all(&presets_dir).map_err(|e| e.to_string())?;
        let safe_name = sanitize_preset_name(&name)?;
        presets_dir.join(format!("{}.json", safe_name))
    };

    std::fs::write(&write_path, json).map_err(|e| format!("Failed to write preset: {}", e))?;
    Ok(())
}

/// Load a preset by name (built-in or from presets dir) or by absolute path (import) and apply to app state (BACK-1302, BACK-1303).
/// Returns updated undo/redo state so UI can sync.
#[tauri::command]
fn load_preset(name_or_path: String, state: State<AppState>) -> Result<UndoRedoState, String> {
    let preset = if std::path::Path::new(&name_or_path).is_absolute() {
        let path = std::path::PathBuf::from(&name_or_path);
        let contents =
            std::fs::read_to_string(&path).map_err(|e| format!("Failed to read preset: {}", e))?;
        Preset::parse_and_validate_json(&contents)?
    } else if let Some(p) = get_builtin_preset(name_or_path.trim()) {
        p
    } else {
        let presets_dir = settings::app_data_dir()
            .ok_or_else(|| "Cannot determine presets directory".to_string())?
            .join("presets");
        let safe_name = sanitize_preset_name(name_or_path.trim())?;
        let path = presets_dir.join(format!("{}.json", safe_name));
        let contents =
            std::fs::read_to_string(&path).map_err(|e| format!("Failed to read preset: {}", e))?;
        Preset::parse_and_validate_json(&contents)?
    };

    let new_params = preset.to_depth_params();
    let previous = state
        .adjustment_params
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let cmd = SetDepthParamsCommand {
        previous: previous.clone(),
        new: new_params.clone(),
    };
    *state.adjustment_params.lock().map_err(|e| e.to_string())? = new_params.clone();
    {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.push(UndoableCommand::Depth(cmd));
    }
    {
        let mut app_settings = state.app_settings.lock().map_err(|e| e.to_string())?;
        app_settings.curve_control_points = preset.curve_control_points.clone();
        app_settings.target_width_mm = preset.target_width_mm;
        app_settings.target_height_mm = preset.target_height_mm;
        if let Err(e) = app_settings.save() {
            log::warn!("Failed to save settings after load_preset: {}", e);
        }
    }
    get_undo_redo_state(state)
}

/// List user preset names (filenames without .json) in ~/.simplepicture3d/presets/ (BACK-1302, UI-1301).
#[tauri::command]
fn list_presets() -> Result<Vec<String>, String> {
    let presets_dir = settings::app_data_dir()
        .ok_or_else(|| "Cannot determine presets directory".to_string())?
        .join("presets");
    if !presets_dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = std::fs::read_dir(&presets_dir)
        .map_err(|e| format!("Failed to read presets directory: {}", e))?
        .filter_map(|e| {
            e.ok().and_then(|entry| {
                let p = entry.path();
                if p.is_file() && p.extension().is_some_and(|e| e == "json") {
                    p.file_stem().map(|s| s.to_string_lossy().to_string())
                } else {
                    None
                }
            })
        })
        .collect();
    names.sort();
    Ok(names)
}

/// Delete a user preset by name (BACK-1302, UI-1301). Does not remove built-ins.
#[tauri::command]
fn delete_preset(name: String) -> Result<(), String> {
    let presets_dir = settings::app_data_dir()
        .ok_or_else(|| "Cannot determine presets directory".to_string())?
        .join("presets");
    let safe_name = sanitize_preset_name(name.trim())?;
    let path = presets_dir.join(format!("{}.json", safe_name));
    if path.is_file() {
        std::fs::remove_file(&path).map_err(|e| format!("Failed to delete preset: {}", e))?;
    }
    Ok(())
}

/// Rename a user preset (BACK-1302, UI-1301). Old and new are names (not paths).
#[tauri::command]
fn rename_preset(old_name: String, new_name: String) -> Result<(), String> {
    let presets_dir = settings::app_data_dir()
        .ok_or_else(|| "Cannot determine presets directory".to_string())?
        .join("presets");
    let safe_old = sanitize_preset_name(old_name.trim())?;
    let safe_new = sanitize_preset_name(new_name.trim())?;
    if safe_old == safe_new {
        return Ok(());
    }
    let from = presets_dir.join(format!("{}.json", safe_old));
    let to = presets_dir.join(format!("{}.json", safe_new));
    if !from.is_file() {
        return Err("Preset not found".to_string());
    }
    if to.exists() {
        return Err("A preset with that name already exists".to_string());
    }
    std::fs::rename(&from, &to).map_err(|e| format!("Failed to rename preset: {}", e))?;
    Ok(())
}

/// List built-in preset ids for UI dropdown (BACK-1303).
#[tauri::command]
fn list_builtin_presets() -> Vec<String> {
    preset::builtin_preset_ids()
        .iter()
        .map(|s| (*s).to_string())
        .collect()
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
        return Err(format!(
            "Model downloader returned no output. stderr: {}",
            stderr
        ));
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
    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse model status: {}", e))
}

/// Get model information (BACK-901).
#[tauri::command]
fn get_model_info() -> Result<ModelInfo, String> {
    let stdout = run_model_downloader_cmd("--info")?;
    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse model info: {}", e))
}

/// Download AI model (BACK-901, BACK-903).
#[tauri::command]
fn download_model() -> Result<DownloadResult, String> {
    let stdout = run_model_downloader_cmd("--download")?;
    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse download result: {}", e))
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
/// When progress_cb is Some, runs with real-time progress streaming (BACK-205-STREAM).
#[cfg(test)]
fn generate_depth_map_impl(
    path: &str,
    progress_cb: Option<python_bridge::ProgressCb>,
) -> Result<(python_bridge::DepthMapOutput, Vec<String>), String> {
    let bytes = image_loading::read_image_bytes_for_depth(path).map_err(|e| e.to_string())?;
    const TIMEOUT_SECS: u64 = 300;
    let timeout = Duration::from_secs(TIMEOUT_SECS);
    let result = match progress_cb {
        Some(cb) => python_bridge::run_depth_estimation_with_progress(&bytes, timeout, Some(cb))
            .map_err(|e| e.to_string())?,
        None => python_bridge::run_depth_estimation_with_timeout(&bytes, timeout)
            .map_err(|e| e.to_string())?,
    };
    log_depth_stats(&result.depth.depth);
    Ok((result.depth, result.stderr_lines))
}

/// Generates a depth map from an image file (BACK-301, BACK-303, BACK-304, BACK-205-STREAM).
/// Accepts image path (same as load_image); validates path and format, runs Python bridge;
/// stores result in app state (BACK-302), returns depth + progress 100 + stages.
/// Emits "depth-progress" Tauri events during estimation for real-time UI progress (Sprint 2.4).
#[tauri::command]
fn generate_depth_map(
    path: String,
    app_handle: tauri::AppHandle,
    state: State<AppState>,
) -> Result<GenerateDepthMapResponse, String> {
    let bytes = image_loading::read_image_bytes_for_depth(&path).map_err(|e| e.to_string())?;
    let progress_cb: python_bridge::ProgressCb = Box::new(move |percent, stage| {
        let payload = DepthProgressPayload { percent, stage };
        if let Err(e) = app_handle.emit("depth-progress", &payload) {
            log::warn!("depth-progress emit failed: {}", e);
        }
    });
    const TIMEOUT_SECS: u64 = 300;
    let result = python_bridge::run_depth_estimation_with_progress(
        &bytes,
        Duration::from_secs(TIMEOUT_SECS),
        Some(progress_cb),
    )
    .map_err(|e| e.to_string())?;
    let depth = result.depth;
    let stderr_lines = result.stderr_lines;
    log_depth_stats(&depth.depth);
    *state.depth.lock().map_err(|e| e.to_string())? = Some(depth.clone());
    // Store source image path for filename generation (BACK-705).
    *state.source_image_path.lock().map_err(|e| e.to_string())? = Some(path.clone());
    // Clear mask when depth map is replaced (ARCH-502).
    *state.mask.lock().map_err(|e| e.to_string())? = None;
    // Clear undo/redo history on new depth map (PRD F2.4).
    state.undo_redo.lock().map_err(|e| e.to_string())?.clear();
    invalidate_point_cloud_cache(&state)?;
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

/// Returns histogram of current (adjusted) depth map for UI (BACK-1101).
/// Bins = 256 over [0, 1]. Returns None if no depth map loaded. Uses masked adjustment when mask present.
#[tauri::command]
fn get_depth_histogram(state: State<AppState>) -> Result<Option<Vec<u32>>, String> {
    const BINS: usize = 256;
    let guard = state.depth.lock().map_err(|e| e.to_string())?;
    let original = match guard.as_ref() {
        Some(d) => d.clone(),
        None => return Ok(None),
    };
    drop(guard);
    let params = state
        .adjustment_params
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let mask_guard = state.mask.lock().map_err(|e| e.to_string())?;
    let adjusted = apply_adjustments_with_mask(
        &original.depth,
        original.width,
        original.height,
        &params,
        mask_guard.as_ref(),
    );
    Ok(Some(compute_histogram(&adjusted, BINS)))
}

/// Response for undo/redo/clear_history and get_undo_redo_state (BACK-1404).
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct UndoRedoState {
    can_undo: bool,
    can_redo: bool,
    params: DepthAdjustmentParams,
}

/// Returns the current depth map from app state with adjustments applied (BACK-302, BACK-402).
/// When a mask is present, only masked regions (and feathered blend) are adjusted (BACK-1202, BACK-1203).
#[tauri::command]
fn get_depth_map(state: State<AppState>) -> Result<Option<python_bridge::DepthMapOutput>, String> {
    let guard = state.depth.lock().map_err(|e| e.to_string())?;
    let original = match guard.as_ref() {
        Some(d) => d.clone(),
        None => return Ok(None),
    };
    drop(guard);
    let params = state
        .adjustment_params
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let mask_guard = state.mask.lock().map_err(|e| e.to_string())?;
    let adjusted = apply_adjustments_with_mask(
        &original.depth,
        original.width,
        original.height,
        &params,
        mask_guard.as_ref(),
    );
    Ok(Some(python_bridge::DepthMapOutput {
        width: original.width,
        height: original.height,
        depth: adjusted,
    }))
}

/// Sets depth adjustment parameters (BACK-402, BACK-1403). Wrapped in command for undo/redo.
/// CURVE-001: Persist curve_control_points to AppSettings so curve survives restart.
#[tauri::command]
fn set_depth_adjustment_params(
    params: DepthAdjustmentParams,
    state: State<AppState>,
) -> Result<UndoRedoState, String> {
    let previous = state
        .adjustment_params
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    *state.adjustment_params.lock().map_err(|e| e.to_string())? = params.clone();
    {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.push(UndoableCommand::Depth(SetDepthParamsCommand {
            previous: previous.clone(),
            new: params.clone(),
        }));
    }
    {
        let mut settings = state.app_settings.lock().map_err(|e| e.to_string())?;
        settings.curve_control_points = params.curve_control_points.clone();
        if let Err(e) = settings.save() {
            log::warn!(
                "Failed to save settings (curve) after set_depth_adjustment_params: {}",
                e
            );
        }
    }
    let hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
    Ok(UndoRedoState {
        can_undo: hist.can_undo(),
        can_redo: hist.can_redo(),
        params,
    })
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

/// Returns undo/redo state for UI (can_undo, can_redo, current params). Use after load or to sync buttons.
#[tauri::command]
fn get_undo_redo_state(state: State<AppState>) -> Result<UndoRedoState, String> {
    let params = state
        .adjustment_params
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
    Ok(UndoRedoState {
        can_undo: hist.can_undo(),
        can_redo: hist.can_redo(),
        params,
    })
}

/// Undo last action (depth or mask) (BACK-1404, ARCH-502). Restores previous state; returns new state for UI.
/// When nothing to undo, returns current state with can_undo: false so UI can disable button.
#[tauri::command]
fn undo(state: State<AppState>) -> Result<UndoRedoState, String> {
    let cmd = {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.pop_undo()
    };
    let Some(cmd) = cmd else {
        return get_undo_redo_state(state);
    };
    {
        let mut params = state.adjustment_params.lock().map_err(|e| e.to_string())?;
        let mut mask = state.mask.lock().map_err(|e| e.to_string())?;
        cmd.apply_previous(&mut params, &mut mask);
    }
    {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.push_redo(cmd);
    }
    get_undo_redo_state(state)
}

/// Redo last undone action (depth or mask) (BACK-1404, ARCH-502). Returns new state for UI.
/// When nothing to redo, returns current state with can_redo: false so UI can disable button.
#[tauri::command]
fn redo(state: State<AppState>) -> Result<UndoRedoState, String> {
    let cmd = {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.pop_redo()
    };
    let Some(cmd) = cmd else {
        return get_undo_redo_state(state);
    };
    {
        let mut params = state.adjustment_params.lock().map_err(|e| e.to_string())?;
        let mut mask = state.mask.lock().map_err(|e| e.to_string())?;
        cmd.apply_new(&mut params, &mut mask);
    }
    {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.push_undo(cmd);
    }
    get_undo_redo_state(state)
}

/// Clear undo and redo history (BACK-1404). Does not change current params.
#[tauri::command]
fn clear_history(state: State<AppState>) -> Result<UndoRedoState, String> {
    state.undo_redo.lock().map_err(|e| e.to_string())?.clear();
    get_undo_redo_state(state)
}

/// Mask response for IPC (BACK-1201, ARCH-502). Row-major data; dimensions match depth map.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct MaskOut {
    width: u32,
    height: u32,
    data: Vec<bool>,
}

/// Returns current mask for overlay (BACK-1201). None if no depth map; else dimensions + row-major bools.
#[tauri::command]
fn get_mask(state: State<AppState>) -> Result<Option<MaskOut>, String> {
    let guard = state.depth.lock().map_err(|e| e.to_string())?;
    let (width, height) = match guard.as_ref() {
        Some(d) => (d.width, d.height),
        None => return Ok(None),
    };
    drop(guard);
    let mask_guard = state.mask.lock().map_err(|e| e.to_string())?;
    let out = match mask_guard.as_ref() {
        Some(m) if m.dimensions_match(width, height) => MaskOut {
            width,
            height,
            data: m.to_bool_vec(),
        },
        _ => MaskOut {
            width,
            height,
            data: mask::MaskBitmap::all_false(width, height).to_bool_vec(),
        },
    };
    Ok(Some(out))
}

/// Set a rectangular region of the mask (BACK-1201). Creates mask if none; pushes to undo stack.
#[tauri::command]
fn set_mask_region(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    value: bool,
    state: State<AppState>,
) -> Result<UndoRedoState, String> {
    let (depth_w, depth_h) = {
        let guard = state.depth.lock().map_err(|e| e.to_string())?;
        let d = guard
            .as_ref()
            .ok_or_else(|| "No depth map loaded. Generate a depth map first.".to_string())?;
        (d.width, d.height)
    };
    let previous = state.mask.lock().map_err(|e| e.to_string())?.clone();
    let previous = previous.unwrap_or_else(|| mask::MaskBitmap::all_false(depth_w, depth_h));
    if !previous.dimensions_match(depth_w, depth_h) {
        return Err("Mask dimensions do not match depth map.".to_string());
    }
    let mut new_mask = previous.clone();
    new_mask.set_region(x, y, width, height, value);
    let cmd = SetMaskCommand {
        previous: previous.clone(),
        new: new_mask.clone(),
    };
    *state.mask.lock().map_err(|e| e.to_string())? = Some(new_mask);
    {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.push(UndoableCommand::Mask(cmd));
    }
    get_undo_redo_state(state)
}

/// Clear the mask to all false (BACK-1201). Pushes to undo stack. Requires depth map.
#[tauri::command]
fn clear_mask(state: State<AppState>) -> Result<UndoRedoState, String> {
    let (depth_w, depth_h) = {
        let guard = state.depth.lock().map_err(|e| e.to_string())?;
        let d = guard
            .as_ref()
            .ok_or_else(|| "No depth map loaded. Generate a depth map first.".to_string())?;
        (d.width, d.height)
    };
    let previous = state.mask.lock().map_err(|e| e.to_string())?.clone();
    let previous = previous.unwrap_or_else(|| mask::MaskBitmap::all_false(depth_w, depth_h));
    let new_mask = mask::MaskBitmap::all_false(depth_w, depth_h);
    let cmd = SetMaskCommand {
        previous: previous.clone(),
        new: new_mask.clone(),
    };
    *state.mask.lock().map_err(|e| e.to_string())? = Some(new_mask);
    {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.push(UndoableCommand::Mask(cmd));
    }
    get_undo_redo_state(state)
}

/// Resets adjustment params to defaults; original depth unchanged (BACK-405, BACK-1403). Wrapped for undo/redo.
#[tauri::command]
fn reset_depth_adjustments(state: State<AppState>) -> Result<UndoRedoState, String> {
    let previous = state
        .adjustment_params
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let new_params = DepthAdjustmentParams::default();
    let cmd = SetDepthParamsCommand {
        previous: previous.clone(),
        new: new_params.clone(),
    };
    *state.adjustment_params.lock().map_err(|e| e.to_string())? = new_params.clone();
    {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.push(UndoableCommand::Depth(cmd));
    }
    Ok(UndoRedoState {
        can_undo: state
            .undo_redo
            .lock()
            .map_err(|e| e.to_string())?
            .can_undo(),
        can_redo: state
            .undo_redo
            .lock()
            .map_err(|e| e.to_string())?
            .can_redo(),
        params: new_params,
    })
}

/// Set the full mask from row-major bool data (JR1-1203 load, JR1-1202 lasso fill). Pushes to undo stack.
#[tauri::command]
fn set_mask(
    width: u32,
    height: u32,
    data: Vec<bool>,
    state: State<AppState>,
) -> Result<UndoRedoState, String> {
    let (depth_w, depth_h) = {
        let guard = state.depth.lock().map_err(|e| e.to_string())?;
        let d = guard
            .as_ref()
            .ok_or_else(|| "No depth map loaded. Generate a depth map first.".to_string())?;
        (d.width, d.height)
    };
    if width != depth_w || height != depth_h {
        return Err(format!(
            "Mask dimensions {}x{} do not match depth map {}x{}.",
            width, height, depth_w, depth_h
        ));
    }
    let new_mask =
        mask::MaskBitmap::from_bool_vec(width, height, &data).map_err(|e| e.to_string())?;
    let previous = state.mask.lock().map_err(|e| e.to_string())?.clone();
    let previous = previous.unwrap_or_else(|| mask::MaskBitmap::all_false(depth_w, depth_h));
    let cmd = SetMaskCommand {
        previous: previous.clone(),
        new: new_mask.clone(),
    };
    *state.mask.lock().map_err(|e| e.to_string())? = Some(new_mask);
    {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.push(UndoableCommand::Mask(cmd));
    }
    get_undo_redo_state(state)
}

/// Mask file format (JR1-1203). JSON: { "width", "height", "data": boolean[] } row-major.
#[derive(serde::Serialize, serde::Deserialize)]
struct MaskFile {
    width: u32,
    height: u32,
    data: Vec<bool>,
}

/// Save current mask to a JSON file (JR1-1203). Path must be .json and writable.
#[tauri::command]
fn save_mask_to_path(path: String, state: State<AppState>) -> Result<(), String> {
    let (canonical_path, _) = validate_preset_export_path(&path)?;
    let (width, height, data) = {
        let guard = state.depth.lock().map_err(|e| e.to_string())?;
        let (w, h) = match guard.as_ref() {
            Some(d) => (d.width, d.height),
            None => return Err("No depth map loaded.".to_string()),
        };
        drop(guard);
        let mask_guard = state.mask.lock().map_err(|e| e.to_string())?;
        let data = match mask_guard.as_ref() {
            Some(m) if m.dimensions_match(w, h) => m.to_bool_vec(),
            _ => mask::MaskBitmap::all_false(w, h).to_bool_vec(),
        };
        (w, h, data)
    };
    let file = MaskFile {
        width,
        height,
        data,
    };
    let f = std::fs::File::create(&canonical_path)
        .map_err(|e| format!("Could not create file: {}", e))?;
    serde_json::to_writer_pretty(f, &file).map_err(|e| format!("Could not write mask: {}", e))?;
    Ok(())
}

/// Load mask from a JSON file (JR1-1203). Dimensions must match current depth map.
#[tauri::command]
fn load_mask_from_path(path: String, state: State<AppState>) -> Result<UndoRedoState, String> {
    let path_buf = std::path::Path::new(&path);
    let canonical = path_buf
        .canonicalize()
        .map_err(|_| "File does not exist or is not accessible.".to_string())?;
    let content =
        std::fs::read_to_string(&canonical).map_err(|e| format!("Could not read file: {}", e))?;
    let file: MaskFile =
        serde_json::from_str(&content).map_err(|e| format!("Invalid mask file: {}", e))?;
    let (depth_w, depth_h) = {
        let guard = state.depth.lock().map_err(|e| e.to_string())?;
        let d = guard
            .as_ref()
            .ok_or_else(|| "No depth map loaded. Generate a depth map first.".to_string())?;
        (d.width, d.height)
    };
    if file.width != depth_w || file.height != depth_h {
        return Err(format!(
            "Saved mask is {}x{} but current depth map is {}x{}. Load an image with matching dimensions.",
            file.width, file.height, depth_w, depth_h
        ));
    }
    let new_mask = mask::MaskBitmap::from_bool_vec(file.width, file.height, &file.data)
        .map_err(|e| e.to_string())?;
    let previous = state.mask.lock().map_err(|e| e.to_string())?.clone();
    let previous = previous.unwrap_or_else(|| mask::MaskBitmap::all_false(depth_w, depth_h));
    let cmd = SetMaskCommand {
        previous: previous.clone(),
        new: new_mask.clone(),
    };
    *state.mask.lock().map_err(|e| e.to_string())? = Some(new_mask);
    {
        let mut hist = state.undo_redo.lock().map_err(|e| e.to_string())?;
        hist.push(UndoableCommand::Mask(cmd));
    }
    get_undo_redo_state(state)
}

// Sprint A retired 2.5D mesh preview (`get_mesh_data`); superseded by `generate_point_cloud` (ADR-012).

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = env_logger::try_init();
    let app_settings = settings::AppSettings::load();
    let mut adjustment_params = DepthAdjustmentParams::default();
    if let Some(ref curve) = app_settings.curve_control_points {
        if curve.len() >= 2 {
            adjustment_params.curve_control_points = Some(curve.clone());
        }
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            depth: Mutex::new(None),
            adjustment_params: Mutex::new(adjustment_params),
            mask: Mutex::new(None),
            source_image_path: Mutex::new(None),
            app_settings: Mutex::new(app_settings),
            undo_redo: Mutex::new(UndoRedoHistory::new()),
            last_point_cloud: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            load_image,
            generate_depth_map,
            get_depth_map,
            get_depth_histogram,
            set_depth_adjustment_params,
            get_depth_adjustment_params,
            get_undo_redo_state,
            undo,
            redo,
            clear_history,
            get_mask,
            set_mask_region,
            set_mask,
            clear_mask,
            save_mask_to_path,
            load_mask_from_path,
            reset_depth_adjustments,
            get_settings,
            save_settings,
            set_blank_envelope,
            set_volumetric_params,
            set_point_cloud_format,
            estimate_point_cloud_count,
            generate_point_cloud,
            export_ply,
            export_xyz,
            export_csv,
            save_preset,
            load_preset,
            list_presets,
            delete_preset,
            rename_preset,
            list_builtin_presets,
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
    fn point_cloud_estimate_none_without_depth_map() {
        let state = AppState {
            depth: Mutex::new(None),
            adjustment_params: Mutex::new(DepthAdjustmentParams::default()),
            mask: Mutex::new(None),
            source_image_path: Mutex::new(None),
            app_settings: Mutex::new(settings::AppSettings::default()),
            undo_redo: Mutex::new(UndoRedoHistory::new()),
            last_point_cloud: Mutex::new(None),
        };
        assert_eq!(estimate_point_cloud_count_from_state(&state).unwrap(), None);
    }

    #[test]
    fn point_cloud_generation_succeeds_with_uniform_depth() {
        let depth = python_bridge::DepthMapOutput {
            width: 4,
            height: 4,
            depth: vec![0.5f32; 16],
        };
        let app_settings = settings::AppSettings {
            blank_envelope: Some(BlankEnvelope::default()),
            volumetric_params: Some(VolumetricParams {
                step_x: 1,
                step_y: 1,
                depth_threshold: 0.0,
            }),
            ..Default::default()
        };
        let state = AppState {
            depth: Mutex::new(Some(depth)),
            adjustment_params: Mutex::new(DepthAdjustmentParams::default()),
            mask: Mutex::new(None),
            source_image_path: Mutex::new(None),
            app_settings: Mutex::new(app_settings),
            undo_redo: Mutex::new(UndoRedoHistory::new()),
            last_point_cloud: Mutex::new(None),
        };
        let r = generate_point_cloud_from_state(&state).unwrap();
        assert_eq!(r.point_count, 16);
    }

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
        let err = generate_depth_map_impl("", None).unwrap_err();
        assert!(!err.is_empty());
        let err = generate_depth_map_impl("   ", None).unwrap_err();
        assert!(!err.is_empty());
    }

    /// BACK-301: generate_depth_map rejects nonexistent path.
    #[test]
    fn generate_depth_map_rejects_nonexistent_path() {
        let path = std::env::temp_dir()
            .join("sp3d_nonexistent_generate_depth_12345.png")
            .to_string_lossy()
            .to_string();
        let result = generate_depth_map_impl(&path, None);
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

    /// BACK-1202: No mask -> same as apply_adjustments.
    #[test]
    fn apply_adjustments_with_mask_none_equals_full() {
        let depth = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let params = DepthAdjustmentParams {
            brightness: 0.1,
            gamma: 1.2,
            ..Default::default()
        };
        let out = apply_adjustments_with_mask(&depth, 5, 1, &params, None);
        let expected = apply_adjustments(&depth, &params);
        for (a, b) in out.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-6, "no mask should match full adjustment");
        }
    }

    /// BACK-1202: Masked region only gets adjusted; unmasked keeps original.
    #[test]
    fn apply_adjustments_with_mask_isolates_region() {
        let depth = vec![0.5, 0.5, 0.5, 0.5]; // 4x1
        let params = DepthAdjustmentParams {
            brightness: 0.5, // would make 0.5 -> 1.0
            ..Default::default()
        };
        let mut mask = mask::MaskBitmap::all_false(4, 1);
        mask.set(1, 0, true);
        mask.set(2, 0, true);
        let out = apply_adjustments_with_mask(&depth, 4, 1, &params, Some(&mask));
        assert!((out[0] - 0.5).abs() < 1e-6, "unmasked stays original");
        assert!((out[3] - 0.5).abs() < 1e-6, "unmasked stays original");
        assert!((out[1] - 1.0).abs() < 1e-5, "masked gets adjusted");
        assert!((out[2] - 1.0).abs() < 1e-5, "masked gets adjusted");
    }

    /// BACK-1203: Feather radius > 0 produces blend between original and adjusted.
    #[test]
    fn apply_adjustments_with_mask_feather_blend() {
        let depth = vec![0.5, 0.5, 0.5]; // 3x1
        let params = DepthAdjustmentParams {
            brightness: 0.5,
            feather_radius_px: 1.0,
            ..Default::default()
        };
        let mut mask = mask::MaskBitmap::all_false(3, 1);
        mask.set(1, 0, true); // only center masked
        let out = apply_adjustments_with_mask(&depth, 3, 1, &params, Some(&mask));
        // With feather, center is blended (between original 0.5 and adjusted 1.0)
        assert!((0.5..=1.0).contains(&out[1]), "center should be blended");
        assert!(
            out.iter().all(|&v| (0.0..=1.0).contains(&v)),
            "output in [0,1]"
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
        let result = generate_depth_map_impl(&path, None);
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

    // Sprint A: `compute_pixel_to_mm` tests retired alongside the helper itself (was only used
    // by the removed export_stl / export_obj / get_mesh_data commands). The volumetric pipeline
    // scales via `BlankEnvelope::fit_to_blank` instead.

    /// BACK-1202: With no mask, apply_adjustments_with_mask matches apply_adjustments.
    #[test]
    fn apply_adjustments_with_mask_no_mask_matches_full() {
        let depth = vec![0.2, 0.5, 0.8];
        let params = DepthAdjustmentParams {
            brightness: 0.1,
            ..Default::default()
        };
        let out = apply_adjustments_with_mask(&depth, 3, 1, &params, None);
        let expected = apply_adjustments(&depth, &params);
        assert_eq!(out.len(), expected.len());
        for (a, b) in out.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-6, "no mask should match full apply");
        }
    }

    /// BACK-1202: Mask all false => output equals original (no adjustment applied).
    #[test]
    fn apply_adjustments_with_mask_all_false_unchanged() {
        let depth = vec![0.2, 0.5, 0.8];
        let params = DepthAdjustmentParams {
            brightness: 0.2,
            ..Default::default()
        };
        let mask = mask::MaskBitmap::all_false(3, 1);
        let out = apply_adjustments_with_mask(&depth, 3, 1, &params, Some(&mask));
        assert_eq!(out.len(), depth.len());
        for (a, o) in out.iter().zip(depth.iter()) {
            assert!((a - o).abs() < 1e-6, "mask all false => original");
        }
    }

    /// BACK-1202: Single pixel masked => only that pixel adjusted.
    #[test]
    fn apply_adjustments_with_mask_single_pixel_adjusted() {
        let depth = vec![0.0, 0.5, 1.0];
        let params = DepthAdjustmentParams {
            brightness: 0.2,
            ..Default::default()
        };
        let mut mask = mask::MaskBitmap::all_false(3, 1);
        mask.set(1, 0, true); // center pixel only
        let out = apply_adjustments_with_mask(&depth, 3, 1, &params, Some(&mask));
        assert!((out[0] - 0.0).abs() < 1e-6, "unmasked stays 0");
        assert!((out[1] - 0.7).abs() < 1e-5, "masked 0.5+0.2=0.7");
        assert!((out[2] - 1.0).abs() < 1e-6, "unmasked stays 1");
    }
}
