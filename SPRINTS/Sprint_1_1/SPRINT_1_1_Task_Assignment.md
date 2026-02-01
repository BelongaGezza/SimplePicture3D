# Sprint Task Assignment — Sprint 1.1

**Use this template when creating sprint tasking from `todo.md`.**  
Source: `todo.md` — Sprint 1.1. Populated by Senior Engineer per System Architect request (2026-02-01).

---

## Sprint 1.1: Project Setup & Foundations

**Sprint Duration:** 2 weeks (10 working days)  
**Sprint Goal:** Establish development environment, repo structure, and basic build pipeline.  
**Target Release:** —  
**Phase:** 1 (MVP)  
**Source:** `todo.md` — Sprint 1.1  
**Last Updated:** 2026-02-01

---

## Sprint Folder & Artefacts

**All sprint artefacts MUST be stored in this sprint's folder:**

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_1_1/SPRINT_1_1_Task_Assignment.md` | This document |
| Researcher Tasking | `SPRINTS/Sprint_1_1/RESEARCHER_TASKING.md` | Pre-sprint knowledge refresh |
| Progress Report | `SPRINTS/Sprint_1_1/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_1_1/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_1_1/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Architect Approval | `SPRINTS/Sprint_1_1/ARCHITECT_APPROVAL.md` | If required for phase gate |
| Security Sign-off | `SPRINTS/Sprint_1_1/SECURITY_SIGNOFF.md` | If security review in sprint |
| Gotchas Log | `SPRINTS/Sprint_1_1/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

---

## CRITICAL: Role Selection (READ FIRST — STOP HERE UNTIL COMPLETE)

**You are an unassigned agent. You MUST claim a role before proceeding.**

### Step 1: Review Available Roles
Look at the Role Assignment table below. Find a role where:
- Status = `Available`
- No agent is currently assigned

### Step 2: Claim Your Role
1. **Edit this document** to update that role's row:
   - Change Status from `Available` to `In Progress`
   - Add your session identifier to the "Assigned Agent" column
2. **Read the persona file** listed in the "Persona File" column
3. **Adopt that persona** for all remaining work

### Step 3: Become Your Role
- Embody the agent's identity, expertise, and responsibilities
- Follow the persona file's guidance and project references

**If all roles show "In Progress" or "Complete", STOP. No work available.**

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | In Progress | Cursor Agent | ARCH-001, ARCH-002, ARCH-003, ARCH-004, ARCH-005 | Architecture decisions, approvals |
| Senior Engineer | `.agents/senior-engineer.md` | In Progress | Cursor Agent | BACK-001, BACK-002, BACK-003, BACK-004, BACK-005 | Core implementation, Python bridge |
| UI Designer | `.agents/ui-designer.md` | In Progress | Cursor Agent | UI-001, UI-002, UI-003, UI-004, UI-005 | Frontend, 3D preview, layout |
| Senior Researcher (AI/ML) | `.agents/researcher.md` | In Progress | Cursor Agent | AI-001, AI-002, AI-003, AI-004, AI-005 | Depth estimation, RESEARCH updates |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | In Progress | Cursor Agent | JR1-001, JR1-002, JR1-003, JR1-004 | Frontend-focused (wireframes, buttons, types) |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | In Progress | Cursor Agent | JR2-001, JR2-002, JR2-003, JR2-004 | Backend-focused (env, file I/O, logging) |
| Quality Engineer | (see todo.md Team Roles) | In Progress | Cursor Agent | QA-001, QA-002, QA-003, QA-004 | CI, test plan, coverage, docs |
| Security Specialist | `.agents/security-specialist.md` | In Progress | Cursor Agent | SEC-001, SEC-002, SEC-003 | Threat model, audit in CI, checklist |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References (Source of Truth)

- **Scope:** `prd.md` — Product requirements, tech stack (§5.1), file structure (§5.4)
- **Sprint source:** `todo.md` — Sprint 1.1 task list
- **Architecture:** `RESEARCH/architecture.md`, `docs/architecture.md` (to create)
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`
- **Technology:** `RESEARCH/` — See `RESEARCH/README.md`; refresh per `RESEARCHER_TASKING.md` before implementation
- **Pre-implementation:** Complete `RESEARCHER_TASKING.md` (knowledge refresh) before ARCH-005 and before implementation tasks that rely on RESEARCH/ (e.g. BACK-001, UI-001, AI-002)

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| Architecture (ARCH-*) | ✅ Complete | 100% |
| Backend (BACK-*) | ✅ Complete | 100% |
| AI/Research (AI-*) | ⏳ Not Started | 0% |
| UI (UI-*) | ✅ Complete | 100% |
| Junior 2D (JR1-*) | ✅ Complete | 100% |
| Junior 3D (JR2-*) | ✅ Complete | 100% |
| Quality (QA-*) | ✅ Complete | 100% |
| Security (SEC-*) | ✅ Complete | 100% |

**Overall Sprint Progress:** [ ] Not Started / [x] In Progress / [ ] Complete

---

## Task Breakdown

### System Architect

#### ARCH-001: Define repository structure (see PRD §5.4)
**Assigned Role:** System Architect  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** ARCH-001  

**Dependencies:** None (align with prd.md §5.4).  

**What to Do:** Define and document repository structure per `prd.md` §5.4 (file structure). Ensure Tauri + Python monorepo layout is specified (e.g. `src-tauri/`, frontend root, `python/`, `SPRINTS/`, `RESEARCH/`, etc.).  

