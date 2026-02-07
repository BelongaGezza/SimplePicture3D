# SimplePicture3D - Development TODO & Sprint Planning

**Version:** 1.2  
**Last Updated:** February 7, 2026  
**Repository:** https://github.com/[org]/SimplePicture3D  
**Project Board:** GitHub Projects (Kanban)

---

## Table of Contents
1. [Sprint Creation Process](#sprint-creation-process)
2. [Phase Overview](#phase-overview)
3. [Team Roles & Responsibilities](#team-roles--responsibilities)
4. [Phase 1: MVP (Minimum Viable Product)](#phase-1-mvp-minimum-viable-product)
5. [Phase 2: Enhanced UX](#phase-2-enhanced-ux)
6. [Phase 3: Cross-Platform Parity](#phase-3-cross-platform-parity)
7. [Phase 4: Production Polish](#phase-4-production-polish)
8. [Beyond Phase 4: Far-Future Roadmap](#beyond-phase-4-far-future-roadmap)
9. [Testing Strategy](#testing-strategy)
10. [CI/CD Pipeline](#cicd-pipeline)
11. [Release Checklist](#release-checklist)
12. [Dependencies & Risks](#dependencies--risks)

---

## Sprint Creation Process

**When an agent is asked to generate a sprint from this todo list, follow this process:**

### 1. Create the sprint folder
Create a folder for the sprint: `SPRINTS/Sprint_X_Y/`  
Example: For Sprint 1.1 → `SPRINTS/Sprint_1_1/`

### 2. Use the tasking template
- **Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`
- **Output file:** `SPRINTS/Sprint_X_Y/SPRINT_X_Y_Task_Assignment.md`
- Copy the template and populate it with:
  - Sprint goal, duration, phase (from the relevant Sprint N.X section below)
  - Role Assignment table — map each task from the sprint to a role (System Architect, Senior Engineer, UI Designer, Researcher, Junior Engineer 2D, Junior Engineer 3D, Security Specialist, Documentation Specialist)
  - Task Breakdown — one block per task, with Task ID, Assigned Role, Dependencies, Acceptance Criteria
  - Auto-allocation: Each task in todo.md has a suggested role (e.g. "Senior Engineer:", "UI Specialist:"); use these to populate the Role Assignment "Owned Tasks" column

### 3. Auto-assignment of agent personas
The Role Assignment table enables agents to claim roles:
- **Available** — Any agent can claim by setting Status to "In Progress" and adding their session ID
- Agents read the Persona File (e.g. `.agents/senior-engineer.md`) and adopt that identity
- Task ownership flows from the table: each role has explicit "Owned Tasks"
- Tasks which have Status "In Progress", "Complete" or "Blocked" cannot be claimed
- Agents should set Status to "Complete" when the task has been completed, or "Blocked" if they are unable to complete the task.

### 4. Artefact storage
**All tasking, reporting, verification, and approval artefacts for the sprint MUST be stored in that sprint's folder:**

| Artefact | Path |
|----------|------|
| Task Assignment | `SPRINTS/Sprint_X_Y/SPRINT_X_Y_Task_Assignment.md` |
| Progress Report | `SPRINTS/Sprint_X_Y/PROGRESS_REPORT.md` |
| Manual Test Report | `SPRINTS/Sprint_X_Y/MANUAL_TEST_REPORT.md` |
| Verification Checklist | `SPRINTS/Sprint_X_Y/VERIFICATION_CHECKLIST.md` |
| Architect Approval | `SPRINTS/Sprint_X_Y/ARCHITECT_APPROVAL.md` (if phase gate) |
| Security Sign-off | `SPRINTS/Sprint_X_Y/SECURITY_SIGNOFF.md` (if security review) |
| Gotchas | `SPRINTS/Sprint_X_Y/GOTCHAS.md` (merge to `RESEARCH/GOTCHAS.md` when done) |

### 5. References
- **Source of sprint content:** This todo.md — locate the "Sprint N.X" section
- **Template location:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`
- **Persona files:** `.agents/*.md`
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`

---

## Phase Overview

### Milestone Summary

| Phase | Focus | Deliverable | Est. Duration |
|-------|-------|-------------|---------------|
| **Phase 1** | MVP | Functional Windows app with core features | 8-10 sprints (16-20 weeks) |
| **Phase 2** | Enhanced UX | Advanced controls, presets, undo/redo | 4-6 sprints (8-12 weeks) |
| **Phase 3** | Cross-Platform | macOS and Linux builds | 3-4 sprints (6-8 weeks) |
| **Phase 4** | Production | Performance, docs, installers, v1.0 release | 4-5 sprints (8-10 weeks) |

**Total Estimated Timeline:** 19-25 sprints (38-50 weeks / ~9-12 months)

**Sprint Duration:** 2 weeks (10 working days)

**Architect Phase Validation (2026-02-01):**
- Phasing is **validated** against `prd.md` §2.3 (Strategic Goals) and §4 (Feature Requirements).
- Phase 1 (MVP) → Phase 2 (Enhanced UX) → Phase 3 (Cross-Platform) → Phase 4 (Production) aligns with PRD feature groups F1.x, F2.x, F3.x, F4.x.
- Sprint 1.1 scope (project setup, Tauri + Rust + Python + frontend scaffolding, CI, security baseline) is appropriate as foundation; no reordering recommended.
- **Pre-Sprint 1.1:** Researcher must refresh RESEARCH/ for knowledge-cutoff → today (see `SPRINTS/Sprint_1_1/RESEARCHER_TASKING.md`). Senior Engineer must produce detailed Sprint 1.1 task assignment (see `SPRINTS/Sprint_1_1/SPRINT_1_1_Task_Assignment.md`).

**Team Composition:**
- 1× System Architect
- 1× Senior Engineer (Rust/Backend)
- 1× Senior Researcher (AI/ML)
- 1× UI Specialist (Frontend)
- 2× Junior Engineers (Full-stack)
- 1× Security Engineer (part-time, reviews per sprint)
- 1× Quality Engineer (Testing lead)

---

## Team Roles & Responsibilities

### System Architect
- **Primary:** Architecture decisions, tech stack selection, code reviews
- **Secondary:** Mentoring junior engineers, spike/POC development
- **Tools:** Cursor IDE, draw.io (architecture diagrams), GitHub reviews
- **Ownership:** `docs/architecture.md`, Rust module structure

### Senior Engineer (Backend)
- **Primary:** Rust backend development, mesh generation algorithms
- **Secondary:** Python-Rust bridge, performance optimization
- **Tools:** Cursor IDE, Cargo, PyO3, profiling tools (perf, Instruments)
- **Ownership:** `src-tauri/src/mesh_generator.rs`, `src-tauri/src/exporters/`

### Senior Researcher (AI/ML)
- **Primary:** AI model integration, depth estimation pipeline
- **Secondary:** Model optimization (quantization, ONNX conversion)
- **Tools:** Cursor IDE, Jupyter notebooks, PyTorch, Hugging Face
- **Ownership:** `python/depth_estimator.py`, model download scripts

### UI Specialist (Frontend)
- **Primary:** Svelte/React components, 3D preview implementation
- **Secondary:** UX design, accessibility, responsive layouts
- **Tools:** Cursor IDE, Figma (mockups), Three.js, browser DevTools
- **Ownership:** `src/components/`, `src/styles/`, Tauri IPC interfaces

### Junior Engineers (2×)
**Junior Engineer #1 (Frontend-focused):**
- **Tasks:** Component implementation, UI bug fixes, testing
- **Learning Goals:** TypeScript, Svelte, WebGL basics
- **Mentored by:** UI Specialist

**Junior Engineer #2 (Backend-focused):**
- **Tasks:** File I/O, settings persistence, unit tests
- **Learning Goals:** Rust, async programming, debugging
- **Mentored by:** Senior Engineer

### Security Engineer (Part-time)
- **Primary:** Security reviews, dependency audits, threat modeling
- **Cadence:** 1 day per sprint (Sprint Review day)
- **Tools:** `cargo audit`, `npm audit`, SAST scanners
- **Ownership:** Security checklist sign-off per phase

### Quality Engineer (QA Lead)
- **Primary:** Test planning, automated test implementation, manual testing
- **Secondary:** CI/CD pipeline maintenance, bug triage
- **Tools:** Cursor IDE, pytest, Cargo test, Playwright (UI tests)
- **Ownership:** `tests/` directory, test documentation; **Testing requirements backlog** (see [Testing Strategy](#testing-strategy) — current state, gaps, and CI requirements)

---

## Phase 1: MVP (Minimum Viable Product)

**Goal:** Deliver a functional Windows desktop application that can convert a 2D image to an STL file using AI depth estimation.

**Exit Criteria:**
- ✅ User can load PNG/JPG, generate depth map, export STL on Windows
- ✅ Automated tests passing: >70% coverage on core logic
- ✅ Manual testing: 5+ beta testers successfully complete first conversion
- ✅ Documentation: README.md with installation and usage instructions
- ✅ Security review: No critical vulnerabilities, dependency audit clean
- ✅ GitHub repository public with MIT license

---

### Sprint 1.1: Project Setup & Foundations (2 weeks)

**Sprint Goal:** Establish development environment, repo structure, and basic build pipeline.

#### Tasks

**System Architect:**
- [ ] **ARCH-001:** Define repository structure (see PRD §5.4)
- [ ] **ARCH-002:** Create `docs/architecture.md` with diagrams
- [ ] **ARCH-003:** Set up monorepo structure (Tauri + Python)
- [ ] **ARCH-004:** Configure Cursor IDE workspace settings
- [ ] **ARCH-005:** Review and approve tech stack decisions

**Senior Engineer:**
- [ ] **BACK-001:** Initialize Tauri project with Rust backend
- [ ] **BACK-002:** Set up Cargo.toml with dependencies (image, tokio, serde)
- [ ] **BACK-003:** Create placeholder IPC commands (load_image, export_stl)
- [ ] **BACK-004:** Implement basic error handling pattern (anyhow crate)
- [ ] **BACK-005:** Set up Rust testing framework (cargo test)

**Senior Researcher:**
- [ ] **AI-001:** Research Depth-Anything-V2 vs MiDaS performance benchmarks
- [ ] **AI-002:** Set up Python virtual environment (requirements.txt)
- [ ] **AI-003:** Prototype depth estimation script (standalone CLI)
- [ ] **AI-004:** Test model download from Hugging Face
- [ ] **AI-005:** Document Python dependencies and setup instructions

**UI Specialist:**
- [ ] **UI-001:** Initialize Svelte frontend (or React if preferred)
- [ ] **UI-002:** Set up TailwindCSS and basic layout
- [ ] **UI-003:** Create placeholder components (ImageImport, Preview3D)
- [ ] **UI-004:** Configure Tauri frontend-backend IPC
- [ ] **UI-005:** Set up development hot-reload workflow

**Junior Engineer #1:**
- [ ] **JR1-001:** Create Figma wireframes for main workspace (with UI Specialist)
- [ ] **JR1-002:** Implement basic button components (Load, Export)
- [ ] **JR1-003:** Write TypeScript types for Tauri commands
- [ ] **JR1-004:** Learn Svelte tutorial (onboarding)

**Junior Engineer #2:**
- [ ] **JR2-001:** Set up local development environment (Rust, Node, Python)
- [ ] **JR2-002:** Write file I/O utility functions (read/write temp files)
- [ ] **JR2-003:** Create logging configuration (env_logger)
- [ ] **JR2-004:** Learn Rust basics (The Rust Book, chapters 1-5)

**Quality Engineer:**
- [x] **QA-001:** Set up GitHub Actions CI workflow (build + test) — *Done; add clippy step (see Testing requirements).*
- [ ] **QA-002:** Create test plan template for sprints
- [ ] **QA-003:** Configure code coverage reporting (tarpaulin for Rust, coverage.py for Python)
- [ ] **QA-004:** Document local testing commands (README)
- [ ] **QA-005:** Add `cargo clippy` to CI backend job (fail on warnings)

**Security Engineer:**
- [ ] **SEC-001:** Initial threat model review (see PRD §8.1)
- [ ] **SEC-002:** Set up `cargo audit` and `npm audit` in CI
- [ ] **SEC-003:** Create security checklist for dependency reviews

#### Exit Criteria
- ✅ Repository builds successfully on Windows
- ✅ Basic Tauri window opens with "Hello World" UI
- ✅ CI pipeline runs and passes (build + lint)
- ✅ All team members can run project locally
- ✅ Architecture document reviewed and approved

---

### Sprint 1.2: Image Loading & Display (2 weeks)

**Sprint Goal:** User can load an image file and see it displayed in the UI.

#### Tasks

**Senior Engineer:**
- [ ] **BACK-101:** Implement `load_image` Tauri command
- [ ] **BACK-102:** Image validation (format, size, integrity)
- [ ] **BACK-103:** Downsample logic for >8K images
- [ ] **BACK-104:** Convert image to RGB if needed (grayscale/RGBA → RGB)
- [ ] **BACK-105:** Return image dimensions to frontend

**UI Specialist:**
- [ ] **UI-101:** Implement ImageImport component (file picker)
- [ ] **UI-102:** Drag-and-drop support for image files
- [ ] **UI-103:** Display loaded image in preview panel
- [ ] **UI-104:** Show image metadata (dimensions, file size)
- [ ] **UI-105:** Loading spinner during image processing

**Junior Engineer #1:**
- [ ] **JR1-101:** Style file picker button (Tailwind)
- [ ] **JR1-102:** Add file type filter (PNG, JPG) to picker
- [ ] **JR1-103:** Implement drag-and-drop visual feedback
- [ ] **JR1-104:** Test on various image sizes (unit tests)

**Junior Engineer #2:**
- [ ] **JR2-101:** Write unit tests for image loading function
- [ ] **JR2-102:** Handle file path edge cases (Unicode, spaces, long paths)
- [ ] **JR2-103:** Test downsampling logic with 16K test image
- [ ] **JR2-104:** Log image load time (performance monitoring)

**Quality Engineer:**
- [ ] **QA-101:** Create test image dataset (various sizes, formats, corrupt files)
- [ ] **QA-102:** Manual test plan: load images from different sources
- [ ] **QA-103:** Automated test: load valid PNG, verify dimensions
- [ ] **QA-104:** Negative test: load invalid file, verify error message

**Security Engineer:**
- [ ] **SEC-101:** Review image loading for path traversal vulnerabilities
- [ ] **SEC-102:** Validate magic bytes before processing (prevent malicious files)

#### Exit Criteria
- ✅ User can load PNG/JPG via file picker or drag-and-drop
- ✅ Image displays correctly in UI
- ✅ Downsampling works for oversized images
- ✅ Error handling for corrupt/invalid files
- ✅ Automated tests passing (image loading)

---

### Sprint 1.3: Python-Rust Bridge & Model Setup (2 weeks)

**Sprint Goal:** Establish communication between Rust backend and Python AI inference.

#### Tasks

**System Architect:**
- [ ] **ARCH-101:** Design IPC protocol for Rust ↔ Python communication
- [ ] **ARCH-102:** Decide on data format (JSON vs binary for images)
- [ ] **ARCH-103:** Review subprocess vs PyO3 trade-offs
- [ ] **ARCH-104:** Document data flow diagram (image → Python → depth map)

**Senior Engineer:**
- [ ] **BACK-201:** Implement Python subprocess spawner in Rust
- [ ] **BACK-202:** Pass image data to Python (stdin or temp file)
- [ ] **BACK-203:** Parse depth map output (JSON or NumPy binary)
- [ ] **BACK-204:** Handle Python errors and timeouts
- [ ] **BACK-205:** Add progress reporting from Python to Rust

**Senior Researcher:**
- [ ] **AI-201:** Refactor depth estimator to accept stdin image data
- [ ] **AI-202:** Implement depth map output serialization (JSON or msgpack)
- [ ] **AI-203:** Add progress logging (% complete, ETA)
- [ ] **AI-204:** Test with Depth-Anything-V2 model (local download)
- [ ] **AI-205:** Optimize inference for CPU and GPU

**Junior Engineer #2:**
- [ ] **JR2-201:** Write integration test: Rust → Python → Rust roundtrip
- [ ] **JR2-202:** Test subprocess with intentional Python crash (error handling)
- [ ] **JR2-203:** Benchmark image serialization formats
- [ ] **JR2-204:** Document Python environment setup (README)

**Quality Engineer:**
- [ ] **QA-201:** Test Python subprocess on Windows (cmd.exe vs PowerShell)
- [ ] **QA-202:** Verify model download script (checksum validation)
- [ ] **QA-203:** Test with and without GPU (CUDA availability)
- [ ] **QA-204:** Manual test: run depth estimation on sample images

**Security Engineer:**
- [ ] **SEC-201:** Review subprocess for command injection vulnerabilities
- [ ] **SEC-202:** Validate model download URLs (HTTPS only, checksum verification)

#### Exit Criteria
- ✅ Rust can spawn Python subprocess and receive depth map
- ✅ AI model download script works (with checksum)
- ✅ Depth estimation completes on CPU and GPU
- ✅ Error handling for missing model, Python errors
- ✅ Integration test passing

---

### Sprint 1.4: Depth Map Generation & Preview (2 weeks)

**Sprint Goal:** User sees AI-generated depth map displayed in the UI.

#### Tasks

**Senior Researcher:**
- [ ] **AI-301:** Normalize depth map output to 0-1 range
- [ ] **AI-302:** Return depth map as JSON array (height × width)
- [ ] **AI-303:** Benchmark inference time for various image sizes
- [ ] **AI-304:** Document expected depth map format in architecture.md

**Senior Engineer:**
- [ ] **BACK-301:** Implement `generate_depth_map` Tauri command
- [ ] **BACK-302:** Store depth map in Rust state (Arc<Mutex<>>)
- [ ] **BACK-303:** Return depth map to frontend for visualization
- [ ] **BACK-304:** Add progress indicator (0-100%) during inference

**UI Specialist:**
- [ ] **UI-301:** Create DepthMapPreview component
- [ ] **UI-302:** Render depth map as grayscale image (canvas or img)
- [ ] **UI-303:** Add "Generate Depth Map" button
- [ ] **UI-304:** Show progress bar during AI inference
- [ ] **UI-305:** Side-by-side comparison (original vs depth map)

**Junior Engineer #1:**
- [ ] **JR1-301:** Implement canvas rendering for depth map
- [ ] **JR1-302:** Add zoom/pan controls for depth preview
- [ ] **JR1-303:** Test rendering performance with large depth maps
- [ ] **JR1-304:** Add loading skeleton during generation

**Junior Engineer #2:**
- [ ] **JR2-301:** Write unit test for depth map normalization
- [ ] **JR2-302:** Test edge case: all-black or all-white image
- [ ] **JR2-303:** Log depth map statistics (min, max, mean)

**Quality Engineer:**
- [ ] **QA-301:** Manual test: generate depth map on various images
- [ ] **QA-302:** Verify depth map accuracy (qualitative assessment)
- [ ] **QA-303:** Performance test: measure inference time (4K image)
- [ ] **QA-304:** Automated test: depth map dimensions match input

#### Exit Criteria
- ✅ User can click "Generate Depth Map" and see result
- ✅ Depth map displays correctly in UI
- ✅ Progress indicator shows during AI processing
- ✅ Performance meets target (<30s for 4K on GPU)
- ✅ Depth map data structure documented

---

### Sprint 1.5: Manual Depth Adjustments (2 weeks)

**Sprint Goal:** User can modify the AI-generated depth map with sliders and controls.

#### Tasks

**Senior Engineer:**
- [ ] **BACK-401:** Implement depth adjustment functions (brightness, contrast, gamma)
- [ ] **BACK-402:** Apply transformations to depth map array
- [ ] **BACK-403:** Invert depth toggle
- [ ] **BACK-404:** Depth range mapping (0-1 → 2-10mm)
- [ ] **BACK-405:** Cache original depth map for reset function

**UI Specialist:**
- [ ] **UI-401:** Create DepthControls component (sidebar)
- [ ] **UI-402:** Implement sliders: Depth Range, Brightness, Gamma
- [ ] **UI-403:** Invert Depth checkbox
- [ ] **UI-404:** Real-time preview updates (debounced)
- [ ] **UI-405:** Reset button to restore original depth map

**Junior Engineer #1:**
- [ ] **JR1-401:** Style sliders with TailwindCSS
- [ ] **JR1-402:** Add numerical input fields next to sliders
- [ ] **JR1-403:** Implement keyboard controls (arrow keys for sliders)
- [ ] **JR1-404:** Test slider responsiveness

**Junior Engineer #2:**
- [ ] **JR2-401:** Write unit tests for depth adjustment algorithms
- [ ] **JR2-402:** Test boundary conditions (min/max values)
- [ ] **JR2-403:** Benchmark adjustment performance (real-time?)

**Quality Engineer:**
- [ ] **QA-401:** Manual test: adjust all controls, verify preview updates
- [ ] **QA-402:** Test extreme values (brightness 0%, 200%)
- [ ] **QA-403:** Verify reset button restores original depth map
- [ ] **QA-404:** Automated test: apply adjustments, check output array
- [ ] **QA-405:** Add `cargo clippy -- -D warnings` to CI (per Consultant Report Priority 1)

**Senior Researcher (Testing Infrastructure - Consultant Priority 1):**
- [ ] **AI-401:** Create pytest suite for `depth_estimator.py` (stub mode tests)
- [ ] **AI-402:** Add pytest to CI workflow (with SP3D_USE_STUB=1)
- [ ] **AI-403:** Document Python test commands in CLAUDE.md

**System Architect (ADR Documentation - Consultant Priority 3):**
- [ ] **ARCH-101:** Review and approve ADRs in RESEARCH/architecture.md
- [ ] **ARCH-102:** Document Python distribution strategy for README.md

#### Exit Criteria
- ✅ All depth adjustment controls functional
- ✅ Preview updates within 100ms of slider change
- ✅ Reset button works correctly
- ✅ Automated tests cover adjustment logic
- ✅ UI responsive and intuitive
- ✅ `cargo clippy` enforced in CI (Consultant Priority 1)
- ✅ Python pytest suite exists with stub mode tests (Consultant Priority 1)
- ✅ ADRs documented in RESEARCH/architecture.md (Consultant Priority 3)

---

### Sprint 1.6: Mesh Generation Algorithm (2 weeks)

**Sprint Goal:** Convert depth map to 3D point cloud/mesh in Rust.

**Sprint Status:** Implementation complete; QA pending. See `Consultant_Recommendations_Report_7Feb2026.md` §3.1.

#### Tasks

**System Architect:**
- [x] **ARCH-201:** Design mesh generation algorithm (point cloud vs triangulated) — ADR-006 in RESEARCH/architecture.md
- [x] **ARCH-202:** Define vertex format (position, normal, color?) — MeshData with positions + normals
- [x] **ARCH-203:** Document mesh topology requirements for laser engraving — 2.5D, no overhangs
- [x] **ARCH-204:** Review memory efficiency for large meshes — single buffer, 4K ~200MB estimated

**Senior Engineer:**
- [x] **BACK-501:** Implement point cloud generation from depth map — `mesh_generator.rs` (406 lines)
- [x] **BACK-502:** Uniform grid sampling strategy — `MeshParams.step_x/step_y`
- [x] **BACK-503:** Scale vertices to millimeter dimensions (2-10mm Z-axis) — `depth_to_z_mm()`
- [x] **BACK-504:** Optional: Delaunay triangulation for mesh connectivity — Deferred per ADR-006; point cloud for MVP
- [x] **BACK-505:** Calculate vertex normals (for shading) — `normal_from_gradient()` finite difference
- [x] **BACK-506:** Optimize memory usage (streaming, chunking) — single-pass build, `MAX_DIMENSION=8192`

**Junior Engineer #2:**
- [x] **JR2-501:** Write unit tests for point cloud generation — 5×5 bounds, constant depth, gradient tests
- [x] **JR2-502:** Test edge cases (empty depth map, single pixel) — empty rejected, single row/column valid
- [x] **JR2-503:** Benchmark mesh generation (1K, 4K, 8K images) — 1K ~9.3ms, 4K ~73ms (well under 15s target)
- [x] **JR2-504:** Profile memory usage with valgrind/Instruments — Procedure documented; actual measurement TBD

**Quality Engineer:**
- [ ] **QA-501:** Manual test: generate mesh, verify vertex count — *Not started (Consultant Priority 1)*
- [ ] **QA-502:** Validate mesh dimensions match depth range — *Not started (Consultant Priority 1)*
- [ ] **QA-503:** Performance test: mesh generation time (target <15s for 4K) — *Not started (Consultant Priority 1)*
- [ ] **QA-504:** Automated test: mesh statistics (bounds, normals) — *Not started (Consultant Priority 1)*

**Security Engineer:**
- [x] **SEC-301:** Review for integer overflow in vertex calculations — `checked_mul` verified; SECURITY_SIGNOFF.md
- [x] **SEC-302:** Validate depth map inputs before processing — `validate_depth_input()`, MAX_DIMENSION enforced

#### Exit Criteria
- ✅ Mesh generation produces valid point cloud
- ✅ Vertex positions in correct units (mm)
- ✅ Performance meets targets (<15s for 4K) — benchmark: 73ms for 4K
- ⬜ Memory usage within budget (<2GB for 4K) — estimated ~200MB, empirical measurement pending (JR2-504)
- ✅ Algorithm documented in architecture.md — ADR-006, ARCH-201–204
- ⬜ QA tasks QA-501–504 not yet executed — must complete before Sprint 1.7

---

### Sprint 1.6A: QA Completion & Pre-1.7 Hardening (1 week)

**Sprint Goal:** Complete Sprint 1.6 QA, enforce coverage thresholds, improve lib.rs testability, run IPC benchmark, and decide on binary transfer — gating work before 3D preview and export sprints.

**Rationale:** Consultant Recommendations Report (7 Feb 2026) identifies Sprint 1.6 QA as Priority 1 blocker. Additionally, IPC performance and triangulation decisions are critical-path dependencies for Sprints 1.7 and 1.8 respectively.

#### Tasks

**Quality Engineer (Sprint 1.6 QA — Consultant Priority 1):**
- [ ] **QA-501:** Manual test — generate mesh, verify vertex count
- [ ] **QA-502:** Validate mesh dimensions match configured depth range (Z bounds = 2–10mm)
- [ ] **QA-503:** Performance test — mesh generation time on real hardware (target <15s for 4K)
- [ ] **QA-504:** Automated test — mesh statistics (bounds, normals unit length); integrate in CI
- [ ] **QA-505:** Update Sprint 1.6 `VERIFICATION_CHECKLIST.md` and sign off

**Senior Engineer (lib.rs Testability — Consultant Priority 2):**
- [ ] **BACK-507:** Extract business logic from Tauri command handlers (`lib.rs`) into standalone testable functions
- [ ] **BACK-508:** Add unit tests for extracted functions — target lib.rs coverage from 6% to >50%

**Quality Engineer (Coverage Enforcement — Consultant Priority 3):**
- [ ] **QA-506:** Switch `cargo tarpaulin` to `--fail-under 65` (current baseline 63.6%)
- [ ] **QA-507:** Remove `continue-on-error: true` from coverage CI steps
- [ ] **QA-508:** Document plan to increment threshold by 5% each sprint until 70%

**System Architect (Triangulation Planning — Consultant Priority 4):**
- [ ] **ARCH-205:** Document triangulation requirement as Sprint 1.8 dependency
- [ ] **ARCH-206:** Decide: triangulation in `mesh_generator.rs` vs dedicated export module
- [ ] **ARCH-207:** Spike: evaluate `delaunator` crate or grid-based triangulation for uniform grids

**Senior Engineer / Junior Engineer #2 (IPC Binary Transfer — Consultant Priority 5):**
- [ ] **BACK-509:** Run IPC serialization benchmark and document results (640×480, 1080p, 4K)
- [ ] **BACK-510:** If latency >100ms for 1080p: implement binary transfer via temp file
- [ ] **ARCH-208:** Document decision as ADR-007 (IPC transfer mechanism for mesh data)

**Junior Engineer #2 (Memory Profile — Consultant Priority 6):**
- [ ] **JR2-505:** Run mesh generation memory profile on development hardware (fill in JR2-504 results)
- [ ] **JR2-506:** Update Sprint 1.6 GOTCHAS.md with empirical peak memory measurement

#### Exit Criteria
- ✅ Sprint 1.6 QA-501–504 executed and signed off
- ✅ Sprint 1.6 `VERIFICATION_CHECKLIST.md` complete
- ✅ lib.rs coverage improved (>50%)
- ✅ Coverage thresholds enforced in CI (`--fail-under 65`)
- ✅ Triangulation strategy documented for Sprint 1.8
- ✅ IPC benchmark results documented; binary transfer decision made (ADR-007)
- ✅ Memory profile completed with empirical measurements

---

### Sprint 1.7: 3D Preview Rendering (2 weeks)

**Sprint Goal:** Display generated mesh in interactive 3D viewport.

**Dependencies (from Sprint 1.6A / Consultant Report):**
- **IPC Transfer:** ADR-007 decision (Sprint 1.6A BACK-509/510) determines how mesh data flows to frontend. If latency >100ms for 1080p, binary transfer via temp file needed before this sprint.
- **Mesh Format:** Point cloud only (ADR-006). Use `THREE.Points` or `THREE.BufferGeometry` with positions + normals from `get_mesh_data`. Triangulated faces not available until Sprint 1.8.
- **`get_mesh_data` command:** Already implemented in Sprint 1.6 (BACK-501). BACK-601 below updates to use ADR-007 transfer mechanism if binary.

#### Tasks

**UI Specialist:**
- [ ] **UI-501:** Integrate Three.js into Svelte component
- [ ] **UI-502:** Create 3D scene with camera, lights, grid
- [ ] **UI-503:** Load mesh data from Rust (via Tauri IPC — per ADR-007 transfer mechanism)
- [ ] **UI-504:** Render point cloud (BufferGeometry with `THREE.Points`); mesh rendering deferred until triangulation (Sprint 1.8)
- [ ] **UI-505:** Implement orbit controls (rotate, zoom, pan)
- [ ] **UI-506:** Toggle wireframe/solid/point rendering modes (wireframe/solid require triangulated mesh — placeholder until 1.8)

**Junior Engineer #1:**
- [ ] **JR1-501:** Add camera presets (top, front, side views)
- [ ] **JR1-502:** Implement grid floor for scale reference
- [ ] **JR1-503:** Display mesh statistics (vertex count, bounds)
- [ ] **JR1-504:** Test rendering performance with large meshes (>1M vertices)
- [ ] **JR1-505:** Add lighting controls (ambient, directional)

**Senior Engineer:**
- [ ] **BACK-601:** Update `get_mesh_data` Tauri command for ADR-007 transfer mechanism (already exists from Sprint 1.6; adapt if binary transfer chosen)
- [ ] **BACK-602:** Serialize vertex array for frontend (JSON or binary — per ADR-007)
- [ ] **BACK-603:** LOD (Level of Detail) for large meshes (optional)

**Quality Engineer:**
- [ ] **QA-601:** Manual test: rotate/zoom mesh, verify responsiveness
- [ ] **QA-602:** Test on integrated GPU (Intel UHD) vs dedicated GPU
- [ ] **QA-603:** Verify mesh matches depth map (visual inspection)
- [ ] **QA-604:** Performance test: FPS with 100K, 500K, 1M vertices

#### Exit Criteria
- ✅ 3D preview displays mesh correctly
- ✅ Orbit controls smooth and responsive
- ✅ Render modes (wireframe, solid, points) functional
- ✅ Performance target: 30+ FPS for 100K vertices
- ✅ Mesh statistics displayed accurately

---

### Sprint 1.8: STL Export Implementation (2 weeks)

**Sprint Goal:** User can export generated mesh as binary STL file.

**Critical Dependency — Triangulation (from Consultant Report §3.5 / Sprint 1.6A ARCH-205–207):**
- **STL format requires triangulated faces.** The current mesh generator outputs a point cloud (positions + normals). A point cloud cannot be directly written to STL.
- **OBJ format can represent points** but laser engraving software typically expects mesh faces.
- This sprint must implement triangulation (grid-based or Delaunay) either in `mesh_generator.rs` or a dedicated export module, per the decision from Sprint 1.6A ARCH-206.
- **Spike results from ARCH-207** (`delaunator` vs grid-based triangulation) should be available before this sprint begins.

#### Tasks

**System Architect:**
- [ ] **ARCH-301:** Finalize triangulation implementation approach (from ARCH-206/207 spike)

**Senior Engineer:**
- [ ] **BACK-700:** Implement triangulation for point cloud → triangle mesh (grid-based for uniform grid; add to `mesh_generator.rs` or new module per ARCH-206)
- [ ] **BACK-701:** Implement STL binary format writer (stl_io crate) — consumes triangulated mesh
- [ ] **BACK-702:** Validate mesh before export (manifold check, normals)
- [ ] **BACK-703:** Implement `export_stl` Tauri command
- [ ] **BACK-704:** File dialog integration (save location)
- [ ] **BACK-705:** Auto-generate filename (source_image_timestamp.stl)
- [ ] **BACK-706:** Remember last export location (settings persistence)

**UI Specialist:**
- [ ] **UI-701:** Create ExportPanel component (bottom panel)
- [ ] **UI-702:** Export button with file format dropdown (STL, OBJ)
- [ ] **UI-703:** Progress indicator for export (large files)
- [ ] **UI-704:** Success notification with "Open Folder" button

**Junior Engineer #2:**
- [ ] **JR2-701:** Write unit tests for STL writer
- [ ] **JR2-702:** Validate STL output with external tool (MeshLab CLI)
- [ ] **JR2-703:** Test edge cases (empty mesh, single triangle)
- [ ] **JR2-704:** Benchmark export time for large meshes

**Quality Engineer:**
- [ ] **QA-701:** Manual test: export STL, open in MeshLab/PrusaSlicer
- [ ] **QA-702:** Verify dimensions match specified depth range
- [ ] **QA-703:** Test filename generation (timestamp format, Unicode characters)
- [ ] **QA-704:** Automated test: export → re-import → validate mesh

**Security Engineer:**
- [ ] **SEC-401:** Review file path handling (prevent overwriting system files)
- [ ] **SEC-402:** Validate export directory permissions

#### Exit Criteria
- ✅ User can export mesh as binary STL
- ✅ Exported STL opens correctly in external tools
- ✅ Dimensions accurate (±0.1mm tolerance)
- ✅ Export completes within targets (<5s for 1M vertices)
- ✅ Filename auto-generation works correctly

---

### Sprint 1.9: OBJ Export & Settings Persistence (2 weeks)

**Sprint Goal:** Add OBJ export format and save user settings between sessions.

#### Tasks

**Senior Engineer:**
- [ ] **BACK-801:** Implement OBJ ASCII format writer
- [ ] **BACK-802:** Generate material file (.mtl) if needed
- [ ] **BACK-803:** Add OBJ option to export dropdown
- [ ] **BACK-804:** Implement settings persistence (serde JSON)
- [ ] **BACK-805:** Save/load: last export path, depth adjustments, window size

**Junior Engineer #2:**
- [ ] **JR2-801:** Write unit tests for OBJ writer
- [ ] **JR2-802:** Validate OBJ output with external tool (Blender import)
- [ ] **JR2-803:** Test settings save/load (create, modify, restart app)
- [ ] **JR2-804:** Handle corrupt settings file (fallback to defaults)

**UI Specialist:**
- [ ] **UI-801:** Add format selector to export panel
- [ ] **UI-802:** Settings menu (File → Preferences)
- [ ] **UI-803:** Default export location setting
- [ ] **UI-804:** Reset settings button

**Quality Engineer:**
- [ ] **QA-801:** Manual test: export OBJ, import to Blender
- [ ] **QA-802:** Test settings persistence across app restarts
- [ ] **QA-803:** Verify defaults when settings file missing
- [ ] **QA-804:** Automated test: settings round-trip

#### Exit Criteria
- ✅ OBJ export functional and validated
- ✅ User settings persist between sessions
- ✅ Settings UI accessible and intuitive
- ✅ Corrupt settings handled gracefully

---

### Sprint 1.10: Model Installer & First-Run Experience (2 weeks)

**Sprint Goal:** Implement AI model download wizard on first run.

#### Tasks

**Senior Researcher:**
- [ ] **AI-401:** Create model download script (Python or Rust?)
- [ ] **AI-402:** Hugging Face API integration (download with resume)
- [ ] **AI-403:** SHA256 checksum verification
- [ ] **AI-404:** Progress reporting (bytes downloaded, ETA)
- [ ] **AI-405:** Store models in `~/.simplepicture3d/models/`

**Senior Engineer:**
- [ ] **BACK-901:** Implement `download_model` Tauri command
- [ ] **BACK-902:** Detect if models installed (first run check)
- [ ] **BACK-903:** Handle download failures (retry, cancel)
- [ ] **BACK-904:** Background download (non-blocking UI)

**UI Specialist:**
- [ ] **UI-901:** Create FirstRunWizard component
- [ ] **UI-902:** Model download dialog (download now / skip)
- [ ] **UI-903:** Progress bar with speed/ETA
- [ ] **UI-904:** Privacy notice (offline processing message)
- [ ] **UI-905:** Onboarding tour (tooltip overlay, optional)

**Junior Engineer #1:**
- [ ] **JR1-901:** Style wizard with multi-step layout
- [ ] **JR1-902:** Add "Skip" button with confirmation dialog
- [ ] **JR1-903:** Test wizard on slow connection (throttle download)

**Quality Engineer:**
- [ ] **QA-901:** Manual test: complete first run wizard
- [ ] **QA-902:** Test download cancellation (partial download cleanup)
- [ ] **QA-903:** Verify checksum validation (inject corrupt file)
- [ ] **QA-904:** Test app behavior without models (manual mode)

**Security Engineer:**
- [ ] **SEC-501:** Review model download for MITM vulnerabilities (HTTPS enforced)
- [ ] **SEC-502:** Validate checksum source (trusted origin)

#### Exit Criteria
- ✅ First-run wizard appears on initial launch
- ✅ Model download completes successfully
- ✅ Checksum verification works
- ✅ App functions without models (manual mode only)
- ✅ Privacy notice displayed and acknowledged

---

### Sprint 1.11: Integration Testing & Bug Fixes (2 weeks)

**Sprint Goal:** End-to-end testing of MVP workflow, fix critical bugs.

#### Tasks

**Quality Engineer:**
- [ ] **QA-1001:** Create end-to-end test suite (Playwright or manual)
- [ ] **QA-1002:** Test complete workflow: load → generate → adjust → export
- [ ] **QA-1003:** Regression testing: all previous sprint features
- [ ] **QA-1004:** Performance benchmarking: meet all PRD targets
- [ ] **QA-1005:** Create bug report template (GitHub Issues)

**All Engineers:**
- [ ] **BUG-1001:** Triage and fix P0 (critical) bugs
- [ ] **BUG-1002:** Triage and fix P1 (high priority) bugs
- [ ] **BUG-1003:** Document known issues (P2/P3) for Phase 2
- [ ] **BUG-1004:** Code cleanup (remove debug logs, TODO comments)

**System Architect:**
- [ ] **ARCH-301:** Review codebase for architectural issues
- [ ] **ARCH-302:** Refactor hotspots (code smells, duplication)
- [ ] **ARCH-303:** Update architecture.md with "as-built" diagrams

**Security Engineer:**
- [ ] **SEC-601:** Full security review (dependency audit, code scan)
- [ ] **SEC-602:** Penetration testing (file upload, path traversal)
- [ ] **SEC-603:** Sign-off on MVP security posture

#### Exit Criteria
- ✅ All P0 and P1 bugs fixed
- ✅ End-to-end tests passing
- ✅ Performance targets met (see PRD §7.1)
- ✅ Security review complete, no critical vulnerabilities
- ✅ Code coverage >70% on core logic

---

### Sprint 1.12: Documentation & Beta Preparation (2 weeks)

**Sprint Goal:** Prepare MVP for beta testing with comprehensive documentation.

#### Tasks

**UI Specialist:**
- [ ] **DOC-101:** Write user guide (docs/user-guide.md)
  - Installation instructions
  - First conversion walkthrough
  - Troubleshooting FAQ
- [ ] **DOC-102:** Create video tutorial script (3-5 min)
- [ ] **DOC-103:** Screenshot all UI states for docs

**Senior Engineer:**
- [ ] **DOC-201:** Write developer guide (docs/developer-guide.md)
  - Build from source instructions
  - Architecture overview
  - Contribution guidelines
- [ ] **DOC-202:** API documentation (Rust docs, JSDoc)
- [ ] **DOC-203:** Generate docs with `cargo doc`

**System Architect:**
- [ ] **DOC-301:** Review and finalize architecture.md
- [ ] **DOC-302:** Create CONTRIBUTING.md
- [ ] **DOC-303:** Write README.md with badges (build status, license)

**Quality Engineer:**
- [ ] **DOC-401:** Document test procedures (manual testing checklist)
- [ ] **DOC-402:** Create bug report template
- [ ] **DOC-403:** Beta tester onboarding guide

**All Team:**
- [ ] **DOC-501:** Code comments and inline documentation
- [ ] **DOC-502:** Update CHANGELOG.md for MVP release
- [ ] **DOC-503:** Prepare release notes (GitHub release draft)

#### Exit Criteria
- ✅ User guide complete with screenshots
- ✅ Developer guide reviewed and approved
- ✅ README.md comprehensive (installation, usage, contributing)
- ✅ API documentation generated
- ✅ Beta tester onboarding materials ready

---

### Phase 1 Exit Gate

**Checkpoint Meeting:** All team members + stakeholders review Phase 1 deliverables.

**Go/No-Go Criteria:**
- ✅ All MVP features functional (see PRD §4.1)
- ✅ Beta testing plan ready (5+ testers identified)
- ✅ Security sign-off complete
- ✅ Documentation complete
- ✅ GitHub repository public with MIT license
- ✅ CI/CD pipeline green (all tests passing)

**Decision:** Proceed to Phase 2 / Iterate on Phase 1 / Pivot

---

## Phase 2: Enhanced UX

**Goal:** Polish user experience with advanced controls, presets, and improved workflow.

**Exit Criteria:**
- ✅ Advanced mode toggle functional with all controls
- ✅ Preset system implemented (save/load/import/export)
- ✅ Undo/redo system working for all mutable actions
- ✅ Enhanced 3D preview (lighting, measurements, cross-section)
- ✅ 90% user satisfaction rating from beta testers
- ✅ Performance maintained or improved

---

### Sprint 2.1: Advanced Depth Controls - Histogram & Curves (2 weeks)

**Sprint Goal:** Add power-user tools for precise depth manipulation.

#### Tasks

**Senior Engineer:**
- [ ] **BACK-1101:** Implement depth histogram calculation
- [ ] **BACK-1102:** Curves tool data structure (control points)
- [ ] **BACK-1103:** Apply curve transformation to depth map
- [ ] **BACK-1104:** Optimize for real-time preview

**UI Specialist:**
- [ ] **UI-1101:** Create HistogramPanel component (depth distribution graph)
- [ ] **UI-1102:** Implement CurvesTool component (Photoshop-style)
- [ ] **UI-1103:** Draggable control points on curve
- [ ] **UI-1104:** Preset curves (Linear, S-curve, Exponential)
- [ ] **UI-1105:** Advanced mode toggle (show/hide advanced controls)

**Junior Engineer #1:**
- [ ] **JR1-1101:** Render histogram with Canvas API
- [ ] **JR1-1102:** Implement curve drawing with cubic Bezier
- [ ] **JR1-1103:** Add reset curve button

**Quality Engineer:**
- [ ] **QA-1101:** Manual test: adjust curves, verify depth changes
- [ ] **QA-1102:** Test preset curves (apply, verify result)
- [ ] **QA-1103:** Performance test: curve application on large depth maps

#### Exit Criteria
- ✅ Histogram displays depth distribution
- ✅ Curves tool functional with draggable points
- ✅ Preset curves apply correctly
- ✅ Advanced mode toggle works
- ✅ Real-time preview updates (<200ms)

---

### Sprint 2.2: Masking & Regional Adjustments (2 weeks)

**Sprint Goal:** Enable selective depth adjustments via masking tools.

#### Tasks

**Senior Engineer:**
- [ ] **BACK-1201:** Implement mask data structure (2D boolean array)
- [ ] **BACK-1202:** Apply adjustments to masked regions only
- [ ] **BACK-1203:** Blend masked and unmasked regions (feathering)

**UI Specialist:**
- [ ] **UI-1201:** Create MaskingTools component (brush, eraser, select)
- [ ] **UI-1202:** Canvas-based mask painting
- [ ] **UI-1203:** Mask opacity overlay on depth preview
- [ ] **UI-1204:** Brush size/hardness controls

**Junior Engineer #1:**
- [ ] **JR1-1201:** Implement brush stroke smoothing (interpolation)
- [ ] **JR1-1202:** Add selection tools (rectangle, lasso)
- [ ] **JR1-1203:** Mask save/load functionality

**Quality Engineer:**
- [ ] **QA-1201:** Manual test: paint mask, adjust depth, verify isolation
- [ ] **QA-1202:** Test mask feathering (soft edges)
- [ ] **QA-1203:** Test undo/redo with masking

#### Exit Criteria
- ✅ Brush tool paints mask on depth preview
- ✅ Adjustments apply only to masked regions
- ✅ Mask feathering works (no hard edges)
- ✅ Undo/redo compatible with masking

---

### Sprint 2.3: Presets & Templates System (2 weeks)

**Sprint Goal:** Users can save and share processing configurations.

#### Tasks

**Senior Engineer:**
- [ ] **BACK-1301:** Define preset JSON schema (depth settings, mesh params)
- [ ] **BACK-1302:** Implement save_preset / load_preset commands
- [ ] **BACK-1303:** Built-in presets: Portrait, Landscape, High Detail, Low Relief
- [ ] **BACK-1304:** Import/export presets (file dialog)

**UI Specialist:**
- [ ] **UI-1301:** Create PresetManager component (list, rename, delete)
- [ ] **UI-1302:** Save/Load preset buttons in depth controls
- [ ] **UI-1303:** Preset dropdown in toolbar
- [ ] **UI-1304:** Import/export preset dialogs

**Junior Engineer #2:**
- [ ] **JR2-1301:** Write unit tests for preset serialization
- [ ] **JR2-1302:** Test import/export with invalid JSON
- [ ] **JR2-1303:** Versioned schema migration (future-proofing)

**Quality Engineer:**
- [ ] **QA-1301:** Manual test: save preset, load in new image
- [ ] **QA-1302:** Test built-in presets on various images
- [ ] **QA-1303:** Test preset import from external file

#### Exit Criteria
- ✅ User can save current settings as preset
- ✅ Presets load and apply correctly
- ✅ Built-in presets functional
- ✅ Import/export works with JSON files

---

### Sprint 2.4: Undo/Redo System (2 weeks)

**Sprint Goal:** Implement command pattern for reverting actions.

#### Tasks

**System Architect:**
- [ ] **ARCH-401:** Design undo/redo architecture (command pattern)
- [ ] **ARCH-402:** Define mutable operations to track
- [ ] **ARCH-403:** Memory budget for history stack (last 20 actions)

**Senior Engineer:**
- [ ] **BACK-1401:** Implement command trait (execute, undo)
- [ ] **BACK-1402:** History stack data structure
- [ ] **BACK-1403:** Wrap depth adjustments in commands
- [ ] **BACK-1404:** Tauri commands: undo, redo, clear_history

**UI Specialist:**
- [ ] **UI-1401:** Undo/Redo buttons in toolbar
- [ ] **UI-1402:** Keyboard shortcuts (Ctrl+Z, Ctrl+Y)
- [ ] **UI-1403:** Optional: Action history panel (list of commands)

**Junior Engineer #2:**
- [ ] **JR2-1401:** Write tests for command execution/undo
- [ ] **JR2-1402:** Test undo/redo with various actions
- [ ] **JR2-1403:** Test history stack limits (>20 actions)

**Quality Engineer:**
- [ ] **QA-1401:** Manual test: perform actions, undo, redo, verify state
- [ ] **QA-1402:** Test undo/redo with complex sequences
- [ ] **QA-1403:** Verify history cleared on new image load

#### Exit Criteria
- ✅ Undo/Redo functional for all depth adjustments
- ✅ Keyboard shortcuts work
- ✅ History stack limits enforced
- ✅ State consistency after undo/redo sequences

---

### Sprint 2.5: Enhanced 3D Preview (2 weeks)

**Sprint Goal:** Improve 3D visualization with lighting, measurements, cross-sections.

#### Tasks

**UI Specialist:**
- [ ] **UI-1501:** Implement PBR shading (diffuse, specular)
- [ ] **UI-1502:** Add lighting controls (ambient, directional intensity)
- [ ] **UI-1503:** Measure tool (click two points, show distance)
- [ ] **UI-1504:** Cross-section view (slice mesh at depth plane)
- [ ] **UI-1505:** Export preview as PNG (screenshot function)

**Junior Engineer #1:**
- [ ] **JR1-1501:** Implement camera projection toggle (orthographic/perspective)
- [ ] **JR1-1502:** Add grid size toggle (mm, cm)
- [ ] **JR1-1503:** Axis helpers (X, Y, Z labels)

**Senior Engineer:**
- [ ] **BACK-1501:** Provide mesh bounds data to frontend
- [ ] **BACK-1502:** Calculate cross-section slice (if backend-computed)

**Quality Engineer:**
- [ ] **QA-1501:** Manual test: all preview features (lighting, measure, slice)
- [ ] **QA-1502:** Verify measurement accuracy (compare to STL dimensions)
- [ ] **QA-1503:** Test screenshot export (resolution, format)

#### Exit Criteria
- ✅ PBR lighting improves visual quality
- ✅ Measure tool displays accurate distances
- ✅ Cross-section view functional
- ✅ Screenshot export works

---

### Sprint 2.6: UI Polish & Beta Feedback Iteration (2 weeks)

**Sprint Goal:** Refine UI based on beta tester feedback, fix UX issues.

#### Tasks

**UI Specialist:**
- [ ] **UI-1601:** Collect and triage beta feedback
- [ ] **UI-1602:** Improve control layouts (spacing, grouping)
- [ ] **UI-1603:** Add tooltips to all controls (help text)
- [ ] **UI-1604:** Keyboard shortcuts documentation (in-app help)

**Junior Engineer #1:**
- [ ] **JR1-1601:** Implement dark mode toggle (if not MVP)
- [ ] **JR1-1602:** Improve color palette (accessibility contrast)
- [ ] **JR1-1603:** Add loading skeletons for async operations

**All Engineers:**
- [ ] **BUG-2001:** Fix P1/P2 bugs from beta feedback
- [ ] **BUG-2002:** Performance improvements based on user reports

**Quality Engineer:**
- [ ] **QA-1601:** Re-test with new UI changes
- [ ] **QA-1602:** Usability testing (observe users, collect metrics)

#### Exit Criteria
- ✅ Beta feedback addressed (top 10 requests)
- ✅ Tooltips on all controls
- ✅ Dark mode functional (if implemented)
- ✅ 90% user satisfaction score

---

### Phase 2 Exit Gate

**Go/No-Go Criteria:**
- ✅ Advanced mode fully functional
- ✅ Presets system working with import/export
- ✅ Undo/redo system stable
- ✅ Enhanced preview features complete
- ✅ Beta tester feedback positive (>85% satisfaction)
- ✅ No regressions from Phase 1 features

**Decision:** Proceed to Phase 3 / Iterate on Phase 2

---

## Phase 3: Cross-Platform Parity

**Goal:** Achieve full feature parity on macOS and Linux, resolve platform-specific bugs.

**Exit Criteria:**
- ✅ macOS and Linux builds functional
- ✅ Installers available for all platforms
- ✅ GPU acceleration working (Metal, Vulkan)
- ✅ Platform-specific features implemented (menu bars, native dialogs)
- ✅ All tests passing on all platforms

---

### Sprint 3.1: macOS Build & Testing (2 weeks)

**Sprint Goal:** Create macOS build, test on Intel and Apple Silicon.

#### Tasks

**Senior Engineer:**
- [ ] **PLAT-101:** Configure Tauri for macOS target
- [ ] **PLAT-102:** Set up macOS development environment (VM or physical)
- [ ] **PLAT-103:** Build DMG installer with code signing
- [ ] **PLAT-104:** Test Python subprocess on macOS (zsh, bash)

**Senior Researcher:**
- [ ] **AI-501:** Test PyTorch with Metal GPU acceleration
- [ ] **AI-502:** Benchmark depth estimation on M1/M2 vs Intel
- [ ] **AI-503:** Fix macOS-specific Python issues (if any)

**UI Specialist:**
- [ ] **UI-1701:** Implement macOS native menu bar
- [ ] **UI-1702:** Keyboard shortcuts (Cmd instead of Ctrl)
- [ ] **UI-1703:** Retina display support (2x, 3x scaling)

**Quality Engineer:**
- [ ] **QA-1701:** Test on macOS Monterey, Ventura, Sonoma
- [ ] **QA-1702:** Test on Intel and Apple Silicon hardware
- [ ] **QA-1703:** Create macOS-specific test checklist

**Security Engineer:**
- [ ] **SEC-701:** macOS code signing and notarization
- [ ] **SEC-702:** Test installer security (Gatekeeper)

#### Exit Criteria
- ✅ macOS DMG installer builds successfully
- ✅ App runs on Intel and Apple Silicon
- ✅ Native macOS features functional (menu bar, shortcuts)
- ✅ GPU acceleration works (Metal)
- ✅ All tests passing on macOS

---

### Sprint 3.2: Linux Build & Packaging (2 weeks)

**Sprint Goal:** Create Linux builds (AppImage, .deb), test on Ubuntu and Fedora.

#### Tasks

**Senior Engineer:**
- [ ] **PLAT-201:** Configure Tauri for Linux target
- [ ] **PLAT-202:** Build AppImage (universal binary)
- [ ] **PLAT-203:** Build .deb package (Debian/Ubuntu)
- [ ] **PLAT-204:** Test on Ubuntu 22.04, Fedora 38

**Senior Researcher:**
- [ ] **AI-601:** Test PyTorch with CUDA on Linux
- [ ] **AI-602:** Fallback to CPU if GPU not available
- [ ] **AI-603:** Test Python virtual environment on various distros

**UI Specialist:**
- [ ] **UI-1801:** Test UI on Wayland and X11
- [ ] **UI-1802:** Desktop file integration (app launcher icon)
- [ ] **UI-1803:** Native file dialogs (GTK, Qt)

**Quality Engineer:**
- [ ] **QA-1801:** Test on multiple Linux distributions
- [ ] **QA-1802:** Test with different desktop environments (GNOME, KDE, XFCE)
- [ ] **QA-1803:** Create Linux-specific test checklist

#### Exit Criteria
- ✅ AppImage and .deb packages build successfully
- ✅ App runs on Ubuntu and Fedora
- ✅ Desktop integration works (launcher, file associations)
- ✅ GPU acceleration works (Vulkan/CUDA)
- ✅ All tests passing on Linux

---

### Sprint 3.3: Platform-Specific Optimizations (2 weeks)

**Sprint Goal:** Leverage OS-specific APIs for better performance and UX.

#### Tasks

**Senior Engineer:**
- [ ] **PLAT-301:** GPU auto-detection (DirectX/Metal/Vulkan)
- [ ] **PLAT-302:** OS-specific optimizations (file I/O, memory allocation)
- [ ] **PLAT-303:** Native notifications (Windows Toast, macOS Notification Center)

**UI Specialist:**
- [ ] **UI-1901:** Platform-specific drag-and-drop behaviors
- [ ] **UI-1902:** Native color pickers (if used)
- [ ] **UI-1903:** High DPI support across platforms

**Quality Engineer:**
- [ ] **QA-1901:** Cross-platform regression testing
- [ ] **QA-1902:** Performance benchmarks on all platforms
- [ ] **QA-1903:** Document platform differences (README)

#### Exit Criteria
- ✅ GPU auto-detection works on all platforms
- ✅ Native features functional (notifications, file dialogs)
- ✅ Performance parity across platforms (±10%)
- ✅ Platform differences documented

---

### Sprint 3.4: Cross-Platform Bug Fixes & Stabilization (2 weeks)

**Sprint Goal:** Fix platform-specific bugs, ensure stability.

#### Tasks

**All Engineers:**
- [ ] **BUG-3001:** Triage platform-specific bugs
- [ ] **BUG-3002:** Fix macOS-specific issues
- [ ] **BUG-3003:** Fix Linux-specific issues
- [ ] **BUG-3004:** Regression testing on Windows

**Quality Engineer:**
- [ ] **QA-2001:** Full platform test matrix (Windows, macOS, Linux)
- [ ] **QA-2002:** Performance regression tests
- [ ] **QA-2003:** Document known platform limitations

**Security Engineer:**
- [ ] **SEC-801:** Platform-specific security review
- [ ] **SEC-802:** Installer security (code signing, package verification)

#### Exit Criteria
- ✅ All P0/P1 platform bugs fixed
- ✅ All tests passing on all platforms
- ✅ Security review complete for all installers
- ✅ Platform limitations documented

---

### Phase 3 Exit Gate

**Go/No-Go Criteria:**
- ✅ macOS and Linux builds functional and tested
- ✅ Installers available for Windows, macOS, Linux
- ✅ Feature parity across all platforms
- ✅ GPU acceleration working on supported hardware
- ✅ Security sign-off for all platforms

**Decision:** Proceed to Phase 4 / Iterate on Phase 3

---

## Phase 4: Production Polish

**Goal:** Optimize performance, finalize documentation, create professional installers, prepare for v1.0 release.

**Exit Criteria:**
- ✅ All performance targets met (see PRD §7.1)
- ✅ Comprehensive documentation (user guide, dev guide, videos)
- ✅ Professional installers with auto-update
- ✅ Accessibility (WCAG AA compliance)
- ✅ No known critical bugs
- ✅ v1.0 release ready

---

### Sprint 4.1: Performance Optimization (2 weeks)

**Sprint Goal:** Meet all performance targets from PRD.

#### Tasks

**Senior Engineer:**
- [ ] **PERF-101:** Profile app startup (reduce to <2s)
- [ ] **PERF-102:** Optimize mesh generation (multi-threading with Rayon)
- [ ] **PERF-103:** Memory profiling (reduce runtime usage)
- [ ] **PERF-104:** Async I/O for file operations

**Senior Researcher:**
- [ ] **AI-701:** Optimize model inference (quantization, ONNX conversion)
- [ ] **AI-702:** Lazy loading of models (load on demand)
- [ ] **AI-703:** Clear GPU memory after inference

**UI Specialist:**
- [ ] **UI-2001:** Optimize 3D rendering (LOD, frustum culling)
- [ ] **UI-2002:** Debounce slider updates (reduce redraws)
- [ ] **UI-2003:** Lazy load UI components (code splitting)

**Quality Engineer:**
- [ ] **QA-2101:** Performance benchmarks on reference hardware
- [ ] **QA-2102:** Create performance regression tests (CI)
- [ ] **QA-2103:** Document performance tuning options

#### Exit Criteria
- ✅ All performance targets met (PRD §7.1)
- ✅ Startup time <2s
- ✅ Memory usage <2GB (typical workflow)
- ✅ Mesh generation <15s (4K image)

---

### Sprint 4.2: Error Handling & Logging (2 weeks)

**Sprint Goal:** Robust error handling and debugging support.

#### Tasks

**Senior Engineer:**
- [ ] **ERR-101:** Implement comprehensive error handling (anyhow, thiserror)
- [ ] **ERR-102:** User-friendly error messages (no stack traces)
- [ ] **ERR-103:** Logging system (env_logger, log rotation)
- [ ] **ERR-104:** Crash reporting (opt-in, Sentry integration?)

**Junior Engineer #2:**
- [ ] **JR2-2101:** Write error handling tests (invalid inputs)
- [ ] **JR2-2102:** Test edge cases (disk full, OOM)
- [ ] **JR2-2103:** Document error codes (if applicable)

**UI Specialist:**
- [ ] **UI-2101:** Error dialog component (clear, actionable messages)
- [ ] **UI-2102:** Recovery suggestions (e.g., "Free up disk space")
- [ ] **UI-2103:** Log viewer (in-app, for debugging)

**Quality Engineer:**
- [ ] **QA-2201:** Test error scenarios (corrupted files, missing models)
- [ ] **QA-2202:** Verify error messages are helpful (user testing)
- [ ] **QA-2203:** Test crash reporting (if implemented)

#### Exit Criteria
- ✅ All errors display user-friendly messages
- ✅ Logging system functional (debug, info, error levels)
- ✅ Crash reporting opt-in working (if implemented)
- ✅ Error recovery tested

---

### Sprint 4.3: Automated Testing & CI/CD (2 weeks)

**Sprint Goal:** Comprehensive test coverage and CI/CD pipeline.

#### Tasks

**Quality Engineer:**
- [ ] **TEST-101:** Increase test coverage to >80% (core logic)
- [ ] **TEST-102:** Integration tests (full workflow)
- [ ] **TEST-103:** UI tests (Playwright or Tauri's testing tools)
- [ ] **TEST-104:** Performance benchmarks in CI (track regressions)

**Senior Engineer:**
- [ ] **CI-101:** Configure GitHub Actions for all platforms
- [ ] **CI-102:** Automated builds (Windows, macOS, Linux)
- [ ] **CI-103:** Dependency caching (reduce CI time)
- [ ] **CI-104:** Release automation (tag → build → publish)

**Junior Engineer #1:**
- [ ] **JR1-2301:** Write frontend tests (component testing)
- [ ] **JR1-2302:** Test accessibility features (screen reader, keyboard nav)

**Security Engineer:**
- [ ] **SEC-901:** SAST (Static Application Security Testing) in CI
- [ ] **SEC-902:** Dependency vulnerability scans (daily)
- [ ] **SEC-903:** Code signing in CI (secure credential storage)

#### Exit Criteria
- ✅ Test coverage >80% (Rust core), >60% overall
- ✅ CI builds passing on all platforms
- ✅ Automated release pipeline functional
- ✅ Security scans integrated in CI

---

### Sprint 4.4: Documentation & Tutorials (2 weeks)

**Sprint Goal:** Complete user and developer documentation, create video tutorials.

#### Tasks

**UI Specialist:**
- [ ] **DOC-601:** Update user guide for Phase 2-4 features
- [ ] **DOC-602:** Record video tutorials (installation, first conversion, advanced)
- [ ] **DOC-603:** Create FAQ based on beta feedback
- [ ] **DOC-604:** In-app help (tooltips, getting started guide)

**Senior Engineer:**
- [ ] **DOC-701:** Update developer guide (build, architecture, contributing)
- [ ] **DOC-702:** API documentation (Rust docs, JSDoc)
- [ ] **DOC-703:** Write architecture decision records (ADRs)

**System Architect:**
- [ ] **DOC-801:** Review all documentation for accuracy
- [ ] **DOC-802:** Create onboarding guide for new contributors
- [ ] **DOC-803:** Update README with badges (CI status, coverage, license)

**Quality Engineer:**
- [ ] **DOC-901:** Document testing procedures (manual + automated)
- [ ] **DOC-902:** Create bug report template (GitHub Issues)
- [ ] **DOC-903:** Write release checklist

#### Exit Criteria
- ✅ User guide comprehensive (all features documented)
- ✅ Video tutorials published (YouTube, docs site)
- ✅ Developer guide complete with examples
- ✅ In-app help functional

---

### Sprint 4.5: Installers & Packaging (2 weeks)

**Sprint Goal:** Professional installers with auto-update, code signing.

#### Tasks

**Senior Engineer:**
- [ ] **PKG-101:** Windows installer (MSI and EXE with NSIS)
- [ ] **PKG-102:** macOS DMG with custom background image
- [ ] **PKG-103:** Linux AppImage, .deb packages
- [ ] **PKG-104:** Auto-update mechanism (Tauri updater)

**Security Engineer:**
- [ ] **SEC-1001:** Code sign Windows installer (Authenticode)
- [ ] **SEC-1002:** Code sign macOS app (Developer ID, notarization)
- [ ] **SEC-1003:** GPG sign Linux checksums
- [ ] **SEC-1004:** Verify installer integrity (checksum validation)

**Quality Engineer:**
- [ ] **QA-2401:** Test installers on clean VMs (Windows, macOS, Linux)
- [ ] **QA-2402:** Test auto-update flow (mock update server)
- [ ] **QA-2403:** Verify uninstaller removes all data

**UI Specialist:**
- [ ] **UI-2401:** Design installer splash screens
- [ ] **UI-2402:** Installer options (model download, shortcuts)

#### Exit Criteria
- ✅ Installers functional on all platforms
- ✅ Code signing working (verified by OS)
- ✅ Auto-update mechanism tested
- ✅ Installers size optimized (<100MB core app)

---

### Sprint 4.6: Accessibility & i18n Framework (2 weeks)

**Sprint Goal:** WCAG AA compliance, i18n infrastructure (English only for v1.0).

#### Tasks

**UI Specialist:**
- [ ] **A11Y-101:** Keyboard navigation audit (all features accessible)
- [ ] **A11Y-102:** Screen reader testing (NVDA, VoiceOver)
- [ ] **A11Y-103:** Color contrast fixes (WCAG AA 4.5:1)
- [ ] **A11Y-104:** High contrast mode support

**Junior Engineer #1:**
- [ ] **JR1-2501:** Add ARIA labels to all interactive elements
- [ ] **JR1-2502:** Focus indicators (visible outlines)
- [ ] **JR1-2503:** Skip navigation links (if needed)

**System Architect:**
- [ ] **I18N-101:** Set up i18n framework (i18next or similar)
- [ ] **I18N-102:** Externalize UI strings (en.json)
- [ ] **I18N-103:** Document translation workflow (for future)

**Quality Engineer:**
- [ ] **QA-2501:** Accessibility audit (automated + manual)
- [ ] **QA-2502:** Test with screen readers
- [ ] **QA-2503:** Test keyboard-only navigation

#### Exit Criteria
- ✅ WCAG 2.1 AA compliance achieved
- ✅ Keyboard navigation functional for all features
- ✅ Screen reader compatible
- ✅ i18n framework in place (English only)

---

### Sprint 4.7: Final Bug Bash & Release Prep (2 weeks)

**Sprint Goal:** Fix all remaining bugs, prepare for v1.0 release.

#### Tasks

**All Engineers:**
- [ ] **BUG-4001:** Triage all open issues (prioritize for v1.0)
- [ ] **BUG-4002:** Fix all P0 (critical) bugs
- [ ] **BUG-4003:** Fix P1 (high) bugs (or defer to v1.1)
- [ ] **BUG-4004:** Code review backlog (all PRs merged or closed)

**Quality Engineer:**
- [ ] **QA-2601:** Full regression testing (all features, all platforms)
- [ ] **QA-2602:** Smoke testing on fresh installs
- [ ] **QA-2603:** Performance regression checks
- [ ] **QA-2604:** Create release candidate (RC1)

**System Architect:**
- [ ] **REL-101:** Finalize CHANGELOG.md for v1.0
- [ ] **REL-102:** Update version numbers (Cargo.toml, package.json, tauri.conf.json)
- [ ] **REL-103:** Create GitHub release draft (notes, screenshots)

**Security Engineer:**
- [ ] **SEC-1101:** Final security audit (penetration testing)
- [ ] **SEC-1102:** Dependency audit (no vulnerabilities)
- [ ] **SEC-1103:** Sign-off on v1.0 security posture

#### Exit Criteria
- ✅ All P0 and P1 bugs fixed
- ✅ Release candidate tested (no critical bugs)
- ✅ CHANGELOG.md complete
- ✅ Security audit passed
- ✅ Ready for v1.0 release

---

### Phase 4 Exit Gate: v1.0 Release

**Release Checklist:**
- ✅ All performance targets met
- ✅ Comprehensive documentation complete
- ✅ Installers tested and code-signed
- ✅ Accessibility compliance (WCAG AA)
- ✅ No critical bugs
- ✅ Security audit passed
- ✅ GitHub release published
- ✅ Social media announcement prepared
- ✅ Co-Fi (donation) page set up

**v1.0 Launch:**
1. Tag release in GitHub (`v1.0.0`)
2. Publish installers (GitHub Releases, website)
3. Announce on:
   - GitHub Discussions
   - Reddit (r/lasercutting, r/maker, r/3Dprinting)
   - Hacker News (Show HN)
   - Maker forums
4. Submit to package managers (Homebrew, Chocolatey, AUR)
5. Monitor community feedback, triage bug reports

---

## Beyond Phase 4: Far-Future Roadmap

Items below are **not** scheduled for Phases 1–4. They are candidates for a future major or maintenance release (e.g. v1.1+), to be prioritized when v1.0 is stable and maintainer capacity allows.

| Item | Description |
|------|-------------|
| **Svelte 5 migration** | Upgrade frontend from Svelte 4 to Svelte 5: runes, new component API, `@sveltejs/vite-plugin-svelte` ^4.x, `@testing-library/svelte` v5. See ADR-001, RESEARCH/frontend.md, prd.md §11.3. Evaluate when Svelte 4 approaches EOL or when runes/DX justify the migration. |

---

## Testing Strategy

*Software Quality Lead review (2026-02-06): Current state and requirements below.*

### Current state (as of 2026-02-07, post Sprint 1.6)

| Layer | Implemented | Gaps |
|-------|-------------|------|
| **Rust** | **59 passing + 5 ignored** (64 total). `cargo test`, `cargo clippy`, and `cargo tarpaulin` in CI. Coverage: **63.6%** (199/313 lines). | lib.rs at only 6.0% (Tauri handlers need extraction — Sprint 1.6A BACK-507/508). Coverage thresholds advisory only (`continue-on-error: true`). 5 ignored tests require Python. |
| **Frontend** | **34 Vitest tests** passing in 1.4s. `@testing-library/svelte`, 5 test files covering components + utilities. `npm test` in CI. | Estimated ~40–50% coverage. No formal coverage measurement yet. |
| **Python** | 19 pytest tests (stub mode); `pytest --cov` in CI. | pytest-cov advisory only. Coverage ~70–80% estimated. |
| **Integration** | Rust tests for load_image, generate_depth_map, get_mesh_data (path validation, normalization, mesh generation); roundtrip test exists but ignored without Python. | Integration tests requiring Python not automated in CI. |
| **E2E** | Manual test plans and reports per sprint. | No Playwright (or similar) E2E automation. |

**Quantitative summary (Consultant Report 7 Feb 2026):**

| Stack | Source Lines | Tests | Measured Coverage | Target | Status |
|-------|-------------|-------|-------------------|--------|--------|
| Rust backend | ~2,014 | 64 (59+5 ignored) | 63.6% (tarpaulin) | >70% | Approaching target |
| Python | ~224 | 19 | ~70–80% (est.) | >70% | Meets target |
| Frontend | ~1,078 | 34 | ~40–50% (est.) | >60% | Improving |
| **Total** | **~3,316** | **117** | — | — | Up from 65 tests at Sprint 1.5 |

**Local test commands (verified):**
- Rust: `cargo test --manifest-path src-tauri/Cargo.toml` — 59 passed, 5 ignored.
- Frontend: `npm test` — 34 tests via Vitest; `npm run test:watch` for watch mode.
- Lint: `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` — in CI.
- Python: `SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v` (Windows: set env in PowerShell). — 19 tests, in CI.
- Audits: `cargo audit`, `npm audit --audit-level=high` — both in CI.
- Coverage (Rust): `cargo tarpaulin --manifest-path src-tauri/Cargo.toml --out xml` — in CI (advisory).
- Coverage (Python): `pytest --cov=depth_estimator --cov-report=xml` — in CI (advisory).

### Testing requirements (Quality Engineer backlog)

These items are required to meet Phase 1 exit criteria and the Test Pyramid below. Assign to Quality Engineer and/or sprint tasking as needed.

1. ~~**CI — Lint:** Add `cargo clippy` to CI~~ **Done** — clippy runs in backend job.
2. ~~**Rust — Coverage:** Introduce `cargo tarpaulin` in CI~~ **Done** — baseline 63.6%. Enforcement at `--fail-under 65` in Sprint 1.6A (QA-506/507). Target >70% for Phase 1, >80% for Phase 4.
3. ~~**Frontend — Test suite:** Add Vitest, `npm test` script, smoke/unit tests~~ **Done** — 34 tests across 5 files, `@testing-library/svelte`. Target >60% coverage by Phase 4.
4. ~~**Python — Test suite:** Add `pytest` in `python/`, run in CI~~ **Done** — 19 tests, stub mode, in CI. `pytest-cov` baseline in CI.
5. **Integration — Python in CI:** Option A: Run Python-dependent Rust tests in CI when Python 3.10+ and deps are available. Option B: Keep them `#[ignore]` and document running `cargo test -- --ignored` locally or in a separate workflow.
6. **E2E:** Defer to Sprint 1.11 (QA-1001); document Playwright (or Tauri testing) as requirement for Phase 1 exit.
7. **lib.rs testability:** Extract business logic from Tauri command handlers into standalone functions (Sprint 1.6A BACK-507/508). Current lib.rs coverage: 6.0% (10/167 lines). Target: >50% after extraction.
8. **Coverage enforcement:** Enable `--fail-under` thresholds and remove `continue-on-error` (Sprint 1.6A QA-506/507). Increment by 5% per sprint until 70% reached.

### Test Pyramid

```
       /\        E2E Tests (10%)
      /  \       - Full workflow: load → generate → export
     /----\      - Multi-platform smoke tests
    / Integration \ (20%)
   /  Tests       \  - Rust ↔ Python IPC
  /----------------\  - Tauri command integration
 /   Unit Tests    \ (70%)
/  (Core Logic)     \ - Mesh generation algorithms
--------------------  - Depth map processing
                      - File I/O, settings
```

### Test Coverage Goals

| Component | Target Coverage | Tool | Current | Status |
|-----------|----------------|------|---------|--------|
| Rust Backend | >70% (Phase 1); >80% (Phase 4) | `cargo tarpaulin` | **63.6%** (measured) | Approaching — lib.rs gap (6%) pulling down average. Sprint 1.6A targets >65%. |
| Python AI | >70% | `pytest --cov` | **~70–80%** (est.) | Meets target |
| Frontend | >60% (Phase 4) | Vitest | **~40–50%** (est.) | 34 tests; utilities well-covered, components partial |
| Integration | 100% critical paths | Rust tests + optional Python in CI | Partial | Rust-only in CI; Python-dependent tests `#[ignore]` |
| E2E | Happy path + 5 edge cases | Playwright (Sprint 1.11) | None | Manual only |

### Testing Cadence

**Per Sprint:**
- Unit tests written alongside feature code (TDD encouraged)
- Integration tests at sprint end
- Manual testing (QA) throughout sprint
- Security review on Sprint Review day

**Per Phase:**
- Full regression testing
- Performance benchmarking
- Cross-platform testing (Phase 3+)
- Security audit

**Pre-Release:**
- Beta testing (external users)
- Accessibility audit
- Penetration testing

### Manual Testing Checklist

**Core Workflow (Every Sprint):**
- [ ] Load various image formats (PNG, JPG, large, small, corrupt)
- [ ] Generate depth map (GPU and CPU)
- [ ] Adjust depth controls (all sliders, toggles)
- [ ] Generate mesh (verify vertex count, dimensions)
- [ ] Preview mesh (rotate, zoom, rendering modes)
- [ ] Export STL and OBJ (verify files open in external tools)
- [ ] Settings persistence (restart app, verify saved)

**Platform-Specific (Phase 3):**
- [ ] Test on Windows 10, 11
- [ ] Test on macOS Monterey, Ventura, Sonoma (Intel + Apple Silicon)
- [ ] Test on Ubuntu 22.04, Fedora 38
- [ ] Installer installation/uninstallation
- [ ] Auto-update flow

**Accessibility (Phase 4):**
- [ ] Keyboard navigation (Tab, Enter, Escape)
- [ ] Screen reader (NVDA on Windows, VoiceOver on macOS)
- [ ] High contrast mode
- [ ] Color blindness simulation (check for color-independent info)

---

## CI/CD Pipeline

### GitHub Actions Workflows

#### 1. **CI Workflow** (`ci.yml`) — current state (post Sprint 1.6)

**Trigger:** Every push, pull request to `main`, `develop`.

**Five-signal quality gate (as of 2026-02-07):**
```
Frontend:  npm ci → npm run build → npm test → npm audit
Backend:   cargo build → cargo test → cargo clippy -D warnings → cargo tarpaulin → cargo audit
Python:    python 3.10 → pip install → pytest --cov (stub mode)
Coverage:  tarpaulin XML + pytest-cov XML (advisory — continue-on-error: true)
```

**Status of previous requirements:**
- ~~**Lint:** Run `cargo clippy` in backend job~~ **Done.**
- ~~**Frontend test:** Add `npm test`~~ **Done** — 34 Vitest tests in CI.
- ~~**Python test:** Add pytest in CI~~ **Done** — 19 pytest tests, stub mode, in CI.
- ~~**Coverage:** Add coverage step~~ **Done** — tarpaulin (Rust 63.6%) and pytest-cov (Python) in CI.
- **Coverage enforcement:** Not yet — both steps use `continue-on-error: true`. Sprint 1.6A QA-506/507 to enforce `--fail-under 65`.
- **Codecov upload:** Not yet — reports generated locally only.
- **Matrix (later):** Expand to OS matrix (Windows, macOS, Linux) when Phase 3 cross-platform is active.

---

#### 2. **Security Workflow** (`security.yml`)
**Trigger:** Daily cron, pull request (for dependencies)

**Jobs:**
- **Dependency Audit:**
  - Rust: `cargo audit`
  - NPM: `npm audit`
  - Python: `pip-audit`
- **SAST (Static Analysis):**
  - `cargo clippy -- -D warnings`
  - Optional: Semgrep, CodeQL

---

#### 3. **Release Workflow** (`release.yml`)
**Trigger:** Git tag (`v*.*.*`)

**Jobs:**
- **Build Installers:**
  - Windows: MSI, EXE (code-signed)
  - macOS: DMG (code-signed, notarized)
  - Linux: AppImage, .deb
- **Upload Assets:**
  - Attach to GitHub Release
  - Generate checksums (SHA256)
- **Publish:**
  - Optional: Publish to Homebrew, Chocolatey, AUR

---

## Release Checklist

### Pre-Release
- [ ] All tests passing on all platforms (see [Testing Strategy](#testing-strategy): `cargo test`, `npm run build`; once added: `npm test`, `pytest`, `cargo clippy`)
- [ ] Performance benchmarks meet targets
- [ ] Security audit complete (no critical vulnerabilities)
- [ ] Documentation reviewed and up-to-date
- [ ] CHANGELOG.md updated
- [ ] Version numbers bumped (Cargo.toml, package.json, tauri.conf.json)

### Release
- [ ] Create Git tag (`v1.0.0`)
- [ ] Trigger release workflow (CI builds installers)
- [ ] Verify installers (download, install, smoke test)
- [ ] Publish GitHub Release (notes, assets)
- [ ] Update website/docs with new version

### Post-Release
- [ ] Announce on social media (Twitter, Reddit, HN)
- [ ] Submit to package managers (Homebrew, Chocolatey)
- [ ] Monitor issue tracker for critical bugs
- [ ] Prepare hotfix process (if needed)
- [ ] Plan next release (v1.1) with community feedback

---

## Dependencies & Risks

### Critical Dependencies

| Dependency | Version | License | Risk Mitigation |
|------------|---------|---------|-----------------|
| Tauri | 1.x | MIT | Active maintenance, large community |
| Rust | 1.70+ | MIT/Apache | Stable language, MSRV policy |
| PyTorch | 2.x | BSD-3 | Alternative: ONNX Runtime |
| Depth-Anything-V2 | Latest | Apache-2.0 | Model weights CC-BY-NC-4.0 (non-commercial) |
| Three.js | Latest | MIT | Stable, widely used |
| Svelte / React | Latest | MIT | Mature frameworks |

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Python-Rust IPC bottleneck | Medium | High | Benchmark early (Sprint 1.3), consider ONNX in Rust |
| AI model license issues (non-commercial) | Low | High | Clarify commercial use, offer alternative models |
| Cross-platform GPU bugs | High | Medium | Extensive testing, CPU fallback |
| Memory overflow (8K images) | Medium | High | Stream processing, downsampling |
| Tauri security vulnerability | Low | High | Subscribe to advisories, rapid patching |

### Schedule Risks

| Risk | Mitigation |
|------|------------|
| Feature creep | Strict scope control, defer to future versions |
| Junior engineer ramp-up time | Pair programming, clear task breakdown |
| Cross-platform testing delays | Dedicated testing sprints (Phase 3) |
| Dependency updates break build | Lock file committed, CI alerts on failures |

### Community Risks

| Risk | Mitigation |
|------|------------|
| Low adoption | Marketing plan, demo videos, maker community outreach |
| Toxic community | Code of conduct, active moderation |
| Maintainer burnout | Distribute responsibilities, onboard co-maintainers |
| Competing fork | Encourage collaboration, merge useful PRs |

---

## Appendix: Sprint Template

### Sprint N.X: [Sprint Name] (2 weeks)

**Sprint Goal:** [One-sentence objective]

**Team Capacity:**
- Senior Engineers: X days
- Junior Engineers: Y days
- UI Specialist: Z days
- QA: W days

#### Tasks

**[Role]:**
- [ ] **TASK-ID:** [Task description]
  - **Estimate:** X hours/days
  - **Dependencies:** [Other tasks]
  - **Acceptance Criteria:** [How to verify complete]

#### Exit Criteria
- ✅ [Functional criteria]
- ✅ [Testing criteria]
- ✅ [Documentation criteria]

**Sprint Review:**
- Demo to team/stakeholders
- Retrospective: What went well, what to improve

**Sprint Retrospective Questions:**
1. What helped us succeed this sprint?
2. What blocked progress?
3. What should we change for next sprint?

---

**END OF TODO DOCUMENT**

*This TODO is a living document and will be updated as development progresses. Completed tasks should be marked with ✅ and dated. For questions or updates, create a GitHub Discussion in the SimplePicture3D repository.*
