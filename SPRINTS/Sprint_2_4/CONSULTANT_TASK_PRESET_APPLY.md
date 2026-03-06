# Consultant Task: Preset Apply Does Not Update Depth Sliders

**Document type:** Tasking statement for external development consultant  
**Sprint:** 2.4 (Progress Streaming for Depth Estimation)  
**Created:** 2026-03-06  
**Priority:** P1 — blocks preset QA sign-off (QA-1301, QA-1302) and Sprint 2.4 verification  

---

## 1. Summary

**Problem:** In the SimplePicture3D desktop app (Tauri 2 + Svelte 4), applying a preset (built-in or user-saved) does **not** update the depth adjustment sliders or related settings. Saving presets and listing them (alongside built-in presets) works correctly; only the **apply** path fails to reflect the loaded preset in the UI.

**Requested work:** Investigate the root cause and implement a fix so that when the user selects and applies any preset, the depth adjustment controls (brightness, contrast, gamma, invert, depth min/max mm, and any curve) update to match the preset values.

---

## 2. Reproduction

1. Build and run the app: `npm run tauri dev` (see project root `CLAUDE.md` or `README.md` for full setup).
2. Load an image and generate a depth map (so that depth controls are enabled).
3. Change some sliders (e.g. Brightness to 0.2, Gamma to 1.5) and use **Save as preset** to save a preset (e.g. "Test Preset"). Confirm it appears in the preset list.
4. Use **Apply preset** (or **Load preset**) and choose either:
   - A **built-in** preset (e.g. "Portrait", "Landscape", "High Detail", "Low Relief"), or  
   - The **user** preset just saved.
5. **Expected:** Depth sliders and related settings update to the preset’s values.  
   **Actual:** Sliders do not change; UI does not reflect the applied preset.

Status text may show "Preset applied" (backend reports success) while the controls remain unchanged.

---

## 3. Technical Context

- **Stack:** Tauri v2 (Rust backend), Svelte 4 + TypeScript frontend, TailwindCSS.  
- **Relevant docs:** `prd.md`, `docs/architecture.md`, `RESEARCH/architecture.md`, `RESEARCH/tauri.md`.  
- **Preset feature:** Introduced in Sprint 2.3 (BACK-1301–1304, UI-1301–1304). Schema and flow are described in `RESEARCH/architecture.md` (Preset schema) and `docs/architecture.md`.

**Intended data flow when applying a preset:**

1. User selects a preset → frontend calls `loadPreset(nameOrPath)` (invoke `load_preset`).
2. Backend (`src-tauri/src/lib.rs`): `load_preset` resolves the preset (built-in or file), converts it to `DepthAdjustmentParams`, writes to `state.adjustment_params`, updates undo stack and app settings (curve, target size), then returns.
3. Frontend then calls `applyPresetAndRefresh()` which:
   - Calls `getDepthAdjustmentParams()` (invoke `get_depth_adjustment_params`),
   - Sets `adjustmentParams = { ...params }` (or equivalent),
   - Optionally refreshes depth map preview, histogram, undo/redo state, and settings.
4. `adjustmentParams` is passed as the `params` prop to `DepthControls.svelte`, which drives the sliders and inputs.

**Key files:**

| Area | Path | Notes |
|------|------|--------|
| Backend: load preset | `src-tauri/src/lib.rs` | `load_preset` command (~416); updates `state.adjustment_params` and app_settings. |
| Backend: get params | `src-tauri/src/lib.rs` | `get_depth_adjustment_params` (~818); returns current `state.adjustment_params`. |
| Backend: preset schema | `src-tauri/src/preset.rs` | `Preset`, `to_depth_params()`, `get_builtin_preset()`. |
| Backend: depth params type | `src-tauri/src/depth_adjust.rs` | `DepthAdjustmentParams` (serde `rename_all = "camelCase"`). |
| Frontend: load + refresh | `src/App.svelte` | `handleLoadPreset()`, `applyPresetAndRefresh()`; `adjustmentParams` state. |
| Frontend: API | `src/lib/tauri.ts` | `loadPreset()`, `getDepthAdjustmentParams()`, `DepthAdjustmentParams` interface. |
| Frontend: controls | `src/components/DepthControls.svelte` | Receives `params` prop; sliders bound to `params.brightness`, `params.depthMinMm`, etc. |

