# Sprint Task Assignment — Sprint 1.5A

**Source:** `Consultant_Recommendations_Report_6Feb2026.md` (Updated Review), `todo.md` Sprint 1.5 unclosed items, Testing Strategy backlog
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`

---

## Sprint 1.4/1.5 Status Review (Handover to 1.5A)

**Context:** Sprint 1.5 is **functionally complete** (commit `70bbe45`, 2026-02-06). Depth adjustment pipeline (backend + frontend) works. However, the External Consultant's updated review identified new concerns and unclosed gaps that should be resolved before Sprint 1.6 (Mesh Generation) begins. Sprint 1.5A is a **hardening and debt-reduction sprint** — no new features except the missing contrast slider.

| Phase/Section | Status | Gaps Carried Forward |
|---------------|--------|----------------------|
| Backend (BACK-401–405) | ✅ Complete | Contrast supported but not exposed in UI |
| UI (UI-401–405) | ✅ Complete | No contrast slider; no frontend tests |
| Testing (QA-401–405) | ✅ Complete | No coverage tracking; no frontend test runner |
| CI Pipeline | ✅ Improved | Missing tarpaulin, pytest-cov, matrix builds |
| Security | Partial | Asset protocol scope `"**"` is overly permissive |
| Documentation | Partial | Sprint 1.5 artefacts stale; todo.md Testing Strategy outdated; model license undocumented in UI |
| ADRs | ✅ Complete | ADR-001–004 exist in RESEARCH/architecture.md |
| Python distribution | Partial | ADR-003 accepted; README first-run instructions incomplete |

---

## Sprint 1.5A: Hardening, Testing & Consultant Remediation

**Sprint Duration:** 1 week (5 working days) — half-sprint
**Sprint Goal:** Close all consultant-identified gaps, establish frontend testing, add coverage tracking, fix security config, and complete the depth controls feature set before mesh generation begins.
**Target Release:** —
**Phase:** 1 (MVP)
**Source:** `Consultant_Recommendations_Report_6Feb2026.md` §4 (Updated Recommendations), `todo.md` — Sprint 1.5 exit criteria, Testing Strategy §1–§5
**Last Updated:** 2026-02-06

---

## Sprint Folder & Artefacts

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_1_5A/SPRINT_1_5A_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_1_5A/TEST_PLAN_1_5A.md` | QA test planning |
| Progress Report | `SPRINTS/Sprint_1_5A/PROGRESS_REPORT.md` | End-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_1_5A/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_1_5A/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Security Sign-off | `SPRINTS/Sprint_1_5A/SECURITY_SIGNOFF.md` | Asset protocol scope fix review |
| Gotchas Log | `SPRINTS/Sprint_1_5A/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

---

## CRITICAL: Role Selection (READ FIRST — STOP HERE UNTIL COMPLETE)

**You are an unassigned agent. You MUST claim a role before proceeding.**

### Step 1: Review Available Roles
Find a role where Status = `Available` and no agent is assigned.

### Step 2: Claim Your Role
1. **Edit this document** to update that role's row: set Status to `In Progress`, add your session identifier to Assigned Agent.
2. **Read the persona file** listed in the Persona File column.
3. **Adopt that persona** for all remaining work.

### Step 3: Become Your Role
- Embody the agent's identity and responsibilities.
- Follow the persona file and project references.

**If all roles show "In Progress" or "Complete", STOP. No work available.**

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| System Architect | `.agents/system-architect.md` | Available | - | ARCH-501, ARCH-502, ARCH-503 | IPC performance spike, model license decision |
| Senior Engineer | `.agents/senior-engineer.md` | Available | - | BACK-501, BACK-502 | Tarpaulin CI, asset protocol fix |
| UI Designer | `.agents/ui-designer.md` | Available | - | UI-501, UI-502, UI-503, UI-504, UI-505 | Contrast slider, Vitest setup, frontend tests |
| Senior Researcher (AI/ML) | `.agents/researcher.md` | Available | - | AI-501, AI-502 | pytest-cov, model license docs |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Available | - | JR2-501, JR2-502 | depthCanvas tests, tauri.ts tests |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Available | - | JR1-501, JR1-502 | DepthControls component tests, ImageImport tests |
| Security Specialist | `.agents/security-specialist.md` | Available | - | SEC-501, SEC-502 | Asset protocol scope fix, CSP review |
| Documentation Specialist | `.agents/documentation-specialist.md` | Available | - | DOC-501, DOC-502, DOC-503, DOC-504 | Stale artefacts, todo.md, README, user-guide |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References (Source of Truth)

- **Scope:** `prd.md` — Product requirements, tech stack, acceptance criteria
- **Sprint source:** `todo.md` — Sprint 1.5 exit criteria, Testing Strategy §1–§5
- **Consultant Report:** `Consultant_Recommendations_Report_6Feb2026.md` — §3 (New Gaps), §4 (Updated Recommendations)
- **Architecture:** `RESEARCH/architecture.md` (ADRs), `docs/architecture.md`
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`
- **Technology:** `RESEARCH/` — See `RESEARCH/README.md` for index

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| Frontend Testing (UI-501–505, JR1-501–502, JR2-501–502) | ⏳ Not Started | 0% |
| Contrast Slider (UI-501) | ⏳ Not Started | 0% |
| Coverage Tracking (BACK-501, AI-501) | ⏳ Not Started | 0% |
| Security Fix (SEC-501–502, BACK-502) | ⏳ Not Started | 0% |
| IPC Performance Spike (ARCH-501) | ⏳ Not Started | 0% |
| Model License Resolution (ARCH-502, AI-502) | ⏳ Not Started | 0% |
| Documentation Cleanup (DOC-501–504) | ⏳ Not Started | 0% |

