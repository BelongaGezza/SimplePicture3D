# Sprint 2.5: Masking & Regional Adjustments

**Sprint Duration:** 2 weeks (10 working days)  
**Sprint Goal:** Enable selective depth adjustments via masking (brush, eraser, selection, overlay, feathering) with backend-authoritative mask state and undo/redo integration (depends on Sprint 2.2).  
**Target Release:** v0.5.0-beta (masking)  
**Phase:** 2 — Enhanced UX  
**Source:** `todo.md` — Sprint 2.5; `RESEARCH/architecture.md` ADR-010 / ARCH-502  
**Last Updated:** 2026-04-05

---

## Sprint status (executive summary)

| State | Detail |
|-------|--------|
| **Engineering** | ARCH-502, BACK-1201–1203, UI-1201–1204, JR1-1201–1203 are **implemented**; automated gate **PASS** (see Quality Metrics). |
| **Product acceptance** | **BLOCKED** — Manual **QA-1201 FAILED** (2026-03-14): brush, mask overlay, and depth isolation reported **non-functional** in the running app. **QA-1202 / QA-1203** deferred until P0 fixed. |
| **Sprint close** | **DEFERRED** per `VERIFICATION_CHECKLIST.md` — do **not** start Sprint 2.6 until P0 is fixed and Cases 1–3 pass (or waived by Architect). |
| **Evidence** | `SPRINTS/Sprint_2_5/MANUAL_TEST_REPORT.md`, `SPRINTS/Sprint_2_5/VERIFICATION_CHECKLIST.md`, `todo.md` Phase 2 banner. |

**Next actions (in order):** (1) **P0 remediation** — trace mask paint → IPC → `AppState.mask` → overlay + `apply_adjustments_with_mask` path; add tests or logging as needed. (2) Human re-runs **TEST_PLAN_2_5.md** §3.2 Cases 1–3. (3) Update **`todo.md`** Sprint 2.5 checkboxes and sign **`VERIFICATION_CHECKLIST.md`**.

---

## Scope layers

| Layer | Contents |
|-------|----------|
| **Core (`todo.md` §Sprint 2.5)** | ARCH-502, BACK-1201–1203, UI-1201–1204, JR1-1201–1203, QA-1201–1203 |
| **P0 close-out (same sprint folder)** | End-to-end mask **visible effect** + depth isolation + re-QA — not a new sprint number until closed |

---

