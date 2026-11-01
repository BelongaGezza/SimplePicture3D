# Manual Test Report — Sprint D

**Sprint:** D — Validate and Harden  
**Tester:** _TBD_  
**Build:** _commit / version_  
**OS:** _Windows | macOS | Linux_  
**Date:** _TBD_

**Related procedures:** [QA-D-01_TEST_PROCEDURE.md](./QA-D-01_TEST_PROCEDURE.md) · [docs/testing.md](../../docs/testing.md) · [docs/engraver-charter.md](../../docs/engraver-charter.md) · [TEST_PLAN_D.md](./TEST_PLAN_D.md)

---

## QA-C-01 — Core workflow (`docs/testing.md` §1)

**Task:** QA-C-01 · **Status:** Not run — structure only (QA-D-07)  
**Checklist source:** [docs/testing.md](../../docs/testing.md) §1 — Core workflow (every sprint / before release)

### Environment (fill on execution)

| Field | Value |
|-------|--------|
| Git commit | |
| Build | `tauri dev` ☐ / release build ☐ |
| OS + version | |
| Stub depth (`SP3D_USE_STUB=1`) | Yes ☐ / No ☐ |
| Test images used | e.g. small PNG, 4K sample |

### §1 Core workflow results

| # | Step | How to verify (summary) | Pass? | Notes |
|---|------|-------------------------|-------|-------|
| 1 | Load image | PNG/JPG in 2D view; >8K downsamples with notice | ☐ | |
| 2 | Generate depth | Progress; grayscale depth map | ☐ | |
| 3 | Adjust depth | Brightness/contrast/gamma/invert; depth range mm | ☐ | |
| 4 | Set blank envelope | L×W×H mm + margin; preset or custom | ☐ | |
| 5 | Set sampling params | Pixel step, depth threshold; point estimate updates | ☐ | |
| 6 | Generate point cloud / 3D preview | Points in blank wireframe; rotate/zoom | ☐ | |
| 7 | Export PLY | File created; viewer or engraver pre-check | ☐ | |
| 8 | Export XYZ / CSV | Files created; column layout correct | ☐ | |
| 9 | Settings persistence | Restart restores blank, format, depth range | ☐ | |

### §1 Edge cases (optional)

| Case | Expected | Pass? | Notes |
|------|----------|-------|-------|
| Invalid/corrupt file | Error, no crash | ☐ | |
| Unusual aspect ratio | Depth + point cloud + export succeed | ☐ | |
| Cancel depth mid-run (if supported) | App recovers | ☐ | |

### QA-C-01 §1 summary

| Outcome | Select when |
|---------|-------------|
| ☐ **Pass** | All required §1 rows pass |
| ☐ **Fail** | Any required step fails |
| ☐ **Partial** | Core path works; edge cases deferred |

**Defects (BACK-D-01 / UI):** _Add rows below during execution._

| ID | Step # | Symptom | Expected | Actual | Evidence |
|----|--------|---------|----------|--------|----------|
| | | | | | |

---

## QA-C-01 — Export verification (`docs/testing.md` §2)

**Task:** QA-C-01 · **Status:** Not run — structure only  
**Checklist source:** [docs/testing.md](../../docs/testing.md) §2 — Export verification

### §2 Export verification results

| # | Step | How to verify (summary) | Pass? | Notes |
|---|------|-------------------------|-------|-------|
| 1 | PLY ASCII / binary | Opens in viewer/engraver; point count ≈ estimate | ☐ | |
| 2 | XYZ format | Three columns; mm units; within blank bounds | ☐ | |
| 3 | CSV format | Header row; comma-separated; imports cleanly | ☐ | |
| 4 | Engraver import (Phase D gate) | **Xtool Studio** → **Xtool F2**; axis + scale | ☐ | See § QA-D-01 below |
| 5 | Blank fit | Points inside wireframe / envelope with margin | ☐ | |

### QA-C-01 §2 summary

| Outcome | Select when |
|---------|-------------|
| ☐ **Pass** | §2 rows 1–3 and 5 pass; row 4 Pass or tracked in QA-D-01 |
| ☐ **Fail** | Export format or blank-fit failure |
| ☐ **Blocked** | §2 row 4 only — Xtool Studio/F2 not available this session |

**Overall QA-C-01 (§1 + §2):** ☐ Pass ☐ Fail ☐ Partial — **Date:** ___________

---

## QA-D-01 — Engraver import

**Task status:** Procedure **ready** · Engraver execution **ready** (XTool Studio + xTool F2 Ultra UV)  
**Charter:** [docs/engraver-charter.md](../../docs/engraver-charter.md) (draft, 2026-05-19)

### Environment (fill on execution)

