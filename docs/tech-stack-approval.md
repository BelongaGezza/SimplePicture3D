# Tech Stack Approval — SimplePicture3D

**Status:** Approved  
**Date:** 2026-02-01  
**Owner:** System Architect  
**Source:** prd.md §5.1; RESEARCH/*.md; SPRINTS/Sprint_1_1/RESEARCH_REFRESH_SUMMARY.md

---

## Summary

The tech stack per prd.md §5.1 is **approved** with the following decisions and clarifications. All decisions align with RESEARCH findings (2026-02-01) and RESEARCHER_TASKING refresh.

---

## Approved Stack

### Shell / Desktop Framework

| Component | Decision | Rationale |
|-----------|----------|-----------|
| **Tauri** | **v2** | Stable (Oct 2024); new projects use v2. Use `create-tauri-app` or `tauri init` with CLI `^2.0.0`. See RESEARCH/tauri.md. |

**Migration note:** If starting from Tauri v1, run `tauri migrate`. Config and capabilities differ between v1 and v2.

---

### Backend (Rust)

| Component | Decision | Versions (from RESEARCH) |
|-----------|----------|---------------------------|
| **Core crates** | image, tokio, serde, anyhow | image 0.25.9, tokio 1.49, serde 1.0.228, anyhow 1.0.100 |
| **Mesh export** | stl_io (STL), obj-exporter + wavefront_obj (OBJ) | stl_io 0.10.0; obj-exporter 0.2.0 |
| **Error handling** | anyhow for application code | Pattern: `Result<T, anyhow::Error>`, `.context()` |

**Note:** Pin versions in Cargo.toml; run `cargo update` periodically.

---

### AI Backend (Python)

| Component | Decision | Rationale |
|-----------|----------|-----------|
| **Python** | 3.10+ | Required for PyTorch 2.7.x |
| **PyTorch** | 2.7.x (stable) | Depth model inference |
| **Depth models** | **Depth-Anything-V2** (primary), MiDaS (alternative) | Depth-V2 recommended; MiDaS repo archived. |
| **Bridge** | Subprocess / Tauri shell plugin | Per RESEARCH/tauri.md; use sidecar or subprocess; file-based transfer for large payloads. |
| **ONNX** | Optional, not for MVP | PyTorch-native inference primary; ONNX later if needed. |

**License note:** Depth-Anything-V2 weights are CC-BY-NC-4.0 (non-commercial). Document in app.

---

### Frontend

| Component | Decision | Rationale |
|-----------|----------|-----------|
| **Framework** | **Svelte** (recommended); React acceptable | RESEARCH/frontend.md: Svelte—smaller bundle, less boilerplate; React if team prefers. Choose one and use consistently. |
| **Build** | Vite | Tauri v2 points at Vite dev server |
| **Styling** | TailwindCSS v3 or v4 | v4 uses `@tailwindcss/vite`; v3 remains stable |
| **3D** | Three.js (r16x–r17x, npm) | BufferGeometry, OrbitControls; use npm, not CDN |
| **Language** | TypeScript | Strict mode in tsconfig.json |

---

### IPC / Data Transfer

| Aspect | Decision |
|--------|----------|
| **Tauri commands** | JSON-serialized; keep payloads small |
| **Large data** | File-based or chunked transfer; avoid multi‑MB JSON over invoke |
| **Python I/O** | Image via temp file; depth map via stdout or output file |

---

## Exceptions / Deviations

| Item | PRD | Decision |
|------|-----|----------|
| MiDaS | Listed as alternative | Repo archived; prefer Depth-Anything-V2. MiDaS remains usable if needed. |
| PyO3 vs subprocess | PRD mentions PyO3 | **Subprocess** chosen: simpler, no Python embedding; aligns with Tauri shell/sidecar. PyO3 can be revisited if tight coupling needed. |

---

## References

- prd.md §5.1
- RESEARCH/tauri.md
- RESEARCH/rust-crates.md
- RESEARCH/python-ml.md
- RESEARCH/frontend.md
- RESEARCH/threejs.md
- SPRINTS/Sprint_1_1/RESEARCH_REFRESH_SUMMARY.md

---

**Signed off:** System Architect (ARCH-005)
