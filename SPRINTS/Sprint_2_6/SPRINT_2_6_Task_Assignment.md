# Sprint 2.6: Enhanced 3D Preview

**Sprint Duration:** 2 weeks (10 working days)  
**Sprint Goal:** Improve 3D visualization with lighting, measurements, cross-sections, and preview export; expose authoritative mesh bounds where needed for QA and tooling.  
**Target Release:** v0.6.0-beta (enhanced preview)  
**Phase:** 2 — Enhanced UX  
**Source:** `todo.md` — Sprint 2.6: Enhanced 3D Preview; `prd.md` §F2.5  
**Last Updated:** 2026-04-16

---

## Pre-sprint gate (mandatory)

| Gate | Status |
|------|--------|
| **Sprint 2.5 closed** | Do **not** begin Sprint 2.6 until `SPRINTS/Sprint_2_5/VERIFICATION_CHECKLIST.md` is signed off (or Architect waiver documented). See `todo.md` Phase 2 banner and `SPRINTS/Sprint_2_5/SPRINT_2_5_Task_Assignment.md` — **Next sprint (planning rule)**. |

---

## Sprint Folder & Artefacts

**All sprint artefacts MUST be stored in this sprint's folder:**

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_2_6/SPRINT_2_6_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_2_6/TEST_PLAN_2_6.md` | Copy from `SPRINTS/TEST_PLAN_TEMPLATE.md`; manual + automated planning |
| Progress Report | `SPRINTS/Sprint_2_6/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_2_6/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_2_6/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Architect Approval | `SPRINTS/Sprint_2_6/ARCHITECT_APPROVAL.md` | If required for phase gate |
| Security Sign-off | `SPRINTS/Sprint_2_6/SECURITY_SIGNOFF.md` | If security review in sprint |
| Gotchas Log | `SPRINTS/Sprint_2_6/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

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
2. **Set your Cursor title to the role name.** Update the Cursor session (composer/chat) title so it matches your assigned role exactly (e.g. **System Architect**, **Senior Engineer**, **UI Designer**). This keeps the session clearly identified with the role you have taken.
3. **Read the persona file** listed in the "Persona File" column
4. **Adopt that persona** for all remaining work

### Step 3: Become Your Role
- Embody the agent's identity, expertise, and responsibilities
- Follow the persona file's guidance and project references
- Rename the agent so that it shows the agent identity in the agent list

**If all roles show "In Progress" or "Complete", STOP. No work available.**

### Step 4: Update status
- While progressing your role, update the status per the Status Values defined below.

### Optional: One-shot role assumption (automated)
An agent can **read this task assignment, find unassigned roles, and create one Cursor command per available role**. When you run one of those commands in a chat, that chat becomes a **one-shot agent** for that role (it claims the role and adopts the persona for the rest of the conversation). To generate the commands: run the Cursor command **"Create One-Shot Assume-Role Commands for This Sprint"** (`.cursor/commands/create-assume-role-commands.md`). Optionally @-mention this Task Assignment file so the agent knows which sprint to use. The agent will create files like `.cursor/commands/assume-sprint-X-Y-<role-slug>.md`; run any of them to assume that role one-shot.

---

## Roles required for this sprint

| Role | Why required |
|------|--------------|
| Senior Engineer | BACK-1501–1502: mesh bounds IPC; optional backend slice computation |
| UI Designer | UI-1501–1506: PBR, lighting, measure, cross-section, screenshot, indexed mesh modes |
| Junior Engineer 3D | JR1-1501–1503: camera ortho/persp, grid units, axis helpers — Three.js preview |
| Quality Engineer | QA-1501–1503: manual preview matrix, measurement accuracy, screenshot export |

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | Available | — | — | Optional: ADR-011 alignment for bounds vs blank envelope; slice API sign-off if BACK-1502 adds IPC |
| Senior Engineer | `.agents/senior-engineer.md` | Available | — | BACK-1501, BACK-1502 | `MeshData` / `get_mesh_data`; optional new command for slice |
| UI Designer | `.agents/ui-designer.md` | Available | — | UI-1501–UI-1506 | Primary: `Preview3D.svelte` and related UI |
| Senior Researcher (AI/ML) | `.agents/researcher.md` | Available | — | — | Optional: `RESEARCH/threejs.md` updates after PBR/clipping |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Available | — | — | No dedicated 2.6 task IDs — support only if depth panel cross-links preview |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Available | — | JR1-1501–JR1-1503 | Camera, grid, axes in Three.js preview |
| Quality Engineer | `.agents/quality-engineer.md` | Available | — | QA-1501–QA-1503 | Windows-primary manual matrix; evidence in sprint folder |
| Security Specialist | `.agents/security-specialist.md` | Available | — | — | Optional: review new IPC if BACK-1502 adds commands |
| Documentation Specialist | `.agents/documentation-specialist.md` | Available | — | — | Optional: user guide §3D preview after features land |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References (Source of Truth)

- **Scope:** `prd.md` §F2.5 Enhanced 3D Preview  
- **Sprint source:** `todo.md` — Sprint 2.6: Enhanced 3D Preview (distinct from the duplicate “Sprint 2.6: UI Polish” heading — different scope)  
- **Architecture:** `RESEARCH/architecture.md` (ADR-011 volumetric / preview paths)  
- **Three.js:** `RESEARCH/threejs.md`  
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`  
- **Prior sprint:** `SPRINTS/Sprint_2_5/SPRINT_2_5_Task_Assignment.md` (gate)

