# SimplePicture3D — Program TODO and Sprint Roadmap

**Version:** 4.4  
**Last Updated:** 2026-05-24  
**Owner:** System Architect  
**Latest Commit:** see git log  
**Production path:** `ROADMAP.md`  
**Takeover review:** `docs/ARCHITECT_REVIEW_2026-05.md`  
**Dual-agent review (2026-05-24):** Security Specialist + Quality Engineer — findings below; sprint artefacts in `SPRINTS/Sprint_D/`

---

## Architect Review Summary (2026-05-24)

Delegated read-only reviews to **Security Specialist** and **Quality Engineer** sub-agents. Verified on macOS: `cargo build` / `cargo test` (172) / clippy green after **SEC-E-01** fix; `npm test` not re-run this session; Python pytest requires venv per `SETUP_NOTES.md`.

| Area | Verdict | Blocker |
|------|---------|---------|
| Phases A–C (code) | Substantially complete | Formal gate: QA-C-01 manual UX |
| Phase D (validation) | Not started (manual QA) | Engraver software TBD; SEC-D-02 real hashes |
| Security Wave 1–2 | Partially landed in source | **SEC-E-01** ✅ fixed 2026-05-24; SEC-D-05 path policy active |
| CI | Linux + Windows backend jobs | Frontend still Linux-only |
| Sprint D sign-off | Stale | `SECURITY_SIGNOFF.md` unchecked; ORCHESTRATION overstates “Complete” for SEC-D-05 |

**Immediate P0 (next):** SEC-D-02 real model hashes; QA-C-01 manual UX pass; ARCH-D-01 engraver name.

---

## Current Reality (Post-Sync — 2026-05-24)

This roadmap reflects the deliberate restart decision (2026-05-10) plus two implementation commits landed from GitHub (`17ae055`, `6bbe3da`). The goal remains a single-purpose 3D surface point cloud tool for internal UV laser engraving of crystal glass. Canonical algorithm: ADR-012 in `RESEARCH/architecture.md`.

### What Is Working
- Depth pipeline: image load, depth estimation (Python bridge), depth adjustments, mask, undo/redo, presets, progress streaming.
- `blank_envelope.rs` — `BlankEnvelope` struct, `fit_to_blank`, presets. Fully tested.
- `export.rs` — PLY/XYZ/CSV writers; wired to Tauri export commands.
- `volumetric.rs` — **surface-map algorithm (ADR-012)** implemented: one point per (x,y) at depth-mapped Z; `depth_threshold`, `step_x`/`step_y`; unit tests pass in isolation.
- Tauri commands registered and permissioned (`allow-point-cloud.toml`): `set_blank_envelope`, `set_volumetric_params`, `set_point_cloud_format`, `estimate_point_cloud_count`, `generate_point_cloud`, `export_ply`, `export_xyz`, `export_csv`.
- `settings.rs` persists `blank_envelope`, `volumetric_params`, and `point_cloud_format`.
- Frontend: `ExportPanel.svelte` — blank presets (Standard/Cube/Large/Tall), PLY/XYZ/CSV export workflow.
- Frontend: `Preview3D.svelte` — point cloud from `generate_point_cloud`, crystal blank wireframe overlay.
- `mesh_generator.rs` deleted; STL/OBJ export UI retired.
- `tauri.conf.json` bundle description updated for 3D surface point cloud purpose.
- Python bridge contract unchanged: Rust `--input` temp file, Python emits JSON depth + `PROGRESS`/`STAGE` on stderr.

