# SimplePicture3D Architecture

**Purpose:** System architecture and data flow for SimplePicture3D.

**Source:** Derived from `prd.md` §5.2, §5.3, §5.4.  
**Last checked:** 2026-02-06

---

## Architecture Decision Records (ADRs)

*Added 2026-02-06 per External Consultant Recommendations Report.*

### ADR-001: Svelte over React

**Status:** Accepted
**Date:** 2026-02-06
**Context:** PRD §5.1 listed "Svelte or React" as options. A decision was needed.

**Decision:** Use **Svelte 4** with TypeScript for the frontend.

**Rationale:**
1. **Smaller bundle size** — Svelte compiles to vanilla JS; no runtime library overhead
2. **Simpler state management** — Svelte stores vs Redux/Zustand complexity
3. **Better Tauri integration** — Svelte's reactivity model works well with Tauri's invoke pattern
4. **Learning curve** — Easier onboarding for contributors unfamiliar with React patterns
5. **Performance** — Compiled output is faster for our use case (real-time slider updates)

**Consequences:**
- React components/libraries not directly usable; must find Svelte equivalents
- Three.js integration via `svelte-cubed` or direct imperative code
- Team should document Svelte patterns in RESEARCH/frontend.md

---

### ADR-002: Subprocess over PyO3 for Python Bridge

**Status:** Accepted
**Date:** 2026-02-06
**Context:** PRD mentioned PyO3 and subprocess as options for Rust-Python integration.

**Decision:** Use **subprocess** (via Tauri shell plugin) for MVP.

**Rationale:**
1. **Process isolation** — Python crash does not crash the Tauri app
2. **Simpler packaging** — User installs Python + venv; no embedded interpreter complexity
3. **Security** — Easier to restrict subprocess to temp dir; fixed CLI contract
4. **Debugging** — Can run Python script standalone for testing
5. **PRD alignment** — prd.md §5.3 explicitly describes subprocess approach

**Trade-offs:**
- Spawn overhead (~100-200ms per invocation)
- IPC via file/stdout (acceptable for depth maps)
- No in-process memory sharing

**Future consideration:** If profiling shows IPC bottleneck, evaluate:
- Long-lived Python worker process (subprocess with stdin/stdout loop)
- ONNX Runtime in Rust (eliminates Python dependency)

---

### ADR-003: Python Distribution Strategy

**Status:** Accepted
**Date:** 2026-02-06
**Context:** Need to define how Python + PyTorch + models are distributed to end users.

**Decision (MVP):** **System Python** — Require users to install Python 3.10+ with pip.

**Implementation:**
1. Document Python requirements in README.md and first-run wizard
2. Provide `pip install -r python/requirements.txt` instructions
3. App checks for Python availability on startup; shows helpful error if missing
4. Model download wizard (Sprint 1.10) handles Hugging Face model installation

**Rationale:**
- Lowest implementation effort for MVP
- Users with laser engravers typically have technical comfort with installations
- Avoids 200-500MB bundle size increase from embedded Python

**Future options (v1.1+):**

| Option | Effort | Trade-off |
|--------|--------|-----------|
| PyInstaller sidecar | Medium | Self-contained but 200-500MB |
| ONNX Runtime in Rust | High | Eliminates Python; requires model conversion |
| Docker container | Medium | Isolation but requires Docker installation |

**Consequences:**
- First-run experience requires Python setup
- Cross-platform testing must verify Python availability detection
- Document troubleshooting for common Python issues (venv, PATH, etc.)

---

### ADR-004: Depth Model Selection

**Status:** Accepted
**Date:** 2026-02-06
**Context:** Need to select primary depth estimation model and address licensing concerns.

**Decision:** Support **both** Depth-Anything-V2 (default) and MiDaS (alternative).

**License consideration:**
- **Depth-Anything-V2 weights:** CC-BY-NC-4.0 (non-commercial only)
- **MiDaS weights:** MIT-compatible for commercial use

**Implementation:**
1. Default to Depth-Anything-V2 for best quality
2. Offer MiDaS as "Commercial-friendly" option in model download wizard
3. Document license implications in first-run wizard and README
4. `--model` flag in depth_estimator.py already supports switching

**Consequences:**
- Users intending commercial use should select MiDaS
- Documentation must clearly state license restrictions
- Consider training/licensing permissive model for v1.1 commercial release

---

### ADR-005: Depth Model Licensing and Commercial Use

**Status:** Accepted  
**Date:** 2026-02-07  
**Context:** Consultant Report §1.2, §4 Priority 6. CC-BY-NC-4.0 on Depth-Anything-V2 weights conflicts with commercial use cases. A formal decision and user-facing documentation are required.

**Current situation:**
- **Depth-Anything-V2 weights:** CC-BY-NC-4.0 (non-commercial only). Best quality; default in the app.
- **MiDaS weights:** MIT-compatible; commercial use permitted. Repository archived; code and weights remain usable.
- **SimplePicture3D application:** MIT license. App code does not restrict use; model weights have their own licenses.

**Options considered:**

