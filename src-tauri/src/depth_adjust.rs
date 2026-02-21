// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Depth map adjustment pipeline (BACK-401–405).
//!
//! Transforms normalized depth [0, 1] with brightness, contrast, gamma, and invert.
//! All operations work on f32 in [0, 1]; output is clamped to [0, 1].
//! Order of application: invert → gamma → contrast → brightness (documented for predictable behaviour).
//!
//! Formulas (BACK-401):
//! - Brightness: v' = clamp(v + b, 0, 1)
//! - Contrast:   v' = clamp((v - 0.5) * c + 0.5, 0, 1); c=1 is identity
//! - Gamma:      v' = v^g for v > 0; 0 stays 0
//! - Invert:     v' = 1.0 - v

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

/// User-adjustable parameters for depth map display and future mesh/export (BACK-401, BACK-404).
/// Range [depth_min_mm, depth_max_mm] is stored for mesh generation; preview uses normalized 0–1.
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
        }
    }
}

/// Apply full pipeline in order: invert → gamma → contrast → brightness (BACK-402, BACK-403).
/// Does not mutate `depth`; returns a new Vec. Single pass over the array.
pub fn apply_adjustments(depth: &[f32], params: &DepthAdjustmentParams) -> Vec<f32> {
    depth
        .iter()
        .map(|&v| {
            let v = if params.invert { invert(v) } else { v };
            let v = gamma(v, params.gamma);
            let v = contrast(v, params.contrast);
            brightness(v, params.brightness)
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
}