## Sprint Folder & Artefacts

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_2_5/SPRINT_2_5_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_2_5/TEST_PLAN_2_5.md` | Manual + automated planning |
| Progress Report | `SPRINTS/Sprint_2_5/PROGRESS_REPORT.md` | Sprint status |
| Manual Test Report | `SPRINTS/Sprint_2_5/MANUAL_TEST_REPORT.md` | QA-1201–1203 results (**Case 1 FAIL logged**) |
| Verification Checklist | `SPRINTS/Sprint_2_5/VERIFICATION_CHECKLIST.md` | Sign-off (**DEFERRED** until P0) |
| Gotchas Log | `SPRINTS/Sprint_2_5/GOTCHAS.md` | Merge to `RESEARCH/GOTCHAS.md` when done |

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

**If all roles show "In Progress" or "Complete", STOP. No work available** — *unless you are picking up **P0-MASK**; then claim **Senior Engineer** or **UI Designer** (see P0 section).*

### Step 4: Update status
- While progressing your role, update the status per the Status Values defined below.

### Optional: One-shot role assumption (automated)
An agent can **read this task assignment, find unassigned roles, and create one Cursor command per available role**. When you run one of those commands in a chat, that chat becomes a **one-shot agent** for that role (it claims the role and adopts the persona for the rest of the conversation). To generate the commands: run the Cursor command **"Create One-Shot Assume-Role Commands for This Sprint"** (`.cursor/commands/create-assume-role-commands.md`). Optionally @-mention this Task Assignment file so the agent knows which sprint to use. The agent will create files like `.cursor/commands/assume-sprint-X-Y-<role-slug>.md`; run any of them to assume that role one-shot.

---

## Roles required for this sprint

| Role | Why required |
|------|--------------|
| System Architect | ARCH-502: mask command contract (ADR-010 extension) — **delivered** |
| Senior Engineer | BACK-1201–1203; **P0-MASK** backend/IPC/pipeline debugging |
| UI Designer | UI-1201–1204; **P0-MASK** overlay / coordinates / `refreshMask` wiring |
| Junior Engineer 2D | JR1-1201–1203: stroke smoothing, selection, mask save/load — **delivered** |
| Quality Engineer | QA-1201–1203: manual Cases 1–3; **re-run after P0 fix** |

---

## P0 remediation (blocks sprint close)

**ID:** P0-MASK (informal; link GitHub issue when filed)  
**Symptom:** Manual test: brush does not visibly change mask overlay; depth adjustments do not appear isolated to masked region (`MANUAL_TEST_REPORT.md`, Case 1, 2026-03-14).  
**Primary owners:** **Senior Engineer** + **UI Designer** (pair or sequence: IPC/state vs canvas/UI).  
**Suggested investigation anchors:**

| Area | Files / topics |
|------|----------------|
| Backend state | `src-tauri/src/lib.rs` — `set_mask_region`, `set_mask`, `get_mask`; `AppState.mask`; mask cleared on `generate_depth_map`? |
| Adjusted depth | `apply_adjustments_with_mask` — used on all paths (`get_depth_map`, `get_mesh_data`, export, histogram)? |
| Frontend | `src/App.svelte` — `refreshMask`, `handleMaskPaint`, `onMaskPaint` props; `DepthMapPreview.svelte` — coordinate mapping, overlay draw |
| IPC | `src/lib/tauri.ts` — `getMask`, `setMaskRegion`, invoke arg names |

**Exit for P0:** Case 1 **Pass** in `MANUAL_TEST_REPORT.md`; then Cases 2–3 executable.

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | In Progress | 2026-04-05 — Cursor (this chat) | ARCH-502 ✓; P0 exit / verification / waiver criteria | Re-engaged for sprint close: P0-MASK exit, TEST_PLAN alignment, VERIFICATION_CHECKLIST |
| Senior Engineer | `.agents/senior-engineer.md` | In Progress | 2026-04-05 — Cursor session | BACK-1201–1203, **P0-MASK** | Claim for mask pipeline / IPC / `apply_adjustments_with_mask` |
| UI Designer | `.agents/ui-designer.md` | In Progress | 2026-04-05 — Cursor (this chat) | UI-1201–1204, **P0-MASK** | Claim for overlay, painting, App wiring |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Complete | — | JR1-1201–1203 | Utilities + MaskingTools integration delivered |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | In Progress | 2026-04-05 — Cursor (this chat) | P0-MASK support: mesh/preview path if mask affects `get_mesh_data` / Three.js | No dedicated 2.5 IDs — pair with Senior Engineer on depth→mesh correctness |
| Senior Researcher | `.agents/researcher.md` | In Progress | 2026-04-05 — Cursor (composer) | — | No dedicated 2.5 task IDs — support P0: RESEARCH/architecture vs as-built, GOTCHAS, doc drift |
| Security Specialist | `.agents/security-specialist.md` | In Progress | 2026-04-05 — Cursor (this chat) | Sprint 2.5 advisory: mask IPC / input validation during P0 | No dedicated 2.5 tasks — support P0 review, dependency audits |
| Quality Engineer | `.agents/quality-engineer.md` | In Progress | 2026-04-05 — Cursor (this chat) | QA-1201–1203 | Re-run manual plan after P0 fix |
| Documentation Specialist | `.agents/documentation-specialist.md` | In Progress | 2026-04-05 — Cursor (this chat) | Optional post-P0 | User-guide masking note after P0; maintain sprint docs alignment |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References (Source of Truth)

- **Scope:** `todo.md` — Sprint 2.5; Phase 2 “stuck at 2.5” banner  
- **PRD:** `prd.md` — Phase 2 masking / regional adjustments  
- **Architecture:** `RESEARCH/architecture.md` — ADR-010, ARCH-502 (mask commands, undo stack)  
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`  
- **Consultant context:** `Consultant_Review_1Mar2026.md` §4.5, §6  
- **GOTCHAS:** `RESEARCH/GOTCHAS.md`, `SPRINTS/Sprint_2_5/GOTCHAS.md`

