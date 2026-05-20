# Progress Report — Sprint D

**Sprint:** D — Validate and Harden  
**Last Updated:** 2026-05-20 (Wave 5 — Architect review)

---

## Status summary

| Track | Status | Notes |
|-------|--------|-------|
| Engraver validation (ARCH-D-01, QA-D-01) | Ready for execution | Charter + procedure; target: XTool Studio / F2 Ultra UV |
| Performance (QA-D-02) | Not started | |
| Security P0 (SEC-D-02, SEC-D-03) | Complete | Wave 1 |
| Security P1 (SEC-D-04–SEC-D-06) | Complete | Wave 2 |
| ACL (SEC-D-01, SEC-D-06, ARCH-D-02) | Partial | SEC-D-06 + ARCH-D-02 done; SEC-D-01 split pending |
| Export tests (QA-B-01) | Review | 8 command-level tests in `lib.rs`; cargo blocked on Linux |
| BACK-D-01 | Blocked | Awaiting QA-D-01 engraver run |

---

## 2026-05-19 — Wave 4

- **ARCH-D-02:** `SPRINTS/Sprint_D/ARCH-D-02_COMMAND_AUDIT.md` — 36 commands aligned across lib.rs, tauri.ts, ACL; zero orphans; SEC-D-01 split recommended.
- **QA-B-01:** Eight export integration tests + `export_*_impl` helpers in `src-tauri/src/lib.rs` (PLY ASCII/binary, XYZ, CSV, error paths, SEC-D-12 sanitization).
- **npm test:** 75 passed (after `npm install` fixed rolldown native binding).
- **pytest:** 44 passed (`SP3D_USE_STUB=1`, python3).
- **cargo test:** Still blocked — GTK/WebKit/libsoup dev libs per `SETUP_NOTES.md`.

---

## 2026-05-19 — Wave 3

- **ARCH-D-01:** `docs/engraver-charter.md` — axis mapping, mm units, PLY/XYZ/CSV; target **XTool Studio** + **xTool F2 Ultra UV**.
- **QA-D-01 (procedure):** `SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md`; `MANUAL_TEST_REPORT.md` expanded with charter checks and BACK-D-01 defect log.
- **Export fixture test:** `export_ply_xyz_writes_disk_fixture_format` in `export.rs`.
- **npm test:** 75 passed.
- **cargo test:** Blocked on Linux dev host (GTK/WebKit per `SETUP_NOTES.md`).
- Module comment alignment: `export.rs`, `blank_envelope.rs` reference ADR-012.

---

## 2026-05-19 — Wave 2

- SEC-D-04, SEC-D-05, SEC-D-06 complete (IPC caps, path_security, ACL permissions).

---

## 2026-05-19 — Wave 1

- Security review; SEC-D-02, SEC-D-03, SEC-D-12 complete.

---

## Blockers

| Blocker | Owner | Resolution |
|---------|-------|------------|
| Target engraver software | Product | **Resolved** — XTool Studio + xTool F2 Ultra UV |
| Linux Tauri build needs GTK/WebKit | Dev env | `SETUP_NOTES.md` |
| Real model SHA256 digests | Security / Maintainer | Trusted HF download + `--write-hashes` |

---

## 2026-05-20 — Architect Wave 5 delegation

- **Review:** `docs/ARCHITECT_PROGRESS_REVIEW_2026-05-20.md` — Phase D gate not passed; feature freeze confirmed.
- **Requirements sync:** `todo.md` v4.4, `ROADMAP.md` status diagram updated.
- **Delegated:** SEC-D-01 (senior-engineer), QA-B-01 verify (shell), DOC-D-01/DOC-D-02 (docs).
- **UI-D-01:** F2 volume warnings folded into plan (`docs/ARCHITECT_PROGRESS_REVIEW_2026-05-20.md` §7); implementation on disk, commit pending.

---

## Next actions

1. **QE (human):** Run C1–D7 per `QA-D-01_TEST_PROCEDURE.md` in **XTool Studio** (F2 Ultra UV); fill `MANUAL_TEST_REPORT.md`.
2. **Senior Engineer:** SEC-D-01 ACL split (Wave 5 sub-agent); BACK-D-01 after QA-D-01.
3. **CI / Windows / Linux+GTK:** Run `cargo test` for QA-B-01; mark complete when green.
4. **Docs + Security:** DOC-D-01 interop notes; DOC-D-02 threat model; SECURITY_SIGNOFF.
