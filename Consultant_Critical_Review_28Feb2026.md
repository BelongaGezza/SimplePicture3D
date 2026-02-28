# SimplePicture3D — External Consultant Critical Review

**Prepared for:** System Architect
**Date:** 2026-02-28
**Scope:** Architecture consistency, implementation fidelity, Phase 2 readiness
**Status at review:** Phase 1 complete (Sprint 1.12 exit gate: GO), Sprint 2.1 delivered

---

## Executive Summary

The project has delivered a functional Phase 1 MVP on a substantially expanded schedule (12+1 sprints vs planned 8-10). The Rust backend, depth pipeline, STL/OBJ export, and Tauri shell are solid and well-structured. However, **six issues warrant architect attention before Sprint 2.2 begins** — two of which directly affect end-user experience and one is a security verification gap. A further seven lower-priority items constitute technical debt that will compound in Phase 2.

---

## 1. Phase 1 Retrospective: Schedule Fidelity

**Finding:** Phase 1 ran to 13 delivery events (Sprints 1.1–1.12 + 1.5A hardening) against a planned 8-10. The original `todo.md` estimate was never updated after the consultant's revised forecast of 28-35 total sprints.

**Current state of planning artefacts:**
- `todo.md` version 1.2 — last updated **February 7, 2026** (three weeks behind)
- `prd.md` status header reads "60–65% complete" — accurate on Feb 7 but stale; Phase 1 is now complete
- Sprint velocity is knowable: ~1 sprint per 2 weeks actual vs planned

**Recommendation:** Before Sprint 2.2, update `todo.md` with:
1. Phase 1 as-built sprint count (13 events)
2. Phase 2 remaining feature list re-prioritised in light of Sprint 2.1 delivery
3. Revised overall estimate using actual velocity

---

## 2. Critical Issues (Act Before Sprint 2.2)

### 2.1 ADR-008 Winding Order — Architecture Doc Contradicts Implementation

**Severity:** High (misleads future contributors to the mesh pipeline)

`RESEARCH/architecture.md` ADR-008 documents Triangle 1 winding as `v_tl → v_bl → v_tr`. `RESEARCH/GOTCHAS.md` (2026-02-08) records that this produces **-Z normals**, was fixed to `tl→tr→bl`, and explicitly states:

> "ADR-008 in architecture.md should be updated to match the implemented winding."

That update was **not made**. The ADR currently documents wrong behaviour. The git status (`M RESEARCH/architecture.md`) suggests this file is currently modified — the fix may be in progress, but it must be confirmed and committed.

**Action:** Verify the uncommitted change corrects the winding order table. Commit and close the GOTCHAS note.

---

### 2.2 Wireframe/Solid Mode — Feature Regression Post Sprint 1.8

**Severity:** High (broken UI, false promise to users)

`Preview3D.svelte` line 35 still carries the comment `wireframe/solid placeholder until Sprint 1.8`. The overlay message users see when clicking Wireframe or Solid is:

> "Wireframe/Solid mode requires a triangulated mesh (Sprint 1.8). Use Points for now."

Sprint 1.8 is **complete**. The backend produces `MeshData` with an `indices: Option<Vec<u32>>` field. However, `get_mesh_data` is never invoked with a request for indexed geometry — the frontend only ever builds a `THREE.Points` cloud and never constructs a `THREE.Mesh` with `BufferGeometry` using the index buffer.

**Result:** The Wireframe and Solid buttons are permanently broken. Users who click them see an error message referencing an internal sprint number — a credibility issue in a beta release.

**Action:** This is a Sprint 2.x task. Wire `MeshData.indices` to a `THREE.Mesh` with `THREE.WireframeGeometry` for wireframe and a `THREE.MeshPhongMaterial` for solid. Until that's done, **remove the Wireframe and Solid buttons from the UI** to avoid user confusion.

---

### 2.3 Depth Estimation Progress — Silent Blocking Call

**Severity:** High (UX — 5-minute black-box wait for 4K images)

`generate_depth_map` in `lib.rs` is a **synchronous blocking Tauri command**. It calls `run_depth_estimation` which spawns the Python subprocess, waits for it to complete, then reads all stdout. The Python process emits `PROGRESS 25`, `STAGE inference` etc. on stderr, but these are only parsed **after** the subprocess exits.

The `GenerateDepthMapResponse.progress` field is hardcoded to `100` — it is never set to anything else.

**Result:** For a 4K image on CPU, the user waits up to 5 minutes with a spinner and zero progress feedback. The PRD success metric requires "User can complete first conversion within 5 minutes" — this will feel much longer subjectively.

