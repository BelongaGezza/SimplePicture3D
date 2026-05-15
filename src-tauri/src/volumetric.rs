// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! 3D surface-map point cloud generation (ADR-012).
//!
//! Generates a 3D surface point cloud from a depth map for internal UV laser
//! engraving of crystal glass. For each sampled (x, y) position above the depth
//! threshold the algorithm emits **exactly one point** at the Z position
//! determined by that pixel's depth value:
//!
//! ```text
//! for each sampled (px, py):
//!     d = depth[py * width + px].clamp(0.0, 1.0)
//!     if d < params.depth_threshold { continue }
//!     x_mm = (px / width)  * envelope.interior_length()  + envelope.margin_mm
//!     y_mm = (py / height) * envelope.interior_width()   + envelope.margin_mm
//!     z_mm = envelope.margin_mm + (1.0 - d) * envelope.interior_height()
//!     emit [x_mm, y_mm, z_mm]
//! ```
//!
//! Convention (matches the rest of the pipeline):
//! - `depth = 1.0` (near / foreground) → Z near the front face of the blank.
//! - `depth = 0.0` (far / background) → Z near the back face of the blank.
//! - Points with `depth < depth_threshold` are skipped so background noise does
//!   not get engraved.
//!
//! After point generation, [`fit_to_blank`] is applied to scale and centre the
//! cloud inside the [`BlankEnvelope`] (unchanged from previous behaviour).
//!
//! This module supersedes the ADR-011 column-sweep fill that previously lived
//! here. See `RESEARCH/architecture.md` § ADR-012 for the rationale.

use crate::blank_envelope::{fit_to_blank, BlankEnvelope, FitResult};
use serde::{Deserialize, Serialize};

/// Default minimum depth required to emit a point. Pixels below this value are
/// treated as background and skipped. ADR-012 calls for a small positive value
/// (≈ 0.05) so that pure-background pixels never produce engraving points but
/// realistic foreground/midground content is still captured.
pub const DEFAULT_DEPTH_THRESHOLD: f32 = 0.05;

/// Parameters for 3D surface-map point cloud generation (ADR-012).
///
/// Field semantics:
/// - `step_x` / `step_y` — pixel stride for XY sampling (1 = every pixel).
/// - `depth_threshold` — pixels with `depth < depth_threshold` are skipped.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumetricParams {
    /// Sample every N pixels in X direction (1 = full resolution). Must be > 0.
    pub step_x: u32,
    /// Sample every N pixels in Y direction (1 = full resolution). Must be > 0.
    pub step_y: u32,
    /// Minimum depth value [0.0, 1.0] required to emit a point. Pixels with a
    /// depth value below this threshold are treated as background and skipped.
    pub depth_threshold: f32,
}

impl Default for VolumetricParams {
    fn default() -> Self {
        Self {
            step_x: 1,
            step_y: 1,
            depth_threshold: DEFAULT_DEPTH_THRESHOLD,
        }
    }
}

/// Result of surface-map point cloud generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumetricResult {
    /// Generated 3D points (x, y, z) in mm, fitted to blank envelope.
    pub points: Vec<[f32; 3]>,
    /// Number of points generated.
    pub point_count: usize,
    /// Fit result from blank envelope scaling.
    pub fit_result: FitResult,
    /// Estimated memory usage in bytes.
    pub memory_bytes: usize,
}

