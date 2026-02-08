# Sprint Task Assignment — Sprint 1.7

**Use this template when creating sprint tasking from `todo.md`.**  
**Source:** `todo.md` — Sprint 1.7. Populated by System Architect & Senior Engineer.  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`

---

## Sprint 1.6 Status (Prerequisite)

**Validation:** See `SPRINTS/Sprint_1_6/SPRINT_1_6_STATUS_VALIDATION.md`.

- **Implementation:** Complete (mesh generation, `get_mesh_data`, tests, security sign-off).
- **QA:** QA-501–504 not yet executed; complete via Sprint 1.6A or in parallel with 1.7.
- **Sprint 1.7 may start** using existing `get_mesh_data` (JSON). BACK-601 adapts to ADR-007 if Sprint 1.6A chooses binary transfer.

---

## Sprint 1.7: 3D Preview Rendering

**Sprint Duration:** 2 weeks (Weeks 15–16)  
**Sprint Goal:** Display generated mesh in interactive 3D viewport.  
**Target Release:** —  
**Phase:** 1 (MVP)  
**Source:** `todo.md` — Sprint 1.7  
**Last Updated:** 2026-02-07

---

## Dependencies (from Sprint 1.6 / 1.6A)

- **IPC Transfer:** ADR-007 (Sprint 1.6A BACK-509/510) determines how mesh data flows to frontend. If latency >100ms for 1080p, binary transfer via temp file may be needed. Sprint 1.7 can proceed with current JSON; BACK-601 updates if binary chosen.
- **Mesh Format:** Point cloud only (ADR-006). Use `THREE.Points` or `THREE.BufferGeometry` with positions + normals from `get_mesh_data`. Triangulated faces not available until Sprint 1.8.
- **`get_mesh_data`:** Implemented in Sprint 1.6; BACK-601 adapts for ADR-007 if required.

---

## Proposed sequence for Sprint 1.7

Recommended order so dependencies are ready when needed. Parallel work is possible where no dependency exists.

| Order | Phase | Tasks | Rationale |
|-------|--------|--------|------------|
| 1 | Foundation | UI-501, UI-502 | Three.js in Svelte + scene/camera/lights/grid so the viewport exists. |
| 2 | Backend contract | BACK-601, BACK-602 | Ensure `get_mesh_data` and serialization are stable (or ADR-007 path) so frontend can rely on the contract. Can start in parallel with 1. |
| 3 | Load & render | UI-503, UI-504 | Load mesh from Rust and render as point cloud; depends on 1 and 2. |
| 4 | Controls & modes | UI-505, UI-506 | Orbit controls and render-mode toggle; depends on 3. |
| 5 | Enhancements | JR1-501, JR1-502, JR1-503, JR1-505 | Camera presets, grid refinement, mesh stats, lighting controls; can overlap with 4. |
| 6 | Performance | JR1-504, BACK-603 (optional) | Test large meshes; add LOD or tuning if needed. |
| 7 | QA | QA-601, QA-602, QA-603, QA-604 | **Manual testing during the sprint** (see below); run when 3–5 are testable; complete before sprint sign-off. |

**Test plan and manual testing:** The **Test Plan** is a separate artefact, `SPRINTS/Sprint_1_7/TEST_PLAN_1_7.md`. It is **part of Sprint 1.7**: it defines the manual test cases and automation scope for this sprint. The Quality Engineer tasks **QA-601–QA-604** are **inside** this task assignment and are performed **during** the sprint—**after** implementation is testable (roughly after order 4–5) and **before** the sprint is closed. Manual tests are **not** run before the sprint (that would be Sprint 1.6/1.6A) nor deferred to after the sprint; they are executed in the second half of the sprint and results recorded in `MANUAL_TEST_REPORT.md`, then the team runs through `VERIFICATION_CHECKLIST.md` before sign-off.

---

## Sprint Folder & Artefacts

**All sprint artefacts MUST be stored in this sprint's folder:**

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_1_7/SPRINT_1_7_Task_Assignment.md` | This document |
| 3D Preview API | `SPRINTS/Sprint_1_7/3D_PREVIEW_API.md` | Mesh data contract (get_mesh_data) for UI/BACK coordination |
| Test Plan | `SPRINTS/Sprint_1_7/TEST_PLAN_1_7.md` | QA test planning (manual + automated) |
| Progress Report | `SPRINTS/Sprint_1_7/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_1_7/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_1_7/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Gotchas Log | `SPRINTS/Sprint_1_7/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

