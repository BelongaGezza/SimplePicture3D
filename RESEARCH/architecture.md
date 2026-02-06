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
