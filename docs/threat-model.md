# SimplePicture3D — Threat Model

**Purpose:** Initial threat model per PRD §8.1; documents threats and mitigations for file I/O, subprocess, and IPC.  
**Owner:** Security Specialist  
**Last Updated:** 2026-05-20 (Sprint D — DOC-D-02)

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
| Tampered model executes malicious code | **Model security (PRD §8.2):** Download from official Hugging Face only; verify **SHA256 checksums** against known-good hashes in repo (SEC-202A, 2026-03-06). **HTTPS only** for all model downloads. Expected hashes in `python/python/expected_model_hashes.json`; see RESEARCH/architecture.md ADR-003 and `SPRINTS/Sprint_2_4/SECURITY_SIGNOFF.md`. **SEC-D-02 (Sprint D, 2026-05-20):** Stub manifest committed with `_hash_status: stub_pending_trusted_download`, documented file keys (`config.json`, `preprocessor_config.json`, `model.safetensors`), and `TODO:REPLACE_WITH_SHA256_*` placeholders. `load_expected_hashes()` skips verification until real 64-char hex digests are written via `python -m python.model_downloader --write-hashes` after a trusted maintainer download. **Follow-up (Phase E):** populate real digests; enforcement then fails closed on mismatch. |
| Model run with excessive privileges | **Python subprocess isolation:** Run depth estimator in child process with minimal privileges; no arbitrary filesystem access beyond models directory and designated temp paths. |
| Supply-chain compromise of model provider | Document source and checksum in RESEARCH. Expected SHA256 from trusted source (repo); hashes in `expected_model_hashes.json`. SEC-202A plumbing complete; Sprint D stub documents intent pending maintainer digests. |

### 2.3 Path traversal and export path abuse

| Threat | Mitigation |
|--------|------------|
| User export path overwrites system files (e.g. `/etc/passwd`, Windows system dirs) | **Canonicalize paths;** reject paths outside user-selected export directory or blocklisted system locations. PRD §8.2: “Canonicalize, prevent directory traversal.” |
| Malicious path in IPC (e.g. `load_image`, `export_ply`) | **Input validation:** Validate and canonicalize all file paths in Tauri commands; reject path traversal sequences (`..`, symlinks outside allowed tree). |
| Symlink attacks (e.g. export to symlink pointing to sensitive file) | Resolve symlinks and enforce same canonicalization rules; or disallow overwriting through symlinks. |
| Arbitrary file read via preset/mask import paths | **SEC-D-05:** `load_preset` and `load_mask_from_path` call `path_security::validate_import_file_path` — canonicalize, require `.json`, reject blocklisted system dirs (shared policy with export). |

**Implementation (load_image path validation, SEC-101):** For `load_image`, the path comes from the user via file picker or drag-and-drop (Tauri dialog or frontend drop). (1) **Canonicalize** the path with `std::path::Path::canonicalize()` (or platform equivalent) so `..` and symlinks are resolved. (2) **Allowlist:** Accept only paths that resolve to a regular file under a user-accessible location; paths from the system file dialog or drop are inherently user-chosen. (3) **Blocklist:** Reject paths under system-sensitive directories via `path_security::is_blocklisted()` (Windows: `C:\Windows`, `C:\Program Files`, etc.; Unix: `/etc`, `/usr`, `/bin`, `/sbin`, `/proc`, `/sys`, etc.). Reject if canonicalization fails or path does not exist. Do not read before canonicalization. See `docs/security-checklist.md` §2.2 and Sprint 1.1 Security handover (path validation).

### 2.4 File I/O and input validation

| Threat | Mitigation |
|--------|------------|
| Non-image files (e.g. executable) passed as “image” | **Validate image magic bytes** before decode; reject executable formats. PRD §8.2: “Validate magic bytes, reject executable formats.” |
| Malformed or oversized images causing DoS | Enforce max dimensions (e.g. 8K) and size limits; decode in bounded memory; consider timeouts. **SEC-D-03 (Sprint D):** `image_loading::read_image_bytes_for_depth` enforces `MAX_IMAGE_FILE_BYTES = 128 MiB` (via `fs::metadata` before reading) and `MAX_DIMENSION = 8192` (via `ImageReader::into_dimensions`, header-only — no pixel decode) **before** the Python subprocess is spawned. The same 128 MiB file cap also applies to `load_image_impl`. Rationale: photographic 8K PNG is typically <50 MB, JPEG <15 MB; 128 MiB leaves headroom for high-bit-depth/noisy 8K content while bounding worst-case RAM and Python decode work. |
| User text (filenames, presets) used in shell or file ops | **Sanitize filenames;** escape special characters; avoid passing user-controlled strings to shell. PRD §8.2: “Sanitize filenames, escape special characters.” **SEC-D-12 (Sprint D):** Export metadata comments (`source_image`, blank summary) pass through `export::sanitize_export_comment_field()` — basename only; strip newlines and control characters — in PLY/CSV writers (`src-tauri/src/export.rs`). Rust tests: `export_commands_sanitize_source_image_filename` in `lib.rs`. |

