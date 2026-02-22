# Sprint 1.11 — Test Plan

**Sprint:** 1.11  
**Owner:** Quality Engineer (session-qa-20260222)  
**Last Updated:** 2026-02-22

---

## 1. E2E Approach (QA-1001)

**Decision:** **Repeatable manual checklist** for Phase 1 MVP; automated E2E (e.g. Playwright) deferred to Phase 2.

**Rationale:**
- Tauri desktop app: E2E would require Tauri-specific test harness (e.g. WebDriver + native window) or full `tauri dev` + UI automation; not yet in place.
- Happy-path coverage is achieved via:
  - **Unit/integration:** `cargo test` (Rust), `npm test` (Vitest), `pytest` (Python stub).
  - **Manual:** Documented full-workflow checklist and target-dimensions verification in MANUAL_TEST_REPORT.md.
- Phase 2 can add Playwright or Tauri E2E when test infrastructure is prioritized.

**Documented in:** This file (§2), MANUAL_TEST_REPORT.md (execution), CLAUDE.md Testing section (commands).

---

## 2. Test Scope

| Area | Type | Location / Tool |
|------|------|-----------------|
| Rust backend (lib, mesh, settings, file_io, depth_adjust) | Unit / integration | `cargo test --manifest-path src-tauri/Cargo.toml --lib` |
| Frontend (Svelte, Tauri IPC mocks) | Unit | `npm test` (Vitest) |
| Python depth estimator | Unit (stub mode) | `SP3D_USE_STUB=1 python -m pytest python/ -v` |
| Full workflow (load → depth → adjust → export) | Manual | MANUAL_TEST_REPORT.md § Full workflow |
| Target dimensions (50×70 mm export) | Manual | MANUAL_TEST_REPORT.md § Target dimensions |
| Regression (image load, depth, mesh, preview, export, settings, model wizard) | Manual checklist | MANUAL_TEST_REPORT.md § Regression |
| Performance (PRD §7.1) | Manual / benchmark | MANUAL_TEST_REPORT.md § Performance |

---

## 3. Acceptance Criteria (Sprint 1.11)

- [x] E2E approach chosen and documented (manual checklist for Phase 1).
- [x] Happy-path coverage: unit/integration tests + repeatable manual procedure.
- [x] TEST_PLAN_1_11.md and MANUAL_TEST_REPORT.md in `SPRINTS/Sprint_1_11/`.

---

## 4. References

- `todo.md` — Testing strategy (E2E deferred to Sprint 1.11)
- `prd.md` §7.1 — Performance targets
- `RESEARCH/architecture.md` — ADR-009 target dimensions
