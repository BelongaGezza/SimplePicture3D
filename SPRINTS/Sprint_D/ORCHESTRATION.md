# Sprint D — Sub-Agent Orchestration Board

**Orchestrator:** Senior Engineer  
**Last Updated:** 2026-05-24 (SEC-E-01 compile fix landed)  
**Sprint:** Validate and Harden (Phase D)  
**Roadmap:** `ROADMAP.md` — do not mark Phase D complete until QA-D-01 + SEC-D-02/03 pass

---

## Current sprint goal

Validate engraver-compatible exports; close P0/P1 security gaps; align Tauri ACL with minimum privilege.

---

## Delegation status

| Task ID | Owner role | Sub-agent / session | Status | Notes |
|---------|------------|---------------------|--------|-------|
| SEC-D-02 | Security Specialist | Wave 1 sub-agent | Complete | Stub hash manifest; real digests pending trusted download |
| SEC-D-03 | Senior Engineer | Wave 1 sub-agent | Complete | 128 MiB + 8192 header cap pre-Python |
| SEC-D-04 | Senior Engineer | Orchestrator Wave 2 | Complete | 1M IPC preview cap; 8M generation cap; `forPreview` param |
| SEC-D-05 | Senior Engineer | Orchestrator Wave 2 | Complete | `path_security.rs`; preset/mask import validation; **SEC-E-01** import fix 2026-05-24 |
| SEC-E-01 | Senior Engineer | 2026-05-24 | Complete | `use path_security::{is_blocklisted, validate_import_file_path}` in `lib.rs` |
| SEC-D-06 | Senior Engineer | Orchestrator Wave 2 | Complete | `allow-settings.toml`, `allow-model.toml` in capabilities |
| SEC-D-01 | Senior Engineer | — | Available | Split coarse permissions |
| SEC-D-07–11 | Senior Engineer | — | Available | P2/P3 — see task assignment |
| SEC-D-12 | Security Specialist | Wave 1 sub-agent | Complete | PLY/CSV comment sanitization |
| ARCH-D-01 | System Architect | Wave 3 sub-agent | Complete | `docs/engraver-charter.md` |
| ARCH-D-02 | System Architect | — | Available | Command ↔ ACL audit |
| QA-D-01 | Quality Engineer | Wave 3 sub-agent | Review | Procedure ready; engraver import blocked (TBD software) |
| QA-D-07 | Quality Engineer | 2026-05-24 | Complete | `TEST_PLAN_D.md` from template |
| QA-D-08 | Quality Engineer | 2026-05-24 | Complete | `SPRINT_2_6_CANCELLED.md` |
| QA-D-09 | Quality Engineer | 2026-05-24 | Complete | Three entries in `RESEARCH/GOTCHAS.md` |
| QA-D-02 | Quality Engineer | — | Available | 4K timing benchmark |
| QA-D-03 | Quality Engineer | — | Available | Pentest checklist |
| QA-B-01 | Quality Engineer | 2026-05-24 | Complete | Five export command tests in `lib.rs` |
| CI-D-01 | Quality Engineer | 2026-05-24 | Complete | `backend-windows` CI job |
| QA-D-05 | Quality Engineer | 2026-05-24 | Complete | `exportXyz`/`exportCsv`/`setPointCloudFormat` Vitest |
| BACK-D-01 | Senior Engineer | Orchestrator | Blocked | Awaiting QA-D-01 results |
| DOC-D-01 | Documentation Specialist | — | Available | Interoperability notes |
| DOC-D-02 | Security + Docs | — | Partial | Threat model §2.4/§2.6 updated Wave 2 |

**Status values:** `Available` | `Delegated` | `In Progress` | `Review` | `Complete` | `Blocked`

---

## Execution waves

### Wave 1 — P0 security ✅
1. **SEC-D-03** — max file size + dimensions pre-Python
2. **SEC-D-02** — model hash table + downloader tests

### Wave 2 — P1 hardening ✅
3. **SEC-D-06** — ACL for settings/model commands
4. **SEC-D-04** — IPC point cloud payload caps + preview subsampling
5. **SEC-D-05** — preset/mask path restrictions (`path_security.rs`)

### Wave 3 — Validation (in progress)
6. **ARCH-D-01** — Engraver charter (axes, mm, formats) — delegated
7. **QA-D-01** — Import PLY/XYZ; record in MANUAL_TEST_REPORT — delegated (procedure + checklist)
8. **BACK-D-01** — Fix axis/units bugs from QA — blocked on QA-D-01 execution

### Wave 4 — Closeout
9. **ARCH-D-02** — Full command audit
10. **QA-B-01** — Export integration tests
11. **DOC-D-01**, **DOC-D-02**, **SECURITY_SIGNOFF**

---

## Sub-agent launch log

| Timestamp | Sub-agent type | Task(s) | Result |
|-----------|----------------|---------|--------|
| 2026-05-19 | Security Specialist | SEC-D-02, SEC-D-12 | Complete |
| 2026-05-19 | senior-engineer | SEC-D-03 | Complete |
| 2026-05-19 | Orchestrator (direct) | SEC-D-04, SEC-D-05, SEC-D-06 | Complete — permissions, path_security, IPC caps |
| 2026-05-19 | System Architect | ARCH-D-01 | Complete — `docs/engraver-charter.md` |
| 2026-05-19 | Quality Engineer | QA-D-01 | Procedure complete — `QA-D-01_TEST_PROCEDURE.md`, MANUAL_TEST_REPORT expanded; engraver run blocked |
| 2026-05-24 | Quality Engineer | QA-D-07, QA-D-08, QA-D-09 | `TEST_PLAN_D.md`, `SPRINT_2_6_CANCELLED.md`, GOTCHAS ×3; MANUAL_TEST_REPORT QA-C-01 sections |

---

## Wave 3 plan (orchestrator)

1. **ARCH-D-01:** Document axis mapping (image px → X/Y mm, depth → Z mm), right-handed frame, origin at blank corner (0,0,0), units mm, PLY/XYZ/CSV formats, point order (row-major sample order), `fit_to_blank` post-scale. Source: ADR-012, `volumetric.rs`, `export.rs`, `blank_envelope.rs`.
2. **QA-D-01:** Expand `MANUAL_TEST_REPORT.md` with step-by-step import procedure, pass/fail matrix, engraver software field (TBD product name). Generate a minimal golden PLY/XYZ via `cargo test` export tests or document using app export.
3. **BACK-D-01:** After QA records failures, fix Rust export/coordinate bugs or document engraver limitation in charter.

---

## Blockers

| Blocker | Owner | Resolution |
|---------|-------|------------|
| Linux Tauri build needs GTK/WebKit | Dev env | `SETUP_NOTES.md` |
| Real model SHA256 digests | Security / Maintainer | Trusted HF download + `--write-hashes` |
| Target engraver software TBD | Product / QE | Name in ARCH-D-01 before QA-D-01 |

---

## Integration checklist (orchestrator)

After Wave 3:
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` (blocked: GTK/WebKit on Linux dev)
- [ ] `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`
- [x] `npm test` — 75 passed (Wave 3)
- [x] `SP3D_USE_STUB=1 pytest python/` — 24 passed (Wave 1)
- [x] ARCH-D-01 charter: `docs/engraver-charter.md`
- [x] QA-D-01 procedure: `QA-D-01_TEST_PROCEDURE.md`
- [ ] QA-D-01 engraver execution (blocked: target software TBD)
- [ ] BACK-D-01 (blocked on QA-D-01)
- [x] Update Role Assignment in `SPRINT_D_Task_Assignment.md`
- [x] Update this file
