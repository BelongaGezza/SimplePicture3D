# AI Development Guide

**Purpose:** Multi-agent coordination patterns, workflows, and handover for SimplePicture3D.

**RESEARCH folder:** All agents should review `RESEARCH/` for latest technology guidance before tasks. Record gotchas in `RESEARCH/GOTCHAS.md` when debugging.

---

## Multi-Agent Coordination

### Role-Claiming Model

Unassigned agents claim roles from sprint tasking files:

1. Agent reads sprint file (e.g., `SPRINTS/sprint-NNN-feature.md`)
2. Locates Role Assignment Table
3. Claims available role (Status = "Available")
4. Updates table: Status → "In Progress", adds session ID
5. Reads persona file from Persona File column
6. Adopts that identity and proceeds with assigned tasks

**Critical:** Persona File column MUST reference `.agents/*.md` files.

### Task Handover Protocol

When Task A depends on Task B completion:

**Task B Owner (Provider):**
1. Complete task implementation
2. Update task status to "Completed"
3. Add handover notes in Progress Log:
   - What was delivered
   - Where to find key files
   - Any gotchas or considerations
   - Test coverage added
4. If debugging uncovered gotchas, add to `RESEARCH/GOTCHAS.md`
5. Tag dependent task with ready status

**Task A Owner (Consumer):**
1. Read handover notes from Progress Log
2. Review `RESEARCH/GOTCHAS.md` for relevant gotchas
3. Verify deliverables exist
4. Proceed with dependent task

---

## Agent-Specific Guidelines

### System Architect
- **Review:** `RESEARCH/architecture.md`, `RESEARCH/tauri.md`
- **Deliverables:** Architecture Decision Records, API specifications
- **Update:** `RESEARCH/architecture.md` when architecture changes

### Senior Engineer
- **Review:** `RESEARCH/rust-crates.md`, `RESEARCH/tauri.md`, `RESEARCH/python-ml.md`
- **Creates:** Sprint tasking using `SPRINTS/SPRINT_TASKING_TEMPLATE.md` — see `todo.md` § Sprint Creation Process. Create `SPRINTS/Sprint_X_Y/` folder and `SPRINT_X_Y_Task_Assignment.md`; store all sprint artefacts in that folder.
- **Record:** Gotchas in `RESEARCH/GOTCHAS.md` when debugging

### Junior Engineer 2D / 3D
- **Review:** `RESEARCH/rust-crates.md`, `RESEARCH/frontend.md`, `RESEARCH/threejs.md` (as relevant)
- **Record:** and **Review** Gotchas in `RESEARCH/GOTCHAS.md` when debugging

### Security Specialist
- **Review:** `RESEARCH/tauri.md` (IPC surface), `prd.md` §8
- **Tools:** cargo audit, npm audit, pip-audit

### Documentation Specialist
- **Review:** All `RESEARCH/` files for accuracy when updating docs

### Researcher
- **Owns:** Populating and maintaining `RESEARCH/` technology files
- **On request:** Check deprecations, version changes, official sources
- **Record:** Source URLs and "Last checked" dates in each file

### UI Designer
- **Review:** `RESEARCH/frontend.md`, `RESEARCH/threejs.md`
- **Record:** UI/frontend gotchas in `RESEARCH/GOTCHAS.md`

---

## Tech Stack (SimplePicture3D)

| Layer | Technologies | Research File |
|-------|--------------|---------------|
| Shell | Tauri | tauri.md |
| Backend | Rust (image, stl_io, obj, tokio, serde, anyhow) | rust-crates.md |
| AI | Python, PyTorch, Depth-Anything-V2, MiDaS | python-ml.md |
| Frontend | Svelte or React, TypeScript, TailwindCSS | frontend.md |
| 3D | Three.js | threejs.md |

---

## Tools and Commands

- `cargo fmt` — Format Rust
- `cargo clippy` — Lint Rust
- `cargo test` — Rust tests
- `cargo tauri dev` — Run Tauri app in dev
- `cargo tauri build` — Build for production
- `npm run dev` / `npm run build` — Frontend
- `pytest` — Python tests
- `cargo audit`, `npm audit`, `pip-audit` — Security

---

*Last updated: 2026-02-01*
