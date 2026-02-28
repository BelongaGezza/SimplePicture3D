# Sprint 2.1 — Manual Test Report: Advanced Depth Controls (Histogram & Curves)

**Sprint:** 2.1  
**Last Updated:** 2026-02-28  
**Assigned QA:** Quality Engineer (Cursor-Auto-QE-20260228)

---

## Summary

| Task | Procedure | Result | Notes |
|------|------------|--------|-------|
| QA-1101 | TEST_PLAN_2_1.md § QA-1101 | ☐ Pass / ☐ Fail / ☐ Deferred | Curve change → preview → export |
| QA-1102 | TEST_PLAN_2_1.md § QA-1102 | ☐ Pass / ☐ Fail / ☐ Deferred | Linear, S-curve, Exponential presets |
| QA-1103 | TEST_PLAN_2_1.md § QA-1103 | ☐ Pass / ☐ Fail / ☐ Deferred | Performance <200 ms (1080p/4K) |

---

## QA-1101: Adjust curves, verify depth changes

**Procedure:** See TEST_PLAN_2_1.md — "QA-1101: Adjust curves, verify depth changes".

**Steps (summary):**
1. Load an image and generate depth map.
2. Open **Advanced (histogram & curve)** in depth controls.
3. Confirm histogram shows depth distribution; change a curve control point (drag on curve).
4. Confirm depth preview updates within ~200 ms (debounced).
5. Export STL/OBJ and optionally open in MeshLab; confirm mesh reflects curved depth.

**Result:** ☐ Pass  ☐ Fail  ☐ Deferred  
**Date executed:** _______________  
**Tester:** _______________  
**Environment:** OS _______________, Node _______________, Rust _______________

**Actual / Notes:**
- 

---

## QA-1102: Preset curves (Linear, S-curve, Exponential)

**Procedure:** See TEST_PLAN_2_1.md — "QA-1102: Preset curves".

**Steps (summary):**
1. With depth map and Advanced mode on, select **Linear** — depth should match "no curve".
2. Select **S-curve** — midtones more separated; shadows/highlights compressed.
3. Select **Exponential** — shadows lifted relative to linear.
4. Switch back to Linear and confirm preview matches initial state.

**Result:** ☐ Pass  ☐ Fail  ☐ Deferred  
**Date executed:** _______________  
**Tester:** _______________  
**Regression in basic depth controls:** ☐ None  ☐ Yes (describe): _______________

**Actual / Notes:**
- 

---

## QA-1103: Performance — curve application on large depth maps

**Procedure:** See TEST_PLAN_2_1.md — "QA-1103: Performance".

**Steps (summary):**
1. Use 1080p or 4K source; generate depth.
2. Enable Advanced mode; change curve (preset or drag).
3. Measure time from release of drag / preset select to next preview frame (target <200 ms for 1080p).
4. Document result below. If target missed, note as known limitation.

**Result:** ☐ Pass  ☐ Fail  ☐ Deferred  
**Date executed:** _______________  
**Tester:** _______________  
**Hardware:** _______________

**Measured times:**

| Resolution | Action | Time (ms) | Target | Met? |
|------------|--------|-----------|--------|------|
| 1080p      | Preset change | _____ | <200 | ☐ |
| 1080p      | Drag control point | _____ | <200 | ☐ |
| 4K (if run) | Preset change | _____ | <200 (or note) | ☐ |

**Notes / Known limitations:**
- 

---

## Verification commands (automated)

Run from project root. Results used for VERIFICATION_CHECKLIST.md.

| Command | Result | Date |
|---------|--------|------|
| `cargo test --manifest-path src-tauri/Cargo.toml` | PASS (147 passed, 6 ignored) | 2026-02-28 |
| `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` | PASS (0 warnings) | 2026-02-28 |
| `npm run build` | PASS (Vite build succeeded) | 2026-02-28 |
| `python -m pytest python/ -v` (stub mode, Windows) | PASS (32 passed) | 2026-02-28 |

Manual test cases (QA-1101–1103) are documented above; execution requires human tester. Procedure and exit criteria are in TEST_PLAN_2_1.md.
