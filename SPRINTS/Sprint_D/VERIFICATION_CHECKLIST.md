# Verification Checklist — Sprint D

**Sprint:** D — Validate and Harden  
**Last Updated:** 2026-05-19

---

## Engraver & performance

- [ ] ARCH-D-01 engraver charter published
- [ ] QA-D-01 at least one engraver import validated (MANUAL_TEST_REPORT)
- [ ] BACK-D-01 compatibility fixes merged or waived with rationale
- [ ] QA-D-02 performance result recorded
- [ ] DOC-D-01 interoperability notes published

## Security (P0 gate)

- [ ] SEC-D-02 model SHA256 hashes populated and enforced
- [ ] SEC-D-03 max image size / dimension limits enforced and tested

## Security (P1)

- [ ] SEC-D-04 IPC payload limits implemented
- [ ] SEC-D-05 preset/mask path restrictions implemented

## Security (P2–P3)

- [ ] SEC-D-01 permission groups split (or waiver documented)
- [ ] SEC-D-06 settings + model ACL permissions added
- [ ] SEC-D-07 subprocess env hardening (release)
- [ ] SEC-D-08 depth adjustment param validation
- [ ] SEC-D-09 export TOCTOU / atomic write
- [ ] SEC-D-10 Linux load_image blocklist aligned
- [ ] SEC-D-11 withGlobalTauri / devtools (release)
- [ ] SEC-D-12 export comment sanitization
- [ ] QA-D-03 pentest checklist executed

## Consistency & docs

- [ ] ARCH-D-02 command ↔ ACL ↔ frontend audit complete
- [ ] DOC-D-02 threat model and security checklist updated
- [ ] SECURITY_SIGNOFF.md signed
- [ ] `todo.md` Sprint D tasks marked complete
- [ ] Technical debt TD-15–TD-21 closed or reclassified

---

**Sprint closed:** ☐ Yes ☐ No — **Date:** ___________
