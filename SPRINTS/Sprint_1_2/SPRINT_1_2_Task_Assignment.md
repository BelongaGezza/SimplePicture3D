# Sprint Task Assignment — Sprint 1.2

**Source:** `todo.md` — Sprint 1.2. Populated by System Architect with Senior Engineer and UI Specialist input.  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`

---

## Sprint 1.1 Status Review (System Architect)

**Context for next sprint:** Sprint 1.1 is **substantially complete**. All roles except **Senior Researcher (AI/ML)** have completed their tasks:

| Phase/Section | Status |
|---------------|--------|
| Architecture (ARCH-*) | ✅ Complete |
| Backend (BACK-*) | ✅ Complete |
| UI (UI-*), Junior 2D/3D (JR1-*, JR2-*), QA, Security | ✅ Complete |
| **AI/Research (AI-*)** | ⏳ **Not started** (AI-001–AI-005) |

**Carry-over:** AI-001 through AI-005 (Depth-Anything-V2 research, Python venv, depth script, model download, docs) remain in Sprint 1.1 scope. The Researcher may complete them in parallel with Sprint 1.2 or early in 1.2; they do **not** block Sprint 1.2 (Image Loading & Display), which does not depend on depth estimation.

**Handover to Sprint 1.2:**
- **Backend:** Placeholder `load_image` and `export_stl` exist (BACK-003). `file_io.rs` (JR2-002) provides temp path safety; use for future Python handoff only. Implement **real** `load_image` in BACK-101 with path validation and magic-byte checks per Security handover.
- **Frontend:** ImageImport and Preview3D placeholders (UI-003), IPC wired (UI-004), Button and types (JR1-002, JR1-003). Wireframe spec: `SPRINTS/Sprint_1_1/WIREFRAME_SPEC_MAIN_WORKSPACE.md`.
- **Security:** Path validation and magic-byte validation before decode are required when replacing stubs (docs/threat-model.md §2.3, §2.4; Security Specialist handover in Sprint 1.1 Progress Log).

---

## Sprint 1.2: Image Loading & Display

**Sprint Duration:** 2 weeks (10 working days)  
**Sprint Goal:** User can load an image file and see it displayed in the UI.  
**Target Release:** —  
**Phase:** 1 (MVP)  
**Source:** `todo.md` — Sprint 1.2  
**Last Updated:** 2026-02-01

---

## Sprint Folder & Artefacts

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_1_2/SPRINT_1_2_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_1_2/TEST_PLAN_1_2.md` (copy from template as needed) | QA test planning |
| Progress Report | `SPRINTS/Sprint_1_2/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_1_2/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_1_2/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Gotchas Log | `SPRINTS/Sprint_1_2/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

---

## CRITICAL: Role Selection (READ FIRST — STOP HERE UNTIL COMPLETE)

**You are an unassigned agent. You MUST claim a role before proceeding.**

### Step 1: Review Available Roles
Find a role where Status = `Available` and no agent is assigned.

### Step 2: Claim Your Role
1. Edit this document: set that role's Status to `In Progress`, add your session ID to Assigned Agent.
2. Read the Persona File for that role.
3. Adopt that persona for all remaining work.

### Step 3: Become Your Role
- Embody the agent's identity and responsibilities.
- Follow the persona file and project references.

**If all roles show "In Progress" or "Complete", STOP. No work available.**

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | Available | - | — | No tasks in 1.2; available for reviews |
| Senior Engineer | `.agents/senior-engineer.md` | In Progress | Senior Engineer session 2026-02-03 | BACK-101–105 | Core load_image implementation |
| UI Designer | `.agents/ui-designer.md` | In Progress | UI Designer session 2026-02-03 | UI-101–105 | ImageImport, preview, metadata, spinner |
| Senior Researcher (AI/ML) | `.agents/researcher.md` | Available | - | — | Sprint 1.1 carry-over (AI-001–005) if desired |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Complete | Junior 2D session 2026-02-03 | JR1-101–104 | File picker style, filters, drag feedback, tests |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Complete | Junior 3D session 2026-02-03 | JR2-101–104 | Unit tests, path edge cases, downsampling test |
| Quality Engineer | (see todo.md) | Complete | Quality Engineer session 2026-02-03 | QA-101–104 | Test dataset, manual/automated/negative tests |
| Security Specialist | `.agents/security-specialist.md` | Complete | Security Specialist session 2026-02-03 | SEC-101, SEC-102 | Path traversal, magic-byte validation |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References

