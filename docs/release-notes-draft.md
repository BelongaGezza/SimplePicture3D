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

*This draft is intended to be pasted into the GitHub Release description when the v0.1.0-beta.1 tag is cut. Update the links and date as needed.*
