# Research Refresh Summary — Pre–Sprint 1.1

**Date:** 2026-02-01  
**Owner:** Researcher  
**Trigger:** System Architect (RESEARCHER_TASKING.md)

---

## Versions Chosen

| Technology | Choice | Notes |
|------------|--------|------|
| Tauri | **v2** | Stable (Oct 2024); new projects use v2. Use `create-tauri-app` or `tauri init` with CLI `^2.0.0`. |
| Rust crates | image 0.25.9, stl_io 0.10.0, tokio 1.49, serde 1.0.228, anyhow 1.0.100 | Pin in Cargo.toml. |
| OBJ export | **obj-exporter** 0.2.0 + **wavefront_obj** | Or write ASCII OBJ manually from vertex/index buffers. |
| PyTorch | 2.7.x (stable) | Python 3.10+ required. |
| Frontend | **Svelte** recommended; React acceptable | Both work with Tauri v2 + Vite. |
| Tailwind | v3 or v4 | v4 uses `@tailwindcss/vite` and CSS-first config. |
| Three.js | r16x–r17x (npm) | Use npm; OrbitControls from addons (not standalone packages). |

---

## Breaking / Migration Notes

- **Tauri v1 → v2:** Config and capabilities differ; run `tauri migrate` and update `tauri.conf.json` and `capabilities/default.json`.
- **Tauri IPC:** Large payloads (e.g. depth/mesh) should use file-based or chunked transfer, not large JSON over invoke.
- **MiDaS:** Repo archived (read-only); prefer Depth-Anything-V2 for new work.
- **Depth-Anything-V2 weights:** CC-BY-NC-4.0 (non-commercial); document in app.

---

## Recommendations

1. **Shell / Python:** Use Tauri v2 **shell plugin** and **sidecar** for Python (or subprocess); grant `shell:allow-spawn` / `shell:allow-execute` in capabilities.
2. **Frontend:** Choose Svelte or React once; document in architecture. Use Vite; Tauri points at dev server and `dist`.
3. **ONNX:** Optional later; PyTorch-native inference is primary for Sprint 1.1.

---

## Deliverables Completed

- [x] RESEARCH/tauri.md — v2 recommendation, IPC limits, subprocess/sidecar, Official Sources, Last checked
- [x] RESEARCH/rust-crates.md — crate versions, OBJ (obj-exporter + wavefront_obj), Official Sources, Last checked
- [x] RESEARCH/python-ml.md — PyTorch, Depth-V2, MiDaS, ONNX note, Python 3.10+, Official Sources, Last checked
- [x] RESEARCH/frontend.md — Svelte vs React (recommend Svelte), Tailwind v3/v4, Official Sources, Last checked
- [x] RESEARCH/threejs.md — version, BufferGeometry/OrbitControls, Official Sources, Last checked
- [x] RESEARCH/architecture.md — aligned with prd §5.2–5.4, Last checked
- [x] RESEARCH/README.md — index Last updated dates
- [x] RESEARCH/GOTCHAS.md — Tauri IPC large payloads, shell/sidecar, MiDaS archived

Researcher marks this tasking **complete**.