---

## CRITICAL: Role Selection (READ FIRST — STOP HERE UNTIL COMPLETE)

**You are an unassigned agent. You MUST claim a role before proceeding.**

### Step 1: Review Available Roles
Look at the Role Assignment table below. Find a role where:
- Status = `Available`
- No agent is currently assigned

### Step 2: Claim Your Role
1. **Edit this document** to update that role's row:
   - Change Status from `Available` to `In Progress`
   - Add your session identifier to the "Assigned Agent" column
2. **Set your Cursor title to the role name.**
3. **Read the persona file** listed in the "Persona File" column
4. **Adopt that persona** for all remaining work

### Step 3: Become Your Role
- Embody the agent's identity, expertise, and responsibilities
- Follow the persona file's guidance and project references
- Rename the agent so that it shows the agent's identity in the agent list

### Step 4: Update status
- While progressing your role, update the status per the Status Values defined below.

---

## Roles required for this sprint

Roles that have **Owned Tasks** (non-empty) for Sprint 1.7. These must be staffed for the sprint to deliver.

| Role | Why required |
|------|--------------|
| Senior Engineer | BACK-601, BACK-602, BACK-603 — get_mesh_data for ADR-007, serialization, optional LOD |
| UI Designer | UI-501–UI-506 — Three.js integration, scene, mesh load, orbit controls, render modes |
| Junior Engineer 3D | JR1-501–JR1-505 — Camera presets, grid, mesh stats, performance, lighting |
| Quality Engineer | QA-601–QA-604 — Manual and performance testing; Test Plan and MANUAL_TEST_REPORT |

*All other roles in the Role Assignment table have "—" or "No 1.7 tasks" and are optional for this sprint.*

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | Complete | Cursor-Agent | — | 3D Preview API review done; 3D_PREVIEW_API.md created |
| Senior Engineer | `.agents/senior-engineer.md` | In Progress | Cursor-Agent | BACK-601, BACK-602, BACK-603 | get_mesh_data for ADR-007, serialization, optional LOD |
| UI Designer | `.agents/ui-designer.md` | In Progress | Cursor-Agent | UI-501–UI-506 | Three.js integration, scene, mesh load, orbit controls, render modes |
| Senior Researcher (AI/ML) | `.agents/researcher.md` | Available | - | — | No 1.7 tasks |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Available | - | — | No 1.7 tasks |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | In Progress | Cursor-Agent | JR1-501–JR1-505 | Camera presets, grid, mesh stats, performance, lighting |
| Security Specialist | `.agents/security-specialist.md` | Available | - | — | No 1.7 tasks |
| Documentation Specialist | `.agents/documentation-specialist.md` | Available | - | — | No 1.7 tasks |
| Quality Engineer | (todo.md) | In Progress | Cursor-Agent | QA-601–QA-604 | Manual + performance testing |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

*Note: Junior Engineer #1 (frontend) = Junior Engineer 3D; Quality Engineer has no separate persona — QA tasks in Task Breakdown.*

---

## Canonical References (Source of Truth)

