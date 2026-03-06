# Test Plan — Sprint 2.4: Progress Streaming for Depth Estimation

**Sprint:** 2.4
**Owner:** Quality Engineer
**Last Updated:** 2026-03-06

---

## 1. Progress Streaming (QA-304-STREAM)

**Purpose:** Verify that real-time depth estimation progress is visible to the user.

**Preconditions:**
- App running (`npm run tauri dev` or installed build)
- Python env configured with real model (not stub mode — `SP3D_USE_STUB` must NOT be set)
- AI model installed (`~/.simplepicture3d/models/`)

### Manual test cases

| # | Test Case | Steps | Expected Result | Pass? |
|---|-----------|-------|-----------------|-------|
| 1 | **Basic progress streaming (small image)** | Load a ~800×600 PNG; click Generate Depth Map | Progress bar shows numeric % that increases; depth map appears on completion | ☐ |
| 2 | **Basic progress streaming (large image)** | Load a 4K image; click Generate Depth Map | Progress updates visibly for the duration of the run (≥3 distinct percentages observed) | ☐ |
| 3 | **Completion cleanup** | Complete a depth run; observe progress bar | Bar reaches 100% and disappears (or resets to 0) after depth map is shown | ☐ |
| 4 | **Repeat run** | After first run completes, click Generate again | Progress bar starts from 0%; no stale events from previous run | ☐ |
| 5 | **Error case** | Generate depth with a broken Python env or missing model | Progress bar clears; error message shown; app does not freeze | ☐ |
| 6 | **Accessibility** | Observe progress element during a run | `aria-valuenow` attribute updates with percentage (inspect DOM or use screen reader) | ☐ |

### Automated checks (CI)

| Check | Command | Expected |
|-------|---------|---------|
| `cargo test` | `cargo test --manifest-path src-tauri/Cargo.toml` | PASS |
| `cargo clippy` | `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` | 0 warnings |
| `npm test` | `npm test` | PASS |

---

## 2. Preset Tests — Carryover (QA-1301–1303)

**Purpose:** Sign off on Sprint 2.3 preset functionality that was not QA'd before sprint close.

**Preconditions:** App running; a depth map must be loaded before saving presets.

### QA-1301: Save and apply user preset

| # | Step | Expected | Pass? |
|---|------|----------|-------|
| 1 | Generate depth map | Depth map visible | ☐ |
| 2 | Set brightness to 0.2, gamma to 1.5; click "Save as preset…"; name it "Test Preset" | Preset appears in dropdown and Saved presets list | ☐ |
| 3 | Click Reset to restore defaults | Sliders back to defaults | ☐ |
| 4 | Select "Test Preset" from dropdown; click Apply | brightness = 0.2, gamma = 1.5 restored | ☐ |

### QA-1302: Apply built-in presets

| # | Step | Expected | Pass? |
|---|------|----------|-------|
| 1 | Apply "Portrait" | Sliders update to Portrait values (brightness ≈ 0.05, contrast ≈ 1.1, gamma ≈ 1.15, depth 2–10 mm) | ☐ |
| 2 | Apply "Landscape" | Sliders update (contrast ≈ 1.05, depth 2–12 mm) | ☐ |
| 3 | Apply "High Detail" | Sliders update (contrast ≈ 1.35, gamma ≈ 1.25) | ☐ |
| 4 | Apply "Low Relief" | Sliders update (brightness ≈ 0.1, depth 2–6 mm) | ☐ |
| 5 | Attempt to rename/delete a built-in preset | Option not available (built-ins are read-only) | ☐ |

### QA-1303: Import/export preset JSON and rename/delete

| # | Step | Expected | Pass? |
|---|------|----------|-------|
| 1 | Click "Export preset…"; save to Desktop as `my_preset.json` | JSON file created; open it and confirm keys (brightness, contrast, etc.) present | ☐ |
| 2 | Click "Import preset…"; choose `my_preset.json` | Depth params update to file values immediately | ☐ |
| 3 | In Saved presets, click Rename on "Test Preset"; enter "Renamed Preset"; confirm | "Renamed Preset" appears in list; "Test Preset" gone | ☐ |
| 4 | Click Delete on "Renamed Preset"; confirm | Preset removed; no crash | ☐ |
| 5 | Attempt save with invalid name (e.g. `../bad`) | Error message shown; no file written | ☐ |

---

## 3. Security Review (SEC-202)

**Owner:** Security Specialist
**Test:** Review and document outcome in `SECURITY_SIGNOFF.md`.

| Item | Verification |
|------|-------------|
| HTTPS confirmed for model downloads | Check `model_downloader.py` — `snapshot_download` / `from_pretrained` use HTTPS |
| SHA256 hash of primary model file documented in `RESEARCH/architecture.md` | OR rationale for SEC-202B (risk-accepted) documented |
| `docs/threat-model.md` §2.2 SEC-202 marked reviewed | Confirm date and outcome |
| `docs/security-checklist.md` §2.2 SEC-202 updated | Confirm status |

---

## 4. Regression (every sprint)

Run before sprint close to confirm no regressions from Sprint 2.4 changes:

| # | Test | Expected | Pass? |
|---|------|----------|-------|
| 1 | Core workflow: load → generate depth → adjust → export STL | Unchanged from Sprint 2.3 | ☐ |
| 2 | Undo/redo (Sprint 2.2) | Ctrl+Z / Ctrl+Y work as before | ☐ |
| 3 | Presets (Sprint 2.3) | Save/apply/import/export unaffected by 2.4 changes | ☐ |
| 4 | Settings persist after restart | Depth range, target size restored | ☐ |

---

## 5. Exit Gate Checklist

- [ ] QA-304-STREAM: all manual streaming test cases pass (or failures filed as Issues)
- [ ] QA-1301–1303: preset carryover QA complete (all pass or failures filed)
- [ ] SEC-202: security sign-off filed in `SECURITY_SIGNOFF.md`
- [ ] JR2-1301–1303: automated tests passing in CI
- [ ] Regression: no new failures vs Sprint 2.3 baseline
- [ ] All CI checks green (cargo test, npm test, clippy, build)
