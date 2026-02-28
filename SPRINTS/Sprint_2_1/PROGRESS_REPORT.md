# Sprint 2.1 — Progress Report

**Sprint:** 2.1 — Advanced Depth Controls (Histogram & Curves)  
**Phase:** 2 (Enhanced UX)  
**Last Updated:** 2026-02-28

---

## Summary

Sprint 2.1 delivered power-user depth tools: depth histogram (BACK-1101), curve control points and application (BACK-1102, BACK-1103), and real-time preview integration (BACK-1104). Frontend: HistogramPanel (UI-1101, JR1-1101), CurvesTool with presets and reset (UI-1102–1104, JR1-1102, JR1-1103), and Advanced mode toggle (UI-1105). Phase 1 Exit Gate was executed (PHASE_1_EXIT_GATE.md in Sprint_1_12); decision: GO to Phase 2.

---

## Completion

| Area | Tasks | Status |
|------|--------|--------|
| Backend histogram + curves | BACK-1101–1104 | ✅ |
| Frontend HistogramPanel | UI-1101, JR1-1101 | ✅ |
| Frontend CurvesTool | UI-1102–1104, JR1-1102, JR1-1103 | ✅ |
| Advanced mode toggle | UI-1105 | ✅ |
| QA test plan | QA-1101–1103 | ✅ (TEST_PLAN_2_1.md) |
| Phase 1 Exit Gate | Go/No-Go | ✅ GO |

---

## Blockers / Deferred

- Curve control points not persisted in AppSettings (optional Phase 2).
- Manual performance run (QA-1103) and full manual test execution left to QA; automated Rust tests and frontend build pass.

---

## Next Steps

- Execute manual test procedures in TEST_PLAN_2_1.md.
- Proceed to Sprint 2.2 (Masking & Regional Adjustments) or iterate on 2.1 based on feedback.
