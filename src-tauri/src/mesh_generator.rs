//! Mesh generation from depth map (BACK-501–506).
//!
//! Converts a row-major normalized depth map (0–1) into a point cloud with positions and normals
//! in millimeters. Design: RESEARCH/architecture.md § Mesh Generation (Sprint 1.6), ADR-006.
//!
//! **BACK-504 (Delaunay triangulation):** Deferred for MVP. Point cloud is sufficient for Three.js
//! preview (Sprint 1.7) and export path can use point cloud or add triangulation in a later sprint.

use anyhow::{ensure, Context};
use serde::{Deserialize, Serialize};

/// Parameters for mesh generation (ARCH-201, BACK-502, BACK-503).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshParams {
    /// Grid step in X (sample every step pixels). 1 = full resolution.
    pub step_x: u32,
    /// Grid step in Y. 1 = full resolution.
    pub step_y: u32,
    /// Depth range minimum in mm (e.g. 2 for laser engraving).
    pub depth_min_mm: f32,
    /// Depth range maximum in mm (e.g. 10).
    pub depth_max_mm: f32,
    /// Scale from pixel to mm: 1 pixel = pixel_to_mm mm (ARCH-202).
    pub pixel_to_mm: f32,
}

impl Default for MeshParams {
    fn default() -> Self {
        Self {
            step_x: 1,
            step_y: 1,
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            pixel_to_mm: 1.0,
        }
    }
}

/// Mesh output: positions and normals in mm; compatible with Three.js BufferGeometry and stl_io (ARCH-202, BACK-602).
///
/// **JSON serialization (Tauri IPC):** camelCase. Shape: `{ "positions": [[x,y,z], ...], "normals": [[x,y,z], ...] }`.
/// Each vertex i has `positions[i]` and `normals[i]`; lengths match. Units: mm for positions; normals unit length.
///
/// **Frontend (Three.js BufferGeometry):** Flatten for attributes: `new Float32Array(positions.flat())` for
/// `BufferAttribute(positionsFlat, 3)`; same for normals. Byte order in JSON is numeric; if binary transfer
/// is added (ADR-007), layout and byte order (e.g. little-endian) will be documented in RESEARCH/architecture.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshData {
    /// Vertex positions (x, y, z) in millimeters.
    pub positions: Vec<[f32; 3]>,
    /// Unit-length vertex normals (for shading / STL).
    pub normals: Vec<[f32; 3]>,
}

/// Maximum allowed width/height to avoid overflow and stay within memory budget (ARCH-204, BACK-506).
pub const MAX_DIMENSION: u32 = 8192;

/// Validates depth map input before processing (SEC-302).
/// - Dimensions > 0 and <= MAX_DIMENSION
/// - depth.len() == width * height (checked with overflow-safe multiply)
/// - Depth values expected in [0, 1] (not enforced here; mesh uses clamp)
pub fn validate_depth_input(width: u32, height: u32, depth: &[f32]) -> Result<(), anyhow::Error> {
    ensure!(width > 0, "width must be positive");
    ensure!(height > 0, "height must be positive");
    ensure!(
        width <= MAX_DIMENSION && height <= MAX_DIMENSION,
        "dimensions must be <= {}",
        MAX_DIMENSION
    );
    let expected_len = (width as usize)
        .checked_mul(height as usize)
        .context("depth dimensions overflow")?;
    ensure!(
        depth.len() == expected_len,
        "depth length {} does not match width×height {}",
        depth.len(),
        expected_len
    );
    Ok(())
}

/// Map normalized depth [0, 1] to Z in mm (BACK-503).
#[inline]
fn depth_to_z_mm(v: f32, min_mm: f32, max_mm: f32) -> f32 {
    let v = v.clamp(0.0, 1.0);
    min_mm + v * (max_mm - min_mm)
}

