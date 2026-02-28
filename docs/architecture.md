# SimplePicture3D Architecture

**Purpose:** Technical architecture and system design for SimplePicture3D.  
**Source:** `prd.md` §5.2, §5.3, §5.4; `RESEARCH/architecture.md`  
**Audience:** Developers, contributors, technical reviewers

---

## Overview

SimplePicture3D is a Tauri desktop application with a Rust backend, Svelte/React frontend, and a Python subprocess for AI depth estimation. All processing runs locally (100% offline).

**Architecture decisions** are recorded as ADRs in [RESEARCH/architecture.md](../RESEARCH/architecture.md): ADR-001 (Svelte), ADR-002 (Subprocess), ADR-003 (Python distribution), ADR-004 (Depth models), ADR-005 (Model licensing), ADR-006 (Mesh generation), ADR-008 (Grid triangulation for STL), ADR-009 (Target dimensions for laser etching).

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
│  │  • STL/OBJ export (custom writers in mesh_generator.rs)       │ │
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

## Mesh generation (algorithm) — Sprint 1.6

*Documentation Specialist: algorithm summary for developers. Full design: [RESEARCH/architecture.md](../RESEARCH/architecture.md) § Mesh Generation (Sprint 1.6) and ADR-006.*

**What it does:** The Rust backend converts the adjusted depth map (row-major, 0–1 normalized) into a **point cloud** with positions and normals in **millimeters**, suitable for Three.js preview and future STL/OBJ export.

**Algorithm (ARCH-201, BACK-501–506):**

- **Sampling:** Uniform grid over the depth map. Step size in X/Y (e.g. 1 = full resolution, 2 = every 2nd pixel). Vertex count = `ceil(width/step_x) × ceil(height/step_y)`.
- **Coordinates:** X/Y from pixel indices scaled by a configurable factor (e.g. 1 pixel = 1 mm). Z from normalized depth mapped to a configurable range (e.g. 2–10 mm): `z_mm = min_mm + depth * (max_mm - min_mm)`.
- **Normals:** Derived from depth gradient (finite difference in X/Y), unit length, for shading and STL.
- **Output:** `MeshData` with `positions`, `normals`, and optional `indices` (grid triangulation per ADR-008 for STL/OBJ). Target physical size (ADR-009) is supported via optional `target_width_mm` / `target_height_mm` in settings and commands.

**Where to look:**

| Topic | Location |
|-------|----------|
| Design (algorithm, vertex format, topology, memory) | [RESEARCH/architecture.md](../RESEARCH/architecture.md) § Mesh Generation, ADR-006 |
| Implementation | `src-tauri/src/mesh_generator.rs` (`depth_to_point_cloud`, `MeshParams`, `MeshData`) |
| IPC | Tauri command `get_mesh_data` (see Key Interfaces below) |

**Constraints:** 2.5D only (single Z per (x,y)); no overhangs. Input validated (dimensions, length); max dimension 8192. Memory and scaling notes in RESEARCH/architecture.md § Memory Efficiency (ARCH-204).

---

## Key Interfaces

### Tauri Commands (Rust ↔ Frontend)

| Command           | Input                 | Output              |
|-------------------|-----------------------|---------------------|
| `load_image`      | File path             | Image metadata (width, height, fileSizeBytes, downsampled) + base64 preview PNG for UI |
| `generate_depth_map` | Image path (string) | `{ width, height, depth: number[], progress?, stages? }` or error (see § Sprint 1.4 command contract) |
| `get_depth_map`   | —                     | Current depth map (adjusted by stored params) or `null` |
| `set_depth_adjustment_params` | `DepthAdjustmentParams` | — |
| `get_depth_adjustment_params` | —                  | Current params (brightness, contrast, gamma, invert, depthMinMm, depthMaxMm) |
| `reset_depth_adjustments` | —                  | — (params set to defaults) |
| `get_mesh_data`   | —                     | Vertex/normal data  |
| `export_stl`      | Path, mesh data       | Success/error       |
| `export_obj`      | Path, mesh data       | Success/error       |
| `download_model`  | Model ID              | Progress/result     |

### Depth map adjustments (Sprint 1.5)

Adjustments are applied in the backend. Order of operations: **invert → gamma → contrast → brightness**. All operations work on normalized depth [0, 1]; output is clamped to [0, 1]. Formulas: brightness `v' = clamp(v + b)`, contrast `v' = clamp((v - 0.5)*c + 0.5)`, gamma `v' = v^g` (0 stays 0). Original depth is stored unchanged; `get_depth_map` returns the result of applying current params to the original. Range params `depthMinMm`/`depthMaxMm` (e.g. 2–10 mm) are stored for future mesh/export; mapping `z_mm = min_mm + v*(max_mm - min_mm)`.

