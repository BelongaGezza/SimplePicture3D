# SimplePicture3D — Threat Model

**Purpose:** Initial threat model per PRD §8.1; documents threats and mitigations for file I/O, subprocess, and IPC.  
**Owner:** Security Specialist  
**Last Updated:** 2026-02-01

---

## 1. Assets to Protect (PRD §8.1)

| Asset | Description |
|-------|-------------|
| **User images** | Potentially personal photos, proprietary artwork; must not leave the device |
| **User data** | Export paths, preferences, presets (e.g. under `~/.simplepicture3d/`) |
| **Application integrity** | Prevent malware injection, tampering of app or AI models |

---

## 2. Threats & Mitigations

### 2.1 Privacy violation (user images leaked)

| Threat | Mitigation |
|--------|------------|
| User images sent to third parties or cloud | **100% offline processing.** No cloud APIs; depth and mesh computed locally. Document in privacy notice (PRD §8.3). |
| Telemetry or analytics exfiltrating image data | No telemetry without explicit consent; no ads/tracking. Code is open source for audit. |
| Logs or temp files containing image data | Avoid logging image bytes or paths that reveal user content; clear temp files after use; restrict log retention. |

### 2.2 Malicious or tampered AI models

| Threat | Mitigation |
|--------|------------|
| Tampered model executes malicious code | **Model security (PRD §8.2):** Download from official Hugging Face only; verify **SHA256 checksums** against known-good hashes before use. |
| Model run with excessive privileges | **Python subprocess isolation:** Run depth estimator in child process with minimal privileges; no arbitrary filesystem access beyond models directory and designated temp paths. |
| Supply-chain compromise of model provider | Document source and checksum in RESEARCH; consider pinning to specific release artifacts. |

### 2.3 Path traversal and export path abuse

| Threat | Mitigation |
|--------|------------|
| User export path overwrites system files (e.g. `/etc/passwd`, Windows system dirs) | **Canonicalize paths;** reject paths outside user-selected export directory or blocklisted system locations. PRD §8.2: “Canonicalize, prevent directory traversal.” |
| Malicious path in IPC (e.g. `load_image`, `export_stl`) | **Input validation:** Validate and canonicalize all file paths in Tauri commands; reject path traversal sequences (`..`, symlinks outside allowed tree). |
| Symlink attacks (e.g. export to symlink pointing to sensitive file) | Resolve symlinks and enforce same canonicalization rules; or disallow overwriting through symlinks. |

**Implementation (load_image path validation, SEC-101):** For `load_image`, the path comes from the user via file picker or drag-and-drop (Tauri dialog or frontend drop). (1) **Canonicalize** the path with `std::path::Path::canonicalize()` (or platform equivalent) so `..` and symlinks are resolved. (2) **Allowlist:** Accept only paths that resolve to a regular file under a user-accessible location; paths from the system file dialog or drop are inherently user-chosen. (3) **Blocklist:** Reject paths that resolve under system-sensitive directories (e.g. Windows: `C:\Windows\System32`, `C:\Program Files`; macOS: `/System`, `/usr/bin`; Linux: `/usr/bin`, `/etc`). Reject if canonicalization fails or path does not exist. Do not read before canonicalization. See `docs/security-checklist.md` §2.2 and Sprint 1.1 Security handover (path validation).

### 2.4 File I/O and input validation

| Threat | Mitigation |
|--------|------------|
| Non-image files (e.g. executable) passed as “image” | **Validate image magic bytes** before decode; reject executable formats. PRD §8.2: “Validate magic bytes, reject executable formats.” |
| Malformed or oversized images causing DoS | Enforce max dimensions (e.g. 8K) and size limits; decode in bounded memory; consider timeouts. |
| User text (filenames, presets) used in shell or file ops | **Sanitize filenames;** escape special characters; avoid passing user-controlled strings to shell. PRD §8.2: “Sanitize filenames, escape special characters.” |

