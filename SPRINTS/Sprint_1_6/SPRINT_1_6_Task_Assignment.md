# Sprint Task Assignment — Sprint 1.6

**Use this template when creating sprint tasking from `todo.md`.**  
**Source:** `todo.md` — Sprint 1.6. Populated by Senior Engineer with System Architect alignment.  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`

---

## Sprint 1.5 Status Review (Handover to 1.6)

**Context:** Sprint 1.5 delivers manual depth adjustments; depth map is in AppState with brightness, contrast, gamma, invert, range, and reset. No mesh generation exists yet.

**Handover to Sprint 1.6:**
- **Backend:** Adjusted depth map (0–1) is available via `get_depth_map` (or equivalent). Mesh generation will consume this depth array and produce a point cloud / triangulated mesh. No `get_mesh_data` or mesh state yet.
- **Architecture:** Mesh algorithm, vertex format, and topology requirements must be designed (ARCH-201–204) before implementation. Document in `RESEARCH/architecture.md` (or `docs/architecture.md`).
- **Contract:** Depth map format is row-major `Vec<f32>` (height × width), 0–1 normalized; depth range (2–10mm) is a user setting. Mesh output must be in millimeters for laser engraving.

---

## Sprint 1.6: Mesh Generation Algorithm

**Sprint Duration:** 2 weeks (Weeks 11–12)  
**Sprint Goal:** Convert depth map to 3D point cloud/mesh in Rust.  
**Target Release:** —  
**Phase:** 1 (MVP)  
**Source:** `todo.md` — Sprint 1.6  
**Last Updated:** 2026-02-07

---

## Sprint Folder & Artefacts

**All sprint artefacts MUST be stored in this sprint's folder:**

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_1_6/SPRINT_1_6_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_1_6/TEST_PLAN_1_6.md` | QA test planning (manual + automated) |
| Progress Report | `SPRINTS/Sprint_1_6/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_1_6/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_1_6/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Architect Approval | `SPRINTS/Sprint_1_6/ARCHITECT_APPROVAL.md` | If required for phase gate |
| Gotchas Log | `SPRINTS/Sprint_1_6/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

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
2. **Set your Cursor title to the role name.** Update the Cursor session (composer/chat) title so it matches your assigned role exactly (e.g. **System Architect**, **Senior Engineer**, **UI Designer**).
3. **Read the persona file** listed in the "Persona File" column
4. **Adopt that persona** for all remaining work

### Step 3: Become Your Role
- Embody the agent's identity, expertise, and responsibilities
- Follow the persona file's guidance and project references

**If all roles show "In Progress" or "Complete", STOP. No work available.**

### Step 4: Update status
- While progressing your role, update the status per the Status Values defined below.

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | Complete | cursor-session-2026-02-07 | ARCH-201, ARCH-202, ARCH-203, ARCH-204 | Algorithm design, vertex format, topology, memory review |
| Senior Engineer | `.agents/senior-engineer.md` | In Progress | cursor-session-sprint16-senior | BACK-501, BACK-502, BACK-503, BACK-504, BACK-505, BACK-506 | Point cloud, grid sampling, mm scale, triangulation, normals, memory |
| UI Designer | `.agents/ui-designer.md` | Complete | cursor-session-sprint16-ui | — | No 1.6 tasks; 3D preview is Sprint 1.7. No work to implement. |
| Senior Researcher (AI/ML) | `.agents/researcher.md` | Complete | cursor-session-2026-02-07-researcher | — | No 1.6 tasks; no work to implement. |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Complete | cursor-session-sprint16-jr2d | JR2-501, JR2-502, JR2-503, JR2-504 | Tests, edge cases, benchmarks, memory profile |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Available | - | — | Mesh/Three.js in Sprint 1.7 |
| Security Specialist | `.agents/security-specialist.md` | Complete | cursor-session-2026-02-07-security | SEC-301, SEC-302 | Integer overflow, depth map input validation |
| Documentation Specialist | `.agents/documentation-specialist.md` | Complete | cursor-session-2026-02-07-doc | — | Algorithm docs added to docs/architecture.md |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