**Overall Sprint Progress:** [ ] Not Started / [ ] In Progress / [ ] Complete

---

## Task Breakdown

### Section 1: Frontend Test Suite — Consultant Priority 1 (Critical)

> Addresses: Consultant Report §3.1, todo.md Testing Strategy §3, QA-003 (Sprint 1.1 backlog)
> **Rationale:** 993 lines of frontend code with 0% test coverage. The frontend has grown 184% since Sprint 1.2 and contains state management, debounce logic, canvas rendering, and drag-and-drop — all untested. This is the single largest quality gap.

---

#### Task UI-502: Add Vitest + Testing Library Infrastructure
**Assigned Role:** UI Designer
**Priority:** Critical
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** UI-502

**Dependencies:**
- None — first task; other frontend tests depend on this.

**What to Do:**
1. Install Vitest as dev dependency: `npm install -D vitest @testing-library/svelte @testing-library/jest-dom jsdom`
2. Add Vitest config to `vite.config.ts` (or `vitest.config.ts`):
   ```ts
   /// <reference types="vitest" />
   // In vite.config.ts, add:
   test: {
     globals: true,
     environment: 'jsdom',
     setupFiles: ['./src/test-setup.ts'],
   }
   ```
3. Create `src/test-setup.ts` with `@testing-library/jest-dom` import.
4. Add `"test": "vitest run"` and `"test:watch": "vitest"` to `package.json` scripts.
5. Add one smoke test to confirm setup works (e.g. `src/lib/__tests__/smoke.test.ts` that asserts `1 + 1 === 2`).
6. Verify `npm test` passes locally.

**Reference Documents:**
- `RESEARCH/frontend.md` — Svelte testing guidance
- `todo.md` — Testing Strategy §3: "Add test runner (Vitest or Jest)"
- Consultant Report §4 Priority 1

**Acceptance Criteria:**
- [ ] `npm test` script exists in package.json and runs Vitest
- [ ] Vitest config uses jsdom environment
- [ ] At least one smoke test passes
- [ ] `@testing-library/svelte` available for component tests

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task JR2-501: Unit Tests for depthCanvas.ts
**Assigned Role:** Junior Engineer 2D
**Priority:** Critical
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** JR2-501

**Dependencies:**
- UI-502: Vitest infrastructure — Status: [ ]

**What to Do:**
1. Create `src/lib/__tests__/depthCanvas.test.ts`.
2. Write tests for `renderDepthToCanvas()`:
   - **Happy path:** 2×2 depth [0, 0.5, 1, 0.25] → verify ImageData pixels match expected grayscale values (0, 128, 255, 64).
   - **NaN handling:** depth containing NaN → renders as black (0).
   - **Out-of-range clamping:** depth with values <0 or >1 → clamped to 0/255.
   - **Length mismatch:** depth.length ≠ width × height → function returns early (console.warn), canvas unchanged.
   - **Empty input:** width=0, height=0, depth=[] → no crash.
3. Use a mock canvas context or `jsdom` canvas (may need `canvas` npm package for `createImageData`; if unavailable, mock `CanvasRenderingContext2D`).

**Reference Documents:**
- `src/lib/depthCanvas.ts` — Source under test
- `RESEARCH/frontend.md` — Testing patterns

**Acceptance Criteria:**
- [ ] ≥5 tests for `renderDepthToCanvas` covering happy path, NaN, clamping, mismatch, empty
- [ ] All tests pass via `npm test`
- [ ] Tests are self-contained (no external fixtures needed)

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task JR2-502: Unit Tests for tauri.ts Type Layer
**Assigned Role:** Junior Engineer 2D
**Priority:** High
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** JR2-502