### Remaining Gaps (next work)
- **SEC-E-01:** ✅ Fixed 2026-05-24 — `use path_security::{is_blocklisted, validate_import_file_path}` in `lib.rs`.
- **QA-B-01:** ✅ Export command integration tests in `lib.rs` (2026-05-24).
- **QA-C-01:** Manual full-workflow UX pass not yet recorded.
- **Phase D:** Engraver import not executed (target software TBD in charter §2); performance benchmark pending; `SECURITY_SIGNOFF.md` unsigned.
- **SEC-D-02:** `expected_model_hashes.json` still has `TODO:REPLACE_WITH_SHA256_*` placeholders — verification skipped at runtime.
- **SEC-D-04 (partial):** Point cloud IPC capped (`MAX_IPC_POINT_CLOUD_POINTS`); **depth map** still returned in full via `get_depth_map` — track as **SEC-E-02**.
- **SEC-D-11:** `withGlobalTauri: true` still set in `tauri.conf.json`.
- **CI-D-01:** ✅ `backend-windows` job in `.github/workflows/ci.yml` (2026-05-24).
- **Sprint artefacts:** `SPRINTS/Sprint_D/TEST_PLAN_D.md` missing; `MANUAL_TEST_REPORT.md` template only.
- **Stale settings comments:** `settings.rs` still documents legacy `export_format` (stl/obj) alongside new `point_cloud_format`.

### Security implementation status (2026-05-24 code review)

| Task | Status | Notes |
|------|--------|-------|
| SEC-D-02 | ⏳ | Stub manifest only; real SHA256 pending trusted download |
| SEC-D-03 | ✅ | `MAX_IMAGE_FILE_BYTES` + header dimension cap in `image_loading.rs` |
| SEC-D-04 | ⏳ | Point cloud IPC capped; depth map IPC unbounded (**SEC-E-02**) |
| SEC-D-05 | ✅ | `path_security.rs` + imports in `lib.rs` (2026-05-24) |
| SEC-D-06 | ✅ | `allow-settings.toml`, `allow-model.toml` in `capabilities/default.json` |
| SEC-D-07–09 | ⏳ | Not implemented |
| SEC-D-10 | ⏳ | Verify `path_security` blocklist parity with export |
| SEC-D-11 | ⏳ | `withGlobalTauri: true` |
| SEC-D-12 | ✅ | PLY/CSV comment sanitization in `export.rs` |

---

## Program Priorities

1. ~~**P0: Replace the column-sweep algorithm with surface-map (ADR-012)**~~ ✅ Done (`17ae055`)

2. ~~**P0: Remove all 2.5D relief code and retire the dual-mode design**~~ ✅ Done (`17ae055`)

3. ~~**P0: Wire the volumetric backend into real Tauri commands**~~ ✅ Done (`17ae055`)

4. **P1: Complete the frontend volumetric UI** — ✅ Done
   - ✅ Blank envelope controls, Three.js preview (point cloud + blank wireframe), export panel (`6bbe3da`)
   - ✅ Volumetric params panel (XY step, depth threshold, live point count estimate)

5. ~~**P0: Fix Rust compile (SEC-E-01)**~~ ✅ Done 2026-05-24.
6. **P1: Validate output with real engraver software** — confirm target app (ARCH-D-01 §2); execute QA-D-01.
7. **P1: Security hardening (Sprint D)** — SEC-D-02 real hashes, SEC-E-02 depth IPC caps, SEC-D-07–11; honest `SECURITY_SIGNOFF.md`.
8. **P1: QA gates** — ~~QA-B-01~~ ✅, ~~CI-D-01~~ ✅; QA-C-01 manual UX pass remaining.
9. **P2: Defer Full 3D reconstruction (TripoSR)** — documented future option only.

---

## Phases

### Phase A — Clean House (1 week) — **mostly complete**
**Goal:** remove 2.5D dead weight; update all docs and metadata to reflect single-purpose 3D intent.

**Exit Criteria**
- ✅ `mesh_generator.rs` deleted; relief UI removed; no STL/OBJ in active export paths.
- ✅ `tauri.conf.json`, `prd.md` describe a 3D surface point cloud app.
- ⏳ `README.md` — updated 2026-05-19 (DOC-A-02 ✅).
- ✅ `RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md` archived (DOC-A-05).

### Phase B — Backend Wiring (1 week) — **blocked on real-image engraver verify (Phase D)**
**Goal:** expose the complete 3D surface point cloud pipeline as a real Tauri API.

