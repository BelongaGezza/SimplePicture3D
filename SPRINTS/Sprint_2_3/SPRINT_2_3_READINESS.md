# Sprint 2.3 Readiness — Architect & Senior Engineer Check

**Date:** 2026-03-02  
**Scope:** Confirm Sprint 2.3 (Presets & Templates) is planned and ready for the team.

---

## Summary

**Verdict: Ready for the team**, with pre-sprint gate items to be resolved before or in parallel with implementation.

- **todo.md** already contained Sprint 2.3 scope, tasks, exit criteria, and pre-sprint actions (Consultant_Review_1Mar2026 §8).
- **Gap:** No `SPRINTS/Sprint_2_3/` folder or task assignment existed, so agents could not claim roles or work from a single sprint document. This is now **resolved** — `SPRINT_2_3_Task_Assignment.md` has been created and populated from the template and todo.md.

---

## What Was in Place

| Item | Status |
|------|--------|
| Sprint goal and scope (todo.md §Sprint 2.3) | ✅ Clear: save/load/import/export presets, built-ins, PresetManager |
| PRD F2.3 acceptance criteria | ✅ Save/load, built-in presets (Portrait, Landscape, High Detail, Low Relief), import/export JSON, preset manager UI |
| Task list by role (Senior Engineer, UI, JR2, QA) | ✅ BACK-1301–1304, UI-1301–1304, JR2-1301–1303, QA-1301–1303 |
| Exit criteria | ✅ Four criteria in todo.md |
| Phase 2 sequencing | ✅ Consultant_Review_1Mar2026 §6: 2.3 ready to begin after 2.2 |
| Architecture context | ✅ ADR-010 (state management) in place; presets path `~/.simplepicture3d/presets/` in PRD and architecture.md |

---

## What Was Missing (Now Addressed)

| Item | Action taken |
|------|----------------|
| Sprint folder `SPRINTS/Sprint_2_3/` | Created with task assignment and this readiness doc |
| `SPRINT_2_3_Task_Assignment.md` | Created from `SPRINT_TASKING_TEMPLATE.md` and todo.md Sprint 2.3 |
| Role Assignment table with persona files and task ownership | Populated (Senior Engineer, UI Designer, Junior Engineer 2D, Quality Engineer) |
| Task breakdown with dependencies, acceptance criteria, completion records | All 14 tasks written (BACK-1301–1304, UI-1301–1304, JR2-1301–1303, QA-1301–1303) |
| Pre-sprint gate in one place | Added table at top of task assignment (6 items from todo.md / Consultant §8) |

---

## Pre-Sprint Gate (Must Be Addressed)

These do **not** block creating the sprint plan but should be resolved before or at sprint start so the team has a clear process and no open verification debt:

1. **QA-PROCESS** — Architect decides: manual QA blocking (sprint exit) or non-blocking (48h window, named tester). Apply from Sprint 2.3 onward.
2. **QA-1401-EXEC** — Quality Engineer runs QA-1401 manual cases for Sprint 2.2 sign-off (carry to 2.3 start).
3. **CURVE-UNDO-VERIFY** — Senior Engineer confirms curve mutations create `SetDepthParamsCommand` entries (required for presets + undo consistency).
4. **ADR-008-COMMIT** — Confirm ADR-008 winding order fix is committed in RESEARCH/architecture.md.
5. **DOC-CLEANUP** — Documentation Specialist: prd.md §F1.6 (stl_io), diagram labels (Svelte 4, subprocess).
6. **SEC-202** — SHA256 model checksum verification (before Sprint 2.4; can run in parallel with 2.3).

---

## Design Notes (No Blocker)

- **Preset schema:** BACK-1301 defines the JSON schema; it should align with `DepthAdjustmentParams`, `curveControlPoints`, and mesh params (e.g. `AppSettings`). Architecture already mentions versioned schema (prd.md F2.3).
- **Undo and presets:** Loading a preset should be treated as a state change. Per ADR-010, whether “load preset” pushes an undoable command is a product decision; the current undo stack is depth/curve-oriented and can accommodate “restore snapshot” style if desired.
- **TD-08 (lib.rs coverage):** Consultant recommends addressing in Sprint 2.3 or 2.4. Preset commands will add more Tauri handlers; consider extracting preset serialization/validation into testable functions to avoid widening the coverage gap.

---

## Recommendation

- **Start Sprint 2.3** once the pre-sprint gate table in `SPRINT_2_3_Task_Assignment.md` is reviewed and items 1–5 are either done or explicitly accepted as non-blocking.
- **SEC-202** can run in parallel; it is required before Sprint 2.4, not before 2.3 implementation.
- **Remaining artefacts** (TEST_PLAN_2_3.md, VERIFICATION_CHECKLIST.md, PROGRESS_REPORT.md, MANUAL_TEST_REPORT.md, GOTCHAS.md) can be added by the team during the sprint per the tasking template.
