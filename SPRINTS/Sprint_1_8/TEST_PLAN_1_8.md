# Test Plan -- Sprint 1.8: STL Export

**Author:** Quality Engineer (Claude-Code-QA)
**Date:** 2026-02-08
**Sprint:** 1.8 -- STL Export Implementation
**Status:** Active

---

## 1. Scope

This test plan covers the STL export pipeline introduced in Sprint 1.8:
- Grid-based triangulation (BACK-700)
- Binary STL writer (BACK-701)
- Pre-export mesh validation (BACK-702)
- Export Tauri command and file dialog (BACK-703, BACK-704)
- Auto-generated filenames (BACK-705)
- Persistent export settings (BACK-706)
- Export panel UI (UI-701--704)
- Security hardening (SEC-401, SEC-402)

---

## 2. Automated Test Coverage Summary

### 2.1 Existing Tests (JR2-701--704 and Senior Engineer)

| Test Area | Test Count | Coverage |
|-----------|-----------|----------|
| **Triangulation (BACK-700)** | 7 tests | Grid sizes (2x2, 3x3, edge cases), CCW winding, integration with depth_to_point_cloud, single row/col (no indices) |
| **STL Writer (BACK-701)** | 5 tests (SE) + 11 tests (JR2-701) | Binary format correctness, header, byte layout, roundtrip (write-read-compare), file write, reject no-indices, extreme coordinates (large/small/negative), normals unit-length, flat mesh normals +Z, file size formula |
| **Mesh Validation (BACK-702)** | 8 tests | Valid mesh, no vertices, no triangles, empty indices, NaN, Inf, index out of bounds, degenerate triangles |
| **Full Pipeline** | 1 test | depth -> mesh -> validate -> STL (5x5 grid, 32 triangles) |
| **Filename Generation (BACK-705)** | 4 tests | From image path, empty path, spaces, timestamp format |
| **Settings (BACK-706)** | 5 tests | Default, JSON roundtrip, load fallback, path existence, save/load |
| **Programmatic STL Validation (JR2-702)** | 1 test | 50x50 Gaussian bump, per-triangle binary format check |
| **Edge Cases (JR2-703)** | 11 tests | Empty mesh, single triangle, flat depth, extreme depth (0/1/negative/>1), step>1, large mesh math, non-multiple-of-3 indices |
| **Benchmark (JR2-704)** | 1 test (#[ignore]) | 100K/500K/1M vertices; all under 5s target |
| **Total** | **54 tests** (STL-related) | See below for full suite |

### 2.2 Full Test Suite

- **Total Rust tests:** 113 passed, 0 failed, 6 ignored
- **Ignored tests:** 5 Python-dependent (require Python env), 1 benchmark (#[ignore])
- **Key files:**
  - `src-tauri/src/mesh_generator.rs` -- 51 tests (triangulation, STL writer, validation, filename, edge cases)
  - `src-tauri/src/lib.rs` -- 14 tests (Tauri commands, integration)
  - `src-tauri/src/settings.rs` -- 5 tests
  - `src-tauri/src/depth_adjust.rs` -- depth adjustment tests
  - `src-tauri/src/image_loading.rs` -- image loading tests

---

## 3. Automated Round-Trip Tests (QA-704)

The following existing tests satisfy the QA-704 round-trip requirement:

| Test Name | What It Does | Verifies |
|-----------|-------------|----------|
| `jr2_701_roundtrip_small_quad` | 2x2 quad: write STL -> parse binary -> compare vertex positions | Triangle count, vertex positions within f32 tolerance, attribute bytes = 0 |
| `jr2_701_roundtrip_all_vertices_match` | 3x3 depth map: depth -> mesh -> STL -> parse -> compare every vertex for every triangle | Full pipeline vertex fidelity |
| `jr2_701_roundtrip_medium_grid_100x100` | 100x100 grid (19,602 triangles): write -> parse -> verify count and size | Large mesh round-trip correctness |
| `stl_write_to_file_roundtrip` | Write to temp file -> read back -> verify size and triangle count | File I/O round-trip |
| `stl_binary_roundtrip_vertex_data` | Write -> read back first triangle vertex positions | Binary format correctness |
| `jr2_701_stl_normals_unit_length` | 5x5 grid: write -> parse -> verify all face normals unit-length | Normal vector integrity |
| `jr2_702_programmatic_stl_format_validation` | 50x50 Gaussian bump: full per-triangle binary validation (normals finite + unit, vertices finite, attr=0) | STL format compliance |
| `full_pipeline_depth_to_stl` | 5x5 depth -> mesh -> validate -> STL: verify triangle count and file size | End-to-end pipeline |

**Assessment:** These 8 tests comprehensively cover the round-trip requirement. They verify:
- Triangle count matches between mesh and STL output
- Vertex positions match within f32 tolerance (1e-5)
- Normals are unit-length
- Binary format is correct (header, byte layout, attribute bytes)
- File I/O works (temp file write and read back)

**QA-704 status: SATISFIED by existing tests.**

---

## 4. Manual Test Procedures

### 4.1 QA-701: Export STL and Open in External Tool

**Objective:** Verify the exported STL file is valid and renders correctly in MeshLab and/or PrusaSlicer.

**Procedure:**
1. Launch SimplePicture3D (`npm run tauri dev`)
2. Click "Open Image" and load a test image (any photograph with depth variation)
3. Click "Generate Depth Map" -- wait for completion
4. In the Export Panel, ensure format is "STL"
5. Click "Export STL" -- save dialog should appear
6. Choose a location and filename, click Save
7. Verify success notification appears with file path and "Open Folder" button
8. Open the exported `.stl` file in MeshLab:
   - File > Import Mesh > select the .stl file
   - Verify mesh loads without errors
   - Visually inspect mesh shape matches the depth variation
9. Open in PrusaSlicer (optional):
   - File > Add... > select the .stl file
   - Verify it loads and displays correctly

**Pass Criteria:**
- STL file created successfully
- No errors or warnings from MeshLab/PrusaSlicer
- Mesh shape visually corresponds to the source image depth

**Status:** Procedure documented; execution pending manual testing.

### 4.2 QA-702: Dimension Verification

**Objective:** Verify exported mesh dimensions match the specified depth range parameters.

**Procedure:**
1. Launch SimplePicture3D
2. Load an image with clearly varying depth (e.g., a photo with foreground and background)
3. Set depth range to known values (e.g., depth_min_mm = 2.0, depth_max_mm = 10.0) via depth adjustment panel
4. Generate depth map
5. Export STL
6. Open in MeshLab:
   - Filters > Quality Measure and Computations > Compute Geometric Measures
   - Note the bounding box Z min and Z max values
7. Verify:
   - Z min is approximately depth_min_mm (2.0 mm) within +/-0.1 mm
   - Z max is approximately depth_max_mm (10.0 mm) within +/-0.1 mm
   - X and Y extent corresponds to image dimensions * pixel_to_mm

**Pass Criteria:**
- Z range within +/-0.1 mm of specified depth_min_mm and depth_max_mm
- Mesh dimensions are physically reasonable

**Automated Partial Verification:**
- `z_range_respected` test verifies depth 0.0 -> z=2.0mm and depth 1.0 -> z=10.0mm programmatically
- `point_cloud_5x5_vertex_count_and_bounds` test verifies Z bounds are within [2, 10] mm

**Status:** Procedure documented; execution pending manual testing. Partially verified by automated tests.

### 4.3 QA-703: Filename Generation Testing

**Objective:** Verify auto-generated filenames are correct, safe, and handle edge cases.

**Procedure:**
1. Load an image with a normal filename (e.g., `photo.png`) -- verify suggested filename format: `photo_YYYYMMDD_HHMMSS.stl`
2. Load an image with spaces in the name (e.g., `my photo.jpg`) -- verify spaces replaced: `my_photo_YYYYMMDD_HHMMSS.stl`
3. Load an image with special characters (e.g., `image (1).png`) -- verify special chars replaced: `image__1__YYYYMMDD_HHMMSS.stl`
4. Load an image with Unicode characters (e.g., `foto.png`) -- verify Unicode alphanumeric chars preserved or replaced safely
5. Load without any image -- verify fallback: `mesh_YYYYMMDD_HHMMSS.stl`
6. Verify timestamp is current and in sortable format (YYYYMMDD_HHMMSS)

**Automated Verification (existing tests):**

| Test | Edge Case | Status |
|------|-----------|--------|
| `generate_export_filename_from_image_path` | Windows path `C:\photos\my_image.png` | PASS |
| `generate_export_filename_empty_path` | Empty string -> "mesh" fallback | PASS |
| `generate_export_filename_with_spaces` | Spaces in filename -> underscores | PASS |
| `generate_export_filename_format` | Timestamp format 15 chars YYYYMMDD_HHMMSS | PASS |

**Missing edge cases for automated testing:**
- Unicode filenames (non-ASCII alphanumeric characters)
- Very long filenames
- Filenames with only special characters

**Status:** Partially verified by 4 automated tests. Unicode and long filename edge cases identified for potential future automated coverage. Manual verification of end-to-end dialog flow pending.

---

## 5. Pass/Fail Criteria

### Automated Tests
- All 113 Rust tests must PASS (0 failures)
- cargo clippy must report 0 warnings with `-D warnings`
- npm build must succeed (frontend compiles)
- npm test must succeed (Vitest tests pass)

### Manual Tests (QA-701, QA-702)
- STL opens in at least one external tool without errors
- Z dimensions within +/-0.1mm of specified depth range
- Filename format correct for all tested cases

### Security
- Security sign-off APPROVED (SEC-401, SEC-402)

---

## 6. Test Environment

- **OS:** Windows (primary), macOS/Linux (cross-platform verification deferred)
- **Rust toolchain:** stable
- **Node.js:** per package.json
- **External tools for manual testing:** MeshLab, PrusaSlicer (either one sufficient)

---

**Document Version:** 1.0
**Last Updated:** 2026-02-08