**Exit Criteria**
- ✅ `generate_volumetric_points` rewritten to surface-map algorithm (ADR-012).
- ✅ Tauri commands registered: `set_blank_envelope`, `generate_point_cloud`, `export_ply`, `export_xyz`, `export_csv` (+ params/format helpers).
- ✅ `settings.rs` persists blank envelope, volumetric params, and point cloud format.
- ✅ `allow-point-cloud.toml` capability permission group added.
- ✅ Command-level integration tests for all three export commands (QA-B-01).
- ⏳ `export.rs` output not yet verified against engraver import (Phase D).

### Phase C — Frontend UI (1–2 weeks) — **blocked on QA-C-01**
**Goal:** make the full 3D surface point cloud workflow usable from the UI.

**Exit Criteria**
- ✅ Blank envelope controls (L×W×H mm, margin, standard presets) in `ExportPanel.svelte`.
- ✅ Volumetric params panel: XY step, depth threshold, live point count estimate (UI-C-02).
- ✅ Three.js preview renders point cloud inside blank wireframe box.
- ✅ Export panel: format selector (PLY / XYZ / CSV) with save-to-file.
- ⏳ Full workflow UX pass not yet signed off (QA-C-01).

### Phase D — Validate and Harden (1 week) — **in progress (Wave 3–4)**
**Goal:** confirm output is compatible with real engraver toolchains; tighten security boundaries.

**Exit Criteria**
- **SEC-E-01:** ✅ `cargo build` / `cargo test` / clippy green (2026-05-24).
- At least one validated E2E path: image → point cloud → imported into engraver software with correct axis orientation and units.
- Engraver charter documented: ✅ draft at `docs/engraver-charter.md`; target software name/version filled (ARCH-D-01).
- Capability permissions align with minimum-privilege intent; command/permission audit passes (`SEC-D-01`, `ARCH-D-02`).
- **Security:** `expected_model_hashes.json` populated (`SEC-D-02`); max image bytes/dimensions before Python (`SEC-D-03` ✅); point cloud + depth IPC caps (`SEC-D-04`, `SEC-E-02`); preset/mask paths (`SEC-D-05` ✅); sign-off in `SECURITY_SIGNOFF.md`.
- Performance: 4K image → point cloud in < 2 minutes on mid-range hardware (QA-D-02).

---

## Sprint Plan

All sprints are 1 week unless explicitly marked.

### Sprint A — Clean House
**Goal:** delete 2.5D code; make docs and metadata truthful.

**Tasks**
- **BACK-A-01:** ✅ Delete `mesh_generator.rs` and remove all references (`17ae055`).
- **UI-A-01:** ✅ Remove relief-mode UI; `ExportPanel` now exports PLY/XYZ/CSV (`6bbe3da`).
- **DOC-A-01:** ✅ Update `tauri.conf.json` bundle description (`17ae055`).
- **DOC-A-02:** ✅ Update `README.md` — ADR-012 surface-map language (2026-05-19).
- **DOC-A-03:** ✅ `prd.md` header reflects restart and ADR-012 (2026-05-10).
- **DOC-A-04:** ✅ `CLAUDE.md` updated for 3D surface point cloud workflow.
- **DOC-A-05:** ✅ Archive pivot plan to `RESEARCH/archive/` (2026-05-19).
- **ARCH-A-01:** ✅ ADR-012 in `RESEARCH/architecture.md`.

**Gate**
- ✅ Sprint B gate passed — `mesh_generator.rs` deleted; backend wired.

---

### Sprint B — Backend Wiring
**Goal:** rewrite the generation algorithm; wire all volumetric modules into Tauri commands.

