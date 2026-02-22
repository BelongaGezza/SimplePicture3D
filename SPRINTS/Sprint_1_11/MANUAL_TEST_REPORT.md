# Sprint 1.11 — Manual Test Report

**Sprint:** 1.11
**Owner:** Quality Engineer (session-qa-cursor-20260222)
**Executed by:** External Software Consultant (automated + programmatic verification)
**Execution Date:** 2026-02-22
**Last Updated:** 2026-02-22

---

## 1. Full workflow (QA-1002)

**Scope:** Load image → generate depth → adjust → (target size) → export STL/OBJ.

### Procedure

1. Start app: `npm run tauri dev`
2. **Load:** File → Open / drag image (PNG or JPG); confirm image appears in 2D view.
3. **Depth:** Run depth estimation (Generate Depth); confirm depth map appears (grayscale).
4. **Adjust:** Change depth sliders (brightness, gamma, invert) if available; confirm preview updates.
5. **Target size (optional):** In Export Settings, set Output size e.g. 50×70 mm (preset or Custom).
6. **Export:** Export → STL (binary) and/or OBJ to a chosen path; confirm file created and no error.

### Expected

- No crash; no critical path broken; exported file opens in MeshLab or equivalent.

### Results

| Step | Result | Notes |
|------|--------|-------|
| Load image | PASS (backend) | `load_image_impl` tested: valid PNG loads, invalid/corrupt files rejected, path traversal handled, Unicode paths supported, >8K auto-downsampled. 141 Rust tests pass. |
| Generate depth | PASS (stub) | Python depth estimator: 32 pytest tests pass (stub mode). CLI output validated: JSON shape matches image, values normalised to [0,1]. |
| Adjust depth | PASS (backend) | `depth_adjust` module: brightness, contrast, gamma, invert, depth-to-mm all tested. Pipeline identity and boundary conditions verified. |
| Set target size 50×70 mm | PASS (backend) | `compute_pixel_to_mm(100,100,50,70) = 0.5`; 6 target dimension tests pass (fit, aspect ratio, default fallback). |
| Export STL/OBJ | PASS (backend) | STL binary write + OBJ ASCII write tested: roundtrip vertex data, header format, normals, MTL reference, file I/O. Full pipeline (depth→mesh→STL/OBJ) passes. |

**Sign-off:** All backend workflow steps verified programmatically. No regressions. **GUI-only verification** (visual 2D view, drag-and-drop, 3D preview rendering) **requires human tester with `npm run tauri dev`**.

---

## 2. Regression (QA-1003)

**Scope:** Previous sprint features — image load, depth, mesh, preview, export, settings, model wizard.

### Automated Test Results

| Suite | Tests | Result | Duration |
|-------|-------|--------|----------|
| Rust (`cargo test`) | 141 passed, 6 ignored | PASS | 0.37s |
| Frontend (`npm test`) | 39 passed (5 suites) | PASS | 2.47s |
| Python (`pytest`, stub) | 32 passed | PASS | 0.88s |
| **Total** | **212 passing** | **PASS** | **3.72s** |

### Checklist

| Feature | Test | Result | Notes |
|---------|------|--------|-------|
| Image load (PNG/JPG) | Load valid file; reject invalid | PASS | `load_image_impl`: PNG/JPG magic bytes validated, invalid files rejected, corrupt data errors gracefully, paths with spaces and Unicode handled |
| Depth estimation | Run with stub/real model; depth displayed | PASS | 20 Python tests cover stub inference, CLI output, normalization, error cases |
| Mesh generation | get_mesh_data returns vertices | PASS | `depth_to_point_cloud`: 3x3, 5x5, step=2, single pixel/row/column, extreme coordinates — all pass |
| 3D preview | Preview renders; rotate/zoom | PASS (frontend mocks) | 5 Vitest `depthCanvas` tests verify canvas rendering, 21 `tauri` IPC mock tests. Visual rendering requires human tester. |
| Export STL/OBJ | Export to path; file valid | PASS | STL binary roundtrip, OBJ ASCII roundtrip, 1M vertex benchmark, file write to disk — all verified |
| Settings | get/save settings; persist on restart | PASS | `settings.rs`: roundtrip JSON, partial JSON, corrupt JSON, defaults, skip-serializing-none, extended fields (target dimensions) |
| Model wizard | Download/setup model if applicable | PASS | `model_downloader.py`: 12 tests — check_model_installed, partial files, model info, model dir, JSON serialization |

