# SimplePicture3D — Program TODO and Sprint Roadmap

**Version:** 4.0  
**Last Updated:** 2026-05-10  
**Owner:** System Architect

---

## Current Reality (Restart Review — 2026-05-10)

This roadmap reflects a deliberate restart decision. The goal is a single-purpose 3D surface point cloud tool for internal UV laser engraving of crystal glass. All prior 2.5D relief work is being retired. See ADR-012 in `RESEARCH/architecture.md` for the algorithm decision.

### What Is Working and Retained
- Depth pipeline: image load, depth estimation (Python bridge), depth adjustments, mask, undo/redo, presets, progress streaming.
- `blank_envelope.rs` — `BlankEnvelope` struct, `fit_to_blank`, presets. Fully tested. Keep as-is.
- `export.rs` — PLY/XYZ/CSV writers. Keep; verify output against format specs in Sprint B.
- `volumetric.rs` — struct/param scaffolding and `fit_to_blank` wiring are reusable. **The `generate_volumetric_points` algorithm (column sweep) must be replaced** with a surface-map algorithm (ADR-012) before wiring. See TD-14.
- Svelte UI framework: three-panel layout, depth controls, Three.js preview infrastructure.
- Python bridge contract: Rust `--input` temp file, Python emits JSON depth + `PROGRESS`/`STAGE` on stderr.

### What Is Being Removed
- `mesh_generator.rs` — 2.5D surface triangulation (STL/OBJ). Delete in Sprint A.
- Relief mode UI components (STL/OBJ export panels, relief-specific controls). Remove in Sprint A.
- `RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md` — superseded by this restart decision; archive or delete.

### What Remains Unwired (the core problem)
- `blank_envelope.rs`, `volumetric.rs`, `export.rs` are not connected to any Tauri commands or frontend flows.
- No Tauri commands exist for: `set_blank_envelope`, `generate_point_cloud`, `export_ply`, `export_xyz`, `export_csv`.
- Product metadata is stale: `src-tauri/tauri.conf.json` bundle description still references 2.5D STL/OBJ.
- Capability permissions are uneven; needs regularisation when new commands are added.

---

## Program Priorities

1. **P0: Replace the column-sweep algorithm with surface-map (ADR-012)**
   - The existing `generate_volumetric_points` fills the volume from back plane to depth surface.
   - Crystal laser engraving needs one point per (x,y) sample, placed at the Z position given by the depth value.
   - Solid fill produces a cloudy, indistinct result; surface-map produces the clean 3D shape outline the laser traces.

2. **P0: Remove all 2.5D relief code and retire the dual-mode design**
   - This is a single-purpose 3D surface point cloud app. No relief mode.
   - Delete `mesh_generator.rs`; remove relief UI.

3. **P0: Wire the volumetric backend into real Tauri commands**
   - `blank_envelope.rs`, `volumetric.rs`, `export.rs` must be connected to `lib.rs` and the frontend.

4. **P1: Build the frontend volumetric UI**
   - Blank envelope controls, volumetric params panel, point count estimator, Three.js preview (point cloud + blank wireframe), export panel.

5. **P1: Validate output with real engraver software**
   - At least one confirmed end-to-end path before claiming v1.0.

6. **P2: Defer Full 3D reconstruction (TripoSR)**
   - Keep as a documented future option; do not interleave with core stabilisation.

---

## Phases

### Phase A — Clean House (1 week)
**Goal:** remove 2.5D dead weight; update all docs and metadata to reflect single-purpose 3D intent.

**Exit Criteria**
- `mesh_generator.rs` deleted; relief UI removed; no STL/OBJ references in active code paths.
- `tauri.conf.json`, `README.md`, `prd.md` describe a 3D surface point cloud app, not a 2.5D relief tool.
- Documentation status claims match repository reality; no “pivot complete” false signals.

### Phase B — Backend Wiring (1 week)
**Goal:** expose the complete 3D surface point cloud pipeline as a real Tauri API.

**Exit Criteria**
- `generate_volumetric_points` rewritten to surface-map algorithm (ADR-012): one point per (x,y) sample at depth-mapped Z.
- Tauri commands registered and testable: `set_blank_envelope`, `generate_point_cloud`, `export_ply`, `export_xyz`, `export_csv`.
- `settings.rs` persists blank envelope and export format preference.
- All new commands have capability permission entries.
- Command-level tests cover surface-map generation and all three export formats.

