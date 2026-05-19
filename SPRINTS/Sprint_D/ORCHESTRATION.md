# Sprint D — Sub-Agent Orchestration Board

**Orchestrator:** Senior Engineer  
**Last Updated:** 2026-05-19 (SEC-D-02, SEC-D-03, SEC-D-12)  
**Sprint:** Validate and Harden (Phase D)  
**Roadmap:** `ROADMAP.md` — do not mark Phase D complete until QA-D-01 + SEC-D-02/03 pass

---

## Current sprint goal

Validate engraver-compatible exports; close P0/P1 security gaps; align Tauri ACL with minimum privilege.

---

## Delegation status

| Task ID | Owner role | Sub-agent / session | Status | Notes |
|---------|------------|---------------------|--------|-------|
| SEC-D-02 | Security Specialist | Sub-agent session | Complete | Stub hash manifest + downloader tests; real digests pending trusted download |
| SEC-D-03 | Senior Engineer | senior-engineer subagent | Complete | P0 — `MAX_IMAGE_FILE_BYTES=128 MiB` + `MAX_DIMENSION=8192` enforced in `read_image_bytes_for_depth` pre-Python; 7 new unit tests; threat model §2.4 updated |
| SEC-D-04 | Senior Engineer | — | Available | P1 — IPC payload limits |
| SEC-D-05 | Senior Engineer | — | Available | P1 — preset/mask path policy |
| SEC-D-06 | Senior Engineer | — | Available | P1 — settings/model ACL TOMLs |
| SEC-D-01 | Senior Engineer | — | Available | Split coarse permissions |
| SEC-D-07–11 | Senior Engineer | — | Available | P2/P3 — see task assignment |
| SEC-D-12 | Security Specialist | Sub-agent session | Complete | PLY/CSV source comment sanitization in export.rs |
| ARCH-D-01 | System Architect | — | Available | Engraver charter doc |
| ARCH-D-02 | System Architect | — | Available | Command ↔ ACL audit |
| QA-D-01 | Quality Engineer | — | Available | Manual engraver import |
| QA-D-02 | Quality Engineer | — | Available | 4K timing benchmark |
| QA-D-03 | Quality Engineer | — | Available | Pentest checklist |
| QA-B-01 | Quality Engineer | — | Available | Export integration tests |
| BACK-D-01 | Senior Engineer | — | Available | Fix defects from QA-D-01 |
| DOC-D-01 | Documentation Specialist | — | Available | Interoperability notes |
| DOC-D-02 | Security + Docs | — | Available | Threat model update |

**Status values:** `Available` | `Delegated` | `In Progress` | `Review` | `Complete` | `Blocked`

---

## Execution waves

### Wave 1 — P0 security (parallel)
1. **SEC-D-03** — Senior Engineer: enforce max file size + dimensions in `read_image_bytes_for_depth`
2. **SEC-D-02** — Security Specialist: model hash table + downloader verification tests

### Wave 2 — P1 hardening (after Wave 1 review)
3. **SEC-D-06** — ACL for settings/model commands
4. **SEC-D-04** — IPC payload caps / preview decimation design
5. **SEC-D-05** — preset/mask path restrictions

### Wave 3 — Validation (requires running app + engraver)
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
| 2026-05-19 | Security Specialist | SEC-D-02, SEC-D-12 | Stub hash manifest, downloader verification tests, export comment sanitization |
| 2026-05-19 | senior-engineer | SEC-D-03 | Complete — `MAX_IMAGE_FILE_BYTES=128 MiB` (pre-`fs::read` via metadata) + 8192 header-dimension cap (pre-Python) in `read_image_bytes_for_depth`; same file-size cap on `load_image_impl`; 7 new unit tests; threat model §2.4 + version bumped to 1.3. All 21 `image_loading` tests pass; clippy clean. Full `cargo test` blocked by missing GTK/WebKit dev libs (per ORCHESTRATION blocker) — verified via isolated harness with identical deps. |

---

## Blockers

| Blocker | Owner | Resolution |
|---------|-------|------------|
| Linux Tauri build needs GTK/WebKit | Dev env | `SETUP_NOTES.md` |
| Real model SHA256 digests | Security / Maintainer | Trusted HF download + `--write-hashes` on maintainer machine; commit enforced hashes |
| Target engraver software TBD | Product / QE | Name in ARCH-D-01 before QA-D-01 |

---

## Integration checklist (orchestrator)

After each wave:
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml`
- [ ] `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`
- [ ] `npm test`
- [ ] `SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v`
- [ ] Update Role Assignment in `SPRINT_D_Task_Assignment.md`
- [ ] Update this file and `PROGRESS_REPORT.md`
