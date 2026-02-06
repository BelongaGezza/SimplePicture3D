# Sprint 1.5 Verification Checklist

**Sprint:** 1.5 — Manual Depth Adjustments  
**Purpose:** Sign-off before marking sprint complete.  
**Reference:** `SPRINT_1_5_Task_Assignment.md`

---

## Quality Gates

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` — PASS
- [ ] `cargo clippy` — 0 warnings (QA-405: enforced in CI)
- [ ] `npm run build` — PASS
- [ ] Depth adjustment controls functional (sliders, invert, reset)
- [ ] Manual test cases (QA-401–403) executed and recorded in MANUAL_TEST_REPORT.md
- [ ] Depth controls documented for users (`docs/user-guide.md` § Depth Controls)

## Success Criteria (todo.md Sprint 1.5)

- [ ] All depth adjustment controls functional
- [ ] Preview updates within 100 ms of slider change
- [ ] Reset button restores original depth map
- [ ] Automated tests cover adjustment logic
- [ ] UI responsive and intuitive

## Process

- [ ] Gotchas recorded in `SPRINTS/Sprint_1_5/GOTCHAS.md`
- [ ] Optional: merge notable gotchas to `RESEARCH/GOTCHAS.md`
- [ ] Manual test report completed
- [ ] Progress report filed (PROGRESS_REPORT.md)

## Sign-Off

*(Fill when sprint complete.)*

Sprint 1.5 verification: [ ] Not Started / [ ] In Progress / [ ] Complete

**Last Updated:** 2026-02-06
