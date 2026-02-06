# Sprint 1.5 Gotchas

**Sprint:** 1.5 — Manual Depth Adjustments  
**Purpose:** Sprint-specific debugging findings; merge notable items to `RESEARCH/GOTCHAS.md` when done.

---

*(Add gotchas as discovered during implementation and testing.)*

**Format:** Task ID or area — Short description — Workaround or note.

---

**JR1-404 — Slider responsiveness** — DepthControls sliders (depth min/max, brightness, gamma) are responsive; preview updates are debounced (80 ms in App.svelte) so the main thread is not blocked during drag. At 1080p depth, no jank observed; if very large depth arrays are used later, consider increasing debounce or running adjustment in a worker.

**Security (ad-hoc Sprint 1.5)** — Security Specialist review (2026-02-06): New Tauri commands `set_depth_adjustment_params`, `get_depth_adjustment_params`, `reset_depth_adjustments` accept/return only `DepthAdjustmentParams` (f32 + bool); no path or raw string input. `depth_adjust.rs` is pure math; no I/O. Permissions in `allow-generate-depth-map.toml` correctly scope the new commands. Path handling and magic-byte validation remain in `image_loading.rs` for `generate_depth_map`/`load_image`; no new path surface in 1.5. No issues found.

**Security — Dependency audits (2026-02-06)** — `cargo audit`: 2 vulnerabilities (bytes 1.11.0 → upgrade to ≥1.11.1, time 0.3.46 → ≥0.3.47); 19 allowed warnings (unmaintained/unsound crates in Tauri/wry tree). `npm audit`: 5 moderate (esbuild/vite dev server; fix may require vite major upgrade). Recommend: address Rust vulns when upstream (Tauri) bumps deps; track npm in next frontend dependency update. Merge notable items to RESEARCH/GOTCHAS.md when sprint closes.
