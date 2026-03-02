# Sprint 2.2: Undo/Redo, Curve Persistence, State ADR, Wireframe/Solid

**Sprint Duration:** 2 weeks  
**Sprint Goal:** Implement F2.4 Undo/Redo, persist curve control points, author state management ADR, and fix or remove Wireframe/Solid UI (per Consultant_Critical_Review_28Feb2026 §2.2, §2.6).  
**Target Release:** Phase 2 — Enhanced UX  
**Phase:** 2 — Enhanced UX  
**Source:** `todo.md` — Sprint 2.2  
**Last Updated:** 2026-03-01

---

## Sprint Folder & Artefacts

**All sprint artefacts MUST be stored in this sprint's folder:**

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_2_2/SPRINT_2_2_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_2_2/TEST_PLAN_2_2.md` | QA test planning (copy from template as needed) |
| Progress Report | `SPRINTS/Sprint_2_2/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_2_2/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_2_2/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Architect Approval | `SPRINTS/Sprint_2_2/ARCHITECT_APPROVAL.md` | If required for phase gate |
| Gotchas Log | `SPRINTS/Sprint_2_2/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

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
2. **Set your Cursor title to the role name.**
3. **Read the persona file** listed in the "Persona File" column
4. **Adopt that persona** for all remaining work

### Step 3: Become Your Role
- Embody the agent's identity, expertise, and responsibilities
- Follow the persona file's guidance and project references

**If all roles show "In Progress" or "Complete", STOP. No work available.**

### Step 4: Update status
- While progressing your role, update the status per the Status Values defined below.

### Optional: One-shot role assumption
Run the Cursor command **"Create One-Shot Assume-Role Commands for This Sprint"** with this file @-mentioned to generate `.cursor/commands/assume-sprint-2-2-<role-slug>.md`. Run any of those commands in a chat to assume that role one-shot.

---

## Roles required for this sprint

| Role | Why required |
|------|--------------|
| System Architect | ARCH-401–404: undo/redo architecture, mutable ops, history budget, state management ADR (TD-01) |
| Senior Engineer | BACK-1401–1404: command trait, history stack, wrap depth adjustments, Tauri undo/redo/clear_history |
| UI Designer | UI-1401–1404: Undo/Redo toolbar, shortcuts, Wireframe/Solid fix or remove (TD-02), remove "Sprint 1.8" overlay |
| Junior Engineer 2D | CURVE-001, JR2-1401, JR2-1402: curveControlPoints in AppSettings, tests for command/undo and history limits |
| Quality Engineer | QA-1401–1402: manual undo/redo verification, start macOS smoke tests (TD-05) |

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | Complete | Cursor Agent | ARCH-401, ARCH-402, ARCH-403, ARCH-404 | Undo/redo design; state ADR (TD-01) |
| Senior Engineer | `.agents/senior-engineer.md` | Complete | Cursor Agent | BACK-1401, BACK-1402, BACK-1403, BACK-1404 | Command pattern, Tauri commands |
| UI Designer | `.agents/ui-designer.md` | Complete | Cursor Agent | UI-1401, UI-1402, UI-1403, UI-1404 | Toolbar, shortcuts, Wireframe/Solid (TD-02); acceptance criteria verified; UI-1404 sprint refs removed |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Complete | Cursor Agent | CURVE-001, JR2-1401, JR2-1402 | JR2-1401, JR2-1402 complete |
| Quality Engineer | `.agents/quality-engineer.md` | In Progress | Cursor Agent | QA-1401, QA-1402 | QA-1401 not executed; fix `push_clears_redo` first |
| Senior Researcher | `.agents/researcher.md` | Available | - | — | No 2.2-specific tasks |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Complete | Cursor Agent | — | No 2.2-specific tasks; verified mesh/Wireframe/Solid, tests, GOTCHAS |
| Security Specialist | `.agents/security-specialist.md` | Available | - | — | No 2.2-specific tasks |
| Documentation Specialist | `.agents/documentation-specialist.md` | Complete | Cursor Agent | — | Sprint docs, progress report, user/dev docs verified |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References (Source of Truth)

- **Scope:** `prd.md` — Phase 2 (F2.4 Undo/Redo), acceptance criteria
- **Sprint source:** `todo.md` — Sprint 2.2
- **Consultant review:** `Consultant_Critical_Review_28Feb2026.md` §2.2, §2.6 (Wireframe/Solid, curve persistence)
- **Technical debt:** `todo.md` — Technical Debt Register (TD-01 State ADR, TD-02 Wireframe/Solid, TD-05 macOS smoke)
- **Architecture:** `RESEARCH/architecture.md`
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| Architecture (ARCH-401–404) | ✅ Complete | 100% |
| Backend undo/redo (BACK-1401–1404) | ✅ Complete | 100% |
| Curve persistence (CURVE-001) | ✅ Complete | 100% |
| UI (UI-1401–1404) | ✅ Complete | 100% |
| JR2 tests (JR2-1401, JR2-1402) | ✅ Complete | 100% |
| QA (QA-1401–1402) | 🔄 In Progress | 50% |

**Overall Sprint Progress:** [ ] Not Started / [x] In Progress / [ ] Complete

**Advisory (roles reset 2026-03-01):** Not all steps are complete. UI Designer, Junior Engineer 2D, Quality Engineer, and Documentation Specialist were reset to **Available**. See "Current Blockers" and task status below; claim a role to complete remaining work.

---

## Task Breakdown

### System Architect: Undo/Redo & State ADR

#### ARCH-401: Design undo/redo architecture (command pattern)
**Assigned Role:** System Architect  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** ARCH-401

**Dependencies:** None

**What to Do:**
- Design undo/redo using command pattern (execute, undo).
- Document where commands live (Rust backend vs frontend store) and how they integrate with existing depth adjustment flow.

**Reference Documents:** `RESEARCH/architecture.md`, `todo.md` Technical Debt Register TD-01

**Acceptance Criteria:**
- [x] ADR or architecture section describes command pattern and integration points
- [x] Mutable operations identified (depth params, curve, etc.)

---

#### ARCH-402: Define mutable operations to track
**Assigned Role:** System Architect  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** ARCH-402

**Dependencies:** ARCH-401

**What to Do:**
- List all user actions that mutate state and must be undoable (e.g. depth brightness, gamma, curve control points, invert, load image?, generate depth?).
- Clarify scope: depth adjustments only for 2.2, or include others.

**Acceptance Criteria:**
- [x] List of mutable operations documented
- [x] Scope agreed with Senior Engineer / UI for Sprint 2.2

---

#### ARCH-403: Memory budget for history stack (last 20 actions)
**Assigned Role:** System Architect  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** ARCH-403

**Dependencies:** ARCH-401

**What to Do:**
- Define history stack limit (e.g. last 20 actions per todo.md).
- Document serialization/size considerations if history is kept in backend.

**Acceptance Criteria:**
- [x] Max history length (20) documented
- [x] Behavior when limit reached (drop oldest) specified

---

#### ARCH-404: Author Svelte store / state management ADR (TD-01)
**Assigned Role:** System Architect  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** ARCH-404

**Dependencies:** ARCH-401, ARCH-402

**What to Do:**
- Author state management ADR: how app state (depth params, curve, history) is held and updated (Svelte stores, Tauri state, or hybrid).
- Required before further state-mutation features (masking, brushes in later sprints).

**Reference Documents:** `todo.md` TD-01, `RESEARCH/architecture.md`

**Acceptance Criteria:**
- [x] ADR added to RESEARCH/architecture.md (or linked doc)
- [x] Clear guidance for frontend state and backend sync

---

### Senior Engineer: Command Pattern & Tauri Commands

#### BACK-1401: Implement command trait (execute, undo)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-1401

**Dependencies:** ARCH-401 (design)

**What to Do:**
- Define command trait or enum in Rust: execute(), undo(), optionally description for UI.
- Each concrete command holds state needed to revert (e.g. previous depth params).

**Acceptance Criteria:**
- [x] Trait or enum implemented; at least one concrete command (e.g. depth adjustment) implemented
- [x] Undo restores previous state correctly

---

#### BACK-1402: History stack data structure
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-1402

**Dependencies:** BACK-1401, ARCH-403

**What to Do:**
- Implement history stack (e.g. Vec with max len 20); push on execute, pop on undo; redo stack if required.

**Acceptance Criteria:**
- [x] History limited to last 20 actions (per ARCH-403)
- [x] Clear behavior when stack is full (drop oldest)

---

#### BACK-1403: Wrap depth adjustments in commands
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-1403

**Dependencies:** BACK-1401, BACK-1402, ARCH-402

**What to Do:**
- When user changes depth params (brightness, gamma, contrast, invert, curve control points), create command, execute, push to history.
- Ensure current state is serializable so undo can restore.

**Acceptance Criteria:**
- [x] All ARCH-402 mutable depth operations wrapped in commands
- [x] Undo/redo updates backend state and (via IPC) frontend reflects it

---

#### BACK-1404: Tauri commands: undo, redo, clear_history
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-1404

**Dependencies:** BACK-1402, BACK-1403

**What to Do:**
- Expose Tauri commands: `undo`, `redo`, `clear_history` (optional). Return success + new state or error (e.g. nothing to undo).

**Acceptance Criteria:**
- [x] Frontend can invoke undo/redo/clear_history
- [x] Return value allows UI to enable/disable buttons (e.g. can_undo, can_redo)

---

### UI Designer: Toolbar, Shortcuts, Wireframe/Solid, Overlay

#### UI-1401: Undo/Redo buttons in toolbar
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-1401

**Dependencies:** BACK-1404

**What to Do:**
- Add Undo and Redo buttons to toolbar; call Tauri undo/redo; disable when nothing to undo/redo (use return from backend).

**Acceptance Criteria:**
- [x] Buttons visible and wired to backend
- [x] Buttons disabled when no history / no redo stack

---

#### UI-1402: Keyboard shortcuts (Ctrl+Z, Ctrl+Y)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-1402

**Dependencies:** UI-1401

**What to Do:**
- Ctrl+Z = undo, Ctrl+Y (or Ctrl+Shift+Z) = redo. Ensure no conflict with browser/OS; prefer Tauri keybinding or frontend key handler.

**Acceptance Criteria:**
- [x] Shortcuts work in app window
- [x] Document in UI or help (Phase 2)

---

#### UI-1403: Wireframe/Solid — fix or remove (TD-02)
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-1403

**Dependencies:** None (existing MeshData from get_mesh_data)

**What to Do:**
- Either: wire `MeshData.indices` to THREE.Mesh (WireframeGeometry / MeshPhongMaterial) so Wireframe and Solid modes work, OR remove the buttons until triangulation is fully supported (Sprint 1.8 has triangulation; confirm if indices are always present).

**Reference Documents:** Consultant_Critical_Review_28Feb2026 §2.2, §2.6; `todo.md` TD-02

**Acceptance Criteria:**
- [x] Wireframe/Solid either functional (indices used) or buttons removed/hidden with clear UX
- [x] No dead UI that does nothing

---

#### UI-1404: Remove or replace "Sprint 1.8" overlay message
**Assigned Role:** UI Designer  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** UI-1404

**Dependencies:** None

**What to Do:**
- Find any user-facing overlay or message that references "Sprint 1.8" (or internal sprint numbers) and remove or replace with neutral text.

**Acceptance Criteria:**
- [x] No internal sprint numbers visible to user

---

### Junior Engineer 2D: Curve Persistence & Tests

#### CURVE-001: Persist curveControlPoints in AppSettings (Consultant §2.6)
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** CURVE-001

**Dependencies:** Existing AppSettings and settings load/save (Sprint 1.9)

**What to Do:**
- Add `curveControlPoints` (or equivalent) to `AppSettings`; persist and load in settings so curve control points survive restart.

**Reference Documents:** Consultant_Critical_Review_28Feb2026 §2.6; Sprint 2.1 CurvesTool / BACK-1102

**Acceptance Criteria:**
- [x] Curve control points saved in settings file
- [x] On load, CurvesTool shows restored curve; backend applies it to depth

---

#### JR2-1401: Write tests for command execution/undo
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-1401

**Dependencies:** BACK-1401, BACK-1403

**What to Do:**
- Unit tests: execute command → state changes; undo → state restored. Cover at least one depth adjustment command.

**Acceptance Criteria:**
- [x] Tests in Rust; run with `cargo test`
- [x] Execute and undo both verified

---

#### JR2-1402: Test history stack limits (>20 actions)
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR2-1402

**Dependencies:** BACK-1402, ARCH-403

**What to Do:**
- Test that after 21+ actions, history does not grow beyond 20 (oldest dropped).

**Acceptance Criteria:**
- [x] Automated test; history size capped at 20

---

### Quality Engineer: Manual & macOS Smoke

#### QA-1401: Manual test — perform actions, undo, redo, verify state
**Assigned Role:** Quality Engineer  
**Priority:** Critical  
**Status:** [ ] In Progress (procedure ready; deps complete; manual execution requires human tester)  
**Task ID:** QA-1401

**Dependencies:** UI-1401, UI-1402, BACK-1403, BACK-1404

**What to Do:**
- Manual test: change depth (sliders, curve), undo, redo; verify depth preview and mesh/export reflect state. Test boundary (no undo available, no redo available).

**Acceptance Criteria:**
- [x] Procedure in TEST_PLAN_2_2.md or MANUAL_TEST_REPORT.md
- [ ] All scenarios pass; any regression noted

---

#### QA-1402: Start macOS smoke tests (TD-05; document)
**Assigned Role:** Quality Engineer  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** QA-1402

**Dependencies:** macOS build (if available) or document plan

**What to Do:**
- Begin macOS smoke testing per TD-05: document environment, build steps, and a short smoke checklist (e.g. launch, load image, generate depth). If no macOS env, document "Not yet run" and plan.

**Reference Documents:** `todo.md` TD-05

**Acceptance Criteria:**
- [x] Document in sprint folder (e.g. MACOS_SMOKE.md or TEST_PLAN_2_2.md)
- [x] Either smoke run completed or deferred with clear plan *(deferred; plan in MACOS_SMOKE.md)*

---

## Success Criteria for Sprint

- [ ] All tasks complete per acceptance criteria
- [ ] Exit criteria from todo.md met:
  - [ ] Undo/Redo functional for depth adjustments (and curve when persisted)
  - [ ] Curve control points persisted in AppSettings
  - [ ] State management ADR documented
  - [ ] Wireframe/Solid either working or removed from UI
  - [ ] Keyboard shortcuts work; no internal sprint numbers in user-facing messages
- [ ] No blocking issues
- [ ] Gotchas in `SPRINTS/Sprint_2_2/GOTCHAS.md` (merge to RESEARCH when done)
- [ ] Progress report filed

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| ~~`cargo test`: `undo::tests::push_clears_redo` fails~~ | — | **Resolved:** Test passes (2026-03-01); automated gate unblocked |
| ~~JR2-1401, JR2-1402 not started~~ | ~~Junior Engineer 2D~~ | Resolved: Rust tests added (command_execute_undo_restores_state, history_cap_at_max) |
| QA-1401 manual cases not run | Quality Engineer | Procedure ready; manual execution requires human tester (TEST_PLAN_2_2.md §3.2). Automated gate clear. |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | PASS (151 passed, 6 ignored) |
| cargo clippy | 0 warnings | 0 warnings |
| npm run build | PASS | PASS |
| npm test | PASS | PASS (39 tests) |

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **prd.md** — Phase 2, F2.4 Undo/Redo
3. **todo.md** — Sprint 2.2, Technical Debt Register (TD-01, TD-02, TD-05)
4. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
5. **Consultant_Critical_Review_28Feb2026.md** §2.2, §2.6
6. **RESEARCH/GOTCHAS.md** — Known pitfalls

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Status:** Roles reset to Available; ready for role assignment and execution
