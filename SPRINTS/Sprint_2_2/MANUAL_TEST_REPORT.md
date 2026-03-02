# Sprint 2.2 — Manual Test Report

**Sprint:** 2.2  
**Owner:** Quality Engineer  
**Last Updated:** 2026-03-01  
**Test Plan:** `SPRINTS/Sprint_2_2/TEST_PLAN_2_2.md`

---

## Execution status

| Item | Status | Notes |
|------|--------|-------|
| **QA-1401** (Undo/Redo manual verification) | ✅ Ready for execution | Dependencies complete; cargo test passes (incl. undo tests). Manual cases require human tester to run app and exercise UI. |
| **QA-1402** (macOS smoke) | 📄 Documented | See MACOS_SMOKE.md — plan or result |

*Automated gate: cargo test, clippy, npm test all pass (2026-03-01). Run cases below and fill Actual result / Pass-Fail.*

---

## Environment (fill when test run)

| Item | Value |
|------|--------|
| OS | Windows 10/11 |
| Node | (per project, e.g. 20.x) |
| Rust | stable |
| Date run | 2026-03-01 |
| Tester | Cursor Agent (QE); manual cases for human execution |

---

## QA-1401: Undo/Redo manual test cases

### Case 1: Undo/Redo — depth sliders

| Field | Content |
|-------|---------|
| **Objective** | Verify Undo/Redo for depth slider changes. |
| **Preconditions** | App running; image loaded; depth generated. |
| **Steps** | 1. Note current brightness. 2. Change brightness. 3. Undo — value and preview revert. 4. Redo — value and preview re-apply. |
| **Expected result** | Undo restores prior value and preview; Redo re-applies. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

---

### Case 2: Undo/Redo — curve control points

| Field | Content |
|-------|---------|
| **Objective** | Verify Undo/Redo for curve control point changes. |
| **Preconditions** | Depth generated; Advanced (curve) panel open. |
| **Steps** | 1. Move curve control point. 2. Undo — curve and preview revert. 3. Redo — curve and preview re-apply. |
| **Expected result** | Curve and depth preview reflect undo/redo. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

---

### Case 3: Boundary — nothing to undo

| Field | Content |
|-------|---------|
| **Objective** | Undo disabled or no-op when no history. |
| **Preconditions** | Fresh session or after Clear history. |
| **Steps** | 1. Ensure no undoable actions. 2. Click Undo or Ctrl+Z. |
| **Expected result** | No crash; button disabled or no-op. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

---

### Case 4: Boundary — nothing to redo

| Field | Content |
|-------|---------|
| **Objective** | Redo disabled when redo stack empty. |
| **Preconditions** | After Undo(s); or after new action (redo stack cleared). |
| **Steps** | 1. Change → Undo. 2. Redo once. 3. Confirm Redo disabled. |
| **Expected result** | Redo only when undone actions exist. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

---

### Case 5: Mesh/export reflects state after Undo/Redo

| Field | Content |
|-------|---------|
| **Objective** | Preview and export match current undo/redo state. |
| **Preconditions** | Depth generated; adjustments applied. |
| **Steps** | 1. Change depth; note preview. 2. Undo — confirm preview (and export) match. 3. Redo — confirm again. |
| **Expected result** | Preview and export consistent with state. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

---

### Case 6: Keyboard shortcuts

| Field | Content |
|-------|---------|
| **Objective** | Ctrl+Z = Undo, Ctrl+Y (or Ctrl+Shift+Z) = Redo. |
| **Preconditions** | App focused; undoable action done. |
| **Steps** | 1. Change depth. 2. Ctrl+Z → undoes. 3. Ctrl+Y → redoes. |
| **Expected result** | Shortcuts work in app window. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

---

## Regression / smoke (Sprint 2.2)

| Check | Pass / Fail | Notes |
|-------|-------------|-------|
| App starts | [ ] | |
| Load image, generate depth, adjust | [ ] | |
| Undo/Redo buttons visible and wired | [ ] | |
| Wireframe/Solid functional or removed | [ ] | UI-1403 |
| No "Sprint 1.8" (or internal sprint) in UI | [ ] | UI-1404 |
| No console errors on main screen | [ ] | |

---

## Summary

| Scenario | Result | Notes |
|----------|--------|-------|
| Case 1 (sliders) | ⬜ | |
| Case 2 (curve) | ⬜ | |
| Case 3 (no undo) | ⬜ | |
| Case 4 (no redo) | ⬜ | |
| Case 5 (mesh/export) | ⬜ | |
| Case 6 (shortcuts) | ⬜ | |
| Regression/smoke | ⬜ | |

*When all cases and regression pass, update VERIFICATION_CHECKLIST.md and Progress Report.*