- **Scope:** `prd.md` — F1.4 3D Preview, §5.2–5.3, §7.1 Performance
- **Sprint source:** `todo.md` — Sprint 1.7
- **Architecture:** `RESEARCH/architecture.md`, ADR-006 (mesh), ADR-007 (IPC when available)
- **3D Preview API (Sprint 1.7):** `SPRINTS/Sprint_1_7/3D_PREVIEW_API.md` — mesh data contract for UI-503/504 and BACK-601/602
- **Technology:** `RESEARCH/threejs.md` (BufferGeometry, Points, OrbitControls), `RESEARCH/frontend.md`
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`
- **Gotchas:** `RESEARCH/GOTCHAS.md`

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| UI (UI-501–506) | ✅ Complete | 100% |
| Junior 3D (JR1-501–505) | ✅ Complete | 100% |
| Backend (BACK-601–603) | ✅ Complete | 100% |
| Quality (QA-601–604) | ⏳ Not Started | 0% |

**Overall Sprint Progress:** [ ] Not Started / [x] In Progress / [ ] Complete

---

## Task Breakdown

*From todo.md Sprint 1.7. Map each task to a Role.*

### Phase 1: Frontend — Three.js Integration

#### Task UI-501: Integrate Three.js into Svelte component
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-501

**Dependencies:** None (foundation)

**What to Do:**
- Add Three.js (and addons e.g. OrbitControls) to the frontend; integrate in the existing preview/workspace Svelte component.
- Ensure build and dev workflow include Three.js (npm, Vite). Use RESEARCH/threejs.md for version and import paths.

**Reference Documents:** `prd.md` F1.4; `RESEARCH/threejs.md`; `RESEARCH/frontend.md`

**Acceptance Criteria:**
- [x] Three.js and required addons (e.g. OrbitControls) installed and importable
- [x] Preview component mounts a Three.js scene (scene, camera, renderer)
- [x] No console errors on load; render loop runs

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer - Cursor-Agent
Completed On: 2026-02-07
Notes: three@^0.170.0; OrbitControls from three/addons/controls/OrbitControls.js; Preview3D.svelte.
```

---

#### Task UI-502: Create 3D scene with camera, lights, grid
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-502

**Dependencies:** UI-501

**What to Do:**
- Create Scene, PerspectiveCamera, WebGLRenderer; add ambient (and optional directional) lights.
- Add grid floor for scale reference (or delegate to JR1-502). Handle resize; append renderer.domElement to DOM.

**Reference Documents:** `RESEARCH/threejs.md`; Three.js docs (Scene, Camera, Renderer)

**Acceptance Criteria:**
- [x] Scene renders with camera and lights
- [x] Grid or floor visible for scale
- [x] Resize handling works; no layout overflow

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer - Cursor-Agent
Completed On: 2026-02-07
Notes: Scene, PerspectiveCamera, WebGLRenderer, ambient + directional light, GridHelper; ResizeObserver.
```

---

#### Task UI-503: Load mesh data from Rust (via Tauri IPC — per ADR-007 transfer mechanism)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-503

**Dependencies:** BACK-601/BACK-602 (mesh data contract); Sprint 1.6 `get_mesh_data` exists

**What to Do:**
- Call Tauri command to get mesh data (invoke `get_mesh_data` or equivalent per ADR-007 — JSON or binary path).
- Parse response into positions (and normals) for BufferGeometry. Handle loading state and errors.

**Reference Documents:** `prd.md` F1.4; `RESEARCH/architecture.md` (get_mesh_data); ADR-007 when available

**Acceptance Criteria:**
- [x] Frontend requests mesh data from backend and receives positions (+ normals)
- [x] Loading and error states handled; no silent failures
- [x] Works with current JSON payload; adapts to binary if ADR-007 chooses it

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer - Cursor-Agent
Completed On: 2026-02-07
Notes: getMeshData() in tauri.ts; optional previewStep; Load mesh button; loading/error in Preview3D.
```

---

#### Task UI-504: Render point cloud (BufferGeometry with THREE.Points); mesh rendering deferred until Sprint 1.8
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-504

**Dependencies:** UI-503 (mesh data available)

**What to Do:**
- Build BufferGeometry from positions (and normals); create THREE.Points with PointsMaterial.
- Add to scene. Do not implement triangulated mesh rendering yet (wireframe/solid placeholders per UI-506).

**Reference Documents:** `RESEARCH/threejs.md` (BufferGeometry, Points); ADR-006 (point cloud only)