/// Sample depth at (row, col) with bounds check; returns clamped value.
#[inline]
fn sample_depth(depth: &[f32], width: usize, height: usize, row: usize, col: usize) -> f32 {
    if row >= height || col >= width {
        return 0.5;
    }
    let idx = row * width + col;
    depth.get(idx).copied().unwrap_or(0.5).clamp(0.0, 1.0)
}

/// Compute vertex normal from depth gradient (finite difference). Unit length (BACK-505).
fn normal_from_gradient(
    depth: &[f32],
    width: usize,
    height: usize,
    row: usize,
    col: usize,
    depth_range_mm: f32,
    pixel_to_mm: f32,
) -> [f32; 3] {
    let _d = sample_depth(depth, width, height, row, col);
    let (gx, gy) = if width >= 2 && height >= 2 {
        let col_prev = col.saturating_sub(1);
        let col_next = (col + 1).min(width - 1);
        let row_prev = row.saturating_sub(1);
        let row_next = (row + 1).min(height - 1);
        let dx = (sample_depth(depth, width, height, row, col_next)
            - sample_depth(depth, width, height, row, col_prev))
            / (if col_next > col_prev {
                (col_next - col_prev) as f32
            } else {
                1.0
            });
        let dy = (sample_depth(depth, width, height, row_next, col)
            - sample_depth(depth, width, height, row_prev, col))
            / (if row_next > row_prev {
                (row_next - row_prev) as f32
            } else {
                1.0
            });
        (dx, dy)
    } else {
        (0.0, 0.0)
    };
    // In mm space: slope in x = depth_range * gx / pixel_to_mm (depth change per pixel, then per mm).
    let nx = -depth_range_mm * gx / pixel_to_mm;
    let ny = -depth_range_mm * gy / pixel_to_mm;
    let nz = 1.0;
    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    if len > 1e-8 {
        [nx / len, ny / len, nz / len]
    } else {
        [0.0, 0.0, 1.0]
    }
}

