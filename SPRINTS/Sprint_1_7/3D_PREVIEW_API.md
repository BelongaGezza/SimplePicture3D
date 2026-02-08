# 3D Preview API — Sprint 1.7 Contract

**Purpose:** Single reference for the mesh data contract between Rust backend and Svelte/Three.js frontend.  
**Authority:** System Architect (Sprint 1.7).  
**Source of truth (detailed):** `RESEARCH/architecture.md` § Mesh data IPC contract, ADR-006, ADR-007.

---

## 1. Command

| Item | Value |
|------|--------|
| **Tauri command** | `get_mesh_data` |
| **Backend** | Implemented in Sprint 1.6; BACK-601 (ADR-007 path), BACK-602 (serialization) |
| **Frontend consumers** | UI-503 (load), UI-504 (render point cloud) |

---

## 2. Request (Frontend → Backend)

- **Current:** No required arguments. Backend uses current app state (loaded image, depth, mesh params).
- **Optional (BACK-603):** `preview_step?: number` — when set (e.g. `2`), backend returns reduced-detail mesh (grid step in X/Y). Full-detail export path unchanged. Omit for full-resolution preview.

---

## 3. Response (Backend → Frontend)

### 3.1 Current: JSON over Tauri IPC

- **Type:** `Option<MeshData>`; serialized as JSON.
- **Success:** `{ "positions": [[x,y,z], ...], "normals": [[nx,ny,nz], ...] }` (camelCase).
- **No mesh / error:** `null` or Tauri error; frontend should handle gracefully (e.g. clear viewport, show message).
- **Units:** Positions in **millimeters** (f32). Normals unit length.
- **Layout:** One vertex per array index; `positions.length === normals.length`. Row-major grid order (match depth map sampling).

### 3.2 Frontend consumption (Three.js)

- **Positions:** `new Float32Array(meshData.positions.flat())` → `BufferAttribute` with `itemSize: 3`.
- **Normals:** Same for `normals`; `itemSize: 3`.
- **Geometry:** `THREE.BufferGeometry` with `setAttribute('position', ...)` and `setAttribute('normal', ...)`; use `THREE.Points` or `THREE.PointsMaterial` for point cloud (ADR-006: point cloud only in Sprint 1.7; no triangulated faces until Sprint 1.8).

### 3.3 ADR-007 (when adopted)

- If Sprint 1.6A chooses binary transfer (temp file or binary IPC), BACK-601 will implement an alternative path **alongside** JSON.
- **Caller-facing contract unchanged:** positions, normals, same semantics. Only transport and format (byte order, layout) may differ; layout will be documented in `RESEARCH/architecture.md` and ADR-007.
- UI-503 must work with current JSON; adapt to binary path when ADR-007 is decided and BACK-601 delivers it.

---

## 4. Coordination

| Role | Task | Dependency on this contract |
|------|------|-----------------------------|
| **Senior Engineer** | BACK-601, BACK-602 | Implement/serialize to this contract; add ADR-007 path if required. |
| **UI Designer** | UI-503, UI-504 | Invoke `get_mesh_data`; parse response; build BufferGeometry and render. |
| **Junior Engineer 3D** | JR1-504 (performance) | May use `preview_step` (BACK-603) if full-res preview is slow; mesh stats from same payload. |

---

## 5. References

- **RESEARCH/architecture.md** — Mesh data IPC contract (get_mesh_data, BACK-601, BACK-602), ADR-006, ADR-007.
- **RESEARCH/threejs.md** — BufferGeometry, Points, attribute layout.
- **SPRINT_1_7_Task_Assignment.md** — UI-503, UI-504, BACK-601, BACK-602, coordination row.
