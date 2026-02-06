# External Consultant Recommendations Report

**Project:** SimplePicture3D
**Date:** February 6, 2026 (Updated: February 6, 2026 — Second Review)
**Consultant:** Independent Technical Review
**Documents Reviewed:** prd.md, todo.md, RESEARCH/architecture.md, CLAUDE.md, CI/CD pipeline, full codebase (src/, src-tauri/, python/, docs/), sprint artefacts (Sprints 1.1–1.5), git history (15 commits)

---

## Executive Summary

SimplePicture3D has made **substantial progress** since the initial review. Sprint 1.5 (Manual Depth Adjustments) is now functionally complete — the project has advanced from planning to a working Tauri application with image loading, AI depth estimation, a depth adjustment pipeline, and a real-time grayscale preview. Several Priority 1 recommendations from the initial report have been addressed.

**Current Progress:** Sprint 1.5 Complete — approximately 50-55% through Phase 1 MVP (up from 35-40%).

**Overall Assessment:** The team has responded well to the initial review. CI/CD and Python testing gaps have been addressed. However, frontend testing remains absent, a contrast control is missing from the UI despite backend support, and the export pipeline (STL/OBJ) and 3D preview are still stubs. New concerns around IPC performance and a security configuration issue have been identified.

| Category | Previous Rating | Current Rating | Change |
|----------|----------------|----------------|--------|
| Documentation Quality | **Excellent** | **Excellent** | Maintained; user-guide and security-checklist added |
| Architecture Design | **Good** | **Good** | Depth adjustment pipeline well-designed; IPC concern noted |
| Sprint Planning | **Good** | **Good** | Sprint 1.5 completed; progress report document is stale |
| Testing Infrastructure | **Needs Improvement** | **Improved** | Clippy in CI, pytest suite added; frontend still untested |
| Risk Management | **Adequate** | **Adequate** | GOTCHAS actively maintained; model license still unresolved |
| CI/CD Pipeline | **Partial** | **Good** | Three-job pipeline (frontend, backend, python); coverage still missing |

---

## 1. Recommendations Status — Initial Report

### 1.1 Priority 1: Address Testing Debt — PARTIALLY COMPLETE

| Action | Status | Evidence |
|--------|--------|----------|
| Add `cargo clippy -- -D warnings` to CI | **DONE** | `.github/workflows/ci.yml` line 59, comment references "Consultant Priority 1" |
| Integrate `cargo tarpaulin` for coverage | **NOT DONE** | No coverage tool in CI or Cargo.toml dev-deps |
| Create pytest suite for `depth_estimator.py` | **DONE** | `python/tests/test_depth_estimator.py` — 19 tests (unit + CLI) |
| Add `npm test` with Vitest smoke tests | **NOT DONE** | No Vitest/Jest in package.json; no test script |
| Enable Python tests in CI (with stub mode) | **DONE** | CI python job runs `pytest python/ -v` with `SP3D_USE_STUB=1` |

**Assessment:** 3 of 5 actions completed. The two remaining items (tarpaulin coverage, frontend test suite) should remain high priority.

### 1.2 Priority 2: Resolve Model License Issue — NOT ADDRESSED

The CC-BY-NC-4.0 license on Depth-Anything-V2 weights remains unresolved. `docs/tech-stack-approval.md` acknowledges the issue ("License note: Depth-Anything-V2 weights are CC-BY-NC-4.0 (non-commercial). Document in app.") but no decision has been made on a commercial-friendly alternative.

**Recommendation stands:** Offer MiDaS as a commercial-friendly alternative for MVP.

### 1.3 Priority 3: Create ADRs — PARTIALLY ADDRESSED

Formal `ADR-00X` files were not created, but `docs/tech-stack-approval.md` functions as an informal combined ADR covering:
- Svelte over React (approved)
- Subprocess over PyO3 (approved, with rationale)
- ONNX deferred to post-MVP

**Remaining gap:** No ADR for Python distribution strategy (still the highest-risk undecided architecture question).

### 1.4 Priority 4: Revise Timeline Estimates — NOT ADDRESSED

No visible timeline revision in todo.md. Original 19–25 sprint estimate unchanged.

### 1.5 Priority 5: Python Distribution Strategy — NOT ADDRESSED

Still assumes system Python. No documentation of bundling approach.

---

## 2. Updated Strengths

### 2.1 Sprint 1.5 Implementation Quality

The depth adjustment pipeline is well-implemented:

