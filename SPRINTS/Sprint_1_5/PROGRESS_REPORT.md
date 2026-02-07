# Sprint 1.5 Progress Report

**Sprint:** 1.5 — Manual Depth Adjustments  
**Duration:** 2 weeks (10 working days)  
**Last Updated:** 2026-02-07

---

## Summary

Sprint 1.5 is **functionally complete** (commit `70bbe45`, 2026-02-06). Depth adjustment pipeline (backend + frontend) works. Hardening and contrast-slider gaps are carried to **Sprint 1.5A**.

| Phase | Status | Notes |
|-------|--------|-------|
| Backend (BACK-401–405) | ✅ Complete | Depth adjustment logic, apply transforms, invert, range, cache original; contrast supported in backend |
| UI (UI-401–405) | ✅ Complete | DepthControls, sliders (brightness, gamma, depth min/max), Invert, real-time preview, Reset |
| Junior 2D (JR2-401–403) | ✅ Complete | Unit tests, boundary tests, benchmark |
| Junior 3D (JR1-401–404) | ✅ Complete | Slider styling, numeric inputs, keyboard, responsiveness |
| Quality (QA-401–404) | ✅ Complete | Manual tests, extreme values, reset, automated test; clippy in CI |

## Deliverables

- **Backend:** `src-tauri/src/depth_adjust.rs` — depth adjustment pipeline (brightness, contrast, gamma, invert, range)
- **Frontend:** `src/components/DepthControls.svelte` — sliders, invert checkbox, Reset, debounced preview
- **Python:** pytest suite (19 tests, stub mode), in CI
- **CI:** `cargo clippy` in pipeline; Rust 44 passed / 5 ignored, npm build PASS
- **Docs:** Depth controls documented in `docs/user-guide.md`
- **Gap carried to Sprint 1.5A:** Contrast slider not yet in UI (backend supports it)

## Blockers / Risks

None. Contrast slider and frontend test infrastructure are scheduled for Sprint 1.5A.

## References

- Task assignment: `SPRINT_1_5_Task_Assignment.md`
- Test plan: `TEST_PLAN_1_5.md`
- Manual results: `MANUAL_TEST_REPORT.md`
- Verification: `VERIFICATION_CHECKLIST.md`
- Follow-up sprint: `SPRINTS/Sprint_1_5A/` (hardening, contrast slider, frontend tests)
