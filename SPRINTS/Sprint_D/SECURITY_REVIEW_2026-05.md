# Security Review — 2026-05-19

**Scope:** Full codebase static review (Rust/Tauri, Svelte, Python).  
**Reviewer:** Security Specialist (agent session)  
**Tracked in:** `todo.md` Sprint D — tasks `SEC-D-02` through `SEC-D-12`, `QA-D-03`  
**Threat model:** `docs/threat-model.md`

---

## Executive summary

SimplePicture3D is **offline-first** with **strong subprocess hygiene** (SEC-201). Residual risk is mainly **local DoS** (large IPC payloads), **model integrity not enforced in practice** (empty hash file), and **path policy gaps** on preset/mask import. No telemetry or cloud exfiltration found in application code.

**Posture:** Appropriate for trusted local use; not hardened for hostile multi-tenant scenarios.

---

## Strengths

| Area | Evidence |
|------|----------|
| Python subprocess | `python_bridge.rs` — no shell, fixed argv, temp-dir path validation |
| Image load | Magic bytes (SEC-102), 8K downsample on `load_image`, path blocklist (SEC-101) |
| Export | `validate_export_path` — canonicalize parent, extension, blocklist, writability probe |
| Privacy | No `fetch`/analytics in `src/` |
| CI | `cargo audit`, `npm audit --audit-level=high`, `pip-audit` in `.github/workflows/ci.yml` |
| CSP | `default-src 'self'` in `tauri.conf.json`; `assetProtocol` disabled |

---

## Findings → Sprint tasks

| ID | Severity | Finding | Task |
|----|----------|---------|------|
| H1 | High | `expected_model_hashes.json` has empty `hashes` — SHA256 skipped | SEC-D-02 |
| H2 | High | Unbounded IPC (depth map, mask `Vec<bool>`, full point cloud) | SEC-D-03, SEC-D-04 |
| H3 | High | `withGlobalTauri: true` amplifies webview XSS → IPC | SEC-D-11 |
| M1 | Medium | `load_preset` arbitrary absolute path read | SEC-D-05 |
| M2 | Medium | `load_mask_from_path` no export-style blocklist | SEC-D-05 |
| M3 | Medium | Export path parent-only canonicalize (TOCTOU) | SEC-D-09 (TD-06) |
| M4 | Medium | Linux `load_image` blocklist incomplete | SEC-D-10 |
| M5 | Medium | `get_settings` / `download_model` not in ACL permission files | SEC-D-06 |
| M6 | Medium | `VIRTUAL_ENV` selects Python interpreter | SEC-D-07 |
| M7 | Medium | Child inherits `DEPTH_MODEL_PATH` | SEC-D-07 |
| M8 | Medium | Unbounded `DepthAdjustmentParams` from IPC | SEC-D-08 |
| L1–L5 | Low | JPEG magic minimal, shell open, dev deps, preset schema, PLY comments | SEC-D-12, track Vite in deps |

---

## Prioritized remediation (same as `todo.md`)

1. **P0:** SEC-D-02 (model hashes), SEC-D-03 (max image size)
2. **P1:** SEC-D-04 (IPC caps), SEC-D-05 (preset/mask paths), SEC-D-06 (ACL)
3. **P2:** SEC-D-07–SEC-D-10 (env, params, export, blocklist)
4. **P3:** SEC-D-11, QA-D-03 (pentest checklist)

---

## Manual verification (QA-D-03)

See `SECURITY_PENTEST_CHECKLIST.md`.

---

**Status:** Findings logged; implementation pending Sprint D execution.