**Tasks**
- **BACK-B-01 (TD-14):** ✅ Surface-map algorithm in `volumetric.rs` per ADR-012 (`17ae055`).
- **BACK-B-02:** ✅ Tauri commands registered in `lib.rs` + `allow-point-cloud.toml` (`17ae055`).
- **BACK-B-03:** ✅ `settings.rs` persists `blank_envelope`, `volumetric_params`, `point_cloud_format` (`17ae055`).
- **BACK-B-04:** ✅ Generation uses adjusted depth + mask from pipeline state (`17ae055`).
- **BACK-B-05:** ⏳ Verify `export.rs` PLY/XYZ/CSV output against format specs / engraver import.
- **SEC-B-01:** ✅ Export path validation via `validate_export_path` (inherits `file_io.rs` patterns).
- **QA-B-01:** ✅ Five tests: `export_ply`/`export_xyz`/`export_csv` command paths, generate→export, empty-cache error.

**Exit Criteria**
- ⏳ `cargo test` — blocked on this Linux machine until GTK/WebKit/libsoup dev libs installed (see `SETUP_NOTES.md`).
- ✅ All point cloud commands callable from frontend (`tauri.ts`).
- ⏳ PLY/XYZ/CSV from a real loaded image — manual verification pending (Phase D).

---

### Sprint C — Frontend UI
**Goal:** expose the full 3D workflow through the Svelte UI.

**Tasks**
- **UI-C-01:** ✅ Blank envelope panel in `ExportPanel.svelte` with presets (`6bbe3da`).
- **UI-C-02:** ✅ Volumetric params panel with debounced backend sync and live estimate.
- **UI-C-03:** ✅ `Preview3D.svelte` — point cloud + blank wireframe (`6bbe3da`).
- **UI-C-04:** ✅ `ExportPanel.svelte` — PLY / XYZ / CSV export (`6bbe3da`).
- **UI-C-05:** ✅ Blank envelope and format persisted via backend on export/generate (`6bbe3da`); volumetric params use defaults until UI-C-02.
- **QA-C-01:** ⏳ Manual UX pass not yet recorded.

**Exit Criteria**
- ⏳ Full workflow without terminal — likely achievable; needs manual sign-off (QA-C-01).
- ✅ Preview shows point cloud inside blank wireframe.

---

### Sprint D — Validate and Harden
**Goal:** confirm engraver compatibility; tighten permissions; close findings from the 2026-05-19 security review.

**Sprint package:** `SPRINTS/Sprint_D/` (`SPRINT_D_Task_Assignment.md`, `SECURITY_REVIEW_2026-05.md`, `SECURITY_PENTEST_CHECKLIST.md`).

**Build blocker**
- **SEC-E-01:** ✅ Complete 2026-05-24 — `path_security` imports; 172 tests pass; clippy clean (also fixed `div_ceil` in `volumetric.rs`).

**Engraver & performance**
- **ARCH-D-01:** ✅ Draft charter at `docs/engraver-charter.md` — **remaining:** record target engraver software name/version (§2 TBD).
- **QA-D-01:** Export PLY and XYZ from a real image; import into target engraver software; verify axis orientation and point distribution. **Blocked:** engraver product TBD.
- **BACK-D-01:** Fix any compatibility defects found (axis flips, unit scaling, header metadata, decimal precision).
- **QA-D-02:** Performance check — time from image load to PLY export for a 4K image on a mid-range machine.
- **DOC-D-01:** Publish interoperability notes and known-good engraver configuration in `docs/`.

**Permissions & consistency**
- **SEC-D-01:** Split coarse permissions (`allow-generate-depth-map`) into least-privilege capability groups aligned with actual command groups.
- **SEC-D-06:** Add `allow-settings.toml` and `allow-model.toml`; grant `get_settings`, `save_settings`, `check_model`, `get_model_info`, `download_model` in `capabilities/default.json`.
- **ARCH-D-02:** Re-run command registration/invocation consistency audit (`lib.rs` ↔ `tauri.ts` ↔ capability files).

**Security review remediations (2026-05-19 — owner: Security Specialist + Senior Engineer)**