/// Generate point cloud from depth map (BACK-501, BACK-502, BACK-503, BACK-505, BACK-506).
///
/// Single pass over grid samples; no mutation of `depth`. Output in millimeters.
/// Step size controls density: step_x=step_y=1 → full resolution; step=2 → 1/4 vertices.
pub fn depth_to_point_cloud(
    depth: &[f32],
    width: u32,
    height: u32,
    params: &MeshParams,
) -> Result<MeshData, anyhow::Error> {
    validate_depth_input(width, height, depth)?;

    let width_usize = width as usize;
    let height_usize = height as usize;
    let step_x = params.step_x.max(1);
    let step_y = params.step_y.max(1);

    // Vertex count with ceiling (ARCH-201).
    let step_x_usize = step_x as usize;
    let step_y_usize = step_y as usize;
    let num_cols = width_usize.div_ceil(step_x_usize);
    let num_rows = height_usize.div_ceil(step_y_usize);
    let vertex_count = num_rows
        .checked_mul(num_cols)
        .context("vertex count overflow")?;

    let depth_min = params.depth_min_mm;
    let depth_max = params.depth_max_mm;
    let depth_range = depth_max - depth_min;
    let pixel_to_mm = if params.pixel_to_mm > 0.0 {
        params.pixel_to_mm
    } else {
        1.0
    };

    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);

    for ri in 0..num_rows {
        let row = ri * step_y_usize;
        for ci in 0..num_cols {
            let col = ci * step_x_usize;
            let idx = row * width_usize + col;
            let d = depth.get(idx).copied().unwrap_or(0.5).clamp(0.0, 1.0);

            let x_mm = (col as f32) * pixel_to_mm;
            let y_mm = (row as f32) * pixel_to_mm;
            let z_mm = depth_to_z_mm(d, depth_min, depth_max);

            positions.push([x_mm, y_mm, z_mm]);
            let n = normal_from_gradient(
                depth,
                width_usize,
                height_usize,
                row,
                col,
                depth_range,
                pixel_to_mm,
            );
            normals.push(n);
        }
    }

    Ok(MeshData { positions, normals })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rejects_zero_width() {
        let err = validate_depth_input(0, 10, &vec![0.0; 0]).unwrap_err();
        assert!(err.to_string().contains("positive"));
    }

    #[test]
    fn validate_rejects_length_mismatch() {
        let err = validate_depth_input(3, 3, &vec![0.0; 8]).unwrap_err();
        assert!(err.to_string().contains("length") || err.to_string().contains("match"));
    }

    #[test]
    fn validate_rejects_dimensions_over_max() {
        let w = MAX_DIMENSION + 1;
        let h = 1u32;
        let depth: Vec<f32> = vec![0.0; (w as usize) * (h as usize)];
        let err = validate_depth_input(w, h, &depth).unwrap_err();
        assert!(err.to_string().contains("8192") || err.to_string().contains("dimensions"));
    }

    #[test]
    fn point_cloud_3x3_step1() {
        let depth: Vec<f32> = (0..9).map(|i| (i as f32) / 8.0).collect();
        let params = MeshParams {
            step_x: 1,
            step_y: 1,
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            pixel_to_mm: 1.0,
        };
        let mesh = depth_to_point_cloud(&depth, 3, 3, &params).unwrap();
        assert_eq!(mesh.positions.len(), 9);
        assert_eq!(mesh.normals.len(), 9);
        // (0,0): depth 0 -> z = 2
        assert!((mesh.positions[0][2] - 2.0).abs() < 1e-5);
        // (1,1): depth 4/8 = 0.5 -> z = 6
        assert!((mesh.positions[4][2] - 6.0).abs() < 1e-5);
        // (2,2): depth 1 -> z = 10
        assert!((mesh.positions[8][2] - 10.0).abs() < 1e-5);
    }

    #[test]
    fn point_cloud_step2_reduces_vertices() {
        let depth: Vec<f32> = (0..16).map(|_| 0.5).collect();
        let params = MeshParams {
            step_x: 2,
            step_y: 2,
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            pixel_to_mm: 1.0,
        };
        let mesh = depth_to_point_cloud(&depth, 4, 4, &params).unwrap();
        assert_eq!(mesh.positions.len(), 4);
        assert_eq!(mesh.normals.len(), 4);
    }

    #[test]
    fn z_range_respected() {
        let depth = vec![0.0, 0.5, 1.0];
        let params = MeshParams {
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            ..Default::default()
        };
        let mesh = depth_to_point_cloud(&depth, 3, 1, &params).unwrap();
        assert!((mesh.positions[0][2] - 2.0).abs() < 1e-5);
        assert!((mesh.positions[1][2] - 6.0).abs() < 1e-5);
        assert!((mesh.positions[2][2] - 10.0).abs() < 1e-5);
    }

    #[test]
    fn normals_unit_length() {
        let depth: Vec<f32> = (0..25).map(|i| (i as f32) / 24.0).collect();
        let mesh = depth_to_point_cloud(&depth, 5, 5, &MeshParams::default()).unwrap();
        for n in &mesh.normals {
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert!((len - 1.0).abs() < 1e-4, "normal not unit length: {:?}", n);
        }
    }

    #[test]
    fn single_pixel_depth_map() {
        let depth = vec![0.5];
        let mesh = depth_to_point_cloud(&depth, 1, 1, &MeshParams::default()).unwrap();
        assert_eq!(mesh.positions.len(), 1);
        assert_eq!(mesh.normals.len(), 1);
        assert!((mesh.positions[0][2] - 6.0).abs() < 1e-5);
    }

    // --- JR2-501: Unit tests for point cloud generation ---

    #[test]
    fn point_cloud_5x5_vertex_count_and_bounds() {
        let depth: Vec<f32> = (0..25).map(|i| (i as f32) / 24.0).collect();
        let params = MeshParams::default();
        let mesh = depth_to_point_cloud(&depth, 5, 5, &params).unwrap();
        assert_eq!(mesh.positions.len(), 25);
        assert_eq!(mesh.normals.len(), 25);

        let (min_x, max_x) = mesh
            .positions
            .iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), p| {
                (min.min(p[0]), max.max(p[0]))
            });
        let (min_y, max_y) = mesh
            .positions
            .iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), p| {
                (min.min(p[1]), max.max(p[1]))
            });
        let (min_z, max_z) = mesh
            .positions
            .iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), p| {
                (min.min(p[2]), max.max(p[2]))
            });

        assert!((min_x - 0.0).abs() < 1e-5);
        assert!((max_x - 4.0).abs() < 1e-5);
        assert!((min_y - 0.0).abs() < 1e-5);
        assert!((max_y - 4.0).abs() < 1e-5);
        assert!(
            min_z >= 2.0 - 1e-5 && max_z <= 10.0 + 1e-5,
            "Z in [2,10] mm"
        );
    }

    #[test]
    fn point_cloud_constant_depth_all_half() {
        let depth: Vec<f32> = vec![0.5; 100];
        let mesh = depth_to_point_cloud(&depth, 10, 10, &MeshParams::default()).unwrap();
        assert_eq!(mesh.positions.len(), 100);
        for p in &mesh.positions {
            assert!(
                (p[2] - 6.0).abs() < 1e-5,
                "constant depth 0.5 should map to z=6 mm"
            );
        }
    }

    #[test]
    fn point_cloud_gradient_left_zero_right_one() {
        let width = 10usize;
        let height = 5usize;
        let depth: Vec<f32> = (0..(width * height))
            .map(|i| (i % width) as f32 / (width - 1) as f32)
            .collect();
        let mesh =
            depth_to_point_cloud(&depth, width as u32, height as u32, &MeshParams::default())
                .unwrap();
        assert_eq!(mesh.positions.len(), width * height);
        let left_z = mesh.positions[0][2];
        let right_z = mesh.positions[width - 1][2];
        assert!((left_z - 2.0).abs() < 1e-5, "left (depth 0) -> z=2 mm");
        assert!((right_z - 10.0).abs() < 1e-5, "right (depth 1) -> z=10 mm");
    }

    // --- JR2-502: Edge cases — empty/single pixel/row/column; no panic, graceful error or valid output ---

    #[test]
    fn edge_empty_dimensions_rejected() {
        let err = validate_depth_input(0, 0, &[]).unwrap_err();
        assert!(err.to_string().contains("positive"));
        let err = validate_depth_input(0, 5, &vec![0.0; 0]).unwrap_err();
        assert!(err.to_string().contains("positive"));
        let err = validate_depth_input(5, 0, &vec![0.0; 0]).unwrap_err();
        assert!(err.to_string().contains("positive"));
    }

    #[test]
    fn edge_empty_slice_rejected() {
        let err = depth_to_point_cloud(&[], 3, 3, &MeshParams::default()).unwrap_err();
        assert!(err.to_string().contains("length") || err.to_string().contains("match"));
    }

    #[test]
    fn edge_single_row_produces_valid_mesh() {
        let depth: Vec<f32> = vec![0.0, 0.5, 1.0];
        let mesh = depth_to_point_cloud(&depth, 3, 1, &MeshParams::default()).unwrap();
        assert_eq!(mesh.positions.len(), 3);
        assert_eq!(mesh.normals.len(), 3);
        assert!((mesh.positions[0][2] - 2.0).abs() < 1e-5);
        assert!((mesh.positions[2][2] - 10.0).abs() < 1e-5);
    }

    #[test]
    fn edge_single_column_produces_valid_mesh() {
        let depth: Vec<f32> = vec![0.0, 0.5, 1.0];
        let mesh = depth_to_point_cloud(&depth, 1, 3, &MeshParams::default()).unwrap();
        assert_eq!(mesh.positions.len(), 3);
        assert_eq!(mesh.normals.len(), 3);
        assert!((mesh.positions[0][2] - 2.0).abs() < 1e-5);
        assert!((mesh.positions[2][2] - 10.0).abs() < 1e-5);
    }
}
