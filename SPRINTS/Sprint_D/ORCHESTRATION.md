# Sprint D — Sub-Agent Orchestration Board

**Orchestrator:** Senior Engineer  
**Last Updated:** 2026-05-19 (Wave 2 complete)  
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
| SEC-D-01 | Senior Engineer | — | Available | Split coarse permissions |
| SEC-D-07–11 | Senior Engineer | — | Available | P2/P3 — see task assignment |
| SEC-D-12 | Security Specialist | Wave 1 sub-agent | Complete | PLY/CSV comment sanitization |
| ARCH-D-01 | System Architect | — | Available | Engraver charter doc |
| ARCH-D-02 | System Architect | — | Available | Command ↔ ACL audit |
| QA-D-01 | Quality Engineer | — | Available | Manual engraver import |
| QA-D-02 | Quality Engineer | — | Available | 4K timing benchmark |
| QA-D-03 | Quality Engineer | — | Available | Pentest checklist |
| QA-B-01 | Quality Engineer | — | Available | Export integration tests |
| BACK-D-01 | Senior Engineer | — | Available | Fix defects from QA-D-01 |
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

### Wave 3 — Validation (next)
6. **ARCH-D-01** — Engraver charter (axes, mm, formats)
7. **QA-D-01** — Import PLY/XYZ; record in MANUAL_TEST_REPORT
8. **BACK-D-01** — Fix axis/units bugs from QA

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

---

## Blockers

| Blocker | Owner | Resolution |
|---------|-------|------------|
| Linux Tauri build needs GTK/WebKit | Dev env | `SETUP_NOTES.md` |
| Real model SHA256 digests | Security / Maintainer | Trusted HF download + `--write-hashes` |
| Target engraver software TBD | Product / QE | Name in ARCH-D-01 before QA-D-01 |

---

## Integration checklist (orchestrator)

After Wave 2:
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` (blocked: GTK/WebKit on Linux dev)
- [ ] `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`
- [x] `npm test` — run after Wave 2 commit
- [x] `SP3D_USE_STUB=1 pytest python/` — 24 passed (Wave 1)
- [ ] Update Role Assignment in `SPRINT_D_Task_Assignment.md`
- [x] Update this file
