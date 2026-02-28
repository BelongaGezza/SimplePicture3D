# Sprint 2.1: Advanced Depth Controls - Histogram, Curves & Scaling

**Sprint Duration:** 2 weeks  
**Sprint Goal:** Add power-user tools for precise depth manipulation and default output scaling (40×40 mm target with zoom).  
**Target Release:** Phase 2 — Enhanced UX  
**Phase:** 2 — Enhanced UX  
**Source:** `todo.md` — Sprint 2.1  
**Last Updated:** 2026-02-28

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
| Senior Engineer | `.agents/senior-engineer.md` | Available | - | BACK-1101, BACK-1102, BACK-1103, BACK-1104 | Backend depth logic |
| UI Designer | `.agents/ui-designer.md` | Available | - | UI-1101, UI-1102, UI-1103, UI-1104, UI-1105 | Histogram, curves UI |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Available | - | JR1-1101, JR1-1102, JR1-1103 | Canvas, Bezier, reset |
| Quality Engineer | `.agents/junior-engineer-2d.md` | Available | - | QA-1101, QA-1102, QA-1103 | Test procedures |
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
| Backend: histogram + curves (BACK-1101–1104) | ✅ Complete | 100% |
| UI: HistogramPanel + CurvesTool (UI-1101–1105) | ✅ Complete | 100% |
| Junior: canvas, Bezier, reset (JR1-1101–1103) | ✅ Complete | 100% |
| Scaling: 40×40 mm default + zoom (SCALE-001–003) | ✅ Complete | 100% |
| QA: manual + performance tests (QA-1101–1103) | ⏳ Not Started | 0% |

**Overall Sprint Progress:** [x] Not Started / [ ] In Progress / [ ] Complete

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
**Status:** [ ] Not Started  
**Task ID:** BACK-1101

**Dependencies:** None (consumes existing depth map from state)

**What to Do:**
- Compute histogram of depth values (buckets, e.g. 256 bins)
- Return histogram data to frontend for HistogramPanel (e.g. Tauri command or part of depth state)

**Acceptance Criteria:**
- [ ] Histogram reflects current depth map (normalized 0–1 or raw)
- [ ] Data format suitable for frontend chart (array of counts per bin)
- [ ] Performance acceptable for real-time when depth map updates

---

#### BACK-1102: Curves tool data structure (control points)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [ ] Not Started  
**Task ID:** BACK-1102

**Dependencies:** None

**What to Do:**
- Define data structure for curve control points (e.g. 2–16 points, x/y in 0–1)
- Persist in app state / settings if desired
- Support preset curves (Linear, S-curve, Exponential) as named control-point sets

**Acceptance Criteria:**
- [ ] Control points serializable (JSON) for IPC
- [ ] At least 3 presets defined (Linear, S-curve, Exponential)

---

#### BACK-1103: Apply curve transformation to depth map
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [ ] Not Started  
**Task ID:** BACK-1103

**Dependencies:** BACK-1102

**What to Do:**
- Implement curve remapping: for each depth value, apply curve (control points → interpolated curve) to produce new value
- Integrate with existing depth adjustment pipeline (gamma, range, invert)
- Return adjusted depth map for preview and downstream mesh

**Acceptance Criteria:**
- [ ] Curve applied correctly (monotonic mapping 0–1 → 0–1)
- [ ] Works with existing depth controls (order of operations documented)
- [ ] Unit tests for known curves (e.g. Linear = identity)

---

#### BACK-1104: Optimize for real-time preview
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [ ] Not Started  
**Task ID:** BACK-1104

**Dependencies:** BACK-1103

**What to Do:**
- Ensure curve application + histogram recompute meet <200ms for typical depth map sizes (e.g. 1080p)
- Consider downsampled preview path if needed

**Acceptance Criteria:**
- [ ] Preview updates within 200ms of curve/control-point change (per exit criteria)
- [ ] No regression on existing depth adjustment performance

---

### Frontend: Histogram & Curves UI

#### UI-1101: Create HistogramPanel component (depth distribution graph)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [ ] Not Started  
**Task ID:** UI-1101

**Dependencies:** BACK-1101 (histogram data from backend)

**What to Do:**
- New component that displays depth distribution (e.g. bar chart or line)
- Bind to histogram data from Tauri command or state
- Place in depth controls area (sidebar or tab)

**Acceptance Criteria:**
- [ ] Histogram displays when depth map is available
- [ ] Updates when depth map or adjustments change
- [ ] Clear labels (e.g. depth value range)

---

#### UI-1102: Implement CurvesTool component (Photoshop-style)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [ ] Not Started  
**Task ID:** UI-1102

**Dependencies:** BACK-1102, BACK-1103

**What to Do:**
- Curves tool UI: 2D canvas with input (x) vs output (y) curve
- Load/save control points from/to backend
- Trigger backend curve application on change (debounced)

**Acceptance Criteria:**
- [ ] Curve editable (see UI-1103)
- [ ] Changes drive backend BACK-1103 and preview updates
- [ ] Fits in depth controls layout

---

#### UI-1103: Draggable control points on curve
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [ ] Not Started  
**Task ID:** UI-1103

**Dependencies:** UI-1102

