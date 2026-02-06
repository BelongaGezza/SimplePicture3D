# Sprint Task Assignment — Sprint 1.5

**Source:** `todo.md` — Sprint 1.5. Populated by Senior Engineer per RESEARCH/AI_DEVELOPMENT_GUIDE and SPRINT_TASKING_TEMPLATE.  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`

---

## Sprint 1.4 Status Review (Handover to 1.5)

**Context:** Sprint 1.4 is **complete** (verified 2026-02-04). Depth map generation and preview are implemented; depth controls are placeholder only.

| Phase/Section | Status |
|---------------|--------|
| Backend (BACK-301–304) | ✅ Complete — `generate_depth_map`, `get_depth_map`, AppState, progress in response |
| AI/Research (AI-301–304) | ✅ Complete — 0–1 normalization, JSON contract, benchmarks, architecture format |
| UI (UI-301–305) | ✅ Complete — DepthMapPreview, Generate button, progress, side-by-side layout |
| Junior 2D/3D, QA | ✅ Complete — tests, canvas, zoom/pan, manual tests Pass |

**Handover to Sprint 1.5:**
- **Backend:** Depth map is in `AppState` (Mutex<Option<DepthMapOutput>>). Add depth adjustment logic (brightness, contrast, gamma, invert, range) and apply to a copy or derived state; cache original for reset.
- **Frontend:** Right sidebar has depth preview; add DepthControls component with sliders (Depth Range, Brightness, Gamma), Invert checkbox, Reset button. Preview must update from adjusted depth (debounced).
- **Contract:** `docs/architecture.md` § Depth map format (0–1); depth adjustments operate on the same array; document adjustment API if new Tauri commands are added.

---

## Sprint 1.5: Manual Depth Adjustments

**Sprint Duration:** 2 weeks (10 working days)  
**Sprint Goal:** User can modify the AI-generated depth map with sliders and controls.  
**Target Release:** —  
**Phase:** 1 (MVP)  
**Source:** `todo.md` — Sprint 1.5  
**Last Updated:** 2026-02-04

---

## Sprint Folder & Artefacts

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_1_5/SPRINT_1_5_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_1_5/TEST_PLAN_1_5.md` | QA test planning (manual + automated) |
| Progress Report | `SPRINTS/Sprint_1_5/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_1_5/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_1_5/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Gotchas Log | `SPRINTS/Sprint_1_5/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

---

## CRITICAL: Role Selection (READ FIRST — STOP HERE UNTIL COMPLETE)

**You are an unassigned agent. You MUST claim a role before proceeding.**

### Step 1: Review Available Roles
Find a role where Status = `Available` and no agent is assigned.

### Step 2: Claim Your Role
1. **Edit this document** to update that role's row: set Status to `In Progress`, add your session identifier to Assigned Agent.
2. **Read the persona file** listed in the Persona File column.
3. **Adopt that persona** for all remaining work.

### Step 3: Become Your Role
- Embody the agent's identity and responsibilities.
- Follow the persona file and project references.

**If all roles show "In Progress" or "Complete", STOP. No work available.**

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | Available | — | ARCH-101–102 | ADR review, Python distribution docs (Consultant Priority 3) |
| Senior Engineer | `.agents/senior-engineer.md` | Available | — | BACK-401–405 | Depth adjustment logic, apply transforms, cache original, reset |
| UI Designer | `.agents/ui-designer.md` | Available | — | UI-401–405 | DepthControls, sliders, Invert, real-time preview, Reset |
| Senior Researcher (AI/ML) | `.agents/researcher.md` | Available | — | AI-401–403 | pytest suite, CI integration (Consultant Priority 1) |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Available | — | JR2-401–403 | Unit tests for adjustments, boundary tests, benchmark |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Available | — | JR1-401–404 | Slider styling, numeric inputs, keyboard controls, responsiveness |
| Quality Engineer | (see todo.md) | Available | — | QA-401–405 | Manual/automated tests, clippy CI (Consultant Priority 1) |
| Security Specialist | `.agents/security-specialist.md` | Available | — | — | No dedicated 1.5 tasks; ad-hoc if needed |
| Documentation Specialist | `.agents/documentation-specialist.md` | Available | — | — | Supporting docs if needed |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

