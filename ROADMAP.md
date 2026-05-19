# SimplePicture3D — Production Roadmap

**Version:** 1.0  
**Date:** 2026-05-19  
**Author:** System Architect (takeover review)  
**Status:** Active — supersedes README roadmap checkboxes and pre-restart sprint sequencing

---

## Vision

**SimplePicture3D turns any photograph into a laser-ready 3D point cloud inside a crystal blank — entirely offline, in minutes, with no 3D modeling skills.**

The product serves hobbyists and small engraving shops who internal UV laser engrave K9 crystal, glass, and acrylic. The user loads a 2D image, defines their blank dimensions, adjusts depth, previews the result in 3D, and exports PLY/XYZ/CSV for their engraver software.

### What we are building

| In scope (v1.0) | Out of scope (v1.0) |
|-----------------|---------------------|
| AI depth from 2D image (Depth-Anything-V2 / MiDaS) | Full 3D reconstruction (TripoSR) |
| Surface-map point cloud (ADR-012): one dot per (x,y) at mapped Z | 2.5D relief meshes, STL/OBJ export |
| Crystal blank envelope + fit-to-blank scaling | Cloud sync, accounts, telemetry |
| PLY / XYZ / CSV export | Mobile or web targets |
| Three.js preview (point cloud + blank wireframe) | Commercial model distribution without license review |
| Depth tools: curves, mask, undo/redo, presets | Dark mode / broad UI polish (post-v1.0) |
| 100% offline processing | Bundled Python runtime (ADR-003: system Python for MVP) |

### North-star outcome

A maker can install SimplePicture3D, load a portrait, set an 80×50×50 mm blank, export PLY, import into their engraver, and produce a recognizable internal engraving — **without reading architecture docs or filing a bug report.**

---

## Where we are today

The **2026-05-10 restart** replaced the 2.5D mesh pipeline with ADR-012 surface-map point clouds. Phases A–C are **substantially complete**. The codebase is no longer blocked on algorithm design; it is blocked on **validation, hardening, and release engineering**.

```
┌─────────────────────────────────────────────────────────────────┐
│                        PRODUCTION PATH                           │
├─────────────────────────────────────────────────────────────────┤
│  ✅ Foundation     Image load, depth AI, adjustments, mask       │
│  ✅ Algorithm      ADR-012 surface-map in volumetric.rs          │
│  ✅ Backend API    36 Tauri commands; PLY/XYZ/CSV export         │
│  ✅ Frontend       Blank setup, params, preview, export UI       │
│  ⏳ Validation     Real engraver import not yet signed off       │
│  ⏳ Security       Sprint D remediations (P0–P3)                   │
│  ⏳ Performance    Large IPC payloads; 4K timing not benchmarked │
│  ⬜ Release        Installers, docs, cross-platform packaging    │
└─────────────────────────────────────────────────────────────────┘
```

**Test health (last known):** 74 frontend Vitest tests pass; ~169 Rust unit tests in source; Python pytest designed for CI stub mode. Local Linux Tauri builds require GTK/WebKit dev libs (`SETUP_NOTES.md`).

---

## Architecture (as-built)

```
Svelte 4 UI ──IPC──► Rust (Tauri 2)
                         │
                         ├─ image_loading, depth_adjust, mask, undo, preset, settings
                         ├─ blank_envelope + volumetric (ADR-012 surface-map)
                         ├─ export (PLY / XYZ / CSV)
                         └─ python_bridge ──subprocess──► depth_estimator.py
```

**Canonical algorithm:** `RESEARCH/architecture.md` **ADR-012** — one laser dot per sampled pixel column at `z = margin + (1 − depth) × interior_height`.

**Naming debt:** The module is still called `volumetric.rs` and commands use `VolumetricParams`. Behavior matches ADR-012 surface-map, not the superseded ADR-011 column-sweep fill. Rename is scheduled for Phase E cleanup (non-blocking).

---

## Roadmap phases

### Phase D — Validate & Harden (current, ~2 weeks)

**Goal:** Prove the product works with real engraver software and close security gaps that block a public beta.

| Track | Key deliverables | Exit signal |
|-------|------------------|-------------|
| **Engraver validation** | Engraver charter (axes, units, formats); import PLY/XYZ into ≥1 target app; fix axis/scale bugs | QA-D-01 pass recorded in `SPRINTS/Sprint_D/` |
| **Security P0** | Populate model SHA256 hashes; enforce max image size before Python subprocess | SEC-D-02, SEC-D-03 complete |
| **Security P1** | IPC payload caps; preset/mask path restrictions; Tauri ACL for settings/model commands | SEC-D-04, SEC-D-05, SEC-D-06 |
| **Integration tests** | Export command tests with temp files (QA-B-01) | `cargo test` green in CI |
| **Manual QA** | Full workflow UX pass without terminal (QA-C-01) | Sign-off in Sprint D manual report |

**Do not ship a beta until Phase D P0 items are done or explicitly risk-accepted in `SECURITY_SIGNOFF.md`.**

---

### Phase E — Production MVP (v1.0, ~3–4 weeks)

**Goal:** A trustworthy v1.0 on the primary platform (Windows) that a stranger can install and use.