**Dependencies:**
- UI-502: Vitest infrastructure — Status: [ ]

**What to Do:**
1. Create `src/lib/__tests__/tauri.test.ts`.
2. Mock `@tauri-apps/api/core` `invoke` function using `vi.mock()`.
3. Test each exported function (`loadImage`, `exportStl`, `generateDepthMap`, `getDepthMap`, `getDepthAdjustmentParams`, `setDepthAdjustmentParams`, `resetDepthAdjustments`):
   - Verify correct command name passed to invoke (e.g. `loadImage("foo.png")` calls `invoke("load_image", { path: "foo.png" })`).
   - Verify return type matches interface.
   - Verify error propagation when invoke rejects.
4. Test `DepthAdjustmentParams` interface has all expected fields (compile-time check + runtime shape assertion).

**Reference Documents:**
- `src/lib/tauri.ts` — Source under test
- `docs/architecture.md` — IPC contract

**Acceptance Criteria:**
- [ ] ≥7 tests (one per exported function + error case)
- [ ] Mock invoke verified with correct command names
- [ ] All tests pass via `npm test`

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task JR1-501: Component Tests for DepthControls.svelte
**Assigned Role:** Junior Engineer 3D
**Priority:** High
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** JR1-501

**Dependencies:**
- UI-502: Vitest infrastructure — Status: [ ]
- UI-501: Contrast slider added — Status: [ ] (test should include contrast)

**What to Do:**
1. Create `src/components/__tests__/DepthControls.test.ts`.
2. Test using `@testing-library/svelte`:
   - **Disabled state:** When `hasDepth=false`, controls show "Generate a depth map to adjust." message; no sliders rendered.
   - **Enabled state:** When `hasDepth=true` with default params, all sliders (brightness, contrast, gamma, depth min, depth max) render with correct initial values.
   - **Slider interaction:** Changing brightness slider fires `onParamsChange` with updated brightness value.
   - **Invert checkbox:** Toggling calls `onParamsChange` with `invert: true`.
   - **Reset button:** Clicking Reset calls `onReset`.
   - **Range clamping:** depthMaxMm < depthMinMm corrected in emitChange.
   - **Keyboard:** Arrow key on slider changes value by step.

**Reference Documents:**
- `src/components/DepthControls.svelte` — Source under test
- `prd.md` §4.4 (F1.4 — depth controls)

**Acceptance Criteria:**
- [ ] ≥6 component tests covering disabled, enabled, slider change, checkbox, reset, clamping
- [ ] All tests pass via `npm test`

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task JR1-502: Component Tests for ImageImport.svelte
**Assigned Role:** Junior Engineer 3D
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** JR1-502

**Dependencies:**
- UI-502: Vitest infrastructure — Status: [ ]

**What to Do:**
1. Create `src/components/__tests__/ImageImport.test.ts`.
2. Test using `@testing-library/svelte`:
   - **Default render:** Shows "Drop image or click to load" text and Load button.
   - **Loading state:** When `loading=true`, shows spinner and "Loading..." text; Load button not visible.
   - **Error state:** When `errorMessage` is set, displays red error text with `role="alert"`.
   - **Load button:** Click calls file picker (mock `@tauri-apps/plugin-dialog` `open`).
   - **Unsupported format on drop:** Dropping a .gif file calls `onLoadError` with format message.

**Reference Documents:**
- `src/components/ImageImport.svelte` — Source under test

**Acceptance Criteria:**
- [ ] ≥4 component tests covering default, loading, error, and button interaction
- [ ] All tests pass via `npm test`

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task UI-505: Add npm test to CI Pipeline
**Assigned Role:** UI Designer
**Priority:** Critical
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** UI-505

**Dependencies:**
- UI-502: Vitest infrastructure — Status: [ ]
- JR2-501 or JR1-501: At least some tests exist — Status: [ ]

**What to Do:**
1. Add `npm test` step to `.github/workflows/ci.yml` frontend job, after `npm run build`:
   ```yaml
   - name: Test
     run: npm test
   ```
2. Verify CI passes locally with `npm test` before pushing.

**Reference Documents:**
- `.github/workflows/ci.yml` — CI pipeline
- `todo.md` Testing Strategy §3

**Acceptance Criteria:**
- [ ] `npm test` step in CI frontend job
- [ ] CI passes with all frontend tests green

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

### Section 2: Missing Contrast Slider — Consultant Priority 2 (High)