### Phase C — Frontend UI (1–2 weeks)
**Goal:** make the full 3D surface point cloud workflow usable from the UI.

**Exit Criteria**
- Blank envelope controls (L×W×H mm, margin, standard presets) visible and persisted.
- Volumetric params panel: XY step, depth threshold, point count estimate updated live.
- Three.js preview renders point cloud inside blank wireframe box.
- Export panel: format selector (PLY / XYZ / CSV) with save-to-file.
- Full workflow completable without touching the terminal.

### Phase D — Validate and Harden (1 week)
**Goal:** confirm output is compatible with real engraver toolchains; tighten security boundaries.

**Exit Criteria**
- At least one validated E2E path: image → point cloud → imported into engraver software with correct axis orientation and units.
- Engraver charter documented: accepted formats, axis convention, unit expectations.
- Capability permissions align with minimum-privilege intent; command/permission audit passes.
- Performance: 4K image → point cloud in < 2 minutes on mid-range hardware.

---

## Sprint Plan

All sprints are 1 week unless explicitly marked.

### Sprint A — Clean House
**Goal:** delete 2.5D code; make docs and metadata truthful.

**Tasks**
- **BACK-A-01:** Delete `src-tauri/src/mesh_generator.rs` and remove all references from `lib.rs`, `Cargo.toml`, and tests.
- **UI-A-01:** Remove relief-mode UI components (STL/OBJ export panel, relief-specific control groups).
- **DOC-A-01:** Update `tauri.conf.json` bundle description to reflect 3D surface point cloud purpose.
- **DOC-A-02:** Update `README.md` — replace all 2.5D/relief/STL/OBJ language; describe actual workflow.
- **DOC-A-03:** Update `prd.md` header status block and §F3.5 feature description to surface-map algorithm (ADR-012).
- **DOC-A-04:** Update `CLAUDE.md` current status and data flow to match restart.
- **DOC-A-05:** Archive `RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md` (rename to `RESEARCH/archive/PIVOT_PLAN_archived.md`).
- **ARCH-A-01:** Add ADR-012 to `RESEARCH/architecture.md` documenting the surface-map algorithm decision.

**Gate**
- Sprint B cannot start until `mesh_generator.rs` is deleted and tests pass with `cargo test`.

---

### Sprint B — Backend Wiring
**Goal:** rewrite the generation algorithm; wire all volumetric modules into Tauri commands.

**Tasks**
- **BACK-B-01 (TD-14):** Rewrite `generate_volumetric_points` in `volumetric.rs` to surface-map algorithm per ADR-012.
  - For each sampled (x, y): compute `z = margin + (1.0 - depth[x,y]) × interior_depth`; emit one point `(x_mm, y_mm, z_mm)`.
  - Remove `z_spacing_mm` and `back_plane` from `VolumetricParams`; add `depth_threshold` (minimum depth value to emit a point).
  - Update all unit tests in `volumetric.rs` to reflect the new one-point-per-column contract.
- **BACK-B-02:** Add and register Tauri commands in `src-tauri/src/lib.rs`: `set_blank_envelope`, `generate_point_cloud`, `export_ply`, `export_xyz`, `export_csv`.
- **BACK-B-03:** Wire `settings.rs` — add `blank_envelope: BlankEnvelope` and `export_format: ExportFormat` to persisted app settings.
- **BACK-B-04:** Connect `volumetric.rs` generation to depth+mask adjusted data from existing pipeline state.
- **BACK-B-05:** Verify `export.rs` PLY/XYZ/CSV output against format specs; fix any header or precision issues.
- **SEC-B-01:** Review new command surface; ensure export path validation matches existing `file_io.rs` patterns.
- **QA-B-01:** Add command-level integration tests for `generate_point_cloud` and all three export commands.

**Exit Criteria**
- `cargo test` passes with new surface-map algorithm.
- All five new commands callable from Tauri frontend layer.
- PLY/XYZ/CSV files producible from a real loaded image (not isolated unit tests only).

---

### Sprint C — Frontend UI
**Goal:** expose the full 3D workflow through the Svelte UI.

