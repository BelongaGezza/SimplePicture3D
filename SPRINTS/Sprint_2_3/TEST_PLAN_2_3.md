# Sprint 2.3 — Test Plan: Presets & Templates

**Sprint:** 2.3 — Presets & Templates  
**Owner:** Quality Engineer  
**Last Updated:** 2026-03-06  
**Source:** `SPRINTS/TEST_PLAN_TEMPLATE.md`, `SPRINTS/Sprint_2_3/SPRINT_2_3_Task_Assignment.md`

---

## 1. Scope

| Item | Description |
|------|-------------|
| **Sprint goal** | Users can save and share processing configurations (prd.md F2.3). |
| **Features in scope** | Preset JSON schema, save_preset/load_preset commands, built-in presets (Portrait, Landscape, High Detail, Low Relief), PresetManager UI (list, rename, delete), Save/Load buttons, preset dropdown, import/export dialogs, unit tests for serialization and invalid JSON, schema versioning. |
| **Out of scope** | SEC-202 (SHA256 model checksum — before 2.4); full E2E automation. |

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
| Preset round-trip (JR2-1301) | Rust (preset/settings) | Current state → serialize → load → state matches (depth, curve, mesh params). |
| Invalid JSON / schema (JR2-1302) | Rust | load_preset with malformed/wrong schema returns error; no panic or state corruption. |
| Schema version migration (JR2-1303) | Rust | Older preset version migrates to current; supported fields preserved. |

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | Windows 10/11 (primary); macOS/Linux if available |
| Node version | Per project (e.g. 20) |
| Rust version | stable |
| Python (if used) | 3.x, venv; stub mode for pytest |

### 3.2 Manual test cases (QA-1301–QA-1303)

**Dependencies:** BACK-1302, UI-1301–1304. Run after backend and UI preset features are complete.

#### Case 1 (QA-1301): Save preset, load in new image

| Field | Content |
|-------|---------|
| **Objective** | Verify save-as-preset persists current settings; loading that preset on a different image applies settings and curve correctly. |
| **Preconditions** | App running; image loaded; depth generated; depth/curve adjusted. |
| **Steps** | 1. Adjust depth (e.g. gamma, brightness) and curve. 2. Save as preset (prompt for name). 3. Load a different image. 4. Load the saved preset. 5. Verify sliders, curve, and preview match the saved state. |
| **Expected result** | Preset applies; UI and preview reflect saved depth + curve + mesh params. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 2 (QA-1302): Built-in presets on various images

| Field | Content |
|-------|---------|
| **Objective** | Apply each built-in (Portrait, Landscape, High Detail, Low Relief) to 2–3 images; no crash; depth/preview update as expected. |
| **Preconditions** | App running; built-in presets available in dropdown. |
| **Steps** | For each built-in: load image, select preset from dropdown, verify preview updates. Repeat for 2–3 different images. |
| **Expected result** | All four built-ins apply without crash; visual difference between presets. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 3 (QA-1303): Import preset from external file

| Field | Content |
|-------|---------|
| **Objective** | Export preset to file, move file, import via file dialog — preset applies. Import invalid JSON — clear error, state unchanged. |
| **Preconditions** | App running; at least one user preset saved. |
| **Steps** | 1. Export current preset to a JSON file (save dialog). 2. Move/copy file to another path. 3. Import via file dialog; verify preset applies. 4. Import a non-JSON or malformed file; verify error message and app state unchanged. |
| **Expected result** | Import from file applies preset; invalid file shows error; no state corruption. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

### 3.3 Regression / smoke

- [ ] App starts (`npm run tauri dev`)
- [ ] Load image, generate depth, adjust sliders — no regression
- [ ] Export STL/OBJ still works
- [ ] No console errors on main screen after preset load

---

## 4. Artefacts and sign-off

| Artefact | Path | Owner |
|----------|------|-------|
| Manual test results | `SPRINTS/Sprint_2_3/MANUAL_TEST_REPORT.md` | Quality Engineer |
| Verification checklist | `SPRINTS/Sprint_2_3/VERIFICATION_CHECKLIST.md` | Sprint lead / Architect |
| Gotchas | `SPRINTS/Sprint_2_3/GOTCHAS.md` | Any agent; merge to `RESEARCH/GOTCHAS.md` |

**Process:** Complete manual tests → fill `MANUAL_TEST_REPORT.md` → run through `VERIFICATION_CHECKLIST.md` before marking sprint complete.

---

## 5. References

- **Task list:** `SPRINTS/Sprint_2_3/SPRINT_2_3_Task_Assignment.md`
- **PRD:** `prd.md` §F2.3 (Presets & Templates)
- **Architecture:** `RESEARCH/architecture.md` (preset schema, ADR-010)
- **CLAUDE.md:** Testing commands (cargo test, npm test)

---

**Document Version:** 1.0  
**Template:** `SPRINTS/TEST_PLAN_TEMPLATE.md`
