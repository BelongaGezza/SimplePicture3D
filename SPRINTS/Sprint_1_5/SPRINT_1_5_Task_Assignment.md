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
| System Architect | `.agents/system-architect.md` | Complete | Cursor-Auto-20260206-ARCH | ARCH-101–102 | ADR review, Python distribution docs (Consultant Priority 3) |
| Senior Engineer | `.agents/senior-engineer.md` | Complete | Cursor-Auto-20260206 | BACK-401–405 | Depth adjustment logic, apply transforms, cache original, reset |
| UI Designer | `.agents/ui-designer.md` | Complete | Cursor-Auto-20260206-UI | UI-401–405 | DepthControls, sliders, Invert, real-time preview, Reset |
| Senior Researcher (AI/ML) | `.agents/researcher.md` | Complete | Cursor-Auto-20260206 | AI-401–403 | pytest suite, CI integration (Consultant Priority 1) |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Complete | Cursor-Auto-20260206-JR2D | JR2-401–403 | Unit tests for adjustments, boundary tests, benchmark |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Complete | Cursor-Auto-20260206-JR3D | JR1-401–404 | Slider styling, numeric inputs, keyboard controls, responsiveness |
| Quality Engineer | (see todo.md) | In Progress | Cursor-Auto-20260206-QA | QA-401–405 | Manual/automated tests, clippy CI (Consultant Priority 1) |
| Security Specialist | `.agents/security-specialist.md` | Complete | Cursor-Auto-20260206-SEC | — | Ad-hoc review + dependency audits; no dedicated 1.5 tasks |
| Documentation Specialist | `.agents/documentation-specialist.md` | Complete | Cursor-Auto-20260206-DOC | — | Supporting docs if needed |

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
| Backend (BACK-401–405) | ✅ Complete | 100% |
| UI (UI-401–405) | ✅ Complete | 100% |
| Junior 2D (JR2-401–403) | ✅ Complete | 100% |
| Junior 3D (JR1-401–404) | ✅ Complete | 100% |
| Quality (QA-401–405) | ⏳ In Progress | 100% task completion; manual report complete for QA-401–403; balance below |
| Testing Infra (AI-401–403) | ✅ Complete | 100% |
| ADR Docs (ARCH-101–102) | ✅ Complete | 100% |

**Overall Sprint Progress:** [x] Not Started / [ ] In Progress / [ ] Complete

**Note:** Tasks AI-401–403, ARCH-101–102, QA-405 added 2026-02-06 per Consultant Recommendations Report.

---

## Manual testing status (2026-02-06)

| Item | Status |
|------|--------|
| **Automated** | All suites run 2026-02-06: cargo test (44 passed), cargo clippy (0 warnings), npm run build (pass), pytest 19 passed. |
| **QA-401** | **Pass** — Adjust all controls, verify preview updates; recorded in MANUAL_TEST_REPORT.md. |
| **QA-402** | **Pass** — Extreme values; no crash; recorded. |
| **QA-403** | **Pass** — Reset restores original depth; recorded. |
| **QA-404** | Automated only (depth_adjust.rs tests in CI). |
| **QA-405** | Clippy in CI (complete). |

**Balance of testing still to perform (optional / sign-off):**

- **Regression (TEST_PLAN_1_5.md §3.4):** **Done.** Load image → Generate depth: Pass. Depth preview (grayscale, zoom/pan) with DepthControls: Pass. *UI note:* Zoom/pan is mouse wheel + drag on depth map; no explicit control or sign that it can be zoomed — consider hint for discoverability (see MANUAL_TEST_REPORT.md).
- **Junior 3D quick checks (TEST_PLAN_1_5.md §3.3):** Slider styling, numeric inputs, arrow keys, responsiveness, Reset — optional; Reset already covered in QA-403.

---

## Task Breakdown

### Senior Engineer

