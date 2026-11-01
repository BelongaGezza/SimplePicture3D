# Security Checklist — Dependency & Code Review

**Purpose:** Reusable checklist for adding dependencies and for release; covers dependency audit and review steps.  
**Reference:** prd.md §8 (Security & Privacy), docs/threat-model.md  
**Use:** Before adding a crate/npm/pip package; before release; per sprint if security review is in scope.  
**Last Updated:** 2026-05-20 (Sprint D — DOC-D-02)

---

## 1. Before adding a new dependency

Use this when adding a **Rust crate**, **npm package**, or **Python package**.

### 1.1 License

- [ ] License is **MIT**, **Apache-2.0**, or **BSD** (or otherwise compatible with project MIT license).
- [ ] **No GPL/AGPL** (project must remain MIT-compatible).
- [ ] License text and attribution noted (e.g. in NOTICE or README) if required.

### 1.2 Supply chain & audit

- [ ] **Rust:** Run `cargo audit` (or `cargo audit --dry-run` with the new crate). No known advisories for the new dependency.
- [ ] **npm:** Run `npm audit` after adding. No high/critical vulnerabilities introduced.
- [ ] **Python:** Run `pip-audit` (or `pip install pip-audit` then `pip-audit`). No known CVEs.
- [ ] Prefer pinned versions / lock file (Cargo.lock, package-lock.json, requirements.txt with versions).

### 1.3 Scope and necessity

- [ ] Dependency is necessary (no duplicate functionality already in tree).
- [ ] Minimal scope: prefer small, focused crates/packages over large meta-packages where possible.
- [ ] No unnecessary native/shell execution or broad filesystem/network access implied by the dependency.

### 1.4 Documentation

- [ ] Add/update any third-party notices if the license requires (see prd.md §9.3).
- [ ] If RESEARCH file exists for the stack (e.g. rust-crates.md, frontend.md), note the new dependency there if relevant.

---

## 2. Before release (and in phase gates)

Use this as part of the **Release Checklist** (todo.md) and phase gates.

### 2.1 Dependency audits (all stacks)

- [ ] **Rust:** `cargo audit` passes (no known vulnerabilities in src-tauri).
- [ ] **npm:** `npm audit` passes at project policy level (high/critical = fail; see docs/threat-model.md).
- [ ] **Python:** `pip-audit` passes in project venv (if python/ is used).
- [ ] Lock files are committed (Cargo.lock, package-lock.json, etc.).

### 2.2 Security review

- [ ] Threat model (docs/threat-model.md) reviewed; no new unmitigated threats for this release.
- [ ] No critical/high open security issues in the issue tracker for this release scope.
- [ ] Tauri IPC: only intended commands exposed; capability config (e.g. capabilities/default.json) reviewed.
- [ ] File I/O and export paths: path canonicalization and traversal checks in place (see threat model §2.3, §2.4). For `load_image`: canonicalize path; blocklist system dirs; allow only user-chosen file paths (SEC-101).
- [ ] Image loading: magic-byte validation (PNG/JPEG) before decode; non-image files rejected with clear error (threat model §2.4, SEC-102).
- [ ] **Python subprocess (SEC-201):** No user-controlled command-line args; temp file I/O only as designed (threat model §2.5). Review: (a) argv built only from fixed strings or validated paths (e.g. from `file_io`); (b) any path passed to subprocess canonicalized and under temp dir or allowlist; (c) no shell invocation with user input; (d) findings documented.
- [x] **Model download (SEC-202 / SEC-D-02):** HTTPS only; SHA256 verification when `expected_model_hashes.json` contains real 64-char hex digests (not `TODO:*` placeholders). **Sprint D (2026-05-20):** Stub manifest committed (`_hash_status: stub_pending_trusted_download`); `load_expected_hashes()` skips enforcement until maintainer runs `python -m python.model_downloader --write-hashes` after trusted download. See threat model §2.2 and `python/tests/test_model_downloader.py`.

#### Sprint D verification (2026-05-20 — DOC-D-02)

Use when signing off Phase D / Sprint D scope (`SPRINTS/Sprint_D/SECURITY_SIGNOFF.md`). Evidence: `SPRINTS/Sprint_D/SECURITY_REVIEW_2026-05.md`, `ARCH-D-02_COMMAND_AUDIT.md`.

