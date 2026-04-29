# SimplePicture3D — Program TODO and Sprint Roadmap

**Version:** 3.0  
**Last Updated:** 2026-04-29  
**Owner:** System Architect

---

## Current Reality (Architect Review)

This roadmap is based on actual code status, not prior planning assumptions.

### As-Built and Working
- Core relief pipeline is operational: image load, depth generation, depth adjustments, mask tools, undo/redo, presets, preview, STL/OBJ export.
- Python bridge contract is consistent: Rust uses `--input` temp file, Python emits JSON depth plus `PROGRESS`/`STAGE` on stderr.
- Sprint 2.1–2.4 capabilities (histogram/curves, undo/redo, presets, progress streaming) are integrated in the app.

### Dead-Ends and Architecture Drift (must fix)
- `blank_envelope.rs`, `volumetric.rs`, and `export.rs` exist but are not wired into active Tauri commands or frontend flows.
- No frontend invocations for `export_ply`, `export_xyz`, `export_csv`, or volumetric generation path.
- `todo.md` and portions of `prd.md` previously claimed volumetric path was already shipped; this is inaccurate.
- Product metadata is stale: `src-tauri/tauri.conf.json` bundle description still says 2.5D STL/OBJ.
- Capability design is uneven: broad grouped permission (`allow-generate-depth-map`) plus missing granular permissions for several invoked commands (should be regularized during volumetric integration).

### Consistency Rule Check (this review cycle)
- Reviewed `RESEARCH/GOTCHAS.md` before planning.
- Verified Rust command declarations in `src-tauri/src/lib.rs` are registered in `tauri::generate_handler!`.
- Verified frontend command usage from `src/lib/tauri.ts` and app/components.
- Reviewed capability file and command permission coverage for current/next scope.
- Verified Rust↔Python IO contract alignment between `src-tauri/src/python_bridge.rs` and `python/python/depth_estimator.py`.

---

## Program Priorities (P0–P2)

1. **P0: Close quality gates on already-shipped relief flow**
   - Complete pending manual QA backlog for masking, presets, progress streaming, and undo/redo.
   - Eliminate known regressions before adding new major flows.

2. **P0: Integrate volumetric path end-to-end (ADR-011)**
   - Add explicit backend commands for blank envelope + volumetric generation + PLY/XYZ/CSV export.
   - Wire frontend mode and export UI to those commands.
   - Keep relief mode available until volumetric path is proven on real engraver software.

3. **P1: Resolve architecture/documentation drift**
   - Bring `todo.md`, `prd.md`, `README.md`, and app metadata into the same truth model.
   - Avoid claiming “pivot complete” until volumetric path is selectable and exportable in UI.

4. **P1: Stabilize command/capability boundaries**
   - Move from legacy broad permissions toward least-privilege capability files aligned with actual command groups.

5. **P2: Defer optional Full 3D reconstruction track**
   - Keep TripoSR as post-MVP for volumetric branch; do not interleave with volumetric core stabilization.

---

## Revised Remaining Phases

## Phase A — Stabilization and Truth Alignment (2–3 sprints)
**Goal:** remove dead-ends and clear QA debt while keeping relief path stable.

### Exit Criteria
- All outstanding Phase 2 manual QA cases are either passed or explicitly deferred with rationale/date.
- Documentation status claims match repository reality.
- No P0/P1 defects in mask/undo/preset/progress features.

## Phase B — Volumetric Integration (3–4 sprints)
**Goal:** ship first true ADR-011 workflow in-app (blank-first, volumetric generation, PLY/XYZ/CSV export).

### Exit Criteria
- User can choose volumetric mode, set blank envelope, generate volumetric preview, export PLY/XYZ/CSV.
- New backend commands and capabilities are in place, tested, and documented.
- Relief mode remains functional and intentionally labeled (legacy/advanced).

## Phase C — Engraver Validation and Release Hardening (2 sprints)
**Goal:** validate outputs against target engraver software and harden UX/perf around volumetric flow.

### Exit Criteria
- E1 charter complete (axis conventions, accepted formats, acceptance tolerances).
- Manual E2E passes with real engraver toolchain.
- Blocking interoperability gaps resolved or documented with mitigations.

## Phase D — Cross-Platform and Production Readiness (Phase 3+4 continuation)
**Goal:** platform parity, packaging, performance, accessibility, and release operations.

### Exit Criteria
- Existing Phase 3/4 gates remain, but anchored to the integrated volumetric baseline.

---

## Sprint Plan (Rebased)

All sprints are 2 weeks unless explicitly marked.

### Sprint 2.6R — QA Closure and Drift Cleanup
**Goal:** close known QA gaps and remove false “done” signals.

**Tasks**
- **QA-2.6R-01:** Re-run masking manual cases (`QA-1201`–`QA-1203`) and sign off or document residual defects.
- **QA-2.6R-02:** Execute pending manual tests (`QA-1101`–`QA-1103`, `QA-1301`–`QA-1303`, `QA-1401`, `QA-304-STREAM`).
- **ARCH-2.6R-01:** Rewrite status docs to reflect actual integration state (no “pivot complete” claims).
- **DOC-2.6R-01:** Update `README.md` status section to “foundation implemented, integration pending”.
- **BACK-2.6R-01:** Fix any critical defects surfaced by manual QA reruns.