/// Generate a 3D surface-map point cloud from a depth map (ADR-012).
///
/// For each sampled (x, y) position above [`VolumetricParams::depth_threshold`]
/// the function emits a single point at the depth-mapped Z position. There is
/// no column sweep / Z fill — exactly one point per accepted (x, y) sample.
///
/// After generation, points are scaled and centred inside the blank envelope
/// via [`fit_to_blank`].
///
/// # Arguments
/// * `depth` - Row-major depth map, normalized 0.0-1.0 (0=far, 1=near).
/// * `width` - Depth map width in pixels.
/// * `height` - Depth map height in pixels.
/// * `params` - Surface-map generation parameters.
/// * `envelope` - Target blank envelope for fitting.
///
/// # Returns
/// * `Ok(VolumetricResult)` - Generated point cloud with fit information.
/// * `Err(String)` - Invalid parameters, invalid envelope, or no points above
///   threshold.
pub fn generate_volumetric_points(
    depth: &[f32],
    width: u32,
    height: u32,
    params: &VolumetricParams,
    envelope: &BlankEnvelope,
) -> Result<VolumetricResult, String> {
    let expected_len = (width as usize)
        .checked_mul(height as usize)
        .ok_or("Depth map dimensions overflow")?;

    if depth.len() != expected_len {
        return Err(format!(
            "Depth map length {} doesn't match {}x{}={}",
            depth.len(),
            width,
            height,
            expected_len
        ));
    }

    if width == 0 || height == 0 {
        return Err("Depth map dimensions must be positive".to_string());
    }

    if params.step_x == 0 || params.step_y == 0 {
        return Err("Step size must be positive".to_string());
    }

    if !params.depth_threshold.is_finite()
        || params.depth_threshold < 0.0
        || params.depth_threshold > 1.0
    {
        return Err("Depth threshold must be in [0.0, 1.0]".to_string());
    }

    envelope.validate()?;

    let interior_length = envelope.interior_length();
    let interior_width = envelope.interior_width();
    let interior_height = envelope.interior_height();
    let margin = envelope.margin_mm;

    let width_f = width as f32;
    let height_f = height as f32;

    let num_cols = width.div_ceil(params.step_x);
    let num_rows = height.div_ceil(params.step_y);

    let capacity = (num_cols as usize).saturating_mul(num_rows as usize);
    let mut points: Vec<[f32; 3]> = Vec::with_capacity(capacity);

    let threshold = params.depth_threshold;

    for row in 0..num_rows {
        let py = (row * params.step_y).min(height - 1);
        let y_mm = (py as f32 / height_f) * interior_width + margin;

        for col in 0..num_cols {
            let px = (col * params.step_x).min(width - 1);

            let idx = (py as usize) * (width as usize) + (px as usize);
            let d = depth[idx].clamp(0.0, 1.0);

            if d < threshold {
                continue;
            }

            let x_mm = (px as f32 / width_f) * interior_length + margin;
            let z_mm = margin + (1.0 - d) * interior_height;

            points.push([x_mm, y_mm, z_mm]);
        }
    }

    if points.is_empty() {
        return Err("No points generated (all depth values are below depth_threshold)".to_string());
    }

    let fit_result = fit_to_blank(&mut points, envelope)?;

    let point_count = points.len();
    let memory_bytes = point_count * std::mem::size_of::<[f32; 3]>();

    Ok(VolumetricResult {
        points,
        point_count,
        fit_result,
        memory_bytes,
    })
}

/// Estimate the number of points that will be generated without performing the
/// full sampling pass. Uses the ADR-012 simplified formula:
///
/// ```text
/// num_cols * num_rows * (1.0 - depth_threshold)
/// ```
///
/// where `(1.0 - depth_threshold)` is a coarse approximation of the fraction of
/// pixels above threshold for a uniformly distributed depth map. Useful for
/// driving "estimated point count" UI before generation runs.
pub fn estimate_point_count(
    width: u32,
    height: u32,
    params: &VolumetricParams,
    _envelope: &BlankEnvelope,
) -> usize {
    if width == 0 || height == 0 || params.step_x == 0 || params.step_y == 0 {
        return 0;
    }

    let num_cols = width.div_ceil(params.step_x) as usize;
    let num_rows = height.div_ceil(params.step_y) as usize;
    let grid = num_cols.saturating_mul(num_rows);

    let threshold = params.depth_threshold.clamp(0.0, 1.0);
    let fraction = (1.0 - threshold).max(0.0) as f64;

    ((grid as f64) * fraction).round() as usize
}

