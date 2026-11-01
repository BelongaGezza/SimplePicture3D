# Sprint Test Plan — Sprint D (Phase D: Validate & Harden)

**Source:** `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Sprint:** D — Validate and Harden (Phase D)  
**Task:** QA-D-07  
**Last Updated:** 2026-05-24

---

## 1. Scope

| Item | Description |
|------|-------------|
| Sprint goal | Validate engraver-compatible exports; close P0/P1 security gaps; align Tauri ACL with minimum privilege before v1.0 beta |
| Features in scope | ADR-012 surface-map point cloud export (PLY/XYZ/CSV); engraver import procedure (QA-D-01); export command integration tests (QA-B-01); security hardening (SEC-D-*); Windows CI backend job (CI-D-01) |
| Out of scope | Sprint 2.6 enhanced 3D preview (superseded — see `SPRINTS/Sprint_2_6/SPRINT_2_6_CANCELLED.md`); automated Playwright E2E (Phase 2); macOS/Linux installer and platform matrix (Phase 3 / Phase F); full accessibility pass (Phase G) |

**Phase D gate:** Do not mark Phase D complete until QA-D-01 engraver execution (when target software is named) and security sign-off criteria in `SPRINT_D_Task_Assignment.md` are met. See `ROADMAP.md` and `SPRINTS/Sprint_D/ORCHESTRATION.md`.

---

## 2. Automated Tests

### 2.1 What runs in CI

| Suite | Command | When | Baseline (2026-05-24) |
|-------|---------|------|------------------------|
| Rust unit/integration | `cargo test --manifest-path src-tauri/Cargo.toml` | Every push/PR (`backend`, `backend-windows`) | **172 passed**, 5 ignored (~177 total) |
| Frontend (Vitest) | `npm test` | Every push/PR (`frontend`) | **78 passed** |
| Python (stub, no model) | `SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v` | Every push/PR (`python`) | **24 passed** (stub mode) |
| Rust lint | `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` | `backend`, `backend-windows` | Zero warnings |
| Security audits | `cargo audit`, `npm audit --audit-level=high`, `pip-audit` | `security` job | Per `docs/threat-model.md` |

**Reference:** `.github/workflows/ci.yml`, `todo.md` § CI/CD Pipeline.

#### CI-D-01 — Windows backend job

Primary shipping platform validation without Linux GTK/WebKit on the runner:

| Step | Command | Notes |
|------|---------|-------|
| Build | `cargo build --manifest-path src-tauri/Cargo.toml` | Placeholder `dist/` for Tauri frontend path |
| Test | `cargo test --manifest-path src-tauri/Cargo.toml` | Same suite as Linux `backend` job |
| Clippy | `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` | Zero warnings policy |

Job name: `Backend (Windows)` · `runs-on: windows-latest` · See `.github/workflows/ci.yml` `backend-windows`.

### 2.2 Sprint D automated additions

| Test / area | Location | Task | Purpose |
|-------------|----------|------|---------|
| Export PLY/XYZ/CSV command paths | `src-tauri/src/lib.rs` (`export_*_from_state`, `#[test]` module) | QA-B-01 | Five integration tests: per-format export, generate→export, empty-cache error |
| Export IPC (frontend) | `src/lib/__tests__/tauri.test.ts` | QA-D-05 | `exportXyz`, `exportCsv`, `setPointCloudFormat` invoke wiring |
| Model hash manifest (stub) | `python/tests/test_model_downloader.py` | SEC-D-02 | Loader behaviour; real SHA256 pending trusted download |
| Path security | `src-tauri/src/path_security.rs` | SEC-D-05 | Preset/mask import path validation |

**QA-B-01 pattern:** Tauri command handlers stay thin; tests call `export_ply_from_state`, `export_xyz_from_state`, `export_csv_from_state` with a constructed `AppState` (see `RESEARCH/GOTCHAS.md` 2026-05-24 entry).

### 2.3 Coverage (QA-D-04)

| Tool | Status | Notes |
|------|--------|-------|
| `cargo tarpaulin` | **Not run** — not installed on 2026-05-24 dev session | Install: `cargo install cargo-tarpaulin`. Re-measure before changing any CI coverage threshold. |
| `pytest --cov` | Optional | See `CONTRIBUTING.md` / README when enabled |

*Do not lower or add CI coverage gates until tarpaulin has been run locally and the percentage is recorded here.*

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| **Primary OS** | Windows 10/11 (Phase 1+2 shipping target) |
| **Secondary OS** | macOS / Linux — Phase 3; document in `MANUAL_TEST_REPORT.md` when tested |
| Node version | 20 (CI) |
| Rust version | stable (CI) |
| Python | 3.10–3.12; venv per `SETUP_NOTES.md`; use `SP3D_USE_STUB=1` for repeatable depth without model download |
| App launch | `npm run tauri dev` or release build |

**Linux/macOS local Rust tests:** GTK3 + WebKit2GTK (+ `libsoup-3.0-dev` on Linux) required for `cargo test` / `cargo build` — see `SETUP_NOTES.md` and `RESEARCH/GOTCHAS.md` (2026-05-24).

**Windows CI note:** `backend-windows` does not need GTK/WebKit; Linux `backend` job installs apt packages in CI.

### 3.2 Manual test cases

Canonical step tables live in **`docs/testing.md`**. Sprint D execution is recorded in **`SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md`**.

