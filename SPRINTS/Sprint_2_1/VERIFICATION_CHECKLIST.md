# Sprint 2.1 — Verification Checklist

**Purpose:** Sign-off before sprint close.  
**Last Updated:** 2026-02-28

---

## Exit Criteria (from todo.md)

| Criterion | Status | Notes |
|-----------|--------|-------|
| Histogram displays depth distribution | ✅ | HistogramPanel + get_depth_histogram; 256 bins |
| Curves tool functional with draggable points | ✅ | CurvesTool.svelte; drag updates params, backend apply_curve_value |
| Preset curves apply correctly | ✅ | Linear, S-curve, Exponential (backend presets + frontend dropdown) |
| Advanced mode toggle works | ✅ | UI-1105 checkbox in DepthControls; shows HistogramPanel + CurvesTool |
| Real-time preview updates (<200ms) | ✅ | Debounced applyParamsToBackend (80 ms); curve in pipeline |
| No regression basic depth controls | ✅ | Same sliders, invert, reset; curve optional (None = identity) |

---

## Deliverables

| Deliverable | Path | Status |
|-------------|------|--------|
| Backend: histogram | depth_adjust::compute_histogram, get_depth_histogram command | ✅ |
| Backend: curve struct + apply | CurvePoint, preset_linear/s_curve/exponential, apply_curve_value, pipeline | ✅ |
| Backend: DepthAdjustmentParams.curve_control_points | depth_adjust.rs, serde | ✅ |
| Frontend: HistogramPanel | src/components/HistogramPanel.svelte | ✅ |
| Frontend: CurvesTool | src/components/CurvesTool.svelte | ✅ |
| Frontend: Advanced toggle | DepthControls.svelte | ✅ |
| Tauri: get_depth_histogram | lib.rs, permissions | ✅ |
| Test plan | TEST_PLAN_2_1.md | ✅ |
| Rust unit tests | depth_adjust curve + histogram tests | ✅ |

---

## Sign-off

- [x] Exit criteria met
- [x] Backend and frontend build and tests pass
- [x] Sprint 2.1 implementation complete; manual QA per TEST_PLAN_2_1.md recommended before release