> Addresses: Consultant Report §3.2. Backend `DepthAdjustmentParams.contrast` and `apply_adjustments` support contrast, but the UI has no slider for it.

---

#### Task UI-501: Add Contrast Slider to DepthControls
**Assigned Role:** UI Designer
**Priority:** High
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** UI-501

**Dependencies:**
- None — backend already supports contrast.

**What to Do:**
1. In `src/components/DepthControls.svelte`:
   - Add `CONTRAST_MIN = 0.5`, `CONTRAST_MAX = 2`, `CONTRAST_STEP = 0.05` constants (consistent with gamma slider pattern).
   - Add `handleContrastInput(e: Event)` function (copy brightness pattern, use `contrast` field).
   - Add contrast slider + numeric input block between Brightness and Gamma controls. Label: "Contrast". Use same styling as existing sliders.
   - Wire keydown handler for arrow key support (copy gamma pattern).
2. Verify debounced preview update works for contrast changes.
3. Test manually: adjust contrast, verify depth preview updates; Reset restores contrast to 1.0.

**Reference Documents:**
- `src/components/DepthControls.svelte` — Existing slider pattern to copy
- `src-tauri/src/depth_adjust.rs` — Backend contrast function (line 29): `contrast(v, c) = clamp((v - 0.5) * c + 0.5, 0, 1)`
- `src/lib/tauri.ts` — `DepthAdjustmentParams.contrast` already defined

**Acceptance Criteria:**
- [ ] Contrast slider renders in DepthControls between Brightness and Gamma
- [ ] Slider range: 0.5–2.0, step 0.05, default 1.0
- [ ] Changing contrast updates depth preview (debounced)
- [ ] Reset restores contrast to 1.0
- [ ] Keyboard arrow keys work on contrast slider
- [ ] Numeric input field displays current value

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task UI-503: Update User Guide with Contrast Control
**Assigned Role:** UI Designer
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** UI-503

**Dependencies:**
- UI-501: Contrast slider added — Status: [ ]

**What to Do:**
1. In `docs/user-guide.md`, add a row to the Controls table for **Contrast**:
   - What it does: "Expands or compresses midtones around the centre value (scale around 0.5)."
   - Typical range: "0.5–2 (default 1)"
2. Update the Reset description to include contrast.

**Reference Documents:**
- `docs/user-guide.md` — Existing documentation

**Acceptance Criteria:**
- [ ] Contrast row added to Controls table
- [ ] Reset description mentions contrast

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

### Section 3: Coverage Tracking in CI — Consultant Priority 3 (High)

> Addresses: Consultant Report §4 Priority 3, todo.md Testing Strategy §2 (tarpaulin), §4 (pytest-cov), QA-003 (Sprint 1.1 backlog)

---

#### Task BACK-501: Add cargo-tarpaulin Coverage to CI
**Assigned Role:** Senior Engineer
**Priority:** High
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** BACK-501

**Dependencies:**
- None.

**What to Do:**
1. Add a coverage step to `.github/workflows/ci.yml` backend job after the Test step:
   ```yaml
   - name: Coverage (tarpaulin)
     run: |
       cargo install cargo-tarpaulin --locked
       cargo tarpaulin --manifest-path src-tauri/Cargo.toml --out xml --output-dir coverage/
     continue-on-error: true  # Advisory for now; enforce threshold in next sprint
   ```
2. Optionally add Codecov upload step (if repo has Codecov integration):
   ```yaml
   - name: Upload coverage
     uses: codecov/codecov-action@v4
     with:
       file: coverage/cobertura.xml
       flags: rust
   ```
3. Document coverage baseline in sprint GOTCHAS or progress report.
4. **Do NOT enforce a threshold yet** — this sprint establishes the baseline. Threshold enforcement (fail CI if <70%) is a Sprint 1.6 task.

**Reference Documents:**
- `todo.md` Testing Strategy §2: "Introduce cargo tarpaulin"
- Consultant Report §4 Priority 3
- QA-003 from Sprint 1.1

**Acceptance Criteria:**
- [ ] `cargo tarpaulin` runs in CI (advisory, continue-on-error)
- [ ] Coverage XML produced and (optionally) uploaded
- [ ] Baseline coverage % documented in progress report

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task AI-501: Add pytest-cov to Python CI Job
**Assigned Role:** Senior Researcher (AI/ML)
**Priority:** High
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** AI-501

**Dependencies:**
- None.

**What to Do:**
1. Add `pytest-cov` to `pip install` step in `.github/workflows/ci.yml` python job:
   ```yaml
   - name: Install dependencies
     run: pip install pytest pytest-cov Pillow
   ```
