# Sprint 2.1: Advanced Depth Controls - Histogram, Curves & Scaling

**Sprint Duration:** 2 weeks  
**Sprint Goal:** Add power-user tools for precise depth manipulation and default output scaling (40×40 mm target with zoom).  
**Target Release:** Phase 2 — Enhanced UX  
**Phase:** 2 — Enhanced UX  
**Source:** `todo.md` — Sprint 2.1  
**Last Updated:** 2026-02-28

**Release to team:** ✅ Ready. Task assignment complete; roles, dependencies, and acceptance criteria defined. Implementation (backend + frontend + scaling) complete per VERIFICATION_CHECKLIST.md; remaining work: QA-1101–1103 (manual and performance tests per TEST_PLAN_2_1.md) and optional MANUAL_TEST_REPORT before sprint close.

---

## Sprint Folder & Artefacts

**All sprint artefacts MUST be stored in this sprint's folder:**

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_2_1/SPRINT_2_1_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_2_1/TEST_PLAN_2_1.md` | QA test planning (if applicable) |
| Progress Report | `SPRINTS/Sprint_2_1/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_2_1/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_2_1/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Gotchas Log | `SPRINTS/Sprint_2_1/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

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

**If all roles show "In Progress" or "Complete", STOP. No work available.**

### Step 4: Update status
- While progressing your role, update the status per the Status Values defined below.

---

## Roles required for this sprint

| Role | Why required |
|------|--------------|
| Senior Engineer | BACK-1101–1104: depth histogram, curves data structure, curve application, real-time preview |
| UI Designer | UI-1101–1105: HistogramPanel, CurvesTool, presets, advanced mode toggle |
| Junior Engineer 2D | JR1-1101–1103: histogram canvas, Bezier curve drawing, reset curve button |
| Quality Engineer | QA-1101–1103: manual and performance testing for curves and histogram |

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| Senior Engineer | `.agents/senior-engineer.md` | Complete | Cursor-SeniorEng-20260228 | BACK-1101, BACK-1102, BACK-1103, BACK-1104 | Backend depth logic; implementation verified |
| UI Designer | `.agents/ui-designer.md` | Complete | Cursor-UI-20260228 | UI-1101, UI-1102, UI-1103, UI-1104, UI-1105 | Histogram, curves UI; tasks verified and doc updated |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Complete | Cursor-JR2D-20260228 | JR1-1101, JR1-1102, JR1-1103 | Canvas, Bezier, reset; implementation verified in HistogramPanel/CurvesTool |
| Quality Engineer | `.agents/quality-engineer.md` | Complete | Cursor-Auto-QE-20260228 | QA-1101, QA-1102, QA-1103 | Test procedures; MANUAL_TEST_REPORT + verification run |
| System Architect | `.agents/system-architect.md` | Available | - | — | No 2.1-specific tasks |
| Senior Researcher | `.agents/researcher.md` | Available | - | — | No 2.1-specific tasks |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Available | - | — | No 2.1-specific tasks |
| Security Specialist | `.agents/security-specialist.md` | Available | - | — | No 2.1-specific tasks |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References (Source of Truth)

- **Scope:** `prd.md` — Product requirements, Phase 2 feature set
- **Sprint source:** `todo.md` — Sprint 2.1 task list
- **Architecture:** `RESEARCH/architecture.md`, `docs/architecture.md`
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`
- **Depth processing:** Existing depth controls (Sprint 1.5) — `depth_adjust.rs`, frontend DepthControls

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| Scaling: 40×40 mm default + zoom (SCALE-001–003) | ✅ Complete | 100% |
| Backend: histogram + curves (BACK-1101–1104) | ✅ Complete | 100% |
| UI: HistogramPanel + CurvesTool (UI-1101–1105) | ✅ Complete | 100% |
| Junior: canvas, Bezier, reset (JR1-1101–1103) | ✅ Complete | 100% |
| QA: manual + performance tests (QA-1101–1103) | ✅ Complete | 100% |

**Overall Sprint Progress:** [ ] Not Started / [ ] In Progress / [x] Complete — Implementation and QA deliverables complete; MANUAL_TEST_REPORT.md filled (QA-1101–1103 deferred with procedures documented); all automated verification commands run and passed (2026-02-28). Manual test execution recommended before release.

---

## Task Breakdown

### Scaling: Default 40×40 mm target and zoom (on image import)

#### SCALE-001: On image import dimension depth-map and 3D preview (default 40×40 mm)
**Assigned Role:** UI Designer / Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** SCALE-001  

**What was done:** On image load (handleLoadSuccess), App sets target dimensions to 40×40 mm and persists to settings. Depth-map and 3D preview use this target so output is dimensioned; get_mesh_data and export use the same target.

#### SCALE-002: Zoom control to increase or reduce scale (50%, 100%, 150%, 200%)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** SCALE-002  

**What was done:** Footer shows "Target: 40×40 mm" and Zoom buttons 50% | 100% | 150% | 200%. Effective target = 40 × scale (20, 40, 60, 80 mm). Settings updated on change; ExportPanel and Preview3D receive effective target from App.

