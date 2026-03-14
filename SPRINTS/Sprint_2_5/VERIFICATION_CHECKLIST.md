# Verification Checklist — Sprint 2.5

**Sprint:** 2.5 — Masking & Regional Adjustments
**Last Updated:** 2026-03-14

Sign off each item before closing the sprint. All critical items must pass; non-critical items may be deferred with a filed ticket.

---

## 1. Architecture

- [x] **ARCH-502** — Mask state and command contract documented in RESEARCH/architecture.md (ADR-010 extended or new ADR)
- [x] Undo/redo semantics for mask mutations defined

---

## 2. Backend (Rust)

- [x] **BACK-1201** — Mask data structure and IPC (get/set/clear or equivalent)
- [x] **BACK-1202** — Adjustments apply only to masked regions
- [x] **BACK-1203** — Feathering at mask edges
- [x] `cargo test --manifest-path src-tauri/Cargo.toml --lib` — PASS (194 passed, 6 ignored; 2026-03-08; QE re-run)
- [x] `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` — 0 warnings
- [x] `cargo fmt --check` — PASS (2026-03-08)

---

## 3. Frontend

- [x] **UI-1201** — MaskingTools (brush, eraser, select)
- [x] **UI-1202** — Canvas-based mask painting
- [x] **UI-1203** — Mask opacity overlay on depth preview
- [x] **UI-1204** — Brush size/hardness controls
- [x] **JR1-1201** — Brush stroke smoothing
- [x] **JR1-1202** — Selection tools (rectangle, lasso)
- [x] **JR1-1203** — Mask save/load
- [x] `npm run build` — PASS (2026-03-08)
- [x] `npm test` — PASS (74 tests, 9 files; 2026-03-08)

---

## 4. QA Sign-off

- [x] **QA-1201** — Manual test: paint mask, adjust depth, verify isolation — **FAIL** (2026-03-14): mask has no visible effect; brush/overlay/depth isolation non-functional → **P0 bug**
- [ ] **QA-1202** — Manual test: mask feathering — **Blocked** (deferred until P0 fix)
- [ ] **QA-1203** — Manual test: undo/redo with masking — **Blocked** (deferred until P0 fix)
- [x] `SPRINTS/Sprint_2_5/MANUAL_TEST_REPORT.md` — filled (template + automated gate PASS 2026-03-08; Quick start for human tester added; manual Cases 1–3 ready)
- [ ] **P0:** Mask has no visible effect (brush, overlay, depth isolation). Fix required before QA-1202/1203 and sprint close. Ticket: *(create or link)*

---

## 5. Gotchas

- [x] New findings in `SPRINTS/Sprint_2_5/GOTCHAS.md` (file created; BACK-1203 box-blur note; manual QA deferred)
- [x] Relevant items merged to `RESEARCH/GOTCHAS.md` (2026-03-08: mask feather/soft-mask test assertions)

---

## Sign-off

| Role | Name / Agent ID | Date | Signature |
|------|----------------|------|-----------|
| System Architect | | | ☐ |
| Senior Engineer | | | ☐ |
| UI Designer | | | ☐ |
| Quality Engineer | Agent (QE) | 2026-03-08 | ☐ Automated gate PASS; manual cases handed off to human |

**Sprint Close Decision:** ☐ CLOSED / ☑ **DEFERRED** — P0: mask has no visible effect (Case 1 FAIL 2026-03-14). Fix mask pipeline, re-run manual Cases 1–3, then re-verify.

**Note:** Case 1 executed 2026-03-14; Cases 2–3 blocked until P0 fixed. See MANUAL_TEST_REPORT.md.