*Note: Junior Engineer #1 (frontend) = Junior Engineer 3D; Junior Engineer #2 (backend) = Junior Engineer 2D.*

**Consultant Report Tasks Added (2026-02-06):** ARCH-101–102, AI-401–403, QA-405

---

## Canonical References

- **Scope:** `prd.md` — F1.2 depth controls, F1.4 preview, §5.2–5.3, §6 layout
- **Sprint source:** `todo.md` — Sprint 1.5
- **Architecture:** `docs/architecture.md` § Depth map format, § Frontend implications
- **ADRs:** `RESEARCH/architecture.md` — ADR-001 to ADR-004 (added 2026-02-06)
- **Technology:** `RESEARCH/frontend.md`, `RESEARCH/tauri.md`
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`
- **Consultant Report:** `Consultant_Recommendations_Report_6Feb2026.md` — Priority 1 (Testing), Priority 3 (ADRs)

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| Backend (BACK-401–405) | ⏳ Not Started | 0% |
| UI (UI-401–405) | ⏳ Not Started | 0% |
| Junior 2D (JR2-401–403) | ⏳ Not Started | 0% |
| Junior 3D (JR1-401–404) | ⏳ Not Started | 0% |
| Quality (QA-401–405) | ⏳ Not Started | 0% |
| Testing Infra (AI-401–403) | ⏳ Not Started | 0% |
| ADR Docs (ARCH-101–102) | ⏳ Not Started | 0% |

**Overall Sprint Progress:** [x] Not Started / [ ] In Progress / [ ] Complete

**Note:** Tasks AI-401–403, ARCH-101–102, QA-405 added 2026-02-06 per Consultant Recommendations Report.

---

## Task Breakdown

### Senior Engineer

#### BACK-401: Implement depth adjustment functions (brightness, contrast, gamma)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** BACK-401

**Dependencies:**
- Sprint 1.4: depth map in AppState (Complete)

**What to Do:**
- Implement functions that transform a depth array (0–1) with brightness, contrast, and gamma. Brightness: additive offset (clamped to [0,1]). Contrast: scale around midpoint (e.g. 0.5). Gamma: power-law curve. All operate on f32 in [0,1]; output clamped to [0,1].
- Can live in Rust (new module e.g. `depth_adjust.rs` or in `lib.rs`) or be computed on frontend from raw depth; decide and document. If Rust: accept depth slice + params, return new Vec<f32> or apply in-place to a working copy.
- Document formulas in code or architecture (e.g. brightness: `v + b`, contrast: `(v - 0.5) * c + 0.5`, gamma: `v^g`).

**Reference Documents:** `prd.md` F1.2; `docs/architecture.md` depth format; `src-tauri/src/lib.rs`

**Acceptance Criteria:**
- [ ] Brightness, contrast, gamma functions implemented and produce values in [0, 1]
- [ ] Formulas documented (code comment or architecture)
- [ ] No mutation of original depth map (work on copy or derived state)

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### BACK-402: Apply transformations to depth map array
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** BACK-402

**Dependencies:** BACK-401 (adjustment functions exist).

**What to Do:**
- Wire adjustment parameters (brightness, contrast, gamma, invert) to the depth array. Apply in a defined order (e.g. invert → gamma → contrast → brightness) so behaviour is predictable.
- If backend owns state: apply to a "display" or "adjusted" copy and expose to frontend (e.g. via `get_depth_map` with params or a dedicated `get_adjusted_depth_map`). If frontend owns: pass raw depth + params and compute in frontend; backend may still cache original for reset.
- Ensure single pass or minimal passes over the array for performance (e.g. 1920×1080).

**Reference Documents:** `docs/architecture.md`; `src-tauri/src/lib.rs`, AppState

**Acceptance Criteria:**
- [ ] Transformations applied in consistent order; output in [0, 1]
- [ ] Performance acceptable for at least 1920×1080 depth (e.g. <100ms)
- [ ] Original depth preserved for reset (BACK-405)

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### BACK-403: Invert depth toggle
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** BACK-403

**Dependencies:** BACK-402 (transformation pipeline).

**What to Do:**
- Add invert step: `v' = 1.0 - v` for each depth value. Can be first or last in pipeline; document order. Expose as boolean (invert on/off) to UI.

