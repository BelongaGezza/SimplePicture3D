# Sprint 1.5A Verification Checklist

**Sprint:** 1.5A — Hardening, Testing & Consultant Remediation
**Purpose:** Sign-off before marking sprint complete.
**Reference:** `SPRINT_1_5A_Task_Assignment.md`

---

## Quality Gates

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` — PASS
- [ ] `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` — 0 warnings
- [ ] `cargo fmt --manifest-path src-tauri/Cargo.toml --check` — PASS
- [ ] `npm run build` — PASS
- [ ] `npm test` — PASS, ≥15 frontend tests
- [ ] `pytest python/ -v` (stub mode) — PASS, 19+ tests
- [ ] `cargo tarpaulin` — runs, baseline recorded
- [ ] `pytest --cov` — runs, baseline recorded

## Consultant Remediation

- [ ] Contrast slider functional in DepthControls (§3.2 gap closed)
- [ ] Asset protocol scope restricted or disabled (§3.4 gap closed)
- [ ] IPC performance measured and recommendation documented (§3.3 gap evaluated)
- [ ] Model license decision documented as ADR-005 (§1.2 gap closed)
- [ ] Sprint 1.5 artefacts reflect actual completion state (§3.5 gap closed)
- [ ] todo.md Testing Strategy section reflects current state (§3.7 gap closed)

## Success Criteria (from Task Assignment)

- [ ] `npm test` exists and passes with ≥15 frontend tests
- [ ] All 5 depth adjustment controls functional (brightness, contrast, gamma, invert, depth range)
- [ ] Coverage reporting in CI for Rust and Python
- [ ] Security sign-off completed (SECURITY_SIGNOFF.md)
- [ ] README has Python setup instructions
- [ ] CLAUDE.md testing commands updated
- [ ] No blocking issues carried to Sprint 1.6

## Process

- [ ] Gotchas recorded in `SPRINTS/Sprint_1_5A/GOTCHAS.md`
- [ ] Optional: merge notable gotchas to `RESEARCH/GOTCHAS.md`
- [ ] Progress report filed (PROGRESS_REPORT.md)
- [ ] Security sign-off completed (SECURITY_SIGNOFF.md)

## Sign-Off

*(Fill when sprint complete.)*

Sprint 1.5A verification: [ ] Not Started / [ ] In Progress / [ ] Complete

**Last Updated:** 2026-02-06