### Python Interface (Rust ↔ Python)

- **Input:** Image via temp file path or stdin
- **Output:** Depth map via stdout (JSON) or output file (binary)
- **Spawn:** Shell plugin / sidecar; capabilities configured in Tauri v2

---

## Rust–Python Bridge (Sprint 1.3)

*This section defines the IPC contract and data formats for the depth-estimation subprocess. Agreed by System Architect for implementation by Senior Engineer and Researcher.*

### ARCH-101: IPC Protocol (CLI Contract)

**Invocation**

- Rust spawns the Python depth estimator as a **subprocess** (no shell: `std::process::Command` or Tauri shell plugin with fixed scope).
- **Command:** `python -m python.depth_estimator` (or explicit script path, e.g. `python python/depth_estimator.py`).
- **Working directory:** Application or repo root so that `python` resolves to the intended venv/interpreter.
- **Environment:** Rust sets `VIRTUAL_ENV` (or `PATH`) so the correct Python and dependencies are used; no user-controlled env vars in the command.
- **Arguments (fixed contract):** Only validated, app-controlled values. Image input is **not** passed as a raw CLI argument (no user-controlled strings in argv). See ARCH-102 for how image data is passed.

**Input (image to Python)**

- **Mechanism:** Temp file path **or** stdin (see ARCH-102 decision).
- **Temp file option:** Rust writes image bytes to a temp file in the system temp dir (using `file_io::write_temp_file`), then passes a single argument such as `--input <path>`. The path must be canonicalized and under the temp dir; Python reads the file (Rust cleans up the temp file after use).
- **Stdin option:** Rust streams raw image bytes (PNG/JPEG) to the process stdin; Python reads from stdin until EOF. No path in argv.
- **Format:** PNG or JPEG bytes (as already validated by `load_image`). No base64 in the protocol.

**Output (depth map from Python)**

- **Mechanism:** Stdout **or** output file (see ARCH-102).
- **Stdout:** Depth map as JSON (see below) or binary block; progress/human messages must go to stderr only.
- **Output file option:** Rust passes `--output <path>` (path in app temp dir); Python writes depth map to that file and exits. Stdout reserved for progress lines (stderr or a single stdout channel with a defined line protocol).
- **Depth map format:** Either:
  - **JSON:** A single JSON object on stdout (or in file), e.g. `{"height": H, "width": W, "depth": [f32,...]}` with row-major `depth` array, values normalized 0–1. Or flat array + dimensions. Exact schema documented in ARCH-102.
  - **Binary:** Raw float32 array, row-major, little-endian; dimensions passed via separate args or a small JSON header. Documented for Rust parser (BACK-203).

**Progress**

- Python emits progress to **stderr** only (so stdout can be dedicated to depth data).
- Line-based protocol, e.g. `PROGRESS 25` (integer 0–100). Optional: `STAGE loading_model`, `STAGE inference`, etc. Agreed with BACK-205 / AI-203.

**Errors**

- Python exits with non-zero on failure; writes human-readable error to stderr. Rust captures stderr and surfaces it (BACK-204). No success payload on stdout when exit code ≠ 0.

---

### ARCH-102: Data Format Decision

**Image input (Rust → Python)**

| Option | Pros | Cons |
|--------|------|------|
| **Temp file (path)** | No stdin buffer limits; works for 4K (~12MB); reuses `file_io.rs`; path validated and scoped | Disk I/O; cleanup required |
| **Stdin (raw bytes)** | No temp file; simple for small images | Platform stdin buffer limits (e.g. ~1–2MB on some systems); risk for 4K |
| **Base64 JSON** | Single channel | Large size (~33% overhead); decode time; not chosen |

**Decision:** **Temp file for image input.** Rationale: (1) 4K images (~12MB RGB) exceed comfortable stdin limits on some platforms. (2) `file_io::write_temp_file` already exists and is scoped to system temp dir; path can be canonicalized and validated before passing to Python (threat model §2.5). (3) RESEARCH/python-ml.md recommends file-based for large images. (4) Simplicity: one clear contract (`--input <path>`), no streaming protocol.

**Depth map output (Python → Rust)**