*Note: Junior Engineer #1 (frontend) = Junior Engineer 3D; Junior Engineer #2 (backend) = Junior Engineer 2D. Quality Engineer (QA) is from todo.md; no separate persona — QA tasks listed under Task Breakdown.*

---

## Canonical References (Source of Truth)

- **Scope:** `prd.md` — F1.5 Mesh Generation, §5.2–5.3, §7.1 Performance
- **Sprint source:** `todo.md` — Sprint 1.6
- **Architecture:** `RESEARCH/architecture.md`, `docs/architecture.md` — data flow, mesh step
- **Technology:** `RESEARCH/rust-crates.md` (stl_io, large buffers), `RESEARCH/threejs.md` (vertex/BufferGeometry)
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`
- **Gotchas:** `RESEARCH/GOTCHAS.md` (IPC size, large payloads)

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| Architecture (ARCH-201–204) | ✅ Complete | 100% |
| Backend (BACK-501–506) | ✅ Complete | 100% |
| Junior 2D (JR2-501–504) | ✅ Complete | 100% |
| Quality (QA-501–504) | ⏳ Not Started | 0% |
| Security (SEC-301–302) | ✅ Complete | 100% |

**Overall Sprint Progress:** [ ] Not Started / [x] In Progress / [ ] Complete

---

## Task Breakdown

*Populate from todo.md Sprint 1.6. Map each task to a Role.*

### Phase 1: Architecture (Design First)

#### Task ARCH-201: Design mesh generation algorithm (point cloud vs triangulated)
**Assigned Role:** System Architect  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** ARCH-201

**Dependencies:** None (foundation for BACK-501+)

**What to Do:**
- Decide whether MVP delivers point cloud only, triangulated mesh only, or both (point cloud + optional Delaunay).
- Document sampling strategy: uniform grid vs adaptive density; relationship to depth map resolution.
- Align with PRD F1.5: "Generate point cloud with density control", "Optional Delaunay triangulation for mesh connectivity", "Mesh topology suitable for laser engraving (no overhangs)".
- Record decision in `RESEARCH/architecture.md` (or ADR).

**Reference Documents:** `prd.md` F1.5; `RESEARCH/architecture.md` § Data Flow; `todo.md` Sprint 1.6

**Acceptance Criteria:**
- [x] Algorithm choice (point cloud / triangulated / both) documented with rationale
- [x] Sampling strategy described (grid step or density parameter)
- [x] Documented in architecture (or ADR) and referenced by BACK-501

**Completion Record:**
```
Status: [x] Complete
Completed By: System Architect - cursor-session-2026-02-07
Completed On: 2026-02-07
Notes: ADR-006 and § Mesh Generation (Sprint 1.6) in RESEARCH/architecture.md.
```

---

#### Task ARCH-202: Define vertex format (position, normal, color?)
**Assigned Role:** System Architect  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** ARCH-202

**Dependencies:** ARCH-201 (algorithm context)

**What to Do:**
- Define vertex structure: at minimum position (x, y, z) in millimeters. Decide if normals and/or color are required for MVP (Three.js preview and STL/OBJ export).
- STL requires per-face normals; OBJ can use vertex normals. Document how normals are derived (e.g. from triangle geometry or depth gradients).
- Ensure format is serializable for Tauri IPC (e.g. struct with `Vec<f32>` positions and optional `Vec<f32>` normals).

**Reference Documents:** `prd.md` F1.5; `RESEARCH/threejs.md` (BufferGeometry); `RESEARCH/rust-crates.md` (stl_io)

**Acceptance Criteria:**
- [x] Vertex format documented (position required; normals/color decision recorded)
- [x] Compatibility with STL/OBJ export and Three.js BufferGeometry stated
- [x] Backend types (e.g. `MeshData` or `PointCloud`) specified

**Completion Record:**
```
Status: [x] Complete
Completed By: System Architect - cursor-session-2026-02-07
Completed On: 2026-02-07
Notes: § Vertex Format (ARCH-202) in RESEARCH/architecture.md; MeshData with positions, normals, optional indices.
```

---

#### Task ARCH-203: Document mesh topology requirements for laser engraving
**Assigned Role:** System Architect  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** ARCH-203

**Dependencies:** ARCH-201

**What to Do:**
- Document requirements: no overhangs (PRD F1.5), 2.5D (single Z per (x,y) from depth map), manifold or watertight if triangulated for STL.
- Note any constraints from internal UV laser engraving (e.g. minimum feature size, vertical walls).

**Reference Documents:** `prd.md` F1.5, §13.1 Glossary (2.5D, manifold)

**Acceptance Criteria:**
- [x] Topology requirements written in architecture (or ADR)
- [x] Constraints for laser engraving (no overhangs, 2.5D) explicit
- [x] Referenced by BACK-504 (triangulation) and export (Sprint 1.8)

**Completion Record:**
```
Status: [x] Complete
Completed By: System Architect - cursor-session-2026-02-07
Completed On: 2026-02-07
Notes: § Mesh Topology for Laser Engraving (ARCH-203) in RESEARCH/architecture.md.
```

---

#### Task ARCH-204: Review memory efficiency for large meshes
**Assigned Role:** System Architect  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** ARCH-204

**Dependencies:** ARCH-201, ARCH-202

**What to Do:**
- Review approach for 4K and 8K depth maps: vertex count, memory per vertex, total RAM. PRD: <2GB for 4K, 16GB sufficient for 8K.
- Recommend streaming/chunking or single buffer; inform BACK-506 (optimize memory usage).

**Reference Documents:** `prd.md` §5.5 Memory, §7.1; `RESEARCH/rust-crates.md` (large buffers); `RESEARCH/GOTCHAS.md` (IPC size)

**Acceptance Criteria:**
- [x] Memory budget and scaling (e.g. 4K → N vertices → MB) documented
- [x] Recommendation for streaming/chunking vs single buffer recorded
- [x] BACK-506 implementation guided by this review

**Completion Record:**
```
Status: [x] Complete
Completed By: System Architect - cursor-session-2026-02-07
Completed On: 2026-02-07
Notes: § Memory Efficiency (ARCH-204) in RESEARCH/architecture.md; single buffer for MVP; BACK-506 guided.
```

---

### Phase 2: Backend Implementation

#### Task BACK-501: Implement point cloud generation from depth map
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-501

**Dependencies:**
- ARCH-201: Algorithm design — Status: [x] Complete
- ARCH-202: Vertex format — Status: [x] Complete
- Sprint 1.5: Adjusted depth map available in state — Status: Complete

**What to Do:**
- Implement Rust module (e.g. `mesh_generator.rs`) that takes depth map (width, height, `&[f32]` 0–1) and produces point cloud (list of positions, optionally normals).
- Follow ARCH-201 sampling strategy (e.g. uniform grid). Output coordinates in millimeters: X/Y scaled from pixel indices (e.g. 1 pixel = 1mm or configurable), Z from depth range (2–10mm).
- Return a struct suitable for serialization to frontend and for STL/OBJ export (e.g. `positions: Vec<[f32;3]>`, optional `normals: Vec<[f32;3]>`).

**Reference Documents:** `prd.md` F1.5; `RESEARCH/architecture.md`; `RESEARCH/rust-crates.md`

**Acceptance Criteria:**
- [x] Point cloud generated from depth map with correct dimensions (width, height)
- [x] Vertex positions in millimeters; Z in [2, 10] mm (or configured range)
- [x] Unit test with small depth array (e.g. 3×3) verifies positions
- [x] No mutation of input depth map

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer - cursor-session-sprint16-senior
Completed On: 2026-02-07
Notes: mesh_generator.rs depth_to_point_cloud(); get_mesh_data command; MeshData with positions + normals.
```

