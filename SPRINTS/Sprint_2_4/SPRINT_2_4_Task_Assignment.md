# Sprint 2.4: Progress Streaming for Depth Estimation

**Sprint Duration:** 2 weeks (10 working days)  
**Sprint Goal:** Close the long UX gap during depth estimation: stream Python `PROGRESS` / `STAGE` lines from stderr to the frontend in real time so users see a determinate percentage (not a silent multi-minute wait on large images). Design the event pattern for reuse in Sprint 2.8 (async export).  
**Target Release:** v0.4.0-beta.1 (progress streaming)  
**Phase:** 2 — Enhanced UX  
**Source:** `todo.md` — Sprint 2.4; `RESEARCH/architecture.md` ADR-002 addendum (ARCH-501)  
**Last Updated:** 2026-04-05

---

## Sprint scope (two layers)

| Layer | Contents | Notes |
|-------|----------|--------|
| **Core (strict `todo.md` §Sprint 2.4)** | BACK-205-STREAM, BACK-205-IPC, UI-304, QA-304-STREAM | Exit criteria: stderr → Tauri event → UI % |
| **Sprint envelope (same calendar; gates Phase 2)** | SEC-202 (SHA256 model verify — required before Sprint 2.5), Sprint 2.3 carryover (JR2-1301–1303, QA-1301–1303), DOC-204 | Tracked here so one sprint folder holds sign-off artefacts |

**Implementation status (2026-04-05):** Core streaming + UI + SEC-202 + JR2 carryover + DOC-204 are **implemented in repo**. **Remaining for sprint close:** human execution of **QA-304-STREAM** (and optional preset carryover QA), update **`todo.md` checkboxes** for Sprint 2.4, **`VERIFICATION_CHECKLIST.md`** sign-off, **`GOTCHAS.md`** merge if needed.

---

## Sprint Folder & Artefacts

**All sprint artefacts MUST be stored in this sprint's folder:**

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_2_4/SPRINT_2_4_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_2_4/TEST_PLAN_2_4.md` | QA manual + automated checklist |
| Progress Report | `SPRINTS/Sprint_2_4/PROGRESS_REPORT.md` | Weekly / end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_2_4/MANUAL_TEST_REPORT.md` | QA results (Pass/Fail) |
| Verification Checklist | `SPRINTS/Sprint_2_4/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Security Sign-off | `SPRINTS/Sprint_2_4/SECURITY_SIGNOFF.md` | SEC-202 |
| Gotchas Log | `SPRINTS/Sprint_2_4/GOTCHAS.md` | Merge to `RESEARCH/GOTCHAS.md` when done |

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
2. **Set your Cursor title to the role name.** Update the Cursor session (composer/chat) title so it matches your assigned role exactly (e.g. **System Architect**, **Senior Engineer**, **UI Designer**). This keeps the session clearly identified with the role you have taken.
3. **Read the persona file** listed in the "Persona File" column
4. **Adopt that persona** for all remaining work

### Step 3: Become Your Role
- Embody the agent's identity, expertise, and responsibilities
- Follow the persona file's guidance and project references
- Rename the agent so that it shows the agent identity in the agent list

**If all roles show "In Progress" or "Complete", STOP. No work available.**

### Step 4: Update status
- While progressing your role, update the status per the Status Values defined below.

### Optional: One-shot role assumption (automated)
An agent can **read this task assignment, find unassigned roles, and create one Cursor command per available role**. When you run one of those commands in a chat, that chat becomes a **one-shot agent** for that role (it claims the role and adopts the persona for the rest of the conversation). To generate the commands: run the Cursor command **"Create One-Shot Assume-Role Commands for This Sprint"** (`.cursor/commands/create-assume-role-commands.md`). Optionally @-mention this Task Assignment file so the agent knows which sprint to use. The agent will create files like `.cursor/commands/assume-sprint-X-Y-<role-slug>.md`; run any of them to assume that role one-shot.

---

## Roles required for this sprint

| Role | Why required |
|------|--------------|
| System Architect | ARCH-501: event contract (`depth-progress`, payload, threading); ADR-002 addendum |
| Senior Engineer | BACK-205-STREAM, BACK-205-IPC: `python_bridge` + `generate_depth_map` + `src/lib/tauri.ts` types |
| UI Designer | UI-304: `listen` / `unlisten`, determinate progress bar, a11y |
| Security Specialist | SEC-202: model download SHA256 (gate before Sprint 2.5) |
| Junior Engineer 2D | JR2-1301–1303: preset tests (Sprint 2.3 carryover) |
| Quality Engineer | QA-304-STREAM; optional QA-1301–1303 carryover |
| Documentation Specialist | DOC-204: user/dev/architecture/release notes |

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | Complete | — | ARCH-501 | ADR-002 addendum in RESEARCH/architecture.md |
| Senior Engineer | `.agents/senior-engineer.md` | Complete | — | BACK-205-STREAM, BACK-205-IPC | See key files below |
| UI Designer | `.agents/ui-designer.md` | Complete | — | UI-304 | App.svelte + DepthProgressEvent |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Complete | — | JR2-1301, JR2-1302, JR2-1303 | Preset carryover |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Available | - | — | No 2.4 tasks |
| Senior Researcher | `.agents/researcher.md` | Available | - | — | Optional consult (model hashes) |
| Security Specialist | `.agents/security-specialist.md` | Complete | — | SEC-202 | SECURITY_SIGNOFF.md |
| Quality Engineer | `.agents/quality-engineer.md` | Available | - | QA-304-STREAM, QA-1301–1303 | **Claim to run manual cases & sign MANUAL_TEST_REPORT** |
| Documentation Specialist | `.agents/documentation-specialist.md` | Complete | — | DOC-204 | Docs updated for streaming |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References (Source of Truth)

- **Scope:** `todo.md` — Sprint 2.4 tasks and exit criteria  
- **PRD:** `prd.md` — F1.2 depth estimation progress; Phase 2 UX goals  
- **Architecture:** `RESEARCH/architecture.md` — ADR-002 (subprocess + ARCH-501 addendum), ADR-003 (SEC-202)  
- **Developer contract:** `docs/architecture.md` — `generate_depth_map`, progress protocol  
- **Threat model / security:** `docs/threat-model.md` §2.2, `docs/security-checklist.md` §2.2  
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`  
- **Tauri:** `RESEARCH/tauri.md` — v2 `Emitter`, `listen` / `unlisten`  
- **GOTCHAS:** `RESEARCH/GOTCHAS.md`

