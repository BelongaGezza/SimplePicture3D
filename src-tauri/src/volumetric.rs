// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Volumetric point cloud generation (ADR-011).
//!
//! Generates dense 3D point clouds from depth maps using the column sweep algorithm.
//! Points are distributed through the interior volume of a crystal blank, not just
//! on a surface.

use crate::blank_envelope::{fit_to_blank, BlankEnvelope, FitResult};
use serde::{Deserialize, Serialize};

/// Parameters for volumetric point cloud generation (ADR-011).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumetricParams {
    /// Sample every N pixels in X direction (1 = full resolution).
    pub step_x: u32,
    /// Sample every N pixels in Y direction (1 = full resolution).
    pub step_y: u32,
    /// Spacing between points along Z axis in mm.
    pub z_spacing_mm: f32,
    /// Minimum depth value to include (0.0-1.0, normalized).
    pub depth_min: f32,
    /// Maximum depth value to include (0.0-1.0, normalized).
    pub depth_max: f32,
    /// Back plane position as fraction of blank height (0.0 = bottom, 1.0 = top).
    pub back_plane: f32,
}

impl Default for VolumetricParams {
    fn default() -> Self {
        Self {
            step_x: 1,
            step_y: 1,
            z_spacing_mm: 0.5,
            depth_min: 0.0,
            depth_max: 1.0,
            back_plane: 0.0,
        }
    }
}

/// Result of volumetric point cloud generation.
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

