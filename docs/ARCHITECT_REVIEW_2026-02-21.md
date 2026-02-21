# Architect Review: Project Status, Sprints & Tech Stack Conformance

**Date:** 2026-02-21  
**Role:** System Architect  
**References:** `.cursor/rules/architect.mdc`, `.agents/system-architect.md`, `prd.md`, `todo.md`, `RESEARCH/architecture.md`, RESEARCH tech stack docs

---

## 1. Executive Summary

The project is **beyond planning** and in active Phase 1 MVP implementation. Phase 1 is approximately **60–65% complete** per PRD §2.4; Sprints 1.1–1.10 are delivered with solid traceability and artefact discipline. The codebase largely conforms to the drafted tech stack and architecture, with a few documented deviations and minor documentation gaps. **Recommendation:** Complete Sprint 1.6A remaining items, then proceed to 1.11 (E2E/polish) and 1.12 (docs/beta) with a short README/status refresh.

---

## 2. Current Project Status

### 2.1 Phase and Progress

| Source | Statement |
|-------|-----------|
| **prd.md §2.4** | Phase 1 MVP ~60–65% complete; Sprints 1.1–1.6 delivered at last update; 6 sprints remain before Phase 1 exit. |
| **todo.md** | Sprints 1.1–1.10 marked complete (with some 1.6A and 1.10 items deferred). 1.6A (QA/hardening), 1.11 (E2E), 1.12 (docs) remain. |
| **Codebase** | Full pipeline present: image load, depth generation (Python bridge), depth adjustments, mesh generation (point cloud + grid triangulation), STL/OBJ export, settings, first-run model wizard, 3D preview (Three.js). |

**Conclusion:** Status is **ahead of** the “planning phase” wording in README.md. README should be updated to “In Development — Phase 1 MVP (~65% complete)” and the roadmap dates aligned with todo.md.

### 2.2 Delivered vs Planned

- **Delivered:** Project setup (1.1), image load (1.2), Python bridge (1.3), depth map generation & preview (1.4), manual depth adjustments (1.5), mesh generation (1.6), hardening/QA start (1.5A), 3D preview (1.7), STL export (1.8), OBJ & settings (1.9), model installer & first-run (1.10).
- **Outstanding for Phase 1:** Sprint 1.6A (QA-501–504, coverage enforcement, lib.rs extraction, IPC/memory notes), Sprint 1.11 (E2E, bug triage), Sprint 1.12 (user/dev docs, beta prep), Phase 1 exit gate.

---

## 3. Sprint Conformance

### 3.1 Process and Artefacts

- **Sprint creation:** todo.md defines process (folder `SPRINTS/Sprint_X_Y/`, template, role assignment, artefact storage). Followed for Sprints 1.1–1.10 and 1.5A.
- **Artefacts present:** Task assignments, progress reports, verification checklists, manual test reports, security sign-offs, and GOTCHAS where applicable. Sprint-specific GOTCHAS are merged into RESEARCH/GOTCHAS.md (e.g. Three.js Y-flip, Tauri version pinning).
- **Traceability:** Task IDs (BACK-xxx, ARCH-xxx, UI-xxx, QA-xxx, SEC-xxx) are used in code comments and in RESEARCH/architecture.md ADRs.

### 3.2 Dependencies and Decisions

- **Sprint 1.6A:** Triangulation approach decided in ADR-008 (grid-based in `mesh_generator.rs`); IPC and memory items partially addressed (IPC spike doc in 1.5A). Remaining: formal QA-501–504 sign-off, coverage enforcement (--fail-under), lib.rs extraction.
- **Sprint 1.7/1.8:** 3D preview and STL export implemented; OBJ and settings in 1.9; model wizard in 1.10. Dependencies (triangulation, IPC format) were respected.

### 3.3 Recommendations

- Close 1.6A explicitly: complete QA-501–504, set coverage threshold (e.g. 65% then 70%), and document lib.rs extraction or defer with a ticket.
- Before 1.11, confirm Phase 1 exit criteria in todo.md (e.g. “All MVP features functional”, “Security sign-off”, “CI green”) and tick them as 1.11/1.12 complete.

---

## 4. Tech Stack Conformance

### 4.1 PRD §5.1 and RESEARCH Alignment

| Layer | PRD / RESEARCH | As-Built | Conformance |
|-------|----------------|----------|-------------|
| **Shell** | Tauri | Tauri 2 (Cargo.toml, package.json pinned ~2.9) | ✅ |
| **Frontend** | Svelte or React, TypeScript, TailwindCSS | Svelte 4, TypeScript, TailwindCSS 3, Vite | ✅ ADR-001 |
| **3D** | Three.js or Babylon.js | Three.js (package.json) | ✅ |
| **Backend** | Rust (image, stl_io, tokio, serde, anyhow) | image, tokio, serde, anyhow, env_logger, log, base64; **no stl_io** | ⚠️ See 4.2 |
| **AI** | Python, PyTorch, Depth-Anything-V2 / MiDaS | Python subprocess, depth_estimator, Hugging Face / stub mode | ✅ ADR-002, ADR-003 |
| **IPC** | Tauri commands, subprocess for Python | Tauri commands (lib.rs), tauri-plugin-shell for Python | ✅ |
| **Export** | stl_io, obj crates | Custom binary STL and ASCII OBJ in `mesh_generator.rs` | ⚠️ See 4.2 |

### 4.2 Deviations and Rationale