---

## As-built reference (implementers & QA)

| Area | Location | Behaviour |
|------|----------|-----------|
| Progress protocol | `python/python/depth_estimator.py` | Emits `PROGRESS n` and `STAGE name` on **stderr** |
| Bridge | `src-tauri/src/python_bridge.rs` | `run_depth_estimation_with_progress` + `ProgressCb`; stderr thread parses lines and invokes callback as they arrive |
| Tauri command | `src-tauri/src/lib.rs` | `generate_depth_map` takes `app_handle`, builds closure that `emit("depth-progress", DepthProgressPayload { percent, stage })` |
| Types | `src/lib/tauri.ts` | `DepthProgressEvent`; JSDoc on `generateDepthMap` |
| UI | `src/App.svelte` | `listen("depth-progress", …)` before `generateDepthMap`; `unlisten()` in `finally`; determinate bar from `progressPercent` |

**Sprint 2.4 exit (product):** User sees **increasing percentage** during a real (non-stub) depth run on a large image; no leaked listeners after completion.

---

## Sprint Progress Summary

| Section | Status | Completion |
|---------|--------|------------|
| ARCH-501 | Complete | 100% |
| BACK-205-STREAM / BACK-205-IPC | Complete | 100% |
| UI-304 | Complete | 100% |
| SEC-202 | Complete | 100% |
| JR2-1301–1303 (carryover) | Complete | 100% |
| DOC-204 | Complete | 100% |
| QA-304-STREAM | Pending human run | 0% until MANUAL_TEST_REPORT filled |
| QA-1301–1303 (optional carryover) | Pending human run | 0% until executed |
| Verification checklist / todo.md ticks | Pending | 0% |

**Overall Sprint Progress:** In Progress — **implementation complete; QA sign-off and doc hygiene remain**

---

## Task Breakdown

### ARCH-501: Async / real-time progress event contract

**Assigned Role:** System Architect  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** ARCH-501

**Dependencies:** Existing `generate_depth_map`, `run_depth_estimation` — complete.

**What to Do:** Document event name `"depth-progress"`, payload `{ percent: u8, stage?: String }` (camelCase JSON), sync Tauri command + progress callback (no `AppHandle` inside `python_bridge.rs`). Record in ADR-002 addendum.

**Reference Documents:** `RESEARCH/architecture.md` ADR-002 addendum (ARCH-501)

**Acceptance Criteria:**
- [x] Payload schema and threading documented  
- [x] Senior Engineer unblocked for BACK-205-STREAM  