**Implementation (image magic bytes, SEC-102):** Before decoding image data (e.g. with the `image` crate), **validate magic bytes** from the first bytes of the file. Accept only: **PNG** — `89 50 4E 47 0D 0A 1A 0A`; **JPEG** — `FF D8 FF` (SOI). Reject any other signature (including executables, PDF, etc.) with a clear error (e.g. “Unsupported or invalid image format”). Perform this check on the raw file bytes before calling the decoder; do not rely on file extension. Documented in this threat model and `docs/security-checklist.md` §2.2.

### 2.5 Subprocess (Python) security

| Threat | Mitigation |
|--------|------------|
| **Command injection** in Python invocation | Do not build command line from unsanitized user input. Pass image via **temp file path** or stdin; use fixed CLI contract (e.g. `--input`, `--output`). RESEARCH/tauri.md: use shell plugin/sidecar with fixed arguments. |
| Python process reading/writing outside allowed dirs | Restrict subprocess: only allow access to app temp dir, models dir, and explicitly passed input/output paths. No shell; minimal env. |
| Malicious input (e.g. path) in args to sidecar | Validate and canonicalize any paths passed to the subprocess; use allowlists, not blocklists. |

### 2.6 IPC (Tauri commands) surface

| Threat | Mitigation |
|--------|------------|
| Frontend invoking commands with arbitrary or oversized payloads | **Limit exposed commands** to those in capability config (e.g. `capabilities/default.json`). PRD and RESEARCH/tauri.md: only expose `load_image`, `export_stl`, etc. No generic “run script” or “read any file.” |
| Large payloads over IPC (e.g. depth map, mesh) | Prefer **file-based or chunked transfer** (RESEARCH/tauri.md): e.g. Rust writes to temp file, frontend receives path or chunked data; avoid multi-MB JSON over IPC. |
| Deserialization or validation bugs in command args | Validate and bound all arguments (paths, options); use strong types and fail closed on invalid input. |

### 2.7 Dependency vulnerabilities

| Threat | Mitigation |
|--------|------------|
| Exploitable bugs in Rust, npm, or Python deps | **Dependency management (PRD §8.2):** `cargo audit` and `npm audit` (and `pip-audit`) in CI; Dependabot/lock file committed; pin versions. See SEC-002 (CI audits) and SEC-003 (checklist). |

---

## 3. Summary Table

| Area | Key threats | Key mitigations |
|------|-------------|-----------------|
| **File I/O** | Path traversal, overwriting system files, malicious symlinks | Canonicalize paths; blocklist/allowlist; validate magic bytes for images |
| **Subprocess** | Command injection, excessive privileges, wrong paths | Fixed CLI; temp file I/O; restrict filesystem; no user-controlled shell args |
| **IPC** | Overly broad or unsafe commands; large/unvalidated payloads | Minimal command set; capability-based permissions; file/chunk for large data |
| **Models** | Tampered or malicious models | Official source only; SHA256 checksums; subprocess isolation |
| **Dependencies** | Known CVEs in libraries | cargo audit, npm audit, pip-audit in CI; pinned deps |

---

## 4. CI dependency audit policy (SEC-002)

| Tool | When it runs | Policy |
|------|----------------|--------|
| **npm audit** | On every push/PR (`.github/workflows/ci.yml`) | **Fail** if any vulnerability is high or critical (`--audit-level=high`). Moderate/low do not fail the job but should be triaged. |
| **cargo audit** | Same workflow, backend job | **Fail** if any known Rust advisory applies to dependencies. Fix or upgrade before merge. |

- Both audits run in CI; no merge with failing audits.
- To change policy (e.g. warn instead of fail), update the workflow and this section.

---

## 5. References

- **prd.md** §8 (Security & Privacy), §8.1 (Threat Model), §8.2 (Security Controls)
- **RESEARCH/tauri.md** — IPC, shell plugin, sidecar, large-payload guidance
- **docs/architecture.md** — Data flow and component boundaries
- **.github/workflows/ci.yml** — CI jobs that run npm audit and cargo audit

---

**Document Version:** 1.1  
**Status:** Initial review (Sprint 1.1, SEC-001); image loading implementation notes added (Sprint 1.2, SEC-101, SEC-102).