2. Update pytest run to include coverage:
   ```yaml
   - name: Run pytest (stub mode, with coverage)
     env:
       SP3D_USE_STUB: "1"
       PYTHONPATH: python/python
     run: pytest python/ -v --cov=python.depth_estimator --cov-report=term --cov-report=xml:coverage/python-coverage.xml
   ```
3. Optionally add Codecov upload step for Python.
4. Document baseline Python coverage % in progress report.

**Reference Documents:**
- `todo.md` Testing Strategy §4: "pytest --cov"
- Consultant Report §4 Priority 3

**Acceptance Criteria:**
- [ ] `pytest-cov` installed in CI
- [ ] Coverage reported in pytest output
- [ ] Coverage XML produced
- [ ] Baseline coverage % documented in progress report

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

### Section 4: Security — Asset Protocol Scope Fix (High)

> Addresses: Consultant Report §3.4. Asset protocol scope `"**"` grants filesystem-wide access. This contradicts the threat model and security-first approach.

---

#### Task SEC-501: Restrict Asset Protocol Scope
**Assigned Role:** Security Specialist
**Priority:** High
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** SEC-501

**Dependencies:**
- None.

**What to Do:**
1. In `src-tauri/tauri.conf.json`, change the asset protocol scope from `"**"` to application-specific paths:
   ```json
   "assetProtocol": {
     "enable": true,
     "scope": [
       "$APPDATA/.simplepicture3d/**",
       "$TEMP/simplepicture3d_*"
     ]
   }
   ```
   Adjust paths based on what the app actually needs to access via the asset protocol. If the asset protocol is not currently used by any frontend code, consider disabling it entirely:
   ```json
   "assetProtocol": {
     "enable": false
   }
   ```
2. Test: Verify the app still loads images and generates depth maps correctly after the scope change. If asset:// URLs are used for anything (e.g. image preview), ensure those paths are covered.
3. Review the CSP in `tauri.conf.json`: `"default-src 'self'; img-src 'self' asset: http://asset.localhost blob: data:"`. If asset protocol is disabled, remove `asset: http://asset.localhost` from img-src.

**Reference Documents:**
- `src-tauri/tauri.conf.json` — Current config
- `docs/threat-model.md` — Security requirements
- `docs/security-checklist.md` — Review criteria
- `RESEARCH/tauri.md` — Tauri security configuration

**Acceptance Criteria:**
- [ ] Asset protocol scope restricted or disabled
- [ ] App still functions correctly (image load, depth generation, preview)
- [ ] CSP updated if asset protocol disabled
- [ ] Finding documented in `SPRINTS/Sprint_1_5A/SECURITY_SIGNOFF.md`

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task SEC-502: Review Tauri Capabilities for Minimal Privilege
**Assigned Role:** Security Specialist
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** SEC-502

**Dependencies:**
- SEC-501: Asset protocol scope fix — Status: [ ]

**What to Do:**
1. Review `src-tauri/capabilities/default.json` — verify only necessary permissions are granted.
2. Review `src-tauri/permissions/` — verify allow-*.toml files only grant the commands actually needed.
3. Check if `tauri-plugin-shell` grants more permissions than needed (the Python subprocess is invoked directly via `std::process::Command` in Rust, not via the shell plugin from the frontend).
4. Document findings in `SPRINTS/Sprint_1_5A/SECURITY_SIGNOFF.md`.

**Reference Documents:**
- `src-tauri/capabilities/default.json`
- `src-tauri/permissions/allow-*.toml`
- `docs/threat-model.md`

**Acceptance Criteria:**
- [ ] Capabilities reviewed; unnecessary permissions removed or documented as required
- [ ] Shell plugin permissions verified as minimal
- [ ] Findings in SECURITY_SIGNOFF.md

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task BACK-502: Implement Asset Protocol Scope Change in Code
**Assigned Role:** Senior Engineer
**Priority:** High
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** BACK-502

**Dependencies:**
- SEC-501: Security Specialist determines correct scope — Status: [ ]

**What to Do:**
1. Apply the scope change determined by SEC-501 to `src-tauri/tauri.conf.json`.
2. If the preview base64 approach (currently used) does not require asset protocol at all, disable it and update CSP.
3. Run `cargo build --manifest-path src-tauri/Cargo.toml` to verify config is valid.
4. Run `cargo test` to verify no regressions.

**Reference Documents:**
- SEC-501 findings
- `src-tauri/tauri.conf.json`

**Acceptance Criteria:**
- [ ] tauri.conf.json updated per SEC-501
- [ ] `cargo build` passes
- [ ] `cargo test` passes
- [ ] Manual smoke test: load image, generate depth, adjust sliders — all work

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