**Completion Record:**
```
Status: Complete
Completed By: System Architect
Completed On: 2026-03-06
Notes: RESEARCH/architecture.md ADR-002 addendum; no new Tauri capability for listen-only events.
```

---

### BACK-205-STREAM: Stream Python stderr → Tauri events

**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-205-STREAM

**Dependencies:** ARCH-501 — complete.

**What to Do:** Wire stderr reader to invoke a progress callback per `PROGRESS`/`STAGE` line; in `lib.rs`, pass closure that emits `depth-progress`. Keep non-streaming path testable where applicable.

**Reference Documents:** `src-tauri/src/python_bridge.rs`, `src-tauri/src/lib.rs`

**Acceptance Criteria:**
- [x] Callback invoked as lines arrive (not only after process exit)  
- [x] `generate_depth_map` emits `depth-progress` during subprocess  
- [x] `cargo test` / `cargo clippy -D warnings` pass  

**Completion Record:**
```
Status: Complete
Completed By: Senior Engineer
Completed On: 2026-03-06
Notes: run_depth_estimation_with_progress + generate_depth_map(app_handle).
```

---

### BACK-205-IPC: Frontend types and API documentation

**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-205-IPC

**Dependencies:** ARCH-501 — complete.

**What to Do:** Export `DepthProgressEvent` (or equivalent) from `src/lib/tauri.ts`; document that callers should `listen` before `generateDepthMap`.

**Acceptance Criteria:**
- [x] Type exported; JSDoc on `generateDepthMap`  
- [x] `npm run build` and `npm test` pass  

**Completion Record:**
```
Status: Complete
Completed By: Senior Engineer
Completed On: 2026-03-06
```

---

### UI-304: Determinate progress bar

**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-304

**Dependencies:** BACK-205-STREAM, BACK-205-IPC — complete.

**What to Do:** Subscribe to `depth-progress`; update visible 0–100% bar; `unlisten()` in `finally`; handle slow first `PROGRESS` (e.g. min width / “Starting…”); basic a11y (`aria-valuenow` / min / max).

**Reference Documents:** `src/App.svelte`, `RESEARCH/tauri.md`

**Acceptance Criteria:**
- [x] Percentage visible during run  
- [x] No listener leak on success or error  
- [x] Build and tests pass  

**Completion Record:**
```
Status: Complete
Completed By: UI Designer
Completed On: 2026-03-06
```

---

### SEC-202: SHA256 verification for model downloads

**Assigned Role:** Security Specialist  
**Priority:** Critical (Phase 2 gate before Sprint 2.5)  
**Status:** [x] Complete  
**Task ID:** SEC-202

**Dependencies:** `python/python/model_downloader.py`, trusted hashes in repo.

**What to Do:** Implement verify-after-download against committed expected hashes; document in ADR-003 and threat model; file `SECURITY_SIGNOFF.md`.

**Acceptance Criteria:**
- [x] HTTPS + SHA256 outcome documented  
- [x] `SECURITY_SIGNOFF.md` present  

**Completion Record:**
```
Status: Complete
Completed By: Security Specialist
Completed On: 2026-03-06
Notes: SEC-202A — expected_model_hashes.json, verify_model_sha256, tests.
```

---

### JR2-1301 – JR2-1303: Preset tests (Sprint 2.3 carryover)

**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task IDs:** JR2-1301, JR2-1302, JR2-1303

**What to Do:** Rust preset round-trip / invalid JSON / schema version tests; Vitest preset API invoke-arg tests — per original sprint 2.3 acceptance.

**Acceptance Criteria:**
- [x] `cargo test` preset-related tests pass  
- [x] `npm test` passes  

**Completion Record:**
```
Status: Complete
Completed By: Junior Engineer 2D
Completed On: 2026-03-06
```

---

### DOC-204: Documentation for progress streaming

**Assigned Role:** Documentation Specialist  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** DOC-204

**What to Do:** Update `docs/architecture.md`, `docs/developer-guide.md`, `docs/user-guide.md`, `docs/release-notes-draft.md` for `depth-progress` and determinate UI.

**Acceptance Criteria:**
- [x] All four docs touched as specified in sprint notes  

**Completion Record:**
```
Status: Complete
Completed By: Documentation Specialist
Completed On: 2026-03-06
```

---

### QA-304-STREAM: Manual — real depth run shows increasing %

**Assigned Role:** Quality Engineer  
**Priority:** Critical  
**Status:** [ ] Not Started  
**Task ID:** QA-304-STREAM

**Dependencies:** UI-304, BACK-205-STREAM — complete. Requires **real model** (not `SP3D_USE_STUB=1`).

