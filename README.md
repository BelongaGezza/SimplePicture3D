# SimplePicture3D

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)](https://github.com/BelongaGezza/SimplePicture3D/actions)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)](https://github.com/BelongaGezza/SimplePicture3D)

> **Convert 2D images into 2.5D STL meshes for UV laser engraving in crystal and glass**

SimplePicture3D is an open-source desktop application that transforms ordinary 2D photographs and artwork into stunning 3D engravings. Using AI-powered depth estimation combined with intuitive manual controls, hobbyists can create professional-quality point cloud meshes optimized for internal laser engraving of K9 crystal, glass, and acrylic.

![SimplePicture3D Demo](docs/assets/demo-placeholder.png)
*Demo image coming soon*

---

## ✨ Features

### Current Status: **In Development** 🚧

SimplePicture3D is currently in the planning phase. We're building:

- 🖼️ **Multi-Format Image Support** - PNG, JPG, TIFF, and more
- 🤖 **AI-Powered Depth Estimation** - Depth-Anything-V2 and MiDaS models
- 🎨 **Manual Depth Control** - Fine-tune with curves, masks, and adjustments
- 👁️ **Real-Time 3D Preview** - Interactive visualization with Three.js
- 📤 **STL & OBJ Export** - Industry-standard 3D file formats
- 🔒 **100% Offline Processing** - Your images never leave your device
- 🎯 **Laser-Optimized Output** - 2-10mm depth range, point cloud or mesh
- 🖥️ **Cross-Platform** - Windows, macOS, and Linux support

### Roadmap

**Phase 1: MVP** (Target: Q2 2026)
- Core image → depth map → STL conversion pipeline
- Basic UI with essential controls
- Windows desktop application

**Phase 2: Enhanced UX** (Target: Q3 2026)
- Advanced depth manipulation tools
- Preset system for common use cases
- Undo/redo functionality

**Phase 3: Cross-Platform** (Target: Q4 2026)
- macOS and Linux builds
- Platform-specific optimizations

**Phase 4: Production** (Target: Q1 2027)
- Performance optimization
- Professional installers with auto-update
- Comprehensive documentation and tutorials

See [TODO.md](TODO.md) for detailed development plan.

---

## 🚀 Quick Start

### Installation

> **Note:** SimplePicture3D is not yet available for download. Watch this repository for release announcements!

Once released, you'll be able to download installers for:
- **Windows**: `.msi` or `.exe` installer
- **macOS**: `.dmg` disk image (Intel and Apple Silicon)
- **Linux**: AppImage or `.deb` package

### Usage (Planned)

1. **Load an Image** - Drag and drop or browse for a PNG/JPG file
2. **Generate Depth Map** - AI automatically estimates depth from your 2D image
3. **Adjust Depth** - Use sliders to refine the depth range (2-10mm)
4. **Preview in 3D** - Rotate and inspect your mesh before exporting
5. **Export** - Save as STL or OBJ for your laser engraving software

Detailed usage guide coming soon in [docs/user-guide.md](docs/user-guide.md).

---

## 🛠️ Technology Stack

SimplePicture3D is built with modern, performant technologies:

### Frontend
- **[Tauri](https://tauri.app)** - Lightweight desktop app framework
- **[Svelte](https://svelte.dev)** - Reactive UI framework
- **[Three.js](https://threejs.org)** - 3D rendering and visualization
- **[TailwindCSS](https://tailwindcss.com)** - Utility-first styling

### Backend
- **[Rust](https://www.rust-lang.org)** - High-performance core logic (mesh generation, file I/O)
- **[Python](https://www.python.org)** - AI/ML inference pipeline
- **[PyTorch](https://pytorch.org)** - Deep learning framework for depth estimation

### AI Models
- **[Depth-Anything-V2](https://github.com/DepthAnything/Depth-Anything-V2)** - State-of-the-art monocular depth estimation (default). Weights: **CC-BY-NC-4.0** (non-commercial use only).
- **[MiDaS](https://github.com/isl-org/MiDaS)** - Alternative depth estimation model. **MIT-compatible**; suitable for commercial use. See [RESEARCH/architecture.md](RESEARCH/architecture.md) **ADR-005** for licensing details.

### Architecture

```
┌─────────────────────────────────────────┐
│         Tauri Frontend (Svelte)         │
│    UI Components │ Three.js Preview     │
└──────────────┬──────────────────────────┘
               │ IPC Commands
┌──────────────▼──────────────────────────┐
│          Rust Backend                   │
│  Image Processing │ Mesh Generation     │
│  File I/O │ Settings │ Export (STL/OBJ) │
└──────────────┬──────────────────────────┘
               │ Subprocess
┌──────────────▼──────────────────────────┐
│        Python AI Backend                │
│   PyTorch │ Depth-Anything-V2 │ MiDaS   │
└─────────────────────────────────────────┘
```

**Repository structure:** See [RESEARCH/architecture.md](RESEARCH/architecture.md) for the full monorepo layout (src-tauri/, src/, python/, SPRINTS/, RESEARCH/, etc.) per PRD §5.4.

---

## 📚 Documentation

- **[Product Requirements Document (PRD)](prd.md)** - Comprehensive product specification
- **[Development TODO](todo.md)** - Phased sprint plan with team assignments
- **[Architecture Guide](docs/architecture.md)** - Technical architecture (coming soon)
- **[Setting Up Your Mac](docs/setting_up_your_Mac.md)** - Developer tools and setup for macOS (Apple Silicon)
- **[User Guide](docs/user-guide.md)** - How to use SimplePicture3D (coming soon)
- **[Developer Guide](docs/developer-guide.md)** - Build and contribution instructions (coming soon)

---

## 🤝 Contributing

We welcome contributions from the community! SimplePicture3D is built with collaboration in mind.

### Development Setup

**Required tools and versions:**

| Tool | Minimum version | How to install | Notes |
|------|-----------------|-----------------|-------|
| Rust (rustc, cargo) | 1.70+ | [rustup](https://rustup.rs/) | Use `rustup default stable` |
| Node.js | 18+ | [nodejs.org](https://nodejs.org/) | LTS recommended; includes npm |
| npm | 9+ | Bundled with Node.js | `npm --version` |
| Python | 3.10+ | [python.org](https://www.python.org/) | Required for AI depth estimation; see [Python environment](#python-environment-ai-backend) below |
| Git | 2.x | [git-scm.com](https://git-scm.com/) | For clone and contribution |

**Clone the repository:**
```bash
git clone https://github.com/BelongaGezza/SimplePicture3D.git
cd SimplePicture3D
```

**Setup instructions:**
```bash
npm install
# On Windows: generate app icon before first build (see RESEARCH/GOTCHAS.md)
# npm run tauri icon path/to/1024x1024.png
npm run tauri dev
```

**Python environment (AI backend):**  
Depth estimation runs in a Python subprocess. You need **Python 3.10+** and a virtual environment. From the project root:

```bash
# Create virtual environment (recommended)
python -m venv venv
# Windows:
venv\Scripts\activate
# macOS/Linux:
source venv/bin/activate

# Install dependencies
pip install -r python/requirements.txt
```

- **Stub mode (no AI model):** For tests and CI, set `SP3D_USE_STUB=1` so no model is downloaded. Run pytest from the project root:
  - **macOS/Linux:** `SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v`
  - **Windows PowerShell:** `$env:SP3D_USE_STUB="1"; $env:PYTHONPATH="python\python"; python -m pytest python/ -v`
- **Real depth inference:** Install PyTorch per [pytorch.org](https://pytorch.org/get-started/locally/) (CPU/CUDA/Metal), then install the rest of `python/requirements.txt`. The first run will download the Depth-Anything-V2 model from Hugging Face (model download wizard planned for Sprint 1.10). See [python/README.md](python/README.md) for model paths and `--no-model` usage.
- **Model license:** Depth-Anything-V2 weights are **CC-BY-NC-4.0** (non-commercial use). MiDaS is **MIT-compatible** for commercial use. See [License](#-license) and **ADR-005** in [RESEARCH/architecture.md](RESEARCH/architecture.md).
- **Why system Python:** For rationale (system Python for MVP, no bundling yet), see [RESEARCH/architecture.md](RESEARCH/architecture.md) **ADR-003**.

**Troubleshooting:** If the app reports "Python not found", ensure `python` (or `python3`) is on your PATH and is 3.10+. On Windows, the Tauri app may not see the same PATH as your terminal—install Python for "all users" or add it to the system PATH.

**Verifying your setup:** Run these from the project root to confirm all three environments work:

```bash
# Rust
rustc --version
cargo --version
cd src-tauri && cargo build && cd ..

# Node / frontend
node --version
npm --version
npm run build

# Full app (Tauri + frontend)
npm run tauri dev
```

### Testing

Run the test suites before submitting changes:

```bash
# Rust (from project root)
cargo test --manifest-path src-tauri/Cargo.toml

# Frontend (when tests are configured)
npm test

# Python (depth_estimator; no AI model required in stub mode)
# From project root (see CLAUDE.md for Windows):
SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v
```

Python tests use **stub mode** (`SP3D_USE_STUB=1`) so no model download is needed. See [CLAUDE.md](CLAUDE.md) for the full testing command list and stub mode details.

When the Python environment is set up (see **[python/README.md](python/README.md)** and `python/requirements.txt` when present), you can also run:

```bash
cd python
python -m venv venv
# Windows: venv\Scripts\activate   |   macOS/Linux: source venv/bin/activate
pip install -r requirements.txt
# Run depth estimator: python -m python.depth_estimator --input path/to/image.png
```

**Logging (Rust backend):** Set `RUST_LOG` to control log level (e.g. `RUST_LOG=debug`, `RUST_LOG=simplepicture3d=info`). See [env_logger](https://docs.rs/env_logger). Default is `warn` if unset.

**Development (hot-reload):** `npm run tauri dev` starts the Vite dev server and opens the Tauri window. Frontend changes (Svelte, CSS, TS) hot-reload in the app; Rust changes require a full restart. See [RESEARCH/AI_DEVELOPMENT_GUIDE.md](RESEARCH/AI_DEVELOPMENT_GUIDE.md) for commands.

> **Windows:** If `cargo build` in `src-tauri` fails with RC2176 "old DIB" on `icon.ico`, run `npm run tauri icon path/to/1024x1024.png` to generate compatible icons, then rebuild. See [RESEARCH/GOTCHAS.md](RESEARCH/GOTCHAS.md).
>
> Detailed build instructions will be added to `docs/developer-guide.md` as development progresses.

### How to Contribute

1. **Fork the repository** and create a feature branch
2. **Make your changes** following our coding standards
3. **Write tests** for new functionality
4. **Submit a pull request** with a clear description

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines (coming soon).

### Team Roles

We're looking for contributors in these areas:
- 🏗️ **System Architecture** - Overall design and tech decisions
- ⚙️ **Backend Engineering (Rust)** - Mesh generation, performance optimization
- 🔬 **AI/ML Research** - Depth estimation, model optimization
- 🎨 **Frontend/UI** - User experience, 3D visualization
- 🧪 **Quality Assurance** - Testing, CI/CD, automation
- 🔐 **Security** - Code audits, dependency management

---

## 📋 Project Status

### Current Phase: **Planning & Setup**

✅ Product Requirements Document (PRD) complete  
✅ Development roadmap (TODO.md) finalized  
✅ GitHub repository created  
⬜ Development environment setup  
⬜ Sprint 1.1: Project foundations (starting soon)

### Development Timeline

| Phase | Status | Target |
|-------|--------|--------|
| Phase 1: MVP | 🟡 Planning | Q2 2026 |
| Phase 2: Enhanced UX | ⬜ Not Started | Q3 2026 |
| Phase 3: Cross-Platform | ⬜ Not Started | Q4 2026 |
| Phase 4: Production | ⬜ Not Started | Q1 2027 |

Track progress on our [GitHub Project Board](https://github.com/BelongaGezza/SimplePicture3D/projects) (coming soon).

---

## 🎯 Use Cases

SimplePicture3D is perfect for:

- **Crystal Gift Makers** - Turn family photos into personalized 3D crystal keepsakes
- **Laser Engraving Businesses** - Streamline your workflow from image to engraving
- **Makers & Hobbyists** - Experiment with UV laser engraving on glass and acrylic
- **Artists** - Create original 3D depth art for a unique medium
- **Educators** - Teach 3D concepts and laser technology

---

## 🔒 Privacy & Security

SimplePicture3D is designed with privacy as a core principle:

- ✅ **100% Offline Processing** - All image processing happens on your local machine
- ✅ **No Cloud Dependencies** - AI models downloaded once, run locally
- ✅ **No Telemetry** - We don't collect usage data (crash reports are opt-in)
- ✅ **Open Source** - Audit the code yourself, no hidden behavior
- ✅ **MIT Licensed** - Free to use, modify, and distribute

Your images are yours. They never leave your device.

---

## 📜 License

SimplePicture3D is licensed under the **MIT License** - see [LICENSE](LICENSE) for details.

### Third-Party Licenses

This project incorporates open-source software with compatible licenses:
- Tauri (MIT)
- Rust ecosystem crates (MIT/Apache-2.0)
- PyTorch (BSD-3-Clause)
- Three.js (MIT)

**Note:** AI model weights have separate licenses. **Depth-Anything-V2** (default): CC-BY-NC-4.0 (non-commercial only). **MiDaS**: MIT-compatible (commercial use allowed). See [RESEARCH/architecture.md](RESEARCH/architecture.md) **ADR-005**.

See [ATTRIBUTION.md](ATTRIBUTION.md) for full third-party notices (coming soon).

---

## 💖 Support the Project

SimplePicture3D is open source and free to use. If you find it valuable, consider supporting development:

- ⭐ **Star this repository** to show your support
- 🐛 **Report bugs** via [GitHub Issues](https://github.com/BelongaGezza/SimplePicture3D/issues)
- 💡 **Suggest features** in [GitHub Discussions](https://github.com/BelongaGezza/SimplePicture3D/discussions)
- 🤝 **Contribute code** (see [Contributing](#contributing))
- ☕ **Buy us a coffee** via [Ko-fi](https://ko-fi.com/) (link coming soon)

---

## 🙏 Acknowledgments

SimplePicture3D stands on the shoulders of giants:

- **Depth-Anything-V2** team for state-of-the-art depth estimation
- **MiDaS** researchers for pioneering monocular depth prediction
- **Tauri** contributors for enabling lightweight cross-platform apps
- The **Rust**, **PyTorch**, and **Three.js** communities

Special thanks to early contributors and beta testers (you'll be listed here!).

---

## 📞 Contact & Community

- **GitHub Issues**: [Report bugs](https://github.com/BelongaGezza/SimplePicture3D/issues)
- **GitHub Discussions**: [Ask questions, share ideas](https://github.com/BelongaGezza/SimplePicture3D/discussions)
- **Email**: [Contact maintainer](mailto:your-email@example.com) (update with real email)
- **Discord**: Community server (coming soon)

---

## 🗺️ Roadmap Highlights

### v0.1.0 - MVP (Q2 2026)
- [ ] Basic image loading and display
- [ ] AI depth estimation (Depth-Anything-V2)
- [ ] Manual depth adjustments (sliders)
- [ ] Point cloud mesh generation
- [ ] STL export
- [ ] Windows installer

### v0.2.0 - Enhanced UX (Q3 2026)
- [ ] Advanced depth controls (curves, masks)
- [ ] Preset system
- [ ] Undo/redo
- [ ] OBJ export
- [ ] Enhanced 3D preview

### v0.3.0 - Cross-Platform (Q4 2026)
- [ ] macOS support (Intel + Apple Silicon)
- [ ] Linux support (AppImage, .deb)
- [ ] Platform-specific optimizations

### v1.0.0 - Production (Q1 2027)
- [ ] Performance optimizations
- [ ] Comprehensive documentation
- [ ] Professional installers (code-signed)
- [ ] Auto-update mechanism
- [ ] Accessibility (WCAG AA)

See [TODO.md](todo.md) for the complete sprint-by-sprint plan.

---

## 📊 Project Metrics

> Metrics will be updated as development progresses.

- **Code Coverage**: Target >80% (TBD)
- **Build Status**: Pending setup
- **Contributors**: 1 (growing!)
- **Stars**: ⭐ your support counts!
- **Open Issues**: 0

---

## ⚖️ Legal

SimplePicture3D is provided "as-is" without warranty. While we strive for accuracy and reliability, use at your own risk. See [LICENSE](LICENSE) for full terms.

**AI Model Usage**: Depth estimation models are used in compliance with their respective licenses. Users are responsible for ensuring their use case aligns with model license terms (particularly non-commercial restrictions on some models).

---

<div align="center">

**Made with ❤️ by the SimplePicture3D community**

[⬆ Back to Top](#simplepicture3d)

</div>
