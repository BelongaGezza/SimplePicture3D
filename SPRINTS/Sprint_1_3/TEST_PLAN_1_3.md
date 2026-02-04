# Sprint Test Plan — Sprint 1.3

**Source:** `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Sprint:** Sprint 1.3 — Python–Rust Bridge & Model Setup  
**Last Updated:** 2026-02-03

---

## 1. Scope

| Item | Description |
|------|--------------|
| Sprint goal | Establish communication between Rust backend and Python AI inference. |
| Features in scope | Python subprocess spawn; image pass-through (temp file/stdin); depth map parse; error/timeout handling; progress reporting; depth estimator CLI; model download checksum; CPU/GPU inference. |
| Out of scope | UI for depth (Sprint 1.4); mesh generation; STL export; full first-run wizard. |

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
| Rust → Python → Rust roundtrip | src-tauri or tests/ | JR2-201: integration test, image in → depth out |
| Subprocess error handling | src-tauri | JR2-202: Python crash / non-zero exit handled |
| (Optional) Python depth script | python/ | Pytest for depth estimator CLI output format |

*Fixtures:* Use `tests/fixtures/valid_small.png` or in-test generated image for roundtrip. Integration test may be `#[ignore]` or feature-gated when Python env not present in CI.

### 2.3 Coverage

- **Rust:** Existing `cargo test`; new tests for subprocess spawn, parse, timeout.
- **Python:** Add `pytest` when depth estimator module exists (Researcher / Junior 3D).

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | Windows 10/11 (primary); macOS/Linux as available |
| Node version | 20 |
| Rust version | stable |
| Python | 3.10+; venv with dependencies; Depth-Anything-V2 model available |

### 3.2 Manual test cases

#### Case 1: Subprocess spawn (Windows) — QA-201
- **Objective:** Verify Rust spawns Python subprocess on Windows (cmd.exe and PowerShell).
- **Preconditions:** BACK-201 implemented; Python 3.10+ on PATH or in venv (`VIRTUAL_ENV` set).
- **Steps:**
  1. From **cmd.exe**: `cd` to project root, run `cargo test --manifest-path src-tauri/Cargo.toml subprocess_python` (or run the roundtrip test with `--ignored`); confirm Python process starts and depth/error is returned.
  2. From **PowerShell**: same; note any difference in `python`/`python.exe` resolution or working directory.
  3. Document: path to python.exe, working dir behavior in GOTCHAS if needed.
- **Expected:** No spawn failure in either shell; Python runs and produces output (or clear Err if Python/env missing).
- **Owner:** QA-201.
- **Notes:** Rust uses `std::process::Command` (no shell), so cmd vs PowerShell only affects how the *test runner* is invoked, not the subprocess. Windows: Python from `VIRTUAL_ENV\Scripts\python.exe` when set; else `python` from PATH. See `SPRINTS/Sprint_1_3/GOTCHAS.md` § QA-201.

#### Case 2: Depth estimation roundtrip
- **Objective:** End-to-end: image file → Rust → Python → depth map → Rust.
- **Steps:** Use fixture or small PNG; invoke Rust code that calls Python; read parsed depth map; check dimensions and value range.
- **Expected:** Depth map dimensions match (or match documented downsampling); values in 0–1 range; no NaNs.
- **Owner:** QA-204.

#### Case 3: CPU vs GPU
- **Objective:** Depth estimation runs on CPU and, when available, GPU.
- **Steps:** Run with GPU available; run with GPU disabled (e.g. env var); compare success and timing.
- **Expected:** Both paths complete; no crash; optional ETA/progress.
- **Owner:** QA-203.

#### Case 4: Model download and checksum
- **Objective:** Model download script works; checksum validation passes/fails correctly.
- **Steps:** Run download script; verify checksum step; optionally corrupt file and re-run checksum.
- **Expected:** Pass on good download; fail with clear message on corrupt file.
- **Owner:** QA-202.

#### Case 5: Error handling (missing Python, timeout)
- **Objective:** Graceful handling when Python missing or process times out.
- **Steps:** Point Rust to invalid Python path; run with very short timeout; trigger Python exception.
- **Expected:** Rust returns Err / user-facing message; no panic.
- **Owner:** JR2-202, BACK-204.

### 3.3 Regression

- [ ] `cargo test` still passes (existing 1.2 tests)
- [ ] `npm run build` still passes
- [ ] Image load (Sprint 1.2) still works

---

## 4. Artefacts and sign-off

| Artefact | Path | Owner |
|----------|------|--------|
| Manual test results | `SPRINTS/Sprint_1_3/MANUAL_TEST_REPORT.md` | Quality Engineer |
| Verification checklist | `SPRINTS/Sprint_1_3/VERIFICATION_CHECKLIST.md` | Sprint lead / Architect |
| Gotchas | `SPRINTS/Sprint_1_3/GOTCHAS.md` | Any agent; merge to `RESEARCH/GOTCHAS.md` |

---

## 5. References

- **Task list:** `SPRINTS/Sprint_1_3/SPRINT_1_3_Task_Assignment.md`
- **PRD:** `prd.md` F1.2 (AI Depth Estimation)
- **Architecture:** ARCH-101–104 (protocol, data format, data flow)
- **CLAUDE.md:** Testing commands

---

**Document Version:** 1.0  
**Template:** `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Prepared by:** System Architect (Sprint 1.3 tasking)