- **Backend (`depth_adjust.rs`):** 366 lines, clean separation of concerns. Pipeline order (invert → gamma → contrast → brightness) is documented and tested. 19 unit tests cover identity, boundary, and extreme values. Benchmark confirms 14ms for 1080p — well under the 100ms target.

- **Frontend (`DepthControls.svelte`):** Thoughtful UX with paired slider + numeric input, keyboard arrow key support (accessibility), debounced backend updates (80ms), and proper disabled state when no depth map exists.

- **IPC contract (`tauri.ts`):** Types mirror Rust structs exactly. Clean async wrappers for all 7 Tauri commands.

### 2.2 CI/CD Now Three-Job Pipeline

The CI pipeline has improved materially:

```
Frontend:  npm ci → npm run build → npm audit
Backend:   cargo build → cargo test → cargo clippy -D warnings → cargo audit
Python:    python 3.10 → pip install pytest Pillow → pytest (stub mode)
```

All three stacks are now tested on every push/PR. The clippy step even references the consultant report.

### 2.3 Python Test Suite Created From Scratch

The `python/tests/test_depth_estimator.py` file is thorough:
- Unit tests for `clamp_depth_to_01`, `run_inference_stub`, `load_image_dimensions`
- CLI integration tests (subprocess invocation with fixture images)
- Edge cases (NaN, 1x1 images, missing files, invalid images, env var stub mode)
- Proper conftest.py for path management

### 2.4 Security Documentation Mature

`docs/security-checklist.md` is production-grade:
- Dependency addition checklist (license, audit, scope)
- Release security review checklist
- SEC-201 (Python subprocess) review completed and documented with evidence table
- Reference commands for all three stacks

### 2.5 User Documentation

`docs/user-guide.md` added with depth controls documentation, matching the implemented UI. Good practice for a pre-release project.

---

## 3. New Gaps and Concerns

### 3.1 Critical: Frontend Testing Remains Absent

This is now the **single largest testing gap** in the project. The frontend has grown substantially:

| File | Lines | Complexity |
|------|-------|-----------|
| `App.svelte` | 275 | State management, 7 IPC calls, debounce logic |
| `DepthControls.svelte` | 279 | 6 input handlers, keyboard support, range clamping |
| `DepthMapPreview.svelte` | 171 | Canvas rendering, zoom/pan state machine |
| `ImageImport.svelte` | 134 | File picker, drag-and-drop, error handling |
| `depthCanvas.ts` | 49 | NaN handling, pixel manipulation |
| `tauri.ts` | 85 | Type definitions, IPC wrappers |

**Total: ~993 lines of untested frontend code.**

The `depthCanvas.ts` and `tauri.ts` modules are pure functions that could be tested today with zero mocking infrastructure. The Svelte components could be tested with `@testing-library/svelte` and Vitest.

**Recommendation:** This should be the top priority for Sprint 1.6.

### 3.2 High: Contrast Slider Missing From UI

The backend `DepthAdjustmentParams` includes a `contrast` field (default 1.0), and `apply_adjustments` applies contrast in the pipeline. The `tauri.ts` TypeScript type includes `contrast`. However, the `DepthControls.svelte` component **has no contrast slider** — it only exposes brightness, gamma, invert, and depth range.

This means:
- Users cannot adjust contrast through the UI
- The `contrast` field is always sent as its default (1.0)
- The backend code for contrast is tested but unreachable from the frontend

**Recommendation:** Add a contrast slider to `DepthControls.svelte` in the next sprint. This is a small effort (copy the brightness slider pattern) with high user value.

### 3.3 High: IPC Performance for Large Depth Maps

The depth map is transferred as a JSON array of f32 over Tauri invoke. For a 4K image (3840×2160 = 8.3M pixels), this serializes to ~50MB of JSON text.

GOTCHAS.md already notes: "Sending large depth maps or mesh data (e.g. 3MB+) over Tauri IPC can be slow (e.g. 200ms+ for 3MB)."

Current debounce is 80ms, but if IPC takes 200ms+ for even a 1080p depth map (~8MB JSON), the user will experience lag. This will worsen significantly with 4K images.

**Recommendation:** Before Sprint 1.7 (mesh generation), evaluate:
1. Binary transfer via shared memory or temp file (Rust writes, frontend reads via asset protocol)
2. Chunked WebSocket transfer
3. Only transferring the depth map once, then applying adjustments client-side

### 3.4 Medium: Security — Asset Protocol Scope Too Broad

In `tauri.conf.json`:

```json
"assetProtocol": {
  "enable": true,
  "scope": ["**"]
}
```

