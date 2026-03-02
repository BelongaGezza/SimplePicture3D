# Sprint 2.2 — Gotchas

**Purpose:** Sprint-specific debugging/reproducibility notes. Merge useful entries to `RESEARCH/GOTCHAS.md` when sprint closes.  
**Last Updated:** 2026-03-01

---

## Test / verification

- **`undo::tests::push_clears_redo` fails (undo.rs:176):** Assertion `hist.can_redo()` fails. Indicates redo stack is cleared on push but test expects redo to be available in a specific scenario. Backend/Senior Engineer to confirm intended behavior and fix test or implementation. Blocks verification sign-off until resolved.

---

## 3D / Mesh (Junior Engineer 3D verification 2026-03-01)

- **Wireframe/Solid (UI-1403):** Backend `depth_to_point_cloud` in `mesh_generator.rs` always produces `MeshData.indices` when grid has ≥2 rows and ≥2 columns (via `triangulate_grid`). Preview3D already builds a triangulated `THREE.Mesh` from indices (`buildTriangulatedMesh`) and toggles Wireframe/Solid material. So for normal depth maps, Wireframe and Solid modes are **functional**; the overlay ("Wireframe/Solid mode requires a triangulated mesh") only appears in the edge case where the mesh has no triangles (e.g. single row/column after step). No "Sprint 1.8" user-facing text found in `src/` (UI-1404 may be satisfied or refer to other copy).
- **Mesh tests:** `cargo test -- mesh_generator` — 82 passed, 1 ignored; mesh pipeline and STL/OBJ export tests healthy.

---

*Add further sprint-specific gotchas below; merge to RESEARCH/GOTCHAS.md when done.*