---

## Sprint Progress Summary

| Section | Status | Completion |
|---------|--------|------------|
| BACK-1501 – BACK-1502 | ⏳ Not Started | 0% |
| UI-1501 – UI-1506 | ⏳ Not Started | 0% |
| JR1-1501 – JR1-1503 | ⏳ Not Started | 0% |
| QA-1501 – QA-1503 | ⏳ Not Started | 0% |

**Overall Sprint Progress:** [x] Not Started / [ ] In Progress / [ ] Complete

---

## Task Breakdown

### Phase 2: Enhanced 3D Preview

#### Task: Provide mesh bounds data to frontend
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** BACK-1501

**Dependencies:**
- Sprint 2.5 gate — Status: [ ] (complete before sprint start)
- `get_mesh_data` / `MeshData` — Status: existing

**What to Do:**
- Expose axis-aligned bounds (mm) for the same geometry as `get_mesh_data` (e.g. extend `MeshData` with `boundsMin` / `boundsMax` or nested structure; or shared helper + `compute_bbox`-style pass).
- Keep contract consistent with `src/lib/tauri.ts` types; additive JSON fields preferred.

**Reference Documents:**
- `prd.md` §F2.5  
- `src-tauri/src/mesh_generator.rs`, `src-tauri/src/lib.rs`  
- `RESEARCH/architecture.md` (ADR-011 — distinguish content bounds vs `BlankEnvelope`)

**Acceptance Criteria:**
- [ ] Frontend can read authoritative bounds without rescanning all vertices for basic framing / QA
- [ ] `cargo test` / `cargo clippy -- -D warnings` pass for changed Rust code

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Calculate cross-section slice (if backend-computed)
**Assigned Role:** Senior Engineer (+ System Architect advisory for IPC shape)  
**Priority:** Medium (conditional on product choosing backend slice)  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked / [ ] Waived (frontend-only)  
**Task ID:** BACK-1502

**Dependencies:**
- UI-1504 / UI-1501 — agree slice strategy (Three.js clipping vs Rust slice) — Status: [ ]

**What to Do:**
- If cross-section is implemented **only** in Three.js (clipping planes), document waiver and ensure UI-1504 acceptance still met.
- If backend slice is required: add command(s) returning slice polylines or depth profile; reuse adjusted depth + mask path consistency with `get_mesh_data`; register in Tauri permissions.

**Reference Documents:**
- `RESEARCH/architecture.md`  
- `src-tauri/src/lib.rs` — depth/mesh pipeline

**Acceptance Criteria:**
- [ ] Slice definition matches app axis convention (document in sprint GOTCHAS if Y-flip or image vs world noted)
- [ ] New IPC documented; capabilities updated if new commands

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Implement PBR shading (diffuse, specular)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-1501

**Dependencies:**
- UI-1506 — indexed `THREE.Mesh` path stable for solid mode — Status: [ ]
- Mesh normals present on triangulated path — Status: existing in generator

**What to Do:**
- Use `MeshStandardMaterial` or `MeshPhysicalMaterial` for solid preview; preserve disposal patterns in `Preview3D.svelte`.
- Ensure specular response visible with directional light (see UI-1502).

**Reference Documents:**
- `prd.md` §F2.5 (realistic lighting; PBR technical notes)  
- `RESEARCH/threejs.md`  
- `package.json` — `three` version

**Acceptance Criteria:**
- [ ] Solid mode shows diffuse + specular response; no WebGL errors on toggle
- [ ] Materials and geometry disposed on reload per existing patterns

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Add lighting controls (ambient, directional intensity)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-1502

