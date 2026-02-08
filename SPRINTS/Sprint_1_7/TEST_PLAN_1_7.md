# Sprint 1.7 Test Plan

**Sprint:** 1.7 — 3D Preview Rendering  
**Source:** `SPRINT_1_7_Task_Assignment.md`, `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Last Updated:** 2026-02-07

---

## 1. Scope

| Item | Description |
|------|-------------|
| Sprint goal | Display generated mesh in interactive 3D viewport. |
| Features in scope | Three.js integration, 3D scene (camera, lights, grid), load mesh from Rust (get_mesh_data), point cloud rendering (THREE.Points), orbit controls (rotate, zoom, pan), render-mode toggle (points; wireframe/solid placeholder), camera presets, mesh statistics, lighting controls, BACK-601/602 (IPC/serialization per ADR-007), optional LOD (BACK-603). |
| Out of scope | Triangulated mesh rendering (Sprint 1.8); STL/OBJ export (Sprint 1.8). |

---

## 2. Automated Tests

### 2.1 What runs in CI

| Suite | Command | When |
|-------|---------|------|
| Rust | `cargo test --manifest-path src-tauri/Cargo.toml` | Every push/PR |
| Rust | `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` | Every push/PR |
| Frontend | `npm run build` | Every push/PR |
| Frontend | `npm test` | Every push/PR |
| Python | `SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v` | Every push/PR |

**Reference:** `.github/workflows/ci.yml`, `CLAUDE.md`.

### 2.2 New or updated automated tests this sprint

| Test | Location | Purpose |
|------|----------|---------|
| (Optional) 3D preview component mount | `src/components/__tests__/` | Smoke: preview component renders, no crash when mesh empty |
| (Optional) mesh data parsing | Frontend util or component test | Parse get_mesh_data response into positions/normals for BufferGeometry |

*Add rows as tests are added. Frontend tests should mock Tauri invoke for get_mesh_data.*

### 2.3 Coverage

- **Rust:** No new backend logic required for 1.7 beyond BACK-601/602 (serialization); existing mesh_generator tests remain.
- **Frontend:** New Three.js/preview code should be covered where practical (component mount, mock invoke).

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | Windows 11 (primary); macOS/Linux if available |
| Node | LTS (e.g. 20) |
| Rust | stable |
| GPU | Test on integrated (e.g. Intel UHD) and dedicated GPU if available (QA-602) |

### 3.2 Manual test cases

#### Case 1: Load mesh and display point cloud (QA-603, UI-503, UI-504)

| Field | Content |
|-------|---------|
| **Objective** | Verify mesh data loads from backend and displays as point cloud in 3D viewport. |
| **Preconditions** | App running; image loaded; depth map generated (and optionally adjusted). |
| **Steps** | 1. Generate mesh (or trigger get_mesh_data path). 2. Open 3D preview. 3. Confirm point cloud visible; check scale and orientation (e.g. raised areas match depth). |
| **Expected result** | Point cloud renders; Z (height) matches depth map; no console errors. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 2: Orbit controls (QA-601, UI-505)

| Field | Content |
|-------|---------|
| **Objective** | Verify rotate, zoom, pan are smooth and responsive. |
| **Preconditions** | 3D preview open with mesh displayed. |
| **Steps** | 1. Rotate (drag). 2. Zoom (scroll or pinch). 3. Pan (e.g. right-drag or modifier). 4. Use camera presets (top, front, side) if implemented. |
| **Expected result** | All controls work; no jank or crash; presets frame mesh. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 3: Mesh statistics (JR1-503)

| Field | Content |
|-------|---------|
| **Objective** | Verify vertex count and bounds displayed and accurate. |
| **Preconditions** | Mesh loaded in 3D preview. |
| **Steps** | 1. Read vertex count and bounds (min/max x, y, z or extent). 2. Compare to backend expectation (e.g. width×height for step=1; Z in 2–10mm). |
| **Expected result** | Stats match mesh; units (mm) clear. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 4: Performance — FPS 100K, 500K, 1M vertices (QA-604, JR1-504)

| Field | Content |
|-------|---------|
| **Objective** | Confirm 30+ FPS for 100K vertices; document FPS for 500K and 1M. |
| **Preconditions** | Test images/depth maps that produce ~100K, ~500K, ~1M vertices (or use backend test data). |
| **Steps** | 1. Load mesh with ~100K vertices; measure FPS (e.g. browser dev tools or in-app). 2. Repeat for ~500K and ~1M. 3. Record hardware (GPU, OS). |
| **Expected result** | ≥30 FPS for 100K (prd.md §7.1); results in MANUAL_TEST_REPORT. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 5: Integrated vs dedicated GPU (QA-602)

| Field | Content |
|-------|---------|
| **Objective** | Compare stability and FPS on integrated (e.g. Intel UHD) vs dedicated GPU. |
| **Preconditions** | Same mesh; two machines or switchable graphics. |
| **Steps** | 1. Run 3D preview on integrated GPU; note FPS and any glitches. 2. Run on dedicated GPU if available; compare. |
| **Expected result** | Both run without crash; differences documented in MANUAL_TEST_REPORT or GOTCHAS. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 6: Render mode toggle (UI-506)

| Field | Content |
|-------|---------|
| **Objective** | Points mode works; wireframe/solid show placeholder or message. |
| **Preconditions** | Mesh loaded in 3D preview. |
| **Steps** | 1. Select Points mode; confirm point cloud. 2. Select Wireframe (if enabled); confirm placeholder/message. 3. Select Solid (if enabled); same. |
| **Expected result** | Points renders; wireframe/solid do not crash; behavior documented. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

### 3.3 Regression / smoke

- [ ] App starts (`npm run tauri dev`)
- [ ] Load image → generate depth → adjust depth (Sprint 1.4/1.5) still works
- [ ] Mesh generation path (get_mesh_data) still works from backend
- [ ] No console errors on main screen before opening 3D preview
- [ ] No console errors when opening 3D preview and loading mesh

---

## 4. Artefacts and sign-off

| Artefact | Path | Owner |
|----------|------|-------|
| Manual test results | `SPRINTS/Sprint_1_7/MANUAL_TEST_REPORT.md` | Quality Engineer |
| Verification checklist | `SPRINTS/Sprint_1_7/VERIFICATION_CHECKLIST.md` | Sprint lead / Architect |
| Gotchas | `SPRINTS/Sprint_1_7/GOTCHAS.md` | Any agent; merge to `RESEARCH/GOTCHAS.md` |

**Process:** Complete manual tests → fill `MANUAL_TEST_REPORT.md` → run through `VERIFICATION_CHECKLIST.md` before marking sprint complete.

---

## 5. References

- **Task list:** `SPRINTS/Sprint_1_7/SPRINT_1_7_Task_Assignment.md`
- **PRD:** `prd.md` F1.4, §7.1 (performance)
- **Architecture:** `RESEARCH/architecture.md` (get_mesh_data, ADR-006, ADR-007)
- **Technology:** `RESEARCH/threejs.md`
- **Testing strategy:** `todo.md` § Testing Strategy

---

**Document Version:** 1.0  
**Template:** `SPRINTS/TEST_PLAN_TEMPLATE.md`