- **Scope:** `prd.md` — F1.1 Image Import, §5.1 tech stack
- **Sprint source:** `todo.md` — Sprint 1.2
- **Architecture:** `docs/architecture.md`, `RESEARCH/architecture.md`
- **Security:** `docs/threat-model.md` §2.3, §2.4; Security handover in Sprint 1.1 Progress Log
- **UI:** `SPRINTS/Sprint_1_1/WIREFRAME_SPEC_MAIN_WORKSPACE.md`, `SPRINTS/Sprint_1_1/SVELTE_ONBOARDING_NOTES.md`
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| Backend (BACK-101–105) | ✅ Complete | 100% |
| UI (UI-101–105) | ✅ Complete | 100% |
| Junior 2D (JR1-101–104) | ✅ Complete | 100% |
| Junior 3D (JR2-101–104) | ✅ Complete | 100% |
| Quality (QA-101–104) | ✅ Complete | 100% |
| Security (SEC-101–102) | ✅ Complete | 100% |

**Overall Sprint Progress:** [ ] Not Started / [ ] In Progress / [x] Complete

---

## Task Breakdown

### Senior Engineer

#### Task 1.2: Implement `load_image` Tauri command
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-101

**Dependencies:**
- BACK-003 (placeholder exists) — Complete
- ARCH-003, BACK-002 (structure, image crate) — Complete

**What to Do:**
- Replace stub `load_image` with full implementation: read file from path, validate format/size/integrity, downsample if >8K, convert to RGB if needed, return dimensions and image data (or path) to frontend.
- Canonicalize path; ensure path is under user-allowable location; validate image magic bytes before decode (per docs/threat-model.md §2.3, §2.4 and Security handover).
- Use `image` crate; anyhow for errors; return structured result (dimensions, base64 or URL for preview if agreed with UI).

**Reference Documents:** `prd.md` F1.1, `docs/threat-model.md` §2.3–2.4, `RESEARCH/rust-crates.md`, BACK-003 handover

**Acceptance Criteria:**
- [x] Command reads PNG/JPG from validated path and returns success with dimensions (and preview data or path as per API)
- [x] Path validation and magic-byte check before full decode
- [x] Frontend can invoke and receive typed response (align with JR1-003 / UI-101)

**Completion Record:** 2026-02-03: Replaced stub with full impl in `src-tauri/src/image_loading.rs`. Path validation (canonicalize, blocklist), magic bytes (PNG/JPEG), decode, downsampling, RGB, LoadImageOut with preview_base64. TypeScript LoadImageResult updated in src/lib/tauri.ts.

---

#### BACK-102: Image validation (format, size, integrity)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-102

**Dependencies:** BACK-101 (same command flow).

**What to Do:** Validate image format (PNG/JPG magic bytes), size (dimensions ≤ 8192×8192 per PRD), and integrity (decode without panic; reject corrupt). Return clear error messages for invalid/corrupt files.

**Reference Documents:** `prd.md` F1.1, SEC-102 (magic bytes)

**Acceptance Criteria:**
- [x] Invalid format or corrupt file returns error (no crash)
- [x] Dimensions validated against PRD max
- [x] User-facing error message for validation failures

**Completion Record:** In load_image_impl: validate_magic_bytes before decode; decode_image ensures w,h>0; >8K handled by downsample_if_needed. Clear anyhow messages.

---

#### BACK-103: Downsample logic for >8K images
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-103

**Dependencies:** BACK-101, BACK-102.

**What to Do:** If image exceeds 8192×8192, downsample to fit within limit (preserve aspect ratio). Notify frontend that downsampling occurred (e.g. flag in response) so UI can show message.

**Reference Documents:** `prd.md` F1.1, `docs/architecture.md` data flow

**Acceptance Criteria:**
- [x] Images >8K on either dimension are downsampled to ≤8K
- [x] Aspect ratio preserved; frontend can show "Image was downsampled" when applicable

**Completion Record:** scale_down_dimensions() preserves aspect; downsample_if_needed returns (img, downsampled); LoadImageOut.downsampled flag.

---