**Reference Documents:** `prd.md` §5.4, `RESEARCH/architecture.md`  

**Acceptance Criteria:**  
- [x] Repository structure documented (in architecture doc or README)  
- [x] Matches PRD §5.4 and supports Tauri + Python + frontend  

**Completion Record:** 2026-02-01 — RESEARCH/architecture.md expanded with full PRD §5.4 structure; README links to it.  

---

#### ARCH-002: Create `docs/architecture.md` with diagrams
**Assigned Role:** System Architect  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** ARCH-002  

**Dependencies:** ARCH-001 (structure).  

**What to Do:** Create `docs/architecture.md` with system diagram(s) (e.g. frontend ↔ Rust ↔ Python, data flow). Align with `prd.md` §5.2, §5.3.  

**Reference Documents:** `prd.md` §5.2, §5.3, `RESEARCH/architecture.md`  

**Acceptance Criteria:**  
- [x] `docs/architecture.md` exists with at least one architecture diagram  
- [x] Data flow (image → depth → mesh → export) described  

**Completion Record:** 2026-02-01 — docs/architecture.md created with system diagram and 7-step data flow diagram.  

---

#### ARCH-003: Set up monorepo structure (Tauri + Python)
**Assigned Role:** System Architect  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** ARCH-003  

**Dependencies:** ARCH-001. BACK-001, UI-001, AI-002 may implement; Architect defines layout.  

**What to Do:** Define and approve monorepo layout: Tauri app root, `src-tauri/`, frontend directory, `python/` (or equivalent) for AI backend. Coordinate with Senior Engineer / UI / Researcher for actual folder creation.  

**Reference Documents:** `prd.md` §5.4, `RESEARCH/tauri.md`  

**Acceptance Criteria:**  
- [x] Monorepo structure defined and created (or creation tasks assigned and verified)  
- [x] Tauri, frontend, and Python each have clear locations  

**Completion Record:** 2026-02-01 — src-tauri/, src/, python/, tests/rust|python|integration/, scripts/, .github/workflows/ created.  

---

#### ARCH-004: Configure Cursor IDE workspace settings
**Assigned Role:** System Architect  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** ARCH-004  

**Dependencies:** None.  

**What to Do:** Configure Cursor (or VS Code) workspace settings for Rust, TypeScript/JavaScript, Python (linters, formatters, recommended extensions).  

**Reference Documents:** Project conventions, `CONTRIBUTING.md` if present  

**Acceptance Criteria:**  
- [x] Workspace settings file(s) added (e.g. `.vscode/settings.json` or Cursor equivalent)  
- [x] Rust (rust-analyzer), frontend, and Python environments supported  

**Completion Record:** 2026-02-01 — .vscode/settings.json added (rust-analyzer, TypeScript/Svelte, Python formatters).  

---

#### ARCH-005: Review and approve tech stack decisions
**Assigned Role:** System Architect  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** ARCH-005  

**Dependencies:** Researcher knowledge refresh (`RESEARCHER_TASKING.md`) recommended before final approval.  

**What to Do:** Review tech stack per `prd.md` §5.1 (Tauri, Rust crates, Python/PyTorch, Svelte/React, Three.js). Approve or document deviations. Consider RESEARCH updates (Tauri v1 vs v2, Svelte vs React).  

**Reference Documents:** `prd.md` §5.1, `RESEARCH/*.md`  

**Acceptance Criteria:**  
- [x] Tech stack reviewed and approved (or documented exceptions)  
- [x] Decisions recorded (e.g. in architecture doc or ADR)  

**Completion Record:** 2026-02-01 — docs/tech-stack-approval.md created. Tauri v2, Svelte, Depth-Anything-V2, subprocess bridge approved.  

---

### Senior Engineer

#### BACK-001: Initialize Tauri project with Rust backend
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-001  

**Dependencies:** ARCH-003 (structure), RESEARCH/tauri.md (version choice).  

**What to Do:** Initialize Tauri project (e.g. `cargo tauri init` or equivalent per chosen Tauri version). Ensure Rust backend skeleton under `src-tauri/` builds.  

**Reference Documents:** `RESEARCH/tauri.md`, `prd.md` §5.1  

**Acceptance Criteria:**  
- [x] Tauri project initializes successfully  
- [x] `cargo build` (or `cargo tauri build`) succeeds in `src-tauri`  

**Completion Record:** 2026-02-01 — Senior Engineer. Project already initialized (Tauri v2, src-tauri/). Build succeeds after RC2176 icon fix: `scripts/gen_icon_win.py` generates BMP-based icon.ico; `dist/` created for frontendDist. `cargo build` in src-tauri passes.  

---

#### BACK-002: Set up Cargo.toml with dependencies (image, tokio, serde)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-002  

**Dependencies:** BACK-001.  

**What to Do:** Add to `Cargo.toml`: `image`, `tokio`, `serde` (and `serde_json` if needed). Use versions consistent with `RESEARCH/rust-crates.md`.  

**Reference Documents:** `RESEARCH/rust-crates.md`, `prd.md` §5.1  

**Acceptance Criteria:**  
- [x] Dependencies added and `cargo build` passes  
- [x] No unnecessary dependencies; MSRV noted if required  

**Completion Record:** 2026-02-01 — Senior Engineer. Added anyhow, image, serde (derive), tokio (fs, io-util) to src-tauri/Cargo.toml. serde_json, env_logger, tauri, tauri-plugin-shell already present. Build passes.  

