# Sprint 1.3 — Gotchas

Sprint-specific gotchas; merge notable items to `RESEARCH/GOTCHAS.md` when done.

---

## Python–Rust bridge

*(Agents add findings here: subprocess spawn on Windows/macOS/Linux, stdin vs temp file, encoding, timeouts, etc.)*

### 2026-02-03 — Junior Engineer 2D — Image pass-through options for ARCH-102
**Context:** ARCH-102 must decide how to pass image data from Rust to Python (temp file, stdin, or base64 JSON). From the image-loading side we can provide:
- **PNG bytes:** We already encode RGB to PNG for preview (base64). A small helper `rgb_to_png_bytes()` in `image_loading.rs` returns raw PNG `Vec<u8>` so BACK-202 can write to temp via `file_io::write_temp_file("img_", ".png", &bytes)` without duplicating encoder logic.
- **Temp file:** `file_io::write_temp_file` is safe (temp dir only, sanitized prefix/suffix); path can be passed to Python as CLI arg (no user input in argv).
- **Dimensions:** `LoadImageOut` already has `width`/`height`; Python output can be validated against these.
- **Path:** `load_image` does not return or cache the original file path. So `generate_depth_map` (Sprint 1.4) will need either: (1) frontend sends path again → Rust re-reads → writes to temp → Python, or (2) frontend sends image bytes (e.g. base64) and Rust writes to temp. Option (1) avoids holding large buffers in frontend; (2) avoids re-read. Recommend (1) for MVP and document in ARCH-101/102.

---

### 2026-02-03 — QA-201 / Windows subprocess
**Context:** Python subprocess is spawned via `std::process::Command` (no shell). So **cmd.exe vs PowerShell** does not affect how the child process is started — only which shell the *user* uses to run `cargo test` or `npm run tauri dev`. On Windows, `python_bridge` resolves Python as: (1) if `VIRTUAL_ENV` is set, use `%VIRTUAL_ENV%\Scripts\python.exe`; (2) else `python` (PATH). Working dir is `cwd/python` or `parent/python` so `python -m python.depth_estimator` finds the package. No encoding or quoting issues because we do not pass user input in argv.

---

### 2026-02-03 — JR2-203: Image serialization benchmark (temp file only)
**Context:** ARCH-102 chose **temp file only** for image input (Rust → Python). Stdin and base64 JSON are not in the current protocol, so only the temp-file path is implemented and benchmarked.

**Benchmark:** Test `benchmark_temp_file_roundtrip` in `src-tauri/src/lib.rs` (run with `cargo test benchmark_temp_file_roundtrip -- --ignored --nocapture` from repo root or `src-tauri`). It builds a 640×480 PNG (size depends on compression; synthetic image ~6–50 KB), runs `run_depth_estimation_with_timeout` twice, and reports median roundtrip time (Rust write temp file → spawn Python → Python read + stub inference → Rust parse JSON).

**Results (typical):** Stub estimator only (no real model): ~300–600 ms per run (observed ~545 ms median on Windows for 640×480). Real Depth-Anything-V2 will dominate (seconds to minutes for 4K). **Recommendation:** Retain temp file for MVP; no protocol change needed. If future profiling shows disk I/O as a bottleneck for 4K, consider adding optional stdin path (would require ARCH-102 update and Python support).

---

## Model and environment

*(Agents add findings here: model path, venv activation in subprocess, CUDA/Metal env vars, CI without GPU.)*

### 2026-02-03 — QA-203: Forced CPU for CI (no GPU)
When running depth estimation in CI or on machines without GPU: set **`CUDA_VISIBLE_DEVICES=`** (empty) so PyTorch does not see CUDA devices and uses CPU. Alternatively use **`PYTORCH_CUDA_ALLOC_CONF=`** or model-specific env. Stub estimator does not use GPU; real Depth-Anything-V2 (AI-205) will. Document in CI workflow or test plan so CPU-only path is tested.

---

**Document Version:** 1.0