| Option | Pros | Cons |
|--------|------|------|
| **JSON (flat array + dimensions)** | Easy to parse in Rust (serde); debuggable; single stream | Size for 4K (4K×4K ≈ 64M floats → large JSON) |
| **Binary (raw float32)** | Compact; fast to parse; fixed layout | Requires agreed byte order and dimensions (e.g. header or args) |

**Decision for MVP (Sprint 1.3):** **JSON** for depth output. Schema: `{"height": u32, "width": u32, "depth": [f32,...]}` with row-major `depth`, values in [0, 1]. Rationale: (1) Simplicity and debuggability for first integration. (2) Rust parser is straightforward (serde_json). (3) If performance requires it later, we can add a binary format (e.g. raw float32 + small JSON header) without changing the CLI contract. (4) For 4K, consider chunked or file-based transfer if JSON size becomes an issue (RESEARCH/tauri.md: large payloads); for 1.3 we accept JSON on stdout or to a file.

**JSON stdout contract (AI-302):** Single JSON object on stdout. Keys: `height`, `width`, `depth`. The `depth` array length must equal `height × width`. Layout is **row-major**: row index `y`, column index `x` → linear index `y * width + x`. Rust (`DepthMapOutput`) and frontend consume this format; Python validates shape before emitting.

**Depth map format (AI-304, reference for UI and mesh)**

Single source of truth for depth map data consumed by frontend (depth preview, grayscale canvas) and by the future mesh pipeline. See also SPRINTS/Sprint_1_3/UI_READINESS_1_4.md and Sprint 1.4 task assignment.

| Field    | Type            | Description |
|----------|-----------------|-------------|
| `width`  | `u32` / number  | Width of depth map (pixels). |
| `height` | `u32` / number  | Height of depth map (pixels). |
| `depth`  | `Vec<f32>` / number[] | Flat array; length = `width × height`. **Row-major:** index for pixel (x, y) = `y * width + x`. Values in **[0, 1]** (0 = near/min, 1 = far/max). |

**Example (2×2):** `{"width": 2, "height": 2, "depth": [0.0, 0.5, 0.5, 1.0]}` → row 0: [0.0, 0.5], row 1: [0.5, 1.0].

**Summary**

- **Image:** Temp file path only (`--input <path>`); path validated, under temp dir.
- **Depth:** JSON object `{ "height", "width", "depth" }` on stdout or to output file; 0–1 normalized, row-major.

---

### ARCH-103: Subprocess vs PyO3

| Criterion | Subprocess | PyO3 (embed Python in process) |
|-----------|------------|--------------------------------|
| **Isolation** | Process boundary; Python crash does not crash Rust | Same process; Python crash or GIL issues can affect app |
| **Environment** | Uses system/venv Python; familiar for ML | Must bundle or discover Python; version coupling |
| **Packaging** | User installs Python + venv; or sidecar binary (PyInstaller) | Complex: embed interpreter or link libpython |
| **Tauri capabilities** | Shell plugin / Command; no special capability for Python libs | No Tauri-specific issue |
| **Performance** | Spawn overhead; IPC (file/stdin) | No spawn; direct in-process call |
| **Security** | Easier to restrict (temp dir, no user argv); threat model §2.5 | Same process; must still validate inputs |
| **PRD / RESEARCH** | prd.md §5.3: "Python subprocess"; RESEARCH/tauri.md: shell/sidecar | prd.md §5.2: "PyO3/subprocess" as options |

**Decision:** **Subprocess.** Rationale: (1) PRD §5.3 and RESEARCH explicitly describe subprocess and temp file/stdin. (2) Threat model §2.5 and SEC-201 assume subprocess with fixed CLI and validated paths. (3) Process isolation and simpler packaging (venv or sidecar) favor subprocess for MVP. (4) If profiling later shows spawn/IPC as a bottleneck, we can re-evaluate PyO3 or a long-lived Python worker process; no change to the high-level protocol (image in → depth out).

---

### ARCH-104: Data Flow (Image → Python → Depth Map)