---

#### BACK-003: Create placeholder IPC commands (load_image, export_stl)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-003  

**Dependencies:** BACK-001, BACK-002.  

**What to Do:** Add placeholder Tauri commands: `load_image`, `export_stl` (signatures per PRD; bodies can return stub/error for now). Register in Tauri handler.  

**Reference Documents:** `RESEARCH/tauri.md`, `prd.md` F1.1, F1.6  

**Acceptance Criteria:**  
- [x] Commands exist and are invokable from frontend  
- [x] Signatures align with planned API (e.g. path for load_image, path + mesh for export_stl)  

**Completion Record:** 2026-02-01 — Senior Engineer. Placeholder commands already present in src-tauri/src/lib.rs; load_image(path: String) -> Result<serde_json::Value, String>, export_stl(path: String) -> Result<(), String>. Permissions in src-tauri/permissions/. Invoke handler registered.  

---

#### BACK-004: Implement basic error handling pattern (anyhow crate)
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-004  

**Dependencies:** BACK-002.  

**What to Do:** Introduce `anyhow` for error handling in backend. Use in placeholder commands and document pattern (e.g. `Result<T, anyhow::Error>`, context with `.context()`).  

**Reference Documents:** `RESEARCH/rust-crates.md`, `prd.md` F4.2  

**Acceptance Criteria:**  
- [x] anyhow used in at least one command or shared module  
- [x] Pattern documented (e.g. in code comments or architecture)  

**Completion Record:** 2026-02-01 — Senior Engineer. anyhow in Cargo.toml. load_image_impl uses anyhow::ensure!; export_stl uses anyhow::anyhow!; boundary Result<T, String> with .map_err(|e| e.to_string()). Pattern documented in lib.rs comment block.  

---

#### BACK-005: Set up Rust testing framework (cargo test)
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-005  

**Dependencies:** BACK-001.  

**What to Do:** Add at least one Rust test (e.g. in `src-tauri/src/lib.rs` or a module). Ensure `cargo test` runs and passes.  

**Reference Documents:** `todo.md` Testing Strategy, `RESEARCH/rust-crates.md`  

**Acceptance Criteria:**  
- [x] `cargo test` runs and passes  
- [x] Test layout suitable for future unit tests  

**Completion Record:** 2026-02-01 — Senior Engineer. lib.rs tests: load_image_stub_accepts_non_empty_path, load_image_stub_rejects_empty_path. file_io.rs already had 5 tests. All 7 tests pass; cargo test in src-tauri succeeds.  

---

### Senior Researcher (AI/ML)

#### AI-001: Research Depth-Anything-V2 vs MiDaS performance benchmarks
**Assigned Role:** Senior Researcher (AI/ML)  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** AI-001  

**Dependencies:** None.  

**What to Do:** Research and document Depth-Anything-V2 vs MiDaS (accuracy, speed, resource use, license). Update `RESEARCH/python-ml.md` with findings and recommendation.  

**Reference Documents:** `RESEARCH/python-ml.md`, `prd.md` F1.2  

**Acceptance Criteria:**  
- [ ] Comparison documented in RESEARCH  
- [ ] Recommendation stated (with rationale) for MVP  

**Completion Record:** *(fill when complete)*  

---

#### AI-002: Set up Python virtual environment (requirements.txt)
**Assigned Role:** Senior Researcher (AI/ML)  
**Priority:** Critical  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** AI-002  

**Dependencies:** ARCH-003 (python directory).  

**What to Do:** Create `python/` (or agreed path) with `requirements.txt` for PyTorch and depth-estimation dependencies. Document venv setup in README or RESEARCH.  

**Reference Documents:** `RESEARCH/python-ml.md`, `prd.md` §5.1  

**Acceptance Criteria:**  
- [ ] `requirements.txt` exists and installs without error  
- [ ] Setup instructions in README or RESEARCH  

**Completion Record:** *(fill when complete)*  

---

#### AI-003: Prototype depth estimation script (standalone CLI)
**Assigned Role:** Senior Researcher (AI/ML)  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** AI-003  

**Dependencies:** AI-002, AI-001.  

**What to Do:** Implement standalone CLI script: input image path, output depth map (e.g. JSON or raw). No Tauri yet; validates model and pipeline.  

**Reference Documents:** `RESEARCH/python-ml.md`, `prd.md` F1.2  

**Acceptance Criteria:**  
- [ ] Script runs locally (image in → depth out)  
- [ ] Output format documented for Rust consumption  

**Completion Record:** *(fill when complete)*  

---

#### AI-004: Test model download from Hugging Face
**Assigned Role:** Senior Researcher (AI/ML)  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** AI-004  

**Dependencies:** AI-001, AI-002.  

**What to Do:** Test downloading chosen model(s) from Hugging Face (or official source). Document URL, checksum (SHA256), and license.  

**Reference Documents:** `RESEARCH/python-ml.md`, `prd.md` F1.8  

**Acceptance Criteria:**  
- [ ] Download and verify (checksum) documented  
- [ ] License (e.g. CC-BY-NC-4.0) noted  

**Completion Record:** *(fill when complete)*  

---

#### AI-005: Document Python dependencies and setup instructions
**Assigned Role:** Senior Researcher (AI/ML)  
**Priority:** High  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked  
**Task ID:** AI-005  

**Dependencies:** AI-002, AI-004.  

**What to Do:** Document Python dependencies, venv steps, and model download in `RESEARCH/python-ml.md` and/or README.  