| Field | Value |
|-------|--------|
| Git commit (`git rev-parse --short HEAD`) | |
| Build | `tauri dev` ☐ / release build ☐ |
| OS + version | |
| Engraver software + version | **XTool Studio** (version: _____) · **xTool F2 Ultra UV** |
| Stub depth (`SP3D_USE_STUB=1`) | Yes ☐ / No ☐ |
| Test image (path + size) | e.g. `tests/fixtures/valid_small.png` |
| Blank used | 50×50×80 mm, margin 2 mm ☐ (other: _____) |

### Prerequisites (summary)

- Toolchain per `SETUP_NOTES.md`; `npm run tauri dev` runs.
- Fixtures: `node scripts/gen_fixtures.mjs` if PNG fixtures missing.
- Optional stub depth: `export SP3D_USE_STUB=1` before `npm run tauri dev` (no Hugging Face model).
- Default blank: **50 × 50 × 80 mm**, margin **2 mm** (Standard preset).

**Full steps:** [QA-D-01_TEST_PROCEDURE.md](./QA-D-01_TEST_PROCEDURE.md) § B–C (app), § D (engraver).

### Execution summary

| Phase | Scope | Result | Notes |
|-------|--------|--------|-------|
| App export (C1–C8) | Load → depth → blank → PLY ASCII + XYZ | Not run ☐ / Pass ☐ / Fail ☐ | |
| Engraver import (D1–D7) | PLY into XTool Studio (Point Cloud Engraving) | Not run ☐ / Pass ☐ / Fail ☐ | XYZ: secondary if Studio supports |

### Format results

| Format | Engraver import | Pre-check (CloudCompare/MeshLab) | Point count N | Notes |
|--------|-----------------|----------------------------------|---------------|-------|
| PLY ASCII | Pass / Fail / Blocked | Pass / Fail / N/A | | |
| XYZ | Pass / Fail / Blocked | Pass / Fail / N/A | | |

**PLY/XYZ counts match:** ☐ Yes ☐ No — PLY N= _____ XYZ lines= _____

### Charter verification (docs/engraver-charter.md §5)

| ID | Check | Pass? | Notes |
|----|--------|-------|-------|
| V1 | File opens | ☐ | |
| V2 | Units mm | ☐ | |
| V3 | Axis orientation | ☐ | |
| V4 | Point count reasonable | ☐ | |
| V5 | BBox inside blank | ☐ | |
| V6 | PLY metadata comments | ☐ | |
| V7 | Depth / engraving preview | ☐ | |

**Axis orientation correct:** ☐ Yes ☐ No ☐ N/A (blocked)  
**Units (mm) correct:** ☐ Yes ☐ No ☐ N/A (blocked)

### QA-D-01 overall rubric

| Outcome | Select when |
|---------|-------------|
| ☐ **Pass** | Engraver import Pass; V1–V7 Pass |
| ☐ **Fail** | Wrong units/axes, import error, or bbox violation |
| ☐ **Blocked** | XTool Studio or F2 Ultra UV unavailable |
| ☐ **Partial** | App export Pass; engraver steps not run |

### Defect log (BACK-D-01 handoff)

| ID | Sev | Format | Symptom | Expected | Actual | Evidence | Status |
|----|-----|--------|---------|----------|--------|----------|--------|
| | P0/P1/P2/P3 | PLY/XYZ/Both | | Charter §2–4 | | path/screenshot | Open |

_Add rows from [QA-D-01_TEST_PROCEDURE.md](./QA-D-01_TEST_PROCEDURE.md) § G during execution._

---

## QA-D-02 — Performance (4K image)

**Automated Rust slice (dev reference, 2026-05-24):** `cargo test qa_d02_4k_rust_pipeline_timing --manifest-path src-tauri/Cargo.toml -- --ignored --nocapture` — synthetic 3840×2160 depth, step 1: ~0.6 s generate, ~3.0 s PLY export (~233 MB file). See `QA-D-02_BENCHMARK.md`.

| Step | Time | Notes |
|------|------|-------|
| Load image | | |
| Generate depth | | |
| Generate point cloud | | |
| Export PLY | | |
| **Total** | | Target: &lt; 2 min (PRD) E2E including Python |

---

## QA-D-03 — Security pentest checklist

**Automated (2026-05-24):** `cargo test qa_d03 --manifest-path src-tauri/Cargo.toml` — see `QA-D-03_AUTOMATED_COVERAGE.md` for mapping to checklist #1–18.

| Section | Pass | Fail | Skipped | Notes |
|---------|------|------|---------|-------|
| IPC fuzzing | Auto | | Manual UI | Tests #1–7 |
| Filesystem | Auto | | #9 symlink | Tests #8, #10 |
| Model & subprocess | | | Manual | #11–13 |
| WebView / ACL | | | Manual | #14–16 release build |
| DoS (lab) | Auto | | #18 E2E | #17–18 partial |

### Security pentest (QA-D-03 — static review)

**Reviewer:** Security Specialist + Quality Engineer (Wave 6)  
**Method:** Code review of `path_security.rs`, `image_loading.rs`, `lib.rs` (export/command tests), `export.rs` — not a live built-app run.  
**Commit reviewed:** _uncommitted working tree (2026-05-20)_

