# SimplePicture3D

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/BelongaGezza/SimplePicture3D/actions/workflows/ci.yml/badge.svg)](https://github.com/BelongaGezza/SimplePicture3D/actions)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)](https://github.com/BelongaGezza/SimplePicture3D)

> **Transform 2D images into volumetric 3D point clouds for internal UV laser engraving of crystal blocks**

SimplePicture3D is an open-source desktop application that converts ordinary 2D photographs and artwork into dense 3D point clouds optimized for internal laser engraving of K9 crystal, glass, and acrylic. Using AI-powered depth estimation combined with volumetric sampling, hobbyists can create stunning internal engravings from any image—no 3D modeling expertise required.

![SimplePicture3D Demo](docs/assets/demo-placeholder.png)
*Demo image coming soon*

---

## How It Works

1. **Load any 2D image** (photo, artwork, graphic)
2. **AI estimates depth** from the image (Depth-Anything-V2 / MiDaS)
3. **Define your crystal blank** (e.g., 80×50×50 mm with margins)
4. **Generate volumetric point cloud** filling the interior
5. **Export to PLY/XYZ/CSV** for your laser engraving software

The result: thousands of 3D coordinates distributed through your crystal blank, ready for internal UV laser engraving.

---

## Features

### Core Capabilities

- **AI-Powered Depth Estimation** — Depth-Anything-V2 extracts depth from any 2D image
- **Volumetric Point Cloud Generation** — Dense 3D coordinates filling your crystal blank interior
- **Crystal Blank Presets** — Common sizes (80×50×50mm, 60×60×60mm) or custom dimensions
- **Fit-to-Blank Scaling** — Automatic uniform scaling with configurable safety margins
- **Multiple Export Formats** — PLY, XYZ, CSV for broad engraver compatibility
- **Real-Time 3D Preview** — See your point cloud inside a blank wireframe before export
- **100% Offline Processing** — Your images never leave your device
- **Cross-Platform** — Windows, macOS, Linux

### Depth Adjustment Tools

- Brightness, contrast, gamma controls
- Curve editor with presets (S-curve, Linear, Exponential)
- Regional masking for selective adjustments
- Undo/redo with 20-action history

### Current Status: **In Development**

SimplePicture3D is undergoing an architectural pivot from surface mesh generation to volumetric point cloud output. Core infrastructure is complete; volumetric sampling and new export formats are in active development.

See [RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md](RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md) for the transition roadmap.

---

## Roadmap

**Phase 1: Volumetric Foundation**
- [x] Image loading and validation
- [x] AI depth estimation (Depth-Anything-V2 / MiDaS)
- [x] Depth adjustment tools (curves, masks, undo/redo)
- [x] 3D preview infrastructure (Three.js)
- [ ] BlankEnvelope and fit-to-blank scaling
- [ ] Volumetric point cloud generator (column sweep)
- [ ] PLY/XYZ/CSV exporters

**Phase 2: Engraver Integration**
- [ ] Validate exports with real engraver software
- [ ] Axis convention documentation
- [ ] Point density optimization
- [ ] Blank wireframe preview

**Phase 3: Cross-Platform**
- [ ] macOS builds (Intel + Apple Silicon)
- [ ] Linux builds (AppImage, .deb)

**Phase 4: Production**
- [ ] Professional installers
- [ ] Comprehensive documentation
- [ ] Optional: TripoSR full 3D reconstruction

See [todo.md](todo.md) for detailed sprint planning.

---

## Technology Stack

### Frontend
- **[Tauri](https://tauri.app)** — Lightweight desktop app framework
- **[Svelte 4](https://svelte.dev)** — Reactive UI framework
- **[Three.js](https://threejs.org)** — 3D point cloud rendering and preview
- **[TailwindCSS](https://tailwindcss.com)** — Utility-first styling

### Backend
- **[Rust](https://www.rust-lang.org)** — High-performance core (point cloud generation, export)
- **[Python](https://www.python.org)** — AI/ML inference pipeline
- **[PyTorch](https://pytorch.org)** — Deep learning for depth estimation

### AI Models
- **[Depth-Anything-V2](https://github.com/DepthAnything/Depth-Anything-V2)** — State-of-the-art depth estimation (default). Weights: **CC-BY-NC-4.0** (non-commercial).
- **[MiDaS](https://github.com/isl-org/MiDaS)** — Alternative model. **MIT-compatible** for commercial use.

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Tauri Frontend                        │
│  Svelte 4 │ Three.js Preview │ Blank Wireframe          │
└───────────────────────┬─────────────────────────────────┘
                        │ Tauri Commands (IPC)
┌───────────────────────▼─────────────────────────────────┐
│                   Rust Backend                          │
│  Image loading │ Depth processing │ Volumetric sampling │
│  PLY/XYZ/CSV export │ Settings │ Python subprocess      │
└───────────────────────┬─────────────────────────────────┘
                        │ subprocess
┌───────────────────────▼─────────────────────────────────┐
│                  Python AI Backend                      │
│  Depth-Anything-V2 / MiDaS │ PyTorch                    │
│  Input: Image → Output: Depth map                       │
└─────────────────────────────────────────────────────────┘
```

---

## Quick Start

### Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | 1.70+ | [rustup.rs](https://rustup.rs/) |
| Node.js | 18+ | [nodejs.org](https://nodejs.org/) |
| Python | 3.10+ | [python.org](https://python.org/) |
| Git | 2.x | [git-scm.com](https://git-scm.com/) |

### Setup

```bash
# Clone repository
git clone https://github.com/BelongaGezza/SimplePicture3D.git
cd SimplePicture3D

# Install frontend dependencies
npm install

# Set up Python environment
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r python/requirements.txt

# Run development server
npm run tauri dev
```

### Testing

```bash
# Rust tests
cargo test --manifest-path src-tauri/Cargo.toml

# Frontend tests
npm test

# Python tests (stub mode, no AI model required)
SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v
```

See [CLAUDE.md](CLAUDE.md) for complete build and test commands.

---

## Use Cases

SimplePicture3D is designed for:

- **Crystal Gift Makers** — Turn family photos into personalized 3D crystal keepsakes
- **Laser Engraving Businesses** — Streamline image-to-point-cloud workflow
- **Makers & Hobbyists** — Experiment with internal UV laser engraving
- **Artists** — Create original volumetric art for crystal medium

---

## Privacy & Security

- **100% Offline Processing** — All processing happens locally
- **No Cloud Dependencies** — AI models downloaded once, run locally
- **No Telemetry** — We don't collect usage data
- **Open Source** — Audit the code yourself
- **MIT Licensed** — Free to use, modify, distribute

Your images never leave your device.

---

## Contributing

We welcome contributions! Areas of interest:

- **Rust Backend** — Volumetric algorithms, export formats
- **Frontend/UI** — Svelte components, Three.js preview
- **AI/ML** — Depth model optimization
- **QA** — Testing, validation with real engravers
- **Documentation** — User guides, tutorials

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make changes following our coding standards
4. Write tests for new functionality
5. Submit a pull request

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## Documentation

- **[CLAUDE.md](CLAUDE.md)** — Development guidance and commands
- **[prd.md](prd.md)** — Product requirements document
- **[todo.md](todo.md)** — Sprint planning and task breakdown
- **[RESEARCH/architecture.md](RESEARCH/architecture.md)** — Architecture decisions (ADRs)
- **[RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md](RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md)** — Transition roadmap

---

## License

SimplePicture3D is licensed under the **MIT License** — see [LICENSE](LICENSE).

**AI Model Licenses:**
- Depth-Anything-V2: CC-BY-NC-4.0 (non-commercial)
- MiDaS: MIT-compatible (commercial OK)

See [RESEARCH/architecture.md](RESEARCH/architecture.md) ADR-005 for details.

---

## Acknowledgments

- **Depth-Anything-V2** team for state-of-the-art depth estimation
- **MiDaS** researchers for pioneering monocular depth prediction
- **Tauri** contributors for the desktop framework
- The **Rust**, **PyTorch**, and **Three.js** communities

---

<div align="center">

**Made with precision by the SimplePicture3D community**

[Report Bug](https://github.com/BelongaGezza/SimplePicture3D/issues) · [Request Feature](https://github.com/BelongaGezza/SimplePicture3D/discussions) · [Contribute](CONTRIBUTING.md)

</div>
