# Architect Review — SimplePicture3D Takeover

**Date:** 2026-05-19  
**Reviewer:** System Architect (new team)  
**Scope:** Full repository assessment post 2026-05-10 restart  
**Companion:** `ROADMAP.md` (production path forward)

---

## Executive summary

SimplePicture3D is a **Tauri 2 + Svelte 4 + Rust + Python** desktop app that converts 2D images into **3D surface point clouds** for internal UV laser engraving. The prior team completed a deliberate pivot from 2.5D relief meshes (STL/OBJ) to the **ADR-012 surface-map algorithm** and wired a functional end-to-end pipeline.

**Assessment:** The project is **~75% of the way to a shippable v1.0**. Core engineering risk has shifted from architecture to **operational validation** (engraver compatibility), **security hardening**, and **release polish**. The largest liability is not code quality in the hot path — it is **documentation drift** and **unvalidated export assumptions**.

**Recommendation:** Freeze feature scope. Execute Phase D (validate + harden), then Phase E (v1.0). Do not restart algorithm work or add TripoSR until engraver validation passes.

---

## Repository inventory

| Layer | Location | Maturity |
|-------|----------|----------|
| Frontend | `src/` — 13 Svelte components, Vitest | **Strong** — depth UX complete; preview naming stale |
| Rust backend | `src-tauri/src/` — 13 modules | **Strong** — ADR-012 implemented; legacy settings fields |
| Python AI | `python/python/depth_estimator.py` | **Solid** — stub mode for CI; contract stable |
| Docs / research | `RESEARCH/`, `docs/`, `SPRINTS/` | **Mixed** — extensive but partially obsolete |
| CI | `.github/workflows/ci.yml` | **Good** — Rust + frontend + Python + audit |
| Agent tooling | `.agents/`, `.cursor/` | **Useful** — some personas still reference mesh/STL |

**298 tracked files** including 124+ sprint artefacts. Pre-restart sprints (1.1–2.6) are valuable history; they must not be read as current requirements.

---

## Architecture assessment

### Strengths

1. **Clear IPC boundary** — Business logic in Rust; UI in Svelte; Python stateless subprocess. Matches ADR-002.
2. **ADR-012 correctly implemented** — `volumetric.rs::generate_volumetric_points` places one point per sampled (x,y) with depth-mapped Z and `fit_to_blank`.
3. **Depth pipeline is production-grade** — Adjustments, curves, mask, undo/redo, presets, progress events.
4. **Export writers tested** — PLY (ASCII/binary), XYZ, CSV with unit tests.
5. **Offline-first** — No network calls in processing path; model download is explicit user action.
6. **Test culture** — Broad Rust unit tests, frontend IPC contract tests, Python stub tests.

### Weaknesses

1. **Engraver validation gap** — No recorded end-to-end import into target engraver software. This is the #1 production blocker.
2. **IPC scale risk** — `generate_point_cloud` returns full point arrays. At 4K full step, this exceeds practical memory/latency budgets.
3. **Permission model incomplete** — Settings and model commands lack dedicated ACL groups (TD-19).
4. **Security review open items** — Empty model hashes, unbounded payloads, arbitrary preset path reads (Sprint D).
5. **Naming confusion** — `volumetric` module name contradicts ADR-012 "surface-map" terminology.
6. **Documentation schizophrenia** — README, `docs/architecture.md`, `docs/testing.md`, and parts of `prd.md` still describe mesh/STL workflows.

### Retired (confirmed removed)

- `mesh_generator.rs` — deleted
- `get_mesh_data`, `export_stl`, `export_obj` — removed from IPC
- STL/OBJ permission TOML — removed
- Column-sweep volumetric fill — superseded by ADR-012

---

## Code quality snapshot

### Rust (`src-tauri/`)

- **36 Tauri commands** registered; frontend invokes ~30.
- Error handling uses `anyhow`/`Result` patterns; no egregious `unwrap()` in production paths reviewed.
- `generate_point_cloud` caches in `AppState.last_point_cloud`; exports require prior generation.
- Legacy fields in `settings.rs`: `export_format` (stl/obj), `target_width_mm/height_mm` (mesh era).
- Generated `gen/schemas/windows-schema.json` still references `allow-export-stl` — stale artifact.

### Frontend (`src/`)

- Single-page app; no routing. Workflow: load → depth → adjust → preview → export.
- `Preview3D.svelte` uses `THREE.Points` but retains `mesh*` variable names.
- `VolumetricParamsPanel` provides step/threshold with live count estimate.
- No auto-refresh of 3D preview on depth changes.

### Python (`python/`)

- CLI contract stable: JSON stdout, PROGRESS/STAGE on stderr.
- Stub mode (`SP3D_USE_STUB=1`) enables CI without model download.
- `expected_model_hashes.json` empty — verification skipped.

---

## Documentation audit

