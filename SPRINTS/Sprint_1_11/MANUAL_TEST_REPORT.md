# Sprint 1.11 — Manual Test Report

**Sprint:** 1.11  
**Owner:** Quality Engineer (session-qa-cursor-20260222)  
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
| Load image | ⏳ To be executed | Manual run on dev machine |
| Generate depth | ⏳ To be executed | Stub or real model |
| Adjust depth | ⏳ To be executed | |
| Set target size 50×70 mm | ⏳ To be executed | |
| Export STL/OBJ | ⏳ To be executed | |

**Sign-off:** Execute the procedure above and fill the table. If all steps pass, mark "No critical path broken" in VERIFICATION_CHECKLIST.

---

## 2. Regression (QA-1003)

**Scope:** Previous sprint features — image load, depth, mesh, preview, export, settings, model wizard.

### Checklist

| Feature | Test | Result | Notes |
|---------|------|--------|-------|
| Image load (PNG/JPG) | Load valid file; reject invalid | ⏳ | |
| Depth estimation | Run with stub/real model; depth displayed | ⏳ | |
| Mesh generation | get_mesh_data returns vertices | ⏳ | |
| 3D preview | Preview renders; rotate/zoom | ⏳ | |
| Export STL/OBJ | Export to path; file valid | ⏳ | |
| Settings | get/save settings; persist on restart | ⏳ | |
| Model wizard | Download/setup model if applicable | ⏳ | |

**Regressions:** Any failure should be filed using `.github/ISSUE_TEMPLATE/bug_report.md` and triaged (BUG-1001/1002).

---

## 3. Performance (QA-1004)

**Reference:** prd.md §7.1.

| Operation | Target | Max Acceptable | Measured | Met |
|-----------|--------|----------------|----------|-----|
| App Startup | <2s | 5s | — | ⏳ |
| Image Load (4K PNG) | <500ms | 1s | — | ⏳ |
| Depth Estimation (4K, GPU) | <10s | 30s | — | ⏳ |
| Depth Estimation (4K, CPU) | <60s | 120s | — | ⏳ |
| Mesh Generation (4K) | <15s | 30s | — | ⏳ |
| Preview Render (100K vertices) | 60fps | 30fps | — | ⏳ |
| STL Export (1M vertices) | <5s | 15s | — | Unit test asserts <5s (JR2-704) |
| Memory (Idle) | <200MB | 500MB | — | ⏳ |
| Memory (Processing 4K) | <2GB | 4GB | — | ⏳ |

**Notes:** Key metrics to be measured on a run (manual or CI). Gaps documented for Phase 2. STL export 1M vertices is covered by `cargo test` benchmark in `mesh_generator.rs` (JR2-704).

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

### Results

| Step | Result | Notes |
|------|--------|-------|
| Set 50×70 mm | ⏳ To be executed | |
| Export STL | ⏳ To be executed | |
| Open in MeshLab | ⏳ To be executed | |
| Verify dimensions & aspect | ⏳ To be executed | |

**Sign-off:** When executed, record pass/fail and any deviation (e.g. measured size in mm in MeshLab).

---

## 5. Summary

- **Full workflow (QA-1002):** Procedure and result table above; fill after manual run.
- **Regression (QA-1003):** Checklist above; file regressions as bugs and triage.
- **Performance (QA-1004):** PRD §7.1 table; measure and document; STL 1M covered by unit test.
- **Target dimensions (QA-1006):** 50×70 mm procedure above; verify in MeshLab and record.

All artefacts stored in `SPRINTS/Sprint_1_11/` per task assignment.