**Reference Documents:** `prd.md` F1.2; BACK-402

**Acceptance Criteria:**
- [ ] Invert toggle flips depth (near ↔ far) correctly
- [ ] Combined with other adjustments produces correct result
- [ ] UI can set invert state (checkbox)

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### BACK-404: Depth range mapping (0–1 → 2–10mm)
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** BACK-404

**Dependencies:** BACK-402 (transformation pipeline).

**What to Do:**
- Map normalized depth [0, 1] to physical range [min_mm, max_mm] (e.g. 2–10 mm for laser engraving). Linear: `z_mm = min_mm + v * (max_mm - min_mm)`. This may be used for mesh generation later; for 1.5 focus on the mapping formula and storing/using range params (min_mm, max_mm) so preview and future STL export are consistent.
- Expose min/max (or range + offset) to frontend so user can set depth range; document in architecture if this becomes part of mesh/export contract.

**Reference Documents:** `prd.md` F1.2, §5.3; `docs/architecture.md`

**Acceptance Criteria:**
- [ ] Depth range (e.g. 2–10 mm) configurable and applied
- [ ] Formula documented; values suitable for downstream mesh
- [ ] UI can set range (slider or numeric inputs)

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### BACK-405: Cache original depth map for reset function
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** BACK-405

**Dependencies:** BACK-302 (Sprint 1.4: depth in state); BACK-402 (adjusted copy).

**What to Do:**
- Keep original depth map (from `generate_depth_map`) unchanged when applying adjustments. Store adjusted depth in a separate field or compute on demand from original + params. Provide a "reset" that clears adjustment params to defaults (or restores original for display).
- If backend: e.g. `AppState { original_depth: Option<DepthMapOutput>, adjustment_params: AdjustmentParams }` and compute adjusted view from original + params. If frontend: keep original depth from last `generate_depth_map` response; compute adjusted in JS/TS; reset = clear params.
- Document where original is stored and how reset is triggered (Tauri command vs frontend-only).

**Reference Documents:** `src-tauri/src/lib.rs` AppState; BACK-302

**Acceptance Criteria:**
- [ ] Original depth preserved after adjustments
- [ ] Reset restores display to unadjusted depth (or default params)
- [ ] No unnecessary full clone on every slider change (e.g. store params, compute on demand or debounce)

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

### UI Designer

#### UI-401: Create DepthControls component (sidebar)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-401

**Dependencies:** BACK-401–403 (adjustment params defined); Sprint 1.4 layout (right sidebar).

**What to Do:**
- Add a DepthControls component in the right sidebar (below or beside depth preview). Layout: sliders for Depth Range (min–max mm), Brightness, Gamma; Invert Depth checkbox; Reset button. Use existing design system (Tailwind, Button component).
- Component receives current adjustment params and callbacks (or bindings) for change and reset. Disabled when no depth map loaded.

**Reference Documents:** `prd.md` F1.2, §6; `SPRINTS/Sprint_1_4/` layout; `src/App.svelte`, `src/components/`

**Acceptance Criteria:**
- [ ] DepthControls visible in sidebar when depth is available
- [ ] Disabled or hidden when no depth map
- [ ] Accessible (labels, focus order); theme-aware if applicable

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### UI-402: Implement sliders: Depth Range, Brightness, Gamma
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-402

**Dependencies:** UI-401 (component shell); BACK-401–404 (param semantics).

