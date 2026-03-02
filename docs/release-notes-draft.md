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
