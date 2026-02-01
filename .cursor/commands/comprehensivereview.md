# Comprehensive Review Command

This command guides reviewers through architecture, security, and release readiness for **SimplePicture3D**.

## Sections

1. **Scope & changes** — What changed (Rust, Python, frontend) and impact
2. **Security & license checks**
   - `cargo audit` (Rust)
   - `npm audit` (frontend)
   - `pip-audit` or `pip check` (Python)
   - License compatibility (MIT, Apache-2.0; no GPL)
   - prd.md §8 (privacy, model integrity)
3. **Tests & CI status**
   - Rust: `cargo test`
   - Python: `pytest`
   - Frontend: `npm test` (Vitest/Jest)
   - Integration: full pipeline (image → STL)
4. **Packaging & artifact validation**
   - Tauri build: `cargo tauri build`
   - Installer/platform checks
5. **Approvals & sign-offs** — Architect, Security, QA

*Adapted for SimplePicture3D tech stack (Tauri + Rust + Python + Svelte/React).*