**What to Do:**
- Implement sliders for Depth Range (e.g. min 2 mm, max 10 mm; or single "range" scale), Brightness (e.g. -0.5 to 0.5 or 0–100%), Gamma (e.g. 0.5–2.0). Bind to adjustment state; on change, update params and trigger preview update (debounced per UI-404).
- Ensure min/max and step are clear; optional tooltips. Coordinate with JR1-402 for numeric input fields next to sliders.

**Reference Documents:** `prd.md` F1.2; `RESEARCH/frontend.md`

**Acceptance Criteria:**
- [ ] Sliders control correct params; range and scale documented
- [ ] Values stay within valid bounds
- [ ] Preview updates when sliders change (debounced)

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### UI-403: Invert Depth checkbox
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-403

**Dependencies:** BACK-403 (invert applied); UI-401 (DepthControls).

**What to Do:**
- Add checkbox "Invert Depth" (or similar). Toggle invert on/off; preview updates (debounced). Accessible label and state.

**Reference Documents:** `prd.md` F1.2; UI-402

**Acceptance Criteria:**
- [ ] Checkbox toggles invert; preview reflects state
- [ ] Accessible (label, checked state)

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### UI-404: Real-time preview updates (debounced)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-404

**Dependencies:** BACK-402 (adjusted depth available); UI-402, UI-403 (controls emit changes).

**What to Do:**
- When user changes sliders or invert, update depth preview within 100 ms of last change (debounce). If backend computes adjusted depth: call command or read state with new params and re-render. If frontend computes: run adjustment on current depth + params and re-draw canvas. Target: preview updates within 100 ms of slider release or after debounce interval.

**Reference Documents:** `prd.md` exit criteria "Preview updates within 100ms"; BACK-402

**Acceptance Criteria:**
- [ ] Preview updates within 100 ms of slider change (debounced)
- [ ] No excessive IPC or re-renders (e.g. debounce 50–100 ms)
- [ ] Smooth UX; no visible lag for 1080p depth

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### UI-405: Reset button to restore original depth map
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** UI-405

**Dependencies:** BACK-405 (reset semantics); UI-401 (DepthControls).

**What to Do:**
- Add Reset button in DepthControls. On click: restore adjustment params to defaults (e.g. brightness 0, contrast 1, gamma 1, invert false, range 2–10 mm) and refresh preview from original depth. If backend holds state: invoke reset command or set params to default and refetch. If frontend: set params to default and re-apply to original depth.

**Reference Documents:** `prd.md` F1.2; BACK-405

**Acceptance Criteria:**
- [ ] Reset button restores original depth view (or default params)
- [ ] Sliders/checkbox reflect default state after reset
- [ ] Accessible (button label, focus)

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

### Junior Engineer 3D (frontend)

#### JR1-401: Style sliders with TailwindCSS
**Assigned Role:** Junior Engineer 3D  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR1-401

**Dependencies:** UI-401, UI-402 (DepthControls and sliders exist).

**What to Do:**
- Style slider controls with Tailwind (or existing design tokens). Consistent with app theme; clear track and thumb; optional labels above/below. Coordinate with UI Designer.

**Reference Documents:** `RESEARCH/frontend.md`; `src/app.css`, existing components

**Acceptance Criteria:**
- [ ] Sliders visually consistent with app
- [ ] Usable on 1024×768 (prd minimum)

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### JR1-402: Add numerical input fields next to sliders
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR1-402

**Dependencies:** UI-402 (sliders in place).

**What to Do:**
- Add number inputs beside (or below) each slider so users can type exact values. Keep in sync with slider; validate min/max; same step as slider if applicable.

**Reference Documents:** UI-402; `RESEARCH/frontend.md`

**Acceptance Criteria:**
- [ ] Numeric inputs for Depth Range, Brightness, Gamma
- [ ] Values stay in bounds; sync with slider

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### JR1-403: Implement keyboard controls (arrow keys for sliders)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR1-403

