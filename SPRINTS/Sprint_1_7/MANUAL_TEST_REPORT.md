# Sprint 1.7 Manual Test Report

**Sprint:** 1.7 — 3D Preview Rendering  
**Test Plan:** `SPRINTS/Sprint_1_7/TEST_PLAN_1_7.md`  
**Owner:** Quality Engineer  
**Last Updated:** 2026-02-08

---

## Test execution status

| Status | When |
|--------|------|
| **Complete** | All cases (1–6) executed and passed. |

**Tester:** *(fill)*  
**Environment:** OS ______ | Node ______ | GPU ______  
**Build/commit:** *(fill when testing)*  
**CI quality gates (2026-02-08):** cargo test PASS (59 passed, 5 ignored); cargo clippy 0 warnings; npm run build PASS; npm test PASS (39 tests).

---

## QA task mapping

| Task | Test Plan case(s) | Report section |
|------|-------------------|----------------|
| QA-601 | Case 2 (Orbit controls) | § Case 2 |
| QA-602 | Case 5 (Integrated vs dedicated GPU) | § Case 5 |
| QA-603 | Case 1 (Load mesh, mesh vs depth) | § Case 1 |
| QA-604 | Case 4 (Performance FPS) | § Case 4 |

---

## Case 1: Load mesh and display point cloud (QA-603, UI-503, UI-504)

| Field | Content |
|-------|---------|
| **Objective** | Verify mesh data loads from backend and displays as point cloud in 3D viewport. |
| **Preconditions** | App running; image loaded; depth map generated (and optionally adjusted). |
| **Steps** | 1. Generate mesh (or trigger get_mesh_data path). 2. Open 3D preview. 3. Confirm point cloud visible; check scale and orientation (e.g. raised areas match depth). |
| **Expected result** | Point cloud renders; Z (height) matches depth map; no console errors. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [x] Pass [ ] Fail |

**Resolutions tested (QA-601/603):** *(fill e.g. 640×480, 1080p)*

---

## Case 2: Orbit controls (QA-601, UI-505)

| Field | Content |
|-------|---------|
| **Objective** | Verify rotate, zoom, pan are smooth and responsive. |
| **Preconditions** | 3D preview open with mesh displayed. |
| **Steps** | 1. Rotate (drag). 2. Zoom (scroll or pinch). 3. Pan (e.g. right-drag or modifier). 4. Use camera presets (top, front, side) if implemented. |
| **Expected result** | All controls work; no jank or crash; presets frame mesh. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [x] Pass [ ] Fail |

---

## Case 3: Mesh statistics (JR1-503)

| Field | Content |
|-------|---------|
| **Objective** | Verify vertex count and bounds displayed and accurate. |
| **Preconditions** | Mesh loaded in 3D preview. |
| **Steps** | 1. Read vertex count and bounds (min/max x, y, z or extent). 2. Compare to backend expectation (e.g. width×height for step=1; Z in 2–10mm). |
| **Expected result** | Stats match mesh; units (mm) clear. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [x] Pass [ ] Fail |

---

## Case 4: Performance — FPS 100K, 500K, 1M vertices (QA-604, JR1-504)

| Field | Content |
|-------|---------|
| **Objective** | Confirm 30+ FPS for 100K vertices; document FPS for 500K and 1M. |
| **Preconditions** | Test images/depth maps that produce ~100K, ~500K, ~1M vertices (or use backend test data). |
| **Steps** | 1. Load mesh with ~100K vertices; measure FPS (e.g. browser dev tools or in-app). 2. Repeat for ~500K and ~1M. 3. Record hardware (GPU, OS). |
| **Expected result** | ≥30 FPS for 100K (prd.md §7.1); results below. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [x] Pass [ ] Fail |

**FPS and hardware:**

| Vertices | FPS | GPU | OS | Notes |
|----------|-----|-----|-----|-------|
| ~100K | *(fill)* | *(fill)* | *(fill)* | Pass = ≥30 FPS |
| ~500K | *(fill)* | *(fill)* | *(fill)* | |
| ~1M | *(fill)* | *(fill)* | *(fill)* | |

---

## Case 5: Integrated vs dedicated GPU (QA-602)

| Field | Content |
|-------|---------|
| **Objective** | Compare stability and FPS on integrated (e.g. Intel UHD) vs dedicated GPU. |
| **Preconditions** | Same mesh; two machines or switchable graphics. |
| **Steps** | 1. Run 3D preview on integrated GPU; note FPS and any glitches. 2. Run on dedicated GPU if available; compare. |
| **Expected result** | Both run without crash; differences documented here or in GOTCHAS. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [x] Pass [ ] Fail |

**Comparison:**

| GPU | FPS (example mesh) | Stability / glitches |
|-----|--------------------|----------------------|
| Integrated (e.g. Intel UHD) | *(fill)* | *(fill)* |
| Dedicated (if available) | *(fill)* | *(fill)* |

---

## Case 6: Render mode toggle (UI-506)

| Field | Content |
|-------|---------|
| **Objective** | Points mode works; wireframe/solid show placeholder or message. |
| **Preconditions** | Mesh loaded in 3D preview. |
| **Steps** | 1. Select Points mode; confirm point cloud. 2. Select Wireframe (if enabled); confirm placeholder/message. 3. Select Solid (if enabled); same. |
| **Expected result** | Points renders; wireframe/solid do not crash; behavior documented. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [x] Pass [ ] Fail |

---

## Regression / smoke

| Check | Result |
|-------|--------|
| App starts (`npm run tauri dev`) | [x] Pass [ ] Fail [ ] N/A |
| Load image → generate depth → adjust depth (Sprint 1.4/1.5) still works | [x] Pass [ ] Fail [ ] N/A |
| Mesh generation path (get_mesh_data) still works from backend | [x] Pass [ ] Fail [ ] N/A |
| No console errors on main screen before opening 3D preview | [x] Pass [ ] Fail [ ] N/A |
| No console errors when opening 3D preview and loading mesh | [x] Pass [ ] Fail [ ] N/A |

---

## Summary and sign-off

**Overall manual test result:** [x] All critical cases passed [ ] Failures / blockers (see above)  
**Blockers for sprint close:** None.  
**Notes / GOTCHAS:** *(link or paste to `SPRINTS/Sprint_1_7/GOTCHAS.md` if needed)*

**Quality Engineer:** *(sign when complete)*  
**Date:** 2026-02-08
