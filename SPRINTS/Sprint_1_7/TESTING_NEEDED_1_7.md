# Sprint 1.7 — Testing Needed to Bring Us Up to Date

**Role:** Quality Engineer  
**Source:** `SPRINT_1_7_Task_Assignment.md`, `TEST_PLAN_1_7.md`, `MANUAL_TEST_REPORT.md`, `VERIFICATION_CHECKLIST.md`, `todo.md` § Testing Strategy  
**Last Updated:** 2026-02-08

---

## 1. Current State

| Area | Status | Notes |
|------|--------|------|
| **UI (UI-501–506)** | ✅ Complete | Three.js, scene, mesh load, point cloud, orbit controls, render-mode toggle |
| **Backend (BACK-601–603)** | ✅ Complete | get_mesh_data (JSON), serialization, optional preview_step |
| **Junior 3D (JR1-501–505)** | ⏳ Not started | Camera presets, grid refinement, mesh stats, performance testing, lighting controls |
| **Quality (QA-601–604)** | ⏳ Not started | No manual tests run; report template only |

**Automated tests today:**  
- **Rust:** `cargo test` / `cargo clippy` (existing; no new 1.7-specific backend tests required).  
- **Frontend:** `npm test` — 34 Vitest tests; `getMeshData` covered in `src/lib/__tests__/tauri.test.ts` (invoke args, previewStep, null, errors). **No** `Preview3D.svelte` component test yet.  
- **Python:** pytest (stub) in CI — unchanged for 1.7.

---

## 2. Testing Needed to Bring Sprint 1.7 Up to Date

### 2.1 Mandatory — Quality Engineer

| ID | Task | Artefact | Blocker? |
|----|------|----------|----------|
| **QA-601** | Manual test: rotate/zoom/pan mesh; verify responsiveness; at least two resolutions (e.g. 640×480, 1080p) | `MANUAL_TEST_REPORT.md` § Case 2 | None. Camera presets (JR1-501) optional for this case. |
| **QA-602** | Test on integrated GPU (e.g. Intel UHD) and, if available, dedicated GPU; document FPS/stability | `MANUAL_TEST_REPORT.md` § Case 5 | None. |
| **QA-603** | Verify mesh matches depth map (visual); Z orientation and scale (mm); at least one image/depth pair | `MANUAL_TEST_REPORT.md` § Case 1 | None. |
| **QA-604** | Performance test: FPS at ~100K, ~500K, ~1M vertices; target ≥30 FPS for 100K; record hardware | `MANUAL_TEST_REPORT.md` § Case 4 | None. |

**Process:** Execute the six manual test cases in `TEST_PLAN_1_7.md` (Cases 1–6), fill every section of `MANUAL_TEST_REPORT.md`, then run through `VERIFICATION_CHECKLIST.md`.

**Regression / smoke (must run and record):**

- [ ] App starts (`npm run tauri dev`)
- [ ] Load image → generate depth → adjust depth (Sprint 1.4/1.5) still works
- [ ] `get_mesh_data` path works from backend
- [ ] No console errors on main screen before opening 3D preview
- [ ] No console errors when opening 3D preview and loading mesh

**Verification checklist gates (run and tick):**

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` — PASS
- [ ] `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` — 0 warnings
- [ ] `npm run build` — PASS
- [ ] `npm test` — PASS

---

### 2.2 Partially Blocked on Junior 3D

| Item | Dependency | What QA can do now |
|------|------------|---------------------|
| **Case 3: Mesh statistics (JR1-503)** | JR1-503 (vertex count, bounds in UI) not implemented | Mark Case 3 as N/A or “Blocked – JR1-503” in report; or verify stats via backend/DevTools if mesh data is available elsewhere. |
| **Case 2: Camera presets** | JR1-501 (top/front/side) not implemented | Run orbit (rotate, zoom, pan) only; note “Presets not yet implemented” in Actual result. |

Sprint success criteria require “Mesh statistics displayed accurately” and “Camera presets … implemented.” So for **full** sign-off, JR1-501 and JR1-503 should be completed first, then QA re-runs Case 2 and Case 3.

---

### 2.3 Optional — Automated Tests (TEST_PLAN_1_7 §2.2)

| Test | Location | Purpose |
|------|----------|---------|
| **Preview3D component mount** | `src/components/__tests__/Preview3D.test.ts` | Smoke: component renders, no crash when mesh empty; mock `getMeshData` (e.g. return null or minimal data). |
| **Mesh data → BufferGeometry** | Same file or a small util test | Parse `get_mesh_data`-style response (positions, normals) into flat arrays for BufferGeometry; only needed if we extract that logic from `Preview3D.svelte`. |

**Note:** `getMeshData` and invoke contract are already covered in `tauri.test.ts`; no duplicate tests needed there.

---

## 3. Recommended Order of Work

1. **Run CI quality gates**  
   From repo root:  
   `cargo test --manifest-path src-tauri/Cargo.toml`  
   `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`  
   `npm run build`  
   `npm test`  
   Update `VERIFICATION_CHECKLIST.md` with results.

2. **Execute manual test cases**  
   Follow `TEST_PLAN_1_7.md` §3 (Cases 1–6).  
   - Cases 1, 2 (orbit only), 4, 5, 6: run as-is.  
   - Case 3: mark N/A/Blocked until JR1-503, or document manual verification of stats if possible.

3. **Fill `MANUAL_TEST_REPORT.md`**  
   For each case: Objective, Steps, Expected result, Actual result, Pass/Fail.  
   For Case 4: fill FPS table (100K, 500K, 1M) and hardware.  
   For Case 5: fill GPU comparison table.  
   Complete Regression/smoke and Summary/sign-off.

4. **Complete `VERIFICATION_CHECKLIST.md`**  
   Tick all quality gates and success criteria that are met; leave unchecked any item blocked on JR1 (e.g. “Mesh statistics displayed accurately” if JR1-503 not done).

5. **(Optional)** Add `Preview3D.test.ts`  
   Mount Preview3D, mock `getMeshData`, assert no crash and optional “Load mesh” or empty state.

6. **GOTCHAS**  
   Record any 3D preview / GPU / performance gotchas in `SPRINTS/Sprint_1_7/GOTCHAS.md` and plan merge to `RESEARCH/GOTCHAS.md`.

---

## 4. Summary Table — “What Testing Is Needed”

| Category | What’s needed | Owner |
|----------|----------------|-------|
| **Manual QA** | Execute QA-601–604 and all 6 test cases; fill MANUAL_TEST_REPORT | Quality Engineer |
| **Regression/smoke** | Run 5 smoke checks; record in MANUAL_TEST_REPORT | Quality Engineer |
| **Verification checklist** | Run cargo test, clippy, npm build, npm test; tick checklist | Quality Engineer |
| **JR1-501, JR1-503** | Implement presets and mesh stats so Case 2 & 3 can be fully signed off | Junior Engineer 3D |
| **Optional automation** | Preview3D mount test (and optional parsing test) | Quality Engineer or UI |

Once manual tests are executed, the report filled, and the verification checklist updated, Sprint 1.7 testing is **up to date** for the current scope; full sprint sign-off also requires JR1-501 and JR1-503 (and their verification) if we insist on all success criteria.

---

**Document version:** 1.0  
**References:** `SPRINT_1_7_Task_Assignment.md`, `TEST_PLAN_1_7.md`, `VERIFICATION_CHECKLIST.md`, `MANUAL_TEST_REPORT.md`, `todo.md` § Testing Strategy, § Sprint 1.7
