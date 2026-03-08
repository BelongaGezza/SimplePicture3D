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
    &[
        BUILTIN_PORTRAIT,
        BUILTIN_LANDSCAPE,
        BUILTIN_HIGH_DETAIL,
        BUILTIN_LOW_RELIEF,
    ]
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
            feather_radius_px: 0.0, // BACK-1203: presets don't persist feather; use 0
        }
    }

    /// Parse JSON and validate schema version (JR2-1303). Used by load_preset for file content.
    /// Returns user-facing error string on invalid JSON or unsupported schema version.
    pub fn parse_and_validate_json(contents: &str) -> Result<Self, String> {
        let preset: Preset =
            serde_json::from_str(contents).map_err(|e| format!("Invalid preset file: {}", e))?;
        if preset.schema_version > PRESET_SCHEMA_VERSION {
            return Err(format!(
                "Preset schema version {} is newer than supported ({})",
                preset.schema_version, PRESET_SCHEMA_VERSION
            ));
        }
        Ok(preset)
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

    /// JR2-1301: None curve round-trips cleanly; no curveControlPoints key in JSON.
    #[test]
    fn preset_roundtrip_no_curve_omits_key() {
        let p = Preset {
            schema_version: 1,
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
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(
            !json.contains("curveControlPoints"),
            "None curve should not serialize curveControlPoints key"
        );
        let loaded: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.curve_control_points, None);
    }

    /// JR2-1301: step_x/step_y default to 1 when absent from JSON (backwards compatibility).
    #[test]
    fn preset_deserialize_default_step_when_absent() {
        let json = r#"{"schemaVersion":1,"brightness":0,"contrast":1,"gamma":1,"invert":false,"depthMinMm":2.0,"depthMaxMm":10.0}"#;
        let loaded: Preset = serde_json::from_str(json).unwrap();
        assert_eq!(loaded.step_x, 1);
        assert_eq!(loaded.step_y, 1);
    }

    /// JR2-1301: schema_version is preserved in round-trip.
    #[test]
    fn preset_roundtrip_preserves_schema_version() {
        let p = Preset::default();
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("schemaVersion"));
        let loaded: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.schema_version, p.schema_version);
    }

    /// JR2-1301: Built-in presets return Some and to_depth_params() has valid ranges.
    #[test]
    fn builtin_presets_valid() {
        for id in builtin_preset_ids() {
            let p = get_builtin_preset(id).expect("builtin should exist");
            let dp = p.to_depth_params();
            assert!(
                dp.depth_min_mm < dp.depth_max_mm,
                "builtin {}: depth_min_mm < depth_max_mm",
                id
            );
            assert!(dp.gamma > 0.0, "builtin {}: gamma > 0", id);
            assert!(dp.contrast > 0.0, "builtin {}: contrast > 0", id);
        }
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

    /// JR2-1301: Round-trip default config (from_depth_and_mesh → JSON → deserialize → state matches).
    #[test]
    fn preset_roundtrip_from_depth_and_mesh_default() {
        let p =
            Preset::from_depth_and_mesh(0.0, 1.0, 1.0, false, 2.0, 10.0, None, 1, 1, None, None);
        let json = serde_json::to_string(&p).unwrap();
        let loaded: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.schema_version, p.schema_version);
        assert!((loaded.brightness - p.brightness).abs() < 1e-6);
        assert!((loaded.contrast - p.contrast).abs() < 1e-6);
        assert!((loaded.gamma - p.gamma).abs() < 1e-6);
        assert_eq!(loaded.invert, p.invert);
        assert!((loaded.depth_min_mm - p.depth_min_mm).abs() < 1e-6);
        assert!((loaded.depth_max_mm - p.depth_max_mm).abs() < 1e-6);
        assert_eq!(
            loaded.curve_control_points.is_none(),
            p.curve_control_points.is_none()
        );
        assert_eq!(loaded.step_x, p.step_x);
        assert_eq!(loaded.step_y, p.step_y);
        assert_eq!(loaded.target_width_mm, p.target_width_mm);
        assert_eq!(loaded.target_height_mm, p.target_height_mm);
        let orig_dp = p.to_depth_params();
        let loaded_dp = loaded.to_depth_params();
        assert!((orig_dp.brightness - loaded_dp.brightness).abs() < 1e-6);
        assert!((orig_dp.contrast - loaded_dp.contrast).abs() < 1e-6);
        assert_eq!(orig_dp.invert, loaded_dp.invert);
    }

    /// JR2-1301: Round-trip custom config with curve and target dimensions.
    #[test]
    fn preset_roundtrip_from_depth_and_mesh_custom_curve() {
        let curve = vec![
            CurvePoint { x: 0.0, y: 0.0 },
            CurvePoint { x: 0.5, y: 0.4 },
            CurvePoint { x: 1.0, y: 1.0 },
        ];
        let p = Preset::from_depth_and_mesh(
            0.1,
            1.2,
            1.3,
            true,
            3.0,
            9.0,
            Some(curve.clone()),
            2,
            3,
            Some(50.0),
            Some(70.0),
        );
        let json = serde_json::to_string(&p).unwrap();
        let loaded: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.schema_version, p.schema_version);
        assert!((loaded.brightness - p.brightness).abs() < 1e-6);
        assert_eq!(loaded.invert, p.invert);
        assert_eq!(loaded.step_x, 2);
        assert_eq!(loaded.step_y, 3);
        assert_eq!(loaded.target_width_mm, Some(50.0));
        assert_eq!(loaded.target_height_mm, Some(70.0));
        let loaded_curve = loaded.curve_control_points.as_ref().unwrap();
        assert_eq!(loaded_curve.len(), 3);
        assert!((loaded_curve[1].x - 0.5).abs() < 1e-6);
        assert!((loaded_curve[1].y - 0.4).abs() < 1e-6);
        let loaded_dp = loaded.to_depth_params();
        assert_eq!(loaded_dp.curve_control_points.as_ref().unwrap().len(), 3);
    }

    // --- JR2-1302: Invalid JSON / wrong schema ---

    #[test]
    fn preset_deserialize_rejects_malformed_json() {
        let err = serde_json::from_str::<Preset>("{").unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn preset_deserialize_rejects_empty_string() {
        let err = serde_json::from_str::<Preset>("").unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn preset_deserialize_rejects_missing_required_field() {
        // Valid shape but missing "brightness" (required).
        let json = r#"{"schemaVersion":1,"contrast":1.0,"gamma":1.0,"invert":false,"depthMinMm":2.0,"depthMaxMm":10.0}"#;
        let err = serde_json::from_str::<Preset>(json).unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn preset_deserialize_rejects_wrong_type() {
        // brightness as string instead of number.
        let json = r#"{"schemaVersion":1,"brightness":"high","contrast":1.0,"gamma":1.0,"invert":false,"depthMinMm":2.0,"depthMaxMm":10.0}"#;
        let err = serde_json::from_str::<Preset>(json).unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn preset_deserialize_rejects_not_an_object() {
        let err = serde_json::from_str::<Preset>("[1,2,3]").unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    // --- JR2-1303: Versioned schema ---

    #[test]
    fn preset_schema_version_1_accepted() {
        let p = Preset::default();
        assert_eq!(p.schema_version, PRESET_SCHEMA_VERSION);
        let json = serde_json::to_string(&p).unwrap();
        let loaded: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.schema_version, 1);
    }

    /// Older schema version 0 (same field shape) deserializes; no data loss for supported fields (JR2-1303).
    #[test]
    fn preset_schema_version_0_deserializes() {
        let json = r#"{"schemaVersion":0,"brightness":0.05,"contrast":1.1,"gamma":1.0,"invert":false,"depthMinMm":2.0,"depthMaxMm":10.0,"stepX":1,"stepY":1}"#;
        let loaded: Preset = serde_json::from_str(json).unwrap();
        assert_eq!(loaded.schema_version, 0);
        assert!((loaded.brightness - 0.05).abs() < 1e-6);
        assert!((loaded.contrast - 1.1).abs() < 1e-6);
        assert!(loaded.curve_control_points.is_none());
        assert_eq!(loaded.step_x, 1);
        assert_eq!(loaded.step_y, 1);
        // to_depth_params still works; loader can treat v0 as valid and apply (migration = accept).
        let dp = loaded.to_depth_params();
        assert!((dp.brightness - 0.05).abs() < 1e-6);
    }

    #[test]
    fn sanitize_preset_name_accepts_valid() {
        assert_eq!(sanitize_preset_name("My Preset").unwrap(), "My Preset");
        assert_eq!(sanitize_preset_name("  Portrait  ").unwrap(), "Portrait");
        assert_eq!(
            sanitize_preset_name("high-detail_01").unwrap(),
            "high-detail_01"
        );
    }

    #[test]
    fn sanitize_preset_name_rejects_invalid() {
        assert!(sanitize_preset_name("").is_err());
        assert!(sanitize_preset_name("   ").is_err());
        assert!(sanitize_preset_name("../etc").is_err());
        assert!(sanitize_preset_name("a/b").is_err());
    }

    // --- JR2-1301: Round-trip None curve, step defaults, built-in validity, sanitize edge cases ---

    /// JR2-1301: None curve control points round-trips; no curveControlPoints key in JSON.
    #[test]
    fn preset_roundtrip_none_curve_no_key_in_json() {
        let p = Preset {
            schema_version: 1,
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
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(
            !json.contains("curveControlPoints"),
            "None should not serialize curveControlPoints key"
        );
        let loaded: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.curve_control_points, None);
    }

    /// JR2-1301: step_x/step_y default to 1 when absent from JSON (backwards compatibility).
    #[test]
    fn preset_deserialize_defaults_step_when_absent() {
        let json = r#"{"schemaVersion":1,"brightness":0,"contrast":1,"gamma":1,"invert":false,"depthMinMm":2,"depthMaxMm":10}"#;
        let loaded: Preset = serde_json::from_str(json).unwrap();
        assert_eq!(loaded.step_x, 1);
        assert_eq!(loaded.step_y, 1);
    }

    /// JR2-1301: Each built-in preset returns Some and to_depth_params() has valid ranges.
    #[test]
    fn builtin_presets_valid_depth_params() {
        for id in builtin_preset_ids() {
            let p = get_builtin_preset(id).expect(&format!("builtin {} should exist", id));
            let dp = p.to_depth_params();
            assert!(
                dp.depth_min_mm < dp.depth_max_mm,
                "{}: depth_min_mm < depth_max_mm",
                id
            );
            assert!(dp.gamma > 0.0, "{}: gamma > 0", id);
            assert!(dp.contrast > 0.0, "{}: contrast > 0", id);
        }
    }

    /// JR2-1301: get_builtin_preset returns Some for each built-in id.
    #[test]
    fn builtin_preset_each_returns_some() {
        assert!(get_builtin_preset(BUILTIN_PORTRAIT).is_some());
        assert!(get_builtin_preset(BUILTIN_LANDSCAPE).is_some());
        assert!(get_builtin_preset(BUILTIN_HIGH_DETAIL).is_some());
        assert!(get_builtin_preset(BUILTIN_LOW_RELIEF).is_some());
        assert!(get_builtin_preset("Unknown").is_none());
    }

    /// JR2-1301: Name with only spaces → error.
    #[test]
    fn sanitize_preset_name_only_spaces_error() {
        assert!(sanitize_preset_name("   ").is_err());
        assert!(sanitize_preset_name("\t  \n").is_err());
    }

    /// JR2-1301: Unicode in name: only ASCII alphanumeric (and space, hyphen, underscore) pass through.
    #[test]
    fn sanitize_preset_name_unicode_stripped() {
        // "café" → non-ASCII stripped, result "caf"
        let out = sanitize_preset_name("café").unwrap();
        assert_eq!(out, "caf");
        // "Preset 名称" → "Preset " (trailing space then trim → "Preset")
        let out = sanitize_preset_name("Preset 名称").unwrap();
        assert_eq!(out, "Preset");
    }

    /// JR2-1301: Max-length boundary: 200 chars passes, 201 fails.
    #[test]
    fn sanitize_preset_name_max_length_boundary() {
        let ok_200: String = (0..200).map(|_| 'a').collect();
        assert_eq!(sanitize_preset_name(&ok_200).unwrap().len(), 200);
        let bad_201: String = (0..201).map(|_| 'a').collect();
        assert!(sanitize_preset_name(&bad_201).is_err());
    }

    // --- JR2-1303: load_preset error cases (parse_and_validate_json used by load_preset) ---

    /// JR2-1303: Invalid JSON returns user-facing error, no panic.
    #[test]
    fn parse_and_validate_json_invalid_json_returns_error() {
        let err = Preset::parse_and_validate_json("{").unwrap_err();
        assert!(!err.is_empty());
        assert!(err.contains("Invalid preset file") || err.to_lowercase().contains("invalid"));
    }

    /// JR2-1303: Missing required field (e.g. brightness) returns error.
    #[test]
    fn parse_and_validate_json_missing_required_returns_error() {
        let json = r#"{"schemaVersion":1,"contrast":1,"gamma":1,"invert":false,"depthMinMm":2,"depthMaxMm":10}"#;
        let err = Preset::parse_and_validate_json(json).unwrap_err();
        assert!(!err.is_empty());
    }

    /// JR2-1303: schemaVersion > 1 returns error (newer than supported).
    #[test]
    fn parse_and_validate_json_schema_version_2_rejected() {
        let json = r#"{"schemaVersion":2,"brightness":0,"contrast":1,"gamma":1,"invert":false,"depthMinMm":2,"depthMaxMm":10}"#;
        let err = Preset::parse_and_validate_json(json).unwrap_err();
        assert!(err.contains("newer than supported") || err.contains("schema version"));
    }

    /// JR2-1303: depthMinMm >= depthMaxMm — backend does not reject; deserializes and to_depth_params returns as-is (no clamp).
    #[test]
    fn preset_depth_min_ge_max_deserializes_no_reject() {
        let json = r#"{"schemaVersion":1,"brightness":0,"contrast":1,"gamma":1,"invert":false,"depthMinMm":10,"depthMaxMm":2}"#;
        let loaded: Preset = serde_json::from_str(json).unwrap();
        let dp = loaded.to_depth_params();
        assert!((dp.depth_min_mm - 10.0).abs() < 1e-6);
        assert!((dp.depth_max_mm - 2.0).abs() < 1e-6);
    }
}