### Section 5: IPC Performance Spike — Consultant Priority 5 (Medium)

> Addresses: Consultant Report §3.3, GOTCHAS.md "IPC large payloads slow". Must be evaluated before Sprint 1.6/1.7 mesh generation introduces even larger data transfers.

---

#### Task ARCH-501: IPC Performance Evaluation Spike
**Assigned Role:** System Architect
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** ARCH-501

**Dependencies:**
- None.

**What to Do:**
1. **Measure:** Time the IPC round-trip for depth maps at three resolutions:
   - 640×480 (~1.2M values → ~5MB JSON)
   - 1920×1080 (~2.1M values → ~8MB JSON)
   - 3840×2160 (~8.3M values → ~33MB JSON)
   Use `console.time`/`console.timeEnd` on the frontend side around `getDepthMap()` calls; `std::time::Instant` on the Rust side around serialization.
2. **Document:** Record timings in `SPRINTS/Sprint_1_5A/GOTCHAS.md` and/or a spike document.
3. **Evaluate alternatives** if latency exceeds 100ms for 1080p:
   - Binary transfer via temp file + `convertFileSrc()` (asset protocol)
   - Shared memory / memory-mapped file
   - Client-side depth adjustment (apply adjustments in JS, send only params)
   - Chunked WebSocket transfer
4. **Recommend:** Document a recommendation for Sprint 1.6/1.7 in the progress report. If binary transfer is recommended, outline the implementation approach.
5. **Note:** If asset protocol is disabled by SEC-501, the temp-file approach would need a custom Tauri command to read binary data. Factor this into the recommendation.

**Reference Documents:**
- `RESEARCH/GOTCHAS.md` — "Tauri v2 IPC large payloads slow"
- `src/lib/tauri.ts` — Current IPC pattern
- `src-tauri/src/lib.rs` — get_depth_map returns full array
- Consultant Report §3.3

**Acceptance Criteria:**
- [ ] Latency measured at 3 resolutions
- [ ] Results documented with methodology
- [ ] Alternative approach recommended if latency >100ms at 1080p
- [ ] Recommendation documented for Sprint 1.6/1.7

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

### Section 6: Model License Resolution — Consultant Priority 6 (Medium)

> Addresses: Consultant Report §1.2, §4 Priority 6. CC-BY-NC-4.0 on Depth-Anything-V2 weights conflicts with commercial use cases.

---

#### Task ARCH-502: Document Model License Decision
**Assigned Role:** System Architect
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** ARCH-502

**Dependencies:**
- None.

**What to Do:**
1. Create **ADR-005** in `RESEARCH/architecture.md`: "Depth Model Licensing and Commercial Use".
2. Document the current situation:
   - Depth-Anything-V2 weights: CC-BY-NC-4.0 (non-commercial only)
   - MiDaS weights: MIT license (commercial OK, but repo archived)
   - SimplePicture3D app: MIT license