**Dependencies:**
- UI-1501 — Status: [ ] (meaningful specular depends on PBR-capable material)

**What to Do:**
- Extend or refine ambient / directional controls (sliders already exist — align with sprint spec: ranges, labels, accessibility).
- Live update of light intensities without leaking objects.

**Reference Documents:**
- `prd.md` §F2.5  
- `src/components/Preview3D.svelte`

**Acceptance Criteria:**
- [ ] User can adjust ambient and directional intensity; changes apply immediately
- [ ] Controls meet accessibility expectations (labels, keyboard where applicable)

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Measure tool (click two points, show distance)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-1503

**Dependencies:**
- BACK-1501 — optional but recommended for mm consistency checks — Status: [ ]
- Indexed mesh for efficient raycast preferred — Status: [ ] with UI-1506

**What to Do:**
- Two-pick distance in mm; line or markers; clear/escape; avoid fighting `OrbitControls` (modifier or measure mode).

**Reference Documents:**
- `prd.md` §F2.5 (measure tool)  
- `src/components/Preview3D.svelte`

**Acceptance Criteria:**
- [ ] Distance displayed in mm between two picked points on mesh (or documented behavior for point-only mode)
- [ ] Tool can be exited without leaving stale state

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Cross-section view (slice mesh at depth plane)
**Assigned Role:** UI Designer (+ Senior Engineer if BACK-1502)  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-1504

**Dependencies:**
- UI-1501, UI-1506 — Status: [ ]
- BACK-1502 — Status: [ ] or waived

**What to Do:**
- Implement clipping plane(s) or backend-fed slice per agreed design; slider tied to depth/bounds.

**Reference Documents:**
- `prd.md` §F2.5  
- `RESEARCH/threejs.md`

**Acceptance Criteria:**
- [ ] User can move slice plane and see coherent cross-section for triangulated mesh
- [ ] `renderer.localClippingEnabled` / material clipping configured without per-frame full geometry rebuild

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Export preview as PNG (screenshot function)
**Assigned Role:** UI Designer  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-1505

**Dependencies:**
- Preview rendering stable — Status: [ ]

**What to Do:**
- Canvas screenshot to PNG; handle HiDPI / `devicePixelRatio`; optional fixed resolution (document behavior in TEST_PLAN).

**Reference Documents:**
- `prd.md` §F2.5 (export preview as image)  
- `RESEARCH/threejs.md`

**Acceptance Criteria:**
- [ ] User can save PNG; file opens in system viewer
- [ ] Documented resolution behavior (viewport vs fixed) in `TEST_PLAN_2_6.md`

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Wireframe / Solid via THREE.Mesh + indices (if deferred from 2.2)
**Assigned Role:** UI Designer  
**Priority:** High (enabler)  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-1506

**Dependencies:**
- `MeshData.indices` from backend when triangulated — Status: existing

**What to Do:**
- Verify or complete indexed mesh path for wireframe/solid; overlay when indices missing (points-only).

**Reference Documents:**
- `src/components/Preview3D.svelte`  
- `src/lib/tauri.ts` — `MeshData`

**Acceptance Criteria:**
- [ ] When indices present, wireframe and solid use `THREE.Mesh` with indexed geometry
- [ ] Clear UX when triangulation unavailable

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Camera projection toggle (orthographic / perspective)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR1-1501

**Dependencies:**
- Preview3D camera lifecycle — Status: existing

**What to Do:**
- Toggle `PerspectiveCamera` / `OrthographicCamera`; resize handlers; orbit controls attach to active camera; presets (Top/Front/Side) remain usable.

**Reference Documents:**
- `RESEARCH/threejs.md`  
- `src/components/Preview3D.svelte`

**Acceptance Criteria:**
- [ ] Toggle works without stale camera references; no clipping at typical mesh sizes
- [ ] Documented behavior in test plan

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Grid size toggle (mm, cm)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Low  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR1-1502

**Dependencies:**
- Scene units remain mm internally — Status: convention

**What to Do:**
- UI toggle: display grid spacing in mm vs cm (or adjust grid if product requires — document choice).

**Reference Documents:**
- `src/components/Preview3D.svelte`

**Acceptance Criteria:**
- [ ] Labels reflect selected unit; mesh/blank scale unchanged (mm world space)

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Axis helpers (X, Y, Z labels)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR1-1503

**Dependencies:**
- JR1-1501 — labels must update under ortho/persp — Status: [ ]

