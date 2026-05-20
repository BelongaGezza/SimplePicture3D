# Security Sign-off — Sprint D

**Sprint:** D — Validate and Harden  
**Owner:** Security Specialist  
**Reference:** `SPRINTS/Sprint_D/SECURITY_REVIEW_2026-05.md`, `docs/threat-model.md`, `docs/security-checklist.md`  
**Date:** 2026-05-20 (Wave 6 — sign-off prep; signatures pending)

---

## Scope

Sprint D security tasks **SEC-D-02** through **SEC-D-12**, plus permission work **SEC-D-01** / **SEC-D-06**, and manual verification **QA-D-03**.

---

## Outcome table

| Task | Description | Status | Notes |
|------|-------------|--------|-------|
| SEC-D-01 | Split coarse depth/mask ACL | ☑ | Four permission TOMLs per `ARCH-D-02`; `allow-generate-depth-map` removed from `default.json`. |
| SEC-D-02 | Model SHA256 enforcement | ☑ | Stub `expected_model_hashes.json`; verification skips until trusted HF download + `--write-hashes` (waiver below). |
| SEC-D-03 | Max image size before Python | ☑ | 128 MiB file cap + 8192 px header/dimension gate on `read_image_bytes_for_depth`; unit tests in `image_loading.rs`. |
| SEC-D-04 | IPC payload caps | ☑ | 1M preview / 8M generation caps; export workflow omits full cloud from IPC; `subsample_points_for_ipc` tested. |
| SEC-D-05 | Preset/mask path restrictions | ☑ | `path_security.rs` shared blocklist + `validate_import_file_path`; wired in `load_preset` / `load_mask_from_path`. |
| SEC-D-06 | Settings/model ACL permissions | ☑ | `allow-settings.toml`, `allow-model.toml` granted in `capabilities/default.json`. |
| SEC-D-07 | Subprocess env allowlist | ⏸ Deferred Phase E | P2 — release `env_clear()` + allowlist; see risk acceptances. |
| SEC-D-08 | DepthAdjustmentParams bounds | ⏸ Deferred Phase E | P2 — IPC range validation for gamma/contrast/curve/feather. |
| SEC-D-09 | Export TOCTOU / atomic write | ⏸ Deferred Phase E | P2 — atomic temp+rename or `O_NOFOLLOW` (TD-06); writability probe remains. |
| SEC-D-10 | Linux load blocklist parity | ⏸ Deferred Phase E | P2 — `load_image` already uses `path_security::is_blocklisted`; full parity audit deferred. |
| SEC-D-11 | withGlobalTauri / devtools | ⏸ Deferred Phase E | P3 — release engineering (`withGlobalTauri: false`, devtools off). |
| SEC-D-12 | Export comment sanitization | ☑ | `sanitize_export_comment_field` + PLY/CSV integration tests (`export.rs`, `lib.rs`). |
| QA-D-03 | Pentest checklist | ◐ | Static code review complete (Wave 6); manual checklist in `SECURITY_PENTEST_CHECKLIST.md` — pending built-app run. |

**Risk acceptances (if any):**

| Item | Rationale | Approved by | Date |
|------|-----------|-------------|------|
| SEC-D-02 (stub hashes) | `expected_model_hashes.json` holds placeholder digests until a maintainer runs a **trusted** Hugging Face download and `python -m python.model_downloader --write-hashes`. Verification is intentionally skipped when hashes are non-hex stubs; offline threat model accepts this for Sprint D beta on trusted workstations. Real digests are Phase E / pre-release maintainer action. | _Pending_ | 2026-05-20 |
| SEC-D-07 | Python child may inherit `VIRTUAL_ENV` / `DEPTH_MODEL_PATH` on dev machines; release allowlist hardening deferred — trusted local operator, no network exfiltration path. | _Pending_ | 2026-05-20 |
| SEC-D-08 | Unbounded `DepthAdjustmentParams` over IPC could amplify DoS via huge curves; low likelihood in single-user desktop; validate in Phase E. | _Pending_ | 2026-05-20 |
| SEC-D-09 | Export uses parent canonicalize + writability probe; symlink TOCTOU window remains (TD-06). Atomic write deferred; acceptable for local export to user-selected dirs. | _Pending_ | 2026-05-20 |
| SEC-D-10 | Unix blocklist in `path_security.rs` covers `/usr`, `/bin`, `/etc`, etc.; Windows/macOS parity and edge-case audit deferred to Phase E platform pass. | _Pending_ | 2026-05-20 |
| SEC-D-11 | `withGlobalTauri: true` widens webview XSS blast radius; mitigated by CSP + offline app; release toggle deferred to Phase E build hardening. | _Pending_ | 2026-05-20 |

---

## Sign-off

**Security Specialist:** __________________ **Date:** __________  
**Senior Engineer (implementation):** __________________ **Date:** __________  

**Phase D security gate:** ☐ Pass ☑ Pass with waivers ☐ Fail  

**Gate notes (2026-05-20):** P0/P1 implementation complete or waived as above. **QA-D-03** manual pentest items (built app, ACL invoke, release WebView, engraver-adjacent paths) remain open — see `MANUAL_TEST_REPORT.md` § Security pentest (QA-D-03 — static review). Phase D product gate still requires **QA-D-01** engraver import sign-off.