3. **Decide** on one of these options (or document the team's choice):
   - **Option A:** Non-commercial only — document prominently, restrict to hobbyist use
   - **Option B:** Offer MiDaS as commercial-friendly default; Depth-Anything-V2 as optional higher-quality non-commercial model
   - **Option C:** Dual model support — user chooses at install time; license shown in model download wizard
   - **Option D:** Custom model with permissive license (deferred, high effort)
4. Update `docs/user-guide.md` and README with model license information visible to end users.
5. Plan how the model download wizard (Sprint 1.10) will surface license information.

**Reference Documents:**
- `prd.md` §9.2 — Licensing
- `docs/tech-stack-approval.md` — "License note"
- `RESEARCH/python-ml.md` — Model comparisons
- Consultant Report §1.2, original Priority 2

**Acceptance Criteria:**
- [ ] ADR-005 documented in RESEARCH/architecture.md
- [ ] License implications stated for each model option
- [ ] Decision recorded with rationale
- [ ] User-facing documentation updated (user-guide or README)

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task AI-502: Add License Notice to Python Depth Estimator
**Assigned Role:** Senior Researcher (AI/ML)
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** AI-502

**Dependencies:**
- ARCH-502: License decision made — Status: [ ]

**What to Do:**
1. Add a license notice to `python/python/depth_estimator.py` docstring and stderr output:
   - When Depth-Anything-V2 is loaded, emit `STAGE model_license: CC-BY-NC-4.0 (non-commercial)` to stderr.
   - When using stub mode, no license notice needed.
2. Add a `--show-license` CLI flag that prints model license info to stdout and exits.
3. If MiDaS is added as an option per ARCH-502, add corresponding license notice (MIT).

**Reference Documents:**
- `python/python/depth_estimator.py` — Source
- ARCH-502 decision

**Acceptance Criteria:**
- [ ] License notice emitted to stderr when real model is loaded
- [ ] `--show-license` flag prints license info
- [ ] Test added for `--show-license` output

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

### Section 7: Documentation Cleanup (Medium)

> Addresses: Consultant Report §3.5, §3.7; Sprint 1.5 artefact staleness; todo.md outdated metrics.

---

#### Task DOC-501: Update Sprint 1.5 Artefacts to Reflect Completion
**Assigned Role:** Documentation Specialist
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** DOC-501

**Dependencies:**
- None.

**What to Do:**
1. Update `SPRINTS/Sprint_1_5/PROGRESS_REPORT.md`:
   - Change all phases from "⏳ Not Started" to "✅ Complete".
   - Add deliverables list (depth_adjust.rs, DepthControls.svelte, pytest suite, CI clippy, etc.).
   - Note that Sprint 1.5A was created for hardening items.
2. Update `SPRINTS/Sprint_1_5/VERIFICATION_CHECKLIST.md`:
   - Check all boxes that were satisfied (cargo test PASS, clippy 0 warnings, npm build PASS, controls functional, manual tests executed).
   - Set sprint verification to "Complete".
   - Note: contrast slider gap carried to Sprint 1.5A.

**Reference Documents:**
- `SPRINTS/Sprint_1_5/PROGRESS_REPORT.md` — Stale document
- `SPRINTS/Sprint_1_5/VERIFICATION_CHECKLIST.md` — Unchecked boxes
- `SPRINTS/Sprint_1_5/MANUAL_TEST_REPORT.md` — Evidence of completion
- Git commit `70bbe45` — Sprint 1.5 completion commit

**Acceptance Criteria:**
- [ ] Progress report reflects actual completion state
- [ ] Verification checklist boxes checked where appropriate
- [ ] Contrast slider gap noted as carried to 1.5A

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task DOC-502: Update todo.md Testing Strategy Section
**Assigned Role:** Documentation Specialist
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** DOC-502

**Dependencies:**
- None — can start immediately; update again at sprint end with final numbers.

**What to Do:**
1. In `todo.md`, update the "Current state (as of 2026-02-06)" table (line ~1463):
   - **Rust:** Update from "27 unit/integration tests" to actual count (~46 passed, 5 ignored per manual test report). Note that clippy is now in CI.
   - **Frontend:** Note "Vitest being added in Sprint 1.5A" (or update after UI-502 complete).
   - **Python:** Update from "No pytest suite" to "19 tests, stub mode, in CI".
2. Update "Local test commands (verified)" section to match current state.
3. Update the "Required additions" list — strike through or mark as done items that are complete (clippy in CI, Python in CI).
4. Update Test Coverage Goals table with actual numbers once coverage tools are integrated (BACK-501, AI-501).

**Reference Documents:**
- `todo.md` lines ~1459–1514 — Testing Strategy
- `SPRINTS/Sprint_1_5/MANUAL_TEST_REPORT.md` — Current test counts

**Acceptance Criteria:**
- [ ] Test counts match reality
- [ ] Completed items marked as done
- [ ] Coverage goals table updated after BACK-501/AI-501

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task DOC-503: Update README with Python Setup Instructions
**Assigned Role:** Documentation Specialist
**Priority:** Medium
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** DOC-503

**Dependencies:**
- ARCH-502: License decision (for model license info) — Status: [ ] (can start without; add license info later)

**What to Do:**
1. In `README.md`, ensure the Python environment section includes:
   - Required Python version (3.10+)
   - Virtual environment setup (`python -m venv venv` + activate)
   - `pip install -r python/requirements.txt`
   - Stub mode testing: `SP3D_USE_STUB=1 pytest python/ -v`
   - Note about Depth-Anything-V2 model download (coming in Sprint 1.10)
   - License notice for model weights per ARCH-502 decision
2. Verify instructions work on a clean Python 3.10+ install.
3. Reference ADR-003 (System Python for MVP) from `RESEARCH/architecture.md`.

**Reference Documents:**
- `README.md` — Current state
- `RESEARCH/architecture.md` ADR-003 — Python distribution decision
- `python/README.md` — Python-specific docs
- `CLAUDE.md` — Testing commands (keep in sync)

**Acceptance Criteria:**
- [ ] README has clear Python setup section
- [ ] Instructions verified on clean environment
- [ ] Model license noted (after ARCH-502)

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

#### Task DOC-504: Update CLAUDE.md Testing Commands
**Assigned Role:** Documentation Specialist
**Priority:** Low
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete / [ ] Blocked
**Task ID:** DOC-504

**Dependencies:**
- UI-502: Vitest exists (for `npm test` command) — Status: [ ]

**What to Do:**
1. In `CLAUDE.md`, update the Testing Commands section:
   - Add `npm test` under Frontend.
   - Update Rust test count if documented.
   - Ensure Python stub mode commands match current practice.
2. Verify all commands work as documented.

**Reference Documents:**
- `CLAUDE.md` — Project guidance

**Acceptance Criteria:**
- [ ] `npm test` command listed
- [ ] All documented commands verified

**Completion Record:**
```
Status: [ ] Complete
Completed By: [Role] - [Agent ID]
Completed On: [Date]
Notes:
```

---

## Subtask Allocation (Multi-Role Tasks)

| Sub-task | Role | Owner | Status | Notes |
|----------|------|-------|--------|-------|
| UI-502 (Vitest setup) | UI Designer | - | [ ] | **Prerequisite** for all JR1/JR2 test tasks |
| UI-501 (Contrast slider) | UI Designer | - | [ ] | Prerequisite for JR1-501 contrast test cases |
| SEC-501 (Scope decision) | Security Specialist | - | [ ] | Prerequisite for BACK-502 implementation |
| ARCH-502 (License decision) | System Architect | - | [ ] | Prerequisite for AI-502 and DOC-503 license info |

---

## Dependency Graph

```
UI-502 (Vitest) ─────┬──→ JR2-501 (depthCanvas tests)
                      ├──→ JR2-502 (tauri.ts tests)
                      ├──→ JR1-501 (DepthControls tests) ←── UI-501 (contrast slider)
                      ├──→ JR1-502 (ImageImport tests)
                      └──→ UI-505 (npm test in CI)

SEC-501 (scope decision) ──→ BACK-502 (implement scope change) ──→ SEC-502 (capabilities review)

ARCH-502 (license decision) ──→ AI-502 (license notice in Python)
                                DOC-503 (README Python setup — can start early, add license later)

BACK-501 (tarpaulin) ───────→ DOC-502 (update todo.md with coverage baseline)
AI-501 (pytest-cov) ─────────→ DOC-502

DOC-501 (Sprint 1.5 artefacts) — no dependencies
DOC-504 (CLAUDE.md) ←── UI-502 (Vitest exists)
ARCH-501 (IPC spike) — no dependencies
```

---

## Success Criteria for Sprint

- [ ] `npm test` exists and passes with ≥15 frontend tests
- [ ] Contrast slider functional in DepthControls; all 5 adjustment controls work
- [ ] Coverage reporting in CI for both Rust (tarpaulin) and Python (pytest-cov)
- [ ] Asset protocol scope restricted or disabled; security sign-off completed
- [ ] IPC performance measured and documented; recommendation for Sprint 1.6/1.7
- [ ] Model license decision documented as ADR-005
- [ ] Sprint 1.5 artefacts updated to reflect completion
- [ ] todo.md Testing Strategy section reflects current state
- [ ] All quality metrics pass (see below)
- [ ] No blocking issues carried to Sprint 1.6
- [ ] Gotchas recorded in `SPRINTS/Sprint_1_5A/GOTCHAS.md`

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| None at sprint start | — | — |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS (all non-ignored) | — |
| cargo clippy -- -D warnings | 0 warnings | — |
| cargo fmt --check | PASS | — |
| npm run build | PASS | — |
| npm test | PASS, ≥15 tests | — |
| pytest (stub mode) | PASS, 19+ tests | — |
| cargo tarpaulin | Runs, baseline recorded | — |
| pytest --cov | Runs, baseline recorded | — |

---

## Progress Log (Handover Notes)

*Agents add handover notes when completing tasks that others depend on.*

```
### [Date] — [Role] (Task X.Y COMPLETED)
[What was delivered. Key files. Gotchas. Handover to whom.]
```

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **Consultant_Recommendations_Report_6Feb2026.md** — Updated review (source of this sprint)
3. **prd.md** — Acceptance criteria for your tasks
4. **todo.md** — Sprint 1.5 context, Testing Strategy section
5. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
6. **RESEARCH/GOTCHAS.md** — Known pitfalls before debugging
7. **RESEARCH/architecture.md** — ADRs (especially ADR-003 for Python distribution)
8. **docs/security-checklist.md** — Security review patterns (for SEC roles)

---

**Document Version:** 1.0
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`
**Status:** Ready for role claiming