**Implementation (image magic bytes, SEC-102):** Before decoding image data (e.g. with the `image` crate), **validate magic bytes** from the first bytes of the file. Accept only: **PNG** — `89 50 4E 47 0D 0A 1A 0A`; **JPEG** — `FF D8 FF` (SOI). Reject any other signature (including executables, PDF, etc.) with a clear error (e.g. “Unsupported or invalid image format”). Perform this check on the raw file bytes before calling the decoder; do not rely on file extension. Documented in this threat model and `docs/security-checklist.md` §2.2.

### 2.5 Subprocess (Python) security

| Threat | Mitigation |
|--------|------------|
| **Command injection** in Python invocation | Do not build command line from unsanitized user input. Pass image via **temp file path** or stdin; use fixed CLI contract (e.g. `--input`, `--output`). RESEARCH/tauri.md: use shell plugin/sidecar with fixed arguments. |
| Python process reading/writing outside allowed dirs | Restrict subprocess: only allow access to app temp dir, models dir, and explicitly passed input/output paths. No shell; minimal env. |
| Malicious input (e.g. path) in args to sidecar | Validate and canonicalize any paths passed to the subprocess; use allowlists, not blocklists. |

**Review (SEC-201):** When the Python subprocess spawner is implemented (BACK-201, BACK-202), Security Specialist must review: (1) **No user-controlled argv** — command and arguments are fixed or derived only from validated paths (e.g. temp path from `file_io::write_temp_file`); no frontend or user string in `Command::arg()`. (2) **Path validation** — any path passed to the subprocess (e.g. image temp file) must be canonicalized and restricted to app temp dir or allowlisted locations (same pattern as `file_io::read_file_in_temp_dir`). (3) **No shell** — use `std::process::Command` (or Tauri shell with fixed scope); do not invoke via `cmd /c` or `sh -c` with user input. (4) Document findings in `docs/security-checklist.md` §2.2 and this threat model.

**SEC-201 completed 2026-02-03:** `src-tauri/src/python_bridge.rs` reviewed. No user input in argv; path from `file_io::write_temp_file` and `validate_input_path` (canonicalized, under temp dir); no shell. See `docs/security-checklist.md` §2.2 (SEC-201 Review).

**SEC-D-07 (Sprint D, 2026-05-24):** `apply_hardened_subprocess_env()` calls `Command::env_clear()` then copies only an allowlisted set of variables (`ALLOWED_SUBPROCESS_ENV` in `python_bridge.rs`): on Unix `PATH` and `HOME`; on Windows `PATH`, `PATHEXT`, `SYSTEMROOT`, `WINDIR`, `USERPROFILE`, `HOMEDRIVE`, `HOMEPATH`, `TEMP`, `TMP`. **`VIRTUAL_ENV`**, **`DEPTH_MODEL_PATH`**, and other parent env are **not** passed through. Stub depth uses explicit `--no-model` via `stub_depth_cli_flag()` (parent `SP3D_USE_STUB` mapped at spawn time), not subprocess env inheritance.

### 2.6 IPC (Tauri commands) surface

| Threat | Mitigation |
|--------|------------|
| Frontend invoking commands with arbitrary or oversized payloads | **Limit exposed commands** to those in capability config (`capabilities/default.json`). **SEC-D-01 (Sprint D):** Split monolithic `allow-generate-depth-map` into four least-privilege permission TOMLs (see table below). **SEC-D-06:** `allow-settings.toml` and `allow-model.toml` for settings and model commands. **ARCH-D-02:** 36 registered commands ↔ 36 frontend wrappers ↔ union of permission TOMLs — zero orphans (`SPRINTS/Sprint_D/ARCH-D-02_COMMAND_AUDIT.md`). |
| Large payloads over IPC (e.g. depth map, point cloud) | **SEC-D-04:** `generate_point_cloud` caps IPC responses at `MAX_IPC_POINT_CLOUD_POINTS` (1_000_000). Preview mode subsamples; export mode omits points from IPC while caching the full cloud server-side for `export_ply`/`export_xyz`/`export_csv`. Hard generation cap: `MAX_POINT_CLOUD_GENERATION` (8_000_000). Constants in `src-tauri/src/volumetric.rs`. |
| Arbitrary file read via preset/mask import | **SEC-D-05:** `load_preset` and `load_mask_from_path` use `path_security::validate_import_file_path` — canonicalize, `.json` extension, unified `is_blocklisted()` policy (see §2.3). |
| Deserialization or validation bugs in command args | Validate and bound all arguments (paths, options); use strong types and fail closed on invalid input. **SEC-D-08 (Phase E):** bound `DepthAdjustmentParams` from IPC (see §2.8). |

**SEC-D-01 permission groups (Sprint D):**

