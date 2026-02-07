# Sprint 1.6 Gotchas

**Sprint:** 1.6 — Mesh Generation Algorithm  
**Purpose:** Sprint-specific debugging findings; merge useful items to `RESEARCH/GOTCHAS.md` at sprint end.

---

## Entries

### 2026-02-07 — Security (SEC-301/302) — Mesh input validation and overflow
**Symptom:** Need to ensure mesh generation cannot panic or overflow on malicious or malformed depth input.  
**Cause:** PRD §8 and sprint security tasks SEC-301 (integer overflow), SEC-302 (input validation).  
**Fix:** `validate_depth_input()` in `mesh_generator.rs` enforces dimensions >0, ≤MAX_DIMENSION (8192), and depth.len() == width×height via `checked_mul`. Vertex count uses `checked_mul`. See `SPRINTS/Sprint_1_6/SECURITY_SIGNOFF.md`.

---

## JR2-503: Mesh generation benchmark results

**Command:** `cargo bench --manifest-path src-tauri/Cargo.toml --bench mesh_generation`

| Resolution   | Approx. time | Target (PRD §7.1) |
|-------------|--------------|--------------------|
| 1024×1024   | ~9.3 ms      | —                  |
| 3840×2160 (4K) | ~73 ms    | <15 s ✅           |

*Note: Run on your hardware to reproduce; 4K is well under the 15 s target. For 8K (7680×4320), run the same bench with dimensions updated if needed.*

---

## JR2-504: Memory profiling (4K mesh generation)

**Objective:** Confirm peak memory <2GB for 4K depth map mesh generation (PRD §5.5, §7.1).

**How to run:**

- **Windows:** Use Task Manager (Memory column) or [Visual Studio Profiler](https://learn.microsoft.com/en-us/visualstudio/profiling/) / [Windows Performance Recorder](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/) for heap.
- **Linux:** `valgrind --tool=massif target/release/bench_name` or run a small binary that generates 4K mesh and inspect RSS.
- **macOS:** Instruments (Allocations) attached to the benchmark or a test binary.

**Steps:**

1. Build release: `cargo build --release --manifest-path src-tauri/Cargo.toml`
2. Run mesh generation under the profiler (e.g. run `mesh_generation` bench or a test that builds 3840×2160 mesh).
3. Record peak heap (or process RSS) and compare to 2GB.

**Result (to be filled when run):**

| Metric        | Value | Budget |
|---------------|-------|--------|
| Peak memory (4K) | _TBD_ | <2 GB |

*Document actual tool and result here or in PROGRESS_REPORT when completed.*
