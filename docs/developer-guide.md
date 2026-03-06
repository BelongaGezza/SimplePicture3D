# SimplePicture3D Developer Guide

This guide covers building from source, the high-level architecture, generating API documentation, and where to find contribution guidelines.

**Canonical references:** [prd.md](../prd.md) (requirements), [RESEARCH/architecture.md](../RESEARCH/architecture.md) (architecture and ADRs), [CONTRIBUTING.md](../CONTRIBUTING.md) (contribution process).

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Build from Source](#build-from-source)
3. [Architecture Overview](#architecture-overview)
4. [Tauri Commands (IPC Contract)](#tauri-commands-ipc-contract)
5. [Generating Rust API Documentation](#generating-rust-api-documentation)
6. [Testing and Linting](#testing-and-linting)
7. [Contribution Guidelines](#contribution-guidelines)

---

## Prerequisites

| Tool | Minimum version | How to install |
|------|-----------------|----------------|
| Rust (rustc, cargo) | 1.70+ | [rustup](https://rustup.rs/) |
| Node.js | 18+ | [nodejs.org](https://nodejs.org/) |
| npm | 9+ | Bundled with Node.js |
| Python | 3.10+ | [python.org](https://www.python.org/) — required for AI depth estimation |
| Git | 2.x | [git-scm.com](https://git-scm.com/) |

---

## Build from Source

All commands assume you are at the **project root** unless noted.

### 1. Rust backend

```bash
cd src-tauri
cargo build
cd ..
```

Or from the project root:

```bash
cargo build --manifest-path src-tauri/Cargo.toml
```

### 2. Node / frontend

```bash
npm install
npm run build
```

### 3. Python (AI backend)

Depth estimation runs in a Python subprocess. You need Python 3.10+ and a virtual environment:

```bash
# Create virtual environment
python -m venv venv

# Windows:
venv\Scripts\activate

# macOS/Linux:
source venv/bin/activate

# Install dependencies (from project root)
pip install -r python/requirements.txt
```

- **Stub mode (no AI model):** Set `SP3D_USE_STUB=1` so no model is downloaded. Used by CI and tests.
- **Real depth inference:** Install PyTorch per [pytorch.org](https://pytorch.org/get-started/locally/), then install the rest of `python/requirements.txt`. The first run may download the Depth-Anything-V2 model (see model download wizard in the app).

### 4. Run the application

```bash
npm run tauri dev
```

This starts the Vite dev server and opens the Tauri window. Frontend changes (Svelte, CSS, TypeScript) hot-reload; Rust changes require a full restart.

### 5. Production build

```bash
npm run tauri build
```

Outputs platform-specific installers (e.g. Windows `.msi`/`.exe`, macOS `.dmg`, Linux AppImage) in `src-tauri/target/release/bundle/`.

### Verifying your setup

From the project root:

```bash
# Rust
cd src-tauri && cargo build && cd ..

# Node / frontend
npm run build

# Full app
npm run tauri dev
```

**Windows:** If `cargo build` fails with RC2176 "old DIB" on `icon.ico`, run `npm run tauri icon path/to/1024x1024.png` to generate compatible icons. See [RESEARCH/GOTCHAS.md](../RESEARCH/GOTCHAS.md).

---

## Architecture Overview

SimplePicture3D is a Tauri desktop app with three layers:

- **Frontend (Svelte):** UI components, Three.js 3D preview, TailwindCSS. Communicates with the backend via Tauri IPC (`invoke()`).
- **Rust backend:** Image loading, depth processing, mesh generation, STL/OBJ export, settings, Python subprocess bridge, **undo/redo command history**. Commands are defined in `src-tauri/src/lib.rs` and supporting modules.
- **Python AI backend:** Depth estimation (Depth-Anything-V2 / MiDaS) via subprocess. Input: image bytes or temp file path. Output: depth map (JSON or binary).

**Data flow:** Load image → Validate → Depth (Python) → Depth processing (Rust) → Mesh generation (Rust) → Preview (Three.js) → Export (STL/OBJ).

**State management and undo:** Depth params and curve are the single source of truth in the backend. Undo/redo use a command pattern (max 20 actions); see [RESEARCH/architecture.md](../RESEARCH/architecture.md) **ADR-009** (undo/redo) and **ADR-010** (state management, TD-01).

For full architecture, ADRs, and as-built module list, see:

- [RESEARCH/architecture.md](../RESEARCH/architecture.md) — design authority, data flow, mesh generation, Python bridge contract
- [docs/architecture.md](architecture.md) — user-facing architecture (when present)

---

## Tauri Commands (IPC Contract)

The frontend calls the Rust backend via `invoke('command_name', { ... })`. Typed helpers live in `src/lib/tauri.ts`. Command contracts (inputs/outputs) are documented below and in Rust doc comments; TypeScript types in `tauri.ts` match the backend.

| Command | Input | Output | Description |
|---------|--------|--------|-------------|
| `load_image` | `{ path: string }` | `LoadImageOut` (ok, width, height, fileSizeBytes, downsampled, previewBase64) | Load and validate image; returns dimensions and base64 preview. |
| `generate_depth_map` | `{ path: string }` | `{ width, height, depth: number[], progress, stages }` | Run AI depth estimation; stores depth in app state. **Also emits** `depth-progress` Tauri events with `{ percent, stage? }` during execution for real-time progress bar (Sprint 2.4). |
| `get_depth_map` | — | `{ width, height, depth } \| null` | Current depth map with adjustments applied. |
| `get_depth_adjustment_params` | — | `DepthAdjustmentParams` | Current brightness, contrast, gamma, invert, depthMinMm, depthMaxMm. |
| `set_depth_adjustment_params` | `{ params: DepthAdjustmentParams }` | `void` | Set adjustment params; next get_depth_map uses them. |
| `reset_depth_adjustments` | — | `void` | Reset params to defaults; original depth unchanged. |
| `undo` | — | `UndoRedoResult` (success, current params, can_undo, can_redo) | Pop last command, restore previous state; frontend updates UI from result. |
| `redo` | — | `UndoRedoResult` | Re-execute last undone command; frontend updates UI from result. |
| `clear_history` | — | `void` or result | Clear undo/redo stacks (e.g. on new image). Optional. |
| `get_mesh_data` | `{ preview_step?: number, target_width_mm?: number, target_height_mm?: number }` (all optional) | `MeshData \| null` | Mesh positions/normals (and optional indices) in mm for 3D preview. |
| `export_stl` | `{ path: string, target_width_mm?: number, target_height_mm?: number }` | `void` | Export triangulated mesh as binary STL. Path validated (SEC-401, SEC-402). |
| `export_obj` | `{ path: string, target_width_mm?: number, target_height_mm?: number }` | `void` | Export as ASCII OBJ (+ MTL). Same path validation as export_stl. |
| `get_export_defaults` | — | `{ suggestedFilename, lastExportDir?, exportFormat? }` | Suggested filename and last export directory for save dialog. |
| `get_settings` | — | `AppSettings` | Persisted settings (depth range, target dimensions, window size, etc.). |
| `save_settings` | `{ newSettings: AppSettings }` | `void` | Persist settings to disk. |
| `check_model` | — | `ModelStatus` (installed, modelDir, modelId, missingFiles, sizeMb?) | Whether AI model is installed. |
| `get_model_info` | — | `ModelInfo` (modelId, modelDir, license, estimatedSizeMb, description) | Model metadata for UI. |
| `download_model` | — | `DownloadResult` (status, modelDir?, sizeMb?, error?) | Download AI model (e.g. from Hugging Face). |
| `list_builtin_presets` | — | `string[]` | Return ordered list of built-in preset IDs (Portrait, Landscape, High Detail, Low Relief). |
| `list_presets` | — | `string[]` | Return user preset names from `~/.simplepicture3d/presets/`. |
| `save_preset` | `{ name: string, path?: string }` | `void` | Save current depth params + curve as a named preset (to presets dir) or to an explicit path (export). |
| `load_preset` | `{ nameOrPath: string }` | `void` | Apply a preset by name (user or built-in) or by absolute path (import); updates app depth state. |
| `delete_preset` | `{ name: string }` | `void` | Delete a user preset by name. |
| `rename_preset` | `{ oldName: string, newName: string }` | `void` | Rename a user preset; both names sanitized to prevent path traversal. |

**Frontend API:** Use the typed functions in `src/lib/tauri.ts` (e.g. `loadImage(path)`, `generateDepthMap(path)`, `getMeshData(options)`, `exportStl(path, options)`). They map to the commands above with camelCase args where required.

---

## Generating Rust API Documentation

The Rust backend is documented with doc comments (`///` and `//!`). You can generate and view the API docs locally.

### Generate docs

From the project root:

```bash
cargo doc --no-deps --manifest-path src-tauri/Cargo.toml --open
```

Or from `src-tauri`:

```bash
cd src-tauri
cargo doc --no-deps --open
```

- `--no-deps` builds documentation only for the SimplePicture3D crate (faster; no dependency docs).
- `--open` opens the generated HTML in your default browser after the build.

### View without opening

```bash
cargo doc --no-deps --manifest-path src-tauri/Cargo.toml
```

Output is under `src-tauri/target/doc/`. Open `src-tauri/target/doc/simplepicture3d_lib/index.html` in a browser.

### Key modules

- `lib` — Tauri commands, app state, export path validation
- `mesh_generator` — MeshParams, MeshData, depth_to_point_cloud, triangulation, STL/OBJ writers
- `depth_adjust` — DepthAdjustmentParams, apply_adjustments
- `image_loading` — load_image_impl, read_image_bytes_for_depth
- `python_bridge` — run_depth_estimation, DepthMapOutput
- `settings` — AppSettings load/save
- `preset` — Preset schema, built-in presets, sanitize_preset_name (Sprint 2.3)

---

## Testing and Linting

See [CONTRIBUTING.md](../CONTRIBUTING.md) and [CLAUDE.md](../CLAUDE.md) for the full list. For a repeatable **manual testing checklist** (core workflow, export verification, settings, target dimensions), see [docs/testing.md](testing.md). Summary:

| Stack | Test | Lint / format |
|-------|------|----------------|
| Rust | `cargo test --manifest-path src-tauri/Cargo.toml` | `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`; `cargo fmt --check` |
| Frontend | `npm test` | `npm run lint` |
| Python | `SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v` (see CLAUDE.md for Windows) | black, flake8/ruff |

Run tests before submitting a PR.

---

## Cross-platform Considerations

SimplePicture3D targets Windows (MVP), macOS, and Linux. Code that handles file paths must work correctly on all three platforms.

### Path separator normalisation

Windows uses `\` as the path separator; macOS and Linux use `/`. Paths arriving from user input, Tauri IPC, or the Python bridge may use either separator depending on where they originated.

**Rust:** Always construct and decompose paths with `std::path::Path` or `PathBuf` — never split or join path strings manually on `/` or `\`. When a path string is received from an external source (e.g. a Tauri IPC call or a test fixture using Windows-style paths), normalise it before processing:

```rust
// Normalise a received path string before passing to Path
let normalised: String = raw_path.chars().map(|c| if c == '\\' { '/' } else { c }).collect();
let path = Path::new(&normalised);
```

**Python:** Use `pathlib.Path` throughout. When producing path strings for the Rust backend (e.g. as JSON output from the depth-estimation subprocess), emit them with `Path(p).as_posix()` so forward slashes are used on all platforms.

**TypeScript / Frontend:** File paths from Tauri dialog pickers are OS-native strings. Do not manipulate path strings on the frontend; pass them as-is to Rust IPC commands and let the Rust side normalise with `PathBuf::from()`.

**Tests:** Avoid hard-coded path literals like `"C:\\photos\\file.png"` — they fail on Linux CI because `std::path::Path` does not split on `\` there. Construct paths portably:

```rust
// Portable path construction in tests
let path = Path::new("photos").join("file.png");
```

See [RESEARCH/GOTCHAS.md](../RESEARCH/GOTCHAS.md) for a concrete example of a CI failure caused by this pattern (2026-02-28 entry).

---

## Contribution Guidelines

We welcome contributions. Please read and follow:

- **[CONTRIBUTING.md](../CONTRIBUTING.md)** — Development setup, coding standards (Rust, Python, Svelte/TypeScript), testing, pull request process, and community guidelines.

Highlights:

- **Rust:** `cargo fmt`, `cargo clippy`, doc comments for public APIs.
- **Python:** black, type hints, docstrings (NumPy style).
- **Frontend:** Prettier, ESLint, TypeScript.
- **Commits:** Conventional Commits (e.g. `feat:`, `fix:`, `docs:`).

For architecture decisions and task ownership, see [todo.md](../todo.md) and [RESEARCH/AI_DEVELOPMENT_GUIDE.md](../RESEARCH/AI_DEVELOPMENT_GUIDE.md).
