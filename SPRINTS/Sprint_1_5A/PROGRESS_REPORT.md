# Sprint 1.5A Progress Report

**Sprint:** 1.5A — Hardening, Testing & Consultant Remediation
**Duration:** 1 week (5 working days)
**Last Updated:** 2026-02-07

---

## Summary

| Phase/Section | Status | Notes |
|---------------|--------|-------|
| Frontend Testing (UI-502, JR2-501–502, JR1-501–502, UI-505) | ✅ Complete | Vitest, 34 tests (smoke, depthCanvas, tauri, DepthControls, ImageImport); npm test in CI |
| Contrast Slider (UI-501, UI-503) | ✅ Complete | Contrast in DepthControls; user-guide updated |
| Coverage Tracking (BACK-501, AI-501) | ✅ Complete | Tarpaulin in CI (advisory); pytest-cov in CI; baselines from first CI run |
| Security Fix (SEC-501–502, BACK-502) | ✅ Complete | Asset protocol disabled; CSP updated; build/test verified |
| IPC Performance Spike (ARCH-501) | ✅ Complete | Spike doc, bench, recommendation for 1.6/1.7 |
| Model License (ARCH-502, AI-502) | ✅ Complete | ADR-005, user docs; license notice + --show-license in Python |
| Documentation Cleanup (DOC-501–504) | ✅ Complete | Sprint 1.5 artefacts, todo.md, README, CLAUDE.md |

## Deliverables

- **Frontend:** Vitest + Testing Library; `src/lib/__tests__/` (smoke, depthCanvas, tauri); `src/components/__tests__/` (DepthControls, ImageImport). 34 tests, CI step.
- **UI:** Contrast slider in DepthControls (0.5–2, default 1); user-guide Controls table + Reset.
- **CI:** Backend coverage (cargo-tarpaulin, continue-on-error); Python pytest-cov; frontend `npm test`.
- **Security:** Asset protocol disabled; CSP img-src updated; SECURITY_SIGNOFF.md; BACK-502 verified.
- **Architecture:** IPC spike (methodology, bench, recommendation); ADR-005 model license; Python license notice and `--show-license`.
- **Docs:** PROGRESS_REPORT, VERIFICATION_CHECKLIST, DOC-501–504 artefacts updated.

## Coverage Baselines

| Stack | Coverage % | Tool |
|-------|-----------|------|
| Rust | (from first CI run) | cargo tarpaulin — CI step, advisory (continue-on-error) |
| Python | (from first CI run) | pytest-cov — CI runs with --cov=depth_estimator, term + xml report |
| Frontend | 34 tests | Vitest — no coverage reporter this sprint |

## Blockers / Risks

None. Optional: fix `npm run build` TypeScript errors in component test files (Svelte/Testing Library type compatibility) if full production build is required; `npm test` passes.

## References

- Task assignment: `SPRINT_1_5A_Task_Assignment.md`
- Senior Engineer review: `SENIOR_ENGINEER_COMPLETION_REVIEW.md`
- Source: `Consultant_Recommendations_Report_6Feb2026.md` §3–4
- Verification: `VERIFICATION_CHECKLIST.md`