| Priority | Task | Summary |
|----------|------|---------|
| P0 | **SEC-D-02** | Populate `python/python/expected_model_hashes.json` after trusted download; fail `download_model` when hashes are configured and verification fails (no silent skip in release). |
| P0 | **SEC-D-03** | Enforce max image file size (e.g. 64–128 MB) and dimension cap on `read_image_bytes_for_depth` / before Python subprocess (align with 8K policy). |
| P1 | **SEC-D-04** | Cap IPC payloads: point cloud preview capped ✅; **remaining:** depth map via `get_depth_map` (**SEC-E-02**); document limits in `docs/threat-model.md` §2.6. |
| P0 | **SEC-E-01** | ✅ Fix `path_security` imports in `lib.rs` (2026-05-24). |
| P1 | **SEC-E-02** | Cap or tile `get_depth_map` / depth IPC responses at 8K (subsampling or max dimension policy). |
| P1 | **SEC-D-05** | Restrict `load_preset` (absolute path) and `load_mask_from_path` to dialog-selected paths or app data dirs; apply export-style blocklist. |
| P2 | **SEC-D-07** | Release builds: `Command::env_clear()` + allowlist for Python child (`PATH`, `HOME` only as needed); ignore `VIRTUAL_ENV` / strip `DEPTH_MODEL_PATH` unless app-controlled. |
| P2 | **SEC-D-08** | Validate `DepthAdjustmentParams` on `set_depth_adjustment_params` (gamma/contrast ranges, max curve points, `feather_radius_px` cap). |
| P2 | **SEC-D-09** | Harden export writes: canonicalize full target path; atomic write (temp + rename) or `O_NOFOLLOW` where available (extends TD-06). |
| P2 | **SEC-D-10** | Align Linux `load_image` blocklist with export blocklist (`/bin`, `/sbin`, `/proc`, `/sys`, etc.). |
| P3 | **SEC-D-11** | Set `withGlobalTauri: false` in release; disable devtools in production bundle; tighten CSP if needed. |
| P3 | **QA-D-03** | Execute manual pentest checklist (`SPRINTS/Sprint_D/SECURITY_PENTEST_CHECKLIST.md`); record in `MANUAL_TEST_REPORT.md`. |
| — | **SEC-D-12** | Sanitize PLY/CSV comment fields from `source_image` filename (newline/control chars). |
| — | **DOC-D-02** | Update `docs/threat-model.md` and `docs/security-checklist.md` with Sprint D outcomes; Security Specialist sign-off in `SECURITY_SIGNOFF.md`. |

**QA & CI (from 2026-05-24 QE review)**

| Priority | Task | Summary |
|----------|------|---------|
| P0 | **QA-B-01** | ✅ Tauri export command integration tests in `lib.rs` (2026-05-24) |
| P0 | **QA-C-01** | Execute `docs/testing.md` §1–2; record in `SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md` |
| P0 | **CI-D-01** | ✅ `backend-windows` job — build, test, clippy on `windows-latest` |
| P1 | **QA-D-04** | Measure tarpaulin coverage; raise CI `--fail-under` from 63% if ≥65% achieved |
| P1 | **QA-D-05** | ✅ Vitest: `exportXyz`, `exportCsv`, `setPointCloudFormat` (2026-05-24; 78 frontend tests) |
| P1 | **QA-D-07** | Create `SPRINTS/Sprint_D/TEST_PLAN_D.md` from `SPRINTS/TEST_PLAN_TEMPLATE.md` |
| P2 | **QA-D-06** | Component smoke tests: `ExportPanel`, `VolumetricParamsPanel` (`@testing-library/svelte` ^4.x) |
| P2 | **QA-D-09** | GOTCHAS: GTK local test setup; TODO-hash skip behaviour; `lib.rs` command test pattern |
| P3 | **QA-D-08** | `SPRINTS/Sprint_2_6/SPRINT_2_6_CANCELLED.md` — document 2026-05-10 restart supersession |

