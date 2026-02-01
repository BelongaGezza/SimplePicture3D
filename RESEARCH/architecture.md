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

## Directory Structure (prd.md §5.4)

```
SimplePicture3D/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs
│   │   ├── image_processing.rs
│   │   ├── mesh_generator.rs
│   │   ├── depth_map.rs
│   │   ├── exporters/  (stl.rs, obj.rs)
│   │   └── python_bridge.rs
│   └── Cargo.toml
├── src/                # Frontend (Svelte/React)
├── python/             # AI backend
│   ├── depth_estimator.py
│   └── requirements.txt
└── tests/
```

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
- **Python interface:** stdin or temp file for image; stdout or file for depth map (JSON/binary)
- **Model storage:** `~/.simplepicture3d/models/`
- **Settings:** `~/.simplepicture3d/` (presets, logs, cache)
