# Sprint 1.5A Test Plan

**Sprint:** 1.5A — Hardening, Testing & Consultant Remediation
**Focus:** Frontend test infrastructure, contrast slider, coverage tracking
**Last Updated:** 2026-02-06

---

## 1. Automated Tests (New This Sprint)

### 1.1 Frontend (Vitest) — NEW

| Test File | Tests | Covers | Task |
|-----------|-------|--------|------|
| `src/lib/__tests__/depthCanvas.test.ts` | ≥5 | renderDepthToCanvas: happy path, NaN, clamping, mismatch, empty | JR2-501 |
| `src/lib/__tests__/tauri.test.ts` | ≥7 | IPC wrappers: correct command names, return types, error propagation | JR2-502 |
| `src/components/__tests__/DepthControls.test.ts` | ≥6 | Disabled state, enabled state, slider change, checkbox, reset, clamping | JR1-501 |
| `src/components/__tests__/ImageImport.test.ts` | ≥4 | Default render, loading, error, load button | JR1-502 |

**Total target:** ≥22 frontend tests (sprint success criterion: ≥15)

### 1.2 Existing Suites (Regression)

| Suite | Command | Expected |
|-------|---------|----------|
| Rust | `cargo test --manifest-path src-tauri/Cargo.toml` | 44+ passed, 5 ignored |
| Rust clippy | `cargo clippy -- -D warnings` | 0 warnings |
| Python | `SP3D_USE_STUB=1 pytest python/ -v` | 19 passed |
| Frontend build | `npm run build` | PASS |

---

## 2. Coverage Tracking (New This Sprint)

| Tool | CI Step | Output | Task |
|------|---------|--------|------|
| cargo tarpaulin | Backend job | XML report, baseline % | BACK-501 |
| pytest-cov | Python job | Terminal + XML, baseline % | AI-501 |

---

## 3. Manual Tests

### 3.1 Contrast Slider (UI-501)

| # | Steps | Expected | Pass |
|---|-------|----------|------|
| 1 | Load image, generate depth, locate contrast slider | Slider visible between Brightness and Gamma, value = 1.0 | [ ] |
| 2 | Drag contrast to 2.0 | Depth preview shows expanded midtones (darks darker, lights lighter) | [ ] |
| 3 | Drag contrast to 0.5 | Depth preview shows compressed midtones (flatter image) | [ ] |
| 4 | Type "1.5" in numeric input | Slider moves to 1.5, preview updates | [ ] |
| 5 | Click Reset | Contrast returns to 1.0, preview matches original | [ ] |
| 6 | Use arrow keys on focused slider | Value changes by step (0.05) | [ ] |

### 3.2 Security Regression

| # | Steps | Expected | Pass |
|---|-------|----------|------|
| 1 | Load image via file picker after asset protocol scope change | Image loads, preview displays | [ ] |
| 2 | Generate depth map | Depth preview appears, adjustments work | [ ] |
| 3 | All existing controls (brightness, gamma, invert, reset) | No regression from SEC-501 changes | [ ] |

### 3.3 Regression (Sprint 1.5 Flow)

| # | Steps | Expected | Pass |
|---|-------|----------|------|
| 1 | Load image → Generate depth → Adjust sliders → Reset | All work as before | [ ] |
| 2 | Load invalid file → Error message shown | Error displayed, no crash | [ ] |

---

## 4. Acceptance Summary

| Criterion | Met? |
|-----------|------|
| `npm test` passes with ≥15 tests | [ ] |
| Contrast slider functional | [ ] |
| Tarpaulin runs in CI | [ ] |
| pytest-cov runs in CI | [ ] |
| Asset protocol scope restricted | [ ] |
| IPC spike documented | [ ] |
| Model license ADR written | [ ] |
| Sprint 1.5 artefacts updated | [ ] |
| todo.md updated | [ ] |