#### BACK-401: Implement depth adjustment functions (brightness, contrast, gamma)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
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
Status: [x] Complete
Completed By: Cursor-Auto-20260206 (Senior Engineer)
Completed On: 2026-02-06
Notes: depth_adjust.rs: brightness, contrast, gamma, invert; formulas in module docs.
```

---

#### BACK-402: Apply transformations to depth map array
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
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
Status: [x] Complete
Completed By: Cursor-Auto-20260206 (Senior Engineer)
Completed On: 2026-02-06
Notes: apply_adjustments() single pass; order: invert → gamma → contrast → brightness. get_depth_map returns adjusted.
```

---

#### BACK-403: Invert depth toggle
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
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
Status: [x] Complete
Completed By: Cursor-Auto-20260206 (Senior Engineer)
Completed On: 2026-02-06
Notes: Invert is first step in pipeline; boolean in DepthAdjustmentParams.
```

---

#### BACK-404: Depth range mapping (0–1 → 2–10mm)
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
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
Status: [x] Complete
Completed By: Cursor-Auto-20260206 (Senior Engineer)
Completed On: 2026-02-06
Notes: depth_min_mm, depth_max_mm in DepthAdjustmentParams; depth_to_mm() in depth_adjust.rs for mesh later.
```

---

#### BACK-405: Cache original depth map for reset function
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
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
Status: [x] Complete
Completed By: Cursor-Auto-20260206 (Senior Engineer)
Completed On: 2026-02-06
Notes: AppState.depth = original; adjustment_params stored; get_depth_map computes on demand; reset_depth_adjustments().
```

---

### UI Designer

#### UI-401: Create DepthControls component (sidebar)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
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
Status: [x] Complete
Completed By: Cursor-Auto-20260206-UI (UI Designer)
Completed On: 2026-02-06
Notes: DepthControls.svelte in right sidebar; disabled when no depth; labels and aria.
```

---

#### UI-402: Implement sliders: Depth Range, Brightness, Gamma
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-402

**Dependencies:** UI-401 (component shell); BACK-401–404 (param semantics).

**What to Do:**
- Implement sliders for Depth Range (e.g. min 2 mm, max 10 mm; or single "range" scale), Brightness (e.g. -0.5 to 0.5 or 0–100%), Gamma (e.g. 0.5–2.0). Bind to adjustment state; on change, update params and trigger preview update (debounced per UI-404).
- Ensure min/max and step are clear; optional tooltips. Coordinate with JR1-402 for numeric input fields next to sliders.

**Reference Documents:** `prd.md` F1.2; `RESEARCH/frontend.md`

**Acceptance Criteria:**
- [ ] Sliders control correct params; range and scale documented
- [ ] Values stay within valid bounds
- [x] Preview updates when sliders change (debounced)

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-UI (UI Designer)
Completed On: 2026-02-06
Notes: Depth min/max (1–20 mm), Brightness (-0.5–0.5), Gamma (0.5–2); numeric inputs beside sliders.
```

---

#### UI-403: Invert Depth checkbox
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-403

**Dependencies:** BACK-403 (invert applied); UI-401 (DepthControls).

**What to Do:**
- Add checkbox "Invert Depth" (or similar). Toggle invert on/off; preview updates (debounced). Accessible label and state.

**Reference Documents:** `prd.md` F1.2; UI-402

**Acceptance Criteria:**
- [ ] Checkbox toggles invert; preview reflects state
- [x] Accessible (label, checked state)

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-UI (UI Designer)
Completed On: 2026-02-06
Notes: "Invert depth" checkbox in DepthControls; preview updates via debounced pipeline.
```

---

#### UI-404: Real-time preview updates (debounced)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-404

**Dependencies:** BACK-402 (adjusted depth available); UI-402, UI-403 (controls emit changes).

**What to Do:**
- When user changes sliders or invert, update depth preview within 100 ms of last change (debounce). If backend computes adjusted depth: call command or read state with new params and re-render. If frontend computes: run adjustment on current depth + params and re-draw canvas. Target: preview updates within 100 ms of slider release or after debounce interval.

**Reference Documents:** `prd.md` exit criteria "Preview updates within 100ms"; BACK-402