**Exit Criteria**
- At least one validated real-world engraver path documented with repeatable settings.
- Capability model aligns with minimum-privilege intent; all registered commands appear in ACL (`SEC-D-01`, `SEC-D-06`, `ARCH-D-02`).
- **SEC-D-02** and **SEC-D-03** complete (P0); **SEC-D-04** and **SEC-D-05** complete or explicitly risk-accepted.
- Performance target met or blocker documented with mitigation plan.
- `SPRINTS/Sprint_D/SECURITY_SIGNOFF.md` signed for Sprint D scope.

---

## Deferred Tracks (Explicit)

- **Full 3D reconstruction (TripoSR)** — deferred until after Sprint D validation. Documented in `RESEARCH/3d-reconstruction.md`.
- **Major UI polish** (dark mode, broad beta polish) — deferred until volumetric flow is engraver-validated.
- **Cross-platform rollout** (macOS, Linux packaging) — Phase 3, after Phase D stability confirmed on primary platform.

---

## Active Technical Debt Register

| ID | Title | Severity | Plan |
|---|---|---|---|
| TD-14 | ~~Column-sweep algorithm~~ — **resolved**; surface-map (ADR-012) shipped in `17ae055` | ~~Critical~~ | Closed |
| TD-11 | ~~Volumetric modules not wired~~ — **resolved** (backend + UI + params panel) | ~~Critical~~ | Closed |
| TD-19 | ~~Settings/model ACL~~ — **resolved** (`allow-settings`, `allow-model` in default capability) | ~~Medium~~ | Closed |
| TD-22 | ~~`lib.rs` missing `path_security` imports~~ — **resolved** 2026-05-24 | ~~Critical~~ | Closed |
| TD-13 | Command-permission granularity inconsistent | High | Sprint D — SEC-D-01 |
| TD-15 | Model SHA256 hashes empty — verification skipped on download | High | Sprint D — SEC-D-02 |
| TD-16 | IPC DoS: depth map still unbounded over Tauri; point cloud capped | High | SEC-E-02 + SEC-D-04 |
| TD-17 | ~~`load_preset` path policy~~ — active via `validate_import_file_path` | ~~Medium~~ | Closed (SEC-D-05) |
| TD-18 | ~~`load_mask_from_path` blocklist~~ — active via `path_security` | ~~Medium~~ | Closed (SEC-D-05) |
| TD-20 | Python subprocess inherits `VIRTUAL_ENV` / `DEPTH_MODEL_PATH` | Medium | Sprint D — SEC-D-07 |
| TD-08 | `lib.rs` coverage gap beyond export commands | High | QA-D-04 |
| TD-06 | Export path TOCTOU write-test pattern | Medium | Sprint D — SEC-D-09 |
| TD-21 | `withGlobalTauri: true` widens webview compromise blast radius | Low | Sprint D — SEC-D-11 |
| TD-23 | ~~No Windows CI~~ — **resolved** `backend-windows` job | ~~High~~ | Closed |
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

- This file is the operational source of truth for **task IDs and sprint gates**. Vision and phases: **`ROADMAP.md`**.
- **Do not mark Phase B complete** until a real-image PLY/XYZ export is verified on target engraver (QA-D-01 / Phase D).
- **Do not mark Phase C complete** until QA-C-01 (manual UX pass) is done.
- **Do not mark Phase D complete** until at least one engraver import is validated (QA-D-01) and P0 security tasks **SEC-D-02** / **SEC-D-03** are complete (SEC-D-03 ✅; SEC-D-02 requires real hashes) or risk-accepted in `SPRINTS/Sprint_D/SECURITY_SIGNOFF.md`.
- **2026-05-24:** Reconcile `SPRINTS/Sprint_D/ORCHESTRATION.md` “Complete” statuses with `cargo build` and `SECURITY_SIGNOFF.md` before claiming Wave 2 done.
- If scope changes, update this file and `prd.md` in the same change set to prevent drift.
- **Linux dev setup on this machine (2026-05-18):** `npm`, Python venv/pytest, and Tauri GTK/WebKit/libsoup dev libs not installed — run `SETUP_NOTES.md` steps before local test/build.
