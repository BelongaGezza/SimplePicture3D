# Sprint 2.2 — Progress Report

**Sprint:** 2.2 — Undo/Redo, Curve Persistence, State ADR, Wireframe/Solid  
**Phase:** 2 (Enhanced UX)  
**Last Updated:** 2026-03-01

---

## Summary

Sprint 2.2 goal: Implement F2.4 Undo/Redo, persist curve control points in settings, author state management ADR (TD-01), and fix or remove Wireframe/Solid UI (TD-02). Optional: start macOS smoke tests (TD-05).

---

## Completion

| Area | Tasks | Status |
|------|--------|--------|
| Architecture | ARCH-401–404 | ✅ Complete |
| Backend undo/redo | BACK-1401–1404 | ✅ Complete |
| Curve persistence | CURVE-001 | ✅ Complete |
| UI | UI-1401–1404 | ✅ Complete (verified; UI-1404 sprint refs removed) |
| Tests | JR2-1401, JR2-1402 | ✅ Complete |
| QA | QA-1401–1402 | 🔄 In Progress (procedure & doc ready; automated gate pass; manual QA-1401 needs human tester) |

---

## UI Designer (2026-03-01)

- **UI-1401–1404:** Acceptance criteria verified. Undo/Redo toolbar buttons and Ctrl+Z / Ctrl+Y (Cmd on macOS) are wired; shortcuts documented in user-guide.md. Wireframe/Solid use MeshData.indices in Preview3D; overlay explains when mesh has no triangles. Internal sprint references removed from App.svelte (UI-1404).

## QE handover (Quality Engineer)

- **QA-1401:** Procedure and six manual cases in TEST_PLAN_2_2.md (§3.2) and MANUAL_TEST_REPORT.md. **Automated gate unblocked** (cargo test passes, including `undo::tests::push_clears_redo`). Manual cases require human tester to run app and exercise Undo/Redo UI; MANUAL_TEST_REPORT updated to "Ready for execution". VERIFICATION_CHECKLIST updated: cargo test, clippy, npm test all PASS.
- **QA-1402:** Complete. MACOS_SMOKE.md added with environment template, build steps, short smoke checklist, and status "Not yet run" with plan for Phase 3 / Sprint 3.1.

## Blockers / Deferred

- **~~`push_clears_redo`~~:** Resolved. Test passes (2026-03-01); automated gate clear.
- **JR2-1401, JR2-1402:** Complete (Rust tests for command execute/undo and history cap).
- **QA-1401 manual execution:** Procedure ready; requires human tester to run app and fill MANUAL_TEST_REPORT.md (Cases 1–6, regression/smoke).
- **QA-1402 macOS run:** Deferred (no macOS env this sprint). MACOS_SMOKE.md documents plan; status "Not yet run" with clear plan per TD-05.

---

## Junior Engineer 3D (support role, no 2.2-specific tasks)

- **Mesh pipeline verification:** Confirmed backend `depth_to_point_cloud` returns `MeshData.indices` for grid ≥2×2; Preview3D builds triangulated mesh and toggles Wireframe/Solid (MeshPhongMaterial / MeshBasicMaterial wireframe). Wireframe/Solid are functional for normal depth maps; overlay only shows when mesh has no triangles (edge case).
- **Tests:** `cargo test` — 151 passed, 6 ignored; mesh_generator and undo tests healthy.
- **Sprint GOTCHAS:** Added 3D/Wireframe verification note to `SPRINTS/Sprint_2_2/GOTCHAS.md`.

---

## Documentation Specialist (Sprint 2.2)

- **Progress Report:** Aligned with SPRINT_2_2_Task_Assignment.md (BACK-1401–1404, UI-1401–1404, CURVE-001, JR2-1401/JR2-1402 complete; QA-1401 procedure ready, manual execution requires human tester; automated gate pass).
- **User guide** (`docs/user-guide.md`): Undo/Redo (Phase 2), shortcuts, curve persistence, Wireframe/Solid — verified and current.
- **Developer guide** (`docs/developer-guide.md`): Tauri commands table includes `undo`, `redo`, `clear_history` and ADR-009/ADR-010 references — verified.
- **CHANGELOG.md:** 0.2.0-beta.1 (Phase 2 / Sprint 2.2) section present with undo/redo, curve persistence, TD-01/TD-02, docs notes.

---

## Next Steps

- **QE:** Human tester to run QA-1401 manual cases (TEST_PLAN_2_2.md §3.2), fill Actual result / Pass-Fail in MANUAL_TEST_REPORT.md, then sign off VERIFICATION_CHECKLIST when exit criteria met.
- After sprint: Sprint 2.3 (Presets) or 2.4 (Progress streaming) per todo.md.
