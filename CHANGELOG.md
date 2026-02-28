# Changelog

All notable changes to SimplePicture3D are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned

- Windows installer (Phase 1 exit)
- E2E test suite and beta-ready documentation (Sprint 1.12)

---

## [0.1.0-beta.1] - TBD (MVP / Phase 1 Beta)

First beta release for Phase 1 MVP. Targets internal UV laser engraving of K9 crystal, glass, and acrylic.

### Added

- **Image loading (F1.1)** — Load PNG, JPG (and TIFF where supported). Max 8192×8192; automatic downsampling with notification. Drag-and-drop and file browser with preview.
- **AI depth estimation (F1.2)** — Depth-Anything-V2 and MiDaS support via Python subprocess. CPU/GPU auto-detect; progress indicator; normalized 0–1 depth output.
- **Manual depth controls (F1.3)** — Sliders for brightness, contrast, gamma, invert; depth range (min/max mm). Reset to defaults.
- **Mesh generation (F1.4)** — Point cloud and triangulated mesh from depth map. Configurable step for preview vs full-detail export.
- **3D preview (F1.4)** — Interactive Three.js preview with positions and normals; optional reduced-detail preview step.
- **STL export (F1.5)** — Binary STL export with path validation and security checks (no system directories, writable parent).
- **OBJ export (F1.6)** — ASCII OBJ with optional MTL; same validation as STL.
- **Settings (F1.7)** — Persisted app settings: depth range, target dimensions (ADR-009), last export directory, export format preference, window size.
- **Target dimensions (ADR-009)** — Optional target width/height in mm; mesh XY scales to fit inside target rectangle while preserving aspect ratio.
- **Model wizard (F1.8)** — First-run flow: check model status, display model info (license, size), download Depth-Anything-V2 from Hugging Face.
- **Security** — Path validation (SEC-101, SEC-402), export path canonicalization and blocklist (SEC-401), magic-byte format validation (SEC-102). No telemetry; 100% offline processing.

### Known limitations

- **Platform:** Windows primary; macOS and Linux builds not yet fully validated.
- **Model license:** Default Depth-Anything-V2 weights are CC-BY-NC-4.0 (non-commercial use only). Use MiDaS for MIT-compatible commercial use (see RESEARCH/architecture.md ADR-005).
- **Performance:** 4K image to mesh target &lt;2 min on mid-range hardware; first run requires one-time model download (~500MB–2GB).
- **Localization:** UI and docs are English only.

### Documentation

- User guide: [docs/user-guide.md](docs/user-guide.md) (when complete)
- Developer guide: [docs/developer-guide.md](docs/developer-guide.md) (when complete)
- Architecture: [RESEARCH/architecture.md](RESEARCH/architecture.md), [docs/architecture.md](docs/architecture.md)
- Beta onboarding: [docs/beta-onboarding.md](docs/beta-onboarding.md) (when complete)

---

[Unreleased]: https://github.com/BelongaGezza/SimplePicture3D/compare/v0.1.0-beta.1...HEAD
[0.1.0-beta.1]: https://github.com/BelongaGezza/SimplePicture3D/releases/tag/v0.1.0-beta.1