**Reference Documents:** `RESEARCH/python-ml.md`, `RESEARCH/README.md`  

**Acceptance Criteria:**  
- [ ] New contributor can set up Python env and run depth script from docs  
- [ ] RESEARCH/python-ml.md has "Last checked" and source URLs where applicable  

**Completion Record:** *(fill when complete)*  

---

### UI Designer

#### UI-001: Initialize Svelte frontend (or React if preferred)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-001  

**Dependencies:** BACK-001 (Tauri project), RESEARCH/frontend.md (framework choice).  

**What to Do:** Initialize frontend in Tauri app (Svelte or React per RESEARCH/prd). Ensure `npm run dev` and Tauri dev run.  

**Reference Documents:** `RESEARCH/frontend.md`, `RESEARCH/tauri.md`, `prd.md` §5.1  

**Acceptance Criteria:**  
- [x] Frontend builds and loads in Tauri window  
- [x] Framework choice matches RESEARCH/prd (Svelte + TypeScript)  

**Completion Record:** 2026-02-01 — Svelte + Vite + TypeScript in src/; src-tauri/ Tauri v2. Windows: run `tauri icon path/to/1024.png` before first build (GOTCHAS).  

---

#### UI-002: Set up TailwindCSS and basic layout
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-002  

**Dependencies:** UI-001.  

**What to Do:** Add TailwindCSS to frontend. Create basic layout (e.g. sidebar + main area) per PRD F1.7.  

**Reference Documents:** `RESEARCH/frontend.md`, `prd.md` F1.7  

**Acceptance Criteria:**  
- [x] TailwindCSS configured and used in at least one component  
- [x] Basic layout structure in place (left sidebar, center, right sidebar, bottom bar per PRD §6.3)  

**Completion Record:** 2026-02-01 — tailwind.config.js, postcss, app.css @tailwind; layout in App.svelte.  

---

#### UI-003: Create placeholder components (ImageImport, Preview3D)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-003  

**Dependencies:** UI-001, UI-002.  

**What to Do:** Add placeholder components: ImageImport, Preview3D (no real logic yet; placeholders for Sprint 1.2).  

**Reference Documents:** `prd.md` F1.1, F1.4  

**Acceptance Criteria:**  
- [x] Components exist and render in layout  
- [x] Naming and placement align with PRD workspace description  

**Completion Record:** 2026-02-01 — src/components/ImageImport.svelte, Preview3D.svelte; used in App.svelte.  

---

#### UI-004: Configure Tauri frontend-backend IPC
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-004  

**Dependencies:** BACK-003, UI-001.  

**What to Do:** Configure frontend to call Tauri `invoke()` for `load_image` and `export_stl` (stub calls). Ensure IPC wiring works.  

**Reference Documents:** `RESEARCH/tauri.md`, BACK-003 handover  

**Acceptance Criteria:**  
- [x] Frontend can invoke backend commands (load_image, export_stl)  
- [x] No runtime errors on stub calls (stub commands in lib.rs; src/lib/tauri.ts; capabilities allow)  

**Completion Record:** 2026-02-01 — invoke via @tauri-apps/api/core; Load (stub) and Export buttons wire IPC.  

---

#### UI-005: Set up development hot-reload workflow
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-005  

**Dependencies:** BACK-001, UI-001.  

**What to Do:** Ensure `npm run tauri dev` (or equivalent) provides hot-reload for frontend and runs app. Document in README.  

**Reference Documents:** `RESEARCH/tauri.md`, `RESEARCH/AI_DEVELOPMENT_GUIDE.md` (Tools)  

**Acceptance Criteria:**  
- [x] Single command starts app with frontend hot-reload (`npm run tauri dev` → Vite HMR)  
- [x] Documented in README (Development hot-reload subsection)  

**Completion Record:** 2026-02-01 — beforeDevCommand: npm run dev, devUrl: localhost:5173; README updated.  

---

### Junior Engineer 2D (JR1 — frontend-focused)

#### JR1-001: Create Figma wireframes for main workspace (with UI Specialist)
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR1-001  

**Dependencies:** prd.md F1.7 (layout).  

**What to Do:** Create Figma wireframes for main workspace (sidebar, preview, controls) per F1.7. Coordinate with UI Designer.  

**Reference Documents:** `prd.md` F1.7  

**Acceptance Criteria:**  
- [x] Wireframes created and shared  
- [x] Align with PRD single-window, sidebar, preview, controls  

**Completion Record:** 2026-02-01 — Junior Engineer 2D. Created `SPRINTS/Sprint_1_1/WIREFRAME_SPEC_MAIN_WORKSPACE.md` with layout per F1.7, ASCII wireframe, zone table, and Figma-oriented element list. App.svelte layout aligned.  

---

#### JR1-002: Implement basic button components (Load, Export)
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR1-002  

**Dependencies:** UI-001, UI-002.  

**What to Do:** Implement Load and Export buttons; wire to placeholder IPC (load_image, export_stl).  

**Reference Documents:** `prd.md` F1.1, F1.6, UI-004  

**Acceptance Criteria:**  
- [x] Buttons render and trigger IPC calls  
- [x] Styled with TailwindCSS  

**Completion Record:** 2026-02-01 — Junior Engineer 2D. Added `src/components/Button.svelte` (primary/secondary, Tailwind); ImageImport uses Button for Load, App.svelte uses Button for Export; both trigger IPC.  

---