| Document | Status | Action taken / planned |
|----------|--------|------------------------|
| `ROADMAP.md` | **New** | Production vision and phases |
| `todo.md` | Current | Updated contradictions; points to ROADMAP |
| `README.md` | Stale | Updated for ADR-012 surface-map |
| `CLAUDE.md` | Stale | Updated status and references |
| `prd.md` | Partial | Header current; body retains mesh-era content (trim in Phase E) |
| `RESEARCH/architecture.md` | Stale banner | Updated pivot notice → ADR-012 canonical |
| `RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md` | Obsolete | **Archived** to `RESEARCH/archive/` |
| `docs/architecture.md` | Stale | Updated overview and data flow |
| `docs/testing.md` | Stale | Updated core workflow for point cloud export |
| `docs/developer-guide.md` | Stale | Phase E — update command table |
| `RESEARCH/rust-crates.md` | Stale | Phase E — remove mesh_generator references |
| Sprint 1.x–2.x folders | Historical | Retained as archive; not deleted |

---

## Test and CI assessment

| Suite | Status | Notes |
|-------|--------|-------|
| Frontend Vitest | 74/74 pass (2026-05-19) | No Preview3D/ExportPanel component tests |
| Rust unit tests | ~169 in source | 6 `#[ignore]` need Python |
| Python pytest | CI-ready | Local run needs venv + pytest install |
| Tauri build (Linux) | Blocked locally | Needs GTK/WebKit/libsoup dev packages |
| CI tarpaulin | 63% fail-under | Adequate for MVP; `lib.rs` coverage gap noted |

**Gap:** QA-B-01 export integration tests not yet recorded. QA-C-01 manual UX pass pending.

---

## Security posture (Sprint D summary)

Findings from `SPRINTS/Sprint_D/SECURITY_REVIEW_2026-05.md`:

| Severity | Finding |
|----------|---------|
| P0 | Model SHA256 table empty — downloads not verified |
| P0 | No max image bytes/dimensions before Python spawn |
| P1 | Unbounded IPC payloads (depth maps, point clouds) |
| P1 | `load_preset` accepts arbitrary absolute paths |
| P1 | Settings/model commands not in capability ACL |
| P2 | Python subprocess inherits full environment |
| P3 | `withGlobalTauri: true` in dev config |

**Verdict:** Acceptable for trusted local dev; **not acceptable for public beta** until P0/P1 addressed.

---

## Legacy cleanup performed (2026-05-19)

1. Archived `RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md` → `RESEARCH/archive/`
2. Created `ROADMAP.md` — canonical production path
3. Updated README, CLAUDE.md, architecture pivot banners
4. Updated `docs/testing.md` and `docs/architecture.md` overview sections
5. Corrected stale comments in `settings.rs`, `allow-generate-depth-map.toml`
6. Fixed internal contradictions in `todo.md`
7. Updated `.cursor/rules/junior-3d.mdc` persona scope

**Not changed (deferred to Phase E):**

- Rename `volumetric.rs` → `surface_map.rs` (breaking API change; needs coordinated frontend/Rust rename)
- Remove `export_format` from settings schema (needs migration for existing user settings files)
- Trim 1,300-line `prd.md` body (large editorial pass)
- Delete historical `SPRINTS/Sprint_1_*` folders (retained as project history)

---

## Production readiness scorecard

| Criterion | Score | Notes |
|-----------|-------|-------|
| Core algorithm | ✅ Ready | ADR-012 implemented and tested |
| Backend API | ✅ Ready | Commands wired and permissioned |
| Frontend workflow | ✅ Ready | Minor UX gaps |
| Export formats | ⚠️ Unvalidated | Writers exist; engraver import unproven |
| Security | ❌ Not ready | Sprint D P0 open |
| Performance | ⚠️ Unknown | 4K benchmark not recorded |
| Documentation | ⚠️ Improving | This review + ROADMAP address drift |
| Cross-platform | ❌ Not started | Phase F after Windows v1.0 |
| Installers | ❌ Not started | Phase E |

**Overall:** Not production-ready today. **Achievable in ~5–7 weeks** with focused Phase D + E execution.

---

## Recommended immediate actions

1. **Install Linux dev deps** per `SETUP_NOTES.md`; confirm `cargo test` and `npm run tauri dev` locally.
2. **Execute QA-D-01** — export PLY/XYZ from real image; import into target engraver; document axes/units.
3. **Close SEC-D-02 and SEC-D-03** — model hashes and image size caps.
4. **Add export integration tests** (QA-B-01).
5. **Implement preview decimation** before beta — cap points returned over IPC.
6. **Do not add features** until Phase D exit criteria met.

---

## Conclusion

The prior team delivered a **coherent architectural restart** and a working surface-map pipeline. The new team's job is not to redesign — it is to **validate, harden, and ship**. The path is documented in `ROADMAP.md`; operational tasks remain in `todo.md`.

The project's compelling story for users is simple: *"Your photo becomes a crystal engraving — offline, in minutes."* The engineering story is equally clear: *"We're one validation sprint away from beta, one polish sprint away from v1.0."*