The `"**"` scope grants the frontend access to **any file on the filesystem** via the asset protocol. This contradicts the security-first approach documented in the threat model.

**Recommendation:** Restrict the scope to specific directories needed by the application:

```json
"scope": [
  "$APPDATA/simplepicture3d/**",
  "$TEMP/**"
]
```

### 3.5 Medium: Sprint 1.5 Progress Report Stale

`SPRINTS/Sprint_1_5/PROGRESS_REPORT.md` still shows all phases as "Not Started" despite the sprint being complete (verified by git commit `70bbe45`). The VERIFICATION_CHECKLIST.md also has all boxes unchecked.

This is a process gap — sprint artefacts should be updated when work is completed to maintain the documentation-first culture that is this project's strength.

### 3.6 Medium: Export and 3D Preview Still Stubs

| Feature | Status | Code |
|---------|--------|------|
| `export_stl` | Stub — validates path non-empty, returns `Ok(())` | `lib.rs:41-46` |
| `Preview3D` | Placeholder — gray box with text | `Preview3D.svelte` |
| Mesh generation | Not started | No mesh/triangle code exists |

These are core features for the MVP and represent the largest remaining implementation effort. Sprint 1.6+ task assignments should account for the fact that mesh generation is algorithmically complex and will likely require multiple sprints.

### 3.7 Low: Rust Test Count Stale in todo.md

The Testing Strategy section of `todo.md` (line 1467) states "27 unit/integration tests" and "No `cargo clippy` in CI". Both are outdated — the actual test count is significantly higher (27 original + ~19 depth_adjust + boundary tests), and clippy is now in CI. This section should be updated to reflect current reality.

---

## 4. Updated Recommendations

### Priority 1: Frontend Test Suite (Sprint 1.6)

| Action | Owner | Effort |
|--------|-------|--------|
| Add Vitest as dev dependency + `npm test` script | UI Designer | 2 hours |
| Unit tests for `depthCanvas.ts` (renderDepthToCanvas) | UI Designer | 2 hours |
| Unit tests for `tauri.ts` type validation (mock invoke) | UI Designer | 4 hours |
| Component tests for `DepthControls.svelte` (slider events, reset) | UI Designer | 1 day |
| Component tests for `ImageImport.svelte` (error states, drag-and-drop) | UI Designer | 1 day |

### Priority 2: Add Missing Contrast Slider (Sprint 1.6)

| Action | Owner | Effort |
|--------|-------|--------|
| Add contrast range input to `DepthControls.svelte` | UI Designer | 1 hour |
| Wire `handleContrastInput` to `emitChange` | UI Designer | 30 min |
| Update user-guide.md with contrast control | Documentation | 15 min |
| Add manual test case for contrast slider interaction | QA | 15 min |

### Priority 3: Coverage Tracking (Sprint 1.6)

| Action | Owner | Effort |
|--------|-------|--------|
| Add `cargo tarpaulin` to CI, upload to Codecov | QA Engineer | 4 hours |
| Add `pytest --cov` to Python CI job | QA Engineer | 1 hour |
| Set coverage thresholds (fail CI if <70% Rust, <70% Python) | QA Engineer | 1 hour |

### Priority 4: Fix Asset Protocol Scope (Sprint 1.6)

Restrict `tauri.conf.json` asset protocol scope from `"**"` to application-specific directories only. This is a one-line config change with significant security impact.

### Priority 5: IPC Performance Evaluation (Before Sprint 1.7)

Conduct a spike to measure actual IPC latency for depth maps at various resolutions:
- 640×480 (~1.2MB JSON)
- 1920×1080 (~8.3MB JSON)
- 3840×2160 (~33MB JSON)

If latency exceeds 100ms for 1080p, implement binary transfer via temp file + asset protocol before building the mesh preview.

### Priority 6: Resolve Model License (Before Phase 1 Exit)

Original recommendation stands. At minimum, document the non-commercial restriction prominently in the application UI (not just developer docs) and in the download wizard.

### Priority 7: Python Distribution Strategy (Before Sprint 1.10)

Original recommendation stands. Document the decision formally as an ADR.

---

## 5. Updated Risk Assessment

### Risks Resolved or Reduced

| Risk | Previous | Current | Action Taken |
|------|----------|---------|--------------|
| No Python tests in CI | **High** | **Low** | pytest suite created, CI job added |
| No clippy in CI | **High** | **Resolved** | clippy -D warnings in CI |
| Testing debt compounds | **High** | **Medium** | Partially addressed; frontend gap remains |

### Risks Unchanged