```
┌─────────────────────────────────────────────────────────────────────────┐
│  Rust Backend                                                            │
│                                                                          │
│  load_image (or caller)     file_io::write_temp_file(image_bytes, ".png")│
│         │                                    │                           │
│         └────────────────────────────────────┼───────────────────────────┤
│                                              ▼                           │
│                              ┌──────────────────────────────┐            │
│                              │  Temp file (system temp dir) │            │
│                              │  path canonicalized, scoped   │            │
│                              └──────────────────┬───────────┘            │
│                                                 │                        │
│  spawn: python -m python.depth_estimator --input <path>                   │
│                                                 │                        │
└─────────────────────────────────────────────────┼────────────────────────┘
                                                  │
                    subprocess (no shell; argv = fixed + validated path only)
                                                  │
┌─────────────────────────────────────────────────▼────────────────────────┐
│  Python AI Backend                                                        │
│  • Read image from --input path                                           │
│  • Load model (~/.simplepicture3d/models/ or env)                         │
│  • Run inference (Depth-Anything-V2 / MiDaS)                              │
│  • Write progress to stderr (PROGRESS n, STAGE name)                      │
│  • Write depth map to stdout (JSON): {"height", "width", "depth": [...]}  │
│  • Exit 0 on success; non-zero + stderr message on error                  │
└─────────────────────────────────────────────────┬────────────────────────┘
                                                  │
                    stdout (JSON) │ stderr (progress/errors)
                                                  │
┌─────────────────────────────────────────────────▼────────────────────────┐
│  Rust Backend                                                             │
│  • Capture stdout → parse JSON depth map                                  │
│  • Capture stderr → progress log / user-facing error                      │
│  • Validate dimensions (match input w×h or documented downsampling)      │
│  • Return Vec<f32> or equivalent to caller (future generate_depth_map)    │
│  • Clean up temp input file                                               │
└──────────────────────────────────────────────────────────────────────────┘
```

This flow is consistent with ARCH-101 (CLI contract), ARCH-102 (temp file input, JSON depth output), and ARCH-103 (subprocess).

### Sprint 1.4: generate_depth_map command contract (API approval)

*Approved by System Architect for implementation by Senior Engineer and consumption by UI. Single source of truth for BACK-301, BACK-303, UI-301–305.*

**Command:** `generate_depth_map`

| Aspect | Contract |
|--------|----------|
| **Input** | `path: string` — Filesystem path to the image file (already loaded/validated by frontend via `load_image` or equivalent). Path must be validated by Rust (exist, readable, under allowed dirs); no user-controlled strings in subprocess argv (ARCH-101). *Future:* optional base64/bytes overload for environments where path is unavailable. |
| **Success response** | `{ width: number, height: number, depth: number[] }` — Depth map dimensions and flat float array, **row-major**, values in **[0, 1]** (ARCH-102). Same format as Python stdout contract. For MVP, returned in command response; payload size acceptable for typical resolutions (consider documented limit or chunking for 4K×4K if needed). |
| **Error response** | `{ error: string }` (or Tauri `Result::Err` with message). Messages must be UI-suitable: e.g. "Python not found", "Depth estimation timed out", "Invalid image", "Missing model". No stack traces or raw stderr in production response. |
| **Progress (MVP)** | **Return on completion.** Command blocks until Python exits; response includes full depth map. Frontend shows **indeterminate progress** (spinner) while the command is in flight. Optional: include `progress: 100` and `stages?: string[]` in success response for consistency. |
| **Progress (future)** | Optional enhancement: Tauri events `depth-progress` with payload `{ percent: number, stage?: string }` emitted during inference; UI can show determinate progress bar. Not required for Sprint 1.4 exit criteria. |

**TypeScript (frontend):** Define types in `src/lib/tauri.ts` for the success shape (`DepthMapResponse`) and error handling so UI and DepthMapPreview share the same contract.

**State:** After successful generation, backend may store depth in app state (BACK-302) for a future `get_depth_map` or mesh pipeline; frontend for 1.4 uses the command return value directly for preview.

---

### Frontend implications (depth pipeline)

When the `generate_depth_map` Tauri command is exposed (Sprint 1.4), the frontend will consume the following. This contract aligns BACK-204/BACK-205 with UI design (prd.md F1.2, §6). **Exact command API:** see "Sprint 1.4: generate_depth_map command contract" above.

| Concern | Source | UI use |
|--------|--------|--------|
| **Progress** | Rust parses Python stderr (`PROGRESS n`, optional `STAGE name`) per BACK-205 / AI-203 | Progress indicator during inference (F1.2: "Display progress indicator"); MVP: spinner while command in flight; optional later: determinate bar via events |
| **Errors** | Rust captures Python stderr and exit code; returns `Result` with user-facing message (BACK-204) | Status bar or inline error message; no panic; clear copy for missing Python, missing model, timeout, OOM |
| **Result** | Depth map as structured data (dimensions + flat float array 0–1) from Rust; schema per command contract above | Depth preview (canvas/grayscale), mesh preview when implemented, depth controls (sliders, invert) |
| **Timeout** | Rust enforces timeout and kills subprocess (BACK-204) | Show "Depth estimation timed out" or similar; allow retry |