**Acceptance Criteria:**
- [x] Point cloud displays in 3D viewport from live mesh data
- [x] Vertex positions and scale match backend (mm)
- [x] Performance acceptable for 100K+ vertices (target 30+ FPS)

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer - Cursor-Agent
Completed On: 2026-02-07
Notes: BufferGeometry from positions.flat()/normals.flat(); THREE.Points + PointsMaterial; bounds fit.
```

---

#### Task UI-505: Implement orbit controls (rotate, zoom, pan)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-505

**Dependencies:** UI-501, UI-502

**What to Do:**
- Add OrbitControls (Three.js addon); attach to camera and renderer.domElement.
- Enable rotate, zoom, pan; optional damping. Call controls.update() in animation loop.

**Reference Documents:** `RESEARCH/threejs.md` (OrbitControls); Three.js examples

**Acceptance Criteria:**
- [x] User can orbit (rotate), zoom, and pan the view
- [x] Controls feel responsive; no jank during interaction
- [x] Works with point cloud in scene

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer - Cursor-Agent
Completed On: 2026-02-07
Notes: OrbitControls with damping; controls.update() in animation loop.
```

---

#### Task UI-506: Toggle wireframe/solid/point rendering modes (wireframe/solid require triangulated mesh — placeholder until 1.8)
**Assigned Role:** UI Designer  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** UI-506

**Dependencies:** UI-504 (point cloud visible)

**What to Do:**
- Add UI toggle or dropdown for render mode: Points (implement now), Wireframe (placeholder), Solid (placeholder).
- Points mode uses existing THREE.Points. Wireframe/Solid show message or same point cloud until Sprint 1.8 provides triangulated mesh.

**Reference Documents:** `prd.md` F1.4; `todo.md` Sprint 1.8 (triangulation)

**Acceptance Criteria:**
- [x] "Points" mode works (current point cloud)
- [x] Wireframe/Solid options present; behavior documented as placeholder until 1.8
- [x] No crash when switching modes

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer - Cursor-Agent
Completed On: 2026-02-07
Notes: Toolbar toggle Points / Wireframe / Solid; placeholder message for wireframe/solid until Sprint 1.8.
```

---

### Phase 2: Frontend — Junior Engineer 3D (Camera, Grid, Stats, Performance)

#### Task JR1-501: Add camera presets (top, front, side views)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR1-501

**Dependencies:** UI-502, UI-505 (camera and controls exist)

**What to Do:**
- Implement preset buttons or shortcuts: Top (look down), Front, Side. Set camera position and OrbitControls target to fit mesh bounds.

**Reference Documents:** `RESEARCH/threejs.md`; UX patterns for 3D viewports

**Acceptance Criteria:**
- [x] At least three presets (top, front, side) available
- [x] Camera frames mesh or scene appropriately
- [x] Presets documented or labeled in UI

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 3D - Cursor-Agent
Completed On: 2026-02-08
Notes: Preview3D.svelte: View group with Top, Front, Side buttons; setCameraPreset() uses mesh bounding box to position camera and controls.target; presets disabled when no mesh.
```

---

#### Task JR1-502: Implement grid floor for scale reference
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR1-502

**Dependencies:** UI-502 (scene exists)

**What to Do:**
- Add grid helper (e.g. GridHelper) or ground plane with grid texture. Scale in mm if possible; document in RESEARCH or GOTCHAS.

**Reference Documents:** `RESEARCH/threejs.md`; Three.js GridHelper / axes

**Acceptance Criteria:**
- [x] Grid visible in viewport; does not obscure mesh
- [x] Scale or spacing documented for users

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 3D - Cursor-Agent
Completed On: 2026-02-08
Notes: Grid already present (UI-502). Scale documented: 1 unit = 1 mm; grid 400 mm × 400 mm, 20 divisions (20 mm spacing). SPRINTS/Sprint_1_7/GOTCHAS.md and inline comment in Preview3D.svelte.
```

---

#### Task JR1-503: Display mesh statistics (vertex count, bounds)
**Assigned Role:** Junior Engineer 3D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR1-503

**Dependencies:** UI-503, UI-504 (mesh data loaded and displayed)

**What to Do:**
- Show vertex count and bounds (min/max x, y, z or extent) in UI — from backend response or computed on frontend. Place in sidebar or overlay.

**Reference Documents:** `prd.md` F1.4; backend MeshData contract

**Acceptance Criteria:**
- [x] Vertex count displayed and accurate
- [x] Bounds (or extent) displayed; units (mm) clear
- [x] Updates when new mesh is loaded

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 3D - Cursor-Agent
Completed On: 2026-02-08
Notes: Preview3D.svelte: meshStats computed from get_mesh_data positions (mm); toolbar shows Vertices (locale string) and Bounds (mm) X/Y/Z min–max; cleared when no mesh or on load error.
```