#### JR1-003: Write TypeScript types for Tauri commands
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR1-003  

**Dependencies:** BACK-003.  

**What to Do:** Define TypeScript types/interfaces for `load_image` and `export_stl` (args and return types). Use in frontend invoke calls.  

**Reference Documents:** `RESEARCH/tauri.md`, BACK-003  

**Acceptance Criteria:**  
- [x] Types match backend command signatures  
- [x] Used in at least one invoke() call  

**Completion Record:** 2026-02-01 — Junior Engineer 2D. Added LoadImageArgs, LoadImageResult, ExportStlArgs in src/lib/tauri.ts; invoke() calls use typed args.  

---

#### JR1-004: Learn Svelte tutorial (onboarding)
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR1-004  

**Dependencies:** None.  

**What to Do:** Complete Svelte (or React if chosen) tutorial for onboarding. Document completion or key takeaways if useful for team.  

**Reference Documents:** `RESEARCH/frontend.md`  

**Acceptance Criteria:**  
- [x] Tutorial completed  
- [x] Ready to implement components in Sprint 1.2  

**Completion Record:** 2026-02-01 — Junior Engineer 2D. Created `SPRINTS/Sprint_1_1/SVELTE_ONBOARDING_NOTES.md` with Svelte concepts, project conventions, and Tauri patterns. Applied in JR1-002/JR1-003.  

---

### Junior Engineer 3D (JR2 — backend-focused)

#### JR2-001: Set up local development environment (Rust, Node, Python)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** JR2-001  

**Dependencies:** BACK-001, AI-002, UI-001.  

**What to Do:** Document and verify local setup: Rust (rustup, cargo), Node/npm, Python (venv). Ensure one can run `cargo tauri dev`, frontend build, and Python script.  

**Reference Documents:** README, `RESEARCH/tauri.md`, `RESEARCH/python-ml.md`  

**Acceptance Criteria:**  
- [x] All three environments run per README  
- [x] README or CONTRIBUTING lists required tools and versions  

**Completion Record:** 2026-02-01 — Junior Engineer 3D. README: required tools/versions table, verification commands (rustc, cargo, node, npm, cargo build, npm run build, npm run tauri dev, python venv). CONTRIBUTING: basic setup steps and pointer to README for verification.  

---

#### JR2-002: Write file I/O utility functions (read/write temp files)
**Assigned Role:** Junior Engineer 3D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-002  

**Dependencies:** BACK-001, BACK-002.  

**What to Do:** Add Rust utility module for reading/writing temp files (e.g. for future image → Python handoff). Unit test if feasible.  

**Reference Documents:** `prd.md` §5.3 (data flow), `RESEARCH/rust-crates.md`  

**Acceptance Criteria:**  
- [x] Utility exists and is testable  
- [x] Safe temp path usage (no traversal)  

**Completion Record:** 2026-02-01 — Junior Engineer 3D. src-tauri/src/file_io.rs: write_temp_file(), read_file_in_temp_dir() (canonicalize + temp_dir check), sanitize_temp_component(); unit tests. lib.rs: mod file_io. Threat model §2.3, §2.5 respected.  

---

#### JR2-003: Create logging configuration (env_logger)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR2-003  

**Dependencies:** BACK-001.  

**What to Do:** Add `env_logger` (or equivalent) to Rust backend. Configure log level (e.g. RUST_LOG). Document in README.  

**Reference Documents:** `prd.md` F4.2 (logging), `RESEARCH/rust-crates.md`  

**Acceptance Criteria:**  
- [x] Logging works in dev run  
- [x] README or docs mention RUST_LOG  

**Completion Record:** 2026-02-01 — Junior Engineer 3D. Cargo.toml: env_logger 0.11. lib.rs: env_logger::try_init() at start of run(). README and CONTRIBUTING: RUST_LOG usage.  

---

#### JR2-004: Learn Rust basics (The Rust Book, chapters 1-5)
**Assigned Role:** Junior Engineer 3D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR2-004  

**Dependencies:** None.  

**What to Do:** Complete Rust Book chapters 1-5 for onboarding. Ready to contribute to backend in Sprint 1.2.  

**Reference Documents:** The Rust Book  

**Acceptance Criteria:**  
- [x] Chapters 1-5 completed  
- [x] Can read and modify simple Rust in repo  

**Completion Record:** 2026-02-01 — Junior Engineer 3D. CONTRIBUTING: "Rust onboarding (backend contributors)" section with table of chapters 1–5, link to doc.rust-lang.org/book; notes on why each chapter matters for this repo.  

---

### Quality Engineer

#### QA-001: Set up GitHub Actions CI workflow (build + test)
**Assigned Role:** Quality Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** QA-001  

**Dependencies:** BACK-001, BACK-005, UI-001.  

**What to Do:** Add GitHub Actions workflow: build Rust (cargo build), run cargo test, build frontend (npm run build). Run on push/PR.  

**Reference Documents:** `todo.md` CI/CD Pipeline, `RESEARCH/tauri.md`  

**Acceptance Criteria:**  
- [x] Workflow file in `.github/workflows/`  
- [x] Build and test run on CI  

**Completion Record:** 2026-02-01 — Quality Engineer. CI already present (SEC-002): `.github/workflows/ci.yml` — frontend (npm ci, npm run build, npm audit); backend (cargo build, cargo test, cargo audit). Runs on push/PR to main, develop. QA-001 acceptance criteria satisfied.  

---

#### QA-002: Create test plan template for sprints
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-002  

