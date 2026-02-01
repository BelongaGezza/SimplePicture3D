# Three.js

**Purpose:** Three.js guidance for 3D mesh preview in SimplePicture3D.

## Official Sources

| Source | URL | Last Checked |
|--------|-----|--------------|
| Three.js Docs | https://threejs.org/docs/ | 2026-02-01 |
| Examples | https://threejs.org/examples/ | 2026-02-01 |
| GitHub | https://github.com/mrdoob/three.js | 2026-02-01 |
| npm | https://www.npmjs.com/package/three | 2026-02-01 |

---

## Version and Setup

- **Current:** r160+ through r170+ (e.g. 0.160.0–0.173.0 on npm). Use a recent **r16x or r17x** release from npm for Tauri webview.
- **Usage:** Prefer **npm** (`npm install three`) and ES modules; import from `'three'` and addons from `'three/examples/jsm/...'` or `'three/addons/...'` (path may vary by release — check examples in docs).
- **CDN:** Optional for quick tests; for production with Tauri, bundle via Vite and npm.

---

## Key APIs for SimplePicture3D

- **BufferGeometry:** Build from vertex data (positions, optionally normals). e.g. `BufferAttribute`, `BufferGeometry.setAttribute('position', ...)`, `setIndex()` for indexed mesh.
- **Points / Mesh:** Use `Points` for point clouds with `PointsMaterial`; use `Mesh` with `BufferGeometry` and `MeshBasicMaterial` (or `MeshLambertMaterial`) for solid/wireframe.
- **OrbitControls:** Import from Three.js addons (e.g. `three/examples/jsm/controls/OrbitControls.js` or `three/addons/controls/OrbitControls.js`). Usage: `new OrbitControls(camera, renderer.domElement)`; enable damping if desired; call `controls.update()` in the animation loop.
- **Scene / Camera / Renderer:** Standard `Scene`, `PerspectiveCamera`, `WebGLRenderer`; append `renderer.domElement` to DOM. Resize and projection on window resize.

---

## Project Usage

- **Rendering:** Point cloud or triangulated mesh in Tauri webview.
- **Data:** Vertex array from Rust via Tauri command (or path to binary); build BufferGeometry in frontend.
- **Controls:** OrbitControls (rotate, zoom, pan).
- **Modes:** Wireframe, solid, points (toggle material or geometry type).
- **Performance:** LOD or reduced vertex count for large meshes; target 30+ FPS for ~100K vertices.

---

## Deprecation Note

- Standalone npm packages `three-orbitcontrols` / `three-orbit-controls` are deprecated. Use the built-in addon from the `three` package (see docs for exact import path for your version).

---

## Research Tasks (Researcher)

- [x] Version and npm vs CDN — use npm, r16x/r17x.
- [x] BufferGeometry, OrbitControls, scene/camera/renderer — documented above.
- [x] Official sources and Last checked — in table above.
