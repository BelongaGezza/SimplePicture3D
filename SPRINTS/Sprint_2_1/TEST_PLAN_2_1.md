# Sprint 2.1 — Test Plan: Advanced Depth Controls (Histogram & Curves)

**Sprint:** 2.1  
**Last Updated:** 2026-02-28

---

## Scope

- BACK-1101–1104: Histogram calculation, curve data structure, curve application, real-time preview
- UI-1101–1105: HistogramPanel, CurvesTool, presets, advanced mode toggle
- JR1-1101–1103: Canvas histogram, Bezier/curve drawing, reset curve
- QA-1101–1103: Manual and performance testing

---

## Automated Tests (Done)

| Area | Status | Notes |
|------|--------|-------|
| Rust `depth_adjust` | ✅ | `curve_linear_is_identity`, `curve_piecewise_linear`, `curve_empty_or_single_returns_clamped`, `pipeline_with_curve_linear_unchanged`, `histogram_bins`, `histogram_256_bins` |
| Rust `get_depth_histogram` | ✅ | Command registered; integration via full app (manual or E2E) |
| Frontend build | ✅ | `npm run build` passes |

---

## Manual Test Procedures

### QA-1101: Adjust curves, verify depth changes

1. Load an image and generate depth map.
2. Open **Advanced (histogram & curve)** in depth controls.
3. Confirm histogram shows depth distribution; change a curve control point (drag on curve).
4. Confirm depth preview updates within ~200 ms (debounced).
5. Export STL/OBJ and optionally open in MeshLab; confirm mesh reflects curved depth.

### QA-1102: Preset curves (Linear, S-curve, Exponential)

1. With depth map and Advanced mode on, select **Linear** from Curve preset — depth should match “no curve”.
2. Select **S-curve** — midtones should appear more separated; shadows/highlights compressed.
3. Select **Exponential** — shadows lifted relative to linear.
4. Switch back to Linear and confirm preview matches initial state.

### QA-1103: Performance — curve application on large depth maps

1. Use a 1080p or 4K source; generate depth.
2. Enable Advanced mode; change curve (preset or drag).
3. Measure time from release of drag / preset select to next preview frame (target <200 ms for 1080p).
4. Document result in MANUAL_TEST_REPORT.md or PROGRESS_REPORT.md. If target missed, note as known limitation.

---

## Exit Criteria Checklist

- [ ] Histogram displays depth distribution when depth map exists and Advanced is on
- [ ] Curves tool functional with draggable points; preset dropdown applies Linear, S-curve, Exponential
- [ ] Reset curve restores identity (linear) mapping
- [ ] Advanced mode toggle shows/hides histogram and curve panel
- [ ] Real-time preview updates within 200 ms of curve/control change (or documented deviation)
- [ ] No regression in basic depth controls (sliders, invert, reset)

---

## Notes

- Backend curve is applied after brightness in pipeline (invert → gamma → contrast → brightness → curve).
- Histogram is computed from **adjusted** depth (same as preview).
- Settings persistence: curve control points are not yet persisted in AppSettings; optional Phase 2.
