# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SimplePicture3D is a desktop application that converts 2D images into 2.5D STL/OBJ mesh files for internal UV laser engraving of K9 crystal, glass, and acrylic. Uses AI-powered depth estimation with manual controls.

**Current Status:** Planning/documentation phase. No source code exists yet.

## Repository Structure (Current)

```
SimplePicture3D/
├── .agents/                    # Agent personas for multi-agent development
├── .cursor/rules/              # Cursor IDE rules (reference .agents/*.md)
├── RESEARCH/                   # Technology guidance and knowledge base
│   ├── AI_DEVELOPMENT_GUIDE.md # Multi-agent coordination patterns
│   ├── architecture.md         # System architecture
│   ├── GOTCHAS.md              # Debugging findings (all agents contribute)
│   ├── rust-crates.md          # Rust dependency guidance
│   ├── python-ml.md            # Python/PyTorch/depth models
│   ├── tauri.md                # Tauri framework
│   ├── frontend.md             # Svelte/React/TypeScript
│   └── threejs.md              # Three.js 3D rendering
├── SPRINTS/                    # Sprint tasking templates and artefacts
├── prd.md                      # Product requirements (canonical spec)
├── todo.md                     # Sprint planning and task breakdown
├── CONTRIBUTING.md             # Contribution guidelines
└── README.md                   # Project overview
```

## Key Reference Documents

| Document | Purpose |
|----------|---------|
| `prd.md` | Product requirements, tech stack (§5.1), architecture (§5.2), file structure (§5.4) |
| `todo.md` | Sprint planning, phase milestones, task IDs |
| `RESEARCH/AI_DEVELOPMENT_GUIDE.md` | Multi-agent coordination, handover protocol |
| `RESEARCH/architecture.md` | System design, data flow |
| `RESEARCH/GOTCHAS.md` | Known debugging pitfalls |

## Technology Stack (Planned)

| Layer | Technologies | Research File |
|-------|--------------|---------------|
| Shell | Tauri | `RESEARCH/tauri.md` |
| Backend | Rust (image, stl_io, tokio, serde, anyhow) | `RESEARCH/rust-crates.md` |
| AI | Python, PyTorch, Depth-Anything-V2/MiDaS | `RESEARCH/python-ml.md` |
| Frontend | Svelte or React, TypeScript, TailwindCSS | `RESEARCH/frontend.md` |
| 3D | Three.js | `RESEARCH/threejs.md` |

## Build Commands (When Source Code Exists)

```bash
# Rust backend
cd src-tauri && cargo build

# Frontend
npm install
npm run dev

# Python environment
cd python && python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt

# Development server
npm run tauri dev

# Production build
npm run tauri build
```

## Testing Commands

Run from the **project root** unless noted.

```bash
# Rust (from project root)
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo fmt --check

# Frontend
npm test                # Vitest (Sprint 1.5A); use npm run test:watch for watch mode
npm run lint

# Python (depth_estimator) — no AI model required in stub mode
# Linux/macOS:
SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v
# Windows PowerShell:
$env:SP3D_USE_STUB="1"; $env:PYTHONPATH="python\python"; python -m pytest python/ -v

# Security
cargo audit
npm audit
pip-audit
```

**Python stub mode:** Set `SP3D_USE_STUB=1` (or use `--no-model` when invoking the CLI) to run depth estimation without downloading the AI model. All pytest tests use stub mode by default.

## Multi-Agent Development Workflow

This project uses agent personas for development coordination:

1. **Agent personas** are in `.agents/*.md`
2. **Sprint tasking** uses `SPRINTS/SPRINT_TASKING_TEMPLATE.md`
3. **Task handover** follows `RESEARCH/AI_DEVELOPMENT_GUIDE.md`
4. **Debugging gotchas** are recorded in `RESEARCH/GOTCHAS.md`

### Agent Roles

| Role | Persona File | Focus |
|------|--------------|-------|
| System Architect | `.agents/system-architect.md` | Architecture decisions, API design |
| Senior Engineer | `.agents/senior-engineer.md` | Rust backend, Python bridge, sprint creation |
| UI Designer | `.agents/ui-designer.md` | Frontend, Three.js preview |
| Researcher | `.agents/researcher.md` | RESEARCH/ maintenance, tech verification |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Image/depth processing |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Mesh, STL/OBJ, Three.js |
| Security Specialist | `.agents/security-specialist.md` | Security review, audits |
| Documentation Specialist | `.agents/documentation-specialist.md` | User/dev docs |

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                     Tauri Frontend                      │
│  Svelte/React │ Three.js 3D Preview │ TailwindCSS       │
└───────────────────────┬─────────────────────────────────┘
                        │ Tauri Commands (IPC)
┌───────────────────────▼─────────────────────────────────┐
│                   Rust Backend                          │
│  Image loading │ Depth processing │ Mesh generation     │
│  STL/OBJ export │ Settings │ Python subprocess bridge   │
└───────────────────────┬─────────────────────────────────┘
                        │ subprocess (stdin/temp file)
┌───────────────────────▼─────────────────────────────────┐
│                  Python AI Backend                      │
│  Depth-Anything-V2 / MiDaS │ PyTorch                    │
│  Input: Image → Output: Depth map (JSON/binary)         │
└─────────────────────────────────────────────────────────┘
```

## Data Flow: Image to STL

1. Load image (Frontend → Rust): File picker → `load_image` command
2. Validate (Rust): Format, size, downsample if >8K
3. Depth estimation (Rust → Python): Image bytes → subprocess → depth map
4. Depth processing (Rust): Adjustments (gamma, range, invert)
5. Mesh generation (Rust): Depth map → point cloud / triangulated mesh
6. Preview (Frontend): Vertex data → Three.js BufferGeometry
7. Export (Rust): STL/OBJ to user-selected path

## Tauri Commands (Planned)

- `load_image` - Load and validate image file
- `generate_depth_map` - Run AI depth estimation
- `get_mesh_data` - Get mesh vertices for preview
- `export_stl` - Export binary STL
- `export_obj` - Export ASCII OBJ
- `download_model` - Download AI model from Hugging Face

## Important Constraints

- **100% offline processing** - no cloud services
- **MIT License** - no GPL/AGPL dependencies
- **Cross-platform** - Windows, macOS, Linux
- **Memory budget** - <2GB for 4K images
- **Model license** - Depth-Anything-V2 weights are CC-BY-NC-4.0 (non-commercial)

## User Data Locations (Planned)

```
~/.simplepicture3d/
  ├── models/      # AI models (500MB-2GB)
  ├── presets/     # User presets (JSON)
  ├── logs/        # Application logs
  └── cache/       # Temporary files

~/Documents/SimplePicture3D/exports/  # Default export location
```

## Before Starting Work

1. Review `RESEARCH/` for technologies relevant to your task
2. Check `RESEARCH/GOTCHAS.md` for known pitfalls
3. Read the relevant agent persona in `.agents/`
4. Consult `prd.md` for acceptance criteria
5. Check `todo.md` for task context and dependencies
