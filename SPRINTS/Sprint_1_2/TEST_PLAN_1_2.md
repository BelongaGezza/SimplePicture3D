# Sprint Test Plan — Sprint 1.2

**Source:** `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Sprint:** Sprint 1.2 — Image Loading & Display  
**Last Updated:** 2026-02-03

---

## 1. Scope

| Item | Description |
|------|-------------|
| Sprint goal | User can load an image file and see it displayed in the UI. |
| Features in scope | Image load via file picker and drag-and-drop; validation (format, size, integrity); downsampling >8K; preview and metadata; error handling. |
| Out of scope | Depth estimation, mesh generation, export STL/OBJ, Python backend. |

---

## 2. Automated Tests

### 2.1 What runs in CI

| Suite | Command | When |
|-------|---------|------|
| Rust unit/integration | `cargo test --manifest-path src-tauri/Cargo.toml` | Every push/PR |
| Frontend (if any) | `npm test` | Every push/PR |

**Reference:** `.github/workflows/ci.yml`, `todo.md` § CI/CD Pipeline.

### 2.2 New or updated automated tests this sprint

| Test | Location | Purpose |
|------|----------|---------|
| load_image valid PNG, dimensions | src-tauri/src/lib.rs (or integration) | QA-103: invoke load_image with fixture, assert dimensions in response |
| load_image invalid/corrupt file, error | src-tauri/src/lib.rs (or integration) | QA-104: invalid/corrupt file returns error, no panic |
| Image loading unit tests | src-tauri (BACK-101, JR2-101) | Valid/invalid path, dimensions, downsampling |
| Path edge cases (Unicode, spaces) | src-tauri (JR2-102) | Path handling documented/tested |

*Fixtures:* `tests/fixtures/` (see `tests/fixtures/README.md`). Dataset: valid_1x1.png, valid_small.png, invalid_not_an_image.png, corrupt_truncated.png.

#### JR1-104: Image size test coverage

| Size category | Coverage | Location / notes |
|---------------|----------|------------------|
| Normal (e.g. small, 4K, 8K) | Manual: Case 1 (file picker), Case 2 (drag-drop), Case 3 (different folder). Automated: JR2-101, QA-103 (valid PNG, dimensions). | TEST_PLAN_1_2 §3.2 Cases 1–3; `src-tauri` tests when BACK-101/JR2-101 complete. |
| >8K (downsampling) | Manual: Case 6 (oversized image). Automated: JR2-103 (16K test image). | TEST_PLAN_1_2 §3.2 Case 6; JR2-103 with QA-101 fixture. |

At least two size categories (normal and >8K) are covered by the above. E2E (e.g. Playwright) may be added later if feasible.

### 2.3 Coverage (if configured)

- **Rust:** `cargo tarpaulin` (see README / CONTRIBUTING when configured).

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | Windows 10/11 (primary); macOS/Linux per Phase 3 |
| Node version | 20 |
| Rust version | stable |
| Python (if used) | Not required for Sprint 1.2 |

### 3.2 Manual test cases

#### Case 1: Load image via file picker (local disk)

| Field | Content |
|-------|---------|
| **Objective** | Verify user can open file picker, select PNG/JPG, and see image and metadata. |
| **Preconditions** | App running (`npm run tauri dev`); test images available (e.g. `tests/fixtures/valid_1x1.png`, or user’s own PNG/JPG). |
| **Steps** | 1. Click Load (or “Load image”). 2. In dialog, select a PNG or JPG file. 3. Confirm dialog closes and app shows preview and metadata. |
| **Expected result** | Image appears in preview area; dimensions (and file size if implemented) shown; no error toast. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 2: Load image via drag-and-drop

| Field | Content |
|-------|---------|
| **Objective** | Verify drag-and-drop of PNG/JPG onto drop zone loads the image. |
| **Preconditions** | App running; PNG or JPG file available in Explorer/finder. |
| **Steps** | 1. Drag a PNG or JPG file over the image/drop zone. 2. Observe visual feedback (e.g. highlight). 3. Drop the file. 4. Confirm image loads and preview updates. |
| **Expected result** | Drop zone shows drag feedback; on drop, image loads and displays; metadata updated. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 3: Load from different folder / drive

| Field | Content |
|-------|---------|
| **Objective** | Verify loading from another folder or drive (e.g. D:\, or Documents). |
| **Preconditions** | App running; PNG/JPG on another folder or drive. |
| **Steps** | 1. Click Load. 2. Navigate to different folder or drive and select image. 3. Confirm load succeeds. |
| **Expected result** | Image loads and displays regardless of location (path validation allows user-chosen paths). |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 4: Invalid file — non-image with image extension

| Field | Content |
|-------|---------|
| **Objective** | Verify app rejects non-image file (e.g. text file renamed to .png) with clear error. |
| **Preconditions** | App running; use `tests/fixtures/invalid_not_an_image.png` or similar. |
| **Steps** | 1. Click Load (or drag-drop). 2. Select invalid_not_an_image.png. 3. Observe response. |
| **Expected result** | Error message shown (e.g. invalid format or corrupt); no crash; no image displayed. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 5: Corrupt image file

| Field | Content |
|-------|---------|
| **Objective** | Verify app handles corrupt/truncated image without panic. |
| **Preconditions** | App running; use `tests/fixtures/corrupt_truncated.png`. |
| **Steps** | 1. Load corrupt_truncated.png via picker or drop. 2. Observe response. |
| **Expected result** | Error returned; user-facing message; no crash. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

#### Case 6: Oversized image (>8K) — downsampling

| Field | Content |
|-------|---------|
| **Objective** | Verify image >8192 on a dimension is downsampled and message shown if applicable. |
| **Preconditions** | App running; image with width or height >8192 (e.g. generate with ImageMagick per fixtures README). |
| **Steps** | 1. Load the oversized image. 2. Check dimensions shown and any “downsampled” message. 3. Confirm preview shows image. |
| **Expected result** | Image loads; dimensions ≤8192; aspect ratio preserved; UI indicates downsampling if applicable. |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

### 3.3 Regression / smoke

- [ ] App starts (`npm run tauri dev`)
- [ ] Load image and Export (stub) buttons respond
- [ ] No console errors on main screen after load

---

## 4. Artefacts and sign-off

| Artefact | Path | Owner |
|----------|------|--------|
| Manual test results | `SPRINTS/Sprint_1_2/MANUAL_TEST_REPORT.md` | Quality Engineer |
| Verification checklist | `SPRINTS/Sprint_1_2/VERIFICATION_CHECKLIST.md` | Sprint lead / Architect |
| Gotchas | `SPRINTS/Sprint_1_2/GOTCHAS.md` | Any agent; merge to `RESEARCH/GOTCHAS.md` |
| Test dataset | `tests/fixtures/`, `tests/fixtures/README.md` | QA-101 |

**Process:** Complete manual tests → fill `MANUAL_TEST_REPORT.md` → run through `VERIFICATION_CHECKLIST.md` before marking sprint complete.

---

## 5. References

- **Task list:** `SPRINTS/Sprint_1_2/SPRINT_1_2_Task_Assignment.md`
- **PRD:** `prd.md` (F1.1 acceptance criteria)
- **Testing strategy:** `todo.md` § Testing Strategy, § CI/CD Pipeline
- **Fixtures:** `tests/fixtures/README.md`
- **CLAUDE.md:** Testing commands (cargo test, npm test)

---

**Document Version:** 1.0  
**Template:** `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Prepared by:** Quality Engineer (Sprint 1.2)
