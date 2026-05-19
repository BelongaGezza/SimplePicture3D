# Progress Report — Sprint D

**Sprint:** D — Validate and Harden  
**Last Updated:** 2026-05-19 (Wave 3)

---

## Status summary

| Track | Status | Notes |
|-------|--------|-------|
| Engraver validation (ARCH-D-01, QA-D-01) | In progress | Charter complete; QA procedure ready; engraver import blocked (TBD software) |
| Performance (QA-D-02) | Not started | |
| Security P0 (SEC-D-02, SEC-D-03) | Complete | Wave 1 |
| Security P1 (SEC-D-04–SEC-D-06) | Complete | Wave 2 |
| ACL (SEC-D-01, SEC-D-06, ARCH-D-02) | Partial | SEC-D-06 done; ARCH-D-02 pending |
| BACK-D-01 | Blocked | Awaiting QA-D-01 engraver run |

---

## 2026-05-19 — Wave 3

- **ARCH-D-01:** `docs/engraver-charter.md` — axis mapping, mm units, PLY/XYZ/CSV, validation checklist, engraver TBD.
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
| Target engraver software TBD | Product / QE | Name in charter §2 + MANUAL_TEST_REPORT before QA-D-01 Pass |
| Linux Tauri build needs GTK/WebKit | Dev env | `SETUP_NOTES.md` |
| Real model SHA256 digests | Security / Maintainer | Trusted HF download + `--write-hashes` |

---

## Next actions

1. **Product:** Confirm target engraver software for QA-D-01.
2. **QE:** Run C1–D7 per `QA-D-01_TEST_PROCEDURE.md`; fill `MANUAL_TEST_REPORT.md`.
3. **Senior Engineer:** BACK-D-01 fixes from defect log (axis/units if any).
4. **Wave 4:** ARCH-D-02, QA-B-01, DOC-D-01, SECURITY_SIGNOFF.