---

#### Task JR1-504: Test rendering performance with large meshes (>1M vertices)
**Assigned Role:** Junior Engineer 3D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR1-504

**Dependencies:** UI-504, BACK-603 (optional LOD)

**What to Do:**
- Test with 100K, 500K, 1M+ vertices. Measure FPS; document findings. If below 30 FPS for 100K, consider LOD or reduced point size (BACK-603 or frontend tuning).

**Reference Documents:** `prd.md` §7.1 (30+ FPS for 100K); `RESEARCH/threejs.md` (performance)

**Acceptance Criteria:**
- [x] Performance tested at 100K, 500K, 1M vertices
- [x] Results in GOTCHAS or PROGRESS_REPORT; 30+ FPS target for 100K noted
- [x] Any mitigation (LOD, culling) documented

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 3D - Cursor-Agent
Completed On: 2026-02-08
Notes: "Perf test" button in Preview3D runs synthetic point clouds (100K, 500K, 1M) for 3s each and displays FPS. Results documented in PROGRESS_REPORT. Mitigation: BACK-603 preview_step for reduced-detail preview.
```

---

#### Task JR1-505: Add lighting controls (ambient, directional)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Low  
**Status:** [x] Complete  
**Task ID:** JR1-505

**Dependencies:** UI-502 (lights in scene)

**What to Do:**
- Expose ambient and directional light intensity (and optional direction) via sliders or inputs. Update Three.js lights when values change.

**Reference Documents:** `RESEARCH/threejs.md`; Three.js lighting

**Acceptance Criteria:**
- [x] User can adjust ambient and directional intensity
- [x] Changes apply in real time; no reload required
- [x] Defaults provide readable preview

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 3D - Cursor-Agent
Completed On: 2026-02-08
Notes: Preview3D toolbar: Light section with Ambient and Directional sliders (0–2, step 0.1). Defaults 0.6 and 0.8. Reactive updates to light intensity in real time.
```

---

### Phase 3: Backend — Mesh Data for Frontend

#### Task BACK-601: Update get_mesh_data Tauri command for ADR-007 transfer mechanism
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-601

**Dependencies:** Sprint 1.6 `get_mesh_data` exists; Sprint 1.6A ADR-007 (if binary chosen)

**What to Do:**
- Keep current `get_mesh_data` (JSON) working. If Sprint 1.6A decides binary transfer (temp file or similar), implement that path: write mesh to temp file, return path or read handle to frontend; document in ADR-007.
- Ensure frontend contract (positions, normals, dimensions) is unchanged from caller’s perspective where possible.

**Reference Documents:** `RESEARCH/architecture.md`; ADR-007 (when available); `SPRINTS/Sprint_1_6A/` IPC benchmark

**Acceptance Criteria:**
- [x] Existing JSON `get_mesh_data` remains functional
- [x] If ADR-007 specifies binary: alternative path implemented and documented; frontend can use either path (documented in code + architecture; implementation deferred until ADR-007 decision)
- [x] No regression in mesh generation or Tauri permissions

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer - Cursor-Agent
Completed On: 2026-02-07
Notes: ADR-007 not yet decided. Code comment and RESEARCH/architecture.md § "Mesh data IPC contract" document current JSON path and placeholder for binary path when ADR-007 is adopted.
```

---

#### Task BACK-602: Serialize vertex array for frontend (JSON or binary — per ADR-007)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-602

**Dependencies:** BACK-601; MeshData from mesh_generator

**What to Do:**
- Ensure vertex array (positions, normals) is serialized for frontend consumption. Current: JSON. If ADR-007 chooses binary, add binary format (e.g. raw float array or documented layout) and document in architecture.

**Reference Documents:** `RESEARCH/architecture.md`; ADR-007; `RESEARCH/threejs.md` (BufferGeometry attribute layout)

**Acceptance Criteria:**
- [x] Frontend receives positions (and normals) in a format it can consume for BufferGeometry
- [x] Format documented; byte order and layout explicit if binary (JSON shape and BufferGeometry mapping in MeshData doc + architecture § "Mesh data IPC contract"; binary layout placeholder when ADR-007 adds it)
- [x] Performance acceptable for 100K–1M vertices (benchmark from 1.6A informs this)

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer - Cursor-Agent
Completed On: 2026-02-07
Notes: MeshData doc comment and RESEARCH/architecture.md document JSON shape and Three.js flattening; binary format to be documented when ADR-007 adopts binary transfer.
```

