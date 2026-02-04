# Security Checklist — Dependency & Code Review

**Purpose:** Reusable checklist for adding dependencies and for release; covers dependency audit and review steps.  
**Reference:** prd.md §8 (Security & Privacy), docs/threat-model.md  
**Use:** Before adding a crate/npm/pip package; before release; per sprint if security review is in scope.

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
- [ ] **Model download (SEC-202):** HTTPS only (no redirect to HTTP); SHA256 checksum verified after download; expected hash from trusted source (e.g. repo/RESEARCH), not from download response (threat model §2.2).

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
| Python  | `pip-audit` (in project venv)                     | After adding a package; before PR; before release |

CI runs `cargo audit` and `npm audit` on every push/PR (see .github/workflows/ci.yml and docs/threat-model.md §4).

---

## 4. Where this checklist is referenced

- **SPRINTS:** Phase gates and sprint verification (e.g. VERIFICATION_CHECKLIST.md, SECURITY_SIGNOFF.md) can reference this doc.
- **todo.md:** Release Checklist, Security review ownership.
- **CONTRIBUTING.md:** Contributors can be pointed here for “before adding a dependency” and “before release” steps.

---

**Document Version:** 1.2  
**Status:** Sprint 1.1, SEC-003; image loading (path + magic bytes) items added (Sprint 1.2, SEC-101, SEC-102). Subprocess review criteria (SEC-201) and model download requirements (SEC-202) added (Sprint 1.3).