**Dependencies:** UI-402 (sliders); JR1-401 (slider elements).

**What to Do:**
- When a slider is focused, arrow keys (left/right or up/down) increment/decrement value by step. Document in UI or GOTCHAS if needed.

**Reference Documents:** `prd.md` accessibility; UI-402

**Acceptance Criteria:**
- [ ] Arrow keys change slider value when focused
- [ ] Step size consistent with slider

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### JR1-404: Test slider responsiveness
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR1-404

**Dependencies:** UI-402, UI-404 (sliders and debounced preview).

**What to Do:**
- Manually or with a simple test verify sliders respond smoothly; no jank or freeze during drag. Test with 1080p depth if available. Document any limits in GOTCHAS.

**Reference Documents:** `prd.md` §7.1; UI-404

**Acceptance Criteria:**
- [ ] Sliders responsive; no blocking main thread
- [ ] Results or limits noted in GOTCHAS if needed

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

### Junior Engineer 2D (backend / tests)

#### JR2-401: Write unit tests for depth adjustment algorithms
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR2-401

**Dependencies:** BACK-401 (adjustment functions).

**What to Do:**
- Add Rust (or frontend) unit tests: given a small depth array and params, assert output in [0, 1] and expected behaviour (e.g. brightness +0.2 shifts values; gamma 2 darkens midtones). Cover at least one case per function (brightness, contrast, gamma, invert).

**Reference Documents:** BACK-401; `src-tauri/src/lib.rs` tests

**Acceptance Criteria:**
- [ ] Tests exist for brightness, contrast, gamma, invert
- [ ] Tests pass in CI; no flake

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### JR2-402: Test boundary conditions (min/max values)
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR2-402

**Dependencies:** BACK-401, BACK-402 (adjustments and pipeline).

**What to Do:**
- Test edge cases: all 0, all 1, mixed; extreme params (brightness ±1, gamma 0.1 and 5). Assert no panic, no NaN, output clamped to [0, 1]. Document expected behaviour for boundary params if non-obvious.

**Reference Documents:** BACK-401, BACK-402; RESEARCH/GOTCHAS.md

**Acceptance Criteria:**
- [ ] Boundary tests pass; no panic/NaN
- [ ] Extreme params documented or clamped

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### JR2-403: Benchmark adjustment performance (real-time?)
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** JR2-403

**Dependencies:** BACK-402 (full pipeline).

**What to Do:**
- Benchmark application of all adjustments to a 1920×1080 depth array (or largest target size). Target: complete in <100 ms so real-time preview is feasible. Document in GOTCHAS or test plan; report CPU time.

**Reference Documents:** `prd.md` §7.1; UI-404 (100 ms target); RESEARCH/GOTCHAS.md

**Acceptance Criteria:**
- [ ] Benchmark run for at least 1080p depth
- [ ] Result documented; <100 ms or gap explained

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

### Quality Engineer

#### QA-401: Manual test: adjust all controls, verify preview updates
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-401

**Dependencies:** BACK-401–405, UI-401–405 (full flow).

**What to Do:**
- Execute manual test: load image, generate depth, then change each slider and Invert; verify preview updates. Reset and verify restoration. Record in MANUAL_TEST_REPORT.md.

**Reference Documents:** `SPRINTS/Sprint_1_5/TEST_PLAN_1_5.md`, `MANUAL_TEST_REPORT.md`

**Acceptance Criteria:**
- [ ] All controls exercised; preview matches expectations
- [ ] Results in manual test report

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### QA-402: Test extreme values (brightness 0%, 200%)
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-402

**Dependencies:** UI-402 (sliders); BACK-401 (clamping).

**What to Do:**
- Set brightness (and other params) to extreme min/max. Verify no crash, no corrupt display; depth stays in valid range or UI prevents invalid input. Document in manual test report.

**Reference Documents:** JR2-402; TEST_PLAN_1_5.md