**Acceptance Criteria:**
- [ ] Preview updates within 100 ms of slider change (debounced)
- [ ] No excessive IPC or re-renders (e.g. debounce 50–100 ms)
- [x] Smooth UX; no visible lag for 1080p depth

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-UI (UI Designer)
Completed On: 2026-02-06
Notes: 80 ms debounce in App.svelte; setDepthAdjustmentParams + getDepthMap refresh preview.
```

---

#### UI-405: Reset button to restore original depth map
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-405

**Dependencies:** BACK-405 (reset semantics); UI-401 (DepthControls).

**What to Do:**
- Add Reset button in DepthControls. On click: restore adjustment params to defaults (e.g. brightness 0, contrast 1, gamma 1, invert false, range 2–10 mm) and refresh preview from original depth. If backend holds state: invoke reset command or set params to default and refetch. If frontend: set params to default and re-apply to original depth.

**Reference Documents:** `prd.md` F1.2; BACK-405

**Acceptance Criteria:**
- [ ] Reset button restores original depth view (or default params)
- [ ] Sliders/checkbox reflect default state after reset
- [x] Accessible (button label, focus)

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-UI (UI Designer)
Completed On: 2026-02-06
Notes: Reset in DepthControls calls reset_depth_adjustments then refetches params and depth map.
```

---

### Junior Engineer 3D (frontend)

#### JR1-401: Style sliders with TailwindCSS
**Assigned Role:** Junior Engineer 3D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR1-401

**Dependencies:** UI-401, UI-402 (DepthControls and sliders exist).

**What to Do:**
- Style slider controls with Tailwind (or existing design tokens). Consistent with app theme; clear track and thumb; optional labels above/below. Coordinate with UI Designer.

**Reference Documents:** `RESEARCH/frontend.md`; `src/app.css`, existing components

**Acceptance Criteria:**
- [x] Sliders visually consistent with app
- [x] Usable on 1024×768 (prd minimum)

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-JR3D (Junior Engineer 3D)
Completed On: 2026-02-06
Notes: depth-slider class; 16px thumb, 8px track; min-h-8 rows, min-w-0 for flex; cursor-grab/grabbing; track/thumb in style block.
```

---

#### JR1-402: Add numerical input fields next to sliders
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR1-402

**Dependencies:** UI-402 (sliders in place).

**What to Do:**
- Add number inputs beside (or below) each slider so users can type exact values. Keep in sync with slider; validate min/max; same step as slider if applicable.

**Reference Documents:** UI-402; `RESEARCH/frontend.md`

**Acceptance Criteria:**
- [x] Numeric inputs for Depth Range, Brightness, Gamma
- [x] Values stay in bounds; sync with slider

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-JR3D (Junior Engineer 3D)
Completed On: 2026-02-06
Notes: UI-402 already added number inputs beside each slider (Depth min/max, Brightness, Gamma). JR3D verified: same value and handler as slider; handle*Input clamps to min/max; parent props keep slider and number in sync.
```

---

#### JR1-403: Implement keyboard controls (arrow keys for sliders)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR1-403

**Dependencies:** UI-402 (sliders); JR1-401 (slider elements).

**What to Do:**
- When a slider is focused, arrow keys (left/right or up/down) increment/decrement value by step. Document in UI or GOTCHAS if needed.

**Reference Documents:** `prd.md` accessibility; UI-402

**Acceptance Criteria:**
- [x] Arrow keys change slider value when focused
- [x] Step size consistent with slider

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-JR3D (Junior Engineer 3D)
Completed On: 2026-02-06
Notes: handleRangeKeydown() in DepthControls.svelte: ArrowLeft/ArrowDown decrement by step, ArrowRight/ArrowUp increment; value clamped to min/max; step rounded; preventDefault() to avoid page scroll.
```

---

#### JR1-404: Test slider responsiveness
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR1-404

**Dependencies:** UI-402, UI-404 (sliders and debounced preview).

**What to Do:**
- Manually or with a simple test verify sliders respond smoothly; no jank or freeze during drag. Test with 1080p depth if available. Document any limits in GOTCHAS.

**Reference Documents:** `prd.md` §7.1; UI-404

**Acceptance Criteria:**
- [x] Sliders responsive; no blocking main thread
- [x] Results or limits noted in GOTCHAS if needed

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-JR3D (Junior Engineer 3D)
Completed On: 2026-02-06
Notes: Verified debounced preview (80 ms) keeps main thread responsive. SPRINTS/Sprint_1_5/GOTCHAS.md updated with JR1-404 note on responsiveness and optional debounce/worker for very large depth.
```

