# Verification Checklist — Sprint 2.4

**Sprint:** 2.4 — Progress Streaming for Depth Estimation
**Last Updated:** 2026-03-06

Sign off each item before closing the sprint. All critical items must pass; non-critical items may be deferred with a filed ticket.

---

## 1. Architecture

- [x] **ARCH-501** — Event contract (`depth-progress` name, payload schema) documented in RESEARCH/architecture.md (ADR-002 addendum)
- [x] Threading model for `generate_depth_map` async pattern recorded

---

## 2. Backend (Rust)

- [x] **BACK-205-STREAM** — `run_depth_estimation_with_progress` (or equivalent) emits per-line PROGRESS events via callback
- [x] **BACK-205-IPC** — `generate_depth_map` accepts `tauri::AppHandle`; emits `depth-progress` events during estimation
- [x] `DepthProgressPayload` struct defined and serializable
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` — PASS (2026-03-06: 181 passed)
- [x] `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` — 0 warnings
- [x] `cargo fmt --check` — PASS (run from src-tauri)
- [ ] `cargo audit --manifest-path src-tauri/Cargo.toml` — no new advisories

---

## 3. Frontend

- [x] **UI-304** — Real percentage progress bar replaces indeterminate animation
- [x] `listen<DepthProgressEvent>("depth-progress", ...)` called before depth command
- [x] `unlisten()` called in `finally` block (no memory leak)
- [x] `progressPercent` resets to 0 before each new estimation
- [x] `aria-valuenow` attribute is present and updates correctly
- [x] `npm run build` — PASS (2026-03-06)
- [x] `npm test` — PASS (2026-03-06: 45 tests)
- [ ] `npm audit --audit-level=high` — no high/critical

---

## 4. Automated Tests (Sprint 2.3 Carryover)

- [x] **JR2-1301** — Preset round-trip tests pass
- [x] **JR2-1302** — Frontend preset API tests pass
- [x] **JR2-1303** — Invalid JSON / schema version tests pass
- [x] All above included in `cargo test` and `npm test` CI run

---

## 5. Security (SEC-202)

- [x] **SEC-202** — HTTPS download confirmed for all model download paths
- [x] SHA256 outcome documented: SEC-202A (implemented) or SEC-202B (risk-accepted with rationale)
- [x] `SPRINTS/Sprint_2_4/SECURITY_SIGNOFF.md` filed and signed by Security Specialist
- [x] `docs/threat-model.md` §2.2 SEC-202 marked reviewed with date
- [x] `docs/security-checklist.md` §2.2 SEC-202 updated

---

## 6. QA Sign-off

- [ ] **QA-304-STREAM** — Manual streaming test executed; real percentage visible during 4K run (deferred: requires human tester with app + real model; report template ready)
- [ ] **QA-1301–1303** — Preset manual QA completed by named tester (deferred: report template ready)
- [x] `SPRINTS/Sprint_2_4/MANUAL_TEST_REPORT.md` — filled (template + execution note; Pass/Fail to be filled when manual tests run)
- [x] Any P0/P1 issues from manual QA resolved or explicitly deferred with ticket (N/A until manual run)

---

## 7. Documentation

- [x] **DOC-204** — `docs/architecture.md` progress section updated for Sprint 2.4
- [x] `docs/developer-guide.md` `generate_depth_map` row notes `depth-progress` event
- [x] `docs/user-guide.md` Step 2 wording updated to mention percentage bar
- [x] `docs/release-notes-draft.md` — v0.4.0-beta.1 draft added

---

## 8. Gotchas

- [ ] Any new debugging findings added to `SPRINTS/Sprint_2_4/GOTCHAS.md`
- [ ] Items relevant to other sprints merged to `RESEARCH/GOTCHAS.md`

---

## Sign-off

| Role | Name / Agent ID | Date | Signature |
|------|----------------|------|-----------|
| System Architect | | | ☐ |
| Senior Engineer | | | ☐ |
| UI Designer | | | ☐ |
| Quality Engineer | Cursor Agent 2026-03-06 | 2026-03-06 | ☐ (automated verification done; manual tests deferred to human) |
| Security Specialist | | | ☐ |

**Sprint Close Decision:** ☐ CLOSED — all critical items passed / ☐ DEFERRED — see notes