---

#### Task BACK-603: LOD (Level of Detail) for large meshes (optional)
**Assigned Role:** Senior Engineer  
**Priority:** Low  
**Status:** [x] Complete  
**Task ID:** BACK-603

**Dependencies:** BACK-501–502 (mesh generation with step); JR1-504 (performance findings)

**What to Do:**
- If performance requires it: support reduced vertex count for preview (e.g. step_x/step_y or max vertices). Expose as parameter or automatic based on viewport/resolution. Document in architecture or GOTCHAS.

**Reference Documents:** `prd.md` §7.1; `RESEARCH/architecture.md`; mesh_generator MeshParams

**Acceptance Criteria:**
- [x] If implemented: frontend or backend can request reduced-detail mesh for preview (optional `preview_step` on `get_mesh_data`)
- [x] No regression in full-detail export path (Sprint 1.8) — full detail when `preview_step` is omitted
- [x] Decision (implement vs defer) documented (SPRINTS/Sprint_1_7/GOTCHAS.md; architecture § Mesh data IPC contract)

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer - Cursor-Agent
Completed On: 2026-02-07
Notes: Optional get_mesh_data(preview_step?: number); step applied to MeshParams. Full-detail path unchanged. Decision in GOTCHAS.
```

---

### Phase 4: Quality

#### Task QA-601: Manual test — rotate/zoom mesh, verify responsiveness
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-601

**Dependencies:** UI-501–506, JR1-501 (controls and presets)

**What to Do:**
- Manual test: load image → generate depth → generate mesh → open 3D preview. Rotate, zoom, pan; use presets. Verify smooth response and no crashes. Record in MANUAL_TEST_REPORT.

**Reference Documents:** `SPRINTS/Sprint_1_7/TEST_PLAN_1_7.md`; `prd.md` F1.4

**Acceptance Criteria:**
- [ ] Test procedure in test plan or manual report
- [ ] At least two resolutions tested (e.g. 640×480, 1080p)
- [ ] Result in `SPRINTS/Sprint_1_7/MANUAL_TEST_REPORT.md`

**Completion Record:**
```
Status: [ ] Complete
Completed By: Quality Engineer - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task QA-602: Test on integrated GPU (Intel UHD) vs dedicated GPU
**Assigned Role:** Quality Engineer  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-602

**Dependencies:** UI-504 (point cloud rendering)

**What to Do:**
- Run 3D preview on machine with integrated GPU and (if available) dedicated GPU. Compare FPS and stability; document in MANUAL_TEST_REPORT or GOTCHAS.

**Reference Documents:** `prd.md` §7.1; RESEARCH/threejs.md

**Acceptance Criteria:**
- [ ] At least one test on integrated GPU; result documented
- [ ] Any significant difference or limitation noted
- [ ] Result in sprint folder

**Completion Record:**
```
Status: [ ] Complete
Completed By: Quality Engineer - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task QA-603: Verify mesh matches depth map (visual inspection)
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-603

**Dependencies:** UI-504, BACK-602 (mesh data displayed)

**What to Do:**
- For a known image and depth map, generate mesh and preview. Visually confirm that raised areas in 3D correspond to depth map (e.g. closer = higher Z). Document in MANUAL_TEST_REPORT.

**Reference Documents:** `prd.md` F1.4, F1.5; depth range 2–10mm

**Acceptance Criteria:**
- [ ] At least one image/depth pair verified visually
- [ ] Z orientation and scale (mm) consistent with expectations
- [ ] Result in MANUAL_TEST_REPORT

**Completion Record:**
```
Status: [ ] Complete
Completed By: Quality Engineer - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task QA-604: Performance test — FPS with 100K, 500K, 1M vertices
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-604

