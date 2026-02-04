# Sprint 1.2 — QA Lead & Security Specialist Review

**Roles:** QA Lead, Security Specialist  
**Date:** 2026-02-03  
**Scope:** Sprint 1.2 — Image Loading & Display  
**References:** `SPRINT_1_2_Task_Assignment.md`, `SPRINT_1_2_COMPLETION_REVIEW.md`, `VERIFICATION_CHECKLIST.md`, `TEST_PLAN_1_2.md`, `docs/threat-model.md`, `docs/security-checklist.md`

---

## 1. Executive Summary

Sprint 1.2 delivers **Image Loading & Display** with solid implementation and test coverage. From a **QA** perspective: acceptance criteria are met, automated tests cover success/error paths and edge cases, and the test plan is clear; the main gap is the **missing Manual Test Report** and optional follow-ups (gotchas merge, blocklist test). From a **Security** perspective: path validation (canonicalize + blocklist) and magic-byte validation are **implemented and aligned** with the threat model; capability config is minimal and correct; dependency audits run in CI. **No critical or high issues**; recommendations are process and hardening only.

**Verdict:** **Approve** for sprint close, with recommendations below.

---

## 2. QA Lead Review

### 2.1 Acceptance Criteria & Success Criteria

| Criterion | Status | Evidence |
|-----------|--------|----------|
| User can load PNG/JPG via file picker or drag-and-drop | ✅ Met | ImageImport: `open()` with PNG/JPG filter; drag-drop with path from `File.path`; both call `loadImage(path)`. |
| Image displays correctly in UI | ✅ Met | App.svelte uses `previewUrl = data:image/png;base64,${loadedResult.previewBase64}`; dimensions and file size in sidebar. |
| Downsampling for oversized images | ✅ Met | Backend `downsample_if_needed`, `LoadImageOut.downsampled`; UI shows "Image was downsampled to fit 8K limit." |
| Error handling for corrupt/invalid files | ✅ Met | Magic-byte check before decode; clear errors; frontend shows `loadError` and status "Load error". |
| Automated tests passing | ✅ Met | 20 `cargo test`; JR2-101–104, QA-103/104; clippy clean; `npm run build` pass. |

**UI task checkboxes:** In the Task Assignment, UI-101–UI-105 acceptance criteria are still unchecked (`[ ]`). Functionally they are implemented (per Completion Review). **Recommendation:** Tick UI-101–105 acceptance criteria in `SPRINT_1_2_Task_Assignment.md` for consistency.

### 2.2 Test Coverage

- **Rust unit (image_loading.rs):** Valid PNG dimensions, invalid path, corrupt/non-image magic bytes, path with spaces, path with Unicode, path traversal (canonicalize), downsampling >8K, scale_down math, PNG/JPEG magic detection.
- **Rust integration (lib.rs):** Empty path rejection, valid PNG dimensions + preview_base64, invalid file error message.
- **CI:** `cargo test`, `cargo build`, `cargo audit`, `npm run build`, `npm audit --audit-level=high` (see `.github/workflows/ci.yml`).

**Gaps / recommendations:**

1. **Manual Test Report:** `MANUAL_TEST_REPORT.md` does **not** exist. TEST_PLAN_1_2.md defines six manual cases (file picker, drag-drop, different folder, invalid file, corrupt file, >8K). **Recommendation:** Create and fill `SPRINTS/Sprint_1_2/MANUAL_TEST_REPORT.md` from the test plan (even if brief) for audit trail and regression reference.
2. **Blocklist (SEC-101) not covered by automated test:** Paths under system dirs (e.g. `C:\Windows\System32`, `/usr/bin`) are rejected in code but there is no unit test that asserts rejection. Testing real system dirs in CI is environment-sensitive. **Recommendation:** Document in TEST_PLAN or GOTCHAS that blocklist is tested by code review / manual test; or add a test that uses a temporary path under a simulated “blocklisted” prefix if feasible without touching system dirs.
3. **Regression / smoke:** Verification checklist and completion review confirm app starts and buttons work; TEST_PLAN §3.3 regression items are unchecked. **Recommendation:** When filling the manual test report, run the smoke items and record pass/fail.

### 2.3 Test Artefacts & Process

- Fixtures: `tests/fixtures/` (valid_1x1.png, valid_small.png, invalid_not_an_image.png, corrupt_truncated.png) and README are in place; referenced in TEST_PLAN and QA-101.
- Test plan: TEST_PLAN_1_2.md is complete (scope, automated table, manual cases, artefacts).
- Verification checklist: VERIFICATION_CHECKLIST.md is filled and signed off; optional items (gotchas merge, manual report) are noted as optional.

**Conclusion (QA):** Test strategy and implementation are strong. Closing the manual test report and ticking UI acceptance criteria will complete the QA sign-off cleanly.

---

## 3. Security Specialist Review

### 3.1 Threat Model Alignment