**What to Do:**
- Control points on curve are draggable
- Constrain x (and optionally y) to 0–1; curve remains monotonic or enforce in backend
- Smooth curve drawing between points (e.g. cubic Bezier — JR1-1102 can implement drawing)

**Acceptance Criteria:**
- [ ] User can add/move/remove control points (or move only; minimum 2 points)
- [ ] Curve shape updates in real time; preview updates per BACK-1104

---

#### UI-1104: Preset curves (Linear, S-curve, Exponential)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [ ] Not Started  
**Task ID:** UI-1104

**Dependencies:** BACK-1102 (preset definitions), UI-1102

**What to Do:**
- Dropdown or buttons for presets: Linear, S-curve, Exponential
- Applying preset loads corresponding control points and applies curve

**Acceptance Criteria:**
- [ ] All three presets apply correctly (verified by QA-1102)
- [ ] User can then modify from preset (custom curve)

---

#### UI-1105: Advanced mode toggle (show/hide advanced controls)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [ ] Not Started  
**Task ID:** UI-1105

**Dependencies:** None

**What to Do:**
- Toggle (e.g. "Advanced" or "Show histogram & curves") that shows/hides HistogramPanel and CurvesTool
- Default: hidden for simplicity; power users enable

**Acceptance Criteria:**
- [ ] Toggle persists in session or settings (optional)
- [ ] When hidden, basic depth controls (sliders, invert) still visible
- [ ] When shown, histogram and curves visible and functional

---

### Junior Engineer 2D: Canvas & Curve Drawing

#### JR1-1101: Render histogram with Canvas API
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [ ] Not Started  
**Task ID:** JR1-1101

**Dependencies:** UI-1101 (component shell), BACK-1101 (data)

**What to Do:**
- Use Canvas API (or agreed chart approach) to draw histogram bars/lines from backend data
- Responsive to container size; clear and readable

**Acceptance Criteria:**
- [ ] Histogram renders correctly for various depth distributions
- [ ] No layout/overflow issues in sidebar

---

#### JR1-1102: Implement curve drawing with cubic Bezier
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [ ] Not Started  
**Task ID:** JR1-1102

**Dependencies:** UI-1102, UI-1103

**What to Do:**
- Draw smooth curve through control points using cubic Bezier segments
- Ensure curve stays within 0–1 and is monotonic if required by backend

**Acceptance Criteria:**
- [ ] Curve visually smooth; passes through control points
- [ ] Matches backend interpretation (or backend uses same Bezier logic)

---

#### JR1-1103: Add reset curve button
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [ ] Not Started  
**Task ID:** JR1-1103

**Dependencies:** UI-1102, UI-1104 (Linear = identity)

**What to Do:**
- "Reset curve" button that restores Linear preset (identity mapping)
- Accessible from CurvesTool component

**Acceptance Criteria:**
- [ ] Reset restores identity curve; preview matches un-curved depth
- [ ] No duplicate code with preset application (reuse Linear preset)

---

### Quality Engineer: Testing

#### QA-1101: Manual test: adjust curves, verify depth changes
**Assigned Role:** Quality Engineer  
**Priority:** Critical  
**Status:** [ ] Not Started  
**Task ID:** QA-1101

**Dependencies:** UI-1102, UI-1103, BACK-1103

**What to Do:**
- Manual test: change curve control points, observe depth preview and (if possible) mesh/export
- Document steps and expected behaviour

**Acceptance Criteria:**
- [ ] Test procedure in TEST_PLAN_2_1.md or MANUAL_TEST_REPORT
- [ ] Critical paths: curve change → preview update → export reflects curve

---

#### QA-1102: Test preset curves (apply, verify result)
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started  
**Task ID:** QA-1102

**Dependencies:** UI-1104, BACK-1102, BACK-1103

**What to Do:**
- Apply Linear, S-curve, Exponential to same image; verify distinct outcomes
- Linear should match "no curve"; S-curve and Exponential should alter depth distribution as expected

**Acceptance Criteria:**
- [ ] All three presets tested; results documented
- [ ] Any regression in basic depth controls reported

---

#### QA-1103: Performance test: curve application on large depth maps
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started  
**Task ID:** QA-1103

**Dependencies:** BACK-1104, UI-1102

**What to Do:**
- Measure time from curve change to preview update for 1080p and 4K depth maps
- Target: <200ms (per exit criteria)

**Acceptance Criteria:**
- [ ] Results in MANUAL_TEST_REPORT or PROGRESS_REPORT
- [ ] If target missed, documented as known limitation or backlog item

---

## Success Criteria for Sprint

- [ ] All tasks complete per acceptance criteria
- [ ] Exit criteria from todo.md met:
  - [ ] Histogram displays depth distribution
  - [ ] Curves tool functional with draggable points
  - [ ] Preset curves apply correctly
  - [ ] Advanced mode toggle works
  - [ ] Real-time preview updates (<200ms)
- [ ] No blocking issues
- [ ] Gotchas recorded in `SPRINTS/Sprint_2_1/GOTCHAS.md` (merge to RESEARCH when done)
- [ ] Progress report filed

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| None | — | — |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | — |
| cargo clippy | 0 warnings | — |
| npm run build | PASS | — |
| pytest | PASS | — |

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