---

### Junior Engineer 2D (backend / tests)

#### JR2-401: Write unit tests for depth adjustment algorithms
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-401

**Dependencies:** BACK-401 (adjustment functions).

**What to Do:**
- Add Rust (or frontend) unit tests: given a small depth array and params, assert output in [0, 1] and expected behaviour (e.g. brightness +0.2 shifts values; gamma 2 darkens midtones). Cover at least one case per function (brightness, contrast, gamma, invert).

**Reference Documents:** BACK-401; `src-tauri/src/lib.rs` tests

**Acceptance Criteria:**
- [x] Tests exist for brightness, contrast, gamma, invert
- [x] Tests pass in CI; no flake

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-JR2D (Junior Engineer 2D)
Completed On: 2026-02-06
Notes: depth_adjust.rs already had tests for brightness, contrast, gamma, invert, pipeline; JR2-402 added boundary tests. All 17 tests pass.
```

---

#### JR2-402: Test boundary conditions (min/max values)
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-402

**Dependencies:** BACK-401, BACK-402 (adjustments and pipeline).

**What to Do:**
- Test edge cases: all 0, all 1, mixed; extreme params (brightness ±1, gamma 0.1 and 5). Assert no panic, no NaN, output clamped to [0, 1]. Document expected behaviour for boundary params if non-obvious.

**Reference Documents:** BACK-401, BACK-402; RESEARCH/GOTCHAS.md

**Acceptance Criteria:**
- [x] Boundary tests pass; no panic/NaN
- [x] Extreme params documented or clamped

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-JR2D (Junior Engineer 2D)
Completed On: 2026-02-06
Notes: boundary_all_zeros, boundary_all_ones, boundary_mixed_values, boundary_extreme_brightness, boundary_extreme_gamma, boundary_extreme_contrast. All output in [0,1], no NaN.
```

---

#### JR2-403: Benchmark adjustment performance (real-time?)
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR2-403

**Dependencies:** BACK-402 (full pipeline).

**What to Do:**
- Benchmark application of all adjustments to a 1920×1080 depth array (or largest target size). Target: complete in <100 ms so real-time preview is feasible. Document in GOTCHAS or test plan; report CPU time.

**Reference Documents:** `prd.md` §7.1; UI-404 (100 ms target); RESEARCH/GOTCHAS.md

