# Sprint 1.5 Manual Test Report

**Sprint:** 1.5 — Manual Depth Adjustments  
**Test Plan:** `TEST_PLAN_1_5.md`  
**Last Updated:** 2026-02-06  
**Executed by:** Quality Engineer (Cursor-Auto) with user performing manual steps

---

## Automated test run (2026-02-06)

| Suite | Command | Result |
|-------|---------|--------|
| Rust unit tests | `cargo test --manifest-path src-tauri/Cargo.toml` | **PASS** — 44 passed, 5 ignored |
| Rust clippy | `cargo clippy -- -D warnings` | **PASS** — 0 warnings |
| Frontend build | `npm run build` | **PASS** (A11y suggestions only, build succeeded) |
| Python pytest | `SP3D_USE_STUB=1 pytest python/` | **PASS** — 19 passed |

---

## Summary

| Case | Task | Status | Executed By | Date | Notes |
|------|------|--------|-------------|------|-------|
| 1 | QA-401: Adjust all controls, verify preview updates | Pass | User + QA | 2026-02-06 | All controls update preview; ~100 ms; Reset restores defaults |
| 2 | QA-402: Extreme values (brightness 0%, 200%); no crash | Pass | User + QA | 2026-02-06 | Brightness min/max, Gamma min/max, Depth range extremes; no crash |
| 3 | QA-403: Reset button restores original depth view | Pass | User + QA | 2026-02-06 | Reset restores original depth and default controls |

*Automated tests (QA-404) and CI clippy (QA-405) are implemented; see completion records in SPRINT_1_5_Task_Assignment.md.*

---

## Case 1 — QA-401: Adjust all controls, verify preview updates

**Steps:**
1. Launch app; load an image; generate depth map.
2. Change Depth Range (min/max mm), Brightness, Gamma sliders; toggle Invert.
3. Verify depth preview updates within ~100 ms of last change (debounced).
4. Click Reset; verify preview and controls restore to defaults.

**Result:** All controls (Depth min/max, Brightness, Gamma, Invert) updated preview; update felt within ~100 ms of last change; Reset restored original depth view and default control values.

**Pass/Fail:** **Pass**

---

## Case 2 — QA-402: Extreme values

**Steps:**
1. With depth map loaded, set Brightness to minimum and maximum.
2. Set Gamma to minimum and maximum.
3. Set Contrast to extreme values if exposed.
4. Verify no crash, no corrupt display; depth stays in valid range or UI prevents invalid input.

**Result:** Brightness min/max, Gamma min/max, and Depth range extremes exercised; no crash, no corrupt display; depth stayed valid.

**Pass/Fail:** **Pass**

---

## Case 3 — QA-403: Reset restores original depth

**Steps:**
1. Load image, generate depth.
2. Apply several adjustments (e.g. brightness +0.3, gamma 1.5, invert on).
3. Click Reset.
4. Verify preview matches initial depth (before any slider changes); sliders/checkbox show default state.

**Result:** After applying brightness, gamma, and invert, Reset restored preview to initial depth and all controls to default state.

**Pass/Fail:** **Pass**

---

## Regression (TEST_PLAN §3.4)

| Check | Steps | Result |
|-------|--------|--------|
| Load image → Generate depth (Sprint 1.4 flow) | Load image via app; click Generate; depth appears in sidebar | **Pass** |
| Depth preview (grayscale, zoom/pan) with DepthControls | With depth shown, zoom (scroll/pinch) and pan (drag); verify grayscale and controls visible | **Pass** |

**Note for UI Designer:** Zoom and pan are accomplished by mouse wheel (zoom) and click-drag / "grip" on the depth map; there is no explicit control or on-screen sign that the depth preview can be zoomed or panned. Consider adding a hint (e.g. tooltip, small label, or icon) to improve discoverability.

---

## Junior 3D quick checks (TEST_PLAN §3.3)

*Slider terms: **track** = the horizontal bar/rail the slider moves along; **thumb** = the draggable knob or handle you grab.*

| Check | Action | Pass |
|-------|--------|------|
| Slider styling (track/thumb) | Sliders have clear track and thumb; usable at 1024×768 (or current resolution) | ☑ Pass (track = bar, thumb = draggable handle; both clear; resolution check pass) |
| Numeric inputs | Depth min/max, Brightness, Gamma have number box; typing updates value and stays in bounds | ☑ Pass |
| Arrow keys | Focus a slider (Tab); Arrow Left/Down decrement, Right/Up increment; no page scroll | ☑ Pass |
| Responsiveness | Drag sliders; preview updates within ~100 ms; no freeze or jank | ☑ Pass |
| Reset | Change several controls → Reset → sliders and preview back to defaults | ☑ Pass |

*Executed 2026-02-06; all checks passed. Reset also verified in QA-403.*