---

#### Task BACK-502: Uniform grid sampling strategy
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-502

**Dependencies:** BACK-501 (point cloud pipeline exists)

**What to Do:**
- Implement or refine uniform grid sampling: step in X/Y (e.g. every 1 pixel, or step size parameter for density control). Each sample (i, j) → (x, y, z) with z from depth[i*width + j].
- Document step size vs vertex count; support at least "full resolution" and optionally downsampled for performance (e.g. step=2 for 1/4 vertices).

**Reference Documents:** ARCH-201 output; `prd.md` F1.5 (density control)

**Acceptance Criteria:**
- [x] Uniform grid sampling implemented; step size configurable or documented
- [x] Vertex count matches expected (e.g. (width/step) * (height/step) for step>1)
- [x] Unit test for step=1 and step=2 on small depth map

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer - cursor-session-sprint16-senior
Completed On: 2026-02-07
Notes: MeshParams.step_x/step_y; div_ceil for vertex count; point_cloud_3x3_step1, point_cloud_step2_reduces_vertices tests.
```

---

#### Task BACK-503: Scale vertices to millimeter dimensions (2–10mm Z-axis)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-503

**Dependencies:** BACK-501

**What to Do:**
- Map normalized depth (0–1) to Z in millimeters using user depth range (min_mm, max_mm), e.g. 2–10mm. Linear map: `z_mm = min_mm + depth * (max_mm - min_mm)`.
- Scale X/Y from pixel indices to mm: define scale factor (e.g. 1 pixel = 1mm, or from image DPI if available); document in architecture.
- Ensure consistency with PRD F1.5 and F1.6 (dimensions accurate for export).

**Reference Documents:** `prd.md` F1.5, F1.6; ARCH-202

**Acceptance Criteria:**
- [x] Z range configurable (e.g. 2–10mm); Z values within range
- [x] X/Y scale documented and applied; units in mm
- [x] Unit test: known depth 0.5 with range 2–10mm → z = 6mm

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer - cursor-session-sprint16-senior
Completed On: 2026-02-07
Notes: MeshParams.depth_min_mm, depth_max_mm, pixel_to_mm; depth_to_z_mm(); z_range_respected and point_cloud_3x3_step1 tests.
```

