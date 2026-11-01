# Sprint D: Validate and Harden

**Sprint Duration:** 1 week  
**Sprint Goal:** Validate engraver-compatible exports; complete security remediations from the 2026-05-19 review; align Tauri ACL with minimum privilege.  
**Phase:** Post-restart Phase D (`todo.md` v4.2)  
**Source:** `todo.md` — Sprint D; `SPRINTS/Sprint_D/SECURITY_REVIEW_2026-05.md`  
**Last Updated:** 2026-05-20

---

## Sprint Folder & Artefacts

| Artefact | Path |
|----------|------|
| Task Assignment | `SPRINTS/Sprint_D/SPRINT_D_Task_Assignment.md` (this file) |
| Security review | `SPRINTS/Sprint_D/SECURITY_REVIEW_2026-05.md` |
| Pentest checklist | `SPRINTS/Sprint_D/SECURITY_PENTEST_CHECKLIST.md` |
| Verification checklist | `SPRINTS/Sprint_D/VERIFICATION_CHECKLIST.md` |
| Security sign-off | `SPRINTS/Sprint_D/SECURITY_SIGNOFF.md` |
| Manual test report | `SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md` (create when executing QA) |
| Progress report | `SPRINTS/Sprint_D/PROGRESS_REPORT.md` |

---

## Roles required for this sprint

| Role | Why required |
|------|--------------|
| Security Specialist | SEC-D-02–SEC-D-12, SECURITY_SIGNOFF, pentest oversight |
| Senior Engineer | SEC-D-03–SEC-D-10, BACK-D-01, ACL permission files |
| System Architect | ARCH-D-01, ARCH-D-02, engraver charter |
| Quality Engineer | QA-D-01–QA-D-03, MANUAL_TEST_REPORT |
| Documentation Specialist | DOC-D-01, DOC-D-02 |

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks |
|------|--------------|--------|----------------|-------------|
| Security Specialist | `.agents/security-specialist.md` | Available | — | SEC-D-02, SEC-D-12, DOC-D-02, SECURITY_SIGNOFF |
| Senior Engineer | `.agents/senior-engineer.md` | In Progress | Wave5-Orchestrator | SEC-D-01, BACK-D-01 (blocked), QA-B-01 verify |
| System Architect | `.agents/system-architect.md` | Complete | 2026-05-20 review | ARCH-D-01, ARCH-D-02; `docs/ARCHITECT_PROGRESS_REVIEW_2026-05-20.md` |
| Quality Engineer | `.agents/quality-engineer.md` | Ready | Wave3-QE | QA-D-01 execution (XTool Studio) |
| Documentation Specialist | `.agents/documentation-specialist.md` | In Progress | Wave5-Docs | DOC-D-01 ✅; DOC-D-02 (with Security) |
| UI Designer | `.agents/ui-designer.md` | In Progress | UI-D-01 | F2 volume warnings (Export + Preview) |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Task breakdown

### Engraver validation

| Task ID | Owner | Description | Acceptance criteria |
|---------|-------|-------------|---------------------|
| ARCH-D-01 | Architect | Engraver charter in `docs/` or `RESEARCH/` | Axis mapping, units (mm), formats, point order documented |
| QA-D-01 | QE | Import PLY/XYZ into target engraver | Pass/Fail recorded; screenshots or notes in MANUAL_TEST_REPORT |
| BACK-D-01 | Senior Eng | Fix axis/units/metadata defects | Defects from QA-D-01 resolved or documented as engraver limitation |
| QA-D-02 | QE | 4K image → export timing | Result vs PRD &lt;2 min target in MANUAL_TEST_REPORT |
| DOC-D-01 | Docs | Interoperability notes | Known-good engraver settings linked from README or `docs/` |
| UI-D-01 | UI Designer | F2 Ultra UV volume warnings | Non-blocking exceed (blank > 70×70×150 mm) and near-edge (etch within 5 mm of limit) in Export panel + Preview3D; Vitest for `f2VolumeWarnings.ts`; limits synced with `docs/engraver-charter.md` |

### Permissions & audit

| Task ID | Owner | Description | Acceptance criteria |
|---------|-------|-------------|---------------------|
| SEC-D-01 | Senior Eng | Split `allow-generate-depth-map` into smaller groups | Each command group has a focused permission file |
| SEC-D-06 | Senior Eng | `allow-settings.toml`, `allow-model.toml` | All five settings/model commands in `capabilities/default.json`; invoke works |
| ARCH-D-02 | Architect | `lib.rs` ↔ `tauri.ts` ↔ ACL audit | Table in sprint folder or ARCH-D-02 note; no orphan commands |

### Security remediations (P0 — must complete for Phase D gate)

| Task ID | Owner | Description | Acceptance criteria |
|---------|-------|-------------|---------------------|
| SEC-D-02 | Security | Model SHA256 enforcement | `expected_model_hashes.json` populated; download fails on mismatch when hashes non-empty; test in `test_model_downloader.py` |
| SEC-D-03 | Senior Eng | Max image bytes + dimensions before Python | `read_image_bytes_for_depth` rejects oversize files; unit tests; documented limit in threat model |

### Security remediations (P1)

| Task ID | Owner | Description | Acceptance criteria |
|---------|-------|-------------|---------------------|
| SEC-D-04 | Senior Eng | IPC payload limits | Depth/point-cloud/mask paths bounded or file-based; no multi-hundred-MB single invoke at 8K |
| SEC-D-05 | Senior Eng | Preset/mask path policy | `load_preset` absolute path restricted; `load_mask_from_path` uses blocklist or dialog-only paths |

### Security remediations (P2–P3)

| Task ID | Owner | Description | Acceptance criteria |
|---------|-------|-------------|---------------------|
| SEC-D-07 | Senior Eng | Subprocess env hardening | Release profile clears/sanitizes env for Python children |
| SEC-D-08 | Senior Eng | `DepthAdjustmentParams` validation | Invalid gamma/contrast/curve/feather rejected at IPC boundary |
| SEC-D-09 | Senior Eng | Export atomic write / full-path canonicalize | TD-06 addressed; test or documented platform guard |
| SEC-D-10 | Senior Eng | Linux load_image blocklist parity | Matches export blocklist prefixes |
| SEC-D-11 | Senior Eng | `withGlobalTauri: false` (release) | Release build verified; devtools disabled in production |
| SEC-D-12 | Senior Eng | Export comment sanitization | Newlines stripped from filename metadata in PLY/CSV |
| QA-D-03 | QE | Pentest checklist | `SECURITY_PENTEST_CHECKLIST.md` executed; results in MANUAL_TEST_REPORT |
| DOC-D-02 | Security | Update threat model + checklist | `docs/threat-model.md`, `docs/security-checklist.md` reflect Sprint D |

---

## Dependencies

- SEC-D-02 requires a **trusted** model download on a maintainer machine before `--write-hashes`.
- SEC-D-06 should land before QA-D-03 (pentest should test real ACL behavior).
- QA-D-01 can run in parallel with security work if engraver access is available.

---

## Sprint gate (do not close until)

- [ ] QA-D-01: at least one engraver import validated
- [ ] SEC-D-02 and SEC-D-03 complete (or risk-accepted in SECURITY_SIGNOFF.md)
- [ ] SEC-D-06 + ARCH-D-02 + SEC-D-01: ACL complete
- [ ] UI-D-01: F2 volume warnings committed; Vitest in `npm test` passes
- [ ] SECURITY_SIGNOFF.md signed
- [ ] `todo.md` Sprint D checkboxes updated