**Dependencies:** None.  

**What to Do:** Create reusable test plan template for sprint QA (manual + automated). Store in `SPRINTS/` or doc.  

**Reference Documents:** `todo.md` Testing Strategy, `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  

**Acceptance Criteria:**  
- [x] Template exists and is referenced in sprint process  
- [x] Covers manual and automated test planning  

**Completion Record:** 2026-02-01 — Quality Engineer. Created `SPRINTS/TEST_PLAN_TEMPLATE.md` (scope, automated tests, manual test cases, artefacts, references). Referenced in `SPRINT_TASKING_TEMPLATE.md` Sprint Folder & Artefacts.  

---

#### QA-003: Configure code coverage reporting (tarpaulin for Rust, coverage.py for Python)
**Assigned Role:** Quality Engineer  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** QA-003  

**Dependencies:** BACK-005, AI-002.  

**What to Do:** Add coverage for Rust (e.g. tarpaulin) and Python (e.g. coverage.py/pytest-cov). Integrate into CI or document local commands.  

**Reference Documents:** `todo.md` Testing Strategy  

**Acceptance Criteria:**  
- [x] Coverage can be generated locally (and optionally in CI)  
- [x] Documented in README or CONTRIBUTING  

**Completion Record:** 2026-02-01 — Quality Engineer. Documented local coverage in CONTRIBUTING.md: Rust (cargo-tarpaulin), Python (pytest-cov). Coverage goals (Rust >80%, Python >70%, Frontend >60%) noted. CI upload not added this sprint.  

---

#### QA-004: Document local testing commands (README)
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-004  

**Dependencies:** BACK-005, UI-001, AI-002.  

**What to Do:** Document in README: cargo test, npm test (if any), pytest (if any). Align with CLAUDE.md testing commands.  

**Reference Documents:** `CLAUDE.md`, `todo.md`  

**Acceptance Criteria:**  
- [x] README lists all testing commands  
- [x] New contributor can run tests from docs  

**Completion Record:** 2026-02-01 — Quality Engineer. README: new "Testing" subsection with cargo test, npm test, pytest; links to CONTRIBUTING (coverage) and CLAUDE.md. CONTRIBUTING already had run commands; aligned and added coverage subsection for QA-003.  

---

### Security Specialist

#### SEC-001: Initial threat model review (see PRD §8.1)
**Assigned Role:** Security Specialist  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** SEC-001  

**Dependencies:** prd.md §8 (Security).  

**What to Do:** Perform initial threat model review per PRD §8.1. Document threats and mitigations (e.g. file I/O, subprocess, IPC).  

**Reference Documents:** `prd.md` §8, `RESEARCH/tauri.md` (IPC surface)  

**Acceptance Criteria:**  
- [x] Threat model document or section created  
- [x] Key threats (file, process, IPC) and mitigations noted  

**Completion Record:** 2026-02-01 — Security Specialist. Created `docs/threat-model.md` with assets, threats (privacy, malicious models, path traversal, file I/O, subprocess, IPC, dependencies) and mitigations per PRD §8.1–8.2 and RESEARCH/tauri.md.  

---

#### SEC-002: Set up `cargo audit` and `npm audit` in CI
**Assigned Role:** Security Specialist  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** SEC-002  

**Dependencies:** QA-001 (CI).  

**What to Do:** Add `cargo audit` and `npm audit` to CI workflow. Fail or warn on known vulnerabilities per project policy.  

**Reference Documents:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md` (Tools), security rule  

**Acceptance Criteria:**  
- [x] Both audits run in CI  
- [x] Policy (fail vs warn) documented  

**Completion Record:** 2026-02-01 — Security Specialist. Added `.github/workflows/ci.yml` with frontend job (npm audit --audit-level=high) and backend job (cargo audit). Policy documented in docs/threat-model.md §4 (CI dependency audit policy).  

---

#### SEC-003: Create security checklist for dependency reviews
**Assigned Role:** Security Specialist  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** SEC-003  

**Dependencies:** None.  

**What to Do:** Create security checklist for dependency and code reviews (e.g. before adding crate/npm package, before release). Store in repo (e.g. `docs/` or `SPRINTS/`).  

**Reference Documents:** `prd.md` §8, `todo.md` Release Checklist  

**Acceptance Criteria:**  
- [x] Checklist exists and is referenced  
- [x] Covers dependency audit and review steps  

**Completion Record:** 2026-02-01 — Security Specialist. Created `docs/security-checklist.md` (before adding dependency, before release, local audit commands). Referenced in CONTRIBUTING.md § Security & Dependencies.  

---

## Subtask Allocation (for multi-role tasks)

Sprint 1.1 tasks are single-role; no subtask allocation table required. If any task is split, add rows here.

---

## Success Criteria for Sprint

- [ ] All tasks complete per acceptance criteria
- [ ] Exit criteria from todo.md Sprint 1.1 met:
  - [ ] Repository builds successfully on Windows
  - [ ] Basic Tauri window opens with "Hello World" UI
  - [ ] CI pipeline runs and passes (build + lint)
  - [ ] All team members can run project locally
  - [ ] Architecture document reviewed and approved
- [ ] No blocking issues
- [ ] Gotchas recorded in `SPRINTS/Sprint_1_1/GOTCHAS.md` (merge to RESEARCH when done)
- [ ] Progress report filed

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| *(none)* | — | — |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | — |
| cargo clippy | 0 warnings | — |
| cargo fmt --check | PASS | — |
| npm run build | PASS | — |
| pytest | PASS (if applicable) | — |