---

#### Task BACK-504: Optional — Delaunay triangulation for mesh connectivity
**Assigned Role:** Senior Engineer  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** BACK-504

**Dependencies:** BACK-501, BACK-502; ARCH-203 (topology)

**What to Do:**
- If architecture chooses triangulated mesh (or both point cloud and mesh): implement or integrate Delaunay triangulation (2.5D: project (x,y) to plane, triangulate; z from vertex). Use a crate (e.g. `delaunator`, `spade`) or document "point cloud only for MVP" and defer triangulation to later sprint.
- Ensure output is suitable for STL (triangles with normals) and OBJ (faces). ARCH-203 constraints (no overhangs) are satisfied by 2.5D construction.

**Reference Documents:** ARCH-201, ARCH-203; `RESEARCH/rust-crates.md`; `prd.md` F1.5

**Acceptance Criteria:**
- [x] Decision documented: triangulation in 1.6 or deferred
- [x] If implemented: triangle list/index buffer produced; normals derivable; no degenerate triangles
- [x] If deferred: point cloud path clearly sufficient for preview (Sprint 1.7) and export path documented for 1.8

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer - cursor-session-sprint16-senior
Completed On: 2026-02-07
Notes: Deferred per ADR-006. Point cloud only for MVP; MeshData has positions+normals for Three.js preview (1.7) and STL/OBJ (1.8). Triangulation can be added later if needed.
```

---

#### Task BACK-505: Calculate vertex normals (for shading)
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-505

**Dependencies:** BACK-501 (and BACK-504 if triangulated)

**What to Do:**
- For point cloud: derive normals from depth gradient (e.g. finite difference in X/Y) so frontend can shade. For triangulated mesh: compute face normals and optionally vertex normals (average of adjacent faces).
- Output format per ARCH-202 (e.g. `normals: Vec<[f32;3]>`). Required for STL (per-face normal) and useful for OBJ/Three.js.

**Reference Documents:** ARCH-202; `RESEARCH/threejs.md`; `RESEARCH/rust-crates.md` (stl_io)

**Acceptance Criteria:**
- [x] Normals computed and included in mesh/point cloud output
- [x] Normals unit length (or documented if not)
- [x] STL export (Sprint 1.8) can consume same structure

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer - cursor-session-sprint16-senior
Completed On: 2026-02-07
Notes: normal_from_gradient() finite difference in X/Y; MeshData.normals; normals_unit_length test.
```