**Architecture gap:** The progress protocol is designed (ADR-002, BACK-205) but the plumbing from subprocess stderr → Tauri event → frontend is missing. The `tauri-plugin-shell` sidecar approach would enable streaming events; the current `std::process::Command` approach in `python_bridge.rs` does not.

**Action:** Schedule as an early Phase 2 task. Minimum viable fix: convert the Python stderr reader to a background thread and emit Tauri events (`tauri::Emitter`) on each `PROGRESS` line. The frontend already has a `depthEstimating` flag but no percentage display.

---

### 2.4 Export: Synchronous Full-Resolution on Main Thread

**Severity:** Medium-High (UI freeze for large exports)

Both `export_stl` and `export_obj` in `lib.rs` use hardcoded `step_x: 1, step_y: 1` (full resolution). For a 4K image (3840×2160 = ~8.3M vertices), the export regenerates the entire mesh from scratch, validates it, and writes to disk — all **synchronously** on the Tauri command handler thread, blocking the UI with no feedback.

The `mesh_generation` benchmark covers generation time, but full STL write at 4K resolution was not benchmarked.

**Action:** Wrap export in `tokio::task::spawn_blocking` or an async Tauri command. Add a progress indicator in the UI during export. This is Phase 2 scope but should be architected now before the pattern spreads.

---

### 2.5 Model Download SHA256 Verification — Unverified Implementation

**Severity:** Medium (security gap for a download-and-execute pattern)

`RESEARCH/architecture.md` ADR-003 states: _"Security (SEC-202): When implementing download, use HTTPS only and verify SHA256 checksum against a trusted source."_ Sprint 1.10 delivered the first-run wizard and the Phase 1 security sign-off is documented. However, the architecture documentation does not record **where SHA256 is verified**, what the trusted checksum source is, or whether it was tested.

Given that models are ~2GB files executed via Python, a compromised or corrupted download is a material risk.

**Action:** The System Architect should verify with the Senior Engineer that `download_model` implementation includes checksum verification. If not, treat as a Phase 2 security task (not optional).

---

### 2.6 Undo/Redo Absent When Curves Tool Is Delivered

**Severity:** Medium (user trust gap)

Sprint 2.1 delivered the curves tool (CurvesTool.svelte) with control point editing. However:
- `AppSettings` persists brightness/contrast/gamma/invert/depthMin/depthMax but **not** `curveControlPoints` (noted as deferred in Sprint 2.1 progress report)
- F2.4 (Undo/Redo, "Undo last 20 actions") is not scheduled — no sprint assigned

**Result:** A user who carefully adjusts a curve, then accidentally clicks Reset or loads a new image, loses their work with no recovery. This is a "power tool without a safety net" pattern.

**Action:** Schedule F2.4 (Undo/Redo) as **Sprint 2.2 scope**, ahead of masking (F2.1 remaining) and presets (F2.3). The architectural pattern (command stack, state snapshots) should be decided before further state-mutating features are added. Also: add `curveControlPoints` to `AppSettings` — a one-field change with immediate value.

---

## 3. Architecture Consistency Issues

### 3.1 `prd.md` F1.6 References Deleted Crate

`prd.md` §F1.6 Technical Notes still says: _"Use `stl_io` crate for STL writing."_ `Cargo.toml` contains no `stl_io` dependency. The project uses custom writers in `mesh_generator.rs` per ADR-008. The PRD is a requirements artefact consulted by new contributors — this inconsistency will cause confusion.

**Action:** Update `prd.md` §F1.6 to: "Custom binary STL and ASCII OBJ writers in `mesh_generator.rs` (ADR-008). No external export crate."

### 3.2 Architecture Diagram — Two Stale Labels

The architecture overview diagram (present in `CLAUDE.md`, `RESEARCH/architecture.md`, and `prd.md`) shows `Svelte/React` — but ADR-001 settled on **Svelte 4** in February. It also shows the Python bridge as `subprocess (stdin/temp file)` — but the bridge uses `--input <path>` (temp file input) and stdout; stdin is not used.

**Action:** Update all instances of the diagram with `Svelte 4` and `subprocess (temp file → stdout)`.

### 3.3 `python_bridge.rs` Stale Module Comment

The module-level `#[allow(dead_code)]` annotation says: _"Used by integration test (roundtrip) and future generate_depth_map command (Sprint 1.4)."_ Sprint 1.4 is complete; the module is fully integrated. The attribute is now misleading. Some public functions (`log_progress_from_stderr`) may genuinely be unused — if dead, remove them; if needed, document why they are retained.

---

## 4. Technical Debt Register