**Acceptance Criteria:**
- [x] Benchmark run for at least 1080p depth
- [x] Result documented; <100 ms or gap explained

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-JR2D (Junior Engineer 2D)
Completed On: 2026-02-06
Notes: src-tauri/benches/depth_adjust.rs (criterion). cargo bench --bench depth_adjust: ~14 ms per call (1920×1080). RESEARCH/GOTCHAS.md updated with result; well under 100 ms target.
```

---

### Quality Engineer

#### QA-401: Manual test: adjust all controls, verify preview updates
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-401

**Dependencies:** BACK-401–405, UI-401–405 (full flow).

**What to Do:**
- Execute manual test: load image, generate depth, then change each slider and Invert; verify preview updates. Reset and verify restoration. Record in MANUAL_TEST_REPORT.md.

**Reference Documents:** `SPRINTS/Sprint_1_5/TEST_PLAN_1_5.md`, `MANUAL_TEST_REPORT.md`

**Acceptance Criteria:**
- [x] All controls exercised; preview matches expectations
- [x] Results in manual test report

**Completion Record:**
```
Status: [x] Complete
Completed By: User + Cursor-Auto-20260206-QA (Quality Engineer)
Completed On: 2026-02-06
Notes: Case 1 in MANUAL_TEST_REPORT.md — all controls update preview within ~100 ms; Reset restores defaults. Pass.
```

---

#### QA-402: Test extreme values (brightness 0%, 200%)
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-402

**Dependencies:** UI-402 (sliders); BACK-401 (clamping).

**What to Do:**
- Set brightness (and other params) to extreme min/max. Verify no crash, no corrupt display; depth stays in valid range or UI prevents invalid input. Document in manual test report.

**Reference Documents:** JR2-402; TEST_PLAN_1_5.md

**Acceptance Criteria:**
- [x] Extreme values handled; no crash
- [x] Result in manual test report

**Completion Record:**
```
Status: [x] Complete
Completed By: User + Cursor-Auto-20260206-QA (Quality Engineer)
Completed On: 2026-02-06
Notes: Case 2 in MANUAL_TEST_REPORT.md — Brightness/Gamma/Depth range extremes; no crash, valid display. Pass.
```

---

#### QA-403: Verify reset button restores original depth map
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-403

**Dependencies:** BACK-405, UI-405 (reset implemented).

**What to Do:**
- After applying several adjustments, click Reset. Verify preview matches the initial depth (before any slider changes). Compare visually or via exported/serialized data if available. Record in manual test report.

**Reference Documents:** BACK-405; UI-405; TEST_PLAN_1_5.md

**Acceptance Criteria:**
- [x] Reset restores original depth view
- [x] Pass/fail in manual test report

**Completion Record:**
```
Status: [x] Complete
Completed By: User + Cursor-Auto-20260206-QA (Quality Engineer)
Completed On: 2026-02-06
Notes: Case 3 in MANUAL_TEST_REPORT.md — Reset restored original depth and default controls. Pass.
```

---

#### QA-404: Automated test: apply adjustments, check output array
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-404

**Dependencies:** BACK-401–402 (adjustments and pipeline); JR2-401 (unit tests).

**What to Do:**
- Add automated test: feed known depth array + params, get adjusted array; assert all values in [0, 1], and (optional) assert expected value at a few indices for a known formula. Can be Rust unit test or frontend test. Document in test plan.

**Reference Documents:** JR2-401; TEST_PLAN_1_5.md

**Acceptance Criteria:**
- [x] Automated test exists; asserts output in [0, 1] and optional sanity checks
- [x] Runs in CI

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-QA (Quality Engineer)
Completed On: 2026-02-06
Notes: Covered by src-tauri/src/depth_adjust.rs tests: pipeline_output_in_0_1, pipeline_default_is_identity, pipeline_invert_only, boundary_* (all_zeros, all_ones, mixed_values, extreme brightness/gamma/contrast). CI runs cargo test.
```

---

#### QA-405: Add cargo clippy to CI (Consultant Priority 1)
**Assigned Role:** Quality Engineer
**Priority:** Critical
**Status:** [x] Complete
**Task ID:** QA-405

**Dependencies:** None (standalone CI enhancement).

**What to Do:**
- Add `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` step to `.github/workflows/ci.yml` backend job
- Ensure job fails on any clippy warnings
- Fix any existing clippy warnings that surface

**Reference Documents:** Consultant_Recommendations_Report_6Feb2026.md Priority 1; todo.md Testing requirements

**Acceptance Criteria:**
- [x] CI runs clippy and fails on warnings
- [x] All existing code passes clippy

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-QA (Quality Engineer)
Completed On: 2026-02-06
Notes: Added "Clippy" step to .github/workflows/ci.yml backend job (cargo clippy -- -D warnings). Verified locally: 0 warnings.
```

---

### Senior Researcher (Testing Infrastructure - Consultant Priority 1)

#### AI-401: Create pytest suite for depth_estimator.py
**Assigned Role:** Senior Researcher
**Priority:** Critical
**Status:** [x] Complete
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
Status: [x] Complete
Completed By: Cursor-Auto-20260206 (Senior Researcher)
Completed On: 2026-02-06
Notes: python/tests/test_depth_estimator.py + conftest.py; 19 tests (stub, CLI, clamp, load_image).
```

---

#### AI-402: Add pytest to CI workflow
**Assigned Role:** Senior Researcher
**Priority:** High
**Status:** [x] Complete
**Task ID:** AI-402

**Dependencies:** AI-401 (pytest suite exists).