**What to Do:**
- `AxesHelper` plus labels (e.g. CSS2D or sprites); pointer-events / readability; align with future blank wireframe origin.

**Reference Documents:**
- `RESEARCH/threejs.md`  
- `src/components/Preview3D.svelte`

**Acceptance Criteria:**
- [ ] Axes and labels visible at default window size; readable contrast
- [ ] Works in both camera modes

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Manual test — all preview features
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-1501

**Dependencies:**
- UI + JR1 implementation complete enough for test — Status: [ ]

**What to Do:**
- Execute manual matrix: lighting, PBR, camera, grid, axes, measure, cross-section, screenshot, regression smoke.
- Record in `MANUAL_TEST_REPORT.md`; Windows primary.

**Reference Documents:**
- `SPRINTS/TEST_PLAN_TEMPLATE.md` → `TEST_PLAN_2_6.md`

**Acceptance Criteria:**
- [ ] All features in scope exercised; pass/fail recorded per case

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Verify measurement accuracy (compare to export dimensions)
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-1502

**Dependencies:**
- UI-1503, export path — Status: [ ]
- BACK-1501 — recommended for bounds cross-check — Status: [ ]

**What to Do:**
- Compare preview measure distances to ground-truth from exported mesh (or known fixture); define tolerance in test plan.

**Reference Documents:**
- `prd.md` §F2.5  
- Export docs / mesh format in repo

**Acceptance Criteria:**
- [ ] Documented relative/absolute error within team-agreed tolerance

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task: Test screenshot export (resolution, format)
**Assigned Role:** Quality Engineer  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-1503

**Dependencies:**
- UI-1505 — Status: [ ]

**What to Do:**
- Verify PNG format, dimensions vs viewport/DPI, Windows display scaling cases.

**Reference Documents:**
- `TEST_PLAN_2_6.md` (when created)

**Acceptance Criteria:**
- [ ] PNG valid; resolution behavior matches sprint spec; no black-frame failure

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

## Subtask Allocation (for multi-role tasks)

| Sub-task | Role | Owner | Status |
|----------|------|-------|--------|
| UI-1504 — Three.js clipping | UI Designer | [ ] | [ ] |
| UI-1504 — Rust slice (if any) | Senior Engineer | [ ] | [ ] |
| QA-1502 — Fixture dimensions | Quality Engineer | [ ] | [ ] |

---

## Success Criteria for Sprint

- [ ] All tasks complete per acceptance criteria (or BACK-1502 explicitly waived with Architect note)
- [ ] Exit criteria from `todo.md` Sprint 2.6 met:
  - [ ] PBR lighting improves visual quality
  - [ ] Measure tool displays accurate distances (per QA-1502 tolerance)
  - [ ] Cross-section view functional
  - [ ] Screenshot export works
- [ ] `todo.md` Sprint 2.6 checkboxes updated
- [ ] Gotchas recorded in `SPRINTS/Sprint_2_6/GOTCHAS.md` (merge to `RESEARCH/GOTCHAS.md` when done)
- [ ] `PROGRESS_REPORT.md` filed
- [ ] `VERIFICATION_CHECKLIST.md` signed

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| Sprint 2.5 not closed | Program / Architect | 🔴 Gate until 2.5 VERIFICATION_CHECKLIST signed |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| `cargo test --manifest-path src-tauri/Cargo.toml` | PASS | — |
| `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` | 0 warnings | — |
| `cargo fmt --check` | PASS | — |
| `npm run build` | PASS | — |
| `npm test` | PASS | — |
| `SP3D_USE_STUB=1` pytest | PASS | — |

---

## Progress Log (Handover Notes)

*Agents add handover notes when completing tasks that others depend on.*

```
### [Date] — [Role] (Task COMPLETED)
[What was delivered. Key files. Gotchas. Handover to whom.]
```

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table  
2. **`prd.md` §F2.5** — Enhanced 3D Preview  
3. **`todo.md` — Sprint 2.6: Enhanced 3D Preview** (not the duplicate “UI Polish” Sprint 2.6 block)  
4. **`RESEARCH/AI_DEVELOPMENT_GUIDE.md`** — Coordination  
5. **`RESEARCH/threejs.md`** — Preview implementation  
6. **`RESEARCH/GOTCHAS.md`** — Known pitfalls  
7. **`SPRINTS/Sprint_2_5/SPRINT_2_5_Task_Assignment.md`** — Pre-sprint gate  

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Status:** Ready for role claims — **blocked on Sprint 2.5 close** until gate cleared