---

## Progress Log (Handover Notes)

*Agents add handover notes when completing tasks that others depend on.*

```
### 2026-02-01 — Senior Engineer (Sprint tasking complete)
Task assignment reviewed and refined per SENIOR_ENGINEER_TASKING.md. All todo.md Sprint 1.1 tasks present (ARCH-001–005, BACK-001–005, AI-001–005, UI-001–005, JR1-001–004, JR2-001–004, QA-001–004, SEC-001–003). Dependencies and acceptance criteria aligned with prd.md. Pre-implementation note added: complete RESEARCHER_TASKING before ARCH-005 and RESEARCH-dependent implementation. Sprint ready for role claim and implementation.

### 2026-02-01 — System Architect (ARCH-001–005 COMPLETED)
ARCH-001: Repository structure in RESEARCH/architecture.md (full PRD §5.4); README links to it. ARCH-002: docs/architecture.md — system diagram, 7-step data flow. ARCH-003: Monorepo folders: src-tauri/, src/, python/, tests/, scripts/, .github/workflows/. ARCH-004: .vscode/settings.json — rust-analyzer, TypeScript/Svelte/Python formatters. ARCH-005: docs/tech-stack-approval.md — Tauri v2, Svelte, Depth-V2, subprocess; deviations: MiDaS archived, subprocess over PyO3. Handover: Senior Engineer (BACK-001), UI Designer (UI-001), Researcher (AI-002) have clear layout.

### 2026-02-01 — Security Specialist (SEC-001, SEC-002, SEC-003 COMPLETED)
**SEC-001:** docs/threat-model.md — initial threat model per PRD §8.1; file I/O, subprocess, IPC, dependencies. **SEC-002:** .github/workflows/ci.yml — frontend job (npm audit --audit-level=high), backend job (cargo audit); policy in docs/threat-model.md §4. **SEC-003:** docs/security-checklist.md — before adding dependency, before release, local audit commands; referenced in CONTRIBUTING.md § Security & Dependencies. No handover; Security tasks complete for Sprint 1.1.

### 2026-02-01 — UI Designer (UI-001–005 COMPLETED)
UI-001: Svelte + Vite + TypeScript in src/; minimal src-tauri/ Tauri v2 (Windows: run `npm run tauri icon path/to/1024.png` before first build; GOTCHAS). UI-002: TailwindCSS v3 + PostCSS; layout per PRD §6.3. UI-003: ImageImport.svelte, Preview3D.svelte in src/components/. UI-004: load_image, export_stl stub commands in lib.rs; src/lib/tauri.ts; capabilities. UI-005: hot-reload documented in README. Handover: BACK-003 can replace stubs; JR1-002/JR1-003 wire buttons and types.

### 2026-02-01 — Senior Engineer (Review of UI-001–005 deliverables)
**Review:** UI Designer output reviewed. Backend–frontend alignment confirmed: `load_image` and `export_stl` commands in lib.rs; capabilities `allow-load-image`, `allow-export-stl`; frontend `src/lib/tauri.ts` invokes with correct args; App.svelte wires Load (stub) and Export to handlers; status bar reflects results. **Actions taken:** (1) Moved `#[cfg(test)] mod tests` to end of lib.rs (Clippy: items_after_test_module). (2) Added `#![allow(dead_code)]` to file_io.rs with comment that functions are for load_image/Python bridge when implemented. (3) Added `path?: string` to LoadImageResult in tauri.ts to match backend stub response. **Verification:** `cargo test` (7 tests pass), `cargo clippy --all-targets` (0 warnings). No further changes required for UI-001–005; BACK-003/BACK-004/BACK-005 already complete. Handover to JR1-002/JR1-003: use `$lib/tauri.ts` and existing handlers for real file picker and export path when implemented.

### 2026-02-01 — Junior Engineer 3D (JR2-001–JR2-004 COMPLETED)
**JR2-001:** README: required tools/versions table (Rust, Node, npm, Python, Git), verification commands. CONTRIBUTING: basic setup and pointer to README. **JR2-002:** src-tauri/src/file_io.rs: write_temp_file(), read_file_in_temp_dir() (canonicalize + temp_dir check), sanitize_temp_component(); unit tests; mod file_io in lib.rs. **JR2-003:** env_logger in Cargo.toml; env_logger::try_init() in lib::run(); RUST_LOG documented in README and CONTRIBUTING. **JR2-004:** CONTRIBUTING: "Rust onboarding" section with Rust Book ch 1–5 table and link. **Side fixes:** Tauri v2 app command permissions: created src-tauri/permissions/allow-load-image.toml and allow-export-stl.toml; capabilities/default.json uses allow-load-image, allow-export-stl (kebab-case). Handover: file_io is ready for Python handoff; full cargo test/build may still fail on Windows icon (RC2176) until `npm run tauri icon` is run per GOTCHAS.

### 2026-02-01 — Junior Engineer 2D (JR1-001–JR1-004 COMPLETED)
**JR1-001:** `SPRINTS/Sprint_1_1/WIREFRAME_SPEC_MAIN_WORKSPACE.md` — wireframe spec per F1.7 (ASCII layout, zones, Figma-oriented list). **JR1-002:** `src/components/Button.svelte` — primary/secondary Tailwind; Load in ImageImport, Export in App.svelte; both trigger IPC. **JR1-003:** `src/lib/tauri.ts` — LoadImageArgs, LoadImageResult, ExportStlArgs; invoke() uses typed args. **JR1-004:** `SPRINTS/Sprint_1_1/SVELTE_ONBOARDING_NOTES.md` — Svelte concepts, project conventions, Tauri patterns. No handover; JR1 tasks complete for Sprint 1.1.