---

## As-built reference (implementers)

| Area | Location | Notes |
|------|----------|--------|
| Mask bitmap | `src-tauri/src/mask.rs` | Packed bits, `to_soft_mask`, feather |
| Undo | `src-tauri/src/undo.rs` | `SetMaskCommand`, `UndoableCommand` |
| Depth + mask | `src-tauri/src/lib.rs` | `apply_adjustments_with_mask`; wired to depth/mesh/export/histogram |
| IPC | `src-tauri/src/lib.rs` | `get_mask`, `set_mask_region`, `clear_mask`, `set_mask`, save/load mask paths |
| UI tools | `src/components/MaskingTools.svelte` | Brush, eraser, select, feather, save/load |
| Preview / paint | `src/components/DepthMapPreview.svelte` | Pointer handling, overlay canvas |
| App glue | `src/App.svelte` | Mask state refresh, paint handler, tool props |
| Frontend API | `src/lib/tauri.ts` | Mask-related invokes |

---

## Sprint Progress Summary

| Section | Status | Completion |
|---------|--------|------------|
| ARCH-502 | Complete | 100% |
| BACK-1201 – BACK-1203 | Complete (code) | 100% — **acceptance blocked by P0** |
| UI-1201 – UI-1204 | Complete (code) | 100% — **acceptance blocked by P0** |
| JR1-1201 – JR1-1203 | Complete | 100% |
| P0-MASK | **Open** | 0% until overlay/isolation work |
| QA-1201 | **Fail** (logged) | Re-run after P0 |
| QA-1202, QA-1203 | Blocked | After P0 |
| Verification / todo.md ticks | Deferred | After QA pass |

**Overall Sprint Progress:** **Blocked** — implementation landed; **product verification failed**; remediation required.

---

## Task Breakdown

### ARCH-502: Mask state and command contract

**Assigned Role:** System Architect  
**Priority:** Critical (gate)  
**Status:** [x] Complete  
**Task ID:** ARCH-502

**Dependencies:** ADR-010, Sprint 2.2 undo — complete.

**What to Do:** Document backend-authoritative mask, `SetMaskCommand`, heterogeneous undo stack, IPC contract.

**Reference Documents:** `RESEARCH/architecture.md` § ARCH-502

**Acceptance Criteria:**
- [x] Documented in RESEARCH/architecture.md  
- [x] BACK-1201 / UI-1201 unblocked at time of implementation  

**Completion Record:**
```
Status: Complete
Completed By: System Architect / Senior Engineer (2026-03-07)
Notes: ADR-010 extension; get_mask / set_mask_region / clear_mask / set_mask.
```

---

### BACK-1201: Mask data structure and IPC

**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete (code)  
**Task ID:** BACK-1201

**Dependencies:** ARCH-502 — complete.

**What to Do:** `MaskBitmap`, `AppState.mask`, Tauri commands, dimension validation, clear on new depth.

**Acceptance Criteria:**
- [x] Mask stored; IPC implemented  
- [x] `cargo test` / clippy pass  

**Completion Record:**
```
Status: Complete (code)
Completed By: Senior Engineer (2026-03-07)
Notes: mask.rs, undo integration.
```

---

### BACK-1202: Adjustments only in masked regions

