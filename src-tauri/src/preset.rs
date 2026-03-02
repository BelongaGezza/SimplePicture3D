// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Preset schema and serialization (BACK-1301, Sprint 2.3).
//!
//! Presets store depth adjustment params, curve control points, and mesh params as JSON.
//! Schema is documented in RESEARCH/architecture.md § Preset schema.

use serde::{Deserialize, Serialize};

use crate::depth_adjust::{preset_s_curve, CurvePoint};

/// Current preset schema version for forward compatibility (BACK-1301, JR2-1303).
pub const PRESET_SCHEMA_VERSION: u32 = 1;

/// Built-in preset identifiers (BACK-1303, prd.md F2.3).
pub const BUILTIN_PORTRAIT: &str = "Portrait";
pub const BUILTIN_LANDSCAPE: &str = "Landscape";
pub const BUILTIN_HIGH_DETAIL: &str = "High Detail";
pub const BUILTIN_LOW_RELIEF: &str = "Low Relief";

/// Ordered list of built-in preset ids for UI dropdown (BACK-1303).
pub fn builtin_preset_ids() -> &'static [&'static str] {
    &[BUILTIN_PORTRAIT, BUILTIN_LANDSCAPE, BUILTIN_HIGH_DETAIL, BUILTIN_LOW_RELIEF]
}

/// Returns a built-in preset by id, or None if unknown (BACK-1303).
pub fn get_builtin_preset(id: &str) -> Option<Preset> {
    Some(match id {
        BUILTIN_PORTRAIT => Preset {
            schema_version: PRESET_SCHEMA_VERSION,
            brightness: 0.05,
            contrast: 1.1,
            gamma: 1.15,
            invert: false,
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            curve_control_points: Some(preset_s_curve()),
            step_x: 1,
            step_y: 1,
            target_width_mm: None,
            target_height_mm: None,
        },
        BUILTIN_LANDSCAPE => Preset {
            schema_version: PRESET_SCHEMA_VERSION,
            brightness: 0.0,
            contrast: 1.05,
            gamma: 1.0,
            invert: false,
            depth_min_mm: 2.0,
            depth_max_mm: 12.0,
            curve_control_points: None,
            step_x: 1,
            step_y: 1,
            target_width_mm: None,
            target_height_mm: None,
        },
        BUILTIN_HIGH_DETAIL => Preset {
            schema_version: PRESET_SCHEMA_VERSION,
            brightness: 0.0,
            contrast: 1.35,
            gamma: 1.25,
            invert: false,
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            curve_control_points: None,
            step_x: 1,
            step_y: 1,
            target_width_mm: None,
            target_height_mm: None,
        },
        BUILTIN_LOW_RELIEF => Preset {
            schema_version: PRESET_SCHEMA_VERSION,
            brightness: 0.1,
            contrast: 0.9,
            gamma: 0.9,
            invert: false,
            depth_min_mm: 2.0,
            depth_max_mm: 6.0,
            curve_control_points: None,
            step_x: 1,
            step_y: 1,
            target_width_mm: None,
            target_height_mm: None,
        },
        _ => return None,
    })
}

/// Preset JSON structure: depth params, curve, mesh params (BACK-1301).
/// Serialized with camelCase for JSON; stored under ~/.simplepicture3d/presets/ or user path.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preset {
    /// Schema version for migration (current = 1).
    pub schema_version: u32,

    // --- Depth adjustment (matches DepthAdjustmentParams) ---
    pub brightness: f32,
    pub contrast: f32,
    pub gamma: f32,
    pub invert: bool,
    pub depth_min_mm: f32,
    pub depth_max_mm: f32,

    /// Optional curve control points; None or len < 2 means no curve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curve_control_points: Option<Vec<CurvePoint>>,

    // --- Mesh params ---
    /// Grid step X (1 = full resolution).
    #[serde(default = "default_step")]
    pub step_x: u32,
    /// Grid step Y (1 = full resolution).
    #[serde(default = "default_step")]
    pub step_y: u32,

    /// Optional target output width in mm (ADR-009).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_width_mm: Option<f32>,
    /// Optional target output height in mm (ADR-009).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_height_mm: Option<f32>,
}

fn default_step() -> u32 {
    1
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            schema_version: PRESET_SCHEMA_VERSION,
            brightness: 0.0,
            contrast: 1.0,
            gamma: 1.0,
            invert: false,
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            curve_control_points: None,
            step_x: 1,
            step_y: 1,
            target_width_mm: None,
            target_height_mm: None,
        }
    }
}

/// Max length for preset name when used as filename (avoid filesystem limits).
const PRESET_NAME_MAX_LEN: usize = 200;

