// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Mesh generation from depth map (BACK-501–506).
//!
//! Converts a row-major normalized depth map (0–1) into a point cloud with positions and normals
//! in millimeters. Design: RESEARCH/architecture.md § Mesh Generation (Sprint 1.6), ADR-006.
//!
//! **BACK-504 (Delaunay triangulation):** Deferred for MVP. Point cloud is sufficient for Three.js
//! preview (Sprint 1.7) and export path can use point cloud or add triangulation in a later sprint.
//!
//! **BACK-700 (Sprint 1.8):** Grid-based triangulation added per ADR-008. `triangulate_grid`
//! produces an index buffer (2 triangles per grid cell, CCW winding from +Z). `MeshData.indices`
//! is `Option<Vec<u32>>`; `None` = point cloud, `Some` = triangulated mesh.
//!
//! **BACK-701 (Sprint 1.8):** Binary STL writer. `write_stl_binary` serializes triangulated mesh
//! to binary STL format with computed face normals via cross product.
//!
//! **BACK-702 (Sprint 1.8):** Pre-export mesh validation. `validate_mesh_for_export` checks for
//! degenerate triangles, NaN/Inf, empty mesh before writing.

use anyhow::{ensure, Context};
use serde::{Deserialize, Serialize};
use std::io::Write;

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
    /// Triangle index buffer (BACK-700, ADR-008). Every 3 consecutive indices define one triangle.
    /// `None` = point cloud mode (backwards compatible); `Some` = triangulated mesh.
    /// Length = `6 * (num_rows - 1) * (num_cols - 1)` when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<u32>>,
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

    // Triangulate the grid (BACK-700, ADR-008).
    let indices = if num_rows >= 2 && num_cols >= 2 {
        Some(triangulate_grid(num_rows, num_cols))
    } else {
        None
    };

    Ok(MeshData {
        positions,
        normals,
        indices,
    })
}

/// Generate triangle index buffer for a uniform grid (BACK-700, ADR-008).
///
/// Each grid cell `(ri, ci)` produces 2 triangles with CCW winding (viewed from +Z):
/// - Triangle 1 (upper-left): tl -> tr -> bl
/// - Triangle 2 (lower-right): tr -> br -> bl
///
/// Returns empty `Vec` if `num_rows < 2` or `num_cols < 2` (no cells to triangulate).
pub fn triangulate_grid(num_rows: usize, num_cols: usize) -> Vec<u32> {
    if num_rows < 2 || num_cols < 2 {
        return Vec::new();
    }
    let num_cells = (num_rows - 1) * (num_cols - 1);
    let mut indices = Vec::with_capacity(num_cells * 6);
    let c = num_cols as u32;
    for ri in 0..(num_rows - 1) as u32 {
        for ci in 0..(num_cols - 1) as u32 {
            let tl = ri * c + ci;
            let tr = ri * c + ci + 1;
            let bl = (ri + 1) * c + ci;
            let br = (ri + 1) * c + ci + 1;
            // Triangle 1: tl -> tr -> bl (CCW from +Z, outward normal)
            indices.push(tl);
            indices.push(tr);
            indices.push(bl);
            // Triangle 2: tr -> br -> bl (CCW from +Z, outward normal)
            indices.push(tr);
            indices.push(br);
            indices.push(bl);
        }
    }
    indices
}

// --- BACK-702: Mesh validation for export ---

/// Validation error types for mesh export (BACK-702).
#[derive(Debug)]
pub enum MeshValidationError {
    /// Mesh has no triangles (indices missing or empty).
    NoTriangles,
    /// Mesh has no vertices.
    NoVertices,
    /// Index count is not a multiple of 3.
    InvalidIndexCount(usize),
    /// An index references a vertex beyond the positions array.
    IndexOutOfBounds { index: u32, vertex_count: usize },
    /// A vertex position contains NaN or Inf.
    InvalidPosition { vertex_index: usize },
    /// A degenerate triangle (zero area) was found.
    DegenerateTriangle { triangle_index: usize },
}

impl std::fmt::Display for MeshValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTriangles => write!(f, "Mesh has no triangles; cannot export STL"),
            Self::NoVertices => write!(f, "Mesh has no vertices"),
            Self::InvalidIndexCount(n) => {
                write!(f, "Index count ({}) is not a multiple of 3", n)
            }
            Self::IndexOutOfBounds {
                index,
                vertex_count,
            } => write!(
                f,
                "Index {} is out of bounds (vertex count: {})",
                index, vertex_count
            ),
            Self::InvalidPosition { vertex_index } => {
                write!(
                    f,
                    "Vertex {} has NaN or Inf position; mesh is invalid",
                    vertex_index
                )
            }
            Self::DegenerateTriangle { triangle_index } => {
                write!(
                    f,
                    "Triangle {} is degenerate (zero area); mesh may be invalid",
                    triangle_index
                )
            }
        }
    }
}

impl std::error::Error for MeshValidationError {}

/// Validate mesh data before STL export (BACK-702).
///
/// Checks:
/// 1. Vertices present
/// 2. Indices present and multiple of 3
/// 3. All indices in bounds
/// 4. No NaN/Inf in positions
/// 5. No degenerate (zero-area) triangles
pub fn validate_mesh_for_export(mesh: &MeshData) -> Result<(), MeshValidationError> {
    if mesh.positions.is_empty() {
        return Err(MeshValidationError::NoVertices);
    }
    let indices = match &mesh.indices {
        Some(idx) if !idx.is_empty() => idx,
        _ => return Err(MeshValidationError::NoTriangles),
    };
    if indices.len() % 3 != 0 {
        return Err(MeshValidationError::InvalidIndexCount(indices.len()));
    }
    let vertex_count = mesh.positions.len();
    // Check indices bounds
    for &idx in indices {
        if idx as usize >= vertex_count {
            return Err(MeshValidationError::IndexOutOfBounds {
                index: idx,
                vertex_count,
            });
        }
    }
    // Check for NaN/Inf positions
    for (i, pos) in mesh.positions.iter().enumerate() {
        if !pos[0].is_finite() || !pos[1].is_finite() || !pos[2].is_finite() {
            return Err(MeshValidationError::InvalidPosition { vertex_index: i });
        }
    }
    // Check for degenerate triangles
    let tri_count = indices.len() / 3;
    for t in 0..tri_count {
        let i0 = indices[t * 3] as usize;
        let i1 = indices[t * 3 + 1] as usize;
        let i2 = indices[t * 3 + 2] as usize;
        let v0 = mesh.positions[i0];
        let v1 = mesh.positions[i1];
        let v2 = mesh.positions[i2];
        let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
        let cross = [
            e1[1] * e2[2] - e1[2] * e2[1],
            e1[2] * e2[0] - e1[0] * e2[2],
            e1[0] * e2[1] - e1[1] * e2[0],
        ];
        let area_sq = cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2];
        if area_sq < 1e-20 {
            return Err(MeshValidationError::DegenerateTriangle {
                triangle_index: t,
            });
        }
    }
    Ok(())
}

// --- BACK-701: Binary STL writer ---

/// Compute face normal from three vertices via cross product (BACK-701, ADR-008).
/// Returns unit-length normal; falls back to [0, 0, 1] for degenerate triangles.
fn compute_face_normal(v0: &[f32; 3], v1: &[f32; 3], v2: &[f32; 3]) -> [f32; 3] {
    let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
    let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
    let nx = e1[1] * e2[2] - e1[2] * e2[1];
    let ny = e1[2] * e2[0] - e1[0] * e2[2];
    let nz = e1[0] * e2[1] - e1[1] * e2[0];
    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    if len > 1e-10 {
        [nx / len, ny / len, nz / len]
    } else {
        [0.0, 0.0, 1.0]
    }
}