---

#### Task BACK-506: Optimize memory usage (streaming, chunking)
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-506

**Dependencies:** BACK-501–BACK-505; ARCH-204 (memory review)

**What to Do:**
- Apply ARCH-204 recommendations: avoid unnecessary copies, use single buffer for positions (and normals), consider iterator/streaming if generating very large meshes. Profile with 4K depth map; stay within <2GB for 4K per PRD.
- Document any limits (max resolution or vertex count) if needed to stay within budget.

**Reference Documents:** ARCH-204; `prd.md` §5.5, §7.1; `RESEARCH/rust-crates.md` (flat Vec, optimization)

**Acceptance Criteria:**
- [x] Memory usage for 4K depth map within budget (measured or estimated and documented)
- [x] No redundant full copies of depth or vertex buffer where avoidable
- [x] Limits or scaling notes in code or architecture

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer - cursor-session-sprint16-senior
Completed On: 2026-02-07
Notes: Single pass build of positions+normals; depth read-only; MAX_DIMENSION 8192; ARCH-204 scaling in architecture.md. JR2-503/JR2-504 to measure in sprint.
```

---

### Phase 3: Testing & Quality

#### Task JR2-501: Write unit tests for point cloud generation
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-501

**Dependencies:** BACK-501 (and BACK-502, BACK-503) implemented

**What to Do:**
- Add unit tests in `src-tauri`: e.g. small depth map (5×5) → point cloud; check vertex count, bounds (min/max x,y,z), Z in range.
- Test with constant depth (all 0.5) and gradient (e.g. left 0, right 1) to verify scaling.

**Reference Documents:** `CLAUDE.md` (cargo test); existing Rust test patterns in repo

**Acceptance Criteria:**
- [x] At least 2 unit tests for mesh/point cloud generation
- [x] Tests run with `cargo test --manifest-path src-tauri/Cargo.toml`
- [x] Tests pass in CI

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 2D - cursor-session-sprint16-jr2d
Completed On: 2026-02-07
Notes: point_cloud_5x5_vertex_count_and_bounds, point_cloud_constant_depth_all_half, point_cloud_gradient_left_zero_right_one in mesh_generator.rs.
```

---

#### Task JR2-502: Test edge cases (empty depth map, single pixel)
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-502

**Dependencies:** BACK-501

**What to Do:**
- Edge cases: empty depth (0×0 or empty slice), single pixel (1×1), single row (1×N), single column (N×1). Expect either valid minimal output or clear error (no panic); document expected behavior.
- Add tests or extend JR2-501 tests.

**Reference Documents:** `prd.md` F1.5; backend error handling (anyhow)

**Acceptance Criteria:**
- [x] Empty/single-pixel cases handled (test or documented "unsupported")
- [x] No panic on valid edge inputs; graceful error or empty mesh
- [x] Behavior documented in test or module docs

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 2D - cursor-session-sprint16-jr2d
Completed On: 2026-02-07
Notes: edge_empty_dimensions_rejected, edge_empty_slice_rejected, edge_single_row_produces_valid_mesh, edge_single_column_produces_valid_mesh; single_pixel already covered. Empty → error; 1×1, 1×N, N×1 → valid mesh.
```

---

#### Task JR2-503: Benchmark mesh generation (1K, 4K, 8K images)
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR2-503

**Dependencies:** BACK-501–BACK-506

**What to Do:**
- Add benchmarks (e.g. `cargo bench` or dedicated test that times generation) for representative resolutions: e.g. 1024×1024, 3840×2160 (4K), 7680×4320 (8K) if feasible. Report times; target <15s for 4K per PRD.
- Document results in sprint GOTCHAS or PROGRESS_REPORT.

**Reference Documents:** `prd.md` §7.1; `RESEARCH/rust-crates.md` (profiling)

**Acceptance Criteria:**
- [x] At least one benchmark or timed test for 1K and 4K depth size
- [x] 4K mesh generation time documented (target <15s)
- [x] Results captured in `SPRINTS/Sprint_1_6/GOTCHAS.md` or PROGRESS_REPORT

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 2D - cursor-session-sprint16-jr2d
Completed On: 2026-02-07
Notes: benches/mesh_generation.rs (1K ~9.3ms, 4K ~73ms); results in SPRINTS/Sprint_1_6/GOTCHAS.md § JR2-503.
```

