# SimplePicture3D v0.1.0-beta.1 — Release Notes (Draft)

**Release date:** TBD  
**Type:** Phase 1 MVP — First beta

---

## Summary

SimplePicture3D **v0.1.0-beta.1** is the first beta release of our desktop app that converts 2D images into 2.5D STL/OBJ meshes for internal UV laser engraving of K9 crystal, glass, and acrylic. This release delivers the full Phase 1 MVP pipeline: load image → AI depth estimation → manual adjustments → 3D preview → export STL or OBJ.

We need your feedback to improve stability and UX before the Phase 1 exit (installer and broader release).

---

## Features

- **Image loading** — PNG, JPG; drag-and-drop; automatic downsampling for images over 8192×8192.
- **AI depth estimation** — Depth-Anything-V2 (default) or MiDaS via local Python; 100% offline.
- **Depth controls** — Brightness, contrast, gamma, invert; min/max depth range (mm).
- **3D preview** — Interactive mesh preview with Three.js.
- **Export** — Binary STL and ASCII OBJ with optional target dimensions (mm).
- **Settings** — Persisted depth range, target size, last export directory, window size.
- **Model wizard** — First-run check and one-click download of the default depth model.

---

## Installation

- **From source (recommended for beta):** See [Developer Guide](developer-guide.md) and [README](../README.md#-quick-start) for build instructions (Rust, Node.js, Python 3.10+).
- **Installers:** Planned for Phase 1 exit; not yet available.

---

## Known issues

- Windows is the primary supported platform; macOS and Linux need more testing.
- First run requires a one-time model download (large download; ensure stable connection).
- Depth-Anything-V2 weights are **CC-BY-NC-4.0** (non-commercial use only). See [architecture](../RESEARCH/architecture.md) ADR-005 for licensing.

---

## For beta testers

- **Getting started:** Read the [Beta onboarding guide](beta-onboarding.md).
- **User workflow:** See [User guide](user-guide.md) for step-by-step first conversion.
- **Reporting bugs:** Use the [GitHub Issues bug report template](https://github.com/BelongaGezza/SimplePicture3D/issues/new?template=bug_report.md). Include OS, version, and steps to reproduce.
- **Feedback:** [GitHub Discussions](https://github.com/BelongaGezza/SimplePicture3D/discussions) for questions and suggestions.

---

## Full changelog

See [CHANGELOG.md](../CHANGELOG.md) for the full list of features and known limitations.

---

# SimplePicture3D v0.2.0-beta.1 — Release Notes (Draft)

**Release date:** TBD  
**Type:** Phase 2 — Enhanced UX (Sprint 2.2)

---

## Summary

This release adds **undo/redo** for depth adjustments, **persisted curve control points**, and UX polish: keyboard shortcuts, Wireframe/Solid behaviour fixes, and removal of internal sprint references from the UI. It builds on the Phase 1 MVP (v0.1.0-beta.1) and Sprint 2.1 (curves, histogram, target dimensions).

**Highlights:**

- **Undo and redo** — Last 20 depth-adjustment actions (sliders, curve, invert, range); toolbar buttons and Ctrl+Z / Ctrl+Y (or Cmd+Z / Cmd+Shift+Z on macOS).
- **Curve persistence** — Your tone-curve control points are saved in settings and restored when you reopen the app or load a new image.
- **Keyboard shortcuts** — Undo (Ctrl+Z), Redo (Ctrl+Y or Ctrl+Shift+Z); documented in the [User guide](user-guide.md#undo-and-redo-phase-2).
- **Wireframe/Solid** — Wireframe and Solid 3D view modes now work correctly with the triangulated mesh, or the controls have been removed/hidden until supported (no dead UI).
- **Clean UI** — Internal sprint labels (e.g. “Sprint 1.8”) removed from user-facing messages.

---

## Features

### Undo / Redo (F2.4)

- Toolbar **Undo** and **Redo** buttons; disabled when there is nothing to undo or redo.
- **Ctrl+Z** (undo) and **Ctrl+Y** or **Ctrl+Shift+Z** (redo) on Windows/Linux; **Cmd+Z** and **Cmd+Shift+Z** on macOS.
- History limited to the **last 20 actions**; oldest entries are dropped when the limit is reached.
- Applies to: brightness, contrast, gamma, invert, depth range (min/max), and curve control points.
- History is cleared when you load a new image or generate a new depth map.

### Curve persistence (Consultant §2.6)

- **Curve control points** are stored in app settings and restored on next launch (or when loading a new image, per product behaviour).
- Curves tool shows your last saved curve so you don’t lose your preferred tone mapping.

### 3D preview and UI

- **Wireframe** and **Solid** view modes either work correctly with mesh indices or have been removed/hidden with clear UX (technical debt TD-02).
- **Internal sprint references** (e.g. “Sprint 1.8”) removed from user-visible overlays and messages.

### Documentation and architecture

- **State management ADR** (TD-01) documented in [RESEARCH/architecture.md](../RESEARCH/architecture.md) (ADR-010): Svelte store vs backend sync, single source of truth for depth params and undo history.
- **User guide** updated with [Undo and redo (Phase 2)](user-guide.md#undo-and-redo-phase-2) and [curve persistence](user-guide.md#curve-persistence).
- **Developer guide** updated with `undo`, `redo`, `clear_history` Tauri commands and links to ADR-009 (undo/redo design), ADR-010 (state management).

---

## Installation

- Same as v0.1.0-beta.1: build from source (see [Developer Guide](developer-guide.md)); installers TBD for a later phase.
- **Upgrade:** If you already have settings from a previous version, curve control points will be saved and loaded once you use the Curves tool in this release.

---

## Known issues

- Same as v0.1.0-beta.1 where applicable (Windows primary; model download; Depth-Anything-V2 non-commercial license).
- macOS smoke testing is documented but may not have been run in this sprint (see TD-05).

---

## For testers and contributors

- **User workflow:** [User guide](user-guide.md) — first conversion, depth controls, **undo/redo**, curve persistence.
- **Build and commands:** [Developer guide](developer-guide.md) — Tauri commands table includes `undo`, `redo`, `clear_history`.
- **Architecture:** [RESEARCH/architecture.md](../RESEARCH/architecture.md) — ADR-009 (undo/redo), ADR-010 (state management).

---

## Full changelog

See [CHANGELOG.md](../CHANGELOG.md) for the full list of changes (Sprint 2.2 entry to be added at release).

---

*This draft is for the Phase 2 Sprint 2.2 release. Paste into the GitHub Release description when the v0.2.0-beta.1 tag is cut; update version, date, and “Wireframe/Solid” wording to match as-shipped behaviour.*

---

# SimplePicture3D v0.3.0-beta.1 — Release Notes (Draft)

**Release date:** TBD
**Type:** Phase 2 — Presets & Templates (Sprint 2.3)

---

## Summary

This release adds **Presets & Templates (F2.3)**: users can save, apply, import, export, rename, and delete depth-processing configurations. Four built-in presets (Portrait, Landscape, High Detail, Low Relief) are included out of the box. It builds on Phase 1 MVP (v0.1.0-beta.1) and Phase 2 Sprints 2.1–2.2 (curves, histogram, undo/redo).

**Highlights:**

- **Built-in presets** — Portrait, Landscape, High Detail, Low Relief; apply from the sidebar dropdown.
- **Save and apply user presets** — Save any depth configuration by name; reapply with one click.
- **Import / Export** — Share preset `.json` files with other users or keep backups via the file system.
- **Preset manager** — Rename and delete saved presets from the sidebar.
- **Safe naming** — Preset names are sanitized (no path traversal); files stored under `~/.simplepicture3d/presets/`.

---

## Features

### Presets & Templates (F2.3)

- **Four built-in presets** registered in the backend (`preset.rs`): Portrait (S-curve, slight brightness lift), Landscape (wider depth range 2–12 mm), High Detail (increased contrast/gamma), Low Relief (2–6 mm shallow engraving).
- **`save_preset(name, path?)`** — Saves current `DepthAdjustmentParams` + curve + mesh params as JSON to the user presets directory or an explicit path (export).
- **`load_preset(nameOrPath)`** — Applies a preset by name (built-in or user) or by absolute path (import); updates app depth state and undo history.
- **`list_presets()` / `list_builtin_presets()`** — Backend returns separate lists; frontend merges into a unified `PresetListItem[]` (built-ins first).
- **`delete_preset(name)` / `rename_preset(oldName, newName)`** — Manage user presets; names sanitized to prevent path traversal.
- **PresetManager UI** — Sidebar panel with list, inline rename form, delete confirmation, and Import/Export buttons.
- **Preset schema** version 1 (`schemaVersion` field) for forward-compatible migration.

---

## Installation

- Same as v0.2.0-beta.1: build from source (see [Developer Guide](developer-guide.md)); installers TBD for a later phase.
- **Upgrade:** Existing settings from earlier versions are unaffected; preset files are new and stored separately under `~/.simplepicture3d/presets/`.

---

## Known issues

- Same as v0.2.0-beta.1 where applicable (Windows primary; model download; Depth-Anything-V2 non-commercial license).
- JR2-1301–1303 (automated preset round-trip tests) and QA-1301–1303 (manual QA) not yet completed at sprint close; queued for Sprint 2.4 gate.

---

## For testers and contributors

- **User workflow:** [User guide](user-guide.md) — new **Presets (Phase 2)** section.
- **Manual testing:** [Testing guide](testing.md) — new **Presets** section (§4, 7 test items).
- **Build and commands:** [Developer guide](developer-guide.md) — Tauri commands table includes `save_preset`, `load_preset`, `list_presets`, `list_builtin_presets`, `delete_preset`, `rename_preset`.
- **Architecture:** [RESEARCH/architecture.md](../RESEARCH/architecture.md) — Preset schema, ADR for presets.

---

## Full changelog

See [CHANGELOG.md](../CHANGELOG.md) for the full list of changes (Sprint 2.3 entry to be added at release).

---

*This draft is for the Phase 2 Sprint 2.3 release. Paste into the GitHub Release description when the v0.3.0-beta.1 tag is cut; update version and date before tagging.*

---

# SimplePicture3D v0.4.0-beta.1 — Release Notes (Draft)

**Release date:** TBD  
**Type:** Phase 2 — Progress Streaming for Depth Estimation (Sprint 2.4)

---

## Summary

This release closes the **5-minute UX gap**: depth estimation now streams **real-time progress** from the Python backend to the UI. Instead of an indeterminate spinner, users see a **percentage progress bar** (0–100%) that updates during inference. It builds on Phase 1 MVP and Phase 2 Sprints 2.1–2.3 (curves, undo/redo, presets).

**Highlights:**

- **Real-time progress bar** — During "Generate depth map", the UI shows a determinate 0–100% bar driven by `depth-progress` Tauri events.
- **Backend streaming** — Python emits `PROGRESS n` and `STAGE name` to stderr; Rust bridge forwards these as `depth-progress` events; frontend subscribes and unlistens on completion.
- **No silent wait** — Especially for 4K images, users see progress advance instead of a long indeterminate wait.

---

## Features

### Progress streaming (Sprint 2.4, BACK-205, UI-304)

- **Tauri event** `depth-progress` with payload `{ percent: number, stage?: string }` (ARCH-501).
- **Backend:** `generate_depth_map` accepts `AppHandle`; passes progress callback to `run_depth_estimation_with_progress`; stderr lines parsed in real time and emitted via `app_handle.emit()`.
- **Frontend:** Subscribes with `listen("depth-progress", ...)` before calling `generateDepthMap()`; updates progress bar; calls `unlisten()` in `finally` to avoid leaks.
- **Accessibility:** Progress bar uses `aria-valuenow` / `aria-valuemin` / `aria-valuemax`.

### Documentation

- **docs/architecture.md** — Progress section updated: Sprint 2.4 delivers real-time events and determinate bar.
- **docs/developer-guide.md** — `generate_depth_map` row notes `depth-progress` events.
- **docs/user-guide.md** — First conversion Step 2 mentions percentage progress bar.

---

## Installation

- Same as v0.3.0-beta.1: build from source (see [Developer Guide](developer-guide.md)); installers TBD.
- **Upgrade:** No data migration; progress streaming is transparent to existing workflows.

---

## Known issues

- Same as v0.3.0-beta.1 where applicable (Windows primary; model download; Depth-Anything-V2 non-commercial license).
- SEC-202 (SHA256 model download verification) and preset automated tests (JR2-1301–1303) may be completed in Sprint 2.4 or carried to a later sprint.

---

## For testers and contributors

- **User workflow:** [User guide](user-guide.md) — Step 2 (Generate depth map) now describes the percentage bar.
- **Architecture:** [RESEARCH/architecture.md](../RESEARCH/architecture.md) — ADR-002 addendum (progress event contract), BACK-205.
- **Manual test:** [TEST_PLAN_2_4.md](../SPRINTS/Sprint_2_4/TEST_PLAN_2_4.md) §1 — progress streaming on 4K run.

---

## Full changelog

See [CHANGELOG.md](../CHANGELOG.md) for the full list of changes (Sprint 2.4 entry to be added at release).

---

*This draft is for the Phase 2 Sprint 2.4 release. Paste into the GitHub Release description when the v0.4.0-beta.1 tag is cut; update version and date before tagging.*