| Option | Description | Decision |
|--------|-------------|----------|
| **A** | Non-commercial only — document prominently, restrict to hobbyist use | Rejected: too limiting; MiDaS offers a commercial path. |
| **B** | Offer MiDaS as commercial-friendly default; Depth-Anything-V2 as optional higher-quality non-commercial model | **Accepted** (aligns with ADR-004). |
| **C** | Dual model support — user chooses at install time; license shown in model download wizard | Implemented as part of B: wizard (Sprint 1.10) will surface choice and license. |
| **D** | Custom model with permissive license | Deferred; high effort; consider for v1.1+. |

**Decision:** **Option B (with C in the wizard).**

1. **Default:** Depth-Anything-V2 for best quality. Clearly document in UI and docs that it is **non-commercial only** (CC-BY-NC-4.0).
2. **Commercial use:** MiDaS offered as "Commercial-friendly" option in model download wizard and docs. Users who need commercial use must select MiDaS.
3. **User-facing:** README, user guide, and first-run / model-download flows must state:
   - Depth-Anything-V2: CC-BY-NC-4.0 — personal / non-commercial use only.
   - MiDaS: MIT-compatible — commercial use allowed.
4. **Model download wizard (Sprint 1.10):** Show license for each model at selection time; require acknowledgment for Depth-Anything-V2 (non-commercial). Plan: wizard step "Choose model" lists Depth-Anything-V2 (default, "Non-commercial only") and MiDaS ("Commercial use OK") with short license text or link.

**Rationale:** Balances quality (default) with commercial viability (MiDaS) without blocking either use case. ADR-004 already committed to supporting both models; this ADR formalizes licensing implications and user communication.

**Consequences:**
- AI-502: Python depth_estimator must emit license notice when real model is loaded; `--show-license` flag for tooling.
- DOC-503 / user-guide: Model license info visible to end users.
- Sprint 1.10: Model download wizard surfaces license and choice.

---

### ADR-006: Mesh Generation Algorithm (Sprint 1.6)

**Status:** Accepted  
**Date:** 2026-02-07  
**Context:** ARCH-201. Need to decide point cloud vs triangulated mesh and sampling strategy for depth map → 3D geometry.

**Decision:**

1. **MVP delivers both** point cloud and optional triangulated mesh.
   - **Primary path:** Point cloud — sufficient for Three.js preview (Sprint 1.7) and dimensionally accurate for export. Simpler to implement and validate.
   - **Optional path:** Delaunay triangulation (2.5D) for STL/OBJ with faces; enables solid mesh export and face normals. Can be implemented in Sprint 1.6 or deferred to a follow-up if point cloud is sufficient for MVP export (STL can be generated from point cloud or triangulated mesh per BACK-504).

2. **Sampling strategy: uniform grid.**
   - Depth map is row-major `height × width`. Sample at regular steps in X and Y.
   - **Grid step:** Configurable step size (e.g. 1 = full resolution, 2 = every 2nd pixel → 1/4 vertices). Vertex count = `ceil(width / step_x) × ceil(height / step_y)`.
   - **Density control:** Step size parameter (or equivalent) provides PRD F1.5 "density control"; full resolution and downsampled (e.g. step=2) must be supported.

3. **Relationship to depth map resolution:** One vertex per sample; (i, j) in grid → depth from `depth[i * width + j]`. No adaptive density in MVP; uniform grid only.

**Rationale:** PRD F1.5 requires "point cloud with density control" and "Optional Delaunay triangulation"; 2.5D (single Z per (x,y)) makes a regular grid natural and avoids overhangs. BACK-501–506 implement against this design.

**Consequences:** BACK-501 (point cloud), BACK-502 (uniform grid), BACK-504 (optional Delaunay) and JR2-501/502 tests follow this specification.

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

## Mesh Generation (Sprint 1.6)

*Design authority: System Architect. Implementation: BACK-501–506, referenced by Junior Engineer 2D and Security.*

### Algorithm and Sampling (ARCH-201)

- **Output:** Point cloud (required); optional triangulated mesh via Delaunay (2.5D). See ADR-006.
- **Sampling:** Uniform grid. Step size in X/Y (e.g. 1 pixel = 1 vertex, or step = 2 for 1/4 vertices). Vertex count = `ceil(width/step_x) × ceil(height/step_y)`.
- **Input:** Depth map: row-major `Vec<f32>` (or `&[f32]`), length `width × height`, values 0–1 normalized. Depth range (min_mm, max_mm) is user setting (e.g. 2–10 mm).

### Vertex Format (ARCH-202)

- **Position:** `(x, y, z)` in **millimeters** (f32). Required. X/Y from pixel indices scaled by configurable factor (e.g. 1 pixel = 1 mm); Z from normalized depth mapped to [min_mm, max_mm].
- **Normals:** **Required** for MVP. Format: `Vec<[f32;3]>`, unit length. For point cloud: derived from depth gradient (finite difference in X/Y). For triangulated mesh: per-face normals for STL; vertex normals (average of adjacent face normals) for OBJ and Three.js.
- **Color:** Not required for MVP. Omitted; can be added later if texture/heightmap color is needed.
- **Serialization (Tauri IPC):** Struct with flat arrays: `positions: Vec<[f32;3]>`, `normals: Vec<[f32;3]>`. Optional `indices: Vec<u32>` for triangulated mesh (triangle list). Compatible with Three.js `BufferGeometry` (setAttribute position, normal) and with `stl_io` (vertices + face normals).
- **Backend type:** `MeshData` (or equivalent): `positions`, `normals`, and optionally `indices`. All coordinates and normals in mm / unit length.