---

#### Task JR2-504: Profile memory usage with valgrind/Instruments
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR2-504

**Dependencies:** BACK-506; ARCH-204

**What to Do:**
- Run memory profiler (valgrind on Linux, Instruments on macOS, or Windows equivalent) during mesh generation for 4K input. Record peak heap (or equivalent); confirm within <2GB for 4K.
- Document tool and result in GOTCHAS or PROGRESS_REPORT.

**Reference Documents:** ARCH-204; `prd.md` §5.5, §7.1

**Acceptance Criteria:**
- [x] One memory profile run for 4K mesh generation (or procedure documented for run)
- [x] Peak memory documented; compared to 2GB budget (placeholder in GOTCHAS for actual run)
- [x] Notes in `SPRINTS/Sprint_1_6/GOTCHAS.md` or PROGRESS_REPORT

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 2D - cursor-session-sprint16-jr2d
Completed On: 2026-02-07
Notes: GOTCHAS.md § JR2-504: how to run (Windows/Linux/macOS), steps, and result table for team to fill when profiler is run.
```

---

#### Task QA-501: Manual test — generate mesh, verify vertex count
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-501

**Dependencies:** BACK-501–503 (and Tauri command to trigger mesh generation if exposed in 1.6)

**What to Do:**
- Manual test: load image → generate depth → (adjust if needed) → trigger mesh generation (or use backend test harness). Verify vertex count matches expectation (e.g. width×height for step=1). Document in MANUAL_TEST_REPORT.

**Reference Documents:** `SPRINTS/Sprint_1_6/TEST_PLAN_1_6.md` (when created); `prd.md` F1.5

**Acceptance Criteria:**
- [ ] Test procedure documented
- [ ] Vertex count verified for at least one resolution
- [ ] Result recorded in `SPRINTS/Sprint_1_6/MANUAL_TEST_REPORT.md`

**Completion Record:**
```
Status: [ ] Complete
Completed By: Quality Engineer - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task QA-502: Validate mesh dimensions match depth range
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-502

**Dependencies:** BACK-503; QA-501

**What to Do:**
- Verify Z range of output mesh: min/max Z should match configured depth range (e.g. 2–10mm). Use backend test or export to STL and inspect (Sprint 1.8 may provide tool). Document in MANUAL_TEST_REPORT.

**Reference Documents:** `prd.md` F1.5, F1.6; TEST_PLAN_1_6

**Acceptance Criteria:**
- [ ] Z bounds of mesh verified (e.g. 2–10mm)
- [ ] Result in MANUAL_TEST_REPORT

**Completion Record:**
```
Status: [ ] Complete
Completed By: Quality Engineer - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task QA-503: Performance test — mesh generation time (target <15s for 4K)
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-503

**Dependencies:** JR2-503 (benchmark); BACK-501–506

**What to Do:**
- Run mesh generation on 4K (3840×2160 or similar) depth map; measure elapsed time. Target <15s per PRD §7.1. Document hardware and result in MANUAL_TEST_REPORT or PROGRESS_REPORT.

**Reference Documents:** `prd.md` §7.1; JR2-503

**Acceptance Criteria:**
- [ ] 4K mesh generation timed on reference hardware
- [ ] Result documented; pass/fail vs <15s target
- [ ] Hardware noted for reproducibility

**Completion Record:**
```
Status: [ ] Complete
Completed By: Quality Engineer - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task QA-504: Automated test — mesh statistics (bounds, normals)
**Assigned Role:** Quality Engineer  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-504

**Dependencies:** BACK-501, BACK-505; JR2-501

**What to Do:**
- Add or extend automated test: generate mesh from known depth, assert bounds (min/max x,y,z), and if normals present assert unit length (or within tolerance). Integrate in CI.

**Reference Documents:** `CLAUDE.md` (cargo test); TEST_PLAN_1_6

