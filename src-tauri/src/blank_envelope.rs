// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Crystal blank envelope and fit-to-blank scaling (ADR-011).
//!
//! Defines the physical 3D bounds of a crystal blank and provides functions
//! to scale and translate point clouds to fit within the blank with margins.

use serde::{Deserialize, Serialize};

/// Crystal blank envelope defining the 3D bounds for point cloud fitting (ADR-011).
///
/// The blank envelope represents the physical dimensions of the crystal blank
/// that will be laser engraved. Points are scaled and translated to fit within
/// these bounds while respecting the safety margin.
///
/// **Default:** 80×50×50 mm with 2mm margin (common crystal blank size).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlankEnvelope {
    /// Length of the blank in mm (X axis).
    pub length_mm: f32,
    /// Width of the blank in mm (Y axis).
    pub width_mm: f32,
    /// Height/depth of the blank in mm (Z axis).
    pub height_mm: f32,
    /// Safety margin from blank edges in mm (uniform on all sides).
    pub margin_mm: f32,
}

impl Default for BlankEnvelope {
    fn default() -> Self {
        Self {
            length_mm: 80.0,
            width_mm: 50.0,
            height_mm: 50.0,
            margin_mm: 2.0,
        }
    }
}

impl BlankEnvelope {
    /// Create a new blank envelope with specified dimensions.
    pub fn new(length_mm: f32, width_mm: f32, height_mm: f32, margin_mm: f32) -> Self {
        Self {
            length_mm,
            width_mm,
            height_mm,
            margin_mm,
        }
    }

    /// Create a cubic blank envelope.
    pub fn cube(size_mm: f32, margin_mm: f32) -> Self {
        Self::new(size_mm, size_mm, size_mm, margin_mm)
    }

    /// Interior length available for content (length minus 2× margin).
    pub fn interior_length(&self) -> f32 {
        (self.length_mm - 2.0 * self.margin_mm).max(0.0)
    }

    /// Interior width available for content (width minus 2× margin).
    pub fn interior_width(&self) -> f32 {
        (self.width_mm - 2.0 * self.margin_mm).max(0.0)
    }

    /// Interior height available for content (height minus 2× margin).
    pub fn interior_height(&self) -> f32 {
        (self.height_mm - 2.0 * self.margin_mm).max(0.0)
    }

    /// Validate that the envelope has positive interior dimensions.
    pub fn validate(&self) -> Result<(), String> {
        if self.length_mm <= 0.0 {
            return Err("Blank length must be positive".to_string());
        }
        if self.width_mm <= 0.0 {
            return Err("Blank width must be positive".to_string());
        }
        if self.height_mm <= 0.0 {
            return Err("Blank height must be positive".to_string());
        }
        if self.margin_mm < 0.0 {
            return Err("Margin cannot be negative".to_string());
        }
        if self.interior_length() <= 0.0 {
            return Err("Margin too large for blank length".to_string());
        }
        if self.interior_width() <= 0.0 {
            return Err("Margin too large for blank width".to_string());
        }
        if self.interior_height() <= 0.0 {
            return Err("Margin too large for blank height".to_string());
        }
        Ok(())
    }
}

/// Common blank presets for quick selection.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BlankPreset {
    /// 80×50×50 mm - common rectangular crystal
    Standard,
    /// 60×60×60 mm - cubic crystal
    Cube60,
    /// 100×60×60 mm - large rectangular crystal
    Large,
    /// 50×50×80 mm - tall rectangular crystal
    Tall,
}

impl BlankPreset {
    /// Get the blank envelope for this preset (with default 2mm margin).
    pub fn to_envelope(self) -> BlankEnvelope {
        match self {
            BlankPreset::Standard => BlankEnvelope::new(80.0, 50.0, 50.0, 2.0),
            BlankPreset::Cube60 => BlankEnvelope::cube(60.0, 2.0),
            BlankPreset::Large => BlankEnvelope::new(100.0, 60.0, 60.0, 2.0),
            BlankPreset::Tall => BlankEnvelope::new(50.0, 50.0, 80.0, 2.0),
        }
    }
}