**What to Do:** Execute `SPRINTS/Sprint_2_4/TEST_PLAN_2_4.md` §1. Record Pass/Fail and tester id in `MANUAL_TEST_REPORT.md`. Minimum bar: large image run shows **≥3 distinct** percentage updates; repeat run does not show stale progress.

**Acceptance Criteria:**
- [ ] Cases executed; results in MANUAL_TEST_REPORT.md  
- [ ] Failure → GitHub issue with repro  

**Completion Record:**
```
Status: [ ] Complete
Completed By:
Completed On:
Notes:
```

---

### QA-1301 – QA-1303: Preset manual QA (optional carryover)

**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Not Started  
**Task IDs:** QA-1301, QA-1302, QA-1303

**What to Do:** Run `TEST_PLAN_2_4.md` §2 (or `SPRINTS/Sprint_2_3/TEST_PLAN_2_3.md`); record in `MANUAL_TEST_REPORT.md`.

**Acceptance Criteria:**
- [ ] All steps executed or explicitly waived by Architect  

**Completion Record:**
```
Status: [ ] Complete
Completed By:
Completed On:
Notes:
```

---

## Subtask Allocation (for multi-role tasks)

| Sub-task | Role | Owner | Status |
|----------|------|-------|--------|
| Event contract | System Architect | — | [x] |
| Bridge + emit | Senior Engineer | — | [x] |
| tauri.ts types | Senior Engineer | — | [x] |
| App.svelte bar + listen | UI Designer | — | [x] |
| SEC-202 | Security Specialist | — | [x] |
| JR2 preset tests | Junior Engineer 2D | — | [x] |
| Docs | Documentation Specialist | — | [x] |
| Manual streaming QA | Quality Engineer | *(claim)* | [ ] |
| Preset manual QA | Quality Engineer | *(claim)* | [ ] |

---

## Success Criteria for Sprint

- [x] `depth-progress` emitted during depth subprocess (implementation)  
- [x] Frontend shows determinate percentage and unlistens on completion  
- [x] SEC-202 satisfied (see SECURITY_SIGNOFF.md)  
- [x] Preset automated tests (JR2-1301–1303) in CI  
- [ ] **QA-304-STREAM** passed by named tester (or documented waiver)  
- [ ] **todo.md** Sprint 2.4 checkboxes updated to match reality  
- [ ] **VERIFICATION_CHECKLIST.md** signed off  
- [ ] **GOTCHAS.md** merged to RESEARCH if non-empty  
- [x] `cargo test`, `cargo clippy -D warnings`, `npm test`, `npm run build` green (last verified 2026-04-05: 194 Rust tests passed, 6 ignored; 76 frontend tests passed)

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| None for engineering | — | — |
| Human tester + real model for QA-304-STREAM | Quality Engineer / PM | 🟡 Until manual run |

---

## Quality Metrics

| Metric | Target | Actual (2026-04-05) |
|--------|--------|---------------------|
| `cargo test` (from `src-tauri/`) | PASS | 194 passed, 6 ignored |
| `cargo clippy -- -D warnings` | 0 warnings | Run locally before PR |
| `cargo fmt --check` | PASS | Run locally before PR |
| `npm run build` | PASS | Run locally before PR |
| `npm test` | PASS | 76 passed |

---

## Progress Log (Handover Notes)

```
### 2026-04-05 — Documentation Specialist (task assignment refresh)
Rewrote SPRINT_2_4_Task_Assignment.md to match SPRINT_TASKING_TEMPLATE.md structure, fixed prior inconsistencies (backend marked "not started" vs tasks complete), added as-built file table, split core vs envelope scope, reset QE row to Available for remaining manual QA and checklist/todo hygiene.
```

---

## Required Reading (After Claiming Role)

1. Persona file (Role Assignment)  
2. `todo.md` — Sprint 2.4  
3. `RESEARCH/architecture.md` — ADR-002 addendum, ADR-003  
4. `RESEARCH/AI_DEVELOPMENT_GUIDE.md`  
5. `SPRINTS/Sprint_2_4/TEST_PLAN_2_4.md` — if Quality Engineer  
6. `RESEARCH/GOTCHAS.md`

---

## Next sprint (for planning)

After Sprint 2.4 is signed off: **Sprint 2.5 — Masking** (`SPRINTS/Sprint_2_5/`). **Do not start Sprint 2.6** until Sprint 2.5 P0 (mask visible effect) is fixed and verified per `todo.md` Phase 2 banner.

---

**Document Version:** 2.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Status:** Fully defined — **awaiting QA sign-off and todo/checklist closure**
