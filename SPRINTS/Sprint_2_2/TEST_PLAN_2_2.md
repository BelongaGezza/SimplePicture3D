# Sprint 2.2 — Test Plan: Undo/Redo, Curve Persistence, Wireframe/Solid

**Sprint:** 2.2  
**Owner:** Quality Engineer  
**Last Updated:** 2026-03-01  
**Source:** `SPRINTS/TEST_PLAN_TEMPLATE.md`, `SPRINTS/Sprint_2_2/SPRINT_2_2_Task_Assignment.md`

---

## 1. Scope

| Item | Description |
|------|-------------|
| **Sprint goal** | Implement F2.4 Undo/Redo, persist curve control points, state ADR, fix or remove Wireframe/Solid UI. |
| **Features in scope** | Undo/Redo (toolbar + Ctrl+Z / Ctrl+Y), history stack (max 20), curve persistence in AppSettings, Wireframe/Solid fix or removal, removal of "Sprint 1.8" overlay. |
| **Out of scope** | Full E2E automation (Phase 1 manual checklist); macOS execution if no macOS env (document plan only). |

---

## 2. Automated Tests

### 2.1 What runs in CI

| Suite | Command | When |
|-------|---------|------|
| Rust unit/integration | `cargo test --manifest-path src-tauri/Cargo.toml` | Every push/PR |
| Frontend | `npm test` | Every push/PR |
| Python (stub) | See CLAUDE.md (SP3D_USE_STUB=1) | Every push/PR |

### 2.2 New or updated automated tests this sprint

| Test | Location | Purpose |
|------|----------|---------|
| Command execute/undo | Rust (per BACK-1401, JR2-1401) | Execute command → state changes; undo → state restored (depth adjustment). |
| History stack limit | Rust (per BACK-1402, JR2-1402) | After 21+ actions, history capped at 20 (oldest dropped). |

*QA-1401 depends on these passing before manual undo/redo verification.*

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | Windows 10/11 (primary); macOS if available (see §5) |
| Node version | Per project (e.g. 20) |
| Rust version | stable |
| Python (if used) | 3.x, venv; stub mode for pytest |

### 3.2 Manual test cases (QA-1401)

**Dependencies:** UI-1401 (Undo/Redo buttons), UI-1402 (shortcuts), BACK-1403 (depth wrapped in commands), BACK-1404 (Tauri undo/redo/clear_history). Run after those tasks are complete.

#### Case 1: Undo/Redo — depth sliders

| Field | Content |
|-------|---------|
| **Objective** | Verify changing depth params (e.g. brightness, gamma), then Undo, restores previous state; Redo restores forward state. |
| **Preconditions** | App running; image loaded; depth map generated. |
| **Steps** | 1. Note current brightness (or gamma). 2. Change brightness (or gamma) via slider. 3. Click **Undo** (or Ctrl+Z). 4. Confirm value and preview match previous state. 5. Click **Redo** (or Ctrl+Y). 6. Confirm value and preview match the changed state. |
| **Expected result** | Undo restores prior value and preview; Redo re-applies change. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 2: Undo/Redo — curve control points

| Field | Content |
|-------|---------|
| **Objective** | Verify changing curve control points, then Undo/Redo, updates curve and depth preview correctly. |
| **Preconditions** | App running; image loaded; depth generated; Advanced (curve) panel open. |
| **Steps** | 1. Move a curve control point. 2. Undo — curve and preview revert. 3. Redo — curve and preview re-apply. |
| **Expected result** | Curve and depth preview reflect undo/redo state. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 3: Boundary — nothing to undo

| Field | Content |
|-------|---------|
| **Objective** | Undo button disabled when no history; no crash. |
| **Preconditions** | Fresh session or after Clear history. |
| **Steps** | 1. Ensure no prior undoable actions (or clear history). 2. Click Undo or press Ctrl+Z. |
| **Expected result** | Undo button disabled or no-op; no error. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 4: Boundary — nothing to redo