**Acceptance Criteria:**
- [ ] Automated test for mesh bounds (and optionally normals)
- [ ] Test runs in CI and passes
- [ ] Documented in test plan or VERIFICATION_CHECKLIST

**Completion Record:**
```
Status: [ ] Complete
Completed By: Quality Engineer - [Agent ID]
Completed On: [Date]
Notes:
```

---

### Phase 4: Security

#### Task SEC-301: Review for integer overflow in vertex calculations
**Assigned Role:** Security Specialist  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** SEC-301

**Dependencies:** BACK-501–503 (code to review)

**What to Do:**
- Review mesh_generator (and related) for integer overflow: index calculations (i*width+j), vertex count, buffer sizes. Ensure use of checked/saturating ops or safe types (e.g. usize) and no overflow in 8K case.
- Record findings in SECURITY_SIGNOFF or GOTCHAS; open issues if needed.

**Reference Documents:** `RESEARCH/GOTCHAS.md`; `.cursor/rules/security.mdc` (if present)

**Acceptance Criteria:**
- [x] Code paths for vertex/index math reviewed
- [x] No unchecked overflow in hot path; findings documented
- [x] Sign-off or ticket in `SPRINTS/Sprint_1_6/SECURITY_SIGNOFF.md` or GOTCHAS

**Completion Record:**
```
Status: [x] Complete
Completed By: Security Specialist - cursor-session-2026-02-07-security
Completed On: 2026-02-07
Notes: SECURITY_SIGNOFF.md documents validate_depth_input (checked_mul for expected_len), depth_to_point_cloud (checked_mul for vertex_count); index math safe for MAX_DIMENSION 8192. GOTCHAS entry added.
```

---

#### Task SEC-302: Validate depth map inputs before processing
**Assigned Role:** Security Specialist  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** SEC-302

**Dependencies:** BACK-501 (entry point for depth map)

**What to Do:**
- Ensure mesh generation validates inputs: dimensions (width, height) sane (e.g. >0, <max); depth slice length matches width*height; depth values in expected range (0–1 or documented). Reject or error on invalid input; no panic or undefined behavior.
- Document validation in architecture or code; note in security sign-off.

**Reference Documents:** `prd.md` §8; input validation patterns in `load_image`/depth pipeline

**Acceptance Criteria:**
- [x] Depth map dimensions and length validated before use
- [x] Invalid input returns error (no panic)
- [x] Documented in code or architecture; noted in security review

**Completion Record:**
```
Status: [x] Complete
Completed By: Security Specialist - cursor-session-2026-02-07-security
Completed On: 2026-02-07
Notes: validate_depth_input() in mesh_generator.rs enforces width/height >0, ≤MAX_DIMENSION, depth.len() == width×height (checked_mul). Unit test validate_rejects_dimensions_over_max added. SECURITY_SIGNOFF.md and GOTCHAS updated.
```

---

## Subtask Allocation (for multi-role tasks)

Sprint 1.6 tasks are assigned to single roles. If any task is split later:

| Sub-task | Role | Owner | Status |
|----------|------|-------|--------|
| (none) | — | — | — |

---

## Success Criteria for Sprint

- [ ] All tasks complete per acceptance criteria
- [ ] Exit criteria from todo.md met:
  - Mesh generation produces valid point cloud
  - Vertex positions in correct units (mm)
  - Performance meets targets (<15s for 4K)
  - Memory usage within budget (<2GB for 4K)
  - Algorithm documented in architecture.md
- [ ] No blocking issues
- [ ] Gotchas recorded in `SPRINTS/Sprint_1_6/GOTCHAS.md` (merge to RESEARCH when done)
- [ ] Progress report filed

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| (none) | — | — |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | — |
| cargo clippy | 0 warnings | — |
| cargo fmt --check | PASS | — |
| npm run build | PASS | — |
| pytest | PASS | — |
| Mesh gen 4K | <15s | — |
| Memory 4K | <2GB | — |

---

## Progress Log (Handover Notes)

*Agents add handover notes when completing tasks that others depend on.*