**What to Do:**
- Add Python job to `.github/workflows/ci.yml`
- Install Python 3.10+, pip install pytest
- Run `SP3D_USE_STUB=1 pytest python/`
- Ensure CI fails if pytest fails

**Reference Documents:** AI-401; Consultant_Recommendations_Report_6Feb2026.md

**Acceptance Criteria:**
- [x] CI runs pytest on every push/PR
- [x] Uses stub mode (no model download in CI)
- [x] Failures block merge

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206 (Senior Researcher)
Completed On: 2026-02-06
Notes: .github/workflows/ci.yml — python job, Python 3.10, pytest + Pillow only.
```

---

#### AI-403: Document Python test commands in CLAUDE.md
**Assigned Role:** Senior Researcher
**Priority:** Medium
**Status:** [x] Complete
**Task ID:** AI-403

**Dependencies:** AI-401 (pytest suite exists).

**What to Do:**
- Update CLAUDE.md Testing Commands section with pytest commands
- Document SP3D_USE_STUB environment variable
- Add to README.md if appropriate

**Reference Documents:** CLAUDE.md; AI-401

**Acceptance Criteria:**
- [x] CLAUDE.md documents pytest usage
- [x] Stub mode documented

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206 (Senior Researcher)
Completed On: 2026-02-06
Notes: CLAUDE.md Testing Commands + stub note; README.md Testing section updated.
```

---

### System Architect (ADR Documentation - Consultant Priority 3)

#### ARCH-101: Review and approve ADRs in RESEARCH/architecture.md
**Assigned Role:** System Architect
**Priority:** High
**Status:** [x] Complete
**Task ID:** ARCH-101

**Dependencies:** None (ADRs already drafted per consultant report).

**What to Do:**
- Review ADR-001 (Svelte), ADR-002 (Subprocess), ADR-003 (Python Distribution), ADR-004 (Depth Models)
- Approve or request changes
- Ensure ADRs are linked from docs/architecture.md

**Reference Documents:** RESEARCH/architecture.md; Consultant_Recommendations_Report_6Feb2026.md Priority 3

**Acceptance Criteria:**
- [x] ADRs reviewed and approved
- [x] Status updated from "Proposed" to "Accepted" where applicable

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-ARCH (System Architect)
Completed On: 2026-02-06
Notes: ADR-003, ADR-004 set to Accepted. docs/architecture.md updated with link to RESEARCH/architecture.md and ADR list.
```

---

#### ARCH-102: Document Python distribution strategy for README.md
**Assigned Role:** System Architect
**Priority:** Medium
**Status:** [x] Complete
**Task ID:** ARCH-102

**Dependencies:** ARCH-101 (ADR-003 approved).

**What to Do:**
- Add Python requirements section to README.md
- Document venv setup, pip install, model download
- Reference ADR-003 for rationale

**Reference Documents:** RESEARCH/architecture.md ADR-003; Consultant_Recommendations_Report_6Feb2026.md Priority 5

**Acceptance Criteria:**
- [x] README.md has clear Python setup instructions
- [x] Troubleshooting for common issues documented

**Completion Record:**
```
Status: [x] Complete
Completed By: Cursor-Auto-20260206-ARCH (System Architect)
Completed On: 2026-02-06
Notes: README Development Setup: Python 3.10+ in tools table; new "Python environment (AI backend)" section (venv, pip, stub vs real inference, model download, troubleshooting, ADR-003 link).
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
### 2026-02-06 — UI Designer (UI-401–405 COMPLETED)
- Added src/components/DepthControls.svelte: sliders for Depth min/max (mm), Brightness, Gamma; Invert checkbox; Reset button; numeric inputs beside sliders; disabled when no depth. Tailwind styling and aria labels.
- Updated src/App.svelte: adjustmentParams state; getDepthAdjustmentParams/setDepthAdjustmentParams/resetDepthAdjustments; 80 ms debounce for preview (set params → get_depth_map → update depthMap). DepthControls in right sidebar below Generate button.
- Updated src/lib/tauri.ts: DepthAdjustmentParams type and getDepthAdjustmentParams, setDepthAdjustmentParams, resetDepthAdjustments.
- Handover to: Junior 3D (JR1-401–404 styling/keyboard/responsiveness), Junior 2D (JR2-401–403 tests), QA (QA-401–405).