### Mesh Topology for Laser Engraving (ARCH-203)

- **No overhangs:** 2.5D representation guarantees a single Z per (x, y); the surface is a heightfield. Internal UV laser engraving is vertical; no undercuts.
- **2.5D:** Each (x, y) has exactly one depth value from the depth map; mesh is a continuous surface over the XY plane.
- **Manifold / watertight:** For triangulated mesh, 2.5D grid plus Delaunay in the (x, y) plane with Z from vertices yields a continuous surface. Ensure no degenerate triangles and consistent winding for STL/OBJ.
- **Constraints:** Minimum feature size and vertical wall limits are machine-dependent; the mesh is dimensionally accurate in mm and topologically 2.5D. Export (Sprint 1.8) consumes this structure; BACK-504 and STL/OBJ export reference these requirements.

### Memory Efficiency (ARCH-204)

- **4K (3840×2160):** ~8.3M pixels. Full resolution: 8.3M vertices × (3+3)×4 bytes ≈ 200 MB (positions + normals). Depth map ≈ 33 MB. Total well under PRD <2 GB for 4K.
- **8K (7680×4320):** ~33M vertices × 24 bytes ≈ 800 MB (positions + normals); with depth and working memory, stay within 16 GB.
- **Recommendation:** **Single buffer** for MVP. No streaming or chunking required for 4K/8K at these sizes. Avoid redundant copies: build positions and normals in one pass; depth map is read-only. If vertex count exceeds ~50M or memory pressure appears, consider chunked generation or LOD for preview only; document in BACK-506.
- **Scaling:** Vertex count = `(width/step) × (height/step)` (with ceiling); bytes ≈ 24 × vertex count. Document max input dimensions (e.g. 8192×8192) and any vertex cap to stay within budget. BACK-506 implementation follows this review.

---

## Key Interfaces

- **Tauri commands:** `load_image`, `generate_depth_map`, `get_mesh_data`, `export_stl`, `export_obj`, `download_model`
- **Python interface (Sprint 1.3):** See **docs/architecture.md** § "Rust–Python Bridge (Sprint 1.3)" for the full IPC contract:
  - **Image input:** Temp file path only (`--input <path>`); path validated, under system temp dir (ARCH-102).
  - **Depth output:** JSON `{"height", "width", "depth": [f32,...]}` to stdout (or file); 0–1 normalized, row-major.
  - **Invocation:** Subprocess (no shell); fixed CLI; progress on stderr (ARCH-101, ARCH-103).
- **Model storage:** `~/.simplepicture3d/models/`
- **Settings:** `~/.simplepicture3d/` (presets, logs, cache)

---

## Python Distribution Strategy

*Added 2026-02-06 per External Consultant Recommendations Report. See ADR-003 for decision.*

### MVP Approach: System Python

**Requirements:**
- Python 3.10 or later
- pip package manager
- ~2GB disk space for PyTorch + model weights

**User setup:**
```bash
# Create virtual environment (recommended)
cd python
python3 -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Download model (via app wizard or manual)
python -c "from transformers import AutoModelForDepthEstimation; AutoModelForDepthEstimation.from_pretrained('depth-anything/Depth-Anything-V2-Small-hf')"
```

**App behavior:**
1. On startup, check for Python: `python3 --version` or `python --version`
2. If missing: Show helpful dialog with installation links
3. Check for dependencies: Attempt import of torch, transformers
4. If missing: Show `pip install` instructions
5. Check for model: Look in `~/.simplepicture3d/models/`
6. If missing: Launch model download wizard (Sprint 1.10)

### Future: ONNX Migration Path (v1.1+)

To eliminate Python dependency:

1. Convert Depth-Anything-V2 to ONNX format
2. Use `ort` crate (ONNX Runtime for Rust)
3. Bundle ONNX model with installer (~200MB)
4. Remove Python subprocess bridge

**Benefits:**
- Single binary distribution
- Faster inference (no Python overhead)
- Simpler cross-platform packaging

**Effort:** ~2-3 sprints for implementation and testing

---

## Risk Mitigations

*Added 2026-02-06 per External Consultant Recommendations Report.*

| Risk | Mitigation |
|------|------------|
| Depth-Anything-V2 model unavailable | Mirror weights; document MiDaS alternative |
| Tauri v2 breaking changes | Pin version in Cargo.toml; monitor release notes |
| PyTorch version conflicts | Document tested versions; virtual environment isolation |
| Testing debt compounds | Address in Sprint 1.5; see consultant report Priority 1 |
| Python bundling complexity | System Python for MVP; ONNX for v1.1 |