| Permission ID | Commands |
|---------------|----------|
| `allow-depth-inference` | `generate_depth_map` (Python subprocess — isolated) |
| `allow-depth-read` | `get_depth_map`, `get_depth_histogram` |
| `allow-depth-adjust` | `set_depth_adjustment_params`, `get_depth_adjustment_params`, undo/redo/history, `reset_depth_adjustments` |
| `allow-mask` | `get_mask`, `set_mask`, `set_mask_region`, `clear_mask`, `save_mask_to_path`, `load_mask_from_path` |

Other groups unchanged: `allow-load-image`, `allow-point-cloud`, `allow-presets`, `allow-settings`, `allow-model`. All referenced in `src-tauri/capabilities/default.json`.

### 2.7 Engraver volume guidance (UI-D-01)

| Threat | Mitigation |
|--------|------------|
| User exports geometry that exceeds xTool F2 Ultra UV processing volume | **Informational only (frontend):** `src/lib/f2VolumeWarnings.ts` and `F2VolumeWarnings.svelte` show non-blocking exceed/near-edge alerts in Export panel and 3D preview when blank or etch interior approaches charter limits (70×70×150 mm processing volume; 5 mm clearance). Does **not** block IPC, export, or generation. Constants must stay aligned with `docs/engraver-charter.md`. Vitest: `src/lib/__tests__/f2VolumeWarnings.test.ts`. |

### 2.8 Deferred hardening (Phase E — SEC-D-07 through SEC-D-11)

Sprint D closed P0/P1 items appropriate for trusted local desktop use. The following remain **deferred to Phase E** (release engineering / lower residual risk for single-user offline app):

| Task | Threat | Rationale for deferral |
|------|--------|------------------------|
| **SEC-D-07** | Python child inherits `VIRTUAL_ENV`, `DEPTH_MODEL_PATH`, and full parent env | Dev workflow depends on venv discovery (`python_bridge.rs`). Release `env_clear()` + allowlist needs signed-release validation on Windows/macOS/Linux; no remote attacker in offline model. |
| **SEC-D-08** | Unbounded `DepthAdjustmentParams` over IPC (local DoS / logic abuse) | Affects only in-process undo state; bounded by image dimensions already. Parameter caps are polish, not gate-critical. |
| **SEC-D-09** | Export path TOCTOU (parent-only canonicalize + writability probe) | Local user chooses export path via dialog; symlink race is low likelihood. Atomic write (`temp` + rename) or `O_NOFOLLOW` is release-hardening (extends TD-06). |
| **SEC-D-10** | Linux `load_image` blocklist narrower than export | **Partial:** `path_security::is_blocklisted()` now shared by load, export, and import (SEC-D-05). Formal parity audit and any residual gaps → Phase E. |
| **SEC-D-11** | `withGlobalTauri: true` widens webview XSS → IPC blast radius | Requires release bundle config (`tauri.conf.json`), devtools disabled in production, and regression pass; tracked under Phase E release engineering (`ROADMAP.md`). |

**SEC-D-02 follow-up (maintainer, not Phase E code gate):** Replace stub SHA256 placeholders in `expected_model_hashes.json` after trusted model download.

### 2.9 Dependency vulnerabilities

| Threat | Mitigation |
|--------|------------|
| Exploitable bugs in Rust, npm, or Python deps | **Dependency management (PRD §8.2):** `cargo audit` and `npm audit` (and `pip-audit`) in CI; Dependabot/lock file committed; pin versions. See SEC-002 (CI audits) and SEC-003 (checklist). |

---

## 3. Summary Table

| Area | Key threats | Key mitigations |
|------|-------------|-----------------|
| **File I/O** | Path traversal, overwriting system files, malicious symlinks, export comment injection | Canonicalize paths; `path_security` blocklist; magic bytes; SEC-D-12 comment sanitization |
| **Subprocess** | Command injection, excessive privileges, wrong paths | Fixed CLI; temp file I/O; restrict filesystem; no user-controlled shell args |
| **IPC** | Overly broad or unsafe commands; large/unvalidated payloads | Least-privilege permission TOMLs (SEC-D-01/06); IPC point caps (SEC-D-04); import path policy (SEC-D-05) |
| **Models** | Tampered or malicious models | Official source only; SHA256 manifest stub (SEC-D-02); enforcement when digests populated; subprocess isolation |
| **Engraver UX** | Oversized blank vs machine volume | UI-D-01 informational warnings (non-blocking) |
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
- **SPRINTS/Sprint_D/SECURITY_REVIEW_2026-05.md** — Sprint D static review findings
- **SPRINTS/Sprint_D/ARCH-D-02_COMMAND_AUDIT.md** — Command ↔ ACL ↔ frontend audit

---

**Document Version:** 1.4  
**Status:** Sprint D (DOC-D-02, 2026-05-20): SEC-D-01 through SEC-D-06 and SEC-D-12 implemented; UI-D-01 documented in §2.7. SEC-D-07 through SEC-D-11 deferred to Phase E (§2.8). SEC-D-02 stub hash manifest documented in §2.2; real digests pending maintainer action. See `SPRINTS/Sprint_D/SECURITY_REVIEW_2026-05.md`, `ARCH-D-02_COMMAND_AUDIT.md`, `docs/security-checklist.md` §2.2.