/// Result of fit_to_blank operation containing scaling and translation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FitResult {
    /// Uniform scale factor applied to all axes.
    pub scale: f32,
    /// Translation to center the content in the blank (x, y, z).
    pub translation: [f32; 3],
    /// Number of points that were transformed.
    pub point_count: usize,
    /// Number of points that would be outside the envelope (should be 0 after fit).
    pub outliers: usize,
}

/// Compute the axis-aligned bounding box of a point cloud.
///
/// Returns (min, max) where each is [x, y, z].
/// Returns None if the point cloud is empty.
pub fn compute_bbox(points: &[[f32; 3]]) -> Option<([f32; 3], [f32; 3])> {
    if points.is_empty() {
        return None;
    }

    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for p in points {
        for i in 0..3 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }

    Some((min, max))
}

/// Scale and translate a point cloud to fit within the blank envelope.
///
/// This function:
/// 1. Computes the bounding box of the input points
/// 2. Calculates a uniform scale factor to fit within the interior bounds
/// 3. Translates points to center them within the blank
/// 4. Validates that all points are within bounds (returns outlier count)
///
/// # Arguments
/// * `points` - Mutable slice of 3D points to transform in-place
/// * `envelope` - The blank envelope defining target bounds
///
/// # Returns
/// * `Ok(FitResult)` - Transformation applied successfully
/// * `Err(String)` - Invalid envelope or empty point cloud
pub fn fit_to_blank(
    points: &mut [[f32; 3]],
    envelope: &BlankEnvelope,
) -> Result<FitResult, String> {
    // Validate envelope
    envelope.validate()?;

    if points.is_empty() {
        return Err("Point cloud is empty".to_string());
    }

    // Compute bounding box of content
    let (content_min, content_max) = compute_bbox(points).unwrap();

    let content_span = [
        content_max[0] - content_min[0],
        content_max[1] - content_min[1],
        content_max[2] - content_min[2],
    ];

    // Compute uniform scale factor
    let interior = [
        envelope.interior_length(),
        envelope.interior_width(),
        envelope.interior_height(),
    ];

    // Find minimum scale ratio across all axes (uniform scaling)
    let mut scale = f32::MAX;
    for i in 0..3 {
        if content_span[i] > 0.0 {
            scale = scale.min(interior[i] / content_span[i]);
        }
    }

    // If content has zero span in all dimensions (single point), use scale of 1
    if scale == f32::MAX {
        scale = 1.0;
    }

    // Compute translation to center content in blank interior
    // Center of blank interior
    let blank_center = [
        envelope.margin_mm + interior[0] / 2.0,
        envelope.margin_mm + interior[1] / 2.0,
        envelope.margin_mm + interior[2] / 2.0,
    ];

    // Center of scaled content
    let content_center = [
        (content_min[0] + content_max[0]) / 2.0 * scale,
        (content_min[1] + content_max[1]) / 2.0 * scale,
        (content_min[2] + content_max[2]) / 2.0 * scale,
    ];

    let translation = [
        blank_center[0] - content_center[0],
        blank_center[1] - content_center[1],
        blank_center[2] - content_center[2],
    ];

    // Apply transformation to all points
    for p in points.iter_mut() {
        p[0] = p[0] * scale + translation[0];
        p[1] = p[1] * scale + translation[1];
        p[2] = p[2] * scale + translation[2];
    }

    // Count outliers (points outside envelope bounds)
    let outliers = points
        .iter()
        .filter(|p| {
            p[0] < 0.0
                || p[0] > envelope.length_mm
                || p[1] < 0.0
                || p[1] > envelope.width_mm
                || p[2] < 0.0
                || p[2] > envelope.height_mm
        })
        .count();

    Ok(FitResult {
        scale,
        translation,
        point_count: points.len(),
        outliers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_envelope() {
        let e = BlankEnvelope::default();
        assert_eq!(e.length_mm, 80.0);
        assert_eq!(e.width_mm, 50.0);
        assert_eq!(e.height_mm, 50.0);
        assert_eq!(e.margin_mm, 2.0);
    }

    #[test]
    fn interior_dimensions() {
        let e = BlankEnvelope::new(80.0, 50.0, 50.0, 2.0);
        assert_eq!(e.interior_length(), 76.0);
        assert_eq!(e.interior_width(), 46.0);
        assert_eq!(e.interior_height(), 46.0);
    }

    #[test]
    fn validate_valid_envelope() {
        let e = BlankEnvelope::default();
        assert!(e.validate().is_ok());
    }

    #[test]
    fn validate_invalid_margin() {
        let e = BlankEnvelope::new(10.0, 10.0, 10.0, 6.0);
        assert!(e.validate().is_err());
    }

    #[test]
    fn preset_to_envelope() {
        let e = BlankPreset::Standard.to_envelope();
        assert_eq!(e.length_mm, 80.0);

        let cube = BlankPreset::Cube60.to_envelope();
        assert_eq!(cube.length_mm, 60.0);
        assert_eq!(cube.width_mm, 60.0);
        assert_eq!(cube.height_mm, 60.0);
    }

    #[test]
    fn compute_bbox_simple() {
        let points = vec![[0.0, 0.0, 0.0], [10.0, 5.0, 3.0], [5.0, 2.0, 1.0]];
        let (min, max) = compute_bbox(&points).unwrap();
        assert_eq!(min, [0.0, 0.0, 0.0]);
        assert_eq!(max, [10.0, 5.0, 3.0]);
    }

    #[test]
    fn compute_bbox_empty() {
        let points: Vec<[f32; 3]> = vec![];
        assert!(compute_bbox(&points).is_none());
    }

    #[test]
    fn fit_to_blank_simple() {
        // Points spanning 0..100 in X, 0..50 in Y, 0..25 in Z
        let mut points = vec![[0.0, 0.0, 0.0], [100.0, 50.0, 25.0], [50.0, 25.0, 12.5]];

        let envelope = BlankEnvelope::new(80.0, 50.0, 50.0, 2.0);
        // Interior: 76 x 46 x 46

        let result = fit_to_blank(&mut points, &envelope).unwrap();

        // Scale should be min(76/100, 46/50, 46/25) = min(0.76, 0.92, 1.84) = 0.76
        assert!((result.scale - 0.76).abs() < 0.01);
        assert_eq!(result.point_count, 3);
        assert_eq!(result.outliers, 0);

        // All points should be within envelope
        for p in &points {
            assert!(p[0] >= 0.0 && p[0] <= envelope.length_mm);
            assert!(p[1] >= 0.0 && p[1] <= envelope.width_mm);
            assert!(p[2] >= 0.0 && p[2] <= envelope.height_mm);
        }
    }

    #[test]
    fn fit_to_blank_centered() {
        // Small content that should be centered
        let mut points = vec![[0.0, 0.0, 0.0], [10.0, 10.0, 10.0]];

        let envelope = BlankEnvelope::new(100.0, 100.0, 100.0, 10.0);
        // Interior: 80 x 80 x 80

        let result = fit_to_blank(&mut points, &envelope).unwrap();

        // Content spans 10x10x10, interior is 80x80x80
        // Scale should be min(80/10, 80/10, 80/10) = 8.0
        assert!((result.scale - 8.0).abs() < 0.01);

        // After scaling, content spans 80x80x80
        // It should be centered in the interior (margin=10)
        // So content should span from 10 to 90 in each axis
        let (min, max) = compute_bbox(&points).unwrap();
        assert!((min[0] - 10.0).abs() < 0.01);
        assert!((max[0] - 90.0).abs() < 0.01);
    }

    #[test]
    fn fit_to_blank_empty_points() {
        let mut points: Vec<[f32; 3]> = vec![];
        let envelope = BlankEnvelope::default();
        assert!(fit_to_blank(&mut points, &envelope).is_err());
    }

    #[test]
    fn fit_to_blank_invalid_envelope() {
        let mut points = vec![[0.0, 0.0, 0.0], [10.0, 10.0, 10.0]];
        let envelope = BlankEnvelope::new(10.0, 10.0, 10.0, 6.0); // margin too large
        assert!(fit_to_blank(&mut points, &envelope).is_err());
    }

    #[test]
    fn json_roundtrip() {
        let envelope = BlankEnvelope::new(80.0, 50.0, 50.0, 2.5);
        let json = serde_json::to_string(&envelope).unwrap();
        let loaded: BlankEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.length_mm, 80.0);
        assert_eq!(loaded.margin_mm, 2.5);
    }
}