**Dependencies:** JR1-504; UI-504; BACK-602

**What to Do:**
- Measure FPS for 100K, 500K, and 1M vertex point clouds on reference hardware. Target: 30+ FPS for 100K per PRD. Document hardware and results in MANUAL_TEST_REPORT or PROGRESS_REPORT.

**Reference Documents:** `prd.md` §7.1; TEST_PLAN_1_7

**Acceptance Criteria:**
- [ ] FPS recorded for 100K, 500K, 1M (or max available)
- [ ] Pass/fail vs 30 FPS for 100K; hardware noted
- [ ] Result in sprint folder

**Completion Record:**
```
Status: [ ] Complete
Completed By: Quality Engineer - [Agent ID]
Completed On: [Date]
Notes:
```

---

## Subtask Allocation (for multi-role tasks)

| Sub-task | Role | Owner | Status |
|----------|------|-------|--------|
| UI-503 + BACK-601/602 | UI Designer + Senior Engineer | (when claimed) | Use `3D_PREVIEW_API.md` for mesh data contract; coordinate on ADR-007 if binary path added |
| JR1-502 + UI-502 | Junior Engineer 3D + UI Designer | (when claimed) | Grid can be in UI-502 or JR1-502 per team choice |

---

## Success Criteria for Sprint

- [ ] All tasks complete per acceptance criteria
- [ ] Exit criteria from todo.md met:
  - 3D preview displays mesh correctly
  - Orbit controls smooth and responsive
  - Render modes (wireframe, solid, points) functional (points required; wireframe/solid placeholder OK)
  - Performance target: 30+ FPS for 100K vertices
  - Mesh statistics displayed accurately
- [ ] No blocking issues
- [ ] Gotchas recorded in `SPRINTS/Sprint_1_7/GOTCHAS.md` (merge to RESEARCH when done)
- [ ] Progress report filed

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| (none at sprint start) | — | — |

*If ADR-007 is not decided by Sprint 1.6A, Sprint 1.7 proceeds with JSON; BACK-601 adapts when ADR-007 is done.*

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | — |
| cargo clippy | 0 warnings | — |
| npm run build | PASS | — |
| npm test | PASS | — |
| 3D preview FPS (100K vertices) | ≥30 | — |

---

## Progress Log (Handover Notes)

*Agents add handover notes when completing tasks that others depend on.*

```
### 2026-02-07 — Senior Engineer (BACK-601, BACK-602, BACK-603 COMPLETED)
- BACK-601: get_mesh_data remains JSON; code comment and RESEARCH/architecture.md § "Mesh data IPC contract" document ADR-007 placeholder (binary path when 1.6A decides).
- BACK-602: MeshData serialization documented in mesh_generator.rs (MeshData doc comment) and architecture.md. Frontend: positions/normals as array of [x,y,z]; flatten for BufferGeometry (e.g. positions.flat() → Float32Array).
- BACK-603: Optional get_mesh_data(preview_step?: number). When omitted, step=1 (full detail). When provided (e.g. 2), mesh uses step_x=step_y=step for reduced-detail preview. Full-detail export path unchanged. Decision in SPRINTS/Sprint_1_7/GOTCHAS.md.
- Key files: src-tauri/src/lib.rs, src-tauri/src/mesh_generator.rs, RESEARCH/architecture.md.
- Handover to: UI Designer (UI-503 can call get_mesh_data with or without previewStep); Junior Engineer 3D (JR1-504 can use previewStep for performance testing).

### [Date] — [Role] (Task X.Y COMPLETED)
[What was delivered. Key files. Gotchas. Handover to whom.]
```

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **prd.md** — F1.4 3D Preview, §5.2–5.3, §7.1
3. **todo.md** — Sprint 1.7 full context
4. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
5. **RESEARCH/threejs.md** — BufferGeometry, Points, OrbitControls
6. **RESEARCH/architecture.md** — get_mesh_data, ADR-006, ADR-007 (when available)
7. **RESEARCH/GOTCHAS.md** — Known pitfalls
8. **SPRINTS/Sprint_1_6/SPRINT_1_6_STATUS_VALIDATION.md** — Prerequisite status

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Status:** Ready for team; roles available for claim.