/// Sanitizes a preset name for use as a filename (no path traversal). Allows alphanumeric, space, hyphen, underscore.
/// Returns error if empty, too long, or contains invalid characters (e.g. path separators).
pub fn sanitize_preset_name(name: &str) -> Result<String, String> {
    let s = name.trim();
    if s.is_empty() {
        return Err("Preset name cannot be empty".to_string());
    }
    if s.len() > PRESET_NAME_MAX_LEN {
        return Err(format!(
            "Preset name must be at most {} characters",
            PRESET_NAME_MAX_LEN
        ));
    }
    if s.contains("..") || s.contains('/') || s.contains('\\') {
        return Err("Preset name cannot contain path characters".to_string());
    }
    let ok: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect();
    if ok.trim().is_empty() {
        return Err("Preset name must contain at least one letter or number".to_string());
    }
    Ok(ok.trim().to_string())
}

impl Preset {
    /// Build a preset from depth params and optional curve/mesh overrides.
    /// mesh_step_x/y and target dimensions can be passed from AppSettings or defaults.
    #[allow(clippy::too_many_arguments)]
    pub fn from_depth_and_mesh(
        brightness: f32,
        contrast: f32,
        gamma: f32,
        invert: bool,
        depth_min_mm: f32,
        depth_max_mm: f32,
        curve_control_points: Option<Vec<CurvePoint>>,
        step_x: u32,
        step_y: u32,
        target_width_mm: Option<f32>,
        target_height_mm: Option<f32>,
    ) -> Self {
        Self {
            schema_version: PRESET_SCHEMA_VERSION,
            brightness,
            contrast,
            gamma,
            invert,
            depth_min_mm,
            depth_max_mm,
            curve_control_points,
            step_x: step_x.max(1),
            step_y: step_y.max(1),
            target_width_mm,
            target_height_mm,
        }
    }

    /// Depth params for applying to DepthAdjustmentParams (and undo stack).
    pub fn to_depth_params(&self) -> crate::depth_adjust::DepthAdjustmentParams {
        crate::depth_adjust::DepthAdjustmentParams {
            brightness: self.brightness,
            contrast: self.contrast,
            gamma: self.gamma,
            invert: self.invert,
            depth_min_mm: self.depth_min_mm,
            depth_max_mm: self.depth_max_mm,
            curve_control_points: self.curve_control_points.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_default_has_version_1() {
        let p = Preset::default();
        assert_eq!(p.schema_version, 1);
        assert_eq!(p.step_x, 1);
        assert_eq!(p.step_y, 1);
    }

    #[test]
    fn preset_roundtrip_json() {
        let p = Preset {
            schema_version: 1,
            brightness: 0.2,
            contrast: 1.1,
            gamma: 2.2,
            invert: true,
            depth_min_mm: 3.0,
            depth_max_mm: 8.0,
            curve_control_points: Some(vec![
                CurvePoint { x: 0.0, y: 0.0 },
                CurvePoint { x: 1.0, y: 1.0 },
            ]),
            step_x: 2,
            step_y: 2,
            target_width_mm: Some(50.0),
            target_height_mm: Some(70.0),
        };
        let json = serde_json::to_string(&p).unwrap();
        let loaded: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.schema_version, p.schema_version);
        assert!((loaded.brightness - p.brightness).abs() < 1e-6);
        assert_eq!(loaded.invert, p.invert);
        assert_eq!(loaded.curve_control_points.as_ref().unwrap().len(), 2);
        assert_eq!(loaded.step_x, 2);
        assert_eq!(loaded.target_width_mm, Some(50.0));
    }

    #[test]
    fn preset_to_depth_params() {
        let p = Preset::default();
        let dp = p.to_depth_params();
        assert!((dp.brightness - 0.0).abs() < 1e-6);
        assert!((dp.contrast - 1.0).abs() < 1e-6);
        assert_eq!(dp.depth_min_mm, 2.0);
        assert_eq!(dp.depth_max_mm, 10.0);
    }

    #[test]
    fn sanitize_preset_name_accepts_valid() {
        assert_eq!(sanitize_preset_name("My Preset").unwrap(), "My Preset");
        assert_eq!(sanitize_preset_name("  Portrait  ").unwrap(), "Portrait");
        assert_eq!(sanitize_preset_name("high-detail_01").unwrap(), "high-detail_01");
    }

    #[test]
    fn sanitize_preset_name_rejects_invalid() {
        assert!(sanitize_preset_name("").is_err());
        assert!(sanitize_preset_name("   ").is_err());
        assert!(sanitize_preset_name("../etc").is_err());
        assert!(sanitize_preset_name("a/b").is_err());
    }
}