**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete (code)  
**Task ID:** BACK-1202

**Dependencies:** BACK-1201 — complete.

**What to Do:** `apply_adjustments_with_mask` on depth/mesh/export/histogram paths.

**Acceptance Criteria:**
- [x] Unit tests (no mask, empty mask, single pixel)  
- [ ] **End-user isolation** — blocked until P0-MASK resolved  

**Completion Record:**
```
Status: Complete (code)
Completed By: Senior Engineer (2026-03-08)
```

---

### BACK-1203: Feathering (soft edges)

**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete (code)  
**Task ID:** BACK-1203

**Dependencies:** BACK-1201, BACK-1202 — complete.

**What to Do:** `to_soft_mask`, `feather_radius_px`, blend adjusted vs original.

**Acceptance Criteria:**
- [x] Tests / clippy  
- [ ] **QA-1202** — after P0  

**Completion Record:**
```
Status: Complete (code)
Completed By: Senior Engineer (2026-03-08)
```

---

### UI-1201 – UI-1204: MaskingTools, canvas, overlay, brush controls

**Assigned Role:** UI Designer  
**Priority:** Critical / High  
**Status:** [x] Complete (code)  
**Task IDs:** UI-1201, UI-1202, UI-1203, UI-1204

**Dependencies:** BACK-1201, ARCH-502 — complete.

**What to Do:** Tools, painting on depth preview, semi-transparent overlay, brush size / hardness.

**Acceptance Criteria:**
- [x] Build + Vitest  
- [ ] **Visible mask + isolation in app** — P0-MASK  

**Completion Record:**
```
Status: Complete (code)
Completed By: UI Designer (2026-03-08)
```

---

### JR1-1201 – JR1-1203: Smoothing, selection, mask save/load

**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task IDs:** JR1-1201, JR1-1202, JR1-1203

**Dependencies:** UI-1201–1202, BACK-1201 — complete.

**What to Do:** `maskStroke` / selection helpers / save_mask_to_path, load_mask_from_path wiring.

**Acceptance Criteria:**
- [x] Tests in `src/lib/__tests__/`  
- [x] Backend save/load  

**Completion Record:**
```
Status: Complete
Completed By: Junior Engineer 2D (2026-03-08)
Notes (2026-04-05): JR1-1201 — `interpolateBrushStroke` (`maskStroke.ts`) wired into DepthMapPreview
paint path; App batches `set_mask_region` + single `refreshMask` per stroke segment. JR1-1202/1203
unchanged (selectionTools, MaskingTools save/load, backend JSON) — verified in tree.
```

---

### QA-1201: Manual — paint mask, adjust depth, verify isolation

**Assigned Role:** Quality Engineer  
**Priority:** Critical  
**Status:** [ ] **Fail (2026-03-14)** — **re-run after P0-MASK**  
**Task ID:** QA-1201

**Dependencies:** Full stack — code present; **P0 fix required**.

**What to Do:** Execute `TEST_PLAN_2_5.md` §3.2 Case 1; record in `MANUAL_TEST_REPORT.md`.

**Acceptance Criteria:**
- [ ] Pass after fix (masked region changes; unmasked unchanged)  

**Completion Record:**
```
Status: Fail (historical 2026-03-14); pending re-run
Completed By: Human tester
Notes: P0 — no visible mask effect; see MANUAL_TEST_REPORT.md
```

---

### QA-1202: Manual — feathering

**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Blocked until P0  
**Task ID:** QA-1202

**What to Do:** TEST_PLAN_2_5.md Case 2.

**Acceptance Criteria:**
- [ ] Soft edges verified  

---

### QA-1203: Manual — undo/redo with masking

**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [ ] Blocked until P0  
**Task ID:** QA-1203

**What to Do:** TEST_PLAN_2_5.md Case 3.

**Acceptance Criteria:**
- [ ] Mask + depth history consistent  