**Tasks**
- **UI-C-01:** Add blank envelope panel: L / W / H mm inputs, margin input, preset dropdown (Standard 80×50×50, Cube 60×60×60, Large 100×60×60, Tall 50×50×80).
- **UI-C-02:** Add volumetric params panel: XY step slider (1–8 px), depth threshold slider (0.0–1.0), live point count estimate (calls `estimate_point_count` via Tauri).
- **UI-C-03:** Update `Preview3D.svelte` — render point cloud from `generate_point_cloud` response; overlay blank wireframe box using Three.js `EdgesGeometry`.
- **UI-C-04:** Update `ExportPanel.svelte` — format selector (PLY / XYZ / CSV); replace STL/OBJ with new export commands.
- **UI-C-05:** Persist blank envelope and export format through `settings.rs` load/save cycle.
- **QA-C-01:** Manual UX pass: full workflow from image load → blank setup → generate → preview → export.

**Exit Criteria**
- User can complete the full workflow without touching the terminal.
- Preview shows point cloud correctly positioned inside the blank wireframe.

---

### Sprint D — Validate and Harden
**Goal:** confirm engraver compatibility; tighten permissions; check performance.

**Tasks**
- **ARCH-D-01:** Document engraver charter: axis convention (which app axis maps to laser X/Y/Z), units (mm), accepted formats, point ordering expectations.
- **QA-D-01:** Export PLY and XYZ from a real image; import into target engraver software; verify axis orientation and point distribution.
- **BACK-D-01:** Fix any compatibility defects found (axis flips, unit scaling, header metadata, decimal precision).
- **SEC-D-01:** Split coarse permissions (`allow-generate-depth-map`) into least-privilege capability groups aligned with actual command groups.
- **ARCH-D-02:** Re-run command registration/invocation consistency audit (`lib.rs` ↔ `tauri.ts` ↔ capability files).
- **QA-D-02:** Performance check — time from image load to PLY export for a 4K image on a mid-range machine.
- **DOC-D-01:** Publish interoperability notes and known-good engraver configuration in `docs/`.

**Exit Criteria**
- At least one validated real-world engraver path documented with repeatable settings.
- Capability model aligns with minimum-privilege intent.
- Performance target met or blocker documented with mitigation plan.

---

## Deferred Tracks (Explicit)

- **Full 3D reconstruction (TripoSR)** — deferred until after Sprint D validation. Documented in `RESEARCH/3d-reconstruction.md`.
- **Major UI polish** (dark mode, broad beta polish) — deferred until volumetric flow is engraver-validated.
- **Cross-platform rollout** (macOS, Linux packaging) — Phase 3, after Phase D stability confirmed on primary platform.

---

## Active Technical Debt Register

| ID | Title | Severity | Plan |
|---|---|---|---|
| TD-14 | `generate_volumetric_points` uses column-sweep (fills volume) instead of surface-map (one point per column) | **Critical** | Sprint B — BACK-B-01 |
| TD-11 | Volumetric modules not wired to Tauri commands or frontend | Critical | Sprint B — BACK-B-02 |
| TD-13 | Command-permission granularity inconsistent | High | Sprint D — SEC-D-01 |
| TD-08 | `lib.rs` testability/coverage gap | High | Dedicated refactor slice in Phase D |
| TD-06 | Export path TOCTOU write-test pattern | Medium | Sprint D security hardening |
| TD-09 | Accessibility issues in advanced controls | Medium | Post-Phase D polish |

---

## Sprint Creation Process

When generating a sprint package:
- Use `SPRINTS/SPRINT_TASKING_TEMPLATE.md`.
- Create `SPRINTS/Sprint_X_Y/SPRINT_X_Y_Task_Assignment.md`.
- Store all reports/checklists/sign-offs in that sprint folder.
- Use `.agents/*.md` personas for role ownership.

---

## Notes

- This file is the operational source of truth for scheduling.
- **Do not mark Phase B complete** until `generate_point_cloud` produces a real PLY/XYZ file from a loaded image.
- **Do not mark Phase D complete** until at least one engraver software import is validated end-to-end.
- If scope changes, update this file and `prd.md` in the same change set to prevent drift.
- The column-sweep code in `volumetric.rs` **must not be wired** as-is — it produces a solid fill, not a surface outline. See TD-14 and ADR-012.
