// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Depth map adjustment pipeline (BACK-401–405, BACK-1101–1104).
//!
//! Transforms normalized depth [0, 1] with brightness, contrast, gamma, invert, and optional curve.
//! All operations work on f32 in [0, 1]; output is clamped to [0, 1].
//! Order of application: invert → gamma → contrast → brightness → curve (documented for predictable behaviour).
//!
//! Formulas (BACK-401):
//! - Brightness: v' = clamp(v + b, 0, 1)
//! - Contrast:   v' = clamp((v - 0.5) * c + 0.5, 0, 1); c=1 is identity
//! - Gamma:      v' = v^g for v > 0; 0 stays 0
//! - Invert:     v' = 1.0 - v
//! - Curve:      v' = interpolate(v; control points) — BACK-1102, BACK-1103

use serde::{Deserialize, Serialize};

/// Clamp value to [0, 1].
#[inline]
fn clamp01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

/// Brightness: additive offset. v' = clamp(v + b, 0, 1).
#[inline]
pub fn brightness(v: f32, b: f32) -> f32 {
    clamp01(v + b)
}

/// Contrast: scale around midpoint 0.5. v' = clamp((v - 0.5) * c + 0.5, 0, 1). c=1 is identity.
#[inline]
pub fn contrast(v: f32, c: f32) -> f32 {
    clamp01((v - 0.5) * c + 0.5)
}

/// Gamma: power-law curve. v' = v^g for v > 0; 0 stays 0. Output clamped to [0, 1].
#[inline]
pub fn gamma(v: f32, g: f32) -> f32 {
    if v <= 0.0 {
        0.0
    } else {
        clamp01(v.powf(g))
    }
}

/// Invert: near ↔ far. v' = 1.0 - v.
#[inline]
pub fn invert(v: f32) -> f32 {
    1.0 - v
}

/// Single control point for curve remapping (BACK-1102). x = input depth [0,1], y = output [0,1].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurvePoint {
    pub x: f32,
    pub y: f32,
}

/// Preset curve names (BACK-1102, UI-1104).
pub const PRESET_LINEAR: &str = "linear";
pub const PRESET_S_CURVE: &str = "s-curve";
pub const PRESET_EXPONENTIAL: &str = "exponential";

/// Linear (identity) curve: input maps to output 1:1.
pub fn preset_linear() -> Vec<CurvePoint> {
    vec![CurvePoint { x: 0.0, y: 0.0 }, CurvePoint { x: 1.0, y: 1.0 }]
}

/// S-curve: compresses shadows/highlights, expands midtones.
pub fn preset_s_curve() -> Vec<CurvePoint> {
    vec![
        CurvePoint { x: 0.0, y: 0.0 },
        CurvePoint { x: 0.25, y: 0.12 },
        CurvePoint { x: 0.5, y: 0.5 },
        CurvePoint { x: 0.75, y: 0.88 },
        CurvePoint { x: 1.0, y: 1.0 },
    ]
}

/// Exponential: lifts shadows (e.g. y = x^0.6 style).
pub fn preset_exponential() -> Vec<CurvePoint> {
    vec![
        CurvePoint { x: 0.0, y: 0.0 },
        CurvePoint { x: 0.5, y: 0.32 },
        CurvePoint { x: 1.0, y: 1.0 },
    ]
}

/// Apply curve remapping: map v in [0,1] using piecewise linear interpolation (BACK-1103).
/// Points must be sorted by x; if empty or single point, returns v unchanged.
#[inline]
pub fn apply_curve_value(v: f32, points: &[CurvePoint]) -> f32 {
    if points.len() < 2 {
        return clamp01(v);
    }
    let v = clamp01(v);
    // Find segment: points[i].x <= v <= points[i+1].x
    let mut i = 0;
    for (j, p) in points.iter().enumerate() {
        if p.x >= v {
            break;
        }
        i = j;
    }
    if i >= points.len() - 1 {
        return clamp01(points[points.len() - 1].y);
    }
    let p0 = &points[i];
    let p1 = &points[i + 1];
    let dx = p1.x - p0.x;
    let t = if dx <= 0.0 { 1.0 } else { (v - p0.x) / dx };
    clamp01(p0.y + t * (p1.y - p0.y))
}

/// Compute histogram of depth values (BACK-1101). Returns bin counts for [0, 1] divided into `bins` buckets.
pub fn compute_histogram(depth: &[f32], bins: usize) -> Vec<u32> {
    let bins = bins.max(1);
    let mut counts = vec![0u32; bins];
    for &v in depth {
        if !v.is_nan() && v.is_finite() {
            let v = v.clamp(0.0, 1.0);
            let idx = (v * (bins as f32)).min((bins - 1) as f32) as usize;
            counts[idx] = counts[idx].saturating_add(1);
        }
    }
    counts
}

