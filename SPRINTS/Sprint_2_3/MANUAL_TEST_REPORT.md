# Sprint 2.3 — Manual Test Report

**Sprint:** 2.3 — Presets & Templates  
**Owner:** Quality Engineer  
**Last Updated:** 2026-03-06

---

## Purpose

Manual test results for QA-1301, QA-1302, QA-1303. Test plan: `SPRINTS/Sprint_2_3/TEST_PLAN_2_3.md` §3.2.

---

## QE verification run (2026-03-06)

**Tester:** Quality Engineer (agent)  
**Environment:** Windows; project root.

### Automated gate

| Check | Result |
|-------|--------|
| `cargo test --manifest-path src-tauri/Cargo.toml` | **PASS** — 166 passed, 6 ignored |
| `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` | **PASS** — 0 warnings |
| `npm run build` | **PASS** — (A11y warnings in CurvesTool.svelte only; known) |
| `npm test` | **PASS** — 39 tests, 5 files |

**Conclusion:** Automated quality gate **PASS**. Preset backend and frontend build and tests are green.

### Manual test cases (QA-1301–QA-1303)

Per pre-sprint gate decision, manual QA is **non-blocking** (48h post-sprint window, named tester). Procedures are documented below and in TEST_PLAN_2_3.md §3.2. A human tester should run the app (`npm run tauri dev`), execute Cases 1–3 and regression, and fill **Actual result** and **Pass/Fail** for each.

---

## Results summary

| Case | ID | Description | Pass / Fail | Date | Tester |
|------|-----|-------------|-------------|------|--------|
| 1 | QA-1301 | Save preset, load in new image | Ready | 2026-03-06 | Procedure ready; human execution in 48h window |
| 2 | QA-1301 | Load preset — list and apply by name | Ready | — | Same as Case 1 flow |
| 3 | QA-1302 | Built-in preset — Portrait | Ready | — | See Case 4 |
| 4 | QA-1302 | Built-in presets — Landscape, High Detail, Low Relief | Ready | — | Apply each to 2–3 images; verify no crash, preview updates |
| 5 | QA-1303 | Export preset to file, import from file | Ready | — | Export → move file → import; verify applies |
| 6 | QA-1303 | Import invalid JSON — error handling | Ready | — | Import non-JSON/malformed file; verify error, state unchanged |
| 7 | — | PresetManager — list, rename, delete | Ready | — | Verify list, rename, delete in PresetManager UI |

**Ready** = Procedure documented in TEST_PLAN_2_3.md; automated gate PASS; manual execution to be completed by named tester within 48h post-sprint if not yet run.

---

## Detailed results

### Case 1 (QA-1301): Save preset, load in new image

| Field | Content |
|-------|---------|
| **Steps** | 1. Adjust depth (gamma, brightness) and curve. 2. Save as preset (prompt for name). 3. Load a different image. 4. Load the saved preset. 5. Verify sliders, curve, and preview match the saved state. |
| **Expected result** | Preset applies; UI and preview reflect saved depth + curve + mesh params. |
| **Actual result** | *(To be filled by human tester)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

### Case 2 (QA-1302): Built-in presets on various images

| Field | Content |
|-------|---------|
| **Steps** | For each built-in (Portrait, Landscape, High Detail, Low Relief): load image, select preset from dropdown, verify preview updates. Repeat for 2–3 different images. |
| **Expected result** | All four built-ins apply without crash; visual difference between presets. |
| **Actual result** | *(To be filled by human tester)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

### Case 3 (QA-1303): Import preset from external file

| Field | Content |
|-------|---------|
| **Steps** | 1. Export current preset to a JSON file (save dialog). 2. Move/copy file to another path. 3. Import via file dialog; verify preset applies. 4. Import a non-JSON or malformed file; verify error message and app state unchanged. |
| **Expected result** | Import from file applies preset; invalid file shows error; no state corruption. |
| **Actual result** | *(To be filled by human tester)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

---

## Regression / smoke

| Check | Result |
|-------|--------|
| App builds (frontend + backend) | **PASS** — `npm run build`, cargo build OK |
| App binary (Tauri) | **PASS** — `simplepicture3d.exe` built (tauri build succeeds to binary; installer step failed on category config, unrelated to presets) |
| Load image, generate depth, adjust sliders | *(Manual — run with `npm run tauri dev`)* |
| Export STL/OBJ still works | *(Manual — regression)* |
| No console errors on main screen after preset load | *(Manual)* |

---

**Document Version:** 1.1  
**Template:** Per `TEST_PLAN_2_3.md` and Sprint 2.3 Task Assignment