| Risk | Rating | Notes |
|------|--------|-------|
| Python bundling complexity | **High** | No progress; still assumes system Python |
| Model license commercial conflict | **Medium** | Acknowledged but not resolved |
| Timeline optimism | **Medium** | No revised estimates; core features (mesh, export, 3D preview) still stubs |

### New Risks Identified

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| IPC bottleneck blocks real-time preview at 4K | **High** | **Medium** | Spike binary transfer before mesh sprint |
| Asset protocol `**` scope creates filesystem exposure | **Medium** | **High** | Restrict scope in tauri.conf.json |
| Frontend grows beyond testability before tests exist | **Medium** | **Medium** | Add Vitest immediately; don't add more components without tests |
| Contrast feature gap creates user confusion (backend supports it, UI doesn't expose it) | **Medium** | **Low** | Add contrast slider |
| Sprint artefact staleness erodes documentation culture | **Low** | **Medium** | Update progress reports when sprints complete |

---

## 6. Quantitative Progress Summary

### Codebase Growth

| Metric | Initial Review | Current | Change |
|--------|---------------|---------|--------|
| Git commits | ~12 | 15 | +3 |
| Rust source files | 4 | 6 | +2 (depth_adjust.rs, file_io.rs) |
| Rust source lines (src-tauri/src/) | ~600 | ~1,260 | +110% |
| Rust tests (non-ignored) | 27 | ~46 | +70% |
| Python test files | 0 | 2 | +2 (test_depth_estimator.py, conftest.py) |
| Python tests | 0 | 19 | +19 |
| Svelte components | 3 | 5 | +2 (DepthControls, DepthMapPreview) |
| Frontend source lines (src/) | ~350 | ~993 | +184% |
| Frontend tests | 0 | 0 | No change |
| CI jobs | 2 | 3 | +1 (python) |
| CI steps | ~8 | ~13 | +5 (clippy, python setup, install, pytest) |
| Documentation files (docs/) | 3 | 6 | +3 |
| Sprint artefacts | ~18 | ~29 | +11 |

### Test Coverage Estimate (Approximate)

| Stack | Lines | Tests | Estimated Coverage | Target |
|-------|-------|-------|--------------------|--------|
| Rust backend | ~1,260 | ~46 | ~55-65% | >70% |
| Python | ~224 | 19 | ~70-80% | >70% |
| Frontend | ~993 | 0 | **0%** | >60% |

---

## 7. Positive Observations

1. **Consultant responsiveness:** The team explicitly addressed Priority 1 recommendations — the CI clippy step even cites "Consultant Priority 1" in its comment. This demonstrates good process discipline.

2. **Benchmark-driven development:** The depth adjustment benchmark (`benches/depth_adjust.rs`) provides empirical evidence that real-time preview is feasible (14ms for 1080p). This is best practice for performance-sensitive features.

3. **Accessibility embedded in components:** `DepthControls.svelte` includes `aria-label`, `aria-valuemin/max/now`, `role="group"`, and keyboard arrow key support. This is ahead of where most projects are at this stage.

4. **Security review documentation:** The SEC-201 review in `docs/security-checklist.md` is exemplary — it documents what was checked, the evidence, and the conclusion. This pattern should be replicated for future security-sensitive features.

5. **GOTCHAS.md actively used:** Six entries covering real debugging findings. The benchmark result documented there is exactly the kind of institutional knowledge that prevents future regressions.

---

## 8. Summary: Top 5 Actions for Next Sprint

| # | Action | Effort | Impact |
|---|--------|--------|--------|
| 1 | Add Vitest + frontend tests for `depthCanvas.ts` and `DepthControls.svelte` | 2-3 days | Closes the largest testing gap |
| 2 | Add contrast slider to `DepthControls.svelte` | 2 hours | Completes depth adjustment feature set |
| 3 | Restrict asset protocol scope in `tauri.conf.json` | 15 min | Closes security hole |
| 4 | Add tarpaulin + pytest-cov to CI | Half day | Enables coverage tracking toward exit criteria |
| 5 | Update Sprint 1.5 progress report and verification checklist | 30 min | Maintains documentation integrity |

---

**Report Prepared By:** External Technical Consultant
**Review Status:** Second review — updated with codebase analysis
**Previous Review:** February 6, 2026 (initial)
**Next Review:** Recommended at Sprint 1.7 (pre-mesh generation) or Phase 1 Exit Gate

---

*This report is based on full codebase and documentation review as of February 6, 2026. The assessment reflects actual code inspection, not just documentation review. Recommendations are advisory and should be evaluated against project constraints and priorities.*
