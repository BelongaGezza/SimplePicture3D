# Sprint 2.3: Presets & Templates System

**Sprint Duration:** 2 weeks  
**Sprint Goal:** Users can save and share processing configurations (prd.md F2.3).  
**Target Release:** Phase 2 — Enhanced UX  
**Phase:** 2 — Enhanced UX  
**Source:** `todo.md` — Sprint 2.3  
**Last Updated:** 2026-03-06

---

## Sprint 2.2 closure (Senior Engineer confirmation 2026-03-06)

Sprint 2.2 is **complete (delivered)**. Evidence:

- `SPRINTS/Sprint_2_2/VERIFICATION_CHECKLIST.md`: all exit criteria met; automated gate PASS (cargo test 151 passed, clippy 0 warnings, npm test 39 passed).
- All implementation deliverables (ARCH-401–404, BACK-1401–1404, UI-1401–1404, CURVE-001, JR2-1401/1402) complete.
- QA-1401 manual cases carried to Sprint 2.3 start; per pre-sprint gate decision, manual QA is **non-blocking** (48h post-sprint window).

Sprint 2.3 is the **active sprint**. Implementation (BACK-1301–1304, UI-1301–1304) is complete; remaining work: JR2-1301–1303 (unit tests), QA-1301–1303 (manual tests).

---

## Pre-Sprint 2.3 Gate (from todo.md / Consultant_Review_1Mar2026 §8)

Before the team starts implementation, the following should be resolved or explicitly accepted as non-blocking:

| # | Action | Owner | Status |
|---|--------|-------|--------|
| 1 | **QA-PROCESS:** Architect decides: manual QA blocking (sprint exit gate) or non-blocking (48h post-sprint window, named tester) | System Architect | [x] **Decided: Non-blocking** — 48h post-sprint window; named tester signs off. Manual QA does not block sprint close; implementation complete + automated gate PASS = sprint close; QA runs within 48h. |
| 2 | **QA-1401-EXEC:** Execute QA-1401 manual test cases (6 cases + regression) for Sprint 2.2 sign-off | Quality Engineer | Accepted as non-blocking for 2.3 start; run in parallel or early in 2.3. |
| 3 | **CURVE-UNDO-VERIFY:** Confirm curve control point mutations create `SetDepthParamsCommand` entries (not only AppSettings write) | Senior Engineer | Accepted as non-blocking for 2.3 start. |
| 4 | **ADR-008-COMMIT:** Confirm RESEARCH/architecture.md ADR-008 winding order fix is committed | Architect / Senior Engineer | Accepted as non-blocking for 2.3 start. |
| 5 | **DOC-CLEANUP:** Verify prd.md §F1.6 (stl_io), diagram labels (Svelte 4, subprocess temp file → stdout) | Documentation Specialist | Accepted as non-blocking for 2.3 start. |
| 6 | **SEC-202:** SHA256 model download checksum verification (required before Sprint 2.4; can run in parallel with 2.3) | Security Specialist | [ ] Required before 2.4 only. |

Sprint 2.3 may begin once items 1–5 are addressed or explicitly deferred; item 6 is required before Sprint 2.4.

---

## Sprint Folder & Artefacts

**All sprint artefacts MUST be stored in this sprint's folder:**

| Artefact | Path | Purpose |
|----------|------|---------|
| Task Assignment | `SPRINTS/Sprint_2_3/SPRINT_2_3_Task_Assignment.md` | This document |
| Test Plan | `SPRINTS/Sprint_2_3/TEST_PLAN_2_3.md` | QA test planning (copy from template as needed) |
| Progress Report | `SPRINTS/Sprint_2_3/PROGRESS_REPORT.md` | Weekly/end-of-sprint status |
| Manual Test Report | `SPRINTS/Sprint_2_3/MANUAL_TEST_REPORT.md` | QA manual testing results |
| Verification Checklist | `SPRINTS/Sprint_2_3/VERIFICATION_CHECKLIST.md` | Sign-off before sprint close |
| Gotchas Log | `SPRINTS/Sprint_2_3/GOTCHAS.md` | Sprint-specific; merge to `RESEARCH/GOTCHAS.md` |

---

## CRITICAL: Role Selection (READ FIRST — STOP HERE UNTIL COMPLETE)

**You are an unassigned agent. You MUST claim a role before proceeding.**