**STL/OBJ implementation (no `stl_io` / no separate `exporters/`):**

- **PRD §5.4 / RESEARCH/architecture.md:** Mention `exporters/stl.rs`, `exporters/obj.rs` and RESEARCH/rust-crates.md recommends `stl_io` for binary STL.
- **As-built:** STL and OBJ are implemented inside `mesh_generator.rs` (binary STL and ASCII OBJ with .mtl). No `stl_io` crate; no `exporters/` module.
- **Assessment:** Acceptable. ADR-008 chose grid triangulation in `mesh_generator.rs` and “no new crate”; a single module for mesh + export keeps coupling local. RESEARCH/rust-crates.md and RESEARCH/architecture.md should be updated to state that the project uses **custom** STL/OBJ writers in `mesh_generator.rs` rather than `stl_io`/separate exporters, so future contributors and the Researcher don’t assume the crate.

**ndarray:**

- PRD §5.1 lists “Depth Map Processing: ndarray” in data processing. The Rust backend uses `Vec<f32>` (and slices) for depth; no `ndarray` dependency.
- **Assessment:** Fine. RESEARCH/rust-crates.md already notes “Optional: ndarray … not required if processing with raw Vec<f32>”. No change needed.

### 4.3 Frontend Stack

- **Svelte 4:** Matches ADR-001. `package.json` pins `@testing-library/svelte` ^4.2.0 per RESEARCH/frontend.md (Svelte 5 / v5 testing library gotcha).
- **TailwindCSS:** v3 in use; RESEARCH documents v3/v4. No issue.
- **Three.js:** In use; GOTCHAS document Y-flip and Points vs Wireframe/Solid behavior.

### 4.4 Python Stack

- **Subprocess:** ADR-002. Python bridge via shell plugin; depth map via temp file/CLI. Consistent with RESEARCH/python-ml.md and architecture.
- **Models:** Depth-Anything-V2 and MiDaS; ADR-004, ADR-005. First-run wizard and license messaging align with commercial/non-commercial guidance.

### 4.5 Security and Quality

- **Rust:** Path canonicalization and extension checks for export (lib.rs) address SEC-401/SEC-402. Security sign-offs present for relevant sprints.
- **CI:** Backend (build, test, clippy), frontend (build, test), Python (pytest stub mode), audits (cargo audit, npm audit). Coverage currently advisory (continue-on-error); 1.6A is to enforce thresholds.

---

## 5. Repository and Documentation Conformance

### 5.1 File Structure (PRD §5.4)

- **src-tauri/src:** main.rs, lib.rs, image_loading.rs, file_io.rs, depth_adjust.rs, mesh_generator.rs, python_bridge.rs, settings.rs. No `depth_map.rs` as a separate module (depth state lives in lib.rs + python_bridge types); no `exporters/` (see 4.2).
- **Frontend:** App.svelte, components (ImageImport, DepthControls, DepthMapPreview, Preview3D, ExportPanel, FirstRunWizard, Button), lib (tauri, depthCanvas), tests. Aligned with PRD layout.
- **python/:** depth_estimator, model download, requirements; structure matches.
- **docs/:** architecture.md, user-guide.md, threat-model, security-checklist, tech-stack-approval, etc. RESEARCH/ holds ADRs and technical guidance; docs/ holds user- and security-facing material. Appropriate.

### 5.2 License Headers (PRD §9.1)

- PRD requires: `// Copyright (c) 2026 SimplePicture3D Contributors` and `SPDX-License-Identifier: MIT` in source files.
- **Rust (src-tauri/src):** No standard license headers found in the sampled files.
- **Frontend:** No standard license headers in the sampled .svelte/.ts files.
- **Recommendation:** Add the PRD license header to all new and, in a single pass, existing source files (Rust, TypeScript, Svelte, Python) for conformance and reuse compliance.

---

## 6. Recommendations Summary

| Priority | Action |
|----------|--------|
| **High** | Update README.md: set status to “In Development — Phase 1 MVP (~65% complete)” and align roadmap with todo.md. |
| **High** | Complete Sprint 1.6A: QA-501–504, coverage enforcement, document or do lib.rs extraction. |
| **Medium** | Update RESEARCH/architecture.md and RESEARCH/rust-crates.md: state that STL/OBJ are custom implementations in `mesh_generator.rs` (no `stl_io`, no `exporters/`). |
| **Medium** | Add PRD §9.1 license headers to all source files. |
| **Low** | In RESEARCH/architecture.md repository structure, optionally add a one-line note that “exporters/” was consolidated into mesh_generator for STL/OBJ. |

---

## 7. Conclusion

The project is in good shape for Phase 1: sprint process and artefacts are consistent, tech choices match ADRs and RESEARCH, and the main deviations (custom STL/OBJ, no exporters/) are justified and should be documented. Addressing the high- and medium-priority items above will bring status communication and drafting standards to full conformance and prepare the project for the Phase 1 exit gate.

---

*Review conducted per architect persona and prd.md / todo.md / RESEARCH/architecture.md. For follow-up, see `.agents/system-architect.md` and RESEARCH/AI_DEVELOPMENT_GUIDE.md.*

**Updates applied (2026-02-21):** README status/roadmap updated; RESEARCH/architecture.md and rust-crates.md updated for STL/OBJ as-built; PRD §9.1 license headers added to all Rust, frontend, and Python source files; CI coverage step now uses `--fail-under 63` (Sprint 1.6A QA-506/507).
