# Senior Engineer Critical Review — Sprint 1.3

**Sprint:** 1.3 — Python–Rust Bridge & Model Setup  
**Reviewer:** Senior Engineer (per `.agents/senior-engineer.md`)  
**Date:** 2026-02-03  
**Purpose:** Critical assessment of implementation quality, gaps, and handover readiness for Sprint 1.4.

---

## Executive Summary

Sprint 1.3 is **substantially complete** and the bridge is **fit for use** by the upcoming `generate_depth_map` Tauri command (Sprint 1.4). All critical and high-priority tasks are implemented and tested. A few process/documentation items should be closed before considering the sprint fully signed off.

**Verdict:** **Approve with minor follow-ups** — no blocking technical issues; recommend completing the follow-ups below before sprint close.

---

## What Was Verified

### Code and Tests

- **`cargo test --manifest-path src-tauri/Cargo.toml`** — 22 passed, 2 ignored (roundtrip, benchmark). Confirmed locally.
- **`cargo clippy`** — 0 warnings (python_bridge uses `#![allow(dead_code)]` until 1.4; acceptable).
- **`python_bridge.rs`** — Read in full. Correct use of `std::process::Command` (no shell), path canonicalization and validation under temp dir, `TempFileGuard` cleanup, timeout loop with kill, stderr capture in thread, JSON parse with dimension check. No user-controlled argv (SEC-201 satisfied).
- **`file_io.rs`** — `write_temp_file` used with safe prefix/suffix; validation in bridge is additional defense-in-depth (canonicalize + under temp dir).
- **`depth_estimator.py`** — CLI contract matches ARCH-101/102; stub and Depth-Anything-V2 paths; PROGRESS/STAGE on stderr; device auto-detect; OOM and ImportError handled.
- **Integration test** — `roundtrip_depth_rust_python_rust` builds minimal PNG, calls `run_depth_estimation`, asserts dimensions and 0–1 range; correctly `#[ignore]` for CI without Python.
- **Error-handling test** — `subprocess_python_nonzero_exit_returns_err` passes in CI (invalid image → Python exit non-zero or spawn failure → `Result::Err`); no panic.

### Documentation and Security

- **docs/architecture.md** — Rust–Python bridge section present; data flow and contract align with implementation.
- **SEC-201** — Subprocess review completed; no user input in argv; path validated.
- **SEC-202** — Requirements documented; script verification deferred until model download script exists.

---

## Gaps and Risks

### 1. QA-201 / Case 1 — Windows manual test not executed

**Finding:** In `MANUAL_TEST_REPORT.md`, Case 1 (Subprocess spawn on Windows — cmd/PowerShell) is still **Pending**; Actual result is TBD.

**Impact:** Low. Rust uses `Command` without shell, so cmd vs PowerShell only affects how the *runner* invokes `cargo test` / `npm run tauri dev`, not how the child is spawned. GOTCHAS § QA-201 already documents this.

**Recommendation:** Have QA run Case 1 once on Windows (e.g. run `cargo test subprocess_python_nonzero_exit_returns_err` from both cmd and PowerShell, and optionally `cargo test roundtrip_depth_rust_python_rust -- --ignored` with venv active) and record Pass in MANUAL_TEST_REPORT. If not done before 1.4, document as “Accepted: manual run deferred; automated test passes on Windows.”

### 2. Verification checklist — `npm run build` unchecked

**Finding:** Verification checklist has “`npm run build` — run when needed” left unchecked.

**Impact:** Low. No Tauri command yet consumes the bridge; frontend is unchanged. Risk is only that a future change broke the build.

**Recommendation:** Run `npm run build` once (or ensure CI runs it) and check the box. Quick win for sign-off.

### 3. Gotchas not merged to RESEARCH/GOTCHAS.md

**Finding:** Sprint 1.3 GOTCHAS (Windows subprocess note, temp-file benchmark results, forced CPU for CI) are only in `SPRINTS/Sprint_1_3/GOTCHAS.md`. Verification checklist marks “merge notable gotchas to RESEARCH/GOTCHAS.md” as optional and not done.

**Impact:** Medium for long-term maintainability. New contributors and future sprints benefit from subprocess/CI/model notes in the main RESEARCH/GOTCHAS.md.

**Recommendation:** Merge the three substantive items (QA-201 Windows, JR2-203 benchmark, QA-203 forced CPU) into `RESEARCH/GOTCHAS.md` with a short “Sprint 1.3” or “Python–Rust bridge” subsection, then check the optional verification item.

### 4. todo.md not updated

**Finding:** `todo.md` still lists Sprint 1.3 tasks (ARCH-101–104, BACK-201–205, etc.) as unchecked.

**Impact:** Low; traceability and reporting only.

**Recommendation:** Update `todo.md` to mark Sprint 1.3 tasks complete (or add a single “Sprint 1.3 complete per SPRINTS/Sprint_1_3” note) so backlog state matches the sprint folder.

---

## Deferred Items (Accepted)

- **Model download script (AI-004 / SEC-202 / QA-202):** Not in repo; requirements and security criteria documented. Verification when script is implemented. **No change needed for 1.3 sign-off.**
- **QA-203 (CPU/GPU manual test):** Deferred with documented forced-CPU approach for CI. **Acceptable.**

---

## Handover to Sprint 1.4

- **Backend:** `python_bridge::run_depth_estimation(image_bytes)` and `run_depth_estimation_with_timeout` are ready. Sprint 1.4 should add a Tauri command (e.g. `generate_depth_map`) that: accepts image path or bytes per ARCH-102, calls `load_image` and/or writes temp file, calls `run_depth_estimation`, maps `RunDepthResult` (and progress/errors) to the frontend contract in `docs/architecture.md` and `UI_READINESS_1_4.md`.
- **Python:** Same CLI; no breaking changes required. Optional: add more progress points in real inference path if UI wants finer granularity.
- **Frontend:** No changes in 1.3; `UI_READINESS_1_4.md` describes progress, error, and result states.

---

## Summary Table

| Area              | Status        | Notes                                              |
|-------------------|---------------|----------------------------------------------------|
| Architecture      | Complete      | Protocol and data flow documented and implemented |
| Backend (Rust)    | Complete      | Spawn, temp file, parse, timeout, progress        |
| AI/Research       | Complete      | Stub + Depth-Anything-V2, device, OOM handling   |
| Tests             | Complete      | 22 pass, 2 ignored; roundtrip + error test       |
| Security          | Complete      | SEC-201 done; SEC-202 deferred with doc           |
| Manual QA         | Minor gap     | Case 1 (Windows) still Pending                    |
| Verification      | Minor gap     | npm build unchecked; gotchas not merged           |
| todo.md           | Out of date   | Sprint 1.3 tasks still unchecked                  |

---

## Recommendations (Priority Order)

1. **Before sprint close:** Run `npm run build`; check verification checklist.
2. **Before sprint close:** Merge Sprint 1.3 gotchas (Windows, benchmark, forced CPU) into `RESEARCH/GOTCHAS.md`.
3. **Before or early in 1.4:** Execute and record Case 1 (Windows) in MANUAL_TEST_REPORT, or explicitly accept deferral.
4. **Optional:** Update `todo.md` so Sprint 1.3 tasks are marked complete.

---

**Document Version:** 1.0  
**Next review:** Sprint 1.4 kickoff (backend command and frontend integration).