### Code Quality

| Check | Result | Notes |
|-------|--------|-------|
| `cargo clippy -- -D warnings` | PASS | Zero warnings |
| `npm run build` | PASS | Built in 1.35s. A11y warnings noted (P2/P3, deferred to Phase 2) |
| `cargo audit` | PASS | 19 allowed warnings (unmaintained GTK/Tauri transitive deps); no critical/high |
| `npm audit` | 7 moderate | esbuild dev server + Svelte SSR context — desktop app, SSR not used; acceptable for MVP |

**Regressions found:** None.

---

## 3. Performance (QA-1004)

**Reference:** prd.md §7.1.
**Test environment:** Windows 11, debug build (unoptimized), measured via `cargo test --ignored --nocapture`.

| Operation | Target | Max Acceptable | Measured | Met |
|-----------|--------|----------------|----------|-----|
| App Startup | <2s | 5s | — | Requires GUI (`npm run tauri dev`) |
| Image Load (4K PNG) | <500ms | 1s | <50ms (unit test, in-memory) | PASS |
| Depth Estimation (4K, GPU) | <10s | 30s | — | Requires GPU + AI model |
| Depth Estimation (4K, CPU) | <60s | 120s | — | Requires AI model |
| Mesh Generation (100K vertices) | <15s | 30s | 10.8ms (debug build) | PASS |
| Mesh Generation (500K vertices) | <15s | 30s | 53.0ms (debug build) | PASS |
| Mesh Generation (1M vertices) | <15s | 30s | 108.9ms (debug build) | PASS |
| Preview Render (100K vertices) | 60fps | 30fps | — | Requires GUI |
| STL Export (100K vertices) | <5s | 15s | 71.0ms | PASS |
| STL Export (500K vertices) | <5s | 15s | 359.5ms | PASS |
| STL Export (1M vertices) | <5s | 15s | 720.3ms | PASS |
| Total Pipeline (1M: mesh + STL) | <5s | 15s | 829.2ms (debug build) | PASS |
| Memory (Idle) | <200MB | 500MB | — | Requires running app |
| Memory (Processing 4K) | <2GB | 4GB | — | Requires running app |

**Performance notes:**
- All mesh/export benchmarks measured in **debug build** (unoptimized). Release build expected to be 3-5x faster.
- 1M vertex STL export: **829ms total** (debug) — well under the 5s target.
- STL file sizes: 100K verts = 9.5 MB, 500K = 47.5 MB, 1M = 95.2 MB.
- Items marked "—" require the full Tauri app running or AI model downloaded; deferred to human tester or Phase 2 profiling.

---

## 4. Target dimensions — 50×70 mm (QA-1006)

**Dependency:** BACK-1005 (implemented).

### Procedure

1. Load an image and generate depth (or use existing mesh).
2. In Export Settings, set Output size to **50×70 mm** (preset or Custom width=50, height=70).
3. Export STL to a file (e.g. `test_50x70.stl`).
4. Open the STL in **MeshLab** (or Netfabb/Blender).
5. **Verify:** Mesh XY bounds fit inside 50 mm × 70 mm rectangle; aspect ratio of the mesh matches image aspect ratio (mesh not stretched).

### Expected

- Mesh fits within 50×70 mm; aspect ratio preserved (scale uniform along X and Y from same pixel_to_mm).

### Programmatic Verification Results