/// User-adjustable parameters for depth map display and future mesh/export (BACK-401, BACK-404, BACK-1102).
/// Range [depth_min_mm, depth_max_mm] is stored for mesh generation; preview uses normalized 0–1.
/// Optional curve: when Some, applied after brightness (BACK-1103).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthAdjustmentParams {
    /// Brightness offset, typically in [-0.5, 0.5].
    pub brightness: f32,
    /// Contrast factor; 1.0 = no change.
    pub contrast: f32,
    /// Gamma exponent; 1.0 = linear.
    pub gamma: f32,
    /// Invert depth (near ↔ far).
    pub invert: bool,
    /// Depth range minimum in mm (e.g. 2 for laser engraving).
    pub depth_min_mm: f32,
    /// Depth range maximum in mm (e.g. 10).
    pub depth_max_mm: f32,
    /// Optional curve control points (BACK-1102). When None or len < 2, no curve (identity).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curve_control_points: Option<Vec<CurvePoint>>,
}

impl Default for DepthAdjustmentParams {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 1.0,
            gamma: 1.0,
            invert: false,
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            curve_control_points: None,
        }
    }
}

/// Apply full pipeline in order: invert → gamma → contrast → brightness → curve (BACK-402, BACK-403, BACK-1103).
/// Does not mutate `depth`; returns a new Vec. Single pass over the array.
pub fn apply_adjustments(depth: &[f32], params: &DepthAdjustmentParams) -> Vec<f32> {
    let curve_pts = params
        .curve_control_points
        .as_ref()
        .filter(|p| p.len() >= 2);
    depth
        .iter()
        .map(|&v| {
            let v = if params.invert { invert(v) } else { v };
            let v = gamma(v, params.gamma);
            let v = contrast(v, params.contrast);
            let v = brightness(v, params.brightness);
            if let Some(pts) = curve_pts {
                apply_curve_value(v, pts)
            } else {
                v
            }
        })
        .collect()
}