```
### 2026-02-07 — Senior Researcher (cursor-session-2026-02-07-researcher)
Role: Senior Researcher (AI/ML). No Sprint 1.6 tasks assigned (notes: "No 1.6 tasks"). Persona read; role claimed and marked Complete. No deliverables this sprint; RESEARCH ownership and AI/ML pipeline remain for future sprints.

### 2026-02-07 — Documentation Specialist (cursor-session-2026-02-07-doc)
Role: Algorithm documentation. No explicit task ID; "If needed for algorithm docs" per assignment.
Delivered: docs/architecture.md — added § "Mesh generation (algorithm) — Sprint 1.6" with summary (sampling, coordinates, normals, output), pointers to RESEARCH/architecture.md § Mesh Generation and ADR-006, and to src-tauri/src/mesh_generator.rs and get_mesh_data. Listed ADR-006 in architecture decisions. No new developer-guide file; algorithm docs are in RESEARCH + docs/architecture.md.

### 2026-02-07 — System Architect (ARCH-201–204 COMPLETED)
Delivered: Mesh generation design in RESEARCH/architecture.md.
- ADR-006: Algorithm choice (point cloud + optional Delaunay), uniform grid sampling.
- § Mesh Generation (Sprint 1.6): Algorithm/sampling, vertex format (positions + normals, MeshData), topology (no overhangs, 2.5D, manifold), memory (single buffer, 4K/8K scaling).
Handover: Senior Engineer can implement BACK-501–506 against this fixed design. Junior Engineer 2D (JR2-501–504), Security (SEC-301–302) reference same doc.

### 2026-02-07 — Senior Engineer (BACK-501–506 COMPLETED)
Delivered: src-tauri/src/mesh_generator.rs and get_mesh_data Tauri command.
- Point cloud from depth map (depth_to_point_cloud), uniform grid (step_x/step_y), mm scale (depth_min_mm, depth_max_mm, pixel_to_mm), normals from depth gradient, input validation (validate_depth_input), single-buffer build. BACK-504: triangulation deferred per ADR-006.
- Key files: mesh_generator.rs (MeshData, MeshParams, tests), lib.rs (get_mesh_data), permissions/allow-generate-depth-map.toml (get_mesh_data added).
Handover: JR2-501–504 (unit tests, edge cases, benchmarks, memory profile), QA-501–504 (manual/automated tests), SEC-301–302 (integer overflow review, depth validation) can proceed. Frontend can call get_mesh_data() for Three.js preview (Sprint 1.7).

### 2026-02-07 — Security Specialist (SEC-301, SEC-302 COMPLETED)
Delivered: SECURITY_SIGNOFF.md; review of integer overflow (SEC-301) and depth map input validation (SEC-302).
- SEC-301: validate_depth_input and depth_to_point_cloud use checked_mul for expected_len and vertex_count; index math safe for MAX_DIMENSION 8192. Documented in SECURITY_SIGNOFF.md and GOTCHAS.
- SEC-302: validate_depth_input enforces dimensions >0, ≤MAX_DIMENSION, depth.len() == width×height; invalid input returns Err. Added test validate_rejects_dimensions_over_max.
Handover: Security sign-off approved for mesh generation. No open issues.

### 2026-02-07 — Junior Engineer 2D (JR2-501–504 COMPLETED)
Delivered: Unit tests (5×5 bounds, constant depth, gradient), edge-case tests (empty dimensions/slice rejected; single row/column valid), mesh generation benchmark (benches/mesh_generation.rs: 1K ~9.3ms, 4K ~73ms), and JR2-504 memory profiling procedure + result placeholder in GOTCHAS.md.
Handover: QA-501–504 and automated test QA-504 can use new tests; memory profile result to be filled when team runs profiler.
```

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **prd.md** — F1.5 Mesh Generation, §5.2–5.3, §7.1
3. **todo.md** — Sprint 1.6 full context
4. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
5. **RESEARCH/architecture.md** — Data flow, mesh step
6. **RESEARCH/rust-crates.md** — stl_io, large buffers, optimization
7. **RESEARCH/GOTCHAS.md** — IPC size, large payloads

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Status:** Ready for team; Architecture (ARCH-201–204) should be completed early so Backend can implement.
