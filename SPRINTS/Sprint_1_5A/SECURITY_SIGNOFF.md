# Sprint 1.5A Security Sign-Off

**Sprint:** 1.5A — Hardening, Testing & Consultant Remediation
**Security Specialist:** Cursor-Agent-20260207-SEC (Security Specialist)
**Date:** 2026-02-07

---

## Scope

This security review covers:
1. **SEC-501:** Asset protocol scope in `tauri.conf.json` — restrict from `"**"` to minimal required paths
2. **SEC-502:** Tauri capabilities and permissions — verify minimal privilege

---

## SEC-501: Asset Protocol Scope

**Before:**
```json
"assetProtocol": {
  "enable": true,
  "scope": ["**"]
}
```
CSP: `img-src 'self' asset: http://asset.localhost blob: data:`

**Finding:** The frontend does **not** use the asset protocol. Image preview uses `previewBase64` from the `load_image` command as a data URL (`data:image/png;base64,...`) in `App.svelte`. No `convertFileSrc()` or `asset://` URLs are used anywhere in `src/`. Granting `"**"` scope contradicts the threat model (minimal privilege). **Recommendation:** Disable the asset protocol entirely.

**After:**
- `tauri.conf.json`: `assetProtocol.enable` set to `false` (scope key removed when disabled).
- CSP updated to remove asset protocol sources: `img-src 'self' blob: data:` (no `asset:` or `http://asset.localhost`).
- `src-tauri/Cargo.toml`: Removed `protocol-asset` feature from `tauri` dependency and from `[features]` default so the build matches the config (Tauri v2 requires feature/config alignment).

**Verification:**
- `cargo build --manifest-path src-tauri/Cargo.toml` — **PASS**
- `cargo test --manifest-path src-tauri/Cargo.toml` — **44 passed**, 5 ignored
- Manual smoke test (load image, generate depth, adjust sliders) should be run by team to confirm; no code paths use asset protocol.

---

## SEC-502: Capabilities Review

**Capabilities file:** `src-tauri/capabilities/default.json`

**Permissions reviewed:**

| Permission | Required? | Notes |
|------------|-----------|-------|
| `core:default` | Yes | Core Tauri capabilities for app lifecycle. |
| `shell:allow-open` | Optional | Allows opening URLs in the system default browser. No frontend code currently calls shell open (only `@tauri-apps/plugin-dialog` for file picker). Python subprocess is spawned via `std::process::Command` in Rust, not via shell plugin from frontend. **Recommendation:** Can be removed for stricter minimal privilege; keep if future "Open documentation" / external link feature is planned. |
| `dialog:allow-open` | Yes | File picker for image load; used by `ImageImport.svelte`. |
| `allow-load-image` | Yes | Command `load_image` — image loading. |
| `allow-export-stl` | Yes | Command `export_stl` — STL export. |
| `allow-generate-depth-map` | Yes | Commands: `generate_depth_map`, `get_depth_map`, `set_depth_adjustment_params`, `get_depth_adjustment_params`, `reset_depth_adjustments`. |

**Shell plugin:** `tauri-plugin-shell` is initialized in `lib.rs`. The frontend does not invoke shell spawn/execute; the Python depth estimator is run via `std::process::Command` in `python_bridge.rs`. So shell plugin is not used for subprocess. The only shell permission in use is `shell:allow-open` (open URL in browser), which is not currently used by the app. No `allow-spawn` or `allow-execute` are granted — correct for minimal privilege. **Finding:** Capabilities are minimal; optional tightening is to remove `shell:allow-open` if no external links are ever opened from the app.

**Allow-list files:** `src-tauri/permissions/allow-*.toml` each grant only the intended commands (load_image, export_stl, generate_depth_map + get/set/reset depth params). No over-grant.

---

## Sign-Off

- [x] SEC-501 reviewed and remediated (asset protocol disabled; CSP updated; Cargo.toml aligned)
- [x] SEC-502 reviewed and documented (capabilities and permissions minimal; shell:allow-open optional to remove)
- [x] No new security issues identified
- [x] Findings consistent with `docs/threat-model.md`

**Signed off by:** Security Specialist (Cursor-Agent-20260207-SEC)
**Date:** 2026-02-07