/// Generate a volumetric point cloud from a depth map using column sweep.
///
/// # Algorithm (Column Sweep)
/// For each sampled (x, y) position:
/// 1. Get the depth value from the depth map
/// 2. Convert depth to a front surface Z position
/// 3. Generate points along Z from back plane to front surface
/// 4. Space points according to z_spacing_mm
///
/// After generation, points are fitted to the blank envelope using uniform scaling.
///
/// # Arguments
/// * `depth` - Row-major depth map, normalized 0.0-1.0 (0=far, 1=near)
/// * `width` - Depth map width in pixels
/// * `height` - Depth map height in pixels
/// * `params` - Volumetric generation parameters
/// * `envelope` - Target blank envelope for fitting
///
/// # Returns
/// * `Ok(VolumetricResult)` - Generated point cloud with fit information
/// * `Err(String)` - Invalid parameters or depth map
pub fn generate_volumetric_points(
    depth: &[f32],
    width: u32,
    height: u32,
    params: &VolumetricParams,
    envelope: &BlankEnvelope,
) -> Result<VolumetricResult, String> {
    // Validate inputs
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

    if params.z_spacing_mm <= 0.0 {
        return Err("Z spacing must be positive".to_string());
    }

    envelope.validate()?;

    // Calculate grid dimensions
    let num_cols = (width + params.step_x - 1) / params.step_x;
    let num_rows = (height + params.step_y - 1) / params.step_y;

    // Working space dimensions (before fitting)
    // We'll generate points in a normalized space, then fit to blank
    let work_width = width as f32;
    let work_height = height as f32;
    let work_depth = envelope.interior_height(); // Use interior height as working depth

    // Pre-allocate points (estimate: average ~5 points per column)
    let estimated_points = (num_cols * num_rows * 5) as usize;
    let mut points: Vec<[f32; 3]> = Vec::with_capacity(estimated_points);

    // Back plane Z position in working space
    let back_z = params.back_plane * work_depth;

    // Column sweep: iterate over sampled (x, y) positions
    for row in 0..num_rows {
        let py = (row * params.step_y).min(height - 1);
        let y_mm = (py as f32 / work_height) * work_height;

        for col in 0..num_cols {
            let px = (col * params.step_x).min(width - 1);
            let x_mm = (px as f32 / work_width) * work_width;

            // Get depth value at this position
            let idx = (py as usize) * (width as usize) + (px as usize);
            let d = depth[idx].clamp(0.0, 1.0);

            // Skip if outside depth range
            if d < params.depth_min || d > params.depth_max {
                continue;
            }

            // Convert depth to front surface Z position
            // depth=0 (far) -> back, depth=1 (near) -> front
            let front_z = back_z + d * (work_depth - back_z);

            // Generate points along Z from back to front
            let mut z = back_z;
            while z <= front_z {
                points.push([x_mm, y_mm, z]);
                z += params.z_spacing_mm;
            }

            // Always include the front surface point
            if points.last().map_or(true, |p| (p[2] - front_z).abs() > 0.001) {
                points.push([x_mm, y_mm, front_z]);
            }
        }
    }

    if points.is_empty() {
        return Err("No points generated (check depth range parameters)".to_string());
    }

    // Fit points to blank envelope
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

/// Estimate the number of points that will be generated without actually generating them.
///
/// Useful for UI to show estimated point count before generation.
pub fn estimate_point_count(
    width: u32,
    height: u32,
    params: &VolumetricParams,
    envelope: &BlankEnvelope,
) -> usize {
    let num_cols = (width + params.step_x - 1) / params.step_x;
    let num_rows = (height + params.step_y - 1) / params.step_y;
    let avg_depth = (params.depth_max - params.depth_min) / 2.0;
    let work_depth = envelope.interior_height();
    let points_per_column = (avg_depth * work_depth / params.z_spacing_mm) as usize + 1;

    (num_cols as usize) * (num_rows as usize) * points_per_column
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
                // Gradient from 0 (top-left) to 1 (bottom-right)
                let d = ((x as f32 / width as f32) + (y as f32 / height as f32)) / 2.0;
                depth.push(d);
            }
        }
        depth
    }

    #[test]
    fn generate_flat_depth() {
        let depth = make_flat_depth(10, 10, 0.5);
        let params = VolumetricParams {
            step_x: 2,
            step_y: 2,
            z_spacing_mm: 1.0,
            ..Default::default()
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope).unwrap();

        assert!(result.point_count > 0);
        assert_eq!(result.points.len(), result.point_count);
        assert_eq!(result.fit_result.outliers, 0);
    }

    #[test]
    fn generate_gradient_depth() {
        let depth = make_gradient_depth(20, 20);
        let params = VolumetricParams::default();
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 20, 20, &params, &envelope).unwrap();

        assert!(result.point_count > 0);
        // All points should be within envelope after fitting
        for p in &result.points {
            assert!(p[0] >= 0.0 && p[0] <= envelope.length_mm);
            assert!(p[1] >= 0.0 && p[1] <= envelope.width_mm);
            assert!(p[2] >= 0.0 && p[2] <= envelope.height_mm);
        }
    }

    #[test]
    fn generate_with_depth_range() {
        let depth = make_gradient_depth(10, 10);
        let params = VolumetricParams {
            step_x: 1,
            step_y: 1,
            z_spacing_mm: 1.0,
            depth_min: 0.3,
            depth_max: 0.7,
            ..Default::default()
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope).unwrap();
        assert!(result.point_count > 0);
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
    fn reject_invalid_step() {
        let depth = make_flat_depth(10, 10, 0.5);
        let params = VolumetricParams {
            step_x: 0,
            ..Default::default()
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope);
        assert!(result.is_err());
    }

    #[test]
    fn reject_invalid_z_spacing() {
        let depth = make_flat_depth(10, 10, 0.5);
        let params = VolumetricParams {
            z_spacing_mm: 0.0,
            ..Default::default()
        };
        let envelope = BlankEnvelope::default();

        let result = generate_volumetric_points(&depth, 10, 10, &params, &envelope);
        assert!(result.is_err());
    }

    #[test]
    fn estimate_gives_reasonable_count() {
        let params = VolumetricParams {
            step_x: 2,
            step_y: 2,
            z_spacing_mm: 1.0,
            ..Default::default()
        };
        let envelope = BlankEnvelope::default();

        let estimate = estimate_point_count(100, 100, &params, &envelope);
        assert!(estimate > 0);
        assert!(estimate < 10_000_000); // Sanity check
    }

    #[test]
    fn json_roundtrip_params() {
        let params = VolumetricParams {
            step_x: 2,
            step_y: 3,
            z_spacing_mm: 0.75,
            depth_min: 0.1,
            depth_max: 0.9,
            back_plane: 0.1,
        };
        let json = serde_json::to_string(&params).unwrap();
        let loaded: VolumetricParams = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.step_x, 2);
        assert_eq!(loaded.z_spacing_mm, 0.75);
    }
}