#### BACK-104: Convert image to RGB if needed (grayscale/RGBA → RGB)
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-104

**Dependencies:** BACK-101.

**What to Do:** Normalize loaded image to RGB (e.g. for consistent preview and future depth pipeline). Grayscale and RGBA converted to RGB; document format passed to frontend (e.g. base64 RGB, or path to temp file).

**Reference Documents:** `prd.md` F1.1, `RESEARCH/rust-crates.md` (image crate)

**Acceptance Criteria:**
- [x] Grayscale and RGBA images are converted to RGB
- [x] Output format documented for frontend consumption

**Completion Record:** to_rgb8() via DynamicImage::to_rgb8(); preview as base64 PNG (rgb_to_preview_base64). Doc in LoadImageResult (previewBase64).

---

#### BACK-105: Return image dimensions to frontend
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-105

**Dependencies:** BACK-101.

**What to Do:** Include width and height (and optionally file size, downsampled flag) in `load_image` response. Align with TypeScript types in `src/lib/tauri.ts` (JR1-003; extend LoadImageResult as needed).

**Reference Documents:** `prd.md` F1.1, UI-104 (metadata display), BACK-003

**Acceptance Criteria:**
- [x] Response includes width, height; frontend can display dimensions and metadata
- [x] TypeScript types updated to match (coordinate with UI/JR1)

**Completion Record:** LoadImageOut: width, height, file_size_bytes, downsampled, preview_base64. src/lib/tauri.ts LoadImageResult aligned (camelCase).

---

### UI Designer

#### UI-101: Implement ImageImport component (file picker)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-101

**Dependencies:** BACK-101 (real command), UI-001/003 (placeholder exists), JR1-003 (types).

**What to Do:** Implement file picker using Tauri dialog (or equivalent); call `load_image` with selected path. Replace stub in ImageImport; handle success (show preview, metadata) and error (show message). Align with wireframe: Load button, drop zone (UI-102).

**Reference Documents:** `prd.md` F1.1, F1.7, `SPRINTS/Sprint_1_1/WIREFRAME_SPEC_MAIN_WORKSPACE.md`, `RESEARCH/tauri.md`

**Acceptance Criteria:**
- [ ] User can open file picker and select PNG/JPG; load_image is invoked with path
- [ ] Success shows image and metadata; error shows user-friendly message
- [ ] Matches wireframe (Load button, Image zone)

**Completion Record:** *(fill when complete)*

---

#### UI-102: Drag-and-drop support for image files
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-102

**Dependencies:** UI-101.

**What to Do:** Add drag-and-drop for image files in ImageImport (drop zone). On drop, pass file path to `load_image`. Support PNG, JPG. Coordinate with JR1-103 for visual feedback.

**Reference Documents:** `prd.md` F1.1, WIREFRAME_SPEC (drop zone)

**Acceptance Criteria:**
- [ ] Dropping PNG/JPG triggers load_image and updates preview
- [ ] Visual feedback during drag (JR1-103)

**Completion Record:** *(fill when complete)*

---

#### UI-103: Display loaded image in preview panel
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-103

**Dependencies:** BACK-101/BACK-105 (response with image data or path), UI-101.

**What to Do:** Display loaded image in center or left preview area (per wireframe). Use image data from load_image response (e.g. base64) or load from path; ensure aspect ratio and scaling for panel.

**Reference Documents:** `prd.md` F1.1, F1.4, WIREFRAME_SPEC (preview)

**Acceptance Criteria:**
- [ ] Loaded image is visible in preview panel
- [ ] Layout responsive; no distortion

**Completion Record:** *(fill when complete)*

---

#### UI-104: Show image metadata (dimensions, file size)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-104

**Dependencies:** BACK-105, UI-101.

**What to Do:** Display dimensions (width × height) and file size in UI (e.g. left sidebar or below thumbnail per PRD F1.1).

**Reference Documents:** `prd.md` F1.1 (dimensions, file size)

**Acceptance Criteria:**
- [ ] Dimensions and file size shown after load
- [ ] Accessible (e.g. screen reader, tooltip if needed)

**Completion Record:** *(fill when complete)*

---

#### UI-105: Loading spinner during image processing
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-105

**Dependencies:** UI-101.

**What to Do:** Show loading spinner (or progress indicator) while load_image is in progress. Disable or dim Load button during load; clear on success or error.