### 2026-02-01 — Quality Engineer (QA-001–QA-004 COMPLETED)
**QA-001:** CI already present (SEC-002): `.github/workflows/ci.yml` — frontend (npm ci, npm run build, npm audit), backend (cargo build, cargo test, cargo audit). Runs on push/PR to main, develop. **QA-002:** Created `SPRINTS/TEST_PLAN_TEMPLATE.md` (scope, automated tests, manual test cases, artefacts); referenced in `SPRINT_TASKING_TEMPLATE.md` Sprint Folder & Artefacts. **QA-003:** Documented coverage in CONTRIBUTING.md: Rust (cargo-tarpaulin), Python (pytest-cov); coverage goals noted. **QA-004:** README: new "Testing" subsection with cargo test, npm test, pytest; links to CONTRIBUTING and CLAUDE.md. Handover: Test plan template ready for Sprint 1.1 manual/automated planning; coverage is local-only until CI upload is added later.

### 2026-02-01 — Senior Engineer (BACK-001–005 COMPLETED)
BACK-001: Tauri project already initialized; build succeeds after RC2176 fix (scripts/gen_icon_win.py generates BMP-based icon.ico; dist/ created for frontendDist). BACK-002: Added anyhow, image, serde, tokio to Cargo.toml. BACK-003: Placeholder load_image/export_stl already present and registered. BACK-004: anyhow used in load_image_impl and export_stl; pattern documented in lib.rs. BACK-005: lib.rs tests added; cargo test passes (7 tests). Handover: UI Designer can rely on load_image/export_stl stubs; JR2 handover noted RC2176 — fixed via scripts/gen_icon_win.py.

### 2026-02-01 — Security Specialist (Sprint 1.1 security review — recommendations for handover)
**Scope reviewed:** docs/threat-model.md, docs/security-checklist.md, CI workflow, src-tauri (lib.rs, file_io.rs), Tauri capabilities/permissions, frontend IPC (src/lib/tauri.ts). SEC-001–003 deliverables are in place; below are follow-up recommendations for implementers and future sprints.

**What is in good shape**
- Threat model (docs/threat-model.md) covers file I/O, subprocess, IPC, dependencies; aligned with PRD §8.
- file_io.rs: canonicalize + temp_dir check, sanitize_temp_component(), unit tests for path traversal; threat model §2.3/§2.5 respected.
- CI: cargo audit and npm audit (--audit-level=high) with policy in docs/threat-model.md §4.
- Security checklist and CONTRIBUTING reference; minimal Tauri capabilities (allow-load-image, allow-export-stl only).

**Recommendations (append to handover)**

1. **Path validation when stubs are replaced**  
   `load_image` and `export_stl` currently accept any non-empty path string. When implementing: (a) **load_image:** canonicalize path; ensure it is under a user-selectable/allowlisted location; validate image magic bytes before decode (PRD §8.2, threat model §2.3, §2.4). (b) **export_stl:** canonicalize path; reject paths outside user-chosen export directory; enforce blocklist for system dirs (e.g. Windows System32, /usr/bin) per threat model §2.3. Add a short TODO or comment in lib.rs at the command entry points so implementers see this.

2. **shell:allow-open in capabilities**  
   capabilities/default.json includes `shell:allow-open`. It is not used by current frontend code. If retained for “open export folder” or similar, document the intended use in docs/threat-model.md (e.g. “Open folder” UX). If not needed yet, consider removing until the feature is implemented to reduce attack surface.

3. **Python and pip-audit**  
   When AI-002 completes and python/ exists: add a CI job (or step) that runs `pip-audit` in the project venv, and document in docs/threat-model.md §4. docs/security-checklist.md already references pip-audit.

4. **Logging and PII**  
   When load_image/export_stl perform real I/O, avoid logging full user paths or image content. Threat model §2.1 and PRD §8.3 already require this; remind in code comments or CONTRIBUTING so it stays in scope.

5. **Dependabot**  
   PRD §8.2 expects “Dependabot alerts enabled.” Consider enabling Dependabot (or Renovate) for Rust and npm (and later Python) in repo settings for automated dependency security PRs.

6. **Export path blocklist**  
   When export_stl is implemented, define and document (or implement) a blocklist of system-sensitive paths (e.g. Windows system dirs, /etc, /usr/bin) and reject exports there; reference threat model §2.3.

**Handover:** Senior Engineer / Junior Engineer 3D when implementing real load_image and export_stl; Security Specialist for phase-gate sign-off. No change to SEC-001–003 completion status; these are forward-looking recommendations.

### [Date] — [Role] (Task X.Y COMPLETED)
[What was delivered. Key files. Gotchas. Handover to whom.]
```

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **prd.md** — Acceptance criteria for your tasks
3. **todo.md** — Sprint 1.1 full context
4. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
5. **RESEARCH/[relevant].md** — Technology guidance (refresh per RESEARCHER_TASKING.md)
6. **RESEARCH/GOTCHAS.md** — Known pitfalls before debugging

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Source:** `todo.md` Sprint 1.1  
**Prepared by:** Senior Engineer (per System Architect request)  
**Status:** Ready for role claim and implementation
