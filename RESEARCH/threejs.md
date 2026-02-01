# Three.js

**Purpose:** Three.js guidance for 3D mesh preview in SimplePicture3D.

## Official Sources

| Source | URL | Last Checked |
|--------|-----|--------------|
| Three.js Docs | https://threejs.org/docs/ | — |
| Examples | https://threejs.org/examples/ | — |
| GitHub | https://github.com/mrdoob/three.js | — |

*Researcher: Update "Last Checked" when verifying.*

---

## Project Usage

- **Rendering:** Point cloud or triangulated mesh in Tauri webview
- **Data:** Vertex array from Rust via Tauri command
- **Controls:** OrbitControls (rotate, zoom, pan)
- **Modes:** Wireframe, solid, points
- **Performance:** LOD for large meshes; target 30+ FPS for 100K vertices

---

## Key APIs

- `BufferGeometry` from vertex positions (and optionally normals)
- `Points` or `Mesh` with `PointsMaterial` / `MeshBasicMaterial`
- `OrbitControls` from `three/addons/controls/OrbitControls.js` (or npm equivalent)

---

## Research Tasks (Researcher)

- [ ] Three.js ES modules vs UMD in project setup
- [ ] Memory limits for large BufferGeometry (1M+ vertices)
- [ ] Instanced rendering for point clouds (if needed)
- [ ] Version changes, deprecated APIs since knowledge cutoff