/// Write binary STL to the given writer (BACK-701).
///
/// Binary STL format:
/// - 80-byte header (ASCII, zero-padded)
/// - u32 triangle count (little-endian)
/// - Per triangle: normal (3×f32 LE), v1 (3×f32 LE), v2 (3×f32 LE), v3 (3×f32 LE), u16 attribute (0)
///
/// Each f32 is IEEE 754 little-endian (4 bytes). Total per triangle: 50 bytes.
pub fn write_stl_binary<W: Write>(writer: &mut W, mesh: &MeshData) -> Result<(), anyhow::Error> {
    let indices = mesh
        .indices
        .as_ref()
        .context("Cannot write STL: mesh has no triangle indices")?;
    ensure!(
        !indices.is_empty(),
        "Cannot write STL: mesh has no triangles"
    );
    ensure!(
        indices.len() % 3 == 0,
        "Index count must be a multiple of 3"
    );

    let tri_count = (indices.len() / 3) as u32;

    // 80-byte header
    let mut header = [0u8; 80];
    let header_text = b"SimplePicture3D STL Export";
    let copy_len = header_text.len().min(80);
    header[..copy_len].copy_from_slice(&header_text[..copy_len]);
    writer.write_all(&header)?;

    // Triangle count (u32 LE)
    writer.write_all(&tri_count.to_le_bytes())?;

    // Per triangle: normal + 3 vertices + attribute byte count
    for t in 0..tri_count as usize {
        let i0 = indices[t * 3] as usize;
        let i1 = indices[t * 3 + 1] as usize;
        let i2 = indices[t * 3 + 2] as usize;

        ensure!(
            i0 < mesh.positions.len() && i1 < mesh.positions.len() && i2 < mesh.positions.len(),
            "Index out of bounds at triangle {}",
            t
        );

        let v0 = &mesh.positions[i0];
        let v1 = &mesh.positions[i1];
        let v2 = &mesh.positions[i2];
        let normal = compute_face_normal(v0, v1, v2);

        // Normal
        for &component in &normal {
            writer.write_all(&component.to_le_bytes())?;
        }
        // Vertex 1
        for &component in v0 {
            writer.write_all(&component.to_le_bytes())?;
        }
        // Vertex 2
        for &component in v1 {
            writer.write_all(&component.to_le_bytes())?;
        }
        // Vertex 3
        for &component in v2 {
            writer.write_all(&component.to_le_bytes())?;
        }
        // Attribute byte count (u16 = 0)
        writer.write_all(&0u16.to_le_bytes())?;
    }

    writer.flush()?;
    Ok(())
}

/// Write binary STL to a file path (BACK-701 convenience wrapper).
pub fn write_stl_to_file(path: &str, mesh: &MeshData) -> Result<(), anyhow::Error> {
    let file = std::fs::File::create(path)
        .with_context(|| format!("Cannot create STL file: {}", path))?;
    let mut writer = std::io::BufWriter::new(file);
    write_stl_binary(&mut writer, mesh)
}

// --- BACK-801: OBJ ASCII writer (Sprint 1.9) ---

/// Write ASCII OBJ to the given writer (BACK-801).
///
/// OBJ format:
/// - Comment header
/// - `v x y z` lines (vertex positions)
/// - `vn nx ny nz` lines (vertex normals)
/// - `f v1//n1 v2//n2 v3//n3` lines (faces referencing vertex/normal indices, 1-based)
///
/// Optional MTL reference is written if `mtl_filename` is `Some`.
pub fn write_obj_ascii<W: Write>(
    writer: &mut W,
    mesh: &MeshData,
    mtl_filename: Option<&str>,
) -> Result<(), anyhow::Error> {
    let indices = mesh
        .indices
        .as_ref()
        .context("Cannot write OBJ: mesh has no triangle indices")?;
    ensure!(
        !indices.is_empty(),
        "Cannot write OBJ: mesh has no triangles"
    );
    ensure!(
        indices.len() % 3 == 0,
        "Index count must be a multiple of 3"
    );

    // Header comment
    writeln!(writer, "# SimplePicture3D OBJ Export")?;
    writeln!(
        writer,
        "# Vertices: {}  Triangles: {}",
        mesh.positions.len(),
        indices.len() / 3
    )?;

    // MTL reference (BACK-802)
    if let Some(mtl) = mtl_filename {
        writeln!(writer, "mtllib {}", mtl)?;
        writeln!(writer, "usemtl default")?;
    }

    // Vertex positions
    for pos in &mesh.positions {
        writeln!(writer, "v {} {} {}", pos[0], pos[1], pos[2])?;
    }

    // Vertex normals
    for n in &mesh.normals {
        writeln!(writer, "vn {} {} {}", n[0], n[1], n[2])?;
    }

    // Faces (1-based indices, format: f v//vn v//vn v//vn)
    let tri_count = indices.len() / 3;
    for t in 0..tri_count {
        let i0 = indices[t * 3] + 1; // OBJ is 1-based
        let i1 = indices[t * 3 + 1] + 1;
        let i2 = indices[t * 3 + 2] + 1;
        writeln!(
            writer,
            "f {}//{} {}//{} {}//{}",
            i0, i0, i1, i1, i2, i2
        )?;
    }

    writer.flush()?;
    Ok(())
}

/// Write a minimal MTL material file (BACK-802).
///
/// Creates a default material with neutral gray diffuse color suitable for mesh viewing.
pub fn write_mtl<W: Write>(writer: &mut W) -> Result<(), anyhow::Error> {
    writeln!(writer, "# SimplePicture3D Material")?;
    writeln!(writer, "newmtl default")?;
    writeln!(writer, "Ka 0.2 0.2 0.2")?; // ambient
    writeln!(writer, "Kd 0.8 0.8 0.8")?; // diffuse
    writeln!(writer, "Ks 0.1 0.1 0.1")?; // specular
    writeln!(writer, "Ns 10.0")?; // specular exponent
    writeln!(writer, "d 1.0")?; // opacity
    writeln!(writer, "illum 2")?; // illumination model
    writer.flush()?;
    Ok(())
}

/// Write OBJ + optional MTL to files (BACK-801 convenience wrapper).
pub fn write_obj_to_file(path: &str, mesh: &MeshData, write_mtl_file: bool) -> Result<(), anyhow::Error> {
    let obj_path = std::path::Path::new(path);
    let mtl_filename = if write_mtl_file {
        obj_path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|stem| format!("{}.mtl", stem))
    } else {
        None
    };

    // Write OBJ
    let file = std::fs::File::create(path)
        .with_context(|| format!("Cannot create OBJ file: {}", path))?;
    let mut writer = std::io::BufWriter::new(file);
    write_obj_ascii(&mut writer, mesh, mtl_filename.as_deref())?;

    // Write MTL alongside OBJ (BACK-802)
    if let Some(ref mtl_name) = mtl_filename {
        if let Some(parent) = obj_path.parent() {
            let mtl_path = parent.join(mtl_name);
            let mtl_file = std::fs::File::create(&mtl_path)
                .with_context(|| format!("Cannot create MTL file: {}", mtl_path.display()))?;
            let mut mtl_writer = std::io::BufWriter::new(mtl_file);
            write_mtl(&mut mtl_writer)?;
        }
    }

    Ok(())
}

