# Security Sign-off — Sprint D

**Sprint:** D — Validate and Harden  
**Owner:** Security Specialist  
**Reference:** `SPRINTS/Sprint_D/SECURITY_REVIEW_2026-05.md`, `docs/threat-model.md`, `docs/security-checklist.md`  
**Date:** 2026-05-24

---

## Scope

Sprint D security tasks **SEC-D-02** through **SEC-D-12**, plus permission work **SEC-D-01** / **SEC-D-06**, and manual verification **QA-D-03**.

---

## Outcome table

| Task | Description | Status | Notes |
|------|-------------|--------|-------|
| SEC-D-02 | Model SHA256 enforcement | ☐ | **Stub hashes pending trusted download** — `expected_model_hashes.json` still has `TODO:REPLACE_WITH_SHA256_*`; runtime verification skipped; CI emits `::warning::` until real digests committed |
| SEC-D-03 | Max image size before Python | ✅ | `image_loading.rs`: 128 MiB file cap + 8192 header dimension cap before subprocess |
| SEC-D-04 | IPC payload caps (point cloud) | ✅ | `volumetric.rs` / `lib.rs`: 1M IPC preview, 8M generation cap |
| SEC-D-05 | Preset/mask path restrictions | ✅ | `path_security.rs` + `validate_import_file_path` in `lib.rs` |
| SEC-D-06 | Settings/model ACL permissions | ✅ | `allow-settings.toml`, `allow-model.toml` |
| SEC-D-07 | Subprocess env allowlist | ✅ | `python_bridge.rs`: `env_clear` + `ALLOWED_SUBPROCESS_ENV` |
| SEC-D-08 | DepthAdjustmentParams bounds | ✅ | `depth_adjust.rs`: `validate_depth_adjustment_params` |
| SEC-D-09 | Export TOCTOU / atomic write | ✅ | `export.rs`: `write_export_atomically` |
| SEC-D-10 | Linux load blocklist parity | ✅ | `path_security::is_blocklisted` shared with load/export |
| SEC-D-11 | withGlobalTauri / devtools | ✅ | `tauri.conf.json`: `withGlobalTauri: false` |
| SEC-D-12 | Export comment sanitization | ✅ | `export.rs`: `sanitize_export_comment_field` |
| SEC-E-01 | path_security imports in lib.rs | ✅ | Fixed 2026-05-24; build/tests green |
| SEC-E-02 | Depth map IPC caps | ✅ | `depth_adjust.rs`: `MAX_IPC_DEPTH_PIXELS` subsampling |
| QA-D-03 | Pentest checklist | ☐ | See `SECURITY_PENTEST_CHECKLIST.md` — manual pass pending |

**SEC-D-01:** ✅ Split into `allow-depth-map` + `allow-mask` (see `ARCH-D-02_AUDIT.md`).

**Risk acceptances (if any):**

| Item | Rationale | Approved by | Date |
|------|-----------|-------------|------|
| SEC-D-02 stub hashes | CI cannot download HF models; verification skipped until maintainer runs trusted download + `--write-hashes` | Security Specialist (documented) | 2026-05-24 |

---

## Sign-off

**Security Specialist:** _Gerry / agent session_ **Date:** 2026-05-24  
**Senior Engineer (implementation):** _Pending human counter-sign_ **Date:** __________  

**Phase D security gate:** ☐ Pass ☐ Pass with waivers ☑ **Pass with waivers** (SEC-D-02 stub hashes, QA-D-03 manual pentest pending) ☐ Fail
