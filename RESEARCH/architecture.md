# SimplePicture3D Architecture

**Purpose:** System architecture and data flow for SimplePicture3D.

**Source:** Derived from `prd.md` §5.2, §5.3, §5.4.  
**Last checked:** 2026-03-01

---

## Architecture Decision Records (ADRs)

*ADRs initiated 2026-02-06 per External Consultant Recommendations Report. ADR-008 winding order corrected 2026-02-28. ADR-009 added Sprint 2.1. ADR-010 added Sprint 2.2. Current status: see Consultant_Review_1Mar2026.md.*

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
4. Model download wizard (Sprint 1.10) handles Hugging Face model installation. **Security (SEC-202):** When implementing download, use HTTPS only and verify SHA256 checksum against a trusted source (repo/RESEARCH); see docs/security-checklist.md §2.2 and §5.

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

**SEC-202 verification status (updated 2026-03-01):** The `download_model` flow uses `huggingface_hub.snapshot_download` or `transformers.from_pretrained`; both use HTTPS and HF’s own integrity mechanisms. There is **no** explicit SHA256 verification against a checksum stored in this repo (trusted source per SEC-202). **Still open at Sprint 2.2 close.** Phase 2 security task (not optional): Security Specialist must verify before Sprint 2.4 — either (a) add post-download SHA256 check against hashes documented in RESEARCH, or (b) formally document acceptance of HF integrity and get security sign-off. See Consultant_Review_1Mar2026.md §3.1 (action item #1).

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

### ADR-008: Grid-Based Triangulation for STL Export (Sprint 1.8)

**Status:** Accepted
**Date:** 2026-02-08
**Context:** ARCH-301. Sprint 1.8 requires STL export, which mandates triangulated faces with normals. The existing `mesh_generator.rs` produces a uniform-grid point cloud (ADR-006). A triangulation strategy must be chosen and documented before BACK-700 implementation begins. This ADR supersedes the deferred ARCH-205–207 spike from Sprint 1.6A.

**Options considered:**

| Option | Description | Decision |
|--------|-------------|----------|
| **A** | Grid-based triangulation: each grid cell (ri, ci) produces 2 triangles from its 4 corner vertices. No external crate. | **Accepted.** |
| **B** | Delaunay triangulation via `delaunator` crate: general-purpose for irregular point sets. | Rejected: overkill for uniform grid; adds dependency; slower for regular data. |

**Decision: Option A — Grid-based triangulation in `mesh_generator.rs`.**

**Rationale:**
1. **Uniform grid is trivially triangulable.** Each grid cell at row `ri`, column `ci` (where `ri` ranges `0..num_rows-1` and `ci` ranges `0..num_cols-1`) has four corner vertices: top-left `(ri, ci)`, top-right `(ri, ci+1)`, bottom-left `(ri+1, ci)`, bottom-right `(ri+1, ci+1)`. Each cell yields exactly 2 triangles.
2. **No external crate needed.** The algorithm is ~30 lines of index arithmetic. Adding `delaunator` would increase compile time and binary size for zero benefit on uniform data.
3. **Deterministic and fast.** Triangle count = `2 × (num_rows - 1) × (num_cols - 1)`. O(n) where n is vertex count; no computational geometry overhead.
4. **Compatible with existing code.** `depth_to_point_cloud` already computes vertex positions and normals on the same uniform grid; triangulation is a second pass over the same grid indices.

**Module location: `src-tauri/src/mesh_generator.rs`.**

Triangulation is tightly coupled to point cloud generation (same grid dimensions, same vertex indexing). A separate `triangulator.rs` would add indirection with no benefit at this scale. If adaptive/multi-resolution triangulation is added later, it can be extracted then.

**Data format:**

1. **Index buffer:** `indices: Vec<u32>` added as an `Option` field on `MeshData`. Each consecutive triple `(indices[3*t], indices[3*t+1], indices[3*t+2])` defines one triangle. This is a standard triangle list (not strip).
2. **MeshData extension:**
   ```rust
   pub struct MeshData {
       pub positions: Vec<[f32; 3]>,
       pub normals: Vec<[f32; 3]>,
       pub indices: Option<Vec<u32>>,  // NEW: triangle index buffer
   }
   ```
   - `indices == None`: point cloud mode (current behavior, backwards compatible).
   - `indices == Some(vec)`: triangulated mesh. Length is `6 × (num_rows - 1) × (num_cols - 1)` (2 triangles × 3 indices per cell).
3. **Why indexed (not flat vertex list):** Indexed geometry avoids duplicating vertex data (each grid vertex is shared by up to 6 triangles). For a 1000x1000 grid: 1M vertices + 6M indices (24MB) vs 6M vertices duplicated (144MB). Three.js `BufferGeometry` supports indexed geometry natively. STL export dereferences indices to emit per-triangle vertex triples.
4. **Serialization (Tauri IPC):** `{ "positions": [...], "normals": [...], "indices": [0, 1, 100, 1, 101, 100, ...] }`. Frontend checks for `indices` presence to decide point cloud vs indexed mesh rendering.

**Winding order: Counter-clockwise (CCW) when viewed from +Z (outward normal direction).**

For each grid cell at `(ri, ci)` with vertex indices computed as `v = ri * num_cols + ci`:
- Triangle 1 (upper-left): `v_tl → v_tr → v_bl` i.e. `(ri*C + ci) → (ri*C + ci+1) → ((ri+1)*C + ci)`
- Triangle 2 (lower-right): `v_tr → v_br → v_bl` i.e. `(ri*C + ci+1) → ((ri+1)*C + ci+1) → ((ri+1)*C + ci)`

Where `C = num_cols`. CCW winding ensures the cross product `(v1-v0) × (v2-v0)` points in the +Z direction (outward from the surface), which is the convention for STL face normals. (Updated 2026-02-28 to match implementation; see RESEARCH/GOTCHAS.md 2026-02-08.)

**Face normals for STL:**

STL requires a per-face normal. Compute from the cross product of triangle edges: `normal = normalize((v1 - v0) × (v2 - v0))`. This is computed at export time (BACK-701), not stored in `MeshData`. The existing vertex normals (gradient-based) remain for Three.js smooth shading; face normals are computed on-the-fly during STL serialization.

**API for triangulation:**

A new public function in `mesh_generator.rs`:
```rust
pub fn triangulate_grid(num_rows: usize, num_cols: usize) -> Vec<u32>
```
Returns the index buffer for the grid. Called after `depth_to_point_cloud` to populate `MeshData.indices`. Alternatively, a combined function `depth_to_mesh` can call both in sequence and return `MeshData` with indices populated.

**Edge cases:**
- Grid with `num_rows < 2` or `num_cols < 2`: cannot form any triangle. Return empty indices (valid point cloud, no faces). STL export should reject meshes with 0 triangles.
- Grid step > image dimension: results in a single row or column; same as above.

**Memory impact:**
- Indices add `6 × (R-1) × (C-1) × 4 bytes` per mesh. For 4K (3840/step × 2160/step at step=1): ~198M indices × 4 = ~792MB. At step=2: ~49.5M indices × 4 = ~198MB. Within PRD <2GB budget when combined with positions+normals. For very large grids, the LOD/step mechanism already controls vertex count.

**Consequences:**
- BACK-700 implements `triangulate_grid` and integrates with `MeshData`.
- BACK-701 (STL writer) consumes `MeshData.indices` + `positions` to emit binary STL triangles with computed face normals.
- Frontend (`Preview3D.svelte`) can use indexed `BufferGeometry` when `indices` is present, falling back to point cloud when absent.
- No new crate dependencies introduced.

---

### ADR-009: Target Dimensions for Laser Etching (Point Cloud XY Size)

**Status:** Accepted  
**Date:** 2026-02-21  
**Context:** Mesh output must match the physical size of the laser-etched blank (e.g. 50×70mm crystal). Currently `pixel_to_mm` is fixed at 1.0, so physical size equals pixel dimensions (e.g. 1920×1080 px → 1920×1080 mm), which is not useful for typical blanks. We need a clear way to specify **target physical dimensions** for the generated point cloud.

**Decision: Specify target size in mm; derive uniform scale.**

1. **User-facing concept: Target size (width × height) in mm**
   - The user specifies the **physical XY extent** of the engraved area: `target_width_mm`, `target_height_mm` (e.g. 50 mm × 70 mm for a common crystal blank).
   - **Depth** is already specified separately: `depth_min_mm` and `depth_max_mm` (PRD: 2–10 mm). No change.
   - The mesh generator produces vertices in mm; XY extent of the mesh will **fit inside** the target rectangle with aspect ratio preserved (uniform scale).

2. **Derivation of `pixel_to_mm` from target dimensions**
   - Depth map has pixel dimensions `width_px × height_px`. We use a **single** scale factor (uniform scaling) so the design is not stretched.
   - **Formula:** `pixel_to_mm = min(target_width_mm / width_px, target_height_mm / height_px)`.
   - Effect: The mesh fits inside a rectangle of size `target_width_mm × target_height_mm`. One of the two edges will exactly match the target; the other may be smaller (letterboxing). Aspect ratio of the image is preserved.
   - **Edge case:** If the user wants "exact width" or "exact height" only, that is equivalent to setting the other target dimension large enough that the min is driven by the desired dimension (e.g. exact width 70 mm → use target 70×9999, then pixel_to_mm = 70/width_px).

3. **API and data flow**
   - **Option A (recommended):** Add optional `target_width_mm` and `target_height_mm` to the mesh/export request (or to persisted settings). If present, backend computes `pixel_to_mm` from depth map dimensions and target size; if absent, keep current behaviour (`pixel_to_mm = 1.0` or a default).
   - **Option B:** Add `target_width_mm` and `target_height_mm` to `MeshParams`; mesh_generator receives depth dimensions and params and computes `pixel_to_mm` internally when target is set. This keeps the "fit inside" logic in one place (mesh or lib.rs).
   - **Implementation note:** `MeshParams.pixel_to_mm` remains the single scale used in `depth_to_point_cloud`. The caller (e.g. lib.rs) can compute `pixel_to_mm` from `target_width_mm`, `target_height_mm`, and `(width_px, height_px)` before building `MeshParams`, so no change to `mesh_generator.rs` function signature is strictly required; only the way lib.rs (or settings) sets `pixel_to_mm` changes.

4. **Defaults and presets**
   - **Default:** If target dimensions are not specified, retain current behaviour: `pixel_to_mm = 1.0` (physical size = pixel size). Alternatively, a product default such as 50×70 mm can be documented so "fit to common blank" works out of the box.
   - **Presets (Phase 2):** e.g. "50×70 mm", "40×60 mm", "Custom" with width/height inputs. Presets set `target_width_mm` / `target_height_mm`; backend derives `pixel_to_mm` as above.

5. **UI**
   - Optional for MVP: expose "Output size (mm): width × height" in the mesh/export area or settings (two numeric inputs, or preset dropdown). If not in MVP, backend and settings are still extended so that when the UI is added, it only passes target dimensions.

**Rationale:**
- Laser etchers work with physical blanks; specifying "50×70 mm" is intuitive and avoids manual `pixel_to_mm` calculation.
- Uniform scale preserves aspect ratio and avoids distorted engravings.
- Single formula keeps behaviour predictable and testable.

**Consequences:**
- Backend (lib.rs or mesh layer) accepts optional target dimensions and computes `pixel_to_mm` when building `MeshParams`.
- Settings may store `target_width_mm` / `target_height_mm` (optional) for persistence.
- Documentation and UI copy should use "Output size" or "Target size (mm)" to match user mental model.
- Existing behaviour (fixed `pixel_to_mm`) remains valid when target dimensions are not provided.

---

### Default output scale and zoom (Sprint 2.1)

**Status:** Implemented  
**Date:** 2026-02-28  
**Context:** On image import, depth-map and 3D preview should be dimensioned to a default target so output is "zoomed to fit" without manual setup. User can increase or reduce effective scale.

**Decision:**

1. **On image load:** Set default target dimensions to **40×40 mm** (square). Persist to `AppSettings` so `get_mesh_data` and export use this target. Depth-map and 3D preview are dimensioned to this scale (mesh/export use `pixel_to_mm` derived from target; preview receives target from App and passes to `get_mesh_data`).

2. **Zoom control:** Footer exposes **Zoom: 50% | 100% | 150% | 200%**. Effective target = base 40 mm × scale → 20×20, 40×40, 60×60, 80×80 mm. Single source of truth in App state; passed to ExportPanel (export) and Preview3D (`get_mesh_data`). When zoom changes, 3D preview reloads mesh so view stays fitted.

3. **Fit behaviour:** Once target is set, mesh generation and export use it; depth map preview and 3D preview display content fitted to the bounded viewport (see Depth map preview UI below).

**Rationale:** Matches user request for "target 40×40×40 mm" (XY default 40 mm; depth range remains 2–10 mm unless extended) with optional zoom. Keeps layout predictable and avoids page scroll from oversized depth preview.

**Consequences:** App.svelte owns `effectiveTargetWidthMm` / `effectiveTargetHeightMm`; on `handleLoadSuccess` sets 40×40 and saves settings; `getMeshData(options)` accepts optional `targetWidthMm` / `targetHeightMm`; Preview3D and ExportPanel consume target from App.

---

### Undo/Redo Architecture (Sprint 2.2, ARCH-401–ARCH-403)

**Status:** Accepted (design)  
**Date:** 2026-03-01  
**Context:** PRD F2.4 requires undo last 20 actions, redo, and keyboard shortcuts. Scheduled Sprint 2.2 per consultant recommendation; delivered 2026-03-01. See Consultant_Review_1Mar2026.md §2.

#### ARCH-401: Command pattern and integration

**Decision: Undo/redo uses a command pattern with history held in the Rust backend.**

1. **Where commands live:** **Rust backend** (`src-tauri/src/`). The single source of truth for mutable depth state is `AppState.adjustment_params` (and, when persisted, curve in `AppSettings`). Commands are implemented in Rust; each command captures the previous state needed to undo (e.g. previous `DepthAdjustmentParams` or delta).

2. **Command contract:** Each undoable operation is a **command** with:
   - **execute(ctx):** Apply the change; update `AppState.adjustment_params` (and optionally sync to frontend via return value or event).
   - **undo(ctx):** Restore state from the snapshot stored in the command (e.g. restore previous params).
   - Optional: **description** for UI (e.g. "Brightness +0.1") for a future action panel.

3. **Integration with existing flow:**
   - **Current flow:** Frontend calls `set_depth_adjustment_params` → backend mutates `adjustment_params`; `get_depth_map` / `get_mesh_data` read current params. No history today.
   - **New flow:** Frontend continues to call a single "apply adjustment" entry point (or multiple granular ones). Backend creates a command (e.g. `SetDepthParams { previous, next }`), runs `execute` (writes `next` into `AppState`), pushes command onto **undo stack**. Undo: pop command, call `undo` (writes `previous` back). Redo: push undone command onto **redo stack**; redo pops and re-executes.
   - **IPC:** New Tauri commands `undo`, `redo`, `clear_history` return success and **current state** (e.g. `DepthAdjustmentParams` and `can_undo` / `can_redo`) so the frontend can update UI and disable buttons without extra round-trips.

4. **Frontend role:** Frontend does **not** hold a duplicate history stack. It invokes `undo` / `redo`; backend returns the new state; frontend updates local Svelte state (e.g. depth params store) from the response so the UI reflects the reverted state. Single source of truth remains backend.

#### ARCH-402: Mutable operations to track (Sprint 2.2 scope)

**In scope for Sprint 2.2 (undoable):**

| Operation | Description | Command stores |
|----------|-------------|----------------|
| Depth brightness change | Slider / input | Previous + next `DepthAdjustmentParams` (or brightness only if granular) |
| Depth contrast change | Slider / input | Previous + next params |
| Depth gamma change | Slider / input | Previous + next params |
| Depth invert toggle | Checkbox | Previous + next params |
| Depth range (min/max mm) | Sliders | Previous + next params |
| Curve control points change | CurvesTool drag/preset | Previous + next `curve_control_points` (or full params) |

**Out of scope for 2.2 (not undoable this sprint):**

- **Load image:** Clears history per PRD F2.4 ("Clear history on new image load"). No need to undo "load" as a step.
- **Generate depth:** Not in undo stack; PRD note "Disable undo/redo during mesh generation" — treat as one-shot action.
- **Target dimensions / zoom:** Can be added in a later sprint if desired; not required for F2.4.
- **Export STL/OBJ:** Not undoable (side-effect to disk).

**Scope agreement:** Depth adjustments only (including curve). Masking, brushes, and other state-mutation features (Phase 2 later) will add their own commands and reuse the same history stack contract.

**Open verification (Consultant_Review_1Mar2026 §4.1):** Confirm that curve control point mutations (CurvesTool drag / preset) create `SetDepthParamsCommand` entries in the undo stack — not only written to `AppSettings` for persistence. The CHANGELOG states undo covers curve control points; implementation must be verified before Sprint 2.3. If curve changes bypass the stack, Ctrl+Z has no effect on curve state.

#### ARCH-403: History stack memory budget

**Decision:**

- **Max undo stack size:** **20** actions (per PRD F2.4 and todo.md).
- **When limit reached:** **Drop oldest.** On execute, if undo stack length would exceed 20, remove the oldest entry (bottom of stack) before pushing the new command. FIFO eviction.
- **Redo stack:** When user performs a **new** action (not redo), **clear the redo stack** so redo only reapplies previously undone actions in order.
- **Serialization:** Each command holds a snapshot of `DepthAdjustmentParams` (or a delta). Memory per entry is small (order of hundreds of bytes for params + optional curve points). 20 entries stay well under any reasonable budget; no need for lazy or compressed serialization for 2.2.

---

### ADR-010: State Management — Svelte Stores and Backend Sync (TD-01)

**Status:** Accepted  
**Date:** 2026-03-01  
**Context:** Technical debt TD-01: no documented design for Svelte store vs Tauri state. Required before further state-mutation features (masking, brushes). TD-01 closed Sprint 2.2. See Consultant_Review_1Mar2026.md §2.3.

**Decision: Hybrid — backend is source of truth for mutable depth state; frontend mirrors via IPC.**

1. **Backend (Rust) owns:**
   - **Depth map** (original from Python) and **depth adjustment params** (`DepthAdjustmentParams`: brightness, contrast, gamma, invert, depth range, curve control points). Stored in `AppState` (`depth`, `adjustment_params`).
   - **Undo/redo stacks** (command history). No duplicate stack on frontend.
   - **Persistent settings** (`AppSettings`): last export dir, export format, depth params for session restore, target dimensions, **curve control points** (CURVE-001). Loaded on startup; saved on export / explicit save / or when curve or key params change (per product behaviour).

2. **Frontend (Svelte) holds:**
   - **Mirror of depth params** (and curve) for reactive UI: sliders, curve editor, preview. Updated when: (a) user changes a control → invoke backend → backend updates AppState and returns new params → frontend updates store; (b) user invokes undo/redo → backend returns new params → frontend updates store; (c) on load, from `get_depth_adjustment_params` or get_settings.
   - **Can-undo / can-redo flags** (or derived from backend response after each undo/redo/invocation) to enable/disable toolbar buttons and shortcuts.
   - **Non-mutable UI state:** which panel is open, zoom percentage, target dimensions for display, etc. No need to round-trip these for undo unless product later decides otherwise.

3. **Sync rules:**
   - **Single source of truth for depth params:** Backend. Every mutation goes through Tauri commands. Frontend never "optimistically" holds the only copy of a depth param that can be undone.
   - **After undo/redo:** Backend returns the restored `DepthAdjustmentParams` (and optionally full state); frontend overwrites its depth-params store so the UI reflects the reverted state.
   - **New image load:** Backend clears history and resets params (or applies defaults); frontend clears local mirror and re-fetches params if needed.

4. **Guidance for future state-mutation features (masking, brushes):**
   - New mutable state (e.g. mask bitmap, brush strokes) should follow the same pattern: backend holds authoritative state and history entries; frontend mirrors for reactivity and invokes commands for every mutation. Do not introduce a second source of truth for undoable state on the frontend.

**Rationale:** Keeps undo/redo semantics simple (one stack, one owner), avoids desync between frontend and backend, and sets a clear pattern for Phase 2 masking/brushes.

**Pre-Sprint 2.5 action (Consultant_Review_1Mar2026 §4.5, §6):** Before masking sprint planning begins, the Architect and Senior Engineer must assess whether the current `SetDepthParamsCommand` struct (a flat snapshot of `DepthAdjustmentParams`) adequately models mask state (regions, brush strokes, layer blending). Masking likely requires a new command type. ADR-010 should be extended — or a new ADR authored — covering the mask command contract before Sprint 2.5 begins.

**Consequences:**
- BACK-1401–1404 implement command trait, history stack, and Tauri undo/redo/clear_history; frontend uses command responses to update UI.
- CURVE-001 persists curve in AppSettings; load/save round-trip through backend.
- UI-1401/1402 wire buttons and shortcuts to backend; disable state derived from backend return values.

---

### Future: Full 3D Reconstruction Mode (Phase 2, optional)

**Context:** Single-image **full 3D** reconstruction produces a watertight mesh; internal UV laser engraving consumes **point clouds** (3D coordinates), not meshes. So the Full 3D pipeline must produce a **dimensioned point cloud** (mm), same as 2.5D — generated by **surface sampling** the AI mesh rather than from a depth grid. Use cases: 3D crystal engraving, 3D printing, multi-angle viewing. See RESEARCH/3d-reconstruction.md (last checked 2026-02-22).

**Decision (research 2026-02-22):** Offer as a **secondary mode**. Recommended model: **TripoSR** (MIT, 6 GB VRAM, ~0.5 s). Python runs TripoSR then **Poisson-disk surface sampling** (trimesh); outputs **point cloud** to Rust. Rust **scales and centers** the point cloud to **crystal blank dimensions** (3D bbox → blank volume), then supports **direct point cloud export** (XYZ, PLY, CSV) for engravers plus existing STL/OBJ for tools that sample meshes themselves.

**High-level design:**
- **Python:** `reconstruction_3d.py` — TripoSR inference → OBJ mesh → trimesh `sample_surface_even` (Poisson disk) → output **point cloud** (JSON or binary) to Rust. Subprocess contract like depth_estimator.
- **Rust:** **Blank dimensioning:** new `BlankParams` (blank_width_mm, blank_height_mm, blank_depth_mm, margin_mm); `scale_to_blank` fits 3D bbox to crystal volume. **Import:** `import_point_cloud` (pre-sampled points, no grid). **Export:** existing STL/OBJ; **new** XYZ, PLY, CSV for direct engraver consumption. Optional `MeshData.intensities` for per-point laser power (Phase 2 later).
- **Frontend:** Mode toggle "2.5D Relief" vs "Full 3D"; blank dimension inputs, point spacing (mm), crystal blank wireframe in preview; optional estimated point count before export.
- **Hardware:** 6 GB VRAM for TripoSR; CPU fallback slow. 2.5D remains default, no GPU required.

**Planning:** prd.md §11.1 (deferred feature #11); todo.md Phase 2 "Full 3D Reconstruction Mode" (~4–5 sprints: Python TripoSR + sampling, Rust blank/import/export, UI, testing; optional 5th for intensity).

---

## Overview

Tauri desktop app: Rust backend, Svelte 4 frontend, Python subprocess for AI inference.

```
┌─────────────────────────────────────────────────────────┐
│                     Tauri Frontend                      │
│  Svelte 4 │ Three.js 3D Preview │ TailwindCSS           │
└───────────────────────┬─────────────────────────────────┘
                        │ Tauri Commands (IPC)
┌───────────────────────▼─────────────────────────────────┐
│                   Rust Backend                          │
│  Image loading │ Depth processing │ Mesh generation     │
│  STL/OBJ export │ Settings │ Python subprocess bridge   │
└───────────────────────┬─────────────────────────────────┘
                        │ subprocess (temp file → stdout)
┌───────────────────────▼─────────────────────────────────┐
│                  Python AI Backend                      │
│  Depth-Anything-V2 / MiDaS │ PyTorch                    │
│  Input: Image → Output: Depth map (JSON/binary)         │
└─────────────────────────────────────────────────────────┘
```

---

## As-built (Sprint 1.11, Sprint 2.1)

*Snapshot of the codebase as of Phase 1 exit and Sprint 2.1. Kept in sync with implementation.*

### Rust backend modules (src-tauri/src/)

| Module | Purpose |
|--------|---------|
| `lib.rs` | Tauri app entry, command handlers, `AppState`, export path validation (SEC-401/SEC-402), `get_depth_histogram` (BACK-1101) |
| `image_loading.rs` | Load/validate image (BACK-101–105), path validation (SEC-101), downsampling |
| `file_io.rs` | Temp file I/O for Python handoff; paths restricted to system temp dir |
| `depth_adjust.rs` | Gamma, contrast, brightness, invert, depth range (BACK-402); **curve** (control points, presets Linear/S-curve/Exponential, BACK-1102–1103); **histogram** `compute_histogram` 256 bins (BACK-1101). Order: invert → gamma → contrast → brightness → **curve**. |
| `mesh_generator.rs` | Point cloud, grid triangulation (ADR-008), STL/OBJ writers, validation (BACK-501–506, BACK-700–702, BACK-801–803) |
| `python_bridge.rs` | Subprocess depth estimation (BACK-201–205), progress protocol |
| `settings.rs` | AppSettings load/save (BACK-706, BACK-804–805) |
| `undo.rs` | Undo/redo command pattern, history stack (BACK-1401–1404, ARCH-403); `SetDepthParamsCommand`, `UndoRedoHistory` |

**STL/OBJ export:** Implemented as **custom** binary STL and ASCII OBJ (+ MTL) writers in `mesh_generator.rs`; the project does **not** use `stl_io` or `obj-exporter` crates (see Key Interfaces below).

**Target dimensions (ADR-009):** Implemented. Optional `target_width_mm` and `target_height_mm` are supported on `get_mesh_data`, `export_stl`, and `export_obj` (and in `AppSettings`). `lib.rs::compute_pixel_to_mm` derives `pixel_to_mm = min(target_width_mm/width_px, target_height_mm/height_px)` when both are set; when absent, default `pixel_to_mm = 1.0` is unchanged. **Default output scale (Sprint 2.1):** On image load, App sets default target 40×40 mm and persists; zoom control (50/100/150/200%) scales effective target; Preview3D and ExportPanel receive target from App.

### Data flow (as-built)

Unchanged from design: Load image → Validate → Depth (Python) → Depth processing (Rust, including **curve**) → Mesh generation (Rust) → Preview (Three.js) → Export (STL/OBJ). Export path validation is centralized in `lib.rs::validate_export_path`. **On image load:** target dimensions set to 40×40 mm; depth-map and 3D preview slots are dimensioned so output fits (see below).

### Depth map preview (UI, Sprint 2.1)

- **Bounded slot:** The depth map preview lives in a fixed-height slot (`min-h-[200px] max-h-[40vh] h-[280px]`) so it never expands the page. Slot is in the right sidebar; overflow hidden.
- **Fit-to-view:** When a new depth map is loaded (dimensions change), `applyFitToView()` runs automatically: zoom and pan are set so the depth map fits inside the container. ResizeObserver re-fits when the container is resized.
- **Layout:** The zoom/pan layer (canvas wrapper) is **position: absolute** so the full-resolution canvas does not participate in layout; the container keeps its bounded size and content is clipped. User can still drag to pan and scroll to zoom; "Fit" button re-applies fit.

### Preview3D (Sprint 2.1 behaviour)

- **Target dimensions:** Receives `targetWidthMm` / `targetHeightMm` from App and passes them to `get_mesh_data(options)`; mesh is generated at the correct physical scale. When target dimensions change (e.g. zoom), mesh is reloaded (with guard to avoid reactive loops).
- **Defensive handling:** `buildPointCloud()` guards against empty `positions` (returns empty Points); uses `normals ?? []` and per-vertex fallback `normals[i] ?? [0,0,1]`; guards against undefined/NaN when reading positions/normals so the viewer does not throw on malformed or partial data.

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
│   ├── 3d-reconstruction.md # Full 3D reconstruction models (Phase 2; see ADR "Future: Full 3D")
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
│   │   ├── lib.rs          # IPC command handlers, integration tests
│   │   ├── image_loading.rs # Image load, validate, downsample, RGB (BACK-101–105)
│   │   ├── file_io.rs       # Temp path utilities (Python handoff)
│   │   ├── depth_adjust.rs  # Depth adjustments (gamma, range, invert)
│   │   ├── mesh_generator.rs # Point cloud, grid triangulation, STL/OBJ export (no exporters/; consolidated here per ADR-008)
│   │   ├── python_bridge.rs # Python subprocess, depth map I/O
│   │   ├── settings.rs      # App settings persistence
│   │   └── undo.rs         # Undo/redo (BACK-1401–1404)
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                    # Frontend (Svelte)
│   ├── components/
│   │   ├── ImageImport.svelte
│   │   ├── DepthControls.svelte
│   │   ├── DepthMapPreview.svelte
│   │   ├── HistogramPanel.svelte   # Depth distribution (Sprint 2.1, BACK-1101)
│   │   ├── CurvesTool.svelte       # Curve control points, presets (Sprint 2.1, BACK-1102–1103)
│   │   ├── Preview3D.svelte
│   │   ├── ExportPanel.svelte
│   │   ├── FirstRunWizard.svelte
│   │   └── Button.svelte
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

### Target dimensions for laser etching (ADR-009)

To match the physical size of the engraved blank (e.g. 50×70 mm crystal), the user can specify **target size** in mm. The mesh XY extent is then scaled to **fit inside** that rectangle with aspect ratio preserved.

- **Input (optional):** `target_width_mm`, `target_height_mm` (e.g. 50, 70). Depth is already `depth_min_mm`..`depth_max_mm`.
- **Derivation:** `pixel_to_mm = min(target_width_mm / width_px, target_height_mm / height_px)`. Caller (e.g. lib.rs) computes this from depth map dimensions and target size, then passes `pixel_to_mm` in `MeshParams`. If target is not set, use current default (e.g. `pixel_to_mm = 1.0`).
- **Result:** Mesh physical size fits inside `target_width_mm × target_height_mm`; one dimension matches the target, the other is ≤ target. No stretch.

See **ADR-009** above for full decision, API options, and UI/preset notes.

### Mesh data IPC contract (get_mesh_data, BACK-601, BACK-602)

- **Current transfer:** JSON over Tauri IPC. `get_mesh_data` returns `Option<MeshData>`; serialized as `{ "positions": [[x,y,z], ...], "normals": [[x,y,z], ...] }` (camelCase). Frontend can build Three.js `BufferGeometry` by flattening: `new Float32Array(meshData.positions.flat())` for position attribute (itemSize 3), same for normals. Performance acceptable for 100K–1M vertices; if latency >100ms at 1080p, Sprint 1.6A may adopt binary transfer (ADR-007).
- **ADR-007 (deferred):** If binary transfer is chosen (e.g. temp file or binary IPC), backend will implement the alternative path alongside JSON; format (byte order, layout) will be documented here. Caller-facing contract (positions, normals, dimensions) unchanged.
- **LOD (BACK-603):** Optional `preview_step` argument: when set, backend uses that grid step (e.g. 2 → 1/4 vertices) for preview; full-detail export path is unchanged.
- **Target dimensions (Sprint 2.1):** Frontend may pass optional `target_width_mm` and `target_height_mm` to `get_mesh_data`; when provided, backend uses them (or falls back to settings) to compute `pixel_to_mm`. Enables 3D preview and export to share the same scale (e.g. 40×40 mm default, zoom 50–200%).

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

- **Tauri commands:** `load_image`, `generate_depth_map`, `get_depth_map`, **`get_depth_histogram`** (256 bins of current adjusted depth, BACK-1101), `set_depth_adjustment_params`, `get_depth_adjustment_params`, `reset_depth_adjustments`, **`undo`, `redo`, `clear_history`** (Sprint 2.2, BACK-1404 — return success + current params + can_undo/can_redo), `get_mesh_data` (optional `target_width_mm`, `target_height_mm`), `export_stl`, `export_obj`, `get_settings`, `save_settings`, `check_model`, `get_model_info`, `download_model`
- **STL/OBJ export:** Implemented as **custom** binary STL and ASCII OBJ (with .mtl) writers in `src-tauri/src/mesh_generator.rs`. The project does **not** use the `stl_io` or `obj-exporter` crates; the PRD §5.4 notion of a separate `exporters/` module was consolidated into `mesh_generator.rs` (ADR-008, Sprint 1.8/1.9). See RESEARCH/rust-crates.md for crate guidance vs as-built.
- **Python interface (Sprint 1.3):** See **docs/architecture.md** § "Rust–Python Bridge (Sprint 1.3)" for the full IPC contract:
  - **Image input:** Temp file path only (`--input <path>`); path validated, under system temp dir (ARCH-102).
  - **Depth output:** JSON `{"height", "width", "depth": [f32,...]}` to stdout (or file); 0–1 normalized, row-major.
  - **Invocation:** Subprocess (no shell); fixed CLI; progress on stderr (ARCH-101, ARCH-103).
- **Model storage:** `~/.simplepicture3d/models/`
- **Settings:** `~/.simplepicture3d/` (presets, logs, cache)

---

## Preset schema (Sprint 2.3, BACK-1301)

Presets store processing configuration as JSON for save/load and import/export (prd.md F2.3). Stored under `~/.simplepicture3d/presets/` or user-chosen path.

### Schema version

- **Field:** `schemaVersion` (integer). Current version: **1**. Used for forward-compatible migration when loading older presets (JR2-1303).
- New fields may be added in future versions; unknown fields are ignored. When adding breaking changes, bump version and document migration in code or RESEARCH.

### Fields (version 1)

| Field | Type | Description |
|-------|------|-------------|
| `schemaVersion` | number | Required. Set to 1. |
| `brightness` | number | Depth brightness offset (e.g. -0.5 to 0.5). |
| `contrast` | number | Depth contrast factor; 1.0 = identity. |
| `gamma` | number | Depth gamma exponent; 1.0 = linear. |
| `invert` | boolean | Invert depth (near ↔ far). |
| `depthMinMm` | number | Depth range minimum in mm. |
| `depthMaxMm` | number | Depth range maximum in mm. |
| `curveControlPoints` | array or null | Optional. Array of `{ x, y }` (0–1). When null or length < 2, no curve applied. |
| `stepX` | number | Mesh grid step X (1 = full resolution). |
| `stepY` | number | Mesh grid step Y (1 = full resolution). |
| `targetWidthMm` | number or null | Optional. Target output width in mm (ADR-009). |
| `targetHeightMm` | number or null | Optional. Target output height in mm (ADR-009). |

All depth and mesh params that are restorable from a preset are included so that loading a preset restores depth adjustment, curve, and mesh/export behaviour. Curve control points allow presets to restore the full curve state (BACK-1102, BACK-1103). Target dimensions are optional for “fit to blank” use cases.

### Alignment with app state

- **Depth:** Maps to `DepthAdjustmentParams` (depth_adjust.rs) and undo stack state.
- **Curve:** Same `curve_control_points` as in `AppSettings` and `DepthAdjustmentParams`.
- **Mesh:** `step_x`/`step_y` map to `MeshParams`; `target_width_mm`/`target_height_mm` map to `AppSettings` and are used to derive `pixel_to_mm` at mesh generation time (ADR-009).

### Tauri preset commands (BACK-1302, BACK-1303)

The frontend calls these Tauri command names. Backend must register them with the **exact** names and JSON argument names below so the UI (PresetManager, Save/Load, dropdown, Import/Export) works without change.

| Command | Args (JSON keys, snake_case) | Returns | Description |
|---------|------------------------------|---------|--------------|
| `list_presets` | *(none)* | `PresetListItem[]` | Combined list: built-in presets (BACK-1303) plus user presets from `~/.simplepicture3d/presets/`. See `PresetListItem` below. |
| `save_preset` | `name: string`, `path?: string` | `void` | Serialize current app state (depth + curve + mesh) to preset JSON. If `path` is present, write to that file (export); otherwise write to `~/.simplepicture3d/presets/{name}.json`. |
| `load_preset` | `name_or_path: string` | `void` | Load preset: if `name_or_path` is an absolute path, read from file (import); otherwise resolve by name (user file in presets dir or built-in id). Apply to app state (depth params, curve, mesh, settings). |
| `delete_preset` | `name: string` | `void` | Delete user preset file `~/.simplepicture3d/presets/{name}.json`. Must not allow deleting built-ins. |
| `rename_preset` | `old_name: string`, `new_name: string` | `void` | Rename user preset file (old_name → new_name in presets dir). Must not allow renaming built-ins. |

**`PresetListItem` (returned by `list_presets`):**

| Field | Type | Description |
|-------|------|-------------|
| `kind` | `"user" \| "builtin"` | User presets are in presets dir; built-in are from BACK-1303. |
| `name` | string | Display name (e.g. "My Preset", "Portrait"). |
| `id` | string | For **user:** preset name (key in presets dir). For **builtin:** id used when calling `load_preset(id)`. |

**Built-in preset ids (BACK-1303):** The frontend expects `list_presets` to include built-in entries with these ids and display names. When the user selects one, the frontend calls `load_preset(id)`.

| id | name (display) |
|----|----------------|
| `portrait` | Portrait |
| `landscape` | Landscape |
| `high_detail` | High Detail |
| `low_relief` | Low Relief |

**Frontend reference:** `src/lib/tauri.ts` — `listPresets()`, `savePreset(name, path?)`, `loadPreset(nameOrPath)` (sends `name_or_path`), `deletePreset(name)`, `renamePreset(oldName, newName)` (sends `old_name`, `new_name`). Capabilities: `allow-list-presets`, `allow-list-builtin-presets`, `allow-save-preset`, `allow-load-preset`, `allow-delete-preset`, `allow-rename-preset` in `src-tauri/capabilities/default.json`.

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