| Test # | Static coverage | Manual still required? |
|--------|-----------------|------------------------|
| 1 | **Yes** — `load_image_rejects_empty_path`, `export_commands_reject_empty_path` (`lib.rs`); empty import path rejected in `validate_import_file_path` (`path_security.rs`). | **No** for empty-path behaviour; optional IPC `invoke()` harness smoke. |
| 2 | **Yes** — `load_image_impl_invalid_path_returns_error`, `generate_depth_map_rejects_nonexistent_path` (`image_loading.rs`, `lib.rs`). | **No** for error-without-panic; optional live hang check. |
| 3 | **Partial** — `load_image_impl_path_traversal_resolved_by_canonicalize` proves `..` resolves to real file under temp; `load_image` uses canonicalize + blocklist (`image_loading.rs`). | **Yes** — malicious traversal *outside* allowed tree and cross-volume edge cases need built-app or harness. |
| 4 | **Partial** — `set_mask` rejects `width`/`height` ≠ depth map (`lib.rs`); no dedicated unit test calling `set_mask` with wrong `data.len()`. | **Yes** — confirm IPC rejects length ≠ W×H without panic. |
| 5 | **Partial** — SEC-D-04 caps generation at 8M points (`lib.rs`); depth capped at 8192²; no explicit max `Vec<bool>` size test in scoped files. | **Yes** — huge mask at max depth dimensions via UI/devtools. |
| 6 | **Yes** — `parse_and_validate_json_invalid_json_returns_error` and related cases (`preset.rs`); used by `load_preset` absolute-path branch. | **Partial** — corrupt file on disk via `load_preset` IPC still worth one manual pass. |
| 7 | **Partial** — `validate_import_file_path` blocks non-`.json`, empty path, and system dirs (`path_security.rs` tests use temp `.json`); `load_preset` calls it for absolute paths. | **Yes** — `/etc/passwd` or sensitive absolute path on target OS. |
| 8 | **Partial** — `validate_export_path` + `is_blocklisted` reject system directories (`lib.rs`); **no** unit test asserting `/etc` or `C:\Windows` rejection in scoped files. | **Yes** — export to system dir on Linux/Windows release build. |
| 9 | **No** — SEC-D-09 deferred; writability probe only, no symlink/TOCTOU test. | **Yes** — export with symlinked parent directory. |
| 10 | **Partial** — `load_image` path validation uses `path_security::is_blocklisted` (includes `/usr`, `/bin` on Unix); no unit test loading `/usr/bin/...` in scoped files. | **Yes** — `load_image` on blocklisted path per OS. |
| 11 | **No** (in scoped Rust tests) — stub manifest in `python/python/expected_model_hashes.json`; verification logic in Python `model_downloader.py` + pytest, not in listed Rust modules. | **Yes** — fresh `download_model` after trusted hash population. |
| 12 | **No** (in scoped Rust tests) — tamper handling in Python `verify_model_sha256` tests only. | **Yes** — alter model byte in app data dir and run depth. |
| 13 | **No** — SEC-D-07 deferred; subprocess inherits env today. | **Yes** — lab launch with `VIRTUAL_ENV` hijack; document behaviour. |
| 14 | **No** — ACL files exist (`allow-settings.toml`); no static test of Tauri `invoke` permission allow/deny. | **Yes** — `get_settings` / `save_settings` from UI after SEC-D-06. |
| 15 | **No** — same for `allow-model.toml`. | **Yes** — `check_model` / `download_model` from first-run wizard. |
| 16 | **No** — SEC-D-11 deferred (`withGlobalTauri` / devtools). | **Yes** — release build `window.__TAURI__` inspection. |
| 17 | **Yes** — `enforce_max_file_size_rejects_over_limit`, `read_image_bytes_for_depth_rejects_oversize_file_via_metadata` (`image_loading.rs`). | **Partial** — valid-header file &gt;128 MiB through full `load_image` / UI path. |
| 18 | **Partial** — `load_image_impl_downsamples_over_8k`, dimension caps on `read_image_bytes_for_depth`; not full depth→point-cloud→export E2E in unit tests. | **Yes** — 8192×8192 workflow within memory budget on built app. |

**Static review summary:** 6/18 fully covered by scoped Rust unit tests; 8 partial (logic present, gap in test or IPC layer); 4 require manual/release only. Aligns with **Pass with waivers** in `SECURITY_SIGNOFF.md` pending QA-D-03 manual execution.

---

## Overall

**Sprint D QA sign-off:** ☐ Pass ☐ Fail — **Date:** ___________

**QA-D-01 next step:** Run § C (app export) then § D (XTool Studio import) per [QA-D-01_TEST_PROCEDURE.md](./QA-D-01_TEST_PROCEDURE.md). Record Studio version and any axis/unit transforms in the defect log.