---

### P0-MASK: End-to-end mask effect and isolation

**Assigned Role:** Senior Engineer + UI Designer  
**Priority:** **P0**  
**Status:** [ ] Not Started  
**Task ID:** P0-MASK *(informal)*

**Dependencies:** None — replaces false “complete” for **user-visible** behaviour.

**What to Do:** Reproduce Case 1; fix root cause (state not updating, wrong coordinates, overlay not reading mask, wrong branch in depth pipeline, etc.); add regression test if feasible.

**Acceptance Criteria:**
- [ ] QA-1201 passes on human re-test  
- [ ] Optional: automated test or instrumented log to prevent regression  

**Completion Record:**
```
Status: [ ] Complete
Completed By:
Completed On:
Notes:
```

---

## Subtask Allocation

| Sub-task | Role | Status |
|----------|------|--------|
| ARCH-502 | System Architect | [x] |
| BACK-1201 – BACK-1203 | Senior Engineer | [x] code |
| UI-1201 – UI-1204 | UI Designer | [x] code |
| JR1-1201 – JR1-1203 | Junior Engineer 2D | [x] |
| P0-MASK | Senior Engineer + UI Designer | [ ] |
| QA-1201 – QA-1203 | Quality Engineer | [ ] after P0 |

---

## Success Criteria for Sprint

- [x] ARCH-502 complete  
- [x] Implementation tasks coded (BACK, UI, JR1)  
- [x] Automated gate green  
- [ ] **P0-MASK resolved** — mask visible; depth isolation works in app  
- [ ] **QA-1201 – QA-1203** Pass (or Architect waiver with ticket)  
- [ ] `todo.md` Sprint 2.5 checkboxes updated  
- [ ] `VERIFICATION_CHECKLIST.md` — **CLOSED** (not DEFERRED)  
- [ ] `GOTCHAS.md` merged if new findings  

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| **P0 — mask has no visible effect / no isolation** | Senior Engineer + UI Designer | 🔴 Blocking sprint close |
| Human re-test after fix | Quality Engineer | 🟡 After P0 |

---

## Quality Metrics

| Metric | Target | Actual (2026-04-05) |
|--------|--------|---------------------|
| `cargo test --lib` (from `src-tauri/`) | PASS | 194 passed, 6 ignored |
| `cargo clippy -- -D warnings` | 0 warnings | Run before PR |
| `cargo fmt --check` | PASS | Run before PR |
| `npm run build` | PASS | Run before PR |
| `npm test` | PASS | 76 passed |

---

## Progress Log (Handover Notes)