| Area | Source | Sprint artefact | Status |
|------|--------|-----------------|--------|
| Core workflow (load → depth → blank → preview → export → persistence) | `docs/testing.md` §1 | `MANUAL_TEST_REPORT.md` § QA-C-01 | Ready to fill — not executed in QA-D-07 |
| Export verification (PLY/XYZ/CSV, blank fit) | `docs/testing.md` §2 | `MANUAL_TEST_REPORT.md` § QA-C-01 | Ready to fill |
| Engraver import (Phase D gate) | `QA-D-01_TEST_PROCEDURE.md` § D | `MANUAL_TEST_REPORT.md` § QA-D-01 | **Blocked** until target engraver software is named (ARCH-D-01 / product) |
| 4K performance | PRD timing target | `MANUAL_TEST_REPORT.md` § QA-D-02 | Available (QA-D-02) |
| Security pentest | `SECURITY_PENTEST_CHECKLIST.md` | `MANUAL_TEST_REPORT.md` § QA-D-03 | Available (QA-D-03) |

#### Case D-01: Engraver import (QA-D-01)

| Field | Content |
|-------|---------|
| **Objective** | PLY/XYZ import into target engraver; axes mm, orientation per `docs/engraver-charter.md` |
| **Preconditions** | App export path complete (procedure § C); engraver software installed |
| **Steps** | Full procedure: `SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md` |
| **Expected result** | Charter checks V1–V7 pass; import without scale/axis errors |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail [ ] Blocked (Xtool Studio / F2 unavailable) |

#### Case D-02: Core + export workflow (QA-C-01)

| Field | Content |
|-------|---------|
| **Objective** | End-to-end ADR-012 workflow without terminal; regression on Phase C UI |
| **Preconditions** | Toolchain per `SETUP_NOTES.md`; optional stub depth |
| **Steps** | `docs/testing.md` §1 (steps 1–9) and §2 (steps 1–5) |
| **Expected result** | All checklist rows pass; no crash; exports open in viewer |
| **Actual result** | *(fill in MANUAL_TEST_REPORT § QA-C-01)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

*Additional sections in `docs/testing.md` (§3 settings, §4 presets, §5 first-run) — execute as time permits; record in manual report.*

### 3.3 Regression / smoke

- [ ] `npm run tauri dev` — app starts, main screen loads
- [ ] Load image (fixture or small PNG) — 2D view updates
- [ ] Generate depth (stub or model) — depth map visible
- [ ] Generate point cloud — 3D preview shows points inside blank wireframe
- [ ] Export PLY — file on disk, non-zero size
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` — 172+ passed (local/CI)
- [ ] `npm test` — 78 passed
- [ ] `SP3D_USE_STUB=1` pytest — 24 passed

---

## 4. Artefacts and sign-off

| Artefact | Path | Owner | QA task |
|----------|------|-------|---------|
| This test plan | `SPRINTS/Sprint_D/TEST_PLAN_D.md` | Quality Engineer | QA-D-07 |
| Engraver procedure | `SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md` | Quality Engineer | QA-D-01 |
| Manual test results | `SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md` | Quality Engineer | QA-D-01, QA-C-01, QA-D-02, QA-D-03 |
| Engraver charter | `docs/engraver-charter.md` | System Architect | ARCH-D-01 |
| Security pentest | `SPRINTS/Sprint_D/SECURITY_PENTEST_CHECKLIST.md` | Quality Engineer + Security | QA-D-03 |
| Sprint 2.6 cancellation | `SPRINTS/Sprint_2_6/SPRINT_2_6_CANCELLED.md` | Quality Engineer | QA-D-08 |
| Gotchas (repo-wide) | `RESEARCH/GOTCHAS.md` | Any agent | QA-D-09 |
| Orchestration board | `SPRINTS/Sprint_D/ORCHESTRATION.md` | Senior Engineer | — |

**Process:** Run automated suites in CI or locally → execute manual cases → fill `MANUAL_TEST_REPORT.md` → security checklist → orchestrator sign-off in `SPRINT_D_Task_Assignment.md` / `ORCHESTRATION.md`.

**Sprint D QA sign-off criteria (summary):**

- [ ] QA-C-01: `docs/testing.md` §1–2 recorded
- [ ] QA-D-01: Engraver import Pass or documented Blocked/Partial with defect log
- [ ] QA-D-02: 4K timing within PRD target (or documented deferral)
- [ ] QA-D-03: Pentest checklist complete
- [ ] Automated baselines green in CI (including `backend-windows`)

---

## 5. References

| Document | Purpose |
|----------|---------|
| `SPRINTS/Sprint_D/SPRINT_D_Task_Assignment.md` | Task IDs, owners, acceptance criteria |
| `SPRINTS/Sprint_D/ORCHESTRATION.md` | Delegation status, blockers |
| `docs/testing.md` | Manual checklist §1–7 |
| `docs/engraver-charter.md` | Axis, units, format contract (ARCH-D-01) |
| `prd.md` | Acceptance criteria, performance targets |
| `todo.md` | Phase D tasks, CI/CD pipeline |
| `ROADMAP.md` | Phase D validation milestones |
| `CLAUDE.md` | Canonical test commands |
| `.github/workflows/ci.yml` | CI jobs including CI-D-01 |

---

**Document Version:** 1.0  
**Template:** `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Task:** QA-D-07