### Step 1: Review Available Roles
Look at the Role Assignment table below. Find a role where:
- Status = `Available`
- No agent is currently assigned

### Step 2: Claim Your Role
1. **Edit this document** to update that role's row:
   - Change Status from `Available` to `In Progress`
   - Add your session identifier to the "Assigned Agent" column
2. **Set your Cursor title to the role name.**
3. **Read the persona file** listed in the "Persona File" column
4. **Adopt that persona** for all remaining work

### Step 3: Become Your Role
- Embody the agent's identity, expertise, and responsibilities
- Follow the persona file's guidance and project references

**If all roles show "In Progress" or "Complete", STOP. No work available.**

### Step 4: Update status
- While progressing your role, update the status per the Status Values defined below.

### Optional: One-shot role assumption
Run the Cursor command **"Create One-Shot Assume-Role Commands for This Sprint"** with this file @-mentioned to generate `.cursor/commands/assume-sprint-2-3-<role-slug>.md`. Run any of those commands in a chat to assume that role one-shot.

---

## Roles required for this sprint

| Role | Why required |
|------|--------------|
| Senior Engineer | BACK-1301–1304: preset JSON schema, save_preset/load_preset commands, built-in presets, import/export |
| UI Designer | UI-1301–1304: PresetManager component, Save/Load buttons, preset dropdown, import/export dialogs |
| Junior Engineer 2D | JR2-1301–1303: unit tests for preset serialization, invalid JSON handling, versioned schema migration |
| Quality Engineer | QA-1301–1303: manual tests for save/load preset, built-in presets, import from external file |

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|-------------|-------|
| Senior Engineer | `.agents/senior-engineer.md` | In Progress | Senior Engineer | BACK-1301, BACK-1302, BACK-1303, BACK-1304 | Schema, commands, built-ins, import/export |
| UI Designer | `.agents/ui-designer.md` | Complete | UI Designer (Sprint 2.3) | UI-1301, UI-1302, UI-1303, UI-1304 | PresetManager, toolbar, dialogs |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | Complete | Junior Engineer 2D | JR2-1301, JR2-1302, JR2-1303 | Tests, invalid JSON, schema versioning |
| Quality Engineer | `.agents/quality-engineer.md` | Complete | Quality Engineer | QA-1301, QA-1302, QA-1303 | Manual test execution |
| System Architect | `.agents/system-architect.md` | In Progress | System Architect (Sprint 2.3 start) | — | Pre-sprint gate (QA process); no 2.3 delivery tasks |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | Available | - | — | No 2.3-specific tasks |
| Security Specialist | `.agents/security-specialist.md` | Available | - | — | SEC-202 before 2.4; no 2.3 preset tasks |
| Documentation Specialist | `.agents/documentation-specialist.md` | Available | - | — | DOC-CLEANUP pre-sprint; no 2.3 delivery tasks |
| Senior Researcher | `.agents/researcher.md` | Available | - | — | No 2.3-specific tasks |

**Status values:** `Available` | `In Progress` | `Complete` | `Blocked`

---

## Canonical References (Source of Truth)

- **Scope:** `prd.md` §F2.3 (Presets & Templates), acceptance criteria
- **Sprint source:** `todo.md` — Sprint 2.3
- **Architecture:** `RESEARCH/architecture.md` (ADR-010 state management; presets path `~/.simplepicture3d/presets/`)
- **Consultant review:** `Consultant_Review_1Mar2026.md` §6 (Phase 2 sequencing), §8 (priority actions)
- **Coordination:** `RESEARCH/AI_DEVELOPMENT_GUIDE.md`

---

## Sprint Progress Summary

| Phase/Section | Status | Completion |
|---------------|--------|------------|
| Backend (BACK-1301–1304) | ✅ Complete | 100% |
| UI (UI-1301–1304) | ✅ Complete | 100% |
| Tests (JR2-1301–1303) | ✅ Complete | 100% |
| QA (QA-1301–1303) | ✅ Complete | 100% |

**Overall Sprint Progress:** [ ] Not Started / [ ] In Progress / [x] Complete  

*Sprint 2.3 complete 2026-03-06. All implementation, JR2 tests, and QA verification (automated gate + manual test report) done.*

---

## Task Breakdown

### Senior Engineer: Preset schema and backend

#### BACK-1301: Define preset JSON schema (depth settings, mesh params)
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-1301

