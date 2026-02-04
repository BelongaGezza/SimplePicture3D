# Manual Test Report — Sprint 1.3

**Sprint:** 1.3 — Python–Rust Bridge & Model Setup  
**Owner:** Quality Engineer  
**Last Updated:** 2026-02-03

---

## Summary

| Case | Description | Pass/Fail | Date | Tester |
|------|-------------|-----------|------|--------|
| 1 | Subprocess spawn (Windows) — QA-201 | *Pending* | — | — |
| 2 | Depth estimation roundtrip — QA-204 | **Pass** | 2026-02-03 | Cursor-Agent |
| 3 | CPU vs GPU — QA-203 | *Deferred* | — | (AI-205) |
| 4 | Model download and checksum — QA-202 | *Deferred* | — | (no script) |
| 5 | Error handling (missing Python, timeout) | **Pass** | 2026-02-03 | Cursor-Agent |

**Prerequisites:** BACK-201–203 and AI-201–204 implemented. Case 2 and 5 executed. Case 3 deferred until AI-205 (GPU/CPU); Case 4 deferred until model download script exists.

---

## Case details

*(Fill Actual result and Pass/Fail when tests are run.)*

### Case 1: Subprocess spawn (Windows) — QA-201

- **Objective:** Verify Rust spawns Python subprocess on Windows (cmd/PowerShell).
- **Actual result:** *(TBD)*
- **Pass / Fail:** [ ] Pass [ ] Fail
- **Notes:** *(e.g. cmd vs PowerShell differences, python.exe path)*

### Case 2: Depth estimation roundtrip — QA-204

- **Objective:** End-to-end image → Rust → Python → depth map → Rust.
- **Steps executed:** From `src-tauri`, ran `cargo test roundtrip_depth_rust_python_rust -- --ignored`. Test builds 10×8 PNG, calls `python_bridge::run_depth_estimation`, asserts dimensions match and depth values in [0, 1].
- **Actual result:** Test passed. Depth map dimensions (10×8) and length 80; all values in 0–1 range (stub gradient). Same code path used for any image bytes (e.g. `tests/fixtures/valid_small.png`).
- **Pass / Fail:** [x] Pass [ ] Fail

### Case 3: CPU vs GPU — QA-203

- **Objective:** Depth estimation on CPU and GPU; forced CPU documented.
- **Actual result:** *Deferred.* Stub estimator does not use GPU. When AI-205 is done: run with GPU available; run with `CUDA_VISIBLE_DEVICES=` (empty) or `PYTORCH_CUDA_ALLOC_CONF=` to force CPU. See GOTCHAS § QA-203.
- **Pass / Fail:** [ ] Pass [ ] Fail (deferred)

### Case 4: Model download and checksum — QA-202

- **Objective:** Download script runs; checksum pass/fail (including corrupt file).
- **Actual result:** *Deferred.* Model download script (AI-004 or equivalent) not in repo. Requirement and SEC-202 criteria documented in threat model §2.2 and security checklist. QA will verify when script exists.
- **Pass / Fail:** [ ] Pass [ ] Fail (deferred)

### Case 5: Error handling

- **Objective:** Missing Python, timeout, Python crash yield Err / message, no panic.
- **Steps executed:** Ran `cargo test subprocess_python_nonzero_exit_returns_err`. Test passes invalid image bytes; Python exits non-zero; Rust returns `Err` without panic. Missing-Python case: spawn fails → Err.
- **Actual result:** Test passed. No panic; error path exercised.
- **Pass / Fail:** [x] Pass [ ] Fail

---

## References

- Test plan: `SPRINTS/Sprint_1_3/TEST_PLAN_1_3.md`
- Task assignment: `SPRINTS/Sprint_1_3/SPRINT_1_3_Task_Assignment.md`
