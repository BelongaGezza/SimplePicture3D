// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Persistent application settings (BACK-706).
//!
//! Settings are stored as JSON at `~/.simplepicture3d/settings.json`.
//! Loaded on startup; saved after each export to remember last export directory.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::depth_adjust::CurvePoint;

/// Application settings persisted between sessions (BACK-706, BACK-804, BACK-805).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// Last directory used for STL/OBJ export (BACK-706).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_export_dir: Option<String>,

    /// Preferred export format: "stl" or "obj" (BACK-804).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,

    /// Last-used depth adjustment params for session restore (BACK-805).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_brightness: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_contrast: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_gamma: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_invert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_min_mm: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_max_mm: Option<f32>,

    /// Window dimensions for session restore (BACK-805).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_height: Option<u32>,

    /// Target output size in mm for mesh/export (ADR-009, BACK-1006). When both set and positive,
    /// mesh XY fits inside target_width_mm × target_height_mm with aspect ratio preserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_width_mm: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_height_mm: Option<f32>,

    /// Curve control points for depth remapping (CURVE-001, Consultant §2.6). Persisted so curve
    /// survives restart; applied to depth adjustment on load.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve_control_points: Option<Vec<CurvePoint>>,
}

/// App data directory: `~/.simplepicture3d/` (Sprint 2.3 presets, models, logs).
pub fn app_data_dir() -> Option<PathBuf> {
    dirs_fallback().map(|home| home.join(".simplepicture3d"))
}

impl AppSettings {
    /// Path to the settings file: `~/.simplepicture3d/settings.json`.
    fn settings_path() -> Option<PathBuf> {
        app_data_dir().map(|d| d.join("settings.json"))
    }

    /// Load settings from disk. Returns defaults if file doesn't exist or is unreadable.
    pub fn load() -> Self {
        let path = match Self::settings_path() {
            Some(p) => p,
            None => {
                log::warn!("Could not determine home directory; using default settings");
                return Self::default();
            }
        };
        match std::fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
                log::warn!("Failed to parse settings at {:?}: {}", path, e);
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    /// Save settings to disk. Creates directory if needed.
    pub fn save(&self) -> Result<(), anyhow::Error> {
        let path = Self::settings_path()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory for settings"))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, json)?;
        Ok(())
    }
}

/// Get home directory without external crate. Falls back through env vars.
fn dirs_fallback() -> Option<PathBuf> {
    // Try standard env vars
    if let Ok(home) = std::env::var("HOME") {
        return Some(PathBuf::from(home));
    }
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        return Some(PathBuf::from(userprofile));
    }
    // Windows: HOMEDRIVE + HOMEPATH
    if let (Ok(drive), Ok(path)) = (std::env::var("HOMEDRIVE"), std::env::var("HOMEPATH")) {
        return Some(PathBuf::from(format!("{}{}", drive, path)));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let s = AppSettings::default();
        assert!(s.last_export_dir.is_none());
    }

    #[test]
    fn settings_roundtrip_json() {
        let mut s = AppSettings::default();
        s.last_export_dir = Some("C:\\Users\\test\\exports".to_string());
        s.export_format = Some("obj".to_string());
        let json = serde_json::to_string(&s).unwrap();
        let loaded: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(
            loaded.last_export_dir.as_deref(),
            Some("C:\\Users\\test\\exports")
        );
        assert_eq!(loaded.export_format.as_deref(), Some("obj"));
    }

    #[test]
    fn settings_load_returns_default_when_no_file() {
        // This should not panic; just return defaults
        let s = AppSettings::load();
        // last_export_dir might be set from a real settings file on the dev machine,
        // or None if no file exists. Either way, no panic.
        let _ = s;
    }

    #[test]
    fn settings_path_exists() {
        // settings_path should return Some on any platform with HOME or USERPROFILE
        let path = AppSettings::settings_path();
        assert!(path.is_some(), "settings_path should resolve on this OS");
        let p = path.unwrap();
        assert!(
            p.to_string_lossy().contains("simplepicture3d"),
            "path should contain simplepicture3d: {:?}",
            p
        );
    }

    #[test]
    fn settings_save_and_load_roundtrip() {
        // Save to the real settings path, then load back
        let mut original = AppSettings::default();
        original.last_export_dir = Some("/tmp/sp3d_test_export".to_string());
        // Only test if we can determine the path
        if AppSettings::settings_path().is_some() {
            let save_result = original.save();
            if save_result.is_ok() {
                let loaded = AppSettings::load();
                assert_eq!(
                    loaded.last_export_dir.as_deref(),
                    Some("/tmp/sp3d_test_export")
                );
            }
        }
    }