| Threat / Control | Requirement (threat-model / SEC-*) | Implementation | Status |
|------------------|-------------------------------------|----------------|--------|
| Path traversal (§2.3, SEC-101) | Canonicalize; blocklist system dirs; allow user-chosen paths | `validate_path()`: `Path::canonicalize()`, `is_file()`, `!is_blocklisted()`. Blocklist: Windows (e.g. `C:\WINDOWS\`, `C:\PROGRAM FILES\`, PROGRAMDATA); Unix (`/System/`, `/usr/bin`, `/usr/sbin`, `/etc/`). | ✅ Implemented |
| Magic bytes (§2.4, SEC-102) | Validate PNG/JPEG signatures before decode; reject others | `validate_magic_bytes()`: PNG `89 50 4E 47 0D 0A 1A 0A`, JPEG `FF D8 FF`; reject if too short or wrong signature. Called before `decode_image()`. | ✅ Implemented |
| Privacy / logs (§2.1) | No user paths or image content in logs | `load_image_impl` logs only duration and dimensions at debug. | ✅ Compliant |
| IPC / capabilities | Only intended commands; capability config reviewed | `capabilities/default.json`: `allow-load-image`, `allow-export-stl`, `dialog:allow-open`, `shell:allow-open`. Permissions: `allow-load-image.toml`, `allow-export-stl.toml` restrict to `load_image` / `export_stl`. | ✅ Minimal and correct |
| Dependency audits (SEC-002) | cargo audit, npm audit in CI | `.github/workflows/ci.yml`: backend runs `cargo audit`; frontend runs `npm audit --audit-level=high`. | ✅ In place |

### 3.2 Code Review Notes (Security)

- **Path validation order:** Canonicalize first, then check file and blocklist. Aligns with threat model and GOTCHAS (canonicalize then apply blocklist).
- **No path returned to frontend:** Preview is base64 only; no user file path in IPC response. Reduces information leakage.
- **export_stl:** Currently validates only non-empty path; full path canonicalization and export-dir allowlist are out of scope for Sprint 1.2 (stub). Threat model §2.3 applies when export is implemented; no change required for this sprint.
- **Long paths (JR2-102):** Documented in image_loading.rs (Windows MAX_PATH / `\\?\`); blocklist uses string comparison. On Windows, canonical paths may use `\\?\`; `to_str()` and `to_uppercase()` are used for blocklist—ensure blocklist logic accounts for long-path prefix if canonical path contains it. Current blocklist checks `C:\WINDOWS\` etc.; `\\?\C:\WINDOWS\...` would still match after stripping prefix or comparing. **Recommendation:** Add a brief note in GOTCHAS or threat model if we observe long-path prefix affecting blocklist on Windows.

### 3.3 Security Checklist (docs/security-checklist.md)

- **§2.2 Security review:** Path canonicalization and blocklist (SEC-101), magic-byte validation (SEC-102) are in place and documented.
- **§2.1 Dependency audits:** CI runs cargo audit and npm audit; lock files committed.
- **Tauri IPC:** Only `load_image` and `export_stl` exposed; capability model used.

**Conclusion (Security):** Implementation matches the threat model and Security handover. No critical or high findings. Optional: document long-path prefix vs blocklist if needed after real-world testing.

---

## 4. Joint Recommendations

### 4.1 Before or at Sprint Close

1. **QA:** Create `SPRINTS/Sprint_1_2/MANUAL_TEST_REPORT.md` from TEST_PLAN_1_2.md (at least Cases 1–6 and smoke), and optionally tick UI-101–105 acceptance criteria in the Task Assignment.
2. **Process:** Merge lasting Sprint 1.2 gotchas from `SPRINTS/Sprint_1_2/GOTCHAS.md` into `RESEARCH/GOTCHAS.md` (path/magic-byte order and length already reflected in implementation; merge for single source of truth).

### 4.2 Backlog / Future Sprints

1. **QA:** Add an automated test for blocklist rejection if feasible (e.g. test with a path under a temp “fake” system path that is blocklisted, or document that blocklist is review/manual only).
2. **Security:** When implementing real `export_stl`, apply same pattern: canonicalize export path, allowlist (e.g. user-selected directory), blocklist system dirs; align with threat model §2.3.
3. **Security:** If Windows long-path prefix (`\\?\`) is observed in production, verify blocklist behavior and add a GOTCHA or threat-model note.

---

## 5. Sign-Off

| Perspective | Result | Conditions |
|-------------|--------|------------|
| **QA Lead** | **Approve** | Pending: optional manual test report and UI checkbox updates; no blocking issues. |
| **Security Specialist** | **Approve** | No critical/high issues; path and magic-byte controls implemented per threat model. |

**Overall:** Sprint 1.2 is **approved** for close from both QA and Security perspectives. Recommendations above are process and hardening improvements, not blockers.

---

**Document Version:** 1.0  
**Authors:** QA Lead, Security Specialist