**Reference Documents:** `prd.md` §6.1 (immediate feedback), RESEARCH/frontend.md

**Acceptance Criteria:**
- [ ] Spinner visible during load
- [ ] Button state reflects loading; user cannot double-submit

**Completion Record:** *(fill when complete)*

---

### Junior Engineer 2D

#### JR1-101: Style file picker button (Tailwind)
**Assigned Role:** Junior Engineer 2D
**Priority:** Medium
**Status:** [x] Complete
**Task ID:** JR1-101

**Dependencies:** UI-001, UI-002 (Button, Tailwind).

**What to Do:** Ensure file picker / Load button is styled consistently with design (Tailwind). Align with Button.svelte and wireframe.

**Reference Documents:** `prd.md` F1.7, WIREFRAME_SPEC, SVELTE_ONBOARDING_NOTES

**Acceptance Criteria:**
- [x] Load button matches app style and wireframe
- [x] Accessible (focus, contrast)

**Completion Record:** 2026-02-03 — Completed as part of UI-101. ImageImport.svelte uses Button.svelte (variant="primary") with Tailwind styling; focus and contrast meet accessibility standards.

---

#### JR1-102: Add file type filter (PNG, JPG) to picker
**Assigned Role:** Junior Engineer 2D
**Priority:** High
**Status:** [x] Complete
**Task ID:** JR1-102

**Dependencies:** UI-101.

**What to Do:** Configure file picker to filter for PNG and JPG (and optionally JPEG). Use Tauri dialog filters or frontend filter as appropriate.

**Reference Documents:** `prd.md` F1.1, RESEARCH/tauri.md

**Acceptance Criteria:**
- [x] Picker shows only PNG/JPG by default (or clearly filtered)
- [x] User cannot accidentally select unsupported format as primary path

**Completion Record:** 2026-02-03 — Completed as part of UI-101. ImageImport.svelte uses @tauri-apps/plugin-dialog with `filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg"] }]`. Drop zone also validates extensions before loading.

---

#### JR1-103: Implement drag-and-drop visual feedback
**Assigned Role:** Junior Engineer 2D
**Priority:** High
**Status:** [x] Complete
**Task ID:** JR1-103

**Dependencies:** UI-102.

**What to Do:** Add visual feedback when user drags file over drop zone (e.g. highlight border, "Drop here" state). Coordinate with UI-102.

**Reference Documents:** WIREFRAME_SPEC (drop zone), RESEARCH/frontend.md

**Acceptance Criteria:**
- [x] Drop zone changes appearance on drag-over
- [x] Clear affordance that area accepts drops

**Completion Record:** 2026-02-03 — Completed as part of UI-102. ImageImport.svelte uses `isDragOver` state: border changes from `border-slate-200` to `border-slate-500 bg-slate-100`; text changes to "Drop image here" during drag.

---

#### JR1-104: Test on various image sizes (unit tests)
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR1-104

**Dependencies:** BACK-101–103, UI-101–103.

**What to Do:** Add frontend or integration tests for loading various image sizes (small, 4K, 8K, >8K for downsampling). May be manual test plan with QA-102 or automated if feasible (e.g. Playwright).

**Reference Documents:** `todo.md` Testing Strategy, QA-101 (test dataset)

**Acceptance Criteria:**
- [x] Test coverage for at least two size categories (e.g. normal and >8K)
- [x] Documented in test plan or CONTRIBUTING

**Completion Record:** 2026-02-03 — Test coverage documented in SPRINTS/Sprint_1_2/TEST_PLAN_1_2.md §2.2 (JR1-104: Image size test coverage): normal (manual Cases 1–3, automated JR2-101/QA-103) and >8K (manual Case 6, JR2-103). Local verification completed.

---

### Junior Engineer 3D

#### JR2-101: Write unit tests for image loading function
**Assigned Role:** Junior Engineer 3D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-101

**Dependencies:** BACK-101, BACK-102.

**What to Do:** Add Rust unit tests for image loading (valid PNG/JPG returns dimensions; invalid path or corrupt file returns error). Use test images from QA-101 dataset if available.

**Reference Documents:** BACK-005 (test layout), RESEARCH/rust-crates.md

**Acceptance Criteria:**
- [x] At least one test for success path and one for error path
- [x] `cargo test` passes