**Already attempted (no improvement):** In `App.svelte`, `applyPresetAndRefresh()` was changed to set `adjustmentParams = { ...params }` to force a new object reference for Svelte reactivity. Rebuild and retest did not fix the issue.

---

## 4. Scope of Work

1. **Investigate** why the depth controls do not update after applying a preset. Considerations (non-exhaustive):
   - Backend: Does `load_preset` actually write the correct values to `state.adjustment_params`? Is the same state read by `get_depth_adjustment_params`?
   - IPC: Does the frontend receive the updated params from `get_depth_adjustment_params` after `load_preset` (e.g. correct keys/camelCase, correct values)?
   - Frontend: Is `adjustmentParams` updated and does the `DepthControls` component re-render with the new `params`? Any Svelte reactivity or binding issues (e.g. `value` vs `bind:value`, or child holding internal state)?
   - Ordering: Is there a race or a second update that overwrites the applied params (e.g. another command or reactive block)?
2. **Implement a fix** that ensures applying any preset (built-in or user) updates all relevant depth adjustment controls to the preset values.
3. **Verify** using the steps in §2 (and, if available, `SPRINTS/Sprint_2_4/TEST_PLAN_2_4.md` §2 Preset tests). Confirm that built-in presets and user-saved presets both update the sliders, and that status reflects success/failure appropriately.

---

## 5. Acceptance Criteria

- [ ] When the user applies a **built-in** preset (Portrait, Landscape, High Detail, Low Relief), the depth sliders and related settings (brightness, contrast, gamma, invert, depth min/max mm, curve if applicable) update to match that preset.
- [ ] When the user applies a **user-saved** preset from the list, the same controls update to the saved values.
- [ ] When the user **imports** a preset from a JSON file, the applied preset’s values are reflected in the UI (same flow as load/apply).
- [ ] No regression: saving presets, listing presets, and the rest of the depth/adjustment workflow still work as before.
- [ ] Existing tests remain green: `cargo test --manifest-path src-tauri/Cargo.toml`, `npm test`, `cargo clippy -- -D warnings`, `npm run build`.

---

## 6. Deliverables

1. **Code change(s)** that fix the preset-apply behaviour, with minimal scope and clear comments where non-obvious.
2. **Short written summary** (e.g. in a PR description or handover note) describing:
   - Root cause of the bug,
   - What was changed to fix it,
   - How you verified (manual steps and/or tests).
3. **Update** (or recommendation) for any test that should cover “apply preset → UI shows preset values” (e.g. in `src-tauri` or `src/lib`/`src/components`), if applicable.

---

## 7. References

| Document | Purpose |
|----------|---------|
| `SPRINTS/Sprint_2_4/MANUAL_TEST_REPORT.md` | QA results; preset apply failure and issue table. |
| `SPRINTS/Sprint_2_4/GOTCHAS.md` | Entry “Preset Apply does not update sliders” and other Sprint 2.4 gotchas. |
| `SPRINTS/Sprint_2_4/TEST_PLAN_2_4.md` | Test plan §2 (Preset tests QA-1301–1303). |
| `SPRINTS/Sprint_2_4/SPRINT_2_4_Task_Assignment.md` | Sprint scope and preset task IDs (JR2-1301–1303, QA-1301–1303). |
| `docs/architecture.md` | High-level commands and progress; preset flow. |
| `RESEARCH/architecture.md` | ADRs, preset schema, BACK-130x. |
| `RESEARCH/GOTCHAS.md` | Project-wide gotchas (merge from sprint GOTCHAS if needed). |
| `CLAUDE.md` (project root) | Build, test, and run commands; repo layout. |

---

**Document version:** 1.0  
**Status:** Ready for consultant handoff