| Workstream | Tasks |
|------------|-------|
| **Performance** | Preview decimation (`preview_step` or max points cap); avoid returning multi-million-point arrays over IPC; benchmark 4K → export < 2 min |
| **UX polish** | Auto-refresh or stale-preview warning when depth/blank/params change; rename Preview3D `mesh*` identifiers; wire or remove `clearHistory` |
| **Settings cleanup** | Deprecate unused `export_format` (stl/obj); document `target_width_mm` as depth-preview zoom only; map preset `step_x/y` to `VolumetricParams` on load |
| **Documentation** | User guide: install → first engraving; engraver compatibility matrix; update `docs/testing.md` for point cloud workflow |
| **Release engineering** | Versioning, CHANGELOG, signed Windows installer; `withGlobalTauri: false` in release; devtools disabled |
| **Remaining security** | SEC-D-07 through SEC-D-12 (subprocess env, param bounds, atomic export, Linux blocklist parity) |

**v1.0 definition of done:**

1. Fresh install → load image → depth → preview → export PLY → **confirmed import in documented engraver**
2. All CI jobs green (Rust clippy/tests/coverage, frontend, Python stub pytest, audit)
3. P0/P1 security items closed or risk-accepted
4. User-facing docs cover install, first run, model download, export, and known engraver configs
5. No active references to STL/OBJ/mesh in user-facing docs or primary onboarding paths

---

### Phase F — Cross-platform & distribution (Phase 3, ~4–6 weeks)

**Goal:** macOS and Linux builds with equivalent core workflow.

| Platform | Deliverable |
|----------|-------------|
| **macOS** | Universal binary; code signing; notarization (`PENDING_APPLE_CHANGES.md`) |
| **Linux** | AppImage + .deb; GTK3/WebKit dependency documented |
| **All** | First-run model download verified on each OS; platform guards tested |

Cross-platform work stays gated until Windows v1.0 is engraver-validated.

---

### Phase G — Growth (post-v1.0, optional)

Deferred until core loop is stable:

- TripoSR or other full 3D reconstruction (`RESEARCH/3d-reconstruction.md`)
- Automated E2E (Playwright)
- Accessibility pass (keyboard, screen reader)
- Opt-in crash reporting (design review required)
- Commercial distribution channel with MiDaS as default model (Depth-Anything-V2 is CC-BY-NC-4.0)

---

## Sprint sequencing (operational)

Detailed task IDs live in **`todo.md`**. High-level sequence:

```
[D] Validate & Harden     ← YOU ARE HERE
    ↓
[E1] Performance + IPC limits
[E2] UX polish + settings cleanup
[E3] Docs + release engineering
    ↓
[F] macOS / Linux packaging
    ↓
[G] Optional growth features
```

Each sprint uses `SPRINTS/SPRINT_TASKING_TEMPLATE.md` and stores artefacts under `SPRINTS/Sprint_<id>/`.

---

## Technical debt priorities

| Priority | Item | Phase |
|----------|------|-------|
| P0 | Engraver validation undocumented | D |
| P0 | Model hash verification empty | D |
| P0 | No max image size before Python | D |
| P1 | Full point cloud over IPC at 4K resolution | D/E |
| P1 | Settings/model commands missing ACL | D |
| P1 | `load_preset` arbitrary path read | D |
| P2 | `volumetric.rs` naming vs ADR-012 | E |
| P2 | Legacy `export_format`, preset mesh fields | E |
| P2 | `lib.rs` testability / coverage gap | E |
| P3 | Accessibility in advanced controls | G |

Full register: `todo.md` § Active Technical Debt Register.

---

## Documentation map

| Document | Role |
|----------|------|
| **`ROADMAP.md`** (this file) | Vision, phases, production path |
| **`todo.md`** | Task IDs, sprint gates, debt register |
| **`prd.md`** | Product requirements (header current; body being trimmed over time) |
| **`RESEARCH/architecture.md`** | ADRs; **ADR-012 is canonical** |
| **`docs/ARCHITECT_REVIEW_2026-05.md`** | Takeover assessment and findings |
| **`SPRINTS/Sprint_D/`** | Active validation and security package |
| **`RESEARCH/archive/`** | Historical docs (e.g. pivot plan) |

---

## Risks and mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Engraver axis convention mismatch | Exported files unusable | Phase D charter + fix before v1.0 |
| 4K IPC memory blow-up | OOM / UI freeze | Preview decimation + export-from-cache only |
| Python env variance | Depth fails on user machines | First-run wizard; clear SETUP_NOTES; stub mode for CI |
| CC-BY-NC model in commercial use | License violation | MiDaS default for commercial; NC warning in UI |
| Doc drift confuses contributors | Wrong features built | ROADMAP + todo as sources of truth; archive stale docs |

---

## Summary for stakeholders

SimplePicture3D has **crossed the hard architectural inflection point**. The 2.5D mesh era is over; the surface-map point cloud pipeline is implemented end-to-end. The path to production is no longer "invent the algorithm" — it is **validate with real engravers, harden security, optimize large-image performance, and ship a polished Windows v1.0**.

**Next action:** Execute Sprint D (`SPRINTS/Sprint_D/SPRINT_D_Task_Assignment.md`) — engraver validation and P0 security — then enter Phase E for v1.0 release.