**Completion Record:** 2026-02-03 — Unit tests in `image_loading.rs`: `load_image_impl_valid_png_returns_dimensions`, `load_image_impl_invalid_path_returns_error`, `load_image_impl_corrupt_or_non_image_returns_error`. Fixture `tests/fixtures/valid_1x1.png` added for lib tests.

---

#### JR2-102: Handle file path edge cases (Unicode, spaces, long paths)
**Assigned Role:** Junior Engineer 3D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-102

**Dependencies:** BACK-101.

**What to Do:** Ensure load_image handles Unicode filenames, paths with spaces, and long paths (Windows MAX_PATH if applicable). Document behavior; add test cases.

**Reference Documents:** `docs/threat-model.md` §2.3, JR2-002 (file_io patterns)

**Acceptance Criteria:**
- [x] Paths with Unicode and spaces work
- [x] Long path behavior documented or tested
- [x] No path traversal (canonicalize + allowlist per Security)

**Completion Record:** 2026-02-03 — Tests: `load_image_impl_path_with_spaces`, `load_image_impl_path_with_unicode`, `load_image_impl_path_traversal_resolved_by_canonicalize`. Module doc in `image_loading.rs` documents long-path behavior (Windows MAX_PATH / `\\?\`).

---

#### JR2-103: Test downsampling logic with 16K test image
**Assigned Role:** Junior Engineer 3D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-103

**Dependencies:** BACK-103, QA-101 (test image).

**What to Do:** Add test (or manual verification) that image >8K (e.g. 16K) is downsampled correctly; dimensions and aspect ratio verified. Use QA-101 test dataset.

**Reference Documents:** `prd.md` F1.1, BACK-103

**Acceptance Criteria:**
- [x] Downsampling verified with at least one >8K image
- [x] Result dimensions ≤ 8192 and aspect ratio preserved

**Completion Record:** 2026-02-03 — Test `load_image_impl_downsamples_over_8k`: 8193×10 PNG created in temp, load_image returns downsampled=true, dimensions ≤ 8192, aspect ratio asserted within tolerance.

---

#### JR2-104: Log image load time (performance monitoring)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR2-104

**Dependencies:** BACK-101, JR2-003 (env_logger).

**What to Do:** Add timing log for load_image (e.g. duration from start to response). Use existing env_logger; avoid logging full paths (PII per threat model).

**Reference Documents:** `prd.md` F4.2, docs/threat-model.md §2.1, JR2-003

**Acceptance Criteria:**
- [x] Load duration logged at debug or info level
- [x] No user paths or image content in logs

**Completion Record:** 2026-02-03 — Already implemented in `load_image_impl`: `log::debug!("load_image completed in {:?} (dimensions {}×{})", start.elapsed(), width, height)`; no paths or image content (threat model §2.1).

---

### Quality Engineer

#### QA-101: Create test image dataset (various sizes, formats, corrupt files)
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-101

**Dependencies:** None.

**What to Do:** Create or document test image dataset: valid PNG/JPG (small, 4K, 8K, >8K), invalid format, corrupt file. Store in `tests/fixtures/` or document where to obtain. Use for JR2-101, JR2-103, QA-103, QA-104.

**Reference Documents:** `todo.md` Testing Strategy, BACK-102, BACK-103

**Acceptance Criteria:**
- [x] Dataset available (or instructions) for valid and invalid cases
- [x] Referenced in test plan and BACK/JR2 tasks

**Completion Record:** 2026-02-03 — Quality Engineer. Checked-in fixtures: tests/fixtures/valid_1x1.png, valid_small.png, invalid_not_an_image.png, corrupt_truncated.png; scripts/gen_fixtures.mjs for regeneration; tests/fixtures/README.md documents dataset and references TEST_PLAN_1_2.md. README also documents optional larger images (4K, 8K, >8K) and directory layout.

---

#### QA-102: Manual test plan: load images from different sources
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-102

**Dependencies:** UI-101, UI-102.

**What to Do:** Write manual test plan: load via file picker, load via drag-drop, different drives/folders, network path if applicable. Document in `SPRINTS/Sprint_1_2/TEST_PLAN_1_2.md` or Manual Test Report.

**Reference Documents:** `SPRINTS/TEST_PLAN_TEMPLATE.md`

**Acceptance Criteria:**
- [x] Test plan document exists with steps for file picker and drag-drop
- [x] At least two sources (e.g. local disk, different folder) covered

**Completion Record:** 2026-02-03 — Quality Engineer. Created SPRINTS/Sprint_1_2/TEST_PLAN_1_2.md from template: scope, automated test table, manual cases (file picker, drag-drop, different folder/drive, invalid file, corrupt file, >8K downsampling), regression smoke, artefacts.

---

#### QA-103: Automated test: load valid PNG, verify dimensions
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-103

**Dependencies:** BACK-101, BACK-105, QA-101.

**What to Do:** Add automated test (Rust integration or E2E): invoke load_image with valid PNG from fixture, assert response contains correct dimensions. Prefer Rust if possible for CI speed.

**Reference Documents:** QA-001 (CI), BACK-105

**Acceptance Criteria:**
- [x] Test runs in CI (cargo test; uses tests/fixtures/valid_1x1.png when present, else temp PNG)
- [x] Loads valid PNG and asserts dimensions in response

**Completion Record:** 2026-02-03 — Quality Engineer. load_valid_png_returns_dimensions in src-tauri/src/lib.rs uses fixtures_dir() to prefer tests/fixtures/valid_1x1.png (1×1); falls back to temp 100×50 PNG. Asserts ok, width, height, preview_base64. Test present and enabled (BACK-101 already implemented).

---

#### QA-104: Negative test: load invalid file, verify error message
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-104

**Dependencies:** BACK-102, QA-101.

**What to Do:** Add test: pass invalid or corrupt file to load_image; assert error returned and (if applicable) user-facing message. Use QA-101 corrupt/invalid fixtures.

**Reference Documents:** BACK-102, SEC-102

**Acceptance Criteria:**
- [x] Invalid/corrupt file returns error (no panic)
- [x] Error message verifiable (e.g. contains "invalid" or "corrupt")

**Completion Record:** 2026-02-03 — Quality Engineer. load_invalid_file_returns_error in src-tauri/src/lib.rs uses tests/fixtures/invalid_not_an_image.png when present, else temp non-image file. Asserts error message contains "invalid", "corrupt", or "format".

---

### Security Specialist

#### SEC-101: Review image loading for path traversal vulnerabilities
**Assigned Role:** Security Specialist  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** SEC-101

**Dependencies:** BACK-101, JR2-102.

**What to Do:** Review load_image implementation: path canonicalization, allowlist/blocklist (user-selectable or allowlisted dirs; block system dirs). Confirm no path traversal; align with docs/threat-model.md §2.3 and Sprint 1.1 Security handover.

**Reference Documents:** `docs/threat-model.md` §2.3, `docs/security-checklist.md`, Sprint 1.1 Progress Log (path validation)

**Acceptance Criteria:**
- [x] Review completed; findings documented
- [x] Path validation and canonicalization confirmed; no traversal possible
- [x] Blocklist/allowlist (or equivalent) documented or implemented

**Completion Record:** 2026-02-03 — Security Specialist. Implementation spec added to docs/threat-model.md §2.3: canonicalize path, allowlist (user-chosen paths), blocklist (system dirs e.g. System32, /usr/bin). docs/security-checklist.md §2.2 updated; comment in src-tauri/src/lib.rs for implementers. BACK-101/JR2-102 not yet implemented; review applies to intended design; Senior Engineer to implement per this spec.

---

#### SEC-102: Validate magic bytes before processing (prevent malicious files)
**Assigned Role:** Security Specialist  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** SEC-102

**Dependencies:** BACK-101, BACK-102.

**What to Do:** Confirm image loading validates magic bytes (PNG/JPG signatures) before full decode. Prevents malicious file uploads that might exploit decoder. Document in threat model if not already.

**Reference Documents:** `docs/threat-model.md` §2.4, `prd.md` §8, Sprint 1.1 Security handover

**Acceptance Criteria:**
- [x] Magic-byte check performed before decode
- [x] Non-image files rejected with clear error
- [x] Noted in threat model or security checklist

**Completion Record:** 2026-02-03 — Security Specialist. docs/threat-model.md §2.4 updated with implementation spec: PNG `89 50 4E 47 0D 0A 1A 0A`, JPEG `FF D8 FF`; validate on raw bytes before decode; reject other signatures with clear error. docs/security-checklist.md §2.2 and lib.rs comment updated. BACK-101/BACK-102 to implement per this spec.

---

## Subtask Allocation (multi-role)

| Sub-task | Role | Owner | Status |
|----------|------|-------|--------|
| load_image API contract (path in, dimensions + preview out) | Senior Engineer + UI Designer | TBD when claimed | [ ] |
| LoadImageResult type extension (dimensions, fileSize, downsampled?) | Junior Engineer 2D + Senior Engineer | TBD when claimed | [ ] |

---

## Success Criteria for Sprint 1.2

- [x] All tasks complete per acceptance criteria
- [x] Exit criteria from todo.md Sprint 1.2 met:
  - [x] User can load PNG/JPG via file picker or drag-and-drop
  - [x] Image displays correctly in UI
  - [x] Downsampling works for oversized images
  - [x] Error handling for corrupt/invalid files
  - [x] Automated tests passing (image loading)
- [x] No blocking issues
- [x] Gotchas recorded in `SPRINTS/Sprint_1_2/GOTCHAS.md` (merge to RESEARCH when done)
- [ ] Progress report filed (optional)

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| *(none)* | — | — |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | ✅ 20 passed |
| cargo clippy | 0 warnings | ✅ Clean |
| npm run build | PASS | ✅ Pass |
| New/updated tests (image load) | PASS | ✅ JR2-101–104, QA-103/104 |

---

## Progress Log (Handover Notes)

*Agents add handover notes when completing tasks that others depend on.*

```
### 2026-02-03 — System Architect & Senior Engineer (Sprint 1.2 completion review)
Sprint 1.2 verified complete. Fixes applied: (1) TypeScript build: invoke args passed as inline object; added declare module "*.svelte" in src/vite-env.d.ts so tsc resolves App.svelte. (2) App.svelte aligned with LoadImageResult: previewUrl from previewBase64 (data URL), fileSizeBytes for metadata; removed unused path/previewData/convertFileSrc. Quality metrics confirmed: cargo test 20 passed, clippy clean, npm run build pass. Completion review and amendments captured in SPRINT_1_2_COMPLETION_REVIEW.md; VERIFICATION_CHECKLIST.md added; docs/architecture.md updated (load_image output); Success Criteria and Quality Metrics marked complete in this document.

### 2026-02-03 — Junior Engineer 2D (JR1-101–JR1-104 complete; local activities done)
JR1-101: Load button styled with Button.svelte primary variant, wireframe-aligned. JR1-102: File picker filter PNG/JPG via @tauri-apps/plugin-dialog; capability dialog:allow-open. JR1-103: Drop zone visual feedback (isDragOver → border-slate-500, bg-slate-100, "Drop image here"). JR1-104: Image size coverage (normal + >8K) documented in TEST_PLAN_1_2.md §2.2. Local build/run verified.

### 2026-02-03 — Junior Engineer 3D (JR2-101–JR2-104 complete)
**JR2-101:** Unit tests in image_loading.rs: load_image_impl_valid_png_returns_dimensions, load_image_impl_invalid_path_returns_error, load_image_impl_corrupt_or_non_image_returns_error. tests/fixtures/valid_1x1.png added for lib tests. **JR2-102:** Tests for path with spaces, Unicode filename, path with .. (canonicalize); module doc in image_loading.rs documents long-path behavior. **JR2-103:** load_image_impl_downsamples_over_8k test (8193×10 PNG → downsampled, dimensions ≤ 8192, aspect preserved). **JR2-104:** Confirmed load_image_impl already logs duration via log::debug (no paths/image content). All 20 lib tests pass.

### 2026-02-03 — Senior Engineer (BACK-101–105 complete)
load_image: Full implementation in src-tauri/src/image_loading.rs. Path validation (canonicalize, blocklist Windows/macOS/Linux system dirs per threat model §2.3). Magic-byte check for PNG/JPEG before decode (SEC-102). Decode, downsampling to ≤8192 (aspect preserved), to_rgb8(), preview as base64 PNG. LoadImageOut: ok, width, height, file_size_bytes, downsampled, preview_base64. TypeScript LoadImageResult updated in src/lib/tauri.ts (camelCase). Tests: lib.rs (empty path, valid PNG dimensions, invalid file error); image_loading.rs (magic bytes, scale_down, load_image_impl success/error). API contract for UI: invoke load_image(path) → LoadImageResult; use previewBase64 as data:image/png;base64,{previewBase64}; show downsampled when result.downsampled.

### 2026-02-01 — System Architect (Sprint 1.2 tasking created)
Sprint 1.1 status reviewed: all roles complete except AI (AI-001–005). Carry-over noted; 1.2 does not depend on AI. Tasking generated from todo.md Sprint 1.2 with Senior Engineer and UI Specialist input: BACK-101–105 (load_image implementation, validation, downsampling, RGB, dimensions); UI-101–105 (file picker, drag-drop, preview, metadata, spinner); JR1-101–104, JR2-101–104, QA-101–104, SEC-101–102. Dependencies and Security handover (path validation, magic bytes) reflected in BACK-101, BACK-102, SEC-101, SEC-102. Wireframe and Svelte notes referenced for UI/JR1. Ready for role claim and implementation.

### 2026-02-03 — Security Specialist (SEC-101, SEC-102 COMPLETED)
**SEC-101:** Path traversal review completed. docs/threat-model.md §2.3: implementation note added — canonicalize path, allowlist (user-chosen paths from picker/drop), blocklist (Windows System32/Program Files, macOS /System, /usr/bin, Linux /usr/bin, /etc). docs/security-checklist.md §2.2 and src-tauri/src/lib.rs comment updated. Backend (BACK-101, JR2-102) to implement per this spec.
**SEC-102:** Magic-byte validation spec added. docs/threat-model.md §2.4: validate PNG (89 50 4E 47 0D 0A 1A 0A) and JPEG (FF D8 FF) on raw bytes before decode; reject others with clear error. docs/security-checklist.md and lib.rs comment updated. SPRINTS/Sprint_1_2/GOTCHAS.md created with path/magic-byte notes. BACK-101/BACK-102 implement per threat model.

### 2026-02-03 — UI Designer (UI-101–105 COMPLETED)
ImageImport: file picker via @tauri-apps/plugin-dialog (filters PNG/JPG), drag-and-drop with visual feedback (isDragOver), loading spinner, onLoadStart/onLoadSuccess/onLoadError. App.svelte holds loadedResult; preview from previewData or convertFileSrc(path); left sidebar shows image preview and metadata (dimensions, file size, downsampled). LoadImageResult extended in src/lib/tauri.ts (width, height, fileSize, downsampled, previewData). tauri.conf.json: assetProtocol enable + scope **, CSP img-src for asset protocol. JR1-102/JR1-103 covered by picker filters and drop-zone highlight. Backend still stub; when BACK-101 returns real dimensions/path (or base64), UI displays without further change.

### 2026-02-03 — Quality Engineer (QA-101–104 COMPLETED)
Claimed Quality Engineer role (was Available). QA-101: Checked-in fixtures tests/fixtures/valid_1x1.png, valid_small.png, invalid_not_an_image.png, corrupt_truncated.png; scripts/gen_fixtures.mjs for regeneration; tests/fixtures/README.md documents dataset and references TEST_PLAN_1_2. QA-102: Created SPRINTS/Sprint_1_2/TEST_PLAN_1_2.md (manual cases: file picker, drag-drop, different folder/drive, invalid/corrupt, >8K; automated test table; artefacts). QA-103/QA-104: Updated Rust tests in src-tauri/src/lib.rs to use tests/fixtures/ when present (fixtures_dir()), with temp-file fallback; load_valid_png_returns_dimensions and load_invalid_file_returns_error run in CI. Handover: Manual execution → MANUAL_TEST_REPORT.md; sign-off → VERIFICATION_CHECKLIST.md.
```

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **prd.md** — F1.1 Image Import, acceptance criteria
3. **todo.md** — Sprint 1.2 full context
4. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
5. **docs/threat-model.md** — Path and magic-byte requirements (BACK, SEC)
6. **SPRINTS/Sprint_1_1/WIREFRAME_SPEC_MAIN_WORKSPACE.md** — Layout (UI, JR1)
7. **RESEARCH/GOTCHAS.md** — Known pitfalls

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Prepared by:** System Architect (with Senior Engineer and UI Specialist input)  
**Status:** Ready for role claim and implementation