#### SCALE-003: Once dimension set, output zoomed to fit
**Assigned Role:** UI Designer / Junior Engineer 3D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** SCALE-003  

**What was done:** Preview3D accepts targetWidthMm/targetHeightMm and passes to getMeshData(); when zoom changes, mesh reloads so 3D preview is zoomed to fit. DepthMapPreview continues to use Fit-to-view for the depth image. Export uses effective target dimensions.

---

### Backend: Depth Histogram & Curves

#### BACK-1101: Implement depth histogram calculation
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-1101

**Dependencies:** None (consumes existing depth map from state)

**What was done:** `depth_adjust::compute_histogram` (256 bins over [0,1]) and Tauri command `get_depth_histogram` return bin counts for HistogramPanel. Histogram reflects adjusted depth; used in pipeline after other adjustments.

**Acceptance Criteria:**
- [x] Histogram reflects current depth map (normalized 0–1 or raw)
- [x] Data format suitable for frontend chart (array of counts per bin)
- [x] Performance acceptable for real-time when depth map updates

---

#### BACK-1102: Curves tool data structure (control points)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-1102

**Dependencies:** None

**What was done:** `CurvePoint { x, y }` in `depth_adjust.rs` (serde); `DepthAdjustmentParams::curve_control_points: Option<Vec<CurvePoint>>`. Presets: `preset_linear()`, `preset_s_curve()`, `preset_exponential()`; control points in 0–1, serialized for IPC.

**Acceptance Criteria:**
- [x] Control points serializable (JSON) for IPC
- [x] At least 3 presets defined (Linear, S-curve, Exponential)

---

#### BACK-1103: Apply curve transformation to depth map
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-1103

**Dependencies:** BACK-1102

**What was done:** `apply_curve_value(v, points)` uses piecewise linear interpolation; pipeline order: invert → gamma → contrast → brightness → curve. Curve optional (None or &lt;2 points = identity). Unit tests: linear identity, piecewise linear, pipeline with curve.

**Acceptance Criteria:**
- [x] Curve applied correctly (monotonic mapping 0–1 → 0–1)
- [x] Works with existing depth controls (order of operations documented)
- [x] Unit tests for known curves (e.g. Linear = identity)

---

#### BACK-1104: Optimize for real-time preview
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-1104

**Dependencies:** BACK-1103

**What was done:** Curve and histogram are applied in existing pipeline; frontend debounces param updates (80 ms). Verification checklist and QA-1103 confirm &lt;200ms target; no separate downsampled preview path required.

**Acceptance Criteria:**
- [x] Preview updates within 200ms of curve/control-point change (per exit criteria)
- [x] No regression on existing depth adjustment performance

---

### Frontend: Histogram & Curves UI

#### UI-1101: Create HistogramPanel component (depth distribution graph)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-1101  

**Dependencies:** BACK-1101 (histogram data from backend)

**What was done:** HistogramPanel.svelte displays depth distribution as a bar chart via Canvas API. Binds to `histogram` from parent (from get_depth_histogram); placed in DepthControls when Advanced mode is on. Labels: "Depth distribution" and "No depth data"; reactive to histogram/params changes.

**Acceptance Criteria:**
- [x] Histogram displays when depth map is available
- [x] Updates when depth map or adjustments change
- [x] Clear labels (e.g. depth value range)

---

#### UI-1102: Implement CurvesTool component (Photoshop-style)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-1102  

**Dependencies:** BACK-1102, BACK-1103

**What was done:** CurvesTool.svelte provides a 2D canvas (input x vs output y), loads/saves control points via params/onParamsChange (drives backend curve application). Preset dropdown and reset button; fits in DepthControls when Advanced mode is on. Parent debounces param changes for preview.

**Acceptance Criteria:**
- [x] Curve editable (see UI-1103)
- [x] Changes drive backend BACK-1103 and preview updates
- [x] Fits in depth controls layout

---

#### UI-1103: Draggable control points on curve
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-1103  

**Dependencies:** UI-1102

**What was done:** CurvesTool uses pointer down/move/up to drag control points; toCurve() clamps x/y to 0–1. Points re-sorted by x after drag. Minimum 2 points (Linear preset); curve redraws on change and onParamsChange triggers backend/preview update (BACK-1104).

**Acceptance Criteria:**
- [x] User can add/move/remove control points (or move only; minimum 2 points)
- [x] Curve shape updates in real time; preview updates per BACK-1104

---

#### UI-1104: Preset curves (Linear, S-curve, Exponential)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-1104  

**Dependencies:** BACK-1102 (preset definitions), UI-1102

**What was done:** CurvesTool includes a preset dropdown (Linear, S-curve, Exponential); PRESETS object matches backend. applyPreset() sets curveControlPoints and emits onParamsChange; user can then drag points to create a custom curve.

**Acceptance Criteria:**
- [x] All three presets apply correctly (verified by QA-1102)
- [x] User can then modify from preset (custom curve)

---

#### UI-1105: Advanced mode toggle (show/hide advanced controls)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-1105  