### 2026-02-06 — Junior Engineer 2D (JR2-401–403 COMPLETED)
- JR2-401: Unit tests already present in depth_adjust.rs for brightness, contrast, gamma, invert and pipeline; all 17 tests pass.
- JR2-402: Added boundary tests: boundary_all_zeros, boundary_all_ones, boundary_mixed_values, boundary_extreme_brightness, boundary_extreme_gamma, boundary_extreme_contrast. No panic/NaN; output clamped to [0, 1].
- JR2-403: Added src-tauri/benches/depth_adjust.rs (criterion). cargo bench: ~14 ms per call for 1920×1080 (target <100 ms). RESEARCH/GOTCHAS.md updated with benchmark result.
- lib.rs: depth_adjust made pub for benchmark. Cargo.toml: dev-dependencies criterion, [[bench]] depth_adjust.

### 2026-02-06 — Senior Engineer (BACK-401–405 COMPLETED)
- Added src-tauri/src/depth_adjust.rs: brightness, contrast, gamma, invert; apply_adjustments() in order invert→gamma→contrast→brightness; DepthAdjustmentParams (includes depth_min_mm, depth_max_mm); depth_to_mm() for future mesh.
- Extended AppState: depth (original), adjustment_params. get_depth_map returns adjusted view (on demand); set_depth_adjustment_params, get_depth_adjustment_params, reset_depth_adjustments. generate_depth_map stores original and returns response with current params applied.
- docs/architecture.md: depth adjustment API and pipeline order. Permissions: allow-generate-depth-map now includes set/get/reset adjustment commands.
- Handover to: UI Designer (UI-401–405), Junior 2D (JR2-401–403 for unit tests), Junior 3D (JR1-401–404).

### 2026-02-06 — Senior Researcher (AI-401, AI-402, AI-403 COMPLETED)
- python/tests/test_depth_estimator.py: 19 tests (clamp_depth_to_01, run_inference_stub, load_image_dimensions, CLI stub/--no-model, error cases). conftest.py sets PYTHONPATH for depth_estimator import.
- .github/workflows/ci.yml: new "python" job (Python 3.10, pytest + Pillow, SP3D_USE_STUB=1, PYTHONPATH=python/python).
- CLAUDE.md: Testing Commands updated with pytest and SP3D_USE_STUB; README.md Testing section updated.
- No handover; testing infra is standalone.

### 2026-02-06 — System Architect (ARCH-101, ARCH-102 COMPLETED)
- ARCH-101: ADR-003 and ADR-004 status set to Accepted in RESEARCH/architecture.md. docs/architecture.md updated with explicit link to ADRs (ADR-001–004).
- ARCH-102: README.md — Python 3.10+ in required tools table; new "Python environment (AI backend)" section (venv, pip install, stub vs real inference, model download, troubleshooting, ADR-003 reference).
- No handover; ADR/docs work is standalone.

### 2026-02-06 — Documentation Specialist (Supporting docs COMPLETED)
- Claimed Documentation Specialist role (Cursor-Auto-20260206-DOC).
- Created docs/user-guide.md: Installation pointer, First conversion flow, **Depth Controls** section (location, controls table, preview behaviour, Reset, tips, FAQ stub). Aligned with DepthControls.svelte and docs/architecture.md.
- Updated SPRINTS/Sprint_1_5/VERIFICATION_CHECKLIST.md: added quality gate "Depth controls documented for users (docs/user-guide.md § Depth Controls)"; clarified QA-405 (clippy in CI) and QA-401–403 manual cases; Last Updated 2026-02-06.
- No dedicated 1.5 task IDs; supporting documentation for sprint deliverable.
- Handover: none; doc role is supporting.

