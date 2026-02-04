# Sprint Test Plan — Sprint 1.4

**Source:** `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Sprint:** Sprint 1.4 — Depth Map Generation & Preview  
**Last Updated:** 2026-02-03

---

## 1. Scope

| Item | Description |
|------|-------------|
| Sprint goal | User sees AI-generated depth map displayed in the UI. |
| Features in scope | `generate_depth_map` Tauri command; depth state; progress; DepthMapPreview; Generate button; side-by-side original vs depth; depth normalization and format; performance (4K). |
| Out of scope | Mesh generation; 3D mesh preview; STL/OBJ export from depth; manual depth adjustments (Sprint 1.5). |

---

## 2. Automated Tests

### 2.1 What runs in CI

| Suite | Command | When |
|-------|---------|------|
| Rust unit/integration | `cargo test --manifest-path src-tauri/Cargo.toml` | Every push/PR |
| Frontend | `npm run build` | Every push/PR |
| Python (if added) | `pytest` (e.g. `python/`) | Every push/PR |

**Reference:** `.github/workflows/ci.yml`, `todo.md` § CI/CD Pipeline.

### 2.2 New or updated automated tests this sprint

| Test | Location | Purpose |
|------|----------|---------|
| generate_depth_map command (success path) | src-tauri/src/lib.rs or tests | BACK-301: invoke command with valid path, assert depth returned |
| Depth map normalization 0–1 | src-tauri or python | JR2-301: assert all depth values in [0, 1] |
| Depth dimensions match input | src-tauri or E2E | QA-304: image dimensions = depth width/height (see QA-304 spec below) |
| Edge case: constant image | src-tauri or manual | JR2-302: all-black/all-white no crash |
| Frontend: Generate button, progress, depth display | src/ or E2E | UI-303, UI-304, UI-301/302 (if E2E configured) |

*Integration tests that require Python may use `#[ignore]` and run with `--ignored` when env present.*

**QA-304 automated test spec (implement when BACK-301/BACK-303 exist):** In `src-tauri/src/lib.rs` `#[cfg(test)]` add test: create or use fixture image with known width/height (e.g. 100×50); call `generate_depth_map(path)`; assert returned `width`/`height` equal image dimensions (or documented downsampling). Use `#[ignore]` with reason "requires Python env" so CI runs without Python; run with `cargo test -- --ignored` when env present.

### 2.3 Coverage

- **Rust:** Extend `cargo test` with command and depth-format tests.
- **Frontend:** Unit tests for depth canvas/render if isolated; otherwise manual + E2E when added.

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | Windows 10/11 (primary); macOS/Linux as available |
| Node version | 20 |
| Rust version | stable |
| Python | 3.10+; venv; Depth-Anything-V2 or stub |

### 3.2 Manual test cases

#### Case 1: Generate depth map — happy path (QA-301)
- **Objective:** Load image, click Generate Depth Map, see depth and progress.
- **Preconditions:** App running (`npm run tauri dev`); Python env set; image loaded.
- **Steps:**
  1. Load an image (file picker or drag-drop).
  2. Click "Generate Depth Map".
  3. Observe progress (bar or spinner + %).
  4. When complete, verify depth map visible (grayscale); side-by-side or toggle with original.
- **Expected:** No crash; progress visible; depth correct dimensions and 0–1 grayscale.
- **Owner:** QA-301.

#### Case 2: Error handling — no image, missing Python, timeout
- **Objective:** Clear error messages; no panic.
- **Steps:** (1) Click Generate without loading image — expect disabled or clear message. (2) With Python removed from PATH / venv deactivated, try Generate — expect error like "failed to spawn Python". (3) If feasible, use very large image or mock timeout — expect timeout message.
- **Expected:** User-facing error text; retry possible.
- **Owner:** QA-301 / BACK-304.

#### Case 3: Depth accuracy — qualitative (QA-302)
- **Objective:** Depth map reflects foreground/background (e.g. person vs sky).
- **Steps:** Use image with clear subject and background; generate depth; inspect grayscale (closer = brighter or darker per convention).
- **Expected:** Qualitative note in report; obvious failures (all same, inverted) reported.
- **Owner:** QA-302.

#### Case 4: Performance — 4K inference time (QA-303)
- **Objective:** Measure time for 4K (or max) image; compare to <30s GPU target.
- **Steps:** Load 3840×2160 (or similar) image; start timer; Generate; stop when depth shown. Record GPU/CPU.
- **Expected:** Time documented; target met or gap explained.
- **Owner:** QA-303.

#### Case 5: Dimensions match (QA-304) — automated or manual
- **Objective:** Depth width/height match loaded image (or documented downsampling).
- **Steps:** Load image of known size (e.g. 640×480); generate; check depth dimensions in UI or via devtools/response.
- **Expected:** Match; or documented downsampling rule.
- **Owner:** QA-304.

### 3.3 Regression / smoke

- [ ] App starts (`npm run tauri dev`)
- [ ] Load image still works (Sprint 1.2)
- [ ] Export (stub) still responds
- [ ] No console errors on main screen before/after Generate
- [ ] Generate button and depth preview do not break layout (min 1024×768)

---

## 4. Artefacts and sign-off

| Artefact | Path | Owner |
|----------|------|-------|
| Manual test results | `SPRINTS/Sprint_1_4/MANUAL_TEST_REPORT.md` | Quality Engineer |
| Verification checklist | `SPRINTS/Sprint_1_4/VERIFICATION_CHECKLIST.md` | Sprint lead / Senior Engineer |
| Gotchas | `SPRINTS/Sprint_1_4/GOTCHAS.md` | Any agent; merge to `RESEARCH/GOTCHAS.md` |

**Process:** Complete manual tests → fill MANUAL_TEST_REPORT → run VERIFICATION_CHECKLIST before marking sprint complete.
