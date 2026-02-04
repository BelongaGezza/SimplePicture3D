# SimplePicture3D Architecture

**Purpose:** System architecture and data flow for SimplePicture3D.

**Source:** Derived from `prd.md` §5.2, §5.3, §5.4.  
**Last checked:** 2026-02-01

---

## Overview

Tauri desktop app: Rust backend, Svelte/React frontend, Python subprocess for AI inference.

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

---

## Repository Structure (prd.md §5.4)

**Canonical definition.** Aligned with `prd.md` §5.4. Tauri + Python monorepo layout with clear separation of frontend, backend, and AI layers.

```
SimplePicture3D/
│
├── .agents/                # Multi-agent development personas
│   ├── system-architect.md
│   ├── senior-engineer.md
│   ├── junior-engineer-2d.md
│   ├── junior-engineer-3d.md
│   ├── researcher.md
│   ├── security-specialist.md
│   ├── ui-designer.md
│   └── documentation-specialist.md
│
├── .cursor/                # Cursor IDE configuration
│   ├── commands/           # Custom commands
│   └── rules/              # Agent rules (reference .agents/*.md)
│
├── RESEARCH/               # Technology guidance & knowledge base
│   ├── README.md
│   ├── AI_DEVELOPMENT_GUIDE.md
│   ├── architecture.md     # This file
│   ├── GOTCHAS.md
│   ├── rust-crates.md
│   ├── python-ml.md
│   ├── tauri.md
│   ├── frontend.md
│   └── threejs.md
│
├── SPRINTS/                # Sprint tasking & artefacts
│   ├── SPRINT_TASKING_TEMPLATE.md
│   └── Sprint_X_Y/         # Per-sprint folders
│       ├── SPRINT_X_Y_Task_Assignment.md
│       ├── PROGRESS_REPORT.md
│       ├── MANUAL_TEST_REPORT.md
│       ├── VERIFICATION_CHECKLIST.md
│       └── GOTCHAS.md      # Merge to RESEARCH/ when done
│
├── src-tauri/              # Rust backend (Tauri shell)
│   ├── src/
│   │   ├── main.rs         # Tauri app entry point
│   │   ├── lib.rs          # IPC command handlers (load_image, export_stl), integration tests
│   │   ├── image_loading.rs # Image load, validate, downsample, RGB, preview (BACK-101–105)
│   │   ├── file_io.rs       # Temp path utilities (future Python handoff)
│   │   ├── (future: mesh_generator.rs, depth_map.rs, exporters/, python_bridge.rs)
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                    # Frontend (Svelte/React)
│   ├── components/
│   │   ├── ImageImport.svelte
│   │   ├── DepthControls.svelte
│   │   ├── Preview3D.svelte
│   │   └── ExportPanel.svelte
│   ├── stores/
│   ├── lib/
│   └── App.svelte
│
├── python/                 # Python AI backend
│   ├── depth_estimator.py
│   ├── models/             # Model loading utilities
│   ├── requirements.txt
│   └── setup.py
│
├── tests/
│   ├── rust/
│   ├── python/
│   └── integration/
│
├── docs/
│   ├── user-guide.md
│   ├── developer-guide.md
│   ├── architecture.md     # User-facing architecture
│   └── api/
│
├── scripts/
│   ├── build.sh
│   ├── test.sh
│   └── package.sh
│
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
│
├── CLAUDE.md
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── prd.md
├── todo.md
└── package.json            # Frontend dependencies
```

**Key locations:**
- **Tauri app root:** Repository root; `package.json` and `src-tauri/` sibling
- **Frontend root:** `src/` (Svelte/React components)
- **Python backend:** `python/` (depth estimation, model utilities)

---

## Data Flow: Image → STL

1. **Load image** (Frontend → Rust): File picker → `load_image` command
2. **Validate** (Rust): Format, size, downsample if >8K
3. **Depth estimation** (Rust → Python): Image bytes → subprocess → depth map
4. **Depth processing** (Rust): Adjustments (gamma, range, invert)
5. **Mesh generation** (Rust): Depth map → point cloud / triangulated mesh
6. **Preview** (Frontend): Vertex data → Three.js BufferGeometry
7. **Export** (Rust): STL/OBJ to user-selected path

---

## Key Interfaces

- **Tauri commands:** `load_image`, `generate_depth_map`, `get_mesh_data`, `export_stl`, `export_obj`, `download_model`
- **Python interface (Sprint 1.3):** See **docs/architecture.md** § "Rust–Python Bridge (Sprint 1.3)" for the full IPC contract:
  - **Image input:** Temp file path only (`--input <path>`); path validated, under system temp dir (ARCH-102).
  - **Depth output:** JSON `{"height", "width", "depth": [f32,...]}` to stdout (or file); 0–1 normalized, row-major.
  - **Invocation:** Subprocess (no shell); fixed CLI; progress on stderr (ARCH-101, ARCH-103).
- **Model storage:** `~/.simplepicture3d/models/`
- **Settings:** `~/.simplepicture3d/` (presets, logs, cache)