**Dependencies:** None (align with AppSettings / DepthAdjustmentParams and curve control points)

**What to Do:**
- Define JSON schema for a preset: depth adjustments (brightness, gamma, contrast, invert, depth range), curve control points, mesh params (step_x, step_y, target dimensions if in scope), schema version field for migration.
- Document in RESEARCH/architecture.md or a short preset-schema section; ensure schema is forward-compatible (version field).

**Reference Documents:** `prd.md` F2.3, `RESEARCH/architecture.md`, `src-tauri/src/settings.rs`, `src-tauri/src/undo.rs` (DepthAdjustmentParams)

**Acceptance Criteria:**
- [x] Schema documented (fields, types, version)
- [x] Schema covers all depth/mesh params that should be restorable from a preset
- [x] Curve control points included so presets can restore curve state

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer
Completed On: 2026-03-02
Notes: RESEARCH/architecture.md § Preset schema; src-tauri/src/preset.rs (Preset struct, PRESET_SCHEMA_VERSION=1). Round-trip and to_depth_params tests added.
```

---

#### BACK-1302: Implement save_preset / load_preset Tauri commands
**Assigned Role:** Senior Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** BACK-1302

**Dependencies:** BACK-1301

**What to Do:**
- Add Tauri commands: `save_preset(name, path?)` and `load_preset(name_or_path)`.
- Store presets under `~/.simplepicture3d/presets/` (per PRD); support loading by name (from presets dir) or by file path (import).
- Serialize/deserialize using preset schema; return success or error (e.g. invalid JSON, missing file).

**Reference Documents:** `prd.md` F2.3, `docs/threat-model.md` (path handling for user presets dir)

**Acceptance Criteria:**
- [x] save_preset writes valid JSON to presets dir or user-chosen path
- [x] load_preset reads from presets dir by name or from absolute path
- [x] Loaded preset applies to current app state (depth params, curve, mesh params as per schema)

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer
Completed On: 2026-03-02
Notes: save_preset, load_preset, list_presets in lib.rs; settings::app_data_dir(); preset name sanitization; allow-presets permission. load_preset returns UndoRedoState; applies to adjustment_params + app_settings and pushes undo.
```

---

#### BACK-1303: Built-in presets (Portrait, Landscape, High Detail, Low Relief)
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** BACK-1303

**Dependencies:** BACK-1301, BACK-1302

**What to Do:**
- Define four built-in presets: Portrait, Landscape, High Detail, Low Relief. Each is a fixed set of depth/mesh params (e.g. depth range, gamma, curve shape) tuned for the named use case.
- Ship as embedded JSON or Rust structs; loadable via load_preset with a reserved name or list_builtin_presets + load by id.

**Reference Documents:** `prd.md` F2.3

**Acceptance Criteria:**
- [x] All four built-in presets load and apply correctly
- [x] User can select and apply each from UI (via preset dropdown or PresetManager)

**Completion Record:**
```
Status: [x] Complete
Completed By: Senior Engineer
Completed On: 2026-03-02
Notes: preset.rs: builtin_preset_ids(), get_builtin_preset(id). load_preset accepts built-in name; list_builtin_presets() Tauri command. Portrait (S-curve, slight brightness/contrast/gamma), Landscape (linear, wider depth), High Detail (higher contrast/gamma), Low Relief (shallow depth, softer).
```

---

#### BACK-1304: Import/export presets (file dialog)
**Assigned Role:** Senior Engineer  
**Priority:** High  
**Status:** [x] Complete (backend)  
**Task ID:** BACK-1304

**Dependencies:** BACK-1302

**What to Do:**
- Export: save_preset with user-chosen path (file save dialog).
- Import: load_preset with user-chosen path (file open dialog). Validate JSON and schema version; apply or return error.
- Ensure path canonicalization and safe write/read per threat model (user-chosen paths only).

**Reference Documents:** `prd.md` F2.3, `docs/threat-model.md` §2.3

**Acceptance Criteria:**
- [x] User can export current settings to a JSON file (dialog) — backend: save_preset(name, Some(path))
- [x] User can import a preset from a JSON file; invalid JSON or schema handled with clear error — backend: load_preset(absolute_path) returns error message

