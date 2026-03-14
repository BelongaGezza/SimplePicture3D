# Sprint 2.5 — Manual Test Report

**Sprint:** 2.5 — Masking & Regional Adjustments  
**Owner:** Quality Engineer  
**Last Updated:** 2026-03-14

---

## Purpose

Manual test results for QA-1201, QA-1202, QA-1203. Test plan: `SPRINTS/Sprint_2_5/TEST_PLAN_2_5.md` §3.2.

---

## Execution status

**Dependencies complete.** BACK-1202, BACK-1203, UI-1201–UI-1204, and JR1-1201–JR1-1203 are done. Manual Cases 1–3 are **ready for human execution**: run the app (`npm run tauri dev`), follow TEST_PLAN_2_5.md §3.2 Cases 1–3 and §3.3 regression, then fill **Actual result** and **Pass/Fail** below.

---

## Quick start for human tester

1. From project root: `npm run tauri dev`
2. Load an image → **Generate depth**
3. Open **Masking** panel; use **Brush** to paint a region (e.g. centre); toggle **Mask overlay** to see it
4. Adjust **Depth** sliders (e.g. Brightness/Gamma); confirm only the masked area changes in the 3D preview
5. **Case 1 (QA-1201):** Verify isolation; optionally export STL/OBJ and spot-check
6. **Case 2 (QA-1202):** Set **Feather radius** (e.g. 10 px), paint mask, adjust depth; check soft edge in preview
7. **Case 3 (QA-1203):** Paint mask → change depth → **Undo** (depth then mask); **Redo**; confirm state and overlay match
8. Fill **Actual result** and **Pass/Fail** in the Detailed results section below; update Results summary table

---

## QE verification run (2026-03-08)

**Tester:** Quality Engineer (agent)  
**Environment:** Project root; automated gate executed; manual cases ready for human tester.

### Automated gate

| Check | Result |
|-------|--------|
| `cargo test --manifest-path src-tauri/Cargo.toml --lib` | **PASS** — 194 passed, 6 ignored |
| `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` | **PASS** — 0 warnings |
| `cargo fmt --check --manifest-path src-tauri/Cargo.toml` | **PASS** |
| `npm run build` | **PASS** |
| `npm test` | **PASS** — 74 tests, 9 files |

**Conclusion:** Automated quality gate **PASS** (2026-03-08). Manual testing stopped after Case 1: **P0 — mask has no visible effect** (brush, overlay, and depth isolation all non-functional). Cases 2–3 deferred until P0 fixed.

---

## Results summary

| Case | ID | Description | Pass / Fail | Date | Tester |
|------|-----|-------------|-------------|------|--------|
| 1 | QA-1201 | Paint mask, adjust depth, verify isolation | **Fail** | 2026-03-14 | Human tester |
| 2 | QA-1202 | Mask feathering (soft edges) | Blocked (P0) | — | Deferred until mask effect fixed |
| 3 | QA-1203 | Undo/redo with masking | Blocked (P0) | — | Deferred until mask effect fixed |

**Ready** = Dependencies complete; procedure in TEST_PLAN_2_5.md §3.2; human tester to execute and fill Actual result / Pass-Fail.

---

## Detailed results

### Case 1 (QA-1201): Paint mask, adjust depth, verify isolation

| Field | Content |
|-------|---------|
| **Steps** | 1. Use brush to paint a mask over a distinct region. 2. Apply depth adjustments (e.g. brightness, gamma). 3. Observe depth preview: only masked region should change. 4. Export STL/OBJ and verify adjusted depth confined to masked area. 5. Optionally clear mask and confirm full-image behaviour. |
| **Expected result** | Only masked pixels reflect depth param changes; unmasked regions unchanged; preview and export consistent. |
| **Actual result** | (1) Brush mask does not make any visible change at any time. (2) Depth adjustments work but do not show depth of nose and facial features well (aside from eyes). (3) Depth preview does not appear to be influenced by the mask. (4) No changes appear to be influenced by the mask. (5) Mask has no effect. |
| **Pass / Fail** | [ ] Pass [x] Fail |

### Case 2 (QA-1202): Mask feathering (soft edges)

| Field | Content |
|-------|---------|
| **Steps** | 1. Paint a mask with a clear boundary. 2. Enable feathering; apply depth adjustment. 3. Inspect preview at mask edge for smooth transition. 4. If available, test 2–3 feather radii. 5. Export and spot-check mesh. |
| **Expected result** | No hard artifact at mask boundary; smooth blend. |
| **Actual result** | *Blocked — not run. Deferred until P0 (mask has no effect) is fixed.* |
| **Pass / Fail** | — (blocked) |

### Case 3 (QA-1203): Undo/redo with masking

| Field | Content |
|-------|---------|
| **Steps** | 1. Load image, generate depth. 2. Paint mask; change depth params. 3. Undo (depth then mask if separate). 4. Redo; verify state matches. 5. Repeat with different order of operations. |
| **Expected result** | Undo/redo restores mask and depth correctly; no frontend/backend desync. |
| **Actual result** | *Blocked — not run. Deferred until P0 (mask has no effect) is fixed.* |
| **Pass / Fail** | — (blocked) |

---

## Regression / smoke

To be verified by human tester when executing Cases 1–3 (TEST_PLAN_2_5.md §3.3):

| Check | Result |
|-------|--------|
| App starts (`npm run tauri dev`) | *(Manual)* |
| Load image, generate depth | *(Manual)* |
| Depth preview and sliders without mask | *(Manual)* |
| Export STL/OBJ | *(Manual)* |
| No console errors with mask overlay/tools | *(Manual)* |

---

**Document Version:** 1.0  
**Template:** Per `TEST_PLAN_2_5.md` and Sprint 2.5 Task Assignment
