# SimplePicture3D

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/BelongaGezza/SimplePicture3D/actions/workflows/ci.yml/badge.svg)](https://github.com/BelongaGezza/SimplePicture3D/actions)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)](https://github.com/BelongaGezza/SimplePicture3D)

> **Transform 2D images into 3D surface point clouds for internal UV laser engraving of crystal blocks**

SimplePicture3D is an open-source desktop application that converts ordinary 2D photographs and artwork into 3D point clouds optimized for internal laser engraving of K9 crystal, glass, and acrylic. Using AI-powered depth estimation, hobbyists can create stunning internal engravings from any image—no 3D modeling expertise required.

![SimplePicture3D Demo](docs/assets/demo-placeholder.png)
*Demo image coming soon*

---

## How It Works

1. **Load any 2D image** (photo, artwork, graphic)
2. **AI estimates depth** from the image (Depth-Anything-V2 / MiDaS)
3. **Define your crystal blank** (e.g., 80×50×50 mm with margins)
4. **Generate a surface-map point cloud** — one laser dot per sampled (x,y) at the depth-mapped Z position inside the blank
5. **Export to PLY/XYZ/CSV** for your laser engraving software

The result: a 3D point distribution tracing the depth shape of your image inside the crystal blank, ready for internal UV laser engraving.

---

## Features

### Core Capabilities

- **AI-Powered Depth Estimation** — Depth-Anything-V2 extracts depth from any 2D image
- **Surface-Map Point Cloud** — One dot per column at mapped Z depth (ADR-012)
- **Crystal Blank Presets** — Common sizes (80×50×50 mm, 60×60×60 mm) or custom dimensions
- **Fit-to-Blank Scaling** — Automatic uniform scaling with configurable safety margins
- **Multiple Export Formats** — PLY, XYZ, CSV for broad engraver compatibility
- **Real-Time 3D Preview** — Point cloud inside a blank wireframe before export
- **100% Offline Processing** — Your images never leave your device
- **Cross-Platform** — Windows (primary), macOS and Linux (Phase 3)

### Depth Adjustment Tools

- Brightness, contrast, gamma controls
- Curve editor with presets (S-curve, Linear, Exponential)
- Regional masking for selective adjustments
- Undo/redo with 20-action history

### Current Status: **Pre-release — validation sprint**

Core pipeline is implemented (image → depth → surface-map point cloud → export). The team is validating engraver compatibility and closing security items before v1.0 beta.

See **[ROADMAP.md](ROADMAP.md)** for the production path and **[todo.md](todo.md)** for active tasks.

---

## Roadmap

| Phase | Focus | Status |
|-------|-------|--------|
| **A–C** | Pivot from 2.5D mesh to ADR-012 surface-map; wire backend + UI | ✅ Complete |
| **D** | Engraver validation, security hardening, integration tests | ⏳ In progress |
| **E** | v1.0 MVP — performance, docs, Windows installer | Planned |
| **F** | macOS / Linux packaging | Phase 3 |
| **G** | TripoSR, E2E automation, accessibility | Post-v1.0 |

Full detail: [ROADMAP.md](ROADMAP.md)

---

## Quick Start

### Prerequisites

- **Node.js** 18+ and npm
- **Rust** 1.70+ ([rustup](https://rustup.rs/))
- **Python** 3.10–3.12 with venv
- **Platform deps:** see [SETUP_NOTES.md](SETUP_NOTES.md) (Linux: GTK3 + WebKit dev libraries)

### Development

```bash
# Clone and install frontend
git clone https://github.com/BelongaGezza/SimplePicture3D.git
cd SimplePicture3D
npm install

# Python environment
cd python && python3 -m venv venv && source venv/bin/activate
pip install -r requirements.txt
cd ..

# Run dev server (frontend hot-reload + Tauri)
npm run tauri dev
```

### Testing

```bash
npm test
cargo test --manifest-path src-tauri/Cargo.toml
SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v
```

See [CONTRIBUTING.md](CONTRIBUTING.md) and [CLAUDE.md](CLAUDE.md) for full commands.

---

## Architecture

```
Svelte 4 UI ──Tauri IPC──► Rust backend
                                │
                                ├─ Depth processing, mask, undo, presets
                                ├─ Surface-map point cloud (ADR-012)
                                ├─ PLY / XYZ / CSV export
                                └─ Python subprocess ──► depth_estimator
```

Canonical design: [RESEARCH/architecture.md](RESEARCH/architecture.md) (ADR-012).  
Takeover review: [docs/ARCHITECT_REVIEW_2026-05.md](docs/ARCHITECT_REVIEW_2026-05.md)

---

## Documentation

| Document | Purpose |
|----------|---------|
| [ROADMAP.md](ROADMAP.md) | Vision and path to production |
| [todo.md](todo.md) | Active tasks and sprint gates |
| [prd.md](prd.md) | Product requirements |
| [RESEARCH/architecture.md](RESEARCH/architecture.md) | ADRs and system design |
| [docs/developer-guide.md](docs/developer-guide.md) | Contributor setup |
| [docs/testing.md](docs/testing.md) | Manual QA checklist |

---

## License

MIT License — see [LICENSE](LICENSE).

**Model weights:** Depth-Anything-V2 is [CC-BY-NC-4.0](https://creativecommons.org/licenses/by-nc/4.0/) (non-commercial). MiDaS is MIT-compatible for commercial use. See [RESEARCH/architecture.md](RESEARCH/architecture.md) ADR-005.

---

## Contributing

Contributions welcome! Read [CONTRIBUTING.md](CONTRIBUTING.md) and check [todo.md](todo.md) for open tasks. Use the agent personas in `.agents/` for domain-specific guidance.
