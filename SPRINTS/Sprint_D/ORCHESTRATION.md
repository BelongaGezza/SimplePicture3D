# Sprint D — Sub-Agent Orchestration Board

**Orchestrator:** Senior Engineer  
**Last Updated:** 2026-05-20 (Wave 5 — Architect delegation)  
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
| SEC-D-05 | Senior Engineer | Orchestrator Wave 2 | Complete | `path_security.rs`; preset/mask import validation |
| SEC-D-06 | Senior Engineer | Orchestrator Wave 2 | Complete | `allow-settings.toml`, `allow-model.toml` in capabilities |
| SEC-D-01 | Senior Engineer | Wave 5 sub-agent | Complete | Four TOMLs; `default.json`; old `allow-generate-depth-map` removed |
| SEC-D-07–11 | Senior Engineer | — | Available | P2/P3 — see task assignment |
| SEC-D-12 | Security Specialist | Wave 1 sub-agent | Complete | PLY/CSV comment sanitization |
| ARCH-D-01 | System Architect | Wave 3 sub-agent | Complete | `docs/engraver-charter.md` |
| ARCH-D-02 | System Architect | Wave 4 sub-agent | Complete | `ARCH-D-02_COMMAND_AUDIT.md`; SEC-D-01 split recommended |
| QA-D-01 | Quality Engineer | Wave 3 sub-agent | Ready | XTool Studio + F2 Ultra UV; run § D per procedure |
| QA-D-02 | Quality Engineer | — | Available | 4K timing benchmark |
| QA-D-03 | Quality Engineer | — | Available | Pentest checklist |
| QA-B-01 | Senior Engineer | Wave 5 shell | Blocked | 9 tests in `lib.rs`; `cargo test` exit 101 — WebKit/soup dev libs; sudo required |
| BACK-D-01 | Senior Engineer | Orchestrator | Blocked | Awaiting QA-D-01 results |
| DOC-D-01 | Documentation Specialist | Wave 5 sub-agent | Complete | `docs/engraver-interop.md`; README link |
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

### Wave 3 — Validation ✅ (procedure complete; engraver run blocked)
6. **ARCH-D-01** — Engraver charter — complete
7. **QA-D-01** — Procedure ready; **execute** import in XTool Studio (F2 Ultra UV)
8. **BACK-D-01** — Blocked on QA-D-01 engraver execution

### Wave 4 — Closeout ✅
9. **ARCH-D-02** — Command audit — complete (`ARCH-D-02_COMMAND_AUDIT.md`)
10. **QA-B-01** — Export integration tests — review (8 tests in `lib.rs`; verify on CI/Windows)
11. **ARCH-D-01 / QA-D-01 procedure** — complete; engraver execution pending

### Wave 5 — Gate closeout (active — Architect 2026-05-20)
12. **SEC-D-01** — Split `allow-generate-depth-map` → four TOMLs — **complete**
13. **QA-B-01** — `cargo test` verify — **delegated** (shell / CI)
14. **DOC-D-01** — `docs/engraver-interop.md` — **delegated** (docs sub-agent)
15. **DOC-D-02** — Threat model + security checklist — **delegated** (security/docs)
16. **QA-D-01** — Human engraver session — **blocked on hardware** (QE)
17. **BACK-D-01** — Blocked on QA-D-01
18. **SECURITY_SIGNOFF** — After DOC-D-02 + Wave 5 code land

**Architect review:** `docs/ARCHITECT_PROGRESS_REVIEW_2026-05-20.md`

---

## Sub-agent launch log

| Timestamp | Sub-agent type | Task(s) | Result |
|-----------|----------------|---------|--------|
| 2026-05-19 | Security Specialist | SEC-D-02, SEC-D-12 | Complete |
| 2026-05-19 | senior-engineer | SEC-D-03 | Complete |
| 2026-05-19 | Orchestrator (direct) | SEC-D-04, SEC-D-05, SEC-D-06 | Complete — permissions, path_security, IPC caps |
| 2026-05-19 | System Architect | ARCH-D-01 | Complete — `docs/engraver-charter.md` |
| 2026-05-19 | Quality Engineer | QA-D-01 | Procedure complete — `QA-D-01_TEST_PROCEDURE.md`, MANUAL_TEST_REPORT expanded; engraver run blocked |
| 2026-05-19 | explore (Architect) | ARCH-D-02 | Complete — 36/36 aligned; SEC-D-01 split recommended |
| 2026-05-19 | senior-engineer | QA-B-01 | Review — 8 export command tests + `_impl` refactor in `lib.rs` |
| 2026-05-20 | System Architect | Wave 5 plan | `docs/ARCHITECT_PROGRESS_REVIEW_2026-05-20.md`; delegated SEC-D-01, DOC-D-01/02, QA-B-01 verify |
| 2026-05-20 | shell | QA-B-01 verify | Blocked — WebKit/soup dev libs; sudo required on Linux host |
| 2026-05-20 | senior-engineer | SEC-D-01 | Complete — four depth/mask permission TOMLs |

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
| Target engraver software | Product | **Resolved** — XTool Studio + xTool F2 Ultra UV (2026-05-19) |

---

## Integration checklist (orchestrator)

After Wave 4:
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` (blocked: GTK/WebKit on Linux dev)
- [ ] `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`
- [x] `npm test` — 75 passed (Wave 4 re-run after `npm install`)
- [x] `SP3D_USE_STUB=1 pytest python/` — 24 passed (Wave 1)
- [x] ARCH-D-01 charter: `docs/engraver-charter.md`
- [x] QA-D-01 procedure: `QA-D-01_TEST_PROCEDURE.md`
- [ ] QA-D-01 engraver execution (XTool Studio + F2 Ultra UV)
- [ ] BACK-D-01 (blocked on QA-D-01)
- [x] ARCH-D-02 audit: `ARCH-D-02_COMMAND_AUDIT.md`
- [ ] QA-B-01 cargo verify (CI or Windows; Linux needs apt GTK/WebKit)
- [x] Update Role Assignment in `SPRINT_D_Task_Assignment.md`
- [x] Update this file
