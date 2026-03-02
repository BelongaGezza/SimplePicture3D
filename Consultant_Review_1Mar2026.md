# SimplePicture3D — External Consultant Review

**Prepared for:** System Architect
**Date:** 2026-03-01
**Scope:** Sprint 2.2 delivery assessment, outstanding recommendation status, Phase 2 sequencing
**Status at review:** Phase 1 complete; Phase 2 Sprint 2.2 delivered (pending manual QA sign-off)
**Supersedes:** Consultant_Recommendations_Report_7Feb2026.md, Consultant_Critical_Review_28Feb2026.md
**Documents reviewed:** All sprint artefacts (Sprints 2.1–2.2), CHANGELOG.md, prd.md, todo.md, RESEARCH/architecture.md, src-tauri/src/undo.rs, src-tauri/src/lib.rs, src-tauri/src/settings.rs, src/App.svelte, src/components/Preview3D.svelte, src/components/ExportPanel.svelte, src/lib/tauri.ts, SPRINTS/Sprint_2_2/*, git diff (working tree)

---

## Executive Summary

Sprint 2.2 has delivered its core scope: the undo/redo command stack (F2.4), curve control point persistence (CURVE-001), the state management ADR (TD-01), and the Wireframe/Solid fix (TD-02). All automated quality gates pass — 151 Rust tests (0 failures, 6 ignored), 0 clippy warnings, 39 frontend tests, clean build. The undo implementation is architecturally correct and correctly sequenced before further state-mutation work (masking, presets).

Five of eight priority actions from the 28 Feb report are fully resolved. Two remain open: SHA256 model download verification (SEC-202) and depth estimation progress streaming. One is partially resolved (documentation updates). Manual QA case execution is again documented but not executed at sprint close — a structural pattern now four consecutive sprints long that warrants a process decision.

Sprint 2.3 (Presets & Templates) may begin. The undo/redo and state ADR prerequisites are in place.

---

## 1. Previous Action Item Status

From the 28 Feb 2026 Critical Review priority action list:

| # | Action | Status | Evidence |
|---|--------|--------|----------|
| 1 | Confirm ADR-008 winding order fix committed | ⚠️ Partial | `RESEARCH/architecture.md` shows modified in git working tree (M); committed state unconfirmed in this review cycle |
| 2 | Remove Wireframe/Solid OR wire to indexed mesh (Sprint 2.2) | ✅ Resolved | UI-1403 complete; `Preview3D.svelte` wired to `MeshData.indices`; Sprint 1.8 overlay removed (UI-1404) |
| 3 | Verify SHA256 model download verification (SEC-202) | ❌ Open | No confirmation received; no evidence of implementation in reviewed source |
| 4 | Schedule F2.4 Undo/Redo as Sprint 2.2 | ✅ Done | Delivered — BACK-1401–1404, UI-1401–1404, JR2-1401, JR2-1402 all complete |
| 5 | Schedule progress streaming as Sprint 2.3/2.4 | 🔄 Planned | Allocated to Sprint 2.4 per todo.md; not yet implemented |
| 6 | Update prd.md F1.6, todo.md, architecture diagrams | ⚠️ Partial | todo.md v1.3 (28 Feb); prd.md status header updated; stl_io reference in §F1.6 and diagram label accuracy unconfirmed |
| 7 | Add curveControlPoints to AppSettings | ✅ Done | CURVE-001 complete; persisted and restored on load |
| 8 | Author Svelte store state management ADR | ✅ Done | ADR-010 added to RESEARCH/architecture.md (ARCH-404) |

**Net: 5 fully resolved, 2 open (SHA256; progress streaming planned), 1 partially resolved (docs).**

---

## 2. Sprint 2.2 Delivery Assessment

### 2.1 Undo/Redo (F2.4) — Delivered, Implementation Sound

`src-tauri/src/undo.rs` implements a `VecDeque<SetDepthParamsCommand>` for the undo stack and `Vec<SetDepthParamsCommand>` for redo. The design is correct:

- New `push` clears the redo stack — prevents history forking when the user branches from a mid-history state
- Capacity is enforced via `pop_front` at both `push` and `push_undo` (redo-then-new path), so the 20-entry limit holds in all traversal directions
- `can_undo` / `can_redo` flags returned from Tauri commands drive button enabled state in the frontend

The command struct stores both `previous` and `new` snapshots of `DepthAdjustmentParams`, enabling bidirectional traversal without a separate "inverse" command class — a pragmatic and readable design for this data volume.

Tauri commands exposed: `undo`, `redo`, `clear_history`. History is cleared on new image load and on generate depth, which is the correct scope boundary.

Four unit tests cover the implementation: execute/undo round-trip (JR2-1401), history cap at 20 entries (JR2-1402), full undo/redo round-trip, and `push` clears redo (the last of which was a blocker resolved at sprint close on 2026-03-01).

**One concern:** The `SetDepthParamsCommand` type wraps a single `DepthAdjustmentParams` struct. The CHANGELOG states undo "applies to depth params and curve control points." Whether curve mutations are captured as discrete command entries — rather than merely persisted via `AppSettings` at the `curveControlPoints` field — must be confirmed. If curve changes do not create stack entries, a user who adjusts a curve then presses Ctrl+Z will see no effect on the curve, which is surprising.

### 2.2 Curve Persistence (CURVE-001) — Delivered

`curveControlPoints` added to `AppSettings`. Curve state now survives application restart. This closes the §2.6 finding from the 28 Feb report ("power tool without a safety net"). The one remaining question is the undo stack integration noted above.

### 2.3 State Management ADR (TD-01) — Delivered

ADR-010 provides canonical guidance on the frontend (Svelte stores) vs. backend (Tauri managed state) division ahead of Sprint 2.3+ state-mutation work. This was the highest-priority architectural prerequisite for masking and presets. It is now satisfied.

### 2.4 Wireframe/Solid Fix (TD-02) — Delivered

`Preview3D.svelte` now builds a `THREE.Mesh` using `MeshData.indices` for both wireframe and solid modes. The fallback overlay for meshes with no triangles uses user-neutral language (no internal sprint references). The "Sprint 1.8" overlay text is removed (UI-1404). The dead UI that was the most credibility-damaging finding of the 28 Feb report is resolved.

Junior Engineer 3D confirmed that `depth_to_point_cloud` returns `MeshData.indices` for all grid dimensions ≥ 2×2; the overlay only appears in genuine edge cases (single-row or single-column depth maps).

### 2.5 Automated Quality Gate — Pass

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | PASS — 151 passed, 6 ignored |
| cargo clippy | 0 warnings | 0 warnings |
| npm run build | PASS | PASS (A11y warnings in CurvesTool.svelte only; not build-blocking) |
| npm test | PASS | PASS — 39 tests, 5 files |

Total automated test count at Sprint 2.2: **190** (151 Rust + 39 frontend), up from approximately 117 at Sprint 1.6. The undo module contributes four of the new Rust tests.

### 2.6 Manual QA — Not Executed (Recurring Pattern)

**QA-1401 manual test cases (6 cases + regression) are documented in `TEST_PLAN_2_2.md §3.2` and `MANUAL_TEST_REPORT.md` but have not been executed.** The automated gate passes; the sprint cannot be formally closed without manual sign-off per the verification checklist.

**QA-1402 (macOS smoke tests)** are appropriately deferred — no macOS environment was available this sprint. `MACOS_SMOKE.md` documents the plan for Sprint 3.1. This is a reasonable disposition.

**Pattern observation:** Manual QA has been documented but not executed at sprint close for the last four sprint-close reviews (Sprint 1.6, Sprint 1.11/1.12, Sprint 2.1, Sprint 2.2). The automated quality gate is consistently clean; the manual execution consistently deferred to "human tester to run app." This is a process gap, not a quality gap — but left unresolved it means the team does not have end-to-end validation of any sprint's interactive flows.

---

## 3. Remaining Open Issues

### 3.1 SHA256 Model Download Verification — Unresolved Security Gap

First raised in the 28 Feb report §2.5 (SEC-202). The `download_model` Tauri command downloads a 2GB+ model file executed via Python subprocess. If SHA256 verification is absent or bypassed, a corrupted or intercepted download could lead to execution of untrusted Python. No evidence of resolution appears in Sprint 2.2 artefacts or source review.

This is not a theoretical risk: model distribution via Hugging Face is an increasingly common supply-chain target. Before Sprint 2.4 restructures the Python bridge, the Security Specialist must confirm the implementation status.

### 3.2 Depth Estimation Progress Streaming — Still the #1 UX Gap

`generate_depth_map` remains a synchronous blocking Tauri command. The Python subprocess emits `PROGRESS` lines on stderr, but these are only parsed after the subprocess exits. The `GenerateDepthMapResponse.progress` field returns 100 in all cases. For a 4K CPU inference run, users wait up to 5 minutes with a spinner and no percentage display.

This has been planned for Sprint 2.4 since the 28 Feb report. It must not slip further — it is the most user-visible deficiency in the current beta.

### 3.3 Documentation Partially Updated

The 28 Feb report noted `prd.md §F1.6` references the `stl_io` crate (not in `Cargo.toml`; the project uses custom writers per ADR-008), and the architecture overview diagram in `CLAUDE.md`, `RESEARCH/architecture.md`, and `prd.md` still shows `Svelte or React` and `subprocess (stdin/temp file)` instead of `Svelte 4` and `subprocess (temp file → stdout)`. These were assigned to the Documentation Specialist as Sprint 2.1 close-out work. Their current state is unconfirmed.

**Action:** The Documentation Specialist should verify and commit these two corrections before Sprint 2.3 begins, as stale diagram labels mislead new contributors.

---

## 4. New Observations

### 4.1 Undo/Redo Scope for Curve Mutations Needs Explicit Confirmation

As noted in §2.1, the command implementation covers `DepthAdjustmentParams`. The CHANGELOG asserts undo covers curve control points. If curve mutations are modelled as changes to a `curveControlPoints` field within `DepthAdjustmentParams` (and thus captured by the existing command type), undo works correctly. If curve mutations are handled as a separate code path that only writes to `AppSettings`, they will not appear in the undo stack.

This must be confirmed in code before Sprint 2.3, since Presets will likely interact with both the curve state and the undo stack.

### 4.2 lib.rs Coverage Gap Grows with Each Sprint

The 7 Feb report identified `lib.rs` at approximately 6% coverage because Tauri command handlers require a running Tauri context to exercise. Sprint 2.2 adds three more Tauri command handlers (`undo`, `redo`, `clear_history`) to `lib.rs`, widening the gap. The recommendation to extract business logic from command handlers into standalone testable functions has not been acted on across two sprint cycles.

This compounds: with Phase 2 adding presets, masking, and material settings, the untestable surface area in `lib.rs` will continue to grow if the pattern is not changed. The Senior Engineer should treat this as a Sprint 2.3 or 2.4 task, not a Phase 3 item.

### 4.3 A11y Warnings in CurvesTool.svelte

`npm run build` passes but with accessibility warnings in `CurvesTool.svelte`. Interactive SVG curve editors with drag handles are inherently challenging for keyboard and screen reader users. These warnings are not build-blocking, but they represent a real accessibility gap for the application. Sprint 2.6 (Enhanced 3D Preview) is the natural home for a broader UX polish pass that could include this.

### 4.4 Export Blocking and Progress Streaming Share an Architectural Fix

Sprint 2.4 (progress streaming) and Sprint 2.8 (async export) both require converting synchronous blocking Tauri commands into async operations that emit progress events. The architectural pattern — `tokio::task::spawn_blocking` or an async command + `tauri::Emitter` for progress updates — is the same in both cases. If Sprint 2.4 establishes this pattern cleanly, Sprint 2.8 becomes a straightforward application of it. Consider whether Sprint 2.4's scope should explicitly produce a reusable async-command-with-progress pattern rather than solving only the depth streaming case.

### 4.5 Manual QA Process Decision Needed

The team has produced high-quality manual test procedures (TEST_PLAN_2_2.md, MANUAL_TEST_REPORT.md) but no sprint has had those procedures executed before close. The architect should make one of two explicit decisions:

- **Option A (blocking):** Manual QA is a sprint exit criterion. No sprint closes until cases are executed and signed off. This slows the sprint cadence but provides true end-to-end validation.
- **Option B (non-blocking):** Manual QA is a post-sprint validation activity with a fixed 48-hour window and a named tester. Sprints close on automated gate pass; manual results are filed within the window and any regressions become the first task of the next sprint.

Either is defensible. The current implicit approach (procedure documented; execution deferred indefinitely) is not.

---

## 5. Technical Debt Register

| ID | Area | Issue | Priority | Status |
|----|------|--------|----------|--------|
| TD-01 | Frontend | State management ADR | High | ✅ Closed — ADR-010 (Sprint 2.2) |
| TD-02 | Frontend | Wireframe/Solid dead UI | High | ✅ Closed — Fixed (Sprint 2.2, UI-1403) |
| TD-03 | Docs | architecture.md dual-maintenance policy | Medium | Open |
| TD-04 | Python | setup.py vs pyproject.toml | Low | Open — Phase 4 |
| TD-05 | Cross-platform | No macOS/Linux manual test run | Medium | Partially addressed — MACOS_SMOKE.md plan, Phase 3 |
| TD-06 | Security | TOCTOU export path write-test pattern | Low | Open — Phase 3 |
| TD-07 | Python distribution | System Python end-user friction | High | Open — Phase 4 decision pending |
| TD-08 | Rust | lib.rs coverage gap grows with each sprint's new Tauri handlers | Medium | **New (Sprint 2.2) — escalating** |
| TD-09 | Frontend | A11y warnings in CurvesTool.svelte | Low | New (Sprint 2.2) |
| TD-10 | QA process | Manual QA structurally deferred at every sprint close | Medium | New (recurring pattern; needs process decision) |

---

## 6. Phase 2 Sequencing — Updated

The sequencing recommendation from 28 Feb stands. Sprint 2.2 delivered its prerequisite role on schedule.

| Sprint | Feature | Status |
|--------|---------|--------|
| 2.2 | Undo/Redo, curve persistence, state ADR, Wireframe/Solid | ✅ Delivered (automated gate PASS; manual QA pending) |
| 2.3 | Presets & Templates (F2.3) | Ready to begin |
| 2.4 | Progress streaming for depth estimation | Do not defer; highest UX priority |
| 2.5 | Masking & Regional Adjustments (F2.1 remainder) | Needs undo + presets; confirm ADR-010 covers mask state |
| 2.6 | Enhanced 3D Preview (F2.5) — PBR lighting, orbit controls | Natural home for A11y fix and 3D UX polish |
| 2.7 | Material Presets (F2.6) | Depends on presets framework (2.3) |
| 2.8 | Async export + progress indicators | Use Sprint 2.4 pattern; pair if scope permits |

**One forward-looking concern before Sprint 2.5:** Masking introduces a new state dimension — regions, brush strokes, layer blending — that the current `SetDepthParamsCommand` struct (a flat snapshot of scalar params) does not model. ADR-010 provides state management guidance at the Svelte/Tauri boundary, but the command pattern may need extension before masking can be made undoable. The Architect and Senior Engineer should address this in pre-Sprint-2.5 planning, not at the start of the sprint.

---

## 7. Overall Assessment

| Dimension | Rating | Change | Comment |
|-----------|--------|--------|---------|
| Sprint delivery fidelity | Good | — | Core scope delivered on schedule; manual QA deferred (recurring) |
| Undo/Redo implementation | Strong | New | Command pattern correct; capacity management sound; tests cover edge cases |
| State management | Improved | ↑ | ADR-010 in place; provides guidance before masking and preset sprints |
| Frontend architecture | Improved | ↑ | Wireframe/Solid resolved; undo toolbar; dead UI eliminated |
| Backend (Rust) | Strong | — | 151 tests; clean clippy; undo module well-structured |
| Python bridge | Adequate | — | Functional but progress streaming still absent |
| Security posture | Gap remains | — | SHA256 model checksum unverified entering Sprint 2.3 |
| Cross-platform readiness | Not started | — | macOS plan documented; no smoke run yet |
| QA process | Structural concern | ↓ | Pattern of manual deferral now four sprints long; needs resolution |
| Documentation | Adequate | — | Two confirmed stale items unresolved from 28 Feb report |

**Overall Phase 2 position: On track.** The undo/redo foundation is correctly in place. The two open risks — SHA256 and progress streaming — are manageable but must not compound further. Sprint 2.3 can begin. The architect's immediate focus should be the SHA256 verification question and the manual QA process decision.

---

## 8. Priority Action List

| # | Action | Owner | When |
|---|--------|-------|------|
| 1 | Verify SHA256 model download checksum is implemented (SEC-202) | Security Specialist | Before Sprint 2.4 |
| 2 | Decide: manual QA is blocking (sprint exit gate) or non-blocking (48h post-sprint window) | Architect + QA | Immediately |
| 3 | Execute QA-1401 manual test cases (6 cases + regression) for Sprint 2.2 sign-off | Quality Engineer | This week |
| 4 | Confirm curve mutations create `SetDepthParamsCommand` entries (not just AppSettings write) | Senior Engineer | Sprint 2.3 planning |
| 5 | Confirm ADR-008 winding order fix is committed in RESEARCH/architecture.md | Architect / Senior Engineer | Immediately |
| 6 | Verify and commit prd.md §F1.6 (stl_io reference) and diagram label corrections | Documentation Specialist | Before Sprint 2.3 |
| 7 | Schedule Sprint 2.4 (progress streaming) — do not defer | Architect | Sprint planning |
| 8 | Assess whether ADR-010 state model covers mask state before Sprint 2.5 planning | Architect + Senior Engineer | Pre-Sprint 2.5 |
| 9 | Address lib.rs coverage gap — extract handler logic into testable functions (TD-08) | Senior Engineer | Sprint 2.3 or 2.4 |

---

*End of report. Items 1–3 are the highest-priority findings. Sprint 2.2 is the strongest Phase 2 delivery to date: the automated gate is clean, the undo implementation is architecturally sound, and two long-standing UI regressions (Wireframe/Solid, Sprint overlay) are closed. The remaining concerns are concentrated and addressable without architectural rework.*
