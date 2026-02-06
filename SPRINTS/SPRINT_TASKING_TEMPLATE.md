# Sprint Task Assignment Template

**Use this template when creating sprint tasking from `todo.md`.**  
Copy to `SPRINTS/Sprint_X_Y/SPRINT_X_Y_Task_Assignment.md` and populate.

---

## Sprint X.Y: [Sprint Name]

**Sprint Duration:** 2 weeks (Weeks X–Y)  
**Sprint Goal:** [One-sentence objective from todo.md]  
**Target Release:** vX.Y.Z (if applicable)  
**Phase:** [1=MVP | 2=Enhanced UX | 3=Cross-Platform | 4=Production]  
**Source:** `todo.md` — Sprint X.Y  
**Last Updated:** [Date]

---

## Sprint Folder & Artefacts

**All sprint artefacts MUST be stored in this sprint's folder:**

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_X_Y/SPRINT_X_Y_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/TEST_PLAN_TEMPLATE.md` → copy to sprint folder as needed | QA test planning (manual + automated) |
| Progress Report | `SPRINTS/Sprint_X_Y/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_X_Y/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_X_Y/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Architect Approval | `SPRINTS/Sprint_X_Y/ARCHITECT_APPROVAL.md` | If required for phase gate |
| Security Sign-off | `SPRINTS/Sprint_X_Y/SECURITY_SIGNOFF.md` | If security review in sprint |
| Gotchas Log | `SPRINTS/Sprint_X_Y/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

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

### Step 4: Update status
- While progressing your role, update the status per the Status Values defined below.
---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | Available | - | [List task IDs] | Architecture decisions, approvals |
| Senior Engineer | `.agents/senior-engineer.md` | Available | - | [List task IDs] | Core implementation, Python bridge |
| UI Designer | `.agents/ui-designer.md` | Available | - | [List task IDs] | Frontend, 3D preview, layout |
| Senior Researcher (AI/ML) | `.agents/researcher.md` | Available | - | [List task IDs] | Depth estimation, RESEARCH updates |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Available | - | [List task IDs] | Image/depth handling |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Available | - | [List task IDs] | Mesh, STL/OBJ, Three.js |
| Security Specialist | `.agents/security-specialist.md` | Available | - | [List task IDs] | Security review (if in sprint) |
| Documentation Specialist | `.agents/documentation-specialist.md` | Available | - | [List task IDs] | Docs (if in sprint) |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References (Source of Truth)

- **Scope:** `prd.md` — Product requirements, tech stack, acceptance criteria
- **Sprint source:** `todo.md` — Sprint X.Y task list
- **Architecture:** `RESEARCH/architecture.md`, `docs/architecture.md`
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`
- **Technology:** `RESEARCH/` — See `RESEARCH/README.md` for index

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| [Phase or task group] | ⏳ Not Started / 🟡 In Progress / ✅ Complete | 0% |

**Overall Sprint Progress:** [ ] Not Started / [ ] In Progress / [ ] Complete

---

## Task Breakdown

*Populate from todo.md Sprint X.Y. Map each task to a Role. Use task IDs (e.g. ARCH-001, BACK-101, UI-101, AI-001).*

### Phase [N]: [Phase Name]

#### Task X.Y: [Task Name]
**Assigned Role:** [Role from table]  
**Priority:** Critical / High / Medium / Low  
**Status:** [ ] Not Started / [ ] In Progress / [x] Complete / [ ] Blocked  
**Task ID:** [e.g. BACK-101]

**Dependencies:**
- Task A.B: [Description] — Status: [ ]
- Task C.D: [Description] — Status: [x] Complete

**What to Do:**
- [Task description from todo.md]
- [Steps]

**Reference Documents:**
- `prd.md` §[relevant section]
- `RESEARCH/[relevant].md`

**Acceptance Criteria:**
- [ ] Criterion 1
- [ ] Criterion 2

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

*[Repeat Task X.Y block for each task in sprint]*

---

## Subtask Allocation (for multi-role tasks)

When a task requires multiple roles, allocate sub-tasks explicitly:

| Sub-task | Role | Owner | Status |
|----------|------|-------|--------|
| X.Y.1 | [Role] | [Agent when claimed] | [ ] |
| X.Y.2 | [Role] | [Agent when claimed] | [ ] |

---

## Success Criteria for Sprint

- [ ] All tasks complete per acceptance criteria
- [ ] Exit criteria from todo.md met
- [ ] No blocking issues
- [ ] Gotchas recorded in `SPRINTS/Sprint_X_Y/GOTCHAS.md` (merge to RESEARCH when done)
- [ ] Progress report filed

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| [Description] | [Role] | 🔴 Blocking / 🟡 In Progress |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | — |
| cargo clippy | 0 warnings | — |
| cargo fmt --check | PASS | — |
| npm run build | PASS | — |
| pytest | PASS | — |

---

## Progress Log (Handover Notes)

*Agents add handover notes when completing tasks that others depend on.*

```
### [Date] — [Role] (Task X.Y COMPLETED)
[What was delivered. Key files. Gotchas. Handover to whom.]
```

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **prd.md** — Acceptance criteria for your tasks
3. **todo.md** — Sprint X.Y full context
4. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
5. **RESEARCH/[relevant].md** — Technology guidance for your tasks
6. **RESEARCH/GOTCHAS.md** — Known pitfalls before debugging

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Status:** Ready for population from todo.md