**UI states for depth flow (Sprint 1.4):** idle → estimating (progress %) → depth ready | error. Right sidebar (depth controls) and status bar will show progress and errors; center panel will show 3D preview once mesh data is available.

---

### Sprint 1.4: generate_depth_map command contract

*Agreed by System Architect for implementation by Senior Engineer and UI Designer. Single source of truth for BACK-301–304 and UI-301–305.*

#### Command API

| Aspect | Decision | Notes |
|--------|----------|--------|
| **Command name** | `generate_depth_map` | Registered in Tauri; permission e.g. `allow-generate-depth-map`. |
| **Input** | **Image path** (string) | MVP: frontend passes path of the currently loaded image (same path returned by file picker / used for `load_image`). Rust validates path, re-reads file (or reuses load_image validation), writes temp file, calls `python_bridge::run_depth_estimation`. No raw bytes/base64 in command for 1.4. |
| **Input validation** | Path must exist, readable, and safe | No command injection; temp file scoped to system temp dir per ARCH-102. Reject non-image or unreadable paths with clear error. |
| **Output (success)** | Structured depth map (see Depth map data format below) | Return dimensions + depth array in one response for MVP. For very large (e.g. 4K×4K) consider size limits or chunking if IPC is slow; document in GOTCHAS if needed. |
| **Output (error)** | User-facing message string | Missing Python, timeout, invalid image, OOM — suitable for UI status bar or error area. |

#### Progress protocol (BACK-304, UI-304)

| Option | Decision for MVP | Implementation |
|--------|------------------|----------------|
| **Event vs return** | **Return on completion** | Command blocks until Python finishes. No live Tauri events for 1.4. |
| **Progress in response** | Include `progress: 100` and optional `stages: string[]` in success payload | Rust already parses stderr (`PROGRESS n`, `STAGE name`); include final progress and list of stages in the command response so UI can show "Complete". |
| **During execution** | Frontend shows **indeterminate progress** (spinner + "Estimating…") | No real-time percent until command returns. UI disables Generate button and shows spinner; on return, show depth or error. |
| **Future** | Tauri events `depth-progress` with `{ percent, stage? }` | If backend runs bridge in a spawned task and emits via `app.emit()`, frontend can subscribe and show 0–100% bar. Deferred post-1.4. |

#### Depth map data format (frontend + backend)

Consumed by frontend (depth preview, grayscale canvas) and by future mesh pipeline. Single schema for Rust serialization and TypeScript types.

| Field | Type | Description |
|-------|------|-------------|
| `width` | `u32` / `number` | Width of depth map (pixels). |
| `height` | `u32` / `number` | Height of depth map (pixels). |
| `depth` | `Vec<f32>` / `number[]` | Row-major flat array; length = `width × height`. Index `y * width + x`. Values in **[0, 1]** (0 = near/min, 1 = far/max). |

**Row-major order:** Row 0 (y=0), then row 1 (y=1), etc. Same as Python `depth_estimator` stdout JSON and Rust `DepthMapOutput`.

**TypeScript (e.g. `src/lib/tauri.ts`):**

```ts
export interface DepthMapResult {
  width: number;
  height: number;
  depth: number[];
  /** Present when command returns on completion; 100 when success. */
  progress?: number;
  /** Optional stages from Python stderr (e.g. "loading_model", "inference"). */
  stages?: string[];
}
```

**Rust:** Use existing `DepthMapOutput` (or equivalent) for serialization; add `progress` and `stages` to the command response struct if returning on completion.

**Size:** For 1920×1080, `depth` has ~2M floats (~8 MB). For 4K×4K, ~64M floats (~256 MB). Tauri IPC handles this for MVP; if issues arise, document in GOTCHAS and consider `get_depth_map` (read from app state) or chunking in a future sprint.

#### Summary for implementers

- **Senior Engineer:** Implement `generate_depth_map(path: String)`; validate path; temp file → bridge; return `{ width, height, depth, progress: 100, stages? }` or error. Register command and permission. Optionally store result in app state (BACK-302) and/or return inline (BACK-303).
- **UI Designer:** Call `invoke('generate_depth_map', { path })` with current image path; show spinner while pending; on success render `depth` as grayscale (UI-301/302); on error show message. Progress bar can be indeterminate for 1.4; optional label "Estimating…".

---

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