    // =========================================================================
    // Sprint 1.9: Extended settings tests (BACK-804, BACK-805, JR2-803, JR2-804)
    // =========================================================================

    #[test]
    fn default_settings_all_none() {
        let s = AppSettings::default();
        assert!(s.last_export_dir.is_none());
        assert!(s.export_format.is_none());
        assert!(s.depth_brightness.is_none());
        assert!(s.depth_contrast.is_none());
        assert!(s.depth_gamma.is_none());
        assert!(s.depth_invert.is_none());
        assert!(s.depth_min_mm.is_none());
        assert!(s.depth_max_mm.is_none());
        assert!(s.window_width.is_none());
        assert!(s.window_height.is_none());
        assert!(s.target_width_mm.is_none());
        assert!(s.target_height_mm.is_none());
        assert!(s.curve_control_points.is_none());
    }

    #[test]
    fn settings_extended_roundtrip_json() {
        let mut s = AppSettings::default();
        s.export_format = Some("obj".to_string());
        s.depth_brightness = Some(1.2);
        s.depth_contrast = Some(0.8);
        s.depth_gamma = Some(2.2);
        s.depth_invert = Some(true);
        s.depth_min_mm = Some(3.0);
        s.depth_max_mm = Some(8.0);
        s.window_width = Some(1200);
        s.window_height = Some(800);
        s.target_width_mm = Some(50.0);
        s.target_height_mm = Some(70.0);

        let json = serde_json::to_string_pretty(&s).unwrap();
        let loaded: AppSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.export_format.as_deref(), Some("obj"));
        assert_eq!(loaded.depth_brightness, Some(1.2));
        assert_eq!(loaded.depth_contrast, Some(0.8));
        assert_eq!(loaded.depth_gamma, Some(2.2));
        assert_eq!(loaded.depth_invert, Some(true));
        assert_eq!(loaded.depth_min_mm, Some(3.0));
        assert_eq!(loaded.depth_max_mm, Some(8.0));
        assert_eq!(loaded.window_width, Some(1200));
        assert_eq!(loaded.window_height, Some(800));
        assert_eq!(loaded.target_width_mm, Some(50.0));
        assert_eq!(loaded.target_height_mm, Some(70.0));
    }

    #[test]
    fn settings_skip_serializing_none_fields() {
        let s = AppSettings::default();
        let json = serde_json::to_string(&s).unwrap();
        // With skip_serializing_if, the JSON for defaults should be just {}
        assert_eq!(json, "{}");
    }

    #[test]
    fn settings_partial_json_loads_with_defaults() {
        // Old settings file with only last_export_dir (backwards compatibility)
        let json = r#"{"lastExportDir":"/tmp/old"}"#;
        let loaded: AppSettings = serde_json::from_str(json).unwrap();
        assert_eq!(loaded.last_export_dir.as_deref(), Some("/tmp/old"));
        assert!(loaded.export_format.is_none());
        assert!(loaded.depth_brightness.is_none());
        assert!(loaded.window_width.is_none());
    }

    #[test]
    fn settings_curve_control_points_roundtrip() {
        use crate::depth_adjust::CurvePoint;
        let mut s = AppSettings::default();
        s.curve_control_points = Some(vec![
            CurvePoint { x: 0.0, y: 0.0 },
            CurvePoint { x: 0.5, y: 0.25 },
            CurvePoint { x: 1.0, y: 1.0 },
        ]);
        let json = serde_json::to_string(&s).unwrap();
        let loaded: AppSettings = serde_json::from_str(&json).unwrap();
        let pts = loaded.curve_control_points.as_ref().unwrap();
        assert_eq!(pts.len(), 3);
        assert!((pts[0].x - 0.0).abs() < 1e-6);
        assert!((pts[1].y - 0.25).abs() < 1e-6);
    }

    #[test]
    fn settings_unknown_fields_ignored() {
        // Future settings file with extra unknown fields should still parse
        let json = r#"{"lastExportDir":"/tmp","unknownFutureField":"value"}"#;
        // serde by default ignores unknown fields
        let loaded: AppSettings = serde_json::from_str(json).unwrap();
        assert_eq!(loaded.last_export_dir.as_deref(), Some("/tmp"));
    }

    #[test]
    fn settings_corrupt_json_returns_error() {
        let corrupt = "not json at all {{{";
        let result: Result<AppSettings, _> = serde_json::from_str(corrupt);
        assert!(result.is_err(), "corrupt JSON should fail to parse");
    }

    #[test]
    fn settings_empty_json_returns_defaults() {
        let json = "{}";
        let loaded: AppSettings = serde_json::from_str(json).unwrap();
        assert!(loaded.last_export_dir.is_none());
        assert!(loaded.export_format.is_none());
    }
}