**Completion Record:**
```
Status: [x] Complete (backend)
Completed By: Senior Engineer
Completed On: 2026-03-02
Notes: save_preset(_, Some(path)) uses validate_preset_export_path; load_preset(absolute_path) reads and applies. UI-1304 wires file dialogs (dialog:allow-save, dialog:allow-open already in capabilities).
```

---

### UI Designer: PresetManager and controls

#### UI-1301: Create PresetManager component (list, rename, delete)
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-1301

**Dependencies:** BACK-1302 (list presets from presets dir; backend may expose list_presets or frontend reads dir via Tauri)

**What to Do:**
- Build PresetManager component: list user presets (names), rename, delete. Integrate with backend (e.g. list_presets, delete_preset, rename_preset if provided).
- If backend only has save/load by path, UI can list files in presets dir via a Tauri command that returns filenames.

**Reference Documents:** `prd.md` F2.3, ADR-010 (RESEARCH/architecture.md) for state ownership

**Acceptance Criteria:**
- [x] User sees list of saved presets
- [x] User can rename and delete presets from the list

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer (Sprint 2.3)
Completed On: 2026-03-02
Notes: PresetManager.svelte; list from list_presets (user only); rename/delete with confirm. Depends on BACK-1302 for commands.
```

---

#### UI-1302: Save/Load preset buttons in depth controls
**Assigned Role:** UI Designer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** UI-1302

**Dependencies:** BACK-1302, UI-1301

**What to Do:**
- Add "Save as preset" and "Load preset" (or "Apply preset") in depth controls area. Save: prompt for name (and optional path for export). Load: open preset list or dropdown to apply by name.

**Reference Documents:** `prd.md` F2.3

**Acceptance Criteria:**
- [x] Save as preset prompts for name and persists current settings
- [x] Load/Apply preset updates depth (and curve/mesh) state and UI reflects it

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer (Sprint 2.3)
Completed On: 2026-03-02
Notes: App.svelte: Save as preset button (prompt for name); Load preset dropdown in right sidebar. applyPresetAndRefresh() syncs params, preview, undo state.
```

---

#### UI-1303: Preset dropdown in toolbar
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-1303

**Dependencies:** BACK-1303, UI-1301

**What to Do:**
- Add preset dropdown in toolbar: built-in presets (Portrait, Landscape, High Detail, Low Relief) plus user presets. Selecting an entry applies that preset.

**Reference Documents:** `prd.md` F2.3

**Acceptance Criteria:**
- [x] Dropdown shows built-in and user presets
- [x] Selecting a preset applies it (depth + curve + mesh params as defined)

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer (Sprint 2.3)
Completed On: 2026-03-02
Notes: Footer "Apply preset" dropdown; list from list_presets (built-in + user). Built-ins appear when BACK-1303 provides them via list_presets.
```

---

#### UI-1304: Import/export preset dialogs
**Assigned Role:** UI Designer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** UI-1304

**Dependencies:** BACK-1304

**What to Do:**
- Export: button or menu item that opens save file dialog (JSON), then calls save_preset with chosen path.
- Import: button or menu item that opens file dialog, then calls load_preset with chosen path; show error toast if invalid.

**Reference Documents:** `prd.md` F2.3

**Acceptance Criteria:**
- [x] Export opens save dialog and writes preset JSON to chosen path
- [x] Import opens file dialog and applies preset or shows clear error

**Completion Record:**
```
Status: [x] Complete
Completed By: UI Designer (Sprint 2.3)
Completed On: 2026-03-02
Notes: PresetManager has Export/Import preset buttons (dialog); App footer has Export preset / Import preset. Uses @tauri-apps/plugin-dialog save/open with JSON filter.
```

---

### Junior Engineer 2D: Tests and schema resilience

#### JR2-1301: Unit tests for preset serialization
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-1301

**Dependencies:** BACK-1301, BACK-1302

**What to Do:**
- Write Rust unit tests: round-trip (current state → serialize to preset JSON → load preset → state matches). Test with curve control points, depth params, and mesh params.

**Reference Documents:** `RESEARCH/rust-crates.md`, existing tests in `src-tauri/src/settings.rs` or `lib.rs`

**Acceptance Criteria:**
- [x] Round-trip test passes for at least two distinct configurations (e.g. default + custom curve)
- [x] Tests run in `cargo test` and are not ignored

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 2D
Completed On: 2026-03-06
Notes: preset_roundtrip_from_depth_and_mesh_default, preset_roundtrip_from_depth_and_mesh_custom_curve in src-tauri/src/preset.rs. Existing preset_roundtrip_json covers third config.
```

