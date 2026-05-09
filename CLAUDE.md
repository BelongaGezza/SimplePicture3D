# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SimplePicture3D is a desktop application that transforms 2D images into **volumetric 3D point clouds** for internal UV laser engraving of K9 crystal, glass, and acrylic. Uses AI-powered depth estimation with volumetric sampling to generate dense point distributions inside a user-specified crystal blank.

**Current Status:** Restarted 2026-05-10 as a single-purpose **3D surface point cloud** tool for internal UV laser engraving of crystal glass. 2.5D relief code (`mesh_generator.rs`, STL/OBJ) is being removed. The surface-map algorithm (ADR-012) is canonical — one laser dot per (x,y) sample at the depth-mapped Z position. Core infrastructure is working; volumetric backend wiring and frontend UI are the next sprints (B and C). See `todo.md` for the full sprint plan.

**Canonical algorithm reference:** `RESEARCH/architecture.md` **ADR-012** (surface-map point cloud)  
**Sprint plan:** `todo.md`  
**Critical:** Do NOT wire `volumetric.rs::generate_volumetric_points` until algorithm is rewritten per ADR-012 (TD-14, Sprint B — BACK-B-01).

## Repository Structure (Current)

```
SimplePicture3D/
├── .agents/                    # Agent personas for multi-agent development
├── .cursor/rules/              # Cursor IDE rules (reference .agents/*.md)
├── RESEARCH/                   # Technology guidance and knowledge base
│   ├── architecture.md         # System architecture, ADRs (ADR-011 is canonical)
│   ├── PIVOT_PLAN_2.5D_TO_3D.md # Transition roadmap from 2.5D to 3D
│   ├── 3d-reconstruction.md    # Full 3D reconstruction models (TripoSR, optional)
│   ├── AI_DEVELOPMENT_GUIDE.md # Multi-agent coordination patterns
│   ├── GOTCHAS.md              # Debugging findings (all agents contribute)
│   ├── rust-crates.md          # Rust dependency guidance
│   ├── python-ml.md            # Python/PyTorch/depth models
│   ├── tauri.md                # Tauri framework
│   ├── frontend.md             # Svelte/TypeScript
│   └── threejs.md              # Three.js 3D rendering
├── SPRINTS/                    # Sprint tasking templates and artefacts
├── src-tauri/                  # Rust backend
├── src/                        # Svelte frontend
├── python/                     # Python AI backend
├── prd.md                      # Product requirements (canonical spec)
├── todo.md                     # Sprint planning and task breakdown
├── CONTRIBUTING.md             # Contribution guidelines
└── README.md                   # Project overview
```

## Key Reference Documents

| Document | Purpose |
|----------|---------|
| `RESEARCH/architecture.md` | **ADR-011** is canonical for 3D volumetric architecture |
| `RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md` | Transition roadmap, component analysis, implementation phases |
| `prd.md` | Product requirements (updated for 3D pivot) |
| `todo.md` | Sprint planning, phase milestones, task IDs |
| `RESEARCH/3d-reconstruction.md` | TripoSR full 3D reconstruction (optional future) |
| `RESEARCH/GOTCHAS.md` | Known debugging pitfalls |

## Technology Stack

| Layer | Technologies | Research File |
|-------|--------------|---------------|
| Shell | Tauri | `RESEARCH/tauri.md` |
| Backend | Rust (image, tokio, serde, anyhow; custom PLY/XYZ/CSV writers) | `RESEARCH/rust-crates.md` |
| AI | Python, PyTorch, Depth-Anything-V2/MiDaS | `RESEARCH/python-ml.md` |
| Frontend | Svelte 4, TypeScript, TailwindCSS (ADR-001) | `RESEARCH/frontend.md` |
| 3D Preview | Three.js (point cloud + blank wireframe) | `RESEARCH/threejs.md` |

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

**E2E (Sprint 1.11):** Phase 1 uses a repeatable manual checklist (see `SPRINTS/Sprint_1_11/TEST_PLAN_1_11.md` and `MANUAL_TEST_REPORT.md`). Automated E2E (e.g. Playwright) is deferred to Phase 2.

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
| Quality Engineer | `.agents/quality-engineer.md` | Test plans, manual/automated QA, verification, E2E |
| Security Specialist | `.agents/security-specialist.md` | Security review, audits |
| Documentation Specialist | `.agents/documentation-specialist.md` | User/dev docs |

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                     Tauri Frontend                      │
│  Svelte 4 │ Three.js Preview │ Blank Wireframe          │
│  BlankSetup │ Depth Controls │ Export Panel             │
└───────────────────────┬─────────────────────────────────┘
                        │ Tauri Commands (IPC)