| Field | Content |
|-------|---------|
| **Objective** | Redo button disabled when redo stack empty. |
| **Preconditions** | After Undo(s), no Redo performed yet, or after new action (redo stack cleared). |
| **Steps** | 1. Perform one change, then Undo. 2. Confirm Redo is enabled; perform Redo. 3. Confirm Redo is now disabled (or no-op). |
| **Expected result** | Redo available only when there are undone actions; disabled otherwise. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 5: Mesh/export reflects state after Undo/Redo

| Field | Content |
|-------|---------|
| **Objective** | After Undo or Redo, mesh preview and STL/OBJ export reflect current depth state. |
| **Preconditions** | Depth generated; one or more adjustments applied. |
| **Steps** | 1. Change depth (e.g. brightness); note mesh preview. 2. Undo. 3. Confirm 3D preview and (if exported) exported file match undone state. 4. Redo; confirm preview and export match redone state. |
| **Expected result** | Preview and export consistent with current undo/redo state. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 6: Keyboard shortcuts

| Field | Content |
|-------|---------|
| **Objective** | Ctrl+Z = Undo, Ctrl+Y (or Ctrl+Shift+Z) = Redo in app window. |
| **Preconditions** | App focused; undoable action performed. |
| **Steps** | 1. Change a depth param. 2. Press Ctrl+Z — state undoes. 3. Press Ctrl+Y (or Ctrl+Shift+Z) — state redoes. |
| **Expected result** | Shortcuts work; no conflict with OS/browser. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

### 3.3 Regression / smoke

- [ ] App starts (`npm run tauri dev`)
- [ ] Load image, generate depth, adjust sliders/curve
- [ ] Undo/Redo buttons visible and wired (when BACK-1404 done)
- [ ] Wireframe/Solid: either functional or removed (UI-1403)
- [ ] No "Sprint 1.8" (or internal sprint) text in UI (UI-1404)
- [ ] No console errors on main screen

---

## 4. Artefacts and sign-off

| Artefact | Path | Owner |
|----------|------|-------|
| Manual test results | `SPRINTS/Sprint_2_2/MANUAL_TEST_REPORT.md` | Quality Engineer |
| Verification checklist | `SPRINTS/Sprint_2_2/VERIFICATION_CHECKLIST.md` | Sprint lead / Architect |
| macOS smoke (TD-05) | `SPRINTS/Sprint_2_2/MACOS_SMOKE.md` | Quality Engineer |

**Process:** Complete manual tests → fill `MANUAL_TEST_REPORT.md` → run through `VERIFICATION_CHECKLIST.md` before marking sprint complete.

---

## 5. macOS smoke tests (QA-1402 / TD-05)

**Goal:** Start macOS smoke testing per Technical Debt TD-05. If no macOS environment is available, document "Not yet run" and a clear plan.

**Document:** See `SPRINTS/Sprint_2_2/MACOS_SMOKE.md` for:
- Environment (macOS version, Intel/Apple Silicon, Node/Rust/Python)
- Build steps (`npm run tauri build` or equivalent for macOS target)
- Short smoke checklist (e.g. launch, load image, generate depth)
- Status: either smoke run completed, or deferred with plan and owner

**Acceptance:** Document in sprint folder; either smoke run completed or deferred with clear plan.

---

## 6. References

- **Task list:** `SPRINTS/Sprint_2_2/SPRINT_2_2_Task_Assignment.md`
- **PRD:** `prd.md` Phase 2, F2.4 Undo/Redo
- **todo.md:** Sprint 2.2, TD-01 (State ADR), TD-02 (Wireframe/Solid), TD-05 (macOS smoke)
- **CLAUDE.md:** Testing commands
- **Consultant review:** Consultant_Critical_Review_28Feb2026.md §2.2, §2.6

---

**Document Version:** 1.0  
**Template:** `SPRINTS/TEST_PLAN_TEMPLATE.md`