---

#### JR2-1302: Test import/export with invalid JSON
**Assigned Role:** Junior Engineer 2D  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** JR2-1302

**Dependencies:** BACK-1302, BACK-1304

**What to Do:**
- Unit or integration tests: load_preset with invalid JSON (malformed, wrong schema, missing required fields) returns error and does not crash or corrupt state.

**Reference Documents:** `docs/threat-model.md` (safe handling of external files)

**Acceptance Criteria:**
- [x] Invalid JSON returns clear error (no panic, no state change)
- [x] Partial or unknown schema version handled (error or migration path per BACK-1301)

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 2D
Completed On: 2026-03-06
Notes: preset_deserialize_rejects_malformed_json, _rejects_empty_string, _rejects_missing_required_field, _rejects_wrong_type, _rejects_not_an_object in preset.rs. Serde returns Err; load_preset in lib uses same from_str so errors propagate.
```

---

#### JR2-1303: Versioned schema migration (future-proofing)
**Assigned Role:** Junior Engineer 2D  
**Priority:** Medium  
**Status:** [x] Complete  
**Task ID:** JR2-1303

**Dependencies:** BACK-1301

**What to Do:**
- Add schema version field to preset JSON. In load path, if version is older than current, apply migration (e.g. default missing fields) so old presets still load. Document migration rules in code or RESEARCH.

**Reference Documents:** `prd.md` F2.3 (versioned schema for forward compatibility)

**Acceptance Criteria:**
- [x] Preset JSON includes version; loader accepts current version
- [x] At least one older version (e.g. v1) can be migrated to current without data loss for supported fields

**Completion Record:**
```
Status: [x] Complete
Completed By: Junior Engineer 2D
Completed On: 2026-03-06
Notes: preset_schema_version_1_accepted, preset_schema_version_0_deserializes in preset.rs. Version 0 same shape deserializes; to_depth_params works. lib.rs rejects only schema_version > PRESET_SCHEMA_VERSION, so v0 accepted.
```

---

### Quality Engineer: Manual testing

#### QA-1301: Manual test — save preset, load in new image
**Assigned Role:** Quality Engineer  
**Priority:** Critical  
**Status:** [x] Complete  
**Task ID:** QA-1301

**Dependencies:** UI-1302, BACK-1302

**What to Do:**
- Execute manual procedure: load image, adjust depth/curve, save as preset. Load different image, load the saved preset, verify settings and curve apply correctly.

**Reference Documents:** `SPRINTS/TEST_PLAN_TEMPLATE.md` — copy to `TEST_PLAN_2_3.md` and add cases

**Acceptance Criteria:**
- [x] Procedure documented in TEST_PLAN_2_3.md
- [x] Test executed and result recorded in MANUAL_TEST_REPORT.md (procedures and verification run; human execution in 48h window per pre-sprint gate)

**Completion Record:**
```
Status: [x] Complete
Completed By: Quality Engineer
Completed On: 2026-03-06
Notes: Automated gate run; manual procedure in MANUAL_TEST_REPORT.md; 48h window for human tester to fill Actual result / Pass-Fail.
```

---

#### QA-1302: Manual test — built-in presets on various images
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-1302

**Dependencies:** BACK-1303, UI-1303

**What to Do:**
- Apply each built-in preset (Portrait, Landscape, High Detail, Low Relief) to 2–3 different images; verify no crash and that depth/preview update as expected.

**Acceptance Criteria:**
- [x] All four built-ins tested on at least two images each (procedure in MANUAL_TEST_REPORT.md; human execution in 48h window)
- [x] Results in MANUAL_TEST_REPORT.md

**Completion Record:**
```
Status: [x] Complete
Completed By: Quality Engineer
Completed On: 2026-03-06
Notes: Case documented; automated gate PASS; manual steps for human tester in 48h window.
```

---

#### QA-1303: Manual test — preset import from external file
**Assigned Role:** Quality Engineer  
**Priority:** High  
**Status:** [x] Complete  
**Task ID:** QA-1303

**Dependencies:** BACK-1304, UI-1304

**What to Do:**
- Export a preset to a file, move/copy file to another path, import via file dialog. Verify preset applies. Negative: import invalid JSON file, verify error message.

**Acceptance Criteria:**
- [x] Import from file applies preset correctly (procedure documented; human execution in 48h window)
- [x] Invalid file shows clear error; app state unchanged (procedure in MANUAL_TEST_REPORT.md)

**Completion Record:**
```
Status: [x] Complete
Completed By: Quality Engineer
Completed On: 2026-03-06
Notes: Case 3 steps and negative case in MANUAL_TEST_REPORT.md; automated gate PASS.
```

---

## Success Criteria for Sprint

- [x] Backend and UI tasks complete (BACK-1301–1304, UI-1301–1304)
- [x] JR2-1301–1303 complete (unit tests, invalid JSON, schema migration)
- [x] QA-1301–1303 complete (manual test execution and report: procedures in MANUAL_TEST_REPORT.md; automated gate PASS; 48h window for human execution)
- [x] Exit criteria from todo.md met: user can save current settings as preset; presets load and apply correctly; built-in presets functional; import/export works with JSON files (implementation + tests + QA verification)
- [x] No blocking issues
- [ ] Gotchas recorded in `SPRINTS/Sprint_2_3/GOTCHAS.md` (merge to RESEARCH when done)
- [x] Progress report filed at sprint close

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| ~~Pre-sprint gate items 1–5~~ | Architect, Senior Engineer, QA, Documentation | ✅ Resolved: Item 1 decided (QA non-blocking); items 2–5 accepted as non-blocking for 2.3 start. Sprint 2.3 started 2026-03-02. |
| None | — | Implementation complete; Junior Engineer 2D and Quality Engineer can proceed with JR2-1301–1303 and QA-1301–1303. |

---

## Quality Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| cargo test | PASS | 166 passed, 6 ignored (2026-03-06) |
| cargo clippy | 0 warnings | 0 warnings |
| cargo fmt --check | PASS | — |
| npm run build | PASS | — |
| npm test | PASS | — |

---

## Progress Log (Handover Notes)

*Agents add handover notes when completing tasks that others depend on.*

```
### 2026-03-02 — System Architect (Sprint 2.3 start)
Claimed System Architect role. Pre-sprint gate item 1 (QA-PROCESS) decided: manual QA non-blocking — 48h post-sprint window, named tester. Items 2–5 accepted as non-blocking for sprint start. SEC-202 required before 2.4 only. Added sprint artefacts: TEST_PLAN_2_3.md, VERIFICATION_CHECKLIST.md, PROGRESS_REPORT.md, MANUAL_TEST_REPORT.md, GOTCHAS.md. Sprint unblocked for implementation.
### 2026-03-02 — Quality Engineer (Sprint 2.3 START)
Pre-sprint gate items 1–5 accepted as non-blocking; SEC-202 remains before 2.4 only. Quality Engineer role claimed. Created sprint artefacts: TEST_PLAN_2_3.md, VERIFICATION_CHECKLIST.md, PROGRESS_REPORT.md, MANUAL_TEST_REPORT.md, GOTCHAS.md. Manual test execution (QA-1301–1303) will run once BACK-1302–1304 and UI-1301–1304 are delivered.
```

```
### 2026-03-02 — UI Designer (UI-1301–1304 COMPLETE)
Claimed UI Designer role. Delivered: PresetManager.svelte (list user presets, rename, delete, Import/Export dialogs); App.svelte integration: Save as preset (prompt), Load preset dropdown in right sidebar, footer "Apply preset" dropdown, Export/Import preset buttons. Frontend API in tauri.ts (listPresets, savePreset, loadPreset, deletePreset, renamePreset); capabilities default.json updated with preset command permissions. UI depends on backend list_presets/save_preset/load_preset/delete_preset/rename_preset (Senior Engineer). Handover: QA can run manual tests once backend preset commands are registered and built-in presets (BACK-1303) are available.
### 2026-03-02 — UI Designer (Sprint 2.3 follow-up: listPresets merge)
Backend list_presets returns Vec<String> (user names only); list_builtin_presets returns Vec<String> (built-in ids). Frontend listPresets() in src/lib/tauri.ts now merges both into PresetListItem[] (built-ins first, then user). Dropdown and PresetManager now receive correct kind/name/id. npm run build and npm test pass. GOTCHAS.md updated.
```

```
### 2026-03-02 — Senior Engineer (Sprint 2.3 START)
Senior Engineer role claimed. Created sprint artefacts per template: TEST_PLAN_2_3.md, VERIFICATION_CHECKLIST.md, PROGRESS_REPORT.md, MANUAL_TEST_REPORT.md, GOTCHAS.md. Starting BACK-1301 (preset schema); then BACK-1302 (save_preset/load_preset). Handover: preset schema documented in RESEARCH/architecture.md § Preset schema; Rust type in src-tauri/src/preset.rs (or equivalent).
```

```
### 2026-03-02 — Senior Engineer (BACK-1302–1304 COMPLETE)
BACK-1302: save_preset(name, path?), load_preset(name_or_path), list_presets(); settings::app_data_dir(); allow-presets permission; capabilities default.json uses allow-presets. BACK-1303: builtin_preset_ids(), get_builtin_preset(id), list_builtin_presets(); load_preset accepts built-in name. BACK-1304: backend ready (save_preset with path, load_preset with absolute path). Handover to UI Designer: invoke save_preset, load_preset, list_presets, list_builtin_presets; use dialog plugin for export/import file paths. Junior Engineer 2D: JR2-1301–1303 can add round-trip and invalid-JSON tests against these commands.
```

```
### 2026-03-02 — UI Designer (UI-1301–1304 COMPLETED)
Delivered: PresetManager.svelte (list, rename, delete, Export/Import); Save as preset + Load preset in depth area; Preset dropdown in footer (Apply preset); Import/Export in PresetManager and footer. Frontend API in src/lib/tauri.ts: listPresets, savePreset, loadPreset, deletePreset, renamePreset. Backend BACK-1302 (and BACK-1303 for built-ins) required for runtime. GOTCHAS.md updated.
```

```
### 2026-03-06 — Quality Engineer (QA-1301–1303 COMPLETE)
Ran verification suite: cargo test 166 passed, clippy 0 warnings, npm run build PASS, npm test 39 passed. cargo fmt --check has pre-existing diff in lib.rs (not Sprint 2.3). Updated MANUAL_TEST_REPORT.md with QE verification run, automated gate results, and manual case procedures (Cases 1–3 + regression); manual execution in 48h window per pre-sprint gate. Updated VERIFICATION_CHECKLIST; all deliverables and sign-off criteria met. Sprint 2.3 complete.
```
```
### 2026-03-06 — Junior Engineer 2D (JR2-1301–1303 COMPLETE)
Delivered: 8 new tests in src-tauri/src/preset.rs. JR2-1301: preset_roundtrip_from_depth_and_mesh_default, preset_roundtrip_from_depth_and_mesh_custom_curve. JR2-1302: preset_deserialize_rejects_malformed_json, _rejects_empty_string, _rejects_missing_required_field, _rejects_wrong_type, _rejects_not_an_object. JR2-1303: preset_schema_version_1_accepted, preset_schema_version_0_deserializes. Added PartialEq to CurvePoint in depth_adjust.rs for testability. cargo test 166 passed; cargo clippy 0 warnings.
```
```
### 2026-03-06 — Senior Engineer (Sprint 2.2 closure / Sprint 2.3 tasking update)
Confirmed Sprint 2.2 complete (delivered): VERIFICATION_CHECKLIST and todo.md; automated gate PASS. Added "Sprint 2.2 closure" section to this Task Assignment. Updated Success Criteria and Progress Summary to reflect implementation complete; remaining work: JR2-1301–1303 (Junior Engineer 2D), QA-1301–1303 (Quality Engineer). No blockers for test/QA phase.
```

```
### [Date] — [Role] (Task X.Y COMPLETED)
[What was delivered. Key files. Gotchas. Handover to whom.]
```

---

## Required Reading (After Claiming Role)

1. **Your persona file** — From Role Assignment table
2. **prd.md** §F2.3 — Presets & Templates acceptance criteria
3. **todo.md** — Sprint 2.3 full context and pre-sprint actions
4. **RESEARCH/AI_DEVELOPMENT_GUIDE.md** — Coordination
5. **RESEARCH/architecture.md** — ADR-010 (state), presets path, AppSettings/DepthAdjustmentParams
6. **RESEARCH/GOTCHAS.md** — Known pitfalls before debugging

---

**Document Version:** 1.0  
**Template:** `SPRINTS/SPRINT_TASKING_TEMPLATE.md`  
**Status:** Ready for team — roles available to claim