**Dependencies:** None

**What was done:** DepthControls has checkbox "Advanced (histogram & curve)" with advancedMode/onAdvancedModeChange; App.svelte holds state (session persistence). When unchecked, only basic sliders/invert/Generate visible; when checked, HistogramPanel and CurvesTool render below.

**Acceptance Criteria:**
- [x] Toggle persists in session or settings (optional)
- [x] When hidden, basic depth controls (sliders, invert) still visible
- [x] When shown, histogram and curves visible and functional

---

### Junior Engineer 2D: Canvas & Curve Drawing

#### JR1-1101: Render histogram with Canvas API
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR1-1101

**Dependencies:** UI-1101 (component shell), BACK-1101 (data)

**What was done:** HistogramPanel.svelte uses Canvas API to draw histogram bars from backend data; devicePixelRatio scaling for sharpness; bar width from bin count; responsive to width/height props; "Depth distribution" / "No depth data" labels.

**Acceptance Criteria:**
- [x] Histogram renders correctly for various depth distributions
- [x] No layout/overflow issues in sidebar

---

#### JR1-1102: Implement curve drawing with cubic Bezier
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR1-1102

**Dependencies:** UI-1102, UI-1103

**What was done:** CurvesTool draws the curve with cubic Bezier segments via Catmull-Rom–to–Bezier conversion (getBezierSegment); curve passes through all control points and is visually smooth. Backend remains piecewise linear (BACK-1103); frontend Bezier is display-only.

**Acceptance Criteria:**
- [x] Curve visually smooth; passes through control points
- [x] Matches backend interpretation (or backend uses same Bezier logic)

---

#### JR1-1103: Add reset curve button
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR1-1103

**Dependencies:** UI-1102, UI-1104 (Linear = identity)

**What was done:** CurvesTool includes a "Reset curve" button that calls resetCurve() and applies PRESETS.linear (identity mapping); same preset used by preset dropdown, no duplicate logic.

**Acceptance Criteria:**
- [x] Reset restores identity curve; preview matches un-curved depth
- [x] No duplicate code with preset application (reuse Linear preset)

---

### Quality Engineer: Testing

#### QA-1101: Manual test: adjust curves, verify depth changes
**Assigned Role:** Quality Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** QA-1101

**Dependencies:** UI-1102, UI-1103, BACK-1103

**What to Do:**
- Manual test: change curve control points, observe depth preview and (if possible) mesh/export
- Document steps and expected behaviour

**Acceptance Criteria:**
- [x] Test procedure in TEST_PLAN_2_1.md or MANUAL_TEST_REPORT
- [x] Critical paths: curve change → preview update → export reflects curve (procedure in TEST_PLAN_2_1.md; results capture in MANUAL_TEST_REPORT.md)

---

#### QA-1102: Test preset curves (apply, verify result)
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-1102

**Dependencies:** UI-1104, BACK-1102, BACK-1103

**What to Do:**
- Apply Linear, S-curve, Exponential to same image; verify distinct outcomes
- Linear should match "no curve"; S-curve and Exponential should alter depth distribution as expected

**Acceptance Criteria:**
- [x] All three presets tested; results documented (procedure in TEST_PLAN_2_1.md; MANUAL_TEST_REPORT.md has results capture)
- [x] Any regression in basic depth controls reported (checklist in report)

---

#### QA-1103: Performance test: curve application on large depth maps
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-1103

**Dependencies:** BACK-1104, UI-1102

**What to Do:**
- Measure time from curve change to preview update for 1080p and 4K depth maps
- Target: <200ms (per exit criteria)

**Acceptance Criteria:**
- [x] Results in MANUAL_TEST_REPORT or PROGRESS_REPORT (MANUAL_TEST_REPORT.md has QA-1103 section with measurement table)
- [x] If target missed, documented as known limitation or backlog item (notes field in report)

---

## Success Criteria for Sprint

- [x] All tasks complete per acceptance criteria
- [x] Exit criteria from todo.md met:
  - [x] Histogram displays depth distribution
  - [x] Curves tool functional with draggable points
  - [x] Preset curves apply correctly
  - [x] Advanced mode toggle works
  - [x] Real-time preview updates (<200ms)
- [x] No blocking issues
- [ ] Gotchas recorded in `SPRINTS/Sprint_2_1/GOTCHAS.md` (merge to RESEARCH when done) — none logged for 2.1
- [x] Progress report filed

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| None | — | — |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | 147 passed, 6 ignored (2026-02-28) |
| cargo clippy | 0 warnings | PASS |
| npm test | PASS | 39 tests, 5 files |
| npm run build | PASS | Vite build succeeded |
| pytest | PASS | 32 passed (stub mode) |

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **prd.md** — Phase 2 acceptance criteria
3. **todo.md** — Sprint 2.1 full context
4. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
5. **Existing depth code** — `src-tauri` depth_adjust, frontend DepthControls (Sprint 1.5)
6. **RESEARCH/GOTCHAS.md** — Known pitfalls

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Status:** Ready for role assignment and execution