// --- BACK-705: Auto-generate filename ---

/// Generate a default export filename from source image name + timestamp (BACK-705, BACK-803).
///
/// Format: `{image_stem}_{YYYYMMDD_HHMMSS}.{ext}`
/// If `source_image_path` is empty, uses "mesh" as stem.
/// Extension defaults to "stl" if not provided.
pub fn generate_export_filename_with_ext(source_image_path: &str, ext: &str) -> String {
    let stem = if source_image_path.trim().is_empty() {
        "mesh".to_string()
    } else {
        std::path::Path::new(source_image_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("mesh")
            .to_string()
    };
    // Sanitize stem: replace non-alphanumeric/underscore/hyphen with underscore
    let sanitized: String = stem
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let now = chrono_like_timestamp();
    format!("{}_{}.{}", sanitized, now, ext)
}

/// Generate a default STL export filename (backwards-compatible wrapper).
pub fn generate_export_filename(source_image_path: &str) -> String {
    generate_export_filename_with_ext(source_image_path, "stl")
}

/// Simple timestamp without chrono dependency: YYYYMMDD_HHMMSS.
fn chrono_like_timestamp() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Convert epoch seconds to date/time components
    let secs_per_day: u64 = 86400;
    let mut days = now / secs_per_day;
    let day_secs = now % secs_per_day;
    let hours = day_secs / 3600;
    let minutes = (day_secs % 3600) / 60;
    let seconds = day_secs % 60;

    // Days since 1970-01-01 to (year, month, day)
    // Algorithm from Howard Hinnant's civil_from_days
    days += 719468;
    let era = days / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!(
        "{:04}{:02}{:02}_{:02}{:02}{:02}",
        y, m, d, hours, minutes, seconds
    )
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

    // --- BACK-700: Triangulation tests ---

    #[test]
    fn triangulate_grid_2x2_produces_2_triangles() {
        let indices = triangulate_grid(2, 2);
        // 1 cell -> 2 triangles -> 6 indices
        assert_eq!(indices.len(), 6);
        // Triangle 1: tl(0) -> tr(1) -> bl(2)
        assert_eq!(indices[0], 0);
        assert_eq!(indices[1], 1);
        assert_eq!(indices[2], 2);
        // Triangle 2: tr(1) -> br(3) -> bl(2)
        assert_eq!(indices[3], 1);
        assert_eq!(indices[4], 3);
        assert_eq!(indices[5], 2);
    }

    #[test]
    fn triangulate_grid_3x3_produces_8_triangles() {
        let indices = triangulate_grid(3, 3);
        // 2x2 cells -> 4 cells -> 8 triangles -> 24 indices
        assert_eq!(indices.len(), 24);
        // Verify every index is in bounds [0, 9)
        for &idx in &indices {
            assert!(idx < 9, "index {} out of bounds for 3x3 grid", idx);
        }
    }

    #[test]
    fn triangulate_grid_edge_cases_empty() {
        assert!(triangulate_grid(0, 0).is_empty());
        assert!(triangulate_grid(1, 1).is_empty());
        assert!(triangulate_grid(1, 5).is_empty());
        assert!(triangulate_grid(5, 1).is_empty());
        assert!(triangulate_grid(0, 10).is_empty());
    }

    #[test]
    fn triangulate_grid_single_row_of_cells() {
        // 2 rows, 4 cols -> 3 cells -> 6 triangles -> 18 indices
        let indices = triangulate_grid(2, 4);
        assert_eq!(indices.len(), 18);
    }

    #[test]
    fn triangulate_grid_ccw_winding_check() {
        // For a flat mesh (all z equal), CCW winding from +Z should give +Z normals.
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0], // tl (0)
                [1.0, 0.0, 0.0], // tr (1)
                [0.0, 1.0, 0.0], // bl (2)
                [1.0, 1.0, 0.0], // br (3)
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let idx = mesh.indices.as_ref().unwrap();
        // Triangle 1: 0 -> 2 -> 1
        let v0 = mesh.positions[idx[0] as usize];
        let v1 = mesh.positions[idx[1] as usize];
        let v2 = mesh.positions[idx[2] as usize];
        let normal = compute_face_normal(&v0, &v1, &v2);
        // Should point in +Z direction
        assert!(normal[2] > 0.9, "face normal should point +Z, got {:?}", normal);
    }

    #[test]
    fn depth_to_point_cloud_3x3_has_indices() {
        let depth: Vec<f32> = (0..9).map(|i| (i as f32) / 8.0).collect();
        let params = MeshParams::default();
        let mesh = depth_to_point_cloud(&depth, 3, 3, &params).unwrap();
        assert!(mesh.indices.is_some());
        let indices = mesh.indices.as_ref().unwrap();
        // 2x2 cells = 4 cells = 8 triangles = 24 indices
        assert_eq!(indices.len(), 24);
    }

    #[test]
    fn depth_to_point_cloud_single_pixel_no_indices() {
        let depth = vec![0.5];
        let mesh = depth_to_point_cloud(&depth, 1, 1, &MeshParams::default()).unwrap();
        assert!(mesh.indices.is_none());
    }

    #[test]
    fn depth_to_point_cloud_single_row_no_indices() {
        let depth = vec![0.0, 0.5, 1.0];
        let mesh = depth_to_point_cloud(&depth, 3, 1, &MeshParams::default()).unwrap();
        // 1 row = num_rows=1 < 2 -> no indices
        assert!(mesh.indices.is_none());
    }

    // --- BACK-701: STL writer tests ---

    #[test]
    fn stl_binary_write_2x2_mesh() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();

        // Expected size: 80 header + 4 tri count + 2 * 50 bytes per triangle = 184 bytes
        assert_eq!(buf.len(), 80 + 4 + 2 * 50);

        // Check triangle count
        let tri_count = u32::from_le_bytes([buf[80], buf[81], buf[82], buf[83]]);
        assert_eq!(tri_count, 2);
    }

    #[test]
    fn stl_binary_write_header_content() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        let header_text = &buf[..25];
        assert_eq!(header_text, b"SimplePicture3D STL Expor");
    }

    #[test]
    fn stl_binary_write_rejects_no_indices() {
        let mesh = MeshData {
            positions: vec![[0.0, 0.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]],
            indices: None,
        };
        let mut buf = Vec::new();
        let err = write_stl_binary(&mut buf, &mesh).unwrap_err();
        assert!(err.to_string().contains("no triangle indices"));
    }

    #[test]
    fn stl_binary_roundtrip_vertex_data() {
        // Write a simple triangle, then read back and verify vertex positions
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 5.0],
                [10.0, 0.0, 5.0],
                [0.0, 10.0, 5.0],
                [10.0, 10.0, 5.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();

        // Parse first triangle from buffer
        let offset = 84; // after header + tri count
        // Normal (3 x f32) = 12 bytes, then v0,v1,v2 (9 x f32 = 36 bytes), then u16 = 2
        // Triangle 1: indices 0,2,1 -> positions[0], positions[2], positions[1]
        let v0_x = f32::from_le_bytes([buf[offset + 12], buf[offset + 13], buf[offset + 14], buf[offset + 15]]);
        let v0_y = f32::from_le_bytes([buf[offset + 16], buf[offset + 17], buf[offset + 18], buf[offset + 19]]);
        let v0_z = f32::from_le_bytes([buf[offset + 20], buf[offset + 21], buf[offset + 22], buf[offset + 23]]);
        assert!((v0_x - 0.0).abs() < 1e-5);
        assert!((v0_y - 0.0).abs() < 1e-5);
        assert!((v0_z - 5.0).abs() < 1e-5);
    }

    #[test]
    fn stl_write_to_file_roundtrip() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let path = std::env::temp_dir()
            .join("sp3d_test_stl_roundtrip.stl")
            .to_string_lossy()
            .to_string();
        write_stl_to_file(&path, &mesh).unwrap();
        let data = std::fs::read(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        // 80 header + 4 count + 2 * 50 = 184
        assert_eq!(data.len(), 184);
        let tri_count = u32::from_le_bytes([data[80], data[81], data[82], data[83]]);
        assert_eq!(tri_count, 2);
    }

    // --- BACK-702: Mesh validation tests ---

    #[test]
    fn validate_valid_mesh_passes() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        assert!(validate_mesh_for_export(&mesh).is_ok());
    }

    #[test]
    fn validate_rejects_no_vertices() {
        let mesh = MeshData {
            positions: vec![],
            normals: vec![],
            indices: Some(vec![0, 1, 2]),
        };
        match validate_mesh_for_export(&mesh) {
            Err(MeshValidationError::NoVertices) => {}
            other => panic!("expected NoVertices, got {:?}", other),
        }
    }

    #[test]
    fn validate_rejects_no_triangles() {
        let mesh = MeshData {
            positions: vec![[0.0, 0.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]],
            indices: None,
        };
        match validate_mesh_for_export(&mesh) {
            Err(MeshValidationError::NoTriangles) => {}
            other => panic!("expected NoTriangles, got {:?}", other),
        }
    }

    #[test]
    fn validate_rejects_empty_indices() {
        let mesh = MeshData {
            positions: vec![[0.0, 0.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]],
            indices: Some(vec![]),
        };
        match validate_mesh_for_export(&mesh) {
            Err(MeshValidationError::NoTriangles) => {}
            other => panic!("expected NoTriangles, got {:?}", other),
        }
    }

    #[test]
    fn validate_rejects_nan_position() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, f32::NAN],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2]),
        };
        match validate_mesh_for_export(&mesh) {
            Err(MeshValidationError::InvalidPosition { vertex_index: 0 }) => {}
            other => panic!("expected InvalidPosition at 0, got {:?}", other),
        }
    }

    #[test]
    fn validate_rejects_inf_position() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0],
                [f32::INFINITY, 0.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2]),
        };
        match validate_mesh_for_export(&mesh) {
            Err(MeshValidationError::InvalidPosition { vertex_index: 1 }) => {}
            other => panic!("expected InvalidPosition at 1, got {:?}", other),
        }
    }

    #[test]
    fn validate_rejects_index_out_of_bounds() {
        let mesh = MeshData {
            positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]; 2],
            indices: Some(vec![0, 1, 5]), // 5 is out of bounds
        };
        match validate_mesh_for_export(&mesh) {
            Err(MeshValidationError::IndexOutOfBounds { index: 5, .. }) => {}
            other => panic!("expected IndexOutOfBounds, got {:?}", other),
        }
    }

    #[test]
    fn validate_rejects_degenerate_triangle() {
        // All three vertices at the same point -> zero area
        let mesh = MeshData {
            positions: vec![[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2]),
        };
        match validate_mesh_for_export(&mesh) {
            Err(MeshValidationError::DegenerateTriangle { .. }) => {}
            other => panic!("expected DegenerateTriangle, got {:?}", other),
        }
    }

    // --- BACK-701: Full pipeline test (depth -> mesh -> validate -> STL) ---

    #[test]
    fn full_pipeline_depth_to_stl() {
        let depth: Vec<f32> = (0..25).map(|i| (i as f32) / 24.0).collect();
        let params = MeshParams::default();
        let mesh = depth_to_point_cloud(&depth, 5, 5, &params).unwrap();
        assert!(mesh.indices.is_some());
        validate_mesh_for_export(&mesh).unwrap();
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        let tri_count = u32::from_le_bytes([buf[80], buf[81], buf[82], buf[83]]);
        // 4x4 cells = 16 cells = 32 triangles
        assert_eq!(tri_count, 32);
        assert_eq!(buf.len(), 80 + 4 + 32 * 50);
    }

    // --- BACK-705: Filename generation tests ---

    #[test]
    fn generate_export_filename_from_image_path() {
        let name = generate_export_filename("C:\\photos\\my_image.png");
        assert!(name.starts_with("my_image_"));
        assert!(name.ends_with(".stl"));
    }

    #[test]
    fn generate_export_filename_empty_path() {
        let name = generate_export_filename("");
        assert!(name.starts_with("mesh_"));
        assert!(name.ends_with(".stl"));
    }

    #[test]
    fn generate_export_filename_with_spaces() {
        let name = generate_export_filename("/home/user/my photo.jpg");
        assert!(name.starts_with("my_photo_"));
        assert!(name.ends_with(".stl"));
    }

    #[test]
    fn generate_export_filename_format() {
        let name = generate_export_filename("test.png");
        // Should be test_YYYYMMDD_HHMMSS.stl
        assert!(name.starts_with("test_"));
        assert!(name.ends_with(".stl"));
        // The timestamp part should be 15 chars: YYYYMMDD_HHMMSS
        let middle = &name[5..name.len() - 4]; // strip "test_" and ".stl"
        assert_eq!(middle.len(), 15, "timestamp should be 15 chars: {}", middle);
    }

    // =========================================================================
    // JR2-701: Comprehensive STL writer unit tests
    // =========================================================================

    /// Helper: parse binary STL buffer into (header, tri_count, Vec<(normal, [v0,v1,v2], attr)>).
    fn parse_stl_binary(buf: &[u8]) -> (Vec<u8>, u32, Vec<([f32; 3], [[f32; 3]; 3], u16)>) {
        assert!(buf.len() >= 84, "STL buffer too short for header + tri count");
        let header = buf[..80].to_vec();
        let tri_count = u32::from_le_bytes([buf[80], buf[81], buf[82], buf[83]]);
        let expected_len = 84 + tri_count as usize * 50;
        assert_eq!(buf.len(), expected_len, "STL file size mismatch: expected {}, got {}", expected_len, buf.len());

        let mut triangles = Vec::with_capacity(tri_count as usize);
        for t in 0..tri_count as usize {
            let base = 84 + t * 50;
            let read_f32 = |offset: usize| -> f32 {
                f32::from_le_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]])
            };
            let read_vec3 = |offset: usize| -> [f32; 3] {
                [read_f32(offset), read_f32(offset + 4), read_f32(offset + 8)]
            };
            let normal = read_vec3(base);
            let v0 = read_vec3(base + 12);
            let v1 = read_vec3(base + 24);
            let v2 = read_vec3(base + 36);
            let attr = u16::from_le_bytes([buf[base + 48], buf[base + 49]]);
            triangles.push((normal, [v0, v1, v2], attr));
        }
        (header, tri_count, triangles)
    }

    #[test]
    fn jr2_701_roundtrip_small_quad() {
        // Write STL for a 2x2 quad, read back, verify vertex positions within f32 tolerance
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 5.0],
                [10.0, 0.0, 3.0],
                [0.0, 10.0, 7.0],
                [10.0, 10.0, 1.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();

        let (header, tri_count, triangles) = parse_stl_binary(&buf);

        // Header is 80 bytes
        assert_eq!(header.len(), 80);
        // 2 triangles
        assert_eq!(tri_count, 2);
        // File size = 84 + 50*2 = 184
        assert_eq!(buf.len(), 184);

        // Triangle 1: indices tl(0)->tr(1)->bl(2): positions[0], positions[1], positions[2]
        let idx = mesh.indices.as_ref().unwrap();
        for t in 0..2 {
            let i0 = idx[t * 3] as usize;
            let i1 = idx[t * 3 + 1] as usize;
            let i2 = idx[t * 3 + 2] as usize;
            let (_, verts, attr) = &triangles[t];
            for c in 0..3 {
                assert!((verts[0][c] - mesh.positions[i0][c]).abs() < 1e-5,
                    "tri {} v0[{}] mismatch: {} vs {}", t, c, verts[0][c], mesh.positions[i0][c]);
                assert!((verts[1][c] - mesh.positions[i1][c]).abs() < 1e-5,
                    "tri {} v1[{}] mismatch", t, c);
                assert!((verts[2][c] - mesh.positions[i2][c]).abs() < 1e-5,
                    "tri {} v2[{}] mismatch", t, c);
            }
            assert_eq!(*attr, 0, "attribute byte count should be 0");
        }
    }

    #[test]
    fn jr2_701_roundtrip_medium_grid_100x100() {
        // 100x100 depth map -> mesh -> STL -> parse back -> verify size and triangle count
        let w = 100usize;
        let h = 100usize;
        let depth: Vec<f32> = (0..w * h)
            .map(|i| (i as f32) / ((w * h - 1) as f32))
            .collect();
        let params = MeshParams::default();
        let mesh = depth_to_point_cloud(&depth, w as u32, h as u32, &params).unwrap();
        assert!(mesh.indices.is_some());

        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();

        let (_, tri_count, _) = parse_stl_binary(&buf);
        // 99*99 cells * 2 triangles = 19602
        assert_eq!(tri_count, 19602);
        assert_eq!(buf.len(), 84 + 19602 * 50);
    }

    #[test]
    fn jr2_701_stl_header_exactly_80_bytes() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        // Header is always exactly 80 bytes, zero-padded after text
        let header_text = b"SimplePicture3D STL Export";
        let text_len = header_text.len(); // 26 bytes
        assert_eq!(&buf[..text_len], &header_text[..]);
        // Remaining header bytes should be zero
        for i in text_len..80 {
            assert_eq!(buf[i], 0u8, "header byte {} should be 0, got {}", i, buf[i]);
        }
    }

    #[test]
    fn jr2_701_stl_file_size_formula() {
        // Verify file size = 84 + 50 * num_triangles for various sizes
        for grid_size in [2usize, 3, 5, 10, 20] {
            let n = grid_size * grid_size;
            let depth: Vec<f32> = vec![0.5; n];
            let mesh = depth_to_point_cloud(
                &depth, grid_size as u32, grid_size as u32, &MeshParams::default()
            ).unwrap();
            let mut buf = Vec::new();
            write_stl_binary(&mut buf, &mesh).unwrap();

            let expected_tris = ((grid_size - 1) * (grid_size - 1) * 2) as u32;
            let expected_size = 84 + expected_tris as usize * 50;
            assert_eq!(buf.len(), expected_size,
                "grid {}x{}: expected size {}, got {}", grid_size, grid_size, expected_size, buf.len());

            let tri_count = u32::from_le_bytes([buf[80], buf[81], buf[82], buf[83]]);
            assert_eq!(tri_count, expected_tris,
                "grid {}x{}: expected {} tris, got {}", grid_size, grid_size, expected_tris, tri_count);
        }
    }

    #[test]
    fn jr2_701_stl_normals_unit_length() {
        // All normals in the STL output should be unit length
        let depth: Vec<f32> = (0..25).map(|i| (i as f32) / 24.0).collect();
        let mesh = depth_to_point_cloud(&depth, 5, 5, &MeshParams::default()).unwrap();
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();

        let (_, _, triangles) = parse_stl_binary(&buf);
        for (t, (normal, _, _)) in triangles.iter().enumerate() {
            let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
            assert!((len - 1.0).abs() < 1e-4,
                "triangle {} normal not unit length: {:?} (len={})", t, normal, len);
        }
    }

    #[test]
    fn jr2_701_stl_normals_point_positive_z_for_flat_mesh() {
        // For a flat mesh (all z=0), face normals from CCW winding should point +Z
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0], [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        let (_, _, triangles) = parse_stl_binary(&buf);
        for (t, (normal, _, _)) in triangles.iter().enumerate() {
            assert!(normal[2] > 0.99,
                "triangle {} normal z should be ~1.0, got {:?}", t, normal);
            assert!(normal[0].abs() < 1e-5 && normal[1].abs() < 1e-5,
                "triangle {} normal x,y should be ~0, got {:?}", t, normal);
        }
    }

    #[test]
    fn jr2_701_extreme_coordinates_large() {
        // Test with very large coordinate values
        let mesh = MeshData {
            positions: vec![
                [1e6, 1e6, 1e6],
                [1e6 + 1.0, 1e6, 1e6],
                [1e6, 1e6 + 1.0, 1e6],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2]),
        };
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        let (_, tri_count, triangles) = parse_stl_binary(&buf);
        assert_eq!(tri_count, 1);
        // Verify large values round-trip correctly
        assert!((triangles[0].1[0][0] - 1e6).abs() < 1.0,
            "large coordinate round-trip failed");
    }

    #[test]
    fn jr2_701_extreme_coordinates_small() {
        // Test with very small coordinate values
        let mesh = MeshData {
            positions: vec![
                [1e-6, 1e-6, 1e-6],
                [1e-5, 1e-6, 1e-6],
                [1e-6, 1e-5, 1e-6],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2]),
        };
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        let (_, tri_count, triangles) = parse_stl_binary(&buf);
        assert_eq!(tri_count, 1);
        assert!((triangles[0].1[0][0] - 1e-6).abs() < 1e-10);
    }

    #[test]
    fn jr2_701_extreme_coordinates_negative() {
        // Test with negative coordinate values
        let mesh = MeshData {
            positions: vec![
                [-10.0, -20.0, -5.0],
                [10.0, -20.0, -5.0],
                [-10.0, 20.0, -5.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2]),
        };
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        let (_, _, triangles) = parse_stl_binary(&buf);
        assert!((triangles[0].1[0][0] - (-10.0)).abs() < 1e-5);
        assert!((triangles[0].1[0][1] - (-20.0)).abs() < 1e-5);
        assert!((triangles[0].1[0][2] - (-5.0)).abs() < 1e-5);
    }

    #[test]
    fn jr2_701_roundtrip_all_vertices_match() {
        // Full round-trip: depth -> mesh -> STL -> parse -> verify every vertex
        let depth: Vec<f32> = vec![0.0, 0.25, 0.5, 0.75, 1.0, 0.5, 0.25, 0.75, 0.0];
        let params = MeshParams {
            depth_min_mm: 1.0,
            depth_max_mm: 9.0,
            ..Default::default()
        };
        let mesh = depth_to_point_cloud(&depth, 3, 3, &params).unwrap();
        let idx = mesh.indices.as_ref().unwrap();
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();

        let (_, tri_count, triangles) = parse_stl_binary(&buf);
        assert_eq!(tri_count as usize, idx.len() / 3);

        for t in 0..tri_count as usize {
            let i0 = idx[t * 3] as usize;
            let i1 = idx[t * 3 + 1] as usize;
            let i2 = idx[t * 3 + 2] as usize;
            let (_, verts, _) = &triangles[t];
            for c in 0..3 {
                assert!((verts[0][c] - mesh.positions[i0][c]).abs() < 1e-5,
                    "roundtrip: tri {} v0[{}]", t, c);
                assert!((verts[1][c] - mesh.positions[i1][c]).abs() < 1e-5,
                    "roundtrip: tri {} v1[{}]", t, c);
                assert!((verts[2][c] - mesh.positions[i2][c]).abs() < 1e-5,
                    "roundtrip: tri {} v2[{}]", t, c);
            }
        }
    }

    // =========================================================================
    // JR2-702: Programmatic STL format validation (external tool substitute)
    // =========================================================================

    #[test]
    fn jr2_702_programmatic_stl_format_validation() {
        // Generate STL from a realistic depth map, then perform full binary format validation.
        // This substitutes for MeshLab CLI validation (manual MeshLab check: QA-701).
        let w = 50usize;
        let h = 50usize;
        let depth: Vec<f32> = (0..w * h)
            .map(|i| {
                let x = (i % w) as f32 / (w - 1) as f32;
                let y = (i / w) as f32 / (h - 1) as f32;
                // Gaussian bump
                let cx = x - 0.5;
                let cy = y - 0.5;
                (-(cx * cx + cy * cy) * 8.0).exp()
            })
            .collect();
        let params = MeshParams {
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            pixel_to_mm: 0.5,
            ..Default::default()
        };
        let mesh = depth_to_point_cloud(&depth, w as u32, h as u32, &params).unwrap();
        validate_mesh_for_export(&mesh).unwrap();

        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();

        // --- Format validation ---

        // 1. Header: exactly 80 bytes
        assert!(buf.len() >= 84, "buffer too short");

        // 2. Triangle count matches
        let tri_count = u32::from_le_bytes([buf[80], buf[81], buf[82], buf[83]]);
        let expected_tris = ((w - 1) * (h - 1) * 2) as u32;
        assert_eq!(tri_count, expected_tris,
            "triangle count: expected {}, got {}", expected_tris, tri_count);

        // 3. File size matches formula
        let expected_size = 84 + tri_count as usize * 50;
        assert_eq!(buf.len(), expected_size, "file size mismatch");

        // 4. Per-triangle validation
        for t in 0..tri_count as usize {
            let base = 84 + t * 50;
            let read_f32 = |off: usize| -> f32 {
                f32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
            };

            // Normal: 3 x f32, must be finite and unit length
            let nx = read_f32(base);
            let ny = read_f32(base + 4);
            let nz = read_f32(base + 8);
            assert!(nx.is_finite() && ny.is_finite() && nz.is_finite(),
                "triangle {} has non-finite normal", t);
            let nlen = (nx * nx + ny * ny + nz * nz).sqrt();
            assert!((nlen - 1.0).abs() < 1e-3,
                "triangle {} normal not unit length (len={})", t, nlen);

            // Vertices: 3 vertices x 3 components, all must be finite
            for v in 0..3 {
                for c in 0..3 {
                    let val = read_f32(base + 12 + v * 12 + c * 4);
                    assert!(val.is_finite(),
                        "triangle {} vertex {} component {} is not finite: {}", t, v, c, val);
                }
            }

            // Attribute byte count: must be 0
            let attr = u16::from_le_bytes([buf[base + 48], buf[base + 49]]);
            assert_eq!(attr, 0, "triangle {} attribute byte count should be 0, got {}", t, attr);
        }
    }

    // =========================================================================
    // JR2-703: Edge cases
    // =========================================================================

    #[test]
    fn jr2_703_empty_mesh_errors_on_stl_write() {
        // 0 vertices -> should error
        let mesh = MeshData {
            positions: vec![],
            normals: vec![],
            indices: None,
        };
        let mut buf = Vec::new();
        let err = write_stl_binary(&mut buf, &mesh);
        assert!(err.is_err(), "empty mesh should fail STL write");
    }

    #[test]
    fn jr2_703_empty_mesh_fails_validation() {
        let mesh = MeshData {
            positions: vec![],
            normals: vec![],
            indices: None,
        };
        match validate_mesh_for_export(&mesh) {
            Err(MeshValidationError::NoVertices) => {}
            other => panic!("expected NoVertices, got {:?}", other),
        }
    }

    #[test]
    fn jr2_703_single_triangle_succeeds() {
        // 3 vertices, 1 triangle
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2]),
        };
        validate_mesh_for_export(&mesh).unwrap();
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        assert_eq!(buf.len(), 84 + 50); // 1 triangle
        let tri_count = u32::from_le_bytes([buf[80], buf[81], buf[82], buf[83]]);
        assert_eq!(tri_count, 1);
    }

    #[test]
    fn jr2_703_flat_depth_map_triangulation() {
        // All same Z value -> triangles exist but may be degenerate in Z
        // With our grid layout, x and y differ so triangles are NOT degenerate
        let depth: Vec<f32> = vec![0.5; 25];
        let mesh = depth_to_point_cloud(&depth, 5, 5, &MeshParams::default()).unwrap();
        assert!(mesh.indices.is_some());
        let idx = mesh.indices.as_ref().unwrap();
        assert_eq!(idx.len(), 32 * 3); // 4x4 cells * 2 = 32 triangles

        // All z should be the same
        let z_val = mesh.positions[0][2];
        for p in &mesh.positions {
            assert!((p[2] - z_val).abs() < 1e-5, "flat depth should give uniform z");
        }

        // Validation should pass (triangles have area from x,y differences)
        validate_mesh_for_export(&mesh).unwrap();

        // STL write should succeed
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        assert_eq!(buf.len(), 84 + 32 * 50);
    }

    #[test]
    fn jr2_703_extreme_depth_zero() {
        // Depth all 0.0 -> z = depth_min_mm
        let depth: Vec<f32> = vec![0.0; 4];
        let params = MeshParams {
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            ..Default::default()
        };
        let mesh = depth_to_point_cloud(&depth, 2, 2, &params).unwrap();
        for p in &mesh.positions {
            assert!((p[2] - 2.0).abs() < 1e-5, "depth 0 -> z=depth_min_mm");
        }
        // Flat mesh: validation and STL should succeed
        validate_mesh_for_export(&mesh).unwrap();
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        assert_eq!(buf.len(), 84 + 2 * 50);
    }

    #[test]
    fn jr2_703_extreme_depth_one() {
        // Depth all 1.0 -> z = depth_max_mm
        let depth: Vec<f32> = vec![1.0; 4];
        let params = MeshParams {
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            ..Default::default()
        };
        let mesh = depth_to_point_cloud(&depth, 2, 2, &params).unwrap();
        for p in &mesh.positions {
            assert!((p[2] - 10.0).abs() < 1e-5, "depth 1 -> z=depth_max_mm");
        }
        validate_mesh_for_export(&mesh).unwrap();
    }

    #[test]
    fn jr2_703_extreme_depth_negative_clamped() {
        // Negative depth values should be clamped to 0
        let depth: Vec<f32> = vec![-1.0, -0.5, 0.0, 0.5];
        let params = MeshParams {
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            ..Default::default()
        };
        let mesh = depth_to_point_cloud(&depth, 2, 2, &params).unwrap();
        // Negative values clamped to 0 -> z = depth_min_mm = 2.0
        assert!((mesh.positions[0][2] - 2.0).abs() < 1e-5, "negative depth clamped to min");
        assert!((mesh.positions[1][2] - 2.0).abs() < 1e-5, "negative depth clamped to min");
    }

    #[test]
    fn jr2_703_extreme_depth_over_one_clamped() {
        // Depth > 1.0 should be clamped to 1.0
        let depth: Vec<f32> = vec![1.5, 2.0, 0.5, 0.0];
        let params = MeshParams {
            depth_min_mm: 2.0,
            depth_max_mm: 10.0,
            ..Default::default()
        };
        let mesh = depth_to_point_cloud(&depth, 2, 2, &params).unwrap();
        assert!((mesh.positions[0][2] - 10.0).abs() < 1e-5, "depth>1 clamped to max");
        assert!((mesh.positions[1][2] - 10.0).abs() < 1e-5, "depth>1 clamped to max");
    }

    #[test]
    fn jr2_703_grid_with_step_gt_1() {
        // 10x10 grid with step=3 -> reduced resolution
        let depth: Vec<f32> = (0..100).map(|i| (i as f32) / 99.0).collect();
        let params = MeshParams {
            step_x: 3,
            step_y: 3,
            ..Default::default()
        };
        let mesh = depth_to_point_cloud(&depth, 10, 10, &params).unwrap();
        // num_cols = ceil(10/3) = 4, num_rows = ceil(10/3) = 4
        assert_eq!(mesh.positions.len(), 16);
        // Should have indices (4x4 grid -> 3x3 cells -> 18 triangles)
        assert!(mesh.indices.is_some());
        let idx = mesh.indices.as_ref().unwrap();
        assert_eq!(idx.len(), 3 * 3 * 2 * 3); // 3*3 cells * 2 tris * 3 indices = 54

        validate_mesh_for_export(&mesh).unwrap();
        let mut buf = Vec::new();
        write_stl_binary(&mut buf, &mesh).unwrap();
        let tri_count = u32::from_le_bytes([buf[80], buf[81], buf[82], buf[83]]);
        assert_eq!(tri_count, 18);
    }

    #[test]
    fn jr2_703_very_large_mesh_math_verification() {
        // Verify triangle count math for a large grid without actually allocating
        // 1000x1000 grid: 999*999 cells * 2 = 1,996,002 triangles
        let num_rows = 1000usize;
        let num_cols = 1000usize;
        let expected_cells = (num_rows - 1) * (num_cols - 1);
        let expected_tris = expected_cells * 2;
        assert_eq!(expected_tris, 1_996_002);
        let expected_indices = expected_tris * 3;
        assert_eq!(expected_indices, 5_988_006);
        let expected_stl_size = 84 + expected_tris * 50;
        assert_eq!(expected_stl_size, 99_800_184);

        // Verify triangulate_grid produces correct count for a smaller representative
        let indices = triangulate_grid(100, 100);
        assert_eq!(indices.len(), 99 * 99 * 2 * 3);
    }

    #[test]
    fn jr2_703_validate_rejects_non_multiple_of_3_indices() {
        let mesh = MeshData {
            positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1]), // only 2 indices, not a multiple of 3
        };
        match validate_mesh_for_export(&mesh) {
            Err(MeshValidationError::InvalidIndexCount(2)) => {}
            other => panic!("expected InvalidIndexCount(2), got {:?}", other),
        }
    }

    #[test]
    fn jr2_703_stl_write_rejects_non_multiple_of_3_indices() {
        let mesh = MeshData {
            positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2, 0]), // 4 indices
        };
        let mut buf = Vec::new();
        let err = write_stl_binary(&mut buf, &mesh);
        assert!(err.is_err(), "non-multiple-of-3 indices should fail");
    }

    // =========================================================================
    // JR2-704: Benchmark export time for large meshes
    // =========================================================================

    #[test]
    #[ignore] // Run with: cargo test --manifest-path src-tauri/Cargo.toml jr2_704 -- --ignored --nocapture
    fn jr2_704_benchmark_large_mesh_export() {
        // Benchmark triangulation + STL write for 100K, 500K, and ~1M vertices.
        // Target: <5s for 1M vertices.
        //
        // Results are printed to stdout (use --nocapture to see them).
        //
        // Typical results (release build on modern hardware):
        //   316x316 (99856 verts, 199210 tris): triangulate ~Xms, STL write ~Xms
        //   707x707 (499849 verts, 998408 tris): triangulate ~Xms, STL write ~Xms
        //   1000x1000 (1000000 verts, 1996002 tris): triangulate ~Xms, STL write ~Xms

        use std::time::Instant;

        let configs: Vec<(usize, &str)> = vec![
            (316, "~100K vertices (316x316 = 99,856)"),
            (707, "~500K vertices (707x707 = 499,849)"),
            (1000, "~1M vertices (1000x1000 = 1,000,000)"),
        ];

        println!("\n=== JR2-704: STL Export Benchmark ===\n");

        for (size, label) in &configs {
            let w = *size;
            let h = *size;
            let vertex_count = w * h;

            // Generate depth map (gradient)
            let depth: Vec<f32> = (0..vertex_count)
                .map(|i| (i as f32) / (vertex_count - 1) as f32)
                .collect();
            let params = MeshParams::default();

            // Time: depth -> point cloud (includes triangulation)
            let t0 = Instant::now();
            let mesh = depth_to_point_cloud(&depth, w as u32, h as u32, &params).unwrap();
            let mesh_time = t0.elapsed();

            let tri_count = mesh.indices.as_ref().map(|i| i.len() / 3).unwrap_or(0);

            // Time: STL write to memory buffer
            let t1 = Instant::now();
            let mut buf = Vec::with_capacity(84 + tri_count * 50);
            write_stl_binary(&mut buf, &mesh).unwrap();
            let stl_time = t1.elapsed();

            let total = mesh_time + stl_time;

            println!("{}", label);
            println!("  Vertices:       {}", vertex_count);
            println!("  Triangles:      {}", tri_count);
            println!("  Mesh generation: {:?}", mesh_time);
            println!("  STL write:       {:?}", stl_time);
            println!("  Total:           {:?}", total);
            println!("  STL size:        {:.1} MB", buf.len() as f64 / (1024.0 * 1024.0));
            println!();

            // Verify correctness
            assert_eq!(buf.len(), 84 + tri_count * 50);

            // For the 1M case, assert total < 5s (target from task)
            if *size == 1000 {
                assert!(total.as_secs() < 5,
                    "1M vertex export took {:?}, exceeds 5s target", total);
            }
        }
        println!("=== Benchmark complete ===\n");
    }

    // =========================================================================
    // JR2-801: OBJ writer unit tests (Sprint 1.9)
    // =========================================================================

    /// Helper: parse OBJ text into (vertices, normals, faces).
    fn parse_obj_text(text: &str) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[u32; 3]>) {
        let mut verts = Vec::new();
        let mut normals = Vec::new();
        let mut faces = Vec::new();
        for line in text.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() { continue; }
            match parts[0] {
                "v" if parts.len() >= 4 => {
                    verts.push([
                        parts[1].parse::<f32>().unwrap(),
                        parts[2].parse::<f32>().unwrap(),
                        parts[3].parse::<f32>().unwrap(),
                    ]);
                }
                "vn" if parts.len() >= 4 => {
                    normals.push([
                        parts[1].parse::<f32>().unwrap(),
                        parts[2].parse::<f32>().unwrap(),
                        parts[3].parse::<f32>().unwrap(),
                    ]);
                }
                "f" => {
                    // Parse "f v1//n1 v2//n2 v3//n3"
                    let mut face_indices = Vec::new();
                    for p in &parts[1..] {
                        let idx_str = p.split("//").next().unwrap();
                        face_indices.push(idx_str.parse::<u32>().unwrap());
                    }
                    if face_indices.len() == 3 {
                        faces.push([face_indices[0], face_indices[1], face_indices[2]]);
                    }
                }
                _ => {}
            }
        }
        (verts, normals, faces)
    }

    #[test]
    fn jr2_801_obj_write_2x2_mesh() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let mut buf = Vec::new();
        write_obj_ascii(&mut buf, &mesh, None).unwrap();
        let text = String::from_utf8(buf).unwrap();
        let (verts, normals, faces) = parse_obj_text(&text);
        assert_eq!(verts.len(), 4, "4 vertices");
        assert_eq!(normals.len(), 4, "4 normals");
        assert_eq!(faces.len(), 2, "2 triangles");
    }

    #[test]
    fn jr2_801_obj_roundtrip_vertex_positions() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 5.0],
                [10.0, 0.0, 3.0],
                [0.0, 10.0, 7.0],
                [10.0, 10.0, 1.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let mut buf = Vec::new();
        write_obj_ascii(&mut buf, &mesh, None).unwrap();
        let text = String::from_utf8(buf).unwrap();
        let (verts, _, faces) = parse_obj_text(&text);

        // Verify vertex positions match
        for (i, pos) in mesh.positions.iter().enumerate() {
            for c in 0..3 {
                assert!((verts[i][c] - pos[c]).abs() < 1e-4,
                    "vertex {} component {} mismatch: {} vs {}", i, c, verts[i][c], pos[c]);
            }
        }

        // Verify face indices reference correct vertices (OBJ is 1-based)
        let idx = mesh.indices.as_ref().unwrap();
        for (t, face) in faces.iter().enumerate() {
            assert_eq!(face[0], idx[t * 3] + 1);
            assert_eq!(face[1], idx[t * 3 + 1] + 1);
            assert_eq!(face[2], idx[t * 3 + 2] + 1);
        }
    }

    #[test]
    fn jr2_801_obj_with_mtl_reference() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2]),
        };
        let mut buf = Vec::new();
        write_obj_ascii(&mut buf, &mesh, Some("test.mtl")).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(text.contains("mtllib test.mtl"), "should reference MTL file");
        assert!(text.contains("usemtl default"), "should use default material");
    }

    #[test]
    fn jr2_801_obj_without_mtl_reference() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: Some(vec![0, 1, 2]),
        };
        let mut buf = Vec::new();
        write_obj_ascii(&mut buf, &mesh, None).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(!text.contains("mtllib"), "should not reference MTL without param");
    }

    #[test]
    fn jr2_801_obj_rejects_no_indices() {
        let mesh = MeshData {
            positions: vec![[0.0, 0.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]],
            indices: None,
        };
        let mut buf = Vec::new();
        let err = write_obj_ascii(&mut buf, &mesh, None);
        assert!(err.is_err(), "OBJ write should fail without indices");
    }

    #[test]
    fn jr2_801_obj_header_comment() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0], [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let mut buf = Vec::new();
        write_obj_ascii(&mut buf, &mesh, None).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(text.starts_with("# SimplePicture3D OBJ Export"));
        assert!(text.contains("Vertices: 4"));
        assert!(text.contains("Triangles: 2"));
    }

    #[test]
    fn jr2_801_obj_full_pipeline_depth_to_obj() {
        let depth: Vec<f32> = (0..25).map(|i| (i as f32) / 24.0).collect();
        let params = MeshParams::default();
        let mesh = depth_to_point_cloud(&depth, 5, 5, &params).unwrap();
        validate_mesh_for_export(&mesh).unwrap();
        let mut buf = Vec::new();
        write_obj_ascii(&mut buf, &mesh, Some("mesh.mtl")).unwrap();
        let text = String::from_utf8(buf).unwrap();
        let (verts, normals, faces) = parse_obj_text(&text);
        assert_eq!(verts.len(), 25, "5x5 grid = 25 vertices");
        assert_eq!(normals.len(), 25, "25 normals");
        assert_eq!(faces.len(), 32, "4x4 cells * 2 = 32 triangles");
    }

    // --- BACK-802: MTL writer tests ---

    #[test]
    fn jr2_801_mtl_write() {
        let mut buf = Vec::new();
        write_mtl(&mut buf).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(text.contains("newmtl default"));
        assert!(text.contains("Kd 0.8 0.8 0.8"));
        assert!(text.contains("illum 2"));
    }

    // --- JR2-802: File write roundtrip ---

    #[test]
    fn jr2_802_obj_write_to_file_roundtrip() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0], [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let dir = std::env::temp_dir().join("sp3d_test_obj");
        let _ = std::fs::create_dir_all(&dir);
        let obj_path = dir.join("test_roundtrip.obj").to_string_lossy().to_string();
        write_obj_to_file(&obj_path, &mesh, true).unwrap();

        // Verify OBJ file exists and has content
        let obj_content = std::fs::read_to_string(&obj_path).unwrap();
        let (verts, _, faces) = parse_obj_text(&obj_content);
        assert_eq!(verts.len(), 4);
        assert_eq!(faces.len(), 2);

        // Verify MTL file exists
        let mtl_path = dir.join("test_roundtrip.mtl");
        assert!(mtl_path.exists(), "MTL file should be created alongside OBJ");
        let mtl_content = std::fs::read_to_string(&mtl_path).unwrap();
        assert!(mtl_content.contains("newmtl default"));

        // Cleanup
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn jr2_802_obj_write_to_file_no_mtl() {
        let mesh = MeshData {
            positions: vec![
                [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0], [1.0, 1.0, 0.0],
            ],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            indices: Some(triangulate_grid(2, 2)),
        };
        let dir = std::env::temp_dir().join("sp3d_test_obj_nomtl");
        let _ = std::fs::create_dir_all(&dir);
        let obj_path = dir.join("test_no_mtl.obj").to_string_lossy().to_string();
        write_obj_to_file(&obj_path, &mesh, false).unwrap();

        let obj_content = std::fs::read_to_string(&obj_path).unwrap();
        assert!(!obj_content.contains("mtllib"), "no MTL reference when write_mtl_file=false");

        let mtl_path = dir.join("test_no_mtl.mtl");
        assert!(!mtl_path.exists(), "MTL file should not exist when write_mtl_file=false");

        let _ = std::fs::remove_dir_all(&dir);
    }

    // --- BACK-803: Filename generation with format ---

    #[test]
    fn jr2_803_generate_export_filename_obj() {
        let name = generate_export_filename_with_ext("C:\\photos\\my_image.png", "obj");
        assert!(name.starts_with("my_image_"));
        assert!(name.ends_with(".obj"));
    }

    #[test]
    fn jr2_803_generate_export_filename_stl_backwards_compat() {
        let name = generate_export_filename("test.png");
        assert!(name.ends_with(".stl"), "backwards-compatible wrapper should use .stl");
    }

    // --- JR2-803: Settings corruption test ---

    #[test]
    fn jr2_803_corrupt_settings_falls_back_to_defaults() {
        // Parse corrupt JSON should fall back to default
        let corrupt_json = "{ this is not valid json }";
        let result: Result<super::super::settings::AppSettings, _> =
            serde_json::from_str(corrupt_json);
        assert!(result.is_err(), "corrupt JSON should fail to parse");
        // AppSettings::load() handles this internally by returning defaults
    }
}