┌───────────────────────▼─────────────────────────────────┐
│                   Rust Backend                          │
│  Image loading │ Depth processing │ Volumetric sampling │
│  BlankEnvelope │ fit_to_blank │ PLY/XYZ/CSV export      │
│  Settings │ Undo/redo │ Python subprocess bridge        │
└───────────────────────┬─────────────────────────────────┘
                        │ subprocess (temp file → stdout)
┌───────────────────────▼─────────────────────────────────┐
│                  Python AI Backend                      │
│  Depth-Anything-V2 / MiDaS │ PyTorch                    │
│  Input: Image → Output: Depth map                       │
└─────────────────────────────────────────────────────────┘
```

## Data Flow: Image to Point Cloud

1. **Load image** (Frontend → Rust): File picker → `load_image` command
2. **Validate** (Rust): Format, size, downsample if >8K
3. **Define blank** (Frontend → Rust): User specifies crystal dimensions (L×W×H mm) + margin
4. **Depth estimation** (Rust → Python): Image bytes → subprocess → depth map
5. **Depth processing** (Rust): Adjustments (gamma, curves, mask, invert)
6. **Surface-map sampling** (Rust): Depth map + blank → one point per (x,y) at depth-mapped Z → 3D surface point cloud (ADR-012)
7. **Fit to blank** (Rust): Scale and translate to fit envelope with margin
8. **Preview** (Frontend): Point data → Three.js + blank wireframe
9. **Export** (Rust): PLY/XYZ/CSV to user-selected path

## Tauri Commands

- `load_image` - Load and validate image file
- `generate_depth_map` - Run AI depth estimation
- `set_blank_envelope` - Set crystal blank dimensions
- `get_point_cloud_data` - Get volumetric points for preview
- `export_ply` - Export PLY format
- `export_xyz` - Export XYZ format
- `export_csv` - Export CSV format
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

---

## Cross-Platform Development Rules

> These sections apply on all dev machines. Constraints marked macOS-only apply when
> the current session is on the macOS M-series MacBook Pro only.

### Platform Target Matrix

| Platform | Status | Notes |
|---|---|---|
| Windows | **Active — Primary** | Phase 1 + 2 development target |
| macOS | **Active — Gated (Phase 3)** | Build only on macOS M-series; Tauri universal binary |
| Linux | **Active — Gated (Phase 3)** | AppImage / .deb; requires GTK3 + WebKit dev libs |

This is a **desktop-only** application. There are no iOS, Android, or Web targets.

### Platform Guard Convention — Comment Guards

Mark all platform-specific code with structured comment guards so future sessions
can identify scope without reading full context.

**Rust:**
```rust
// [PLATFORM: Windows] ─── begin ──────────────────────────────────────
// [PLATFORM: Windows] ─── end ────────────────────────────────────────

// [PLATFORM: macOS] ─── begin ─────────────────────────────────────────
// [PLATFORM: macOS] ─── end ───────────────────────────────────────────

// [PLATFORM: Linux] ─── begin ─────────────────────────────────────────
// [PLATFORM: Linux] ─── end ───────────────────────────────────────────
```

**Svelte / TypeScript:**
```typescript
// [PLATFORM: Windows] ─── begin ──────────────────────────────────────
// [PLATFORM: Windows] ─── end ────────────────────────────────────────
```

**Critical rules:**
- Never remove a guard block without explicit confirmation that the feature is
  deprecated on that platform.
- When refactoring, move guard blocks with the code they annotate.
- When a change affects a guarded block, state which platform is affected and
  ask for confirmation before proceeding.

### Path Policy

**Never hardcode absolute paths** in any committed file. Use relative paths or
environment variables. Absolute paths containing `/Users/`, `C:\Users\`, or
`/home/<name>/` are forbidden in committed source, scripts, and workflow files.

### macOS / Linux Session Constraint (Phase 3)

The macOS M-series MacBook Pro is required for macOS and Linux Tauri builds when
code-signing is involved. If the current session is on Windows:
- Do NOT commit macOS/Linux-specific Tauri config changes that require signing.
- Document required changes in `PENDING_APPLE_CHANGES.md` for the next macOS session.

### Subagent Behaviour

All subagents inherit these constraints. Before acting, subagents must:
- Read this `CLAUDE.md` file in full.
- Read the relevant persona from `.agents/`.
- Flag any platform-affecting changes before applying them.

### Session Startup Checklist

- [ ] Read `README.md` and this file
- [ ] Detect current OS (`windows` / `macos` / `linux`)
- [ ] Note which build targets are available this session
- [ ] Check `PENDING_APPLE_CHANGES.md` for outstanding Phase 3 items
- [ ] Check `SETUP_NOTES.md` for any one-time setup steps on this machine
- [ ] Review `RESEARCH/GOTCHAS.md` for known pitfalls

---

*Cross-platform section schema version: 1.0 — 2026-04-06*