**Acceptance Criteria:**
- [ ] Extreme values handled; no crash
- [ ] Result in manual test report

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### QA-403: Verify reset button restores original depth map
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-403

**Dependencies:** BACK-405, UI-405 (reset implemented).

**What to Do:**
- After applying several adjustments, click Reset. Verify preview matches the initial depth (before any slider changes). Compare visually or via exported/serialized data if available. Record in manual test report.

**Reference Documents:** BACK-405; UI-405; TEST_PLAN_1_5.md

**Acceptance Criteria:**
- [ ] Reset restores original depth view
- [ ] Pass/fail in manual test report

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### QA-404: Automated test: apply adjustments, check output array
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** QA-404

**Dependencies:** BACK-401–402 (adjustments and pipeline); JR2-401 (unit tests).

**What to Do:**
- Add automated test: feed known depth array + params, get adjusted array; assert all values in [0, 1], and (optional) assert expected value at a few indices for a known formula. Can be Rust unit test or frontend test. Document in test plan.

**Reference Documents:** JR2-401; TEST_PLAN_1_5.md

**Acceptance Criteria:**
- [ ] Automated test exists; asserts output in [0, 1] and optional sanity checks
- [ ] Runs in CI

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### QA-405: Add cargo clippy to CI (Consultant Priority 1)
**Assigned Role:** Quality Engineer
**Priority:** Critical
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** QA-405

**Dependencies:** None (standalone CI enhancement).

**What to Do:**
- Add `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` step to `.github/workflows/ci.yml` backend job
- Ensure job fails on any clippy warnings
- Fix any existing clippy warnings that surface

**Reference Documents:** Consultant_Recommendations_Report_6Feb2026.md Priority 1; todo.md Testing requirements

**Acceptance Criteria:**
- [ ] CI runs clippy and fails on warnings
- [ ] All existing code passes clippy

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

### Senior Researcher (Testing Infrastructure - Consultant Priority 1)

#### AI-401: Create pytest suite for depth_estimator.py
**Assigned Role:** Senior Researcher
**Priority:** Critical
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** AI-401

**Dependencies:** None.

**What to Do:**
- Create `python/tests/test_depth_estimator.py`
- Test stub mode: verify output shape, 0-1 normalization, JSON format
- Test CLI interface: `--input`, `--no-model` flags
- Test error cases: missing file, invalid image
- All tests must work without downloading AI model (use SP3D_USE_STUB=1)

**Reference Documents:** Consultant_Recommendations_Report_6Feb2026.md Priority 1; python/python/depth_estimator.py

**Acceptance Criteria:**
- [ ] pytest suite exists in python/tests/
- [ ] Tests pass with `SP3D_USE_STUB=1 pytest python/`
- [ ] Coverage for stub mode, CLI args, error handling

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### AI-402: Add pytest to CI workflow
**Assigned Role:** Senior Researcher
**Priority:** High
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** AI-402

**Dependencies:** AI-401 (pytest suite exists).

**What to Do:**
- Add Python job to `.github/workflows/ci.yml`
- Install Python 3.10+, pip install pytest
- Run `SP3D_USE_STUB=1 pytest python/`
- Ensure CI fails if pytest fails

**Reference Documents:** AI-401; Consultant_Recommendations_Report_6Feb2026.md

**Acceptance Criteria:**
- [ ] CI runs pytest on every push/PR
- [ ] Uses stub mode (no model download in CI)
- [ ] Failures block merge

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### AI-403: Document Python test commands in CLAUDE.md
**Assigned Role:** Senior Researcher
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** AI-403

**Dependencies:** AI-401 (pytest suite exists).

**What to Do:**
- Update CLAUDE.md Testing Commands section with pytest commands
- Document SP3D_USE_STUB environment variable
- Add to README.md if appropriate

**Reference Documents:** CLAUDE.md; AI-401

**Acceptance Criteria:**
- [ ] CLAUDE.md documents pytest usage
- [ ] Stub mode documented

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

### System Architect (ADR Documentation - Consultant Priority 3)

