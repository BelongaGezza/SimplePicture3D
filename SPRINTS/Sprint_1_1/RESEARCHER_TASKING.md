# Researcher Tasking — Pre–Sprint 1.1: Knowledge Refresh

**Purpose:** Close the gap between AI knowledge cutoff and today for all technologies used in SimplePicture3D.  
**Owner:** Researcher (`.agents/researcher.md`)  
**Trigger:** System Architect (2026-02-01)  
**Output:** Updated `RESEARCH/` files; optional summary in this folder.

---

## Objective

Agents and developers will rely on `RESEARCH/` for current API usage, deprecations, and version guidance. Research files currently have **no or incomplete "Last checked" dates** and may reflect pre–cutoff knowledge. The Researcher must verify official sources, record "Last checked" dates, and document any breaking changes or migration notes so Sprint 1.1 implementation uses up-to-date guidance.

---

## Scope: Technologies to Refresh

| Layer | Technologies | Research File | Priority |
|-------|--------------|---------------|----------|
| Shell | Tauri (v1 vs v2, IPC, subprocess) | `RESEARCH/tauri.md` | P0 |
| Backend | Rust: image, stl_io, obj (or equivalent), tokio, serde, anyhow | `RESEARCH/rust-crates.md` | P0 |
| AI | Python 3.x, PyTorch, Depth-Anything-V2, MiDaS, Hugging Face, ONNX | `RESEARCH/python-ml.md` | P0 |
| Frontend | Svelte vs React, TypeScript, TailwindCSS, Vite | `RESEARCH/frontend.md` | P0 |
| 3D | Three.js, WebGL | `RESEARCH/threejs.md` | P0 |
| System | Architecture, file structure (prd §5.2, §5.4) | `RESEARCH/architecture.md` | P1 |

---

## Tasks

### 1. Tauri (`RESEARCH/tauri.md`)
- [x] Confirm **Tauri v1 vs v2** recommendation for new project (Feb 2026); document version choice and rationale.
- [x] Check official docs/changelog for: new project scaffolding, `tauri.conf` / capabilities, IPC limits (large payloads), subprocess spawning from Rust.
- [x] Add **Official Sources** table with URLs and **Last checked: YYYY-MM-DD**.
- [x] Note any deprecations or breaking changes since cutoff; add migration notes if relevant.

### 2. Rust crates (`RESEARCH/rust-crates.md`)
- [x] For each crate (image, stl_io, tokio, serde, anyhow): current stable version, docs URL, **Last checked** date.
- [x] **Add OBJ export:** identify recommended crate (`obj`, `wavefront_obj`, or other) for writing ASCII OBJ; add to table and usage.
- [x] Check for deprecations, MSRV, or breaking changes in recent releases.
- [x] Note any guidance for large image/depth buffers (e.g. `ndarray` vs raw vec).

### 3. Python / ML (`RESEARCH/python-ml.md`)
- [x] PyTorch: current stable version; install commands for CPU/CUDA/Metal (Windows/macOS/Linux); **Last checked** date.
- [x] Depth-Anything-V2 and MiDaS: repo status, weight licenses (e.g. CC-BY-NC-4.0), Hugging Face model IDs, any API or dependency changes.
- [x] Python 3.x: minimum version recommendation (e.g. 3.10+).
- [x] ONNX/ONNXRuntime: optional path for inference; note if recommended for this project.
- [x] Add official source URLs and **Last checked** to table.

### 4. Frontend (`RESEARCH/frontend.md`)
- [x] **Svelte vs React:** current guidance for Tauri frontend (bootstrap, Vite, SSR vs SPA); recommend one and document why.
- [x] TypeScript: version/tooling (e.g. strict mode, bundler).
- [x] TailwindCSS: v3/v4, config with Vite, **Last checked**.
- [x] Add Official Sources table with **Last checked** dates.

### 5. Three.js (`RESEARCH/threejs.md`)
- [x] Current major version (r160+); CDN vs npm; usage with Tauri webview.
- [x] Key APIs for: BufferGeometry from vertex data, OrbitControls, basic scene/camera/renderer.
- [x] Add Official Sources and **Last checked** date.

### 6. Architecture (`RESEARCH/architecture.md`)
- [x] Re-verify against `prd.md` §5.2, §5.3, §5.4; update "Last checked" and any file-structure or data-flow details if PRD changed.

### 7. Index and GOTCHAS
- [x] Update `RESEARCH/README.md` index with **Last updated** dates per file where applicable.
- [x] If any refresh uncovers pitfalls (e.g. Tauri v2 migration, PyTorch 2.x), add to `RESEARCH/GOTCHAS.md` with date and context.

---

## Deliverables

| Deliverable | Location |
|-------------|----------|
| Updated technology files | `RESEARCH/tauri.md`, `rust-crates.md`, `python-ml.md`, `frontend.md`, `threejs.md`, `architecture.md` |
| Official sources + Last checked | In each file’s table |
| README index dates | `RESEARCH/README.md` |
| New gotchas (if any) | `RESEARCH/GOTCHAS.md` |
| Optional summary | `SPRINTS/Sprint_1_1/RESEARCH_REFRESH_SUMMARY.md` (short list of versions chosen, breaking changes, recommendations) |

---

## Completion Criteria

- [x] Every RESEARCH technology file in scope has an **Official Sources** (or equivalent) table with **Last checked: YYYY-MM-DD**.
- [x] Tauri version (v1 vs v2) is stated with rationale.
- [x] OBJ export crate is specified in `rust-crates.md`.
- [x] Frontend framework choice (Svelte or React) is stated with rationale.
- [x] No open "—" in Last checked for in-scope files.
- [x] Researcher marks this tasking complete in Progress Log or summary.

---

## References

- `RESEARCH/README.md` — Research folder ownership and format
- `.agents/researcher.md` — Persona and responsibilities
- `prd.md` §5.1 — Technology stack
- `todo.md` — Phase overview and Sprint 1.1 context
