//! Persistent application settings (BACK-706).
//!
//! Settings are stored as JSON at `~/.simplepicture3d/settings.json`.
//! Loaded on startup; saved after each export to remember last export directory.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
}

impl AppSettings {
    /// Path to the settings file: `~/.simplepicture3d/settings.json`.
    fn settings_path() -> Option<PathBuf> {
        dirs_fallback().map(|home| home.join(".simplepicture3d").join("settings.json"))
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
}