- [x] **ACL audit (SEC-D-01, SEC-D-06, ARCH-D-02):** Four depth/mask permission TOMLs (`allow-depth-inference`, `allow-depth-read`, `allow-depth-adjust`, `allow-mask`) replace coarse `allow-generate-depth-map`. `allow-settings.toml` and `allow-model.toml` grant settings/model commands. All six IDs in `src-tauri/capabilities/default.json`. 36/36 commands aligned (Rust ↔ `tauri.ts` ↔ permissions) — zero orphans.
- [x] **IPC caps (SEC-D-04):** `MAX_IPC_POINT_CLOUD_POINTS = 1_000_000` (preview subsample); export mode omits points from IPC; `MAX_POINT_CLOUD_GENERATION = 8_000_000` hard cap (`src-tauri/src/volumetric.rs`, `lib.rs`).
- [x] **Image pre-Python caps (SEC-D-03):** `MAX_IMAGE_FILE_BYTES = 128 MiB`, `MAX_DIMENSION = 8192` before subprocess (`src-tauri/src/image_loading.rs`); unit tests for oversize file and dimensions.
- [x] **Path policy (SEC-D-05):** `path_security::validate_import_file_path` on `load_preset` and `load_mask_from_path`; shared `is_blocklisted()` for load/export/import (`src-tauri/src/path_security.rs`).
- [x] **Export comment sanitization (SEC-D-12):** `sanitize_export_comment_field` in PLY/CSV export (`src-tauri/src/export.rs`); regression tests in `lib.rs`.
- [x] **Engraver volume warnings (UI-D-01):** Informational frontend-only; `f2VolumeWarnings.ts` + Vitest; not an IPC/security boundary.
- [ ] **Phase E deferred (SEC-D-07–SEC-D-11):** Subprocess env allowlist, `DepthAdjustmentParams` bounds, export TOCTOU/atomic write, blocklist parity sign-off, `withGlobalTauri: false` + release devtools — see threat model §2.8.
- [ ] **SEC-D-02 maintainer follow-up:** Replace stub SHA256 placeholders with real digests after trusted model download.

#### SEC-201 Review (Sprint 1.3) — Python subprocess

Completed **2026-02-03**. Implementation: `src-tauri/src/python_bridge.rs`.

| Criterion | Status | Evidence |
|-----------|--------|----------|
| (a) No user-controlled argv | ✅ | Command built only from `python_executable()`, fixed args `-m python.depth_estimator`, `--input`, and `input_path` (from `validate_input_path`). Image bytes written by Rust to temp file; path is app-controlled. |
| (b) Path canonicalized and under temp dir | ✅ | `validate_input_path()` canonicalizes path and enforces `path.starts_with(temp_dir)`. Input path comes from `file_io::write_temp_file` (system temp dir only). |
| (c) No shell | ✅ | `std::process::Command::new(&python)` — no shell; no `cmd /c` or `sh -c`. |
| (d) Findings documented | ✅ | No command injection; implementation aligns with threat model §2.5. |

### 2.3 Sign-off

- [ ] Security checklist sign-off per phase (todo.md Team Roles: Security Specialist ownership).
- [ ] Release checklist item “Security audit complete (no critical vulnerabilities)” satisfied.

---

## 3. Local audit commands (reference)

| Stack   | Command(s) | When to run        |
|---------|------------|--------------------|
| Rust    | `cargo audit --manifest-path src-tauri/Cargo.toml` | After adding a crate; before PR; before release |
| Frontend| `npm audit` or `npm audit --audit-level=high`     | After adding a package; before PR; before release |
| Python  | `pip install pip-audit && pip-audit -r python/requirements.txt` | After adding a package; before PR; before release |

CI runs `cargo audit`, `npm audit`, and `pip-audit` on every push/PR (see .github/workflows/ci.yml and docs/threat-model.md §4). Python: `pip-audit -r python/requirements.txt` (SECURITY_SIGNOFF §4, Sprint 1.11).

---

## 4. Where this checklist is referenced

- **SPRINTS:** Phase gates and sprint verification (e.g. VERIFICATION_CHECKLIST.md, SECURITY_SIGNOFF.md) can reference this doc.
- **todo.md:** Release Checklist, Security review ownership.
- **CONTRIBUTING.md:** Contributors can be pointed here for “before adding a dependency” and “before release” steps.

---

## 5. Phase 2 / Ongoing (SECURITY_SIGNOFF §4)

- **NPM:** Schedule `npm audit fix` or Vite/Svelte dependency upgrades in Phase 2 to reduce moderate advisories (esbuild, Svelte SSR). Do not use `npm audit fix --force` without assessing breaking changes.
- **Rust:** When upgrading Tauri/wry, re-run `cargo audit`; consider stricter policy (e.g. deny unmaintained) if advisories persist. Monitor [RustSec Advisory DB](https://github.com/RustSec/advisory-db) and Tauri release notes.
- **Model download:** When implementing (e.g. model wizard), follow **SEC-202** above: HTTPS only, SHA256 checksum verification, expected hash from trusted source (RESEARCH / repo), not from download response. See prd.md §8.2 (Model Security).

---

**Document Version:** 1.4  
**Status:** Sprint D (DOC-D-02, 2026-05-20): §2.2 Sprint D verification bullets (ACL audit, IPC caps, path policy, SEC-D-03/04/05/12, UI-D-01). SEC-D-07–SEC-D-11 tracked as Phase E in threat model §2.8. Prior: SEC-003 (1.1), SEC-101/102 (1.2), SEC-201/202 (1.3), pip-audit CI (1.11).
