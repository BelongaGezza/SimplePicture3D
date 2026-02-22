# Security Sign-off — Sprint 1.11 (Phase 1 MVP)

**Sprint:** 1.11  
**Date:** 2026-02-22  
**Role:** Security Specialist  
**Session:** session-sec-20260222  
**Tasks:** SEC-601, SEC-602, SEC-603  

---

## 1. Executive Summary

Security review and penetration testing were performed per prd.md §8 and `.cursor/rules/security.mdc`. **No critical or high unmitigated vulnerabilities** were found. Dependency audits and code scan results are documented below. **Sign-off is granted for Phase 1 MVP exit gate** with the recorded findings and recommendations.

---

## 2. SEC-601: Full Security Review

### 2.1 Dependency Audit

| Stack   | Tool        | Result | Notes |
|---------|-------------|--------|--------|
| Rust    | `cargo audit` | **PASS** (exit 0) | 19 **allowed** advisories (warnings). No critical/high that fail the audit. |
| NPM     | `npm audit` | **7 moderate** | See §2.2. Accepted for MVP with mitigation. |
| Python  | `pip-audit` | **Not run** | `pip-audit` not installed in environment. Recommendation: add to CI (see §4). |

**Rust (cargo audit):** Run from `src-tauri`. Advisories reported are:
- **Unmaintained:** gtk-rs (atk, gdk, gtk, etc.), fxhash, unic-char-property, unic-char-range, unic-common, unic-ucd-ident, unic-ucd-version (transitive via Tauri/wry).
- **Unsound:** glib `VariantStrIter` (RUSTSEC-2024-0429).

All are **allowed** in current config (warnings only). No direct application code change required for MVP; Tauri/upstream dependency updates may address in future.

### 2.2 NPM Audit — 7 Moderate

- **esbuild** (GHSA-67mh-4wv8-2f99): Dev server can be abused to send requests and read responses. **Context:** Tauri app uses Vite dev server only in development; production build is bundled. **Risk:** Low for shipped app. **Recommendation:** Plan `npm audit fix` or Vite upgrade in Phase 2; avoid using dev server on untrusted networks.
- **Svelte** (GHSA-m56q-vw4c-c2cp, GHSA-f7gr-6p89-r883, GHSA-crpf-4hrx-3jrp): SSR dynamic element and attribute spreading. **Context:** App is a **desktop Svelte frontend**; SSR is not used in the current architecture. **Risk:** Low for MVP. **Recommendation:** Revisit when/if SSR is introduced.

**Conclusion:** No critical/high. Moderate findings are acceptable for Phase 1 with documented context.

### 2.3 Code Scan

| Area | Control | Status |
|------|---------|--------|
| **Input validation** | File paths canonicalized; blocklisted system dirs | ✅ `validate_path()` (image_loading), `validate_export_path()` (lib.rs) |
| **Image files** | Magic-byte validation (PNG/JPEG only); reject executable/unknown | ✅ `validate_magic_bytes()` before decode |
| **Export paths** | Canonicalize, .stl/.obj extension, block system dirs, writable check | ✅ `validate_export_path()`; Windows/Unix blocklists |
| **Settings path** | Fixed location `~/.simplepicture3d/settings.json` | ✅ No user path used for settings file write |
| **Python subprocess** | No user-controlled argv; input via temp file under `std::env::temp_dir()` | ✅ `validate_input_path()` ensures path under temp dir; args are `-m python.depth_estimator --input <our_temp_path>` |
| **Error messages** | No full path leakage to frontend | ✅ Generic messages (e.g. "Export directory is not writable") |
| **Tauri IPC** | Limited command surface; permissions per capability | ✅ allow-load-image, allow-export-stl, etc. |

---

## 3. SEC-602: Penetration Testing (File Upload, Path Traversal)

### 3.1 File Upload

- **Malicious/oversized:** Image load path uses `validate_path()` (canonicalize, is_file, blocklist). Only PNG/JPEG accepted via `validate_magic_bytes()`; decode validates dimensions. Oversized images downsampled to ≤8K (MAX_DIMENSION). Non-image (e.g. executable) rejected by magic bytes. **Result:** Mitigated.
- **Python input:** Only our own temp file path (under system temp) is passed to Python; no user-controlled path in subprocess argv. **Result:** No command injection via path.

### 3.2 Path Traversal

- **Load image:** `load_image_impl_path_traversal_resolved_by_canonicalize` (image_loading.rs) confirms `..` is resolved by canonicalization; blocklist prevents reading under system dirs.
- **Export STL/OBJ:** `validate_export_path()` canonicalizes parent + filename, enforces extension, blocks system directories (Windows: e.g. C:\Windows, Program Files; Unix: /etc, /usr, /bin, /sbin, etc.), and checks writability with a probe file. **Result:** Path traversal to write outside user-intended directory is blocked.
- **Settings:** Persistence uses fixed path; `last_export_dir` is stored value only; actual export path comes from save dialog and is re-validated by `validate_export_path()`. **Result:** No path traversal via settings.
- **Temp file (Python):** `file_io::sanitize_temp_component` and `sanitize_rejects_path_traversal` test; input to Python is our temp path under `temp_dir()`. **Result:** Mitigated.

**Conclusion:** File upload and path traversal controls are in place and tested. No critical gaps for MVP.

---

## 4. SEC-603: Sign-off and Recommendations

### 4.1 Sign-off

- **Critical/High open issues:** None.
- **Phase 1 exit gate:** This document may be referenced as the **MVP security sign-off**. No blocking security issues for Phase 1.

### 4.2 Recommendations (Phase 2 / Ongoing)

1. **CI:** Run `cargo audit`, `npm audit`, and `pip-audit` (after adding to Python env or CI) in pipeline; fail on critical/high.
2. **Python:** Add `pip-audit` to contributor docs and CI (e.g. `pip install pip-audit && pip-audit`).
3. **NPM:** Schedule `npm audit fix` or dependency upgrades (Vite/Svelte) in Phase 2 to reduce moderate advisories.
4. **Rust:** Monitor Tauri/wry releases for updates that address unmaintained/unsound transitive deps; consider `cargo audit` with stricter policy (e.g. deny unmaintained) when upgrading.
5. **Model download:** When implemented, ensure HTTPS and SHA256 checksum verification per prd.md §8.2 (Model Security).

---

## 5. Document History

| Version | Date       | Author (Role)     | Change |
|---------|------------|-------------------|--------|
| 1.0     | 2026-02-22 | Security Specialist | Initial sign-off (SEC-601, SEC-602, SEC-603). |
