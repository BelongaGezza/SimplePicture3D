# SimplePicture3D Architecture

**Purpose:** Technical architecture and system design for SimplePicture3D.  
**Source:** `prd.md` §5.2, §5.3, §5.4; `RESEARCH/architecture.md`  
**Audience:** Developers, contributors, technical reviewers

---

## Overview

SimplePicture3D is a Tauri desktop application with a Rust backend, Svelte/React frontend, and a Python subprocess for AI depth estimation. All processing runs locally (100% offline).

---

## System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                     Tauri Frontend                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐   │
│  │ Svelte/React │  │  Three.js    │  │  TailwindCSS         │   │
│  │ Components   │  │  3D Preview  │  │  Styling             │   │
│  └──────┬───────┘  └──────┬───────┘  └──────────────────────┘   │
│         │                 │                                       │
│         └─────────┬───────┘                                       │
│                   │ Tauri Commands (IPC)                          │
└───────────────────┼───────────────────────────────────────────────┘
                    │
┌───────────────────▼───────────────────────────────────────────────┐
│                     Rust Backend                                  │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │  Core Logic                                                  │ │
│  │  • Image loading (image crate)                               │ │
│  │  • Depth map processing (gamma, range, invert)               │ │
│  │  • Mesh generation (point cloud, triangulation)              │ │
│  │  • STL/OBJ export (stl_io, obj crates)                       │ │
│  │  • Settings & state (serde JSON)                             │ │
│  └─────────────────────────────────────────────────────────────┘ │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │  Python Bridge (subprocess)                                  │ │
│  │  Spawns Python child; passes image via temp file; receives   │ │
│  │  depth map via stdout or output file                         │ │
│  └─────────────────────────────────────────────────────────────┘ │
└───────────────────┬───────────────────────────────────────────────┘
                    │ subprocess (stdin/temp file)
┌───────────────────▼───────────────────────────────────────────────┐
│                     Python AI Backend                             │
│  • Depth-Anything-V2 / MiDaS (PyTorch)                            │
│  • Input: Image bytes (file path or stdin)                        │
│  • Output: Depth map (JSON or binary)                             │
└───────────────────────────────────────────────────────────────────┘
```

---

## Data Flow: Image → STL

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  1. Load     │     │  2. Validate │     │  3. Depth    │     │  4. Process  │
│  Image       │ ──► │  (Rust)      │ ──► │  Estimation  │ ──► │  Depth       │
│  (Frontend)  │     │  Format,     │     │  (Python)    │     │  (Rust)      │
│  File picker │     │  size, 8K    │     │  NN infer    │     │  Gamma,      │
└──────────────┘     └──────────────┘     └──────────────┘     │  range       │
                                                                └──────┬───────┘
                                                                       │
┌──────────────┐     ┌──────────────┐     ┌──────────────┐             │
│  7. Export   │     │  6. Preview  │     │  5. Mesh     │             │
│  STL/OBJ     │ ◄── │  (Frontend)  │ ◄── │  Generation  │ ◄───────────┘
│  (Rust)      │     │  Three.js    │     │  (Rust)      │
│  User path   │     │  BufferGeo   │     │  Depth→mesh  │
└──────────────┘     └──────────────┘     └──────────────┘
```

### Pipeline Steps

1. **Load image** (Frontend → Rust): File picker → `load_image` Tauri command
2. **Validate** (Rust): Format, dimensions, downsample if >8K
3. **Depth estimation** (Rust → Python): Image bytes → subprocess → depth map
4. **Depth processing** (Rust): Gamma, range, invert per user controls
5. **Mesh generation** (Rust): Depth map → point cloud / triangulated mesh
6. **Preview** (Frontend): Vertex data → Three.js BufferGeometry
7. **Export** (Rust): STL/OBJ to user-selected path

---

## Key Interfaces

### Tauri Commands (Rust ↔ Frontend)

| Command           | Input                 | Output              |
|-------------------|-----------------------|---------------------|
| `load_image`      | File path             | Image metadata      |
| `generate_depth_map` | Image path/bytes   | Depth map           |
| `get_mesh_data`   | —                     | Vertex/normal data  |
| `export_stl`      | Path, mesh data       | Success/error       |
| `export_obj`      | Path, mesh data       | Success/error       |
| `download_model`  | Model ID              | Progress/result     |

### Python Interface (Rust ↔ Python)

- **Input:** Image via temp file path or stdin
- **Output:** Depth map via stdout (JSON) or output file (binary)
- **Spawn:** Shell plugin / sidecar; capabilities configured in Tauri v2

### User Data Locations

| Path                          | Purpose                    |
|-------------------------------|----------------------------|
| `~/.simplepicture3d/models/`  | AI model weights           |
| `~/.simplepicture3d/presets/` | User presets (JSON)        |
| `~/.simplepicture3d/logs/`    | Application logs           |
| `~/.simplepicture3d/cache/`   | Temporary processing       |
| `~/Documents/SimplePicture3D/exports/` | Default export location |

---

## Repository Structure

See [RESEARCH/architecture.md](../RESEARCH/architecture.md) for the full monorepo layout per PRD §5.4. Summary:

- **`src-tauri/`** — Rust backend, Tauri config
- **`src/`** — Frontend (Svelte/React)
- **`python/`** — Python AI backend
- **`RESEARCH/`** — Technology guidance
- **`SPRINTS/`** — Sprint tasking
- **`docs/`** — User and developer documentation

---

## References

- [prd.md](../prd.md) — Product requirements, §5.2–5.4
- [RESEARCH/architecture.md](../RESEARCH/architecture.md) — Detailed architecture, structure
- [RESEARCH/tauri.md](../RESEARCH/tauri.md) — Tauri v2, IPC, subprocess
- [RESEARCH/python-ml.md](../RESEARCH/python-ml.md) — Depth models, PyTorch
