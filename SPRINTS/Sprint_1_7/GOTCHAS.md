# Sprint 1.7 — GOTCHAS

**Sprint:** 1.7 — 3D Preview Rendering  
**Purpose:** Sprint-specific debugging findings and workarounds. Merge useful items to `RESEARCH/GOTCHAS.md` at sprint close.

---

## Format

For each finding, add a short entry:

```markdown
### [Short title]
- **What:** [What went wrong or was surprising]
- **Fix / workaround:** [What to do]
- **Task/area:** [e.g. UI-504, BACK-602]
```

---

## Entries

*(Team adds entries as issues are found.)*

### BACK-603 LOD: Optional preview step implemented
- **What:** Task allowed "implement vs defer" for LOD. Performance target (30+ FPS for 100K) will be validated by JR1-504/QA-604.
- **Decision:** Implemented optional `preview_step` on `get_mesh_data`: when frontend passes a step (e.g. 2), backend returns reduced-detail mesh (step_x = step_y = step). Full-detail export path (Sprint 1.8) unchanged. If JR1-504 finds full-res preview is sufficient, frontend can omit the param; if not, frontend can request e.g. `preview_step: 2` for large images.
- **Task/area:** BACK-603

### 3D preview: mesh upside-down (image Y vs Three.js Y)
- **What:** Backend uses row-major positions with `y_mm = row * pixel_to_mm` (image top = 0, bottom = height). Three.js uses Y-up, so mesh appeared upside-down relative to the source image.
- **Fix / workaround:** In Preview3D.svelte `buildPointCloud()`, flip Y when building the position buffer: `threeJsY = maxY - backendY` (and flip normal Y to match). Image top then appears at the top of the 3D viewport.
- **Task/area:** UI-504

### 3D preview: Points vs Wireframe/Solid toggle had no visible change
- **What:** Wireframe and Solid were placeholders; the point cloud stayed visible for all modes so the toggle appeared to do nothing.
- **Fix / workaround:** When Wireframe or Solid is selected, set `pointCloud.visible = false` and show an overlay message: "Wireframe/Solid mode requires a triangulated mesh (Sprint 1.8). Use Points for now." So Points = mesh visible, Wireframe/Solid = mesh hidden + message.
- **Task/area:** UI-506

### 3D grid floor scale (JR1-502)
- **What:** Grid must provide scale reference; mesh positions from backend are in mm.
- **Fix / workaround:** GridHelper(400, 20) with 1 unit = 1 mm: grid is 400 mm × 400 mm, 20 divisions = 20 mm spacing. Documented in Preview3D.svelte and here. Mesh bounds (JR1-503) also in mm for consistency.
- **Task/area:** JR1-502

---

**Document Version:** 1.0  
**Merged to RESEARCH/GOTCHAS.md:** 2026-02-08 (all entries above).