#### ARCH-101: Review and approve ADRs in RESEARCH/architecture.md
**Assigned Role:** System Architect
**Priority:** High
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** ARCH-101

**Dependencies:** None (ADRs already drafted per consultant report).

**What to Do:**
- Review ADR-001 (Svelte), ADR-002 (Subprocess), ADR-003 (Python Distribution), ADR-004 (Depth Models)
- Approve or request changes
- Ensure ADRs are linked from docs/architecture.md

**Reference Documents:** RESEARCH/architecture.md; Consultant_Recommendations_Report_6Feb2026.md Priority 3

**Acceptance Criteria:**
- [ ] ADRs reviewed and approved
- [ ] Status updated from "Proposed" to "Accepted" where applicable

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

#### ARCH-102: Document Python distribution strategy for README.md
**Assigned Role:** System Architect
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** ARCH-102

**Dependencies:** ARCH-101 (ADR-003 approved).

**What to Do:**
- Add Python requirements section to README.md
- Document venv setup, pip install, model download
- Reference ADR-003 for rationale

**Reference Documents:** RESEARCH/architecture.md ADR-003; Consultant_Recommendations_Report_6Feb2026.md Priority 5

**Acceptance Criteria:**
- [ ] README.md has clear Python setup instructions
- [ ] Troubleshooting for common issues documented

**Completion Record:**
```
Status: [ ] Complete
Completed By: —
Completed On: —
Notes:
```

---

## Subtask Allocation (multi-role)

| Sub-task | Role | Owner | Status |
|----------|------|-------|--------|
| Where to compute adjustments (Rust vs frontend) | Senior Engineer + UI Designer | Senior Engineer | [ ] |
| Slider semantics and bounds (min/max, step) | UI Designer + Senior Engineer | UI Designer | [ ] |
| DepthControls integration in App.svelte | UI Designer + Junior 3D | UI Designer | [ ] |

---

## Success Criteria for Sprint 1.5

- [ ] All tasks complete per acceptance criteria
- [ ] Exit criteria from todo.md met:
  - [ ] All depth adjustment controls functional
  - [ ] Preview updates within 100 ms of slider change
  - [ ] Reset button restores original depth map
  - [ ] Automated tests cover adjustment logic
  - [ ] UI responsive and intuitive
- [ ] **Consultant Priority 1 (Testing Infrastructure):**
  - [ ] `cargo clippy` enforced in CI (QA-405)
  - [ ] Python pytest suite exists with stub mode tests (AI-401)
  - [ ] pytest runs in CI (AI-402)
- [ ] **Consultant Priority 3 (ADR Documentation):**
  - [ ] ADRs reviewed and approved in RESEARCH/architecture.md (ARCH-101)
  - [ ] Python setup documented in README.md (ARCH-102)
- [ ] No blocking issues
- [ ] Gotchas recorded in `SPRINTS/Sprint_1_5/GOTCHAS.md` (merge to RESEARCH when done)
- [ ] Progress report filed

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| None at sprint start | — | — |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | — |
| cargo clippy | 0 warnings (enforced in CI) | — |
| npm run build | PASS | — |
| pytest (stub mode) | PASS | — |
| Manual test report | Cases executed and recorded | — |
| ADRs approved | All 4 ADRs reviewed | — |

---

## Progress Log (Handover Notes)

*Agents add handover notes when completing tasks that others depend on.*

```
### [Date] — [Role] (Task IDs COMPLETED)
[What was delivered. Key files. Handover to whom.]
```

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **prd.md** — F1.2 depth controls, acceptance criteria
3. **todo.md** — Sprint 1.5
4. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
5. **docs/architecture.md** — Depth map format, frontend implications
6. **RESEARCH/GOTCHAS.md** — Known pitfalls
7. **SPRINTS/Sprint_1_4/** — Layout and depth preview (handover context)

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Prepared by:** Senior Engineer  
**Status:** Ready for role claim and implementation
