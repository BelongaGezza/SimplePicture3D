# Sprint 1.4 — Gotchas

Sprint-specific gotchas; merge notable items to `RESEARCH/GOTCHAS.md` when done.

---

## Depth map generation & UI

*(Agents add findings here: command API, progress events, large payloads, canvas performance, etc.)*

### Depth map preview: zoom out / fit full image (Case 4 feedback)

- **Feedback:** Depth map preview does not allow easy appreciation of the full image; zoom-out or "fit to view" would help. Current implementation (JR1-302) has zoom 0.1×–10× and pan, but initial view is 1× so a large depth map in the narrow right sidebar shows only a portion. **Suggestion for future sprint:** On load, set initial zoom to "fit to container" (or min zoom) so the full depth map is visible; or add a "Fit" / "Zoom to fit" control so users can view the whole image.

### Case 2b (manual test): Testing "Python not available"

- **Flawed approach:** Clearing PATH entirely (`$env:Path = ''`) to hide Python also hides Node/npm, so `npm run tauri dev` fails and the app never starts.
- **Correct approach:** Either (A) temporarily rename `python.exe` (e.g. in venv\Scripts or system Python dir) to `python.exe.bak`, then start the app from a normal terminal — app runs, Generate fails with "failed to spawn Python"; or (B) set PATH to exclude only Python/venv: `$env:Path = ($env:Path -split ';' | Where-Object { $_ -notmatch 'Python' -and $_ -notmatch '\\venv\\' }) -join ';'` so Node stays on PATH and npm works, then run the app in that terminal. See MANUAL_TEST_WALKTHROUGH_WINDOWS.md.

---

## Performance & format

*(e.g. inference time by size, IPC size limits, depth array size.)*

### JR1-303: Depth map canvas rendering performance

- **Implementation:** `src/lib/depthCanvas.ts` + `DepthMapPreview.svelte`. Single synchronous pass: depth 0–1 → ImageData grayscale → `putImageData`. No Web Worker; runs on main thread.
- **Expected timings (typical desktop):** ~2M pixels (1920×1080) ≈ 20–80 ms; ~8.3M pixels (3840×2160) ≈ 80–300 ms. No multi-second block.
- **Recommendation:** Smooth canvas update for depth up to 4K is acceptable. If preview feels sluggish for very large maps, consider downsampling for display only (keep full resolution for mesh pipeline) or future OffscreenCanvas/chunked render.
- **Tested:** Manual verification with synthetic depth arrays at 640×480, 1920×1080; no jank. 4K to be verified when backend returns real 4K depth (QA-303).

### JR2-302: All-black / all-white images

- **Expected:** No panic or unhandled error. Depth map may be constant or near-constant (model-dependent). Stub returns 1×1; real model may return uniform or slightly varying values.
- **Tests:** `depth_estimation_all_black_image`, `depth_estimation_all_white_image` in `src-tauri/src/lib.rs` (run with `--ignored` when Python env available).

---

**Document Version:** 1.0