**Gate**
- Sprint 2.7 cannot start until masking QA is closed or explicit architect waiver is documented.

---

### Sprint 2.7R — Backend Volumetric Wiring
**Goal:** expose ADR-011 pipeline as real backend API.

**Tasks**
- **ARCH-2.7R-01:** Finalize command contract for volumetric flow (`set_blank_envelope`, `get_point_cloud_data`, `export_ply`, `export_xyz`, `export_csv`).
- **BACK-2.7R-01:** Add and register volumetric commands in `src-tauri/src/lib.rs`.
- **BACK-2.7R-02:** Wire `settings.rs` blank envelope and point-cloud format fields into active load/save path.
- **BACK-2.7R-03:** Integrate `volumetric.rs` generation path with depth+mask adjusted data.
- **BACK-2.7R-04:** Integrate `export.rs` file export path with secure path validation parity.
- **QA-2.7R-01:** Add command-level tests for volumetric generation and PLY/XYZ/CSV export.
- **SEC-2.7R-01:** Review new command surface and export path handling.

**Exit Criteria**
- Volumetric commands callable from frontend layer.
- PLY/XYZ/CSV exports produced from real app state (not isolated module tests only).

---

### Sprint 2.8R — Frontend Volumetric Mode Integration
**Goal:** make volumetric flow usable from UI without removing relief fallback.

**Tasks**
- **UI-2.8R-01:** Add mode selector (`Relief` vs `Volumetric`) with safe default behavior.
- **UI-2.8R-02:** Add blank envelope controls (dimensions + margin) and persist through settings.
- **UI-2.8R-03:** Update `ExportPanel.svelte` to expose PLY/XYZ/CSV when volumetric mode is active.
- **UI-2.8R-04:** Add volumetric preview behavior in `Preview3D.svelte` (point cloud + blank wireframe).
- **UI-2.8R-05:** Keep existing STL/OBJ path in relief mode as advanced/legacy.
- **QA-2.8R-01:** Manual UX pass for mode switching, settings persistence, and exports.

**Exit Criteria**
- User can complete full volumetric workflow from UI.
- Relief workflow still works and is clearly scoped.

---

### Sprint 2.9R — Capability, Contract, and Safety Hardening
**Goal:** tighten security and architectural boundaries after integration.

**Tasks**
- **SEC-2.9R-01:** Split coarse permissions into least-privilege command groups.
- **BACK-2.9R-01:** Ensure all frontend-invoked commands have explicit permission coverage.
- **ARCH-2.9R-01:** Re-run command registration/invocation consistency audit.
- **QA-2.9R-01:** Regression suite across both modes and all export formats.
- **DOC-2.9R-01:** Update developer docs with finalized command and capability map.

**Exit Criteria**
- Capability model aligns with actual command usage and minimum-privilege intent.
- No command drift between Rust, frontend, and permissions.

---

### Sprint 3.0R — Engraver Charter and Validation (E1 + E2E)
**Goal:** close the practical adoption risk by validating real toolchain compatibility.

**Tasks**
- **ARCH-3.0R-01:** Finalize engraver charter (axis conventions, units, accepted formats, sample profiles).
- **QA-3.0R-01:** Execute manual export/import validation on target engraver software.
- **BACK-3.0R-01:** Fix compatibility defects (axis flips, bounds, decimal precision, header metadata).
- **DOC-3.0R-01:** Publish interoperability notes and known-good configuration matrix.

**Exit Criteria**
- At least one validated real-world engraver path with repeatable settings and expected output quality.

---

## Deferred Tracks (Explicit)

- **Full 3D reconstruction (TripoSR)** remains deferred until after Sprint 3.0R success.
- **Major UI polish initiatives** (dark mode, broad beta polish) are deferred until volumetric flow is validated end-to-end.
- **Cross-platform rollout** (Phase 3) remains after volumetric integration stability on primary target.

---

## Active Technical Debt Register

| ID | Title | Severity | Plan |
|---|---|---|---|
| TD-11 | Volumetric modules are orphaned from runtime pipeline | Critical | Sprint 2.7R + 2.8R |
| TD-12 | Documentation claims shipped features that are not wired | Critical | Sprint 2.6R |
| TD-13 | Command-permission granularity is inconsistent | High | Sprint 2.9R |
| TD-08 | `lib.rs` testability/coverage gap | High | schedule dedicated refactor slice in Phase C |
| TD-09 | Accessibility issues in advanced controls | Medium | Phase C polish |
| TD-06 | Export path TOCTOU write-test pattern | Medium | Phase C security hardening |

---

## Sprint Creation Process (Unchanged)

When generating a sprint package:
- Use `SPRINTS/SPRINT_TASKING_TEMPLATE.md`.
- Create `SPRINTS/Sprint_X_Y/SPRINT_X_Y_Task_Assignment.md`.
- Store all reports/checklists/sign-offs in that sprint folder.
- Use `.agents/*.md` personas for role ownership.

---

## Notes

- This file is the operational source of truth for scheduling.
- Do not mark ADR-011 “implemented” in project status until Sprint 2.8R exit criteria are met.
- If scope changes, update this file and `prd.md` in the same change set to prevent drift.
