# QA-D-02 — 4K performance benchmark

**Goal:** Measure time from loaded depth to exported PLY against PRD target (**&lt; 2 minutes** end-to-end on mid-range hardware).

## What is measured

| Scope | Includes | Excludes |
|-------|----------|----------|
| **Rust-only (automated)** | Surface-map generation + PLY write | Image load, Python depth estimation |
| **Manual (MANUAL_TEST_REPORT)** | Load → depth → point cloud → export PLY | Engraver import |

PRD timing is **end-to-end**; Rust-only numbers are a lower bound for the geometry/export slice.

## Run Rust benchmark (Criterion)

From repository root:

```bash
cargo bench --manifest-path src-tauri/Cargo.toml --bench point_cloud_pipeline
```

Benchmarks:

- `surface_map_1080p_step1` — 1920×1080, step 1
- `surface_map_4k_step2` — 3840×2160, step 2 (faster smoke)
- `surface_map_4k_step1` — 3840×2160, full resolution (~8M points)
- `qa_d02_4k_rust_pipeline/generate_and_export_ply` — generate + atomic PLY export

## Run ignored integration timing test

Prints human-readable breakdown to stdout:

```bash
cargo test qa_d02_4k_rust_pipeline_timing \
  --manifest-path src-tauri/Cargo.toml \
  -- --ignored --nocapture
```

Record output in `MANUAL_TEST_REPORT.md` § QA-D-02.

## Manual E2E (required for sprint sign-off)

1. Use a **3840×2160** (or 4096×2160) test image (see `tests/fixtures/README.md`).
2. In the app: load → generate depth (stub or model) → generate point cloud → export PLY.
3. Note wall-clock times per step in `MANUAL_TEST_REPORT.md`.
4. **Pass:** total &lt; 120 s on target hardware, or document hardware + mitigation.

## Hardware to record

- OS, CPU model, RAM
- Stub vs real Depth-Anything-V2 model
- `step_x` / `step_y` in Volumetric params
