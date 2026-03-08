# Sprint 2.5 — Test Plan: Masking & Regional Adjustments

**Sprint:** 2.5 — Masking & Regional Adjustments  
**Owner:** Quality Engineer  
**Last Updated:** 2026-03-08  
**Source:** `SPRINTS/TEST_PLAN_TEMPLATE.md`, `SPRINTS/Sprint_2_5/SPRINT_2_5_Task_Assignment.md`

---

## 1. Scope

| Item | Description |
|------|-------------|
| **Sprint goal** | Enable selective depth adjustments via masking tools (brush, eraser, select). Mask state covered by ADR-010 extension (ARCH-502). |
| **Features in scope** | Mask data structure and IPC (BACK-1201); masked-only depth adjustments (BACK-1202); feathering at mask edges (BACK-1203); MaskingTools UI (brush, eraser, select), canvas painting, mask overlay, brush size/hardness (UI-1201–1204); brush smoothing, selection tools, mask save/load (JR1-1201–1203). |
| **Out of scope** | Full 3D reconstruction (Phase 2); automated E2E (Playwright deferred). |

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
| Mask dimensions / get/set/clear (BACK-1201) | Rust (mask module / lib) | Mask matches depth dimensions; get_mask, set_mask_region, clear_mask behave as per ARCH-502. |
| Masked-only depth application (BACK-1202) | Rust | Depth params apply only where mask is set; unmasked regions unchanged. |
| Feathering at boundary (BACK-1203) | Rust | Feather radius produces soft falloff; no hard edge. |

*Add rows as backend and frontend tests are added during the sprint.*

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | Windows 10/11, macOS, Linux (per CLAUDE.md) |
| Node version | Per project (e.g. 20) |
| Rust version | stable |
| Python (if used) | 3.x, venv; stub mode for pytest |

### 3.2 Manual test cases (QA-1201–QA-1203)

**Dependencies:** BACK-1202, UI-1201, UI-1202 (for QA-1201); BACK-1203 and UI for QA-1202; BACK-1201, UI-1201, ARCH-502 for QA-1203. Execute manual cases after masking backend and UI are complete.

#### Case 1 (QA-1201): Paint mask, adjust depth, verify isolation

| Field | Content |
|-------|---------|
| **Objective** | Verify that depth adjustments apply only to the masked region; unmasked area remains unchanged in preview and export. |
| **Preconditions** | App running; image loaded; depth generated; MaskingTools available (brush/eraser). |
| **Steps** | 1. Use brush to paint a mask over a distinct region (e.g. centre or one half). 2. Apply depth adjustments (e.g. increase brightness or gamma). 3. Observe depth preview: only masked region should change. 4. Export STL/OBJ and verify in external viewer or mesh inspection that adjusted depth is confined to the masked area. 5. Optionally clear mask and confirm full-image adjustment behaviour. |
| **Expected result** | Only masked pixels reflect depth param changes; unmasked regions retain original (or baseline) depth; preview and export consistent. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 2 (QA-1202): Mask feathering (soft edges)

| Field | Content |
|-------|---------|
| **Objective** | Verify soft transition at mask edges; no visible hard line between masked and unmasked depth. |
| **Preconditions** | App running; mask painting and depth pipeline with feathering (BACK-1203) enabled; feather radius or equivalent control available. |
| **Steps** | 1. Paint a mask with a clear boundary (e.g. circle or rectangle). 2. Enable feathering (default or set radius). 3. Apply depth adjustment. 4. Inspect preview at mask edge: transition should be smooth. 5. If UI exposes feather radius, test 2–3 values and confirm softer edge with larger radius. 6. Export and spot-check mesh in problem areas. |
| **Expected result** | No hard artifact at mask boundary; configurable feather produces smooth blend. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 3 (QA-1203): Undo/redo with masking

| Field | Content |
|-------|---------|
| **Objective** | Verify mask and depth state restore correctly on undo/redo; no desync between frontend and backend. |
| **Preconditions** | App running; mask commands in undo stack (per ARCH-502); Undo/Redo UI available. |
| **Steps** | 1. Load image, generate depth. 2. Paint mask in a region. 3. Change depth params (e.g. gamma). 4. Undo once: depth params should revert; mask should remain. 5. Undo again (if mask mutation is separate): mask should revert. 6. Redo twice: mask and depth state should match state before undos. 7. Verify preview and (if time) export match after redo. 8. Repeat with different order (e.g. adjust depth then paint mask) and undo/redo. |
| **Expected result** | Undo/redo restores mask and depth state correctly; frontend overlay and backend mask stay in sync. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

### 3.3 Regression / smoke

- [ ] App starts (`npm run tauri dev`)
- [ ] Load image, generate depth — no regression
- [ ] Depth preview and sliders work without mask
- [ ] Export STL/OBJ still works
- [ ] No console errors on main screen when toggling mask overlay or tools

---

## 4. Artefacts and sign-off

| Artefact | Path | Owner |
|----------|------|-------|
| Manual test results | `SPRINTS/Sprint_2_5/MANUAL_TEST_REPORT.md` | Quality Engineer |
| Verification checklist | `SPRINTS/Sprint_2_5/VERIFICATION_CHECKLIST.md` | Sprint lead / Architect |
| Gotchas | `SPRINTS/Sprint_2_5/GOTCHAS.md` | Any agent; merge to `RESEARCH/GOTCHAS.md` |

**Process:** Complete manual tests → fill `MANUAL_TEST_REPORT.md` → run through `VERIFICATION_CHECKLIST.md` before marking sprint complete.

---

## 5. References

- **Task list:** `SPRINTS/Sprint_2_5/SPRINT_2_5_Task_Assignment.md`
- **PRD:** `prd.md` (Phase 2 masking, regional adjustments)
- **Architecture:** `RESEARCH/architecture.md` ADR-010, ARCH-502 (mask command contract)
- **CLAUDE.md:** Testing commands (cargo test, npm test)

---

**Document Version:** 1.0  
**Template:** `SPRINTS/TEST_PLAN_TEMPLATE.md`
