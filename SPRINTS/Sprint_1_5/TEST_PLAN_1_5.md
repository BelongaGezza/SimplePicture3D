# Sprint 1.5 Test Plan

**Sprint:** 1.5 — Manual Depth Adjustments  
**Source:** `SPRINT_1_5_Task_Assignment.md`, `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Last Updated:** 2026-02-04

---

## 1. Scope

| Item | Description |
|------|-------------|
| Sprint goal | User can modify the AI-generated depth map with sliders and controls. |
| Features in scope | Depth adjustment (brightness, contrast, gamma, invert, range); DepthControls UI; real-time preview (debounced); Reset. |
| Out of scope | Mesh generation, STL/OBJ export, model download. |

---

## 2. Automated Tests

### 2.1 What runs in CI

| Suite | Command | When |
|-------|---------|------|
| Rust | `cargo test --manifest-path src-tauri/Cargo.toml` | Every push/PR |
| Rust | `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` (QA-405) | Every push/PR |
| Frontend | `npm run build` (lint/test if configured) | Every push/PR |

### 2.2 New or updated automated tests this sprint

| Test | Location | Purpose |
|------|----------|---------|
| Depth adjustment algorithms (JR2-401) | src-tauri or frontend | Unit tests for brightness, contrast, gamma, invert |
| Boundary conditions (JR2-402) | src-tauri or frontend | Min/max, extreme params |
| Apply adjustments, check output (QA-404) | src-tauri/src/depth_adjust.rs | Output in [0, 1], pipeline + boundary tests; runs in CI |

### 2.3 Coverage

Per project defaults; no new coverage target for 1.5.

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | Windows 11 (primary); macOS/Linux if available |
| Node / npm | Per package.json |
| Rust | stable |
| Python | Required for depth generation (Sprint 1.4 flow) |

### 3.2 Manual test cases

*(QA fills when claiming role; link to MANUAL_TEST_REPORT.md.)*

- **Case 1 (QA-401):** Adjust all controls, verify preview updates.
- **Case 2 (QA-402):** Extreme values (brightness 0%, 200%; gamma min/max); no crash.
- **Case 3 (QA-403):** Reset button restores original depth view.
- **Case 4 (QA-404):** Automated test only; manual optional (apply adjustments, check output array).

### 3.3 Junior 3D — DepthControls UI (JR1-401–404)

*Quick check after loading image and generating depth:*

| Check | Action | Pass |
|-------|--------|------|
| Slider styling | Sliders have clear track/thumb; usable at 1024×768 | ☐ |
| Numeric inputs | Depth min/max, Brightness, Gamma have number box beside slider; typing updates value and stays in bounds | ☐ |
| Arrow keys | Focus a slider (Tab to it); Arrow Left/Down decrement, Right/Up increment by step; no page scroll | ☐ |
| Responsiveness | Drag sliders; preview updates within ~100 ms of last change; no freeze or jank | ☐ |
| Reset | Change several controls → Reset → sliders and preview back to defaults | ☐ |

### 3.4 Regression

- Load image → Generate depth (Sprint 1.4) still works.
- Depth preview (grayscale, zoom/pan) still works after adding DepthControls.

---

## 4. References

- Task assignment: `SPRINT_1_5_Task_Assignment.md`
- Manual test report: `MANUAL_TEST_REPORT.md`
- Verification: `VERIFICATION_CHECKLIST.md`
