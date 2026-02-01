# Sprint Test Plan — [Sprint X.Y]

**Use this template when planning QA for a sprint.**  
Copy to `SPRINTS/Sprint_X_Y/TEST_PLAN_Sprint_X_Y.md` (or keep as reference) and populate.

**Source:** `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Sprint:** [Sprint X.Y — name from Task Assignment]  
**Last Updated:** [Date]

---

## 1. Scope

| Item | Description |
|------|-------------|
| Sprint goal | [One sentence from Task Assignment] |
| Features in scope | [List features/tasks that need testing] |
| Out of scope | [Features or platforms explicitly excluded this sprint] |

---

## 2. Automated Tests

### 2.1 What runs in CI

| Suite | Command | When |
|-------|---------|------|
| Rust unit/integration | `cargo test --manifest-path src-tauri/Cargo.toml` | Every push/PR |
| Frontend (if any) | `npm test` | Every push/PR |
| Python (if applicable) | `pytest` (e.g. `python/`) | Every push/PR |

**Reference:** `.github/workflows/ci.yml`, `todo.md` § CI/CD Pipeline.

### 2.2 New or updated automated tests this sprint

| Test | Location | Purpose |
|------|----------|---------|
| [e.g. file_io path sanitization] | [e.g. src-tauri/src/file_io.rs] | [e.g. unit test for safe temp paths] |

*Add rows for each new test or test file added in this sprint.*

### 2.3 Coverage (if configured)

- **Rust:** `cargo tarpaulin` (see README / CONTRIBUTING)
- **Python:** `pytest --cov` (see README / CONTRIBUTING)

*Note: QA-003 configures coverage; once done, record commands and targets here.*

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | [e.g. Windows 11, Ubuntu 22.04] |
| Node version | [e.g. 20] |
| Rust version | [e.g. stable] |
| Python (if used) | [e.g. 3.11, venv] |

### 3.2 Manual test cases

For each feature or user flow that requires manual verification, add a case:

#### Case [N]: [Short name]

| Field | Content |
|-------|---------|
| **Objective** | [What we are verifying] |
| **Preconditions** | [App state, data, or setup required] |
| **Steps** | 1. [Step] 2. [Step] … |
| **Expected result** | [What should happen] |
| **Actual result** | *(fill during test)* |
| **Pass / Fail** | [ ] Pass [ ] Fail |

*Duplicate this block for each manual case. Link to PRD or Task Assignment acceptance criteria where relevant.*

### 3.3 Regression / smoke (optional)

- [ ] App starts (`npm run tauri dev`)
- [ ] Load image (stub) and Export (stub) buttons respond
- [ ] No console errors on main screen

*Adjust per sprint; minimal smoke to avoid regressions.*

---

## 4. Artefacts and sign-off

| Artefact | Path | Owner |
|----------|------|-------|
| Manual test results | `SPRINTS/Sprint_X_Y/MANUAL_TEST_REPORT.md` | Quality Engineer |
| Verification checklist | `SPRINTS/Sprint_X_Y/VERIFICATION_CHECKLIST.md` | Sprint lead / Architect |
| Gotchas | `SPRINTS/Sprint_X_Y/GOTCHAS.md` | Any agent; merge to `RESEARCH/GOTCHAS.md` |

**Process:** Complete manual tests → fill `MANUAL_TEST_REPORT.md` → run through `VERIFICATION_CHECKLIST.md` before marking sprint complete.

---

## 5. References

- **Task list:** `SPRINTS/Sprint_X_Y/SPRINT_X_Y_Task_Assignment.md`
- **PRD:** `prd.md` (acceptance criteria)
- **Testing strategy:** `todo.md` § Testing Strategy, § CI/CD Pipeline
- **CLAUDE.md:** Testing commands (cargo test, npm test, pytest)

---

**Document Version:** 1.0  
**Template:** `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**QA-002:** Reusable test plan template for sprint QA (manual + automated).