### 2026-02-06 — Quality Engineer (QA-404, QA-405 COMPLETED)
- Claimed Quality Engineer role (Cursor-Auto-20260206-QA).
- QA-405: Added "Clippy" step to .github/workflows/ci.yml backend job (cargo clippy -- -D warnings). Verified 0 warnings locally.
- QA-404: Automated adjustment tests in src-tauri/src/depth_adjust.rs (pipeline_output_in_0_1, boundary_*, etc.); documented in TEST_PLAN_1_5.md; runs in CI via cargo test.
- Created SPRINTS/Sprint_1_5/MANUAL_TEST_REPORT.md with placeholders for QA-401, QA-402, QA-403 (manual execution when DepthControls/UI-401–405 available).
- Handover: QA-401–403 to be executed once UI Designer completes UI-401–405.

### 2026-02-06 — Security Specialist (Sprint 1.5 ad-hoc COMPLETED)
- Claimed Security Specialist role (Cursor-Auto-20260206-SEC). No dedicated 1.5 tasks; performed ad-hoc security review.
- Reviewed: (1) New Tauri commands set_depth_adjustment_params, get_depth_adjustment_params, reset_depth_adjustments — accept/return only DepthAdjustmentParams (f32, bool); no path or user-controlled strings. (2) depth_adjust.rs — pure math, no I/O or paths. (3) Permissions in allow-generate-depth-map.toml — new commands correctly gated. (4) Path/magic-byte validation unchanged in image_loading.rs for generate_depth_map path. No issues found. Recorded in SPRINTS/Sprint_1_5/GOTCHAS.md.

### 2026-02-06 — Security Specialist (Ad-hoc Sprint 1.5 COMPLETED)
- Claimed Security Specialist role (Cursor-Auto-20260206-SEC). No dedicated 1.5 task IDs; performed ad-hoc review and dependency audits per persona.
- IPC review: New commands (set/get/reset depth adjustment params) accept only DepthAdjustmentParams (f32 + bool); no path or string injection. depth_adjust.rs is pure math; permissions scoped in allow-generate-depth-map.toml.
- Audits: cargo audit — 2 vulnerabilities (bytes 1.11.0, time 0.3.46); 19 allowed warnings (unmaintained/unsound in Tauri/wry tree). npm audit — 5 moderate (esbuild/vite dev server). Documented in SPRINTS/Sprint_1_5/GOTCHAS.md; recommend addressing Rust vulns when Tauri bumps deps, track npm on next frontend update.
- Handover: none; role complete.

### 2026-02-06 — Quality Engineer (QA-401, QA-402, QA-403 COMPLETED — manual)
- Ran all automated suites: cargo test (44 passed), cargo clippy (0 warnings), npm run build (pass), pytest 19 passed (stub). Recorded in MANUAL_TEST_REPORT.md.
- Executed manual Cases 1–3 with user: QA-401 (all controls + preview + Reset) Pass; QA-402 (extreme values) Pass; QA-403 (Reset restores original) Pass.
- Updated MANUAL_TEST_REPORT.md summary and case results; updated this task assignment with completion records and "Manual testing status" / "Balance of testing still to perform".
- Balance: regression (§3.4) and JR1 quick checks (§3.3) optional for sprint sign-off.

### 2026-02-06 — Quality Engineer (Regression §3.4 COMPLETED)
- Regression: Load image → Generate depth (Sprint 1.4 flow) Pass; Depth preview (grayscale, zoom/pan) with DepthControls Pass.
- **Handover to UI Designer:** Zoom/pan on depth preview is mouse wheel + drag only; no explicit control or on-screen hint. Consider adding discoverability (tooltip, label, or icon) — see MANUAL_TEST_REPORT.md Regression section.

### 2026-02-06 — Junior Engineer 3D (JR1-401–404 COMPLETED)
- JR1-401: DepthControls sliders styled with .depth-slider (16px thumb, 8px track), min-h-8 rows, min-w-0, cursor-grab/grabbing; consistent with app theme and 1024×768.
- JR1-402: Verified numeric inputs beside each slider (from UI-402); sync and bounds confirmed.
- JR1-403: handleRangeKeydown() added for all four range inputs; ArrowLeft/ArrowDown decrement, ArrowRight/ArrowUp increment by step; preventDefault to avoid scroll.
- JR1-404: GOTCHAS.md updated with slider responsiveness note; debounce keeps main thread responsive.
- No handover; frontend polish complete.
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
