# ARCH-501: IPC Performance Evaluation Spike

**Sprint:** 1.5A  
**Role:** System Architect  
**Date:** 2026-02-07  
**Source:** Consultant Report §3.3, RESEARCH/GOTCHAS.md "IPC large payloads slow"

---

## Objective

Evaluate IPC round-trip latency for `get_depth_map` at three resolutions before Sprint 1.6/1.7 mesh generation (which will introduce larger data transfers). If latency exceeds 100 ms at 1080p, recommend an alternative approach.

---

## Methodology

### 1. Payload sizes (depth array only, normalized [0,1] as f32)

| Resolution   | Pixels    | Approx. JSON size |
|-------------|-----------|---------------------|
| 640×480     | ~1.2M     | ~5 MB               |
| 1920×1080   | ~2.1M     | ~8 MB               |
| 3840×2160   | ~8.3M     | ~33 MB              |

Tauri IPC uses JSON-RPC serialization for command return values. Full round-trip = Rust (apply_adjustments + serialize) → IPC → frontend (deserialize + use).

### 2. Rust-side serialization benchmark

A benchmark was added to measure JSON serialization time only (lower bound for IPC cost):

- **Location:** `src-tauri/benches/ipc_depth_map_serialization.rs`
- **Run:** From project root: `cargo bench --manifest-path src-tauri/Cargo.toml --bench ipc_depth_map_serialization`

The benchmark times `serde_json::to_string(&DepthMapPayload)` for the three resolutions. This does not include Tauri’s IPC channel or frontend deserialization.

### 3. Frontend-side round-trip timing

To measure full round-trip from the UI:

1. Run the app in dev: `npm run tauri dev`
2. Load an image at the desired resolution and generate a depth map
3. Trigger a preview refresh (e.g. move a depth slider) so `getDepthMap()` is called
4. In the browser devtools console, observe the `getDepthMap` timing (see below)

**Instrumentation added:** In `src/App.svelte`, when `import.meta.env.DEV` is true, `getDepthMap()` calls are wrapped with `console.time('getDepthMap')` and `console.timeEnd('getDepthMap')`. Dimensions are logged when available so you can correlate with resolution.

### 4. Existing evidence (GOTCHAS)

From RESEARCH/GOTCHAS.md (2026-02-01):  
*"Sending large depth maps or mesh data (e.g. 3MB+) over Tauri IPC can be slow (e.g. 200ms+ for 3MB)."*

So 1080p (~8 MB) can reasonably be expected to exceed 100 ms full round-trip.

---

## Results

### Rust serialization benchmark

Run and paste here (from `cargo bench --manifest-path src-tauri/Cargo.toml --bench ipc_depth_map_serialization`):

```
(Example format)
serialize_depth_map_640x480    ... time:   [XX ms]
serialize_depth_map_1920x1080  ... time:   [XX ms]
serialize_depth_map_3840x2160  ... time:   [XX ms]
```

### Full IPC round-trip (manual)

| Resolution   | getDepthMap() time (ms) | Notes |
|-------------|--------------------------|-------|
| 640×480     | (measure in dev)         |       |
| 1920×1080   | (measure in dev)         |       |
| 3840×2160   | (measure in dev)         |       |

---

## Evaluation (if latency >100 ms at 1080p)

Alternatives to consider for Sprint 1.6/1.7:

1. **Binary transfer via temp file**  
   Rust writes depth to a temp file (e.g. raw f32 or compact binary), returns path; frontend reads via a Tauri command that returns bytes or a blob. No asset protocol required (SEC-501 disabled it); use a dedicated command e.g. `read_depth_map_file(path)`.

2. **Client-side depth adjustment**  
   Send only raw depth once (or via file); apply brightness/gamma/contrast/invert and range in the frontend. Reduces repeated large IPC for slider preview; backend still stores original and params for export.

3. **Chunked transfer**  
   Split depth into tiles or rows; multiple invoke calls. More complex and may not reduce total time much; only consider if single-message limits are hit.

4. **Shared memory / memory-mapped file**  
   Platform-specific; higher implementation cost; defer unless profiling shows IPC as the dominant cost.

---

## Recommendation for Sprint 1.6/1.7

- **Current behaviour:** Keep `get_depth_map` as-is for Sprint 1.5A (no change to contract). The added instrumentation is dev-only and documents methodology.
- **Sprint 1.6/1.7:**  
  - **If** full round-trip at 1080p is observed or measured above ~100 ms:  
    - Prefer **binary transfer via temp file**: new command(s) to write adjusted depth to a temp file and either return a path for a read command or return chunked/base64 binary from Rust.  
    - Optionally move **depth adjustment preview to the client** (apply in JS from raw depth + params) to avoid repeated large IPC on slider change.  
  - **If** latency is acceptable (<100 ms at 1080p):  
    - Document baseline in this spike and in GOTCHAS; re-evaluate when mesh/vertex payloads are added.

**Note:** With the asset protocol disabled (SEC-501), any temp-file approach must use a Tauri command to read binary data (e.g. `read_temp_depth_file`) rather than `asset://` URLs.

---

## Artefacts

| Item | Path |
|------|------|
| Serialization bench | `src-tauri/benches/ipc_depth_map_serialization.rs` |
| Frontend timing (dev) | `src/App.svelte` — `console.time/timeEnd` around `getDepthMap()` |
| This spike | `SPRINTS/Sprint_1_5A/IPC_PERFORMANCE_SPIKE.md` |
| GOTCHAS cross-ref | `SPRINTS/Sprint_1_5A/GOTCHAS.md` |