| ID | Area | Issue | Recommended Sprint |
|----|------|--------|-------------------|
| TD-01 | Frontend | App.svelte owns all state via prop drilling. With Phase 2 adding undo/redo, presets, masking — no state management ADR exists. Svelte stores should be designed before complexity grows. | Sprint 2.2 (ADR only, not full rewrite) |
| TD-02 | Frontend | Wireframe/Solid buttons are dead UI — remove or fix. | Sprint 2.2 |
| TD-03 | Docs | `docs/architecture.md` (user-facing) and `RESEARCH/architecture.md` (canonical) need a clear policy: are they the same file, or separately maintained? Currently ambiguous. | Immediate doc debt |
| TD-04 | Python | `python/setup.py` listed in repo structure is a legacy packaging approach; modern Python uses `pyproject.toml`. Not urgent but matters for Phase 4 installer packaging. | Phase 4 pre-work |
| TD-05 | Cross-platform | Phase 1 exit gate was Windows-only. Linux CI exists and caught path separator bugs. No macOS or Linux manual testing documented. Phase 3 cross-platform work will be harder if gaps accumulate from Phase 2 onward. | Start macOS smoke tests in Sprint 2.2 |
| TD-06 | Security | Export path validation uses a TOCTOU write-test pattern (`create` + `remove` + actual write). Low exploitability but architecturally impure. | Phase 3 |
| TD-07 | Python distribution | ADR-003 System Python is developer-comfortable but user-hostile for the primary persona ("Sarah the Crystal Crafter"). ONNX migration (deferred to v1.1) should be elevated to Phase 4 scope, not post-Phase 4, to meet the "within 5 minutes" success metric. | Phase 4 scope decision |

---

## 5. Phase 2 Sequencing Recommendation

The PRD lists Phase 2 features without ordering. Based on what Sprint 2.1 has built, the recommended sequencing avoids rework and UX debt:

```
Sprint 2.2: Undo/Redo (F2.4) + Curve persistence + State management ADR + Wireframe/Solid fix
Sprint 2.3: Presets & Templates (F2.3) — needs undo/redo foundation
Sprint 2.4: Progress streaming for depth estimation (closes the 5-min UX gap)
Sprint 2.5: Masking & Regional Adjustments (F2.1 remainder)
Sprint 2.6: Enhanced 3D Preview (F2.5) — includes wireframe/solid proper, PBR lighting
Sprint 2.7: Material Presets (F2.6)
Sprint 2.8: Async export + progress indicators
```

**Key constraint:** Undo/Redo (Sprint 2.2) is a **prerequisite** for any further state-mutation features (masking, brushes, layer blending). Building those features without undo/redo will either require retrofitting or will ship a product that frustrates artists.

---

## 6. Overall Assessment

| Dimension | Rating | Comment |
|-----------|--------|---------|
| Backend (Rust) | Strong | Well-structured, tested, benchmarked |
| Python bridge | Adequate | Functional but progress streaming missing |
| Frontend architecture | Adequate | Functional but not designed for Phase 2 state complexity |
| STL/OBJ export | Good | Correct and secure; blocking behaviour is the main gap |
| Architecture documentation | Needs attention | Three confirmed stale sections; ADR-008 winding order unresolved |
| Security posture | Good | Path validation solid; model checksum needs verification |
| Test coverage | Good for Phase 1 | Manual-only E2E; no macOS/Linux manual test run |
| UX readiness | Adequate | Progress gaps and dead UI buttons are beta-visible issues |

**Overall Phase 1 quality: Fit for purpose as a beta.** Not fit for public v1.0 without addressing the wireframe/solid regression, progress feedback, and synchronous export blocking.

---

## 7. Priority Action List for System Architect

| # | Action | Owner | When |
|---|--------|-------|------|
| 1 | Confirm ADR-008 winding order fix is in uncommitted `RESEARCH/architecture.md` edit; commit | Architect / Senior Eng | Immediately |
| 2 | Remove Wireframe/Solid buttons OR schedule wiring them to indexed mesh in Sprint 2.2 | Architect → UI Designer | Before beta release |
| 3 | Verify SHA256 model download verification is implemented (SEC-202) | Architect + Security Specialist | This week |
| 4 | Schedule F2.4 Undo/Redo as Sprint 2.2 (before any further state-mutation features) | Architect | Sprint planning |
| 5 | Schedule progress streaming for depth estimation as Sprint 2.3/2.4 | Senior Eng | Sprint planning |
| 6 | Update `prd.md` F1.6, `todo.md`, and architecture diagrams for known stale content | Documentation Specialist | Sprint 2.1 close-out |
| 7 | Add `curveControlPoints` to `AppSettings` | Junior Engineer | Sprint 2.2 |
| 8 | Author Svelte store state management ADR before Sprint 2.3 | Architect + UI Designer | Pre-Sprint 2.3 |

---

*End of report. Items 1–3 are the highest-priority findings. The project is in a good structural position; these issues are concentrated and addressable without architectural refactoring.*