/// Validate [`VolumetricParams`] without a depth map (e.g. Tauri `set_volumetric_params`).
pub fn validate_volumetric_params(params: &VolumetricParams) -> Result<(), String> {
    if params.step_x == 0 || params.step_y == 0 {
        return Err("Step size must be positive".to_string());
    }
    if !params.depth_threshold.is_finite()
        || params.depth_threshold < 0.0
        || params.depth_threshold > 1.0
    {
        return Err("Depth threshold must be in [0.0, 1.0]".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_flat_depth(width: u32, height: u32, value: f32) -> Vec<f32> {
        vec![value; (width * height) as usize]
    }

    fn make_gradient_depth(width: u32, height: u32) -> Vec<f32> {
        let mut depth = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                let d = ((x as f32 / width as f32) + (y as f32 / height as f32)) / 2.0;
                depth.push(d);
            }
        }
        depth
    }

    #[test]
    fn default_params_have_expected_values() {
        let p = VolumetricParams::default();
        assert_eq!(p.step_x, 1);
        assert_eq!(p.step_y, 1);
        assert!((p.depth_threshold - DEFAULT_DEPTH_THRESHOLD).abs() < f32::EPSILON);
    }

    #[test]
    fn generate_flat_depth_emits_one_point_per_sample() {
        // Flat depth 0.5 with threshold below 0.5 -> every sampled pixel emits one point.
        let depth = make_flat_depth(10, 10, 0.5);
        let params = VolumetricParams {
            step_x: 2,
            step_y: 2,
            depth_threshold: 0.0,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope).unwrap();

        // 10/2 = 5 cols, 5 rows -> exactly 25 points (one per sample).
        let expected = 5usize * 5usize;
        assert_eq!(result.point_count, expected);
        assert_eq!(result.points.len(), expected);
        assert_eq!(result.fit_result.outliers, 0);
    }

    #[test]
    fn generate_full_resolution_emits_width_times_height_points() {
        let depth = make_flat_depth(8, 6, 0.5);
        let params = VolumetricParams {
            step_x: 1,
            step_y: 1,
            depth_threshold: 0.0,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 8, 6, &params, &envelope).unwrap();
        assert_eq!(result.point_count, 8 * 6);
    }

    #[test]
    fn generate_gradient_keeps_points_inside_envelope() {
        let depth = make_gradient_depth(20, 20);
        let params = VolumetricParams {
            step_x: 1,
            step_y: 1,
            depth_threshold: 0.0,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 20, 20, &params, &envelope).unwrap();

        // Every (x,y) sample with d >= 0 is kept -> exactly 20*20 points.
        assert_eq!(result.point_count, 20 * 20);
        for p in &result.points {
            assert!(p[0] >= 0.0 && p[0] <= envelope.length_mm);
            assert!(p[1] >= 0.0 && p[1] <= envelope.width_mm);
            assert!(p[2] >= 0.0 && p[2] <= envelope.height_mm);
        }
    }

    #[test]
    fn threshold_skips_background_pixels() {
        // Half the pixels at d=0.0 (background), half at d=1.0 (foreground).
        // With threshold = 0.5, only the foreground half should be emitted.
        let mut depth = Vec::with_capacity(10 * 10);
        for y in 0..10u32 {
            for _x in 0..10u32 {
                let d = if y < 5 { 0.0 } else { 1.0 };
                depth.push(d);
            }
        }

        let params = VolumetricParams {
            step_x: 1,
            step_y: 1,
            depth_threshold: 0.5,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope).unwrap();
        assert_eq!(result.point_count, 5 * 10);
    }

    #[test]
    fn threshold_one_skips_everything_returns_error() {
        // depth values are 0.5 everywhere; threshold of 1.0 rejects every pixel.
        let depth = make_flat_depth(10, 10, 0.5);
        let params = VolumetricParams {
            step_x: 1,
            step_y: 1,
            depth_threshold: 1.0,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope);
        assert!(result.is_err());
    }

    #[test]
    fn threshold_default_keeps_above_default() {
        // Pixels at d=0.0 (below default 0.05) and d=0.5 (above). Only the 0.5
        // pixels should be emitted with default params.
        let mut depth = Vec::with_capacity(4 * 4);
        for i in 0..16u32 {
            depth.push(if i < 8 { 0.0 } else { 0.5 });
        }

        let params = VolumetricParams::default();
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 4, 4, &params, &envelope).unwrap();
        assert_eq!(result.point_count, 8);
    }

    #[test]
    fn step_reduces_point_count() {
        let depth = make_flat_depth(8, 8, 0.5);
        let envelope = BlankEnvelope::default();

        let p_full = VolumetricParams {
            step_x: 1,
            step_y: 1,
            depth_threshold: 0.0,
        };
        let r_full = generate_volumetric_points(&depth, 8, 8, &p_full, &envelope).unwrap();
        assert_eq!(r_full.point_count, 8 * 8);

        let p_step2 = VolumetricParams {
            step_x: 2,
            step_y: 2,
            depth_threshold: 0.0,
        };
        let r_step2 = generate_volumetric_points(&depth, 8, 8, &p_step2, &envelope).unwrap();
        assert_eq!(r_step2.point_count, 4 * 4);

        let p_step4 = VolumetricParams {
            step_x: 4,
            step_y: 4,
            depth_threshold: 0.0,
        };
        let r_step4 = generate_volumetric_points(&depth, 8, 8, &p_step4, &envelope).unwrap();
        assert_eq!(r_step4.point_count, 2 * 2);
    }

    #[test]
    fn step_handles_non_divisor_dimensions() {
        // width=10, step_x=3 -> ceil(10/3) = 4 columns sampled (px = 0, 3, 6, 9).
        let depth = make_flat_depth(10, 5, 0.5);
        let params = VolumetricParams {
            step_x: 3,
            step_y: 2,
            depth_threshold: 0.0,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 5, &params, &envelope).unwrap();
        // ceil(10/3) = 4 cols, ceil(5/2) = 3 rows -> 12 points.
        assert_eq!(result.point_count, 4 * 3);
    }

    #[test]
    fn reject_empty_depth() {
        let depth: Vec<f32> = vec![];
        let params = VolumetricParams::default();
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 0, 0, &params, &envelope);
        assert!(result.is_err());
    }

    #[test]
    fn reject_mismatched_depth_length() {
        let depth = vec![0.5; 50];
        let params = VolumetricParams::default();
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope);
        assert!(result.is_err());
    }

    #[test]
    fn reject_invalid_step() {
        let depth = make_flat_depth(10, 10, 0.5);
        let params = VolumetricParams {
            step_x: 0,
            step_y: 1,
            depth_threshold: 0.0,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope);
        assert!(result.is_err());
    }

    #[test]
    fn reject_negative_threshold() {
        let depth = make_flat_depth(10, 10, 0.5);
        let params = VolumetricParams {
            step_x: 1,
            step_y: 1,
            depth_threshold: -0.1,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope);
        assert!(result.is_err());
    }

    #[test]
    fn reject_threshold_above_one() {
        let depth = make_flat_depth(10, 10, 0.5);
        let params = VolumetricParams {
            step_x: 1,
            step_y: 1,
            depth_threshold: 1.5,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope);
        assert!(result.is_err());
    }

    #[test]
    fn reject_invalid_envelope() {
        let depth = make_flat_depth(10, 10, 0.5);
        let params = VolumetricParams::default();
        // Margin too large for blank size.
        let envelope = BlankEnvelope::new(10.0, 10.0, 10.0, 6.0);

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope);
        assert!(result.is_err());
    }

    #[test]
    fn z_increases_with_decreasing_depth() {
        // Two pixels with different depths -> verify that the smaller-depth
        // pixel ends up with the larger Z (near back face) before fit_to_blank
        // is applied. Because fit_to_blank scales/translates uniformly, the
        // relative ordering on Z is preserved.
        let depth = vec![0.9, 0.1];
        let params = VolumetricParams {
            step_x: 1,
            step_y: 1,
            depth_threshold: 0.0,
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 2, 1, &params, &envelope).unwrap();
        assert_eq!(result.point_count, 2);
        // First pixel (d=0.9, near) should have smaller Z than second (d=0.1, far).
        assert!(result.points[0][2] < result.points[1][2]);
    }

    #[test]
    fn estimate_matches_actual_for_threshold_zero() {
        // With threshold = 0.0, every sampled pixel is emitted, so the estimator
        // (which uses 1 - threshold as the fraction) should equal the grid size.
        let params = VolumetricParams {
            step_x: 2,
            step_y: 2,
            depth_threshold: 0.0,
        };
        let envelope = BlankEnvelope::default();

        let estimate = estimate_point_count(100, 100, &params, &envelope);
        // 100/2 * 100/2 = 50 * 50 = 2500.
        assert_eq!(estimate, 2500);
    }

    #[test]
    fn estimate_scales_with_threshold() {
        let envelope = BlankEnvelope::default();

        let lo = estimate_point_count(
            100,
            100,
            &VolumetricParams {
                step_x: 1,
                step_y: 1,
                depth_threshold: 0.1,
            },
            &envelope,
        );
        let hi = estimate_point_count(
            100,
            100,
            &VolumetricParams {
                step_x: 1,
                step_y: 1,
                depth_threshold: 0.9,
            },
            &envelope,
        );

        assert!(lo > hi);
        // 0.9 fraction of 10000 = 9000; 0.1 fraction of 10000 = 1000.
        assert_eq!(lo, 9000);
        assert_eq!(hi, 1000);
    }

    #[test]
    fn estimate_handles_zero_dimensions() {
        let params = VolumetricParams::default();
        let envelope = BlankEnvelope::default();
        assert_eq!(estimate_point_count(0, 100, &params, &envelope), 0);
        assert_eq!(estimate_point_count(100, 0, &params, &envelope), 0);
    }

    #[test]
    fn estimate_handles_zero_step() {
        let params = VolumetricParams {
            step_x: 0,
            step_y: 1,
            depth_threshold: 0.0,
        };
        let envelope = BlankEnvelope::default();
        assert_eq!(estimate_point_count(100, 100, &params, &envelope), 0);
    }

    #[test]
    fn json_roundtrip_params() {
        let params = VolumetricParams {
            step_x: 2,
            step_y: 3,
            depth_threshold: 0.25,
        };
        let json = serde_json::to_string(&params).unwrap();
        // camelCase rename should expose depthThreshold in the JSON payload.
        assert!(json.contains("depthThreshold"));
        let loaded: VolumetricParams = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.step_x, 2);
        assert_eq!(loaded.step_y, 3);
        assert!((loaded.depth_threshold - 0.25).abs() < f32::EPSILON);
    }
}