**6 target dimension tests executed — all PASS:**

| Test | Input | Expected | Result | Notes |
|------|-------|----------|--------|-------|
| `compute_pixel_to_mm_target_dimensions_fit_and_aspect_preserved` | 100x100px, 50x70mm | pixel_to_mm = 0.5 | PASS | Fit + aspect verified |
| `target_dimensions_set_mesh_fits_inside_rectangle` | 100x100px depth, 50x70mm | mesh XY ≤ 50x70mm | PASS | `extent_w ≤ 50.0`, `extent_h ≤ 70.0` asserted |
| `target_dimensions_set_aspect_ratio_preserved` | 200x100px (landscape), 50x70mm | pixel_to_mm = 0.25, mesh = 50x25mm | PASS | Aspect ratio 2:1 preserved |
| `target_dimensions_mesh_xy_fits_and_aspect_preserved` | Full pipeline check | Mesh fits target | PASS | End-to-end |
| `target_dimensions_unset_default_behaviour` | No target set | pixel_to_mm = 1.0 | PASS | Fallback to default |
| `target_dimensions_unset_default_pixel_to_mm` | Zero/negative | pixel_to_mm = 1.0 | PASS | Edge case handling |

### Results

| Step | Result | Notes |
|------|--------|-------|
| Set 50×70 mm | PASS (programmatic) | `compute_pixel_to_mm(100, 100, 50.0, 70.0) = 0.5` verified |
| Export STL | PASS (programmatic) | `full_pipeline_depth_to_stl` test: depth→mesh→STL binary write roundtrip passes |
| Open in MeshLab | Requires human tester | MeshLab verification deferred to manual execution |
| Verify dimensions & aspect | PASS (programmatic) | Mesh XY bounds asserted ≤ 50×70 mm; aspect ratio preserved (uniform pixel_to_mm) |

**Sign-off:** Target dimensions logic verified programmatically across multiple image aspect ratios (square, landscape). Mesh correctly fits inside target rectangle while preserving aspect ratio. **MeshLab visual verification requires human tester.**

---

## 5. Summary

### Test Execution Summary

| Area | Automated | Result | GUI-Only (Human Required) |
|------|-----------|--------|---------------------------|
| Full workflow (QA-1002) | Backend pipeline: load→depth→adjust→target→export | PASS | Visual 2D view, drag-drop, 3D preview |
| Regression (QA-1003) | 212 tests across 3 suites | PASS | 3D preview rotate/zoom, visual rendering |
| Performance (QA-1004) | Mesh gen + STL export benchmarks (up to 1M verts) | PASS | App startup, memory, GPU depth estimation |
| Target dimensions (QA-1006) | 6 programmatic tests (fit, aspect, edge cases) | PASS | MeshLab visual verification |

### Overall Assessment

- **212/212 automated tests passing** — zero failures, zero regressions
- **Performance targets met** — 1M vertex mesh+STL in 829ms (debug build), well under 5s target
- **Code quality clean** — clippy zero warnings, no critical security vulnerabilities
- **Target dimensions verified** — mesh fits 50×70mm, aspect ratio preserved across test cases

### Items Requiring Human Tester

The following items cannot be verified programmatically and require a human tester running `npm run tauri dev`:

1. **App startup time** — measure wall clock from launch to interactive
2. **Visual 2D image display** — confirm loaded image renders in UI
3. **3D preview rendering** — confirm Three.js preview renders, supports rotate/zoom
4. **Depth map visualization** — confirm grayscale depth map displayed
5. **Export Settings UI** — confirm preset dropdown and custom inputs work
6. **Drag-and-drop file import** — confirm file drop zone works
7. **MeshLab verification** — open exported STL/OBJ, verify dimensions visually
8. **Memory usage** — monitor via Task Manager during 4K processing
9. **Depth estimation with real model** — requires AI model download

All artefacts stored in `SPRINTS/Sprint_1_11/` per task assignment.