/// Map normalized depth [0, 1] to physical range [min_mm, max_mm] (BACK-404).
/// Formula: z_mm = min_mm + v * (max_mm - min_mm). Used by mesh/export pipeline.
#[inline]
pub fn depth_to_mm(v: f32, min_mm: f32, max_mm: f32) -> f32 {
    min_mm + v * (max_mm - min_mm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_shifts_values() {
        assert!((brightness(0.5, 0.2) - 0.7).abs() < 1e-6);
        assert_eq!(brightness(0.0, 0.5), 0.5);
        assert_eq!(brightness(1.0, -0.3), 0.7);
        assert_eq!(brightness(0.9, 0.2), 1.0);
        assert_eq!(brightness(0.1, -0.2), 0.0);
    }

    #[test]
    fn contrast_identity_at_one() {
        for v in [0.0_f32, 0.25, 0.5, 0.75, 1.0] {
            assert!((contrast(v, 1.0) - v).abs() < 1e-6, "v={}", v);
        }
    }

    #[test]
    fn contrast_expands_midtones() {
        // c > 1: midtones spread; 0.5 stays 0.5
        assert!((contrast(0.5, 2.0) - 0.5).abs() < 1e-6);
        assert!(contrast(0.4, 2.0) < 0.4);
        assert!(contrast(0.6, 2.0) > 0.6);
    }

    #[test]
    fn gamma_one_is_identity() {
        for v in [0.0_f32, 0.25, 0.5, 0.75, 1.0] {
            assert!((gamma(v, 1.0) - v).abs() < 1e-6, "v={}", v);
        }
    }

    #[test]
    fn gamma_darkens_midtones_when_greater_than_one() {
        let v = 0.5;
        assert!(gamma(v, 2.0) < v);
        assert!(gamma(v, 0.5) > v);
    }

    #[test]
    fn gamma_zero_stays_zero() {
        assert_eq!(gamma(0.0, 0.5), 0.0);
        assert_eq!(gamma(0.0, 2.0), 0.0);
    }

    #[test]
    fn invert_flips_values() {
        assert_eq!(invert(0.0), 1.0);
        assert_eq!(invert(1.0), 0.0);
        assert!((invert(0.5) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn pipeline_output_in_0_1() {
        let depth = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let params = DepthAdjustmentParams {
            brightness: 0.1,
            contrast: 1.5,
            gamma: 0.8,
            invert: true,
            ..Default::default()
        };
        let out = apply_adjustments(&depth, &params);
        for &v in &out {
            assert!(
                (0.0..=1.0).contains(&v),
                "output must be in [0,1], got {}",
                v
            );
        }
    }

    #[test]
    fn pipeline_default_is_identity() {
        let depth = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let out = apply_adjustments(&depth, &DepthAdjustmentParams::default());
        for (a, b) in depth.iter().zip(out.iter()) {
            assert!(
                (a - b).abs() < 1e-5,
                "default params should be identity: {} vs {}",
                a,
                b
            );
        }
    }

    #[test]
    fn pipeline_invert_only() {
        let depth = vec![0.0, 0.5, 1.0];
        let params = DepthAdjustmentParams {
            invert: true,
            ..Default::default()
        };
        let out = apply_adjustments(&depth, &params);
        assert_eq!(out[0], 1.0);
        assert!((out[1] - 0.5).abs() < 1e-6);
        assert_eq!(out[2], 0.0);
    }

    #[test]
    fn depth_to_mm_linear() {
        assert!((depth_to_mm(0.0, 2.0, 10.0) - 2.0).abs() < 1e-6);
        assert!((depth_to_mm(1.0, 2.0, 10.0) - 10.0).abs() < 1e-6);
        assert!((depth_to_mm(0.5, 2.0, 10.0) - 6.0).abs() < 1e-6);
    }

    // --- JR2-402: Boundary conditions ---

    #[test]
    fn boundary_all_zeros() {
        let depth: Vec<f32> = vec![0.0; 100];
        let params = DepthAdjustmentParams {
            brightness: 0.5,
            contrast: 2.0,
            gamma: 0.1,
            invert: true,
            ..Default::default()
        };
        let out = apply_adjustments(&depth, &params);
        for (i, &v) in out.iter().enumerate() {
            assert!(
                v >= 0.0 && v <= 1.0,
                "index {}: output must be in [0,1], got {}",
                i,
                v
            );
            assert!(!v.is_nan(), "index {}: must not be NaN", i);
        }
        // Invert(0)=1, then gamma(1)=1, contrast(1)=1, brightness(1+0.5)=1.0 clamped
        assert_eq!(out[0], 1.0);
    }

    #[test]
    fn boundary_all_ones() {
        let depth: Vec<f32> = vec![1.0; 100];
        let params = DepthAdjustmentParams {
            brightness: -0.5,
            contrast: 0.5,
            gamma: 5.0,
            invert: false,
            ..Default::default()
        };
        let out = apply_adjustments(&depth, &params);
        for (i, &v) in out.iter().enumerate() {
            assert!(
                v >= 0.0 && v <= 1.0,
                "index {}: output must be in [0,1], got {}",
                i,
                v
            );
            assert!(!v.is_nan(), "index {}: must not be NaN", i);
        }
    }

    #[test]
    fn boundary_mixed_values() {
        let depth = vec![0.0, 0.001, 0.5, 0.999, 1.0];
        let params = DepthAdjustmentParams {
            brightness: 1.0,
            contrast: 0.01,
            gamma: 0.1,
            invert: true,
            ..Default::default()
        };
        let out = apply_adjustments(&depth, &params);
        for (i, &v) in out.iter().enumerate() {
            assert!(
                v >= 0.0 && v <= 1.0,
                "index {}: output must be in [0,1], got {}",
                i,
                v
            );
            assert!(!v.is_nan(), "index {}: must not be NaN", i);
        }
    }

    #[test]
    fn boundary_extreme_brightness() {
        // Brightness +1: everything shifts to 1 (clamped). Brightness -1: to 0.
        let depth = vec![0.0, 0.5, 1.0];
        let out_plus = apply_adjustments(
            &depth,
            &DepthAdjustmentParams {
                brightness: 1.0,
                ..Default::default()
            },
        );
        let out_minus = apply_adjustments(
            &depth,
            &DepthAdjustmentParams {
                brightness: -1.0,
                ..Default::default()
            },
        );
        for &v in &out_plus {
            assert!(v >= 0.0 && v <= 1.0 && !v.is_nan());
        }
        assert_eq!(out_plus[0], 1.0);
        assert_eq!(out_plus[1], 1.0);
        assert_eq!(out_plus[2], 1.0);
        for &v in &out_minus {
            assert!(v >= 0.0 && v <= 1.0 && !v.is_nan());
        }
        assert_eq!(out_minus[0], 0.0);
        assert_eq!(out_minus[1], 0.0);
        assert_eq!(out_minus[2], 0.0);
    }

    #[test]
    fn boundary_extreme_gamma() {
        // Gamma 0.1: very steep curve (low values stay low, high values pulled). Gamma 5: flattens.
        let depth = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let params_low = DepthAdjustmentParams {
            gamma: 0.1,
            ..Default::default()
        };
        let params_high = DepthAdjustmentParams {
            gamma: 5.0,
            ..Default::default()
        };
        let out_low = apply_adjustments(&depth, &params_low);
        let out_high = apply_adjustments(&depth, &params_high);
        for &v in out_low.iter().chain(out_high.iter()) {
            assert!(v >= 0.0 && v <= 1.0, "output must be in [0,1], got {}", v);
            assert!(!v.is_nan());
        }
        assert_eq!(out_low[0], 0.0);
        assert_eq!(out_high[0], 0.0);
    }

    #[test]
    fn boundary_extreme_contrast() {
        // Contrast 0: everything becomes 0.5. Very high contrast: pushes toward 0/1.
        let depth = vec![0.25, 0.5, 0.75];
        let out_zero = apply_adjustments(
            &depth,
            &DepthAdjustmentParams {
                contrast: 0.0,
                ..Default::default()
            },
        );
        for &v in &out_zero {
            assert!(
                (v - 0.5).abs() < 1e-5,
                "contrast 0 should give 0.5, got {}",
                v
            );
            assert!(!v.is_nan());
        }
        let out_high = apply_adjustments(
            &depth,
            &DepthAdjustmentParams {
                contrast: 100.0,
                ..Default::default()
            },
        );
        for &v in &out_high {
            assert!(v >= 0.0 && v <= 1.0 && !v.is_nan());
        }
    }

    // --- BACK-1102, BACK-1103: Curve ---

    #[test]
    fn curve_linear_is_identity() {
        let linear = preset_linear();
        for v in [0.0_f32, 0.25, 0.5, 0.75, 1.0] {
            assert!(
                (apply_curve_value(v, &linear) - v).abs() < 1e-6,
                "linear curve at {}",
                v
            );
        }
    }

    #[test]
    fn curve_empty_or_single_returns_clamped() {
        assert!((apply_curve_value(0.5, &[]) - 0.5).abs() < 1e-6);
        assert!((apply_curve_value(0.8, &[CurvePoint { x: 0.5, y: 0.5 }]) - 0.8).abs() < 1e-6);
        assert!(apply_curve_value(1.5, &[]) <= 1.0);
    }

    #[test]
    fn curve_piecewise_linear() {
        let pts = vec![
            CurvePoint { x: 0.0, y: 0.0 },
            CurvePoint { x: 0.5, y: 0.25 },
            CurvePoint { x: 1.0, y: 1.0 },
        ];
        assert!((apply_curve_value(0.0, &pts) - 0.0).abs() < 1e-6);
        assert!((apply_curve_value(1.0, &pts) - 1.0).abs() < 1e-6);
        assert!((apply_curve_value(0.5, &pts) - 0.25).abs() < 1e-6);
        assert!((apply_curve_value(0.25, &pts) - 0.125).abs() < 1e-5);
    }

    #[test]
    fn pipeline_with_curve_linear_unchanged() {
        let depth = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let params = DepthAdjustmentParams {
            curve_control_points: Some(preset_linear()),
            ..Default::default()
        };
        let out = apply_adjustments(&depth, &params);
        for (a, b) in depth.iter().zip(out.iter()) {
            assert!((a - b).abs() < 1e-5, "with linear curve: {} vs {}", a, b);
        }
    }

    #[test]
    fn histogram_bins() {
        let depth = vec![0.0, 0.0, 0.5, 0.5, 0.5, 1.0];
        let h = compute_histogram(&depth, 4);
        assert_eq!(h.len(), 4);
        assert_eq!(h[0], 2); // [0, 0.25)
        assert_eq!(h[1], 0); // [0.25, 0.5)
        assert_eq!(h[2], 3); // [0.5, 0.75)
        assert_eq!(h[3], 1); // [0.75, 1.0]
    }

    #[test]
    fn histogram_256_bins() {
        let depth: Vec<f32> = (0..1000).map(|i| i as f32 / 1000.0).collect();
        let h = compute_histogram(&depth, 256);
        assert_eq!(h.len(), 256);
        assert!(h.iter().all(|&c| c <= 4)); // roughly 1000/256 per bin
    }
}