```
### 2026-04-05 — Role claim (System Architect)
System Architect claimed (this Cursor chat) for Sprint 2.5 close-out: P0-MASK exit criteria, `VERIFICATION_CHECKLIST.md` / `TEST_PLAN_2_5.md` consistency with as-built, Architect waiver path if needed; ARCH-502 already delivered.

### 2026-04-05 — JR1 verification + JR1-1201 wiring (Junior Engineer 2D scope)
Reviewed JR1-1201–1203 against repo: selection + save/load were integrated; stroke interpolation existed only in `maskStroke.ts` tests — integrated into `DepthMapPreview.svelte` + batched `handleMaskPaint` in `App.svelte`. `todo.md` Sprint 2.5 JR1 checkboxes aligned.

### 2026-04-05 — Role claim (Junior Engineer 3D)
Junior Engineer 3D claimed (this Cursor chat) for Sprint 2.5 support: verify `get_mesh_data` / export paths use masked-adjusted depth consistently with preview; assist P0-MASK if failure is mesh-side rather than overlay-only; no standalone 2.5 task IDs.

### 2026-04-05 — Role claim (Senior Researcher)
Senior Researcher claimed (Cursor composer) for Sprint 2.5 support: keep RESEARCH/architecture and mask-related docs aligned with as-built; capture P0 findings in RESEARCH/GOTCHAS.md or sprint GOTCHAS; no standalone 2.5 task IDs.

### 2026-04-05 — Role claim (Security Specialist)
Security Specialist claimed (this Cursor chat) for advisory work during P0-MASK: Tauri IPC / mask command input validation, path handling on mask save/load, dependency audit hygiene; no standalone 2.5 task IDs.

### 2026-04-05 — Role claim (Documentation Specialist)
Documentation Specialist claimed (this Cursor chat) for optional post-P0 work: user-guide masking section in `docs/user-guide.md`; keep sprint artefacts (TEST_PLAN, MANUAL_TEST_REPORT pointers) aligned with as-built UI when P0 closes.

### 2026-04-05 — Role claim (Quality Engineer)
Quality Engineer claimed (this Cursor chat) for QA-1201–1203: maintain TEST_PLAN_2_5 alignment, prep re-run after P0-MASK; update MANUAL_TEST_REPORT / VERIFICATION_CHECKLIST when Cases 1–3 execute.

### 2026-04-05 — Role claim (UI Designer)
UI Designer claimed (this Cursor chat) for P0-MASK: overlay, coordinate mapping, `refreshMask` / `handleMaskPaint` wiring per remediation section.

### 2026-04-05 — Role claim
Senior Engineer claimed (Cursor session) for P0-MASK / backend–IPC–pipeline work; see Role Assignment table.

### 2026-04-05 — Task assignment refresh
Rewrote SPRINT_2_5_Task_Assignment.md to match SPRINT_TASKING_TEMPLATE.md: full role-claim block, scope layers, as-built file table, honest blocked status aligned with MANUAL_TEST_REPORT (Case 1 FAIL) and VERIFICATION_CHECKLIST (DEFERRED). Added P0-MASK remediation section; reset Senior Engineer, UI Designer, and QE to Available for close-out work. Fixed footer (removed stale “Sprint 2.4 complete” reference). Quality metrics re-verified.
```

### Historical — implementation phase (2026-03-07 — 2026-03-08)

- **ARCH-502 / BACK-1201:** ADR-010 extension; `mask.rs`, `SetMaskCommand`, `UndoableCommand`, `get_mask` / `set_mask_region` / `clear_mask` / `set_mask`; mask cleared on `generate_depth_map`.
- **BACK-1202 / BACK-1203:** `apply_adjustments_with_mask` wired to depth, mesh, export, histogram; `to_soft_mask` + `feather_radius_px`; unit tests updated.
- **UI-1201–1204:** `MaskingTools`, `DepthMapPreview` paint + overlay, brush size/hardness.
- **JR1-1201–1203:** `maskStroke` / selection / mask save-load JSON; Vitest coverage.
- **QE:** Automated gate PASS; manual Case 1 later **FAIL** (2026-03-14) — P0 logged in MANUAL_TEST_REPORT / VERIFICATION_CHECKLIST.

---

## Required Reading (After Claiming Role)

1. Persona file  
2. `todo.md` — Sprint 2.5 + Phase 2 banner  
3. `RESEARCH/architecture.md` — ADR-010 / ARCH-502  
4. `SPRINTS/Sprint_2_5/MANUAL_TEST_REPORT.md` — Case 1 failure detail  
5. `SPRINTS/Sprint_2_5/TEST_PLAN_2_5.md`  
6. `RESEARCH/AI_DEVELOPMENT_GUIDE.md`  
7. `RESEARCH/GOTCHAS.md`

---

## Next sprint (planning rule)

**Do not start Sprint 2.6** until Sprint 2.5 is **closed** on `VERIFICATION_CHECKLIST.md` (Cases 1–3 pass or explicit Architect decision). See `todo.md` Phase 2 “Next steps (Sprint 2.5)”.

---

**Document Version:** 2.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Status:** Fully defined — **blocked on P0-MASK**; remediation + QA re-run required for close
