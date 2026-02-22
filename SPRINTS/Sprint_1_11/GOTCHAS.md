# Sprint 1.11 — Gotchas

**Purpose:** Sprint-specific debugging and security findings. Merge notable entries to `RESEARCH/GOTCHAS.md` when relevant.

---

## Entries

### 2026-02-22 — Security / pip-audit — Not in default Python env
**Symptom:** `pip-audit` (and `python -m pip_audit`) not available on Windows dev environment.  
**Cause:** pip-audit is not installed by default; project docs (CLAUDE.md) list it in testing/security commands.  
**Fix:** Install with `pip install pip-audit`; add to CONTRIBUTING or CI so Python dependency audit runs (e.g. in same CI step as cargo audit / npm audit). Documented in SECURITY_SIGNOFF.md §2.1 and §4. **Implemented:** CI now runs `pip-audit -r python/requirements.txt` in the Python job (.github/workflows/ci.yml); CONTRIBUTING and docs/security-checklist.md updated; see §5 Phase 2 / Ongoing.

---

### 2026-02-22 — Mesh / JR2-1001 — Mesh XY extent vs target dimensions
**Symptom:** Unit test expected “at least one extent to match target”; mesh extent was slightly smaller (e.g. 49.5 mm for 100 px at 0.5 mm/px).  
**Cause:** Vertices are at col = 0..(num_cols−1), so extent = (num_cols−1)*pixel_to_mm, not width_px*pixel_to_mm. Scaled “logical” size (width_px*pixel_to_mm) fits target; mesh bounds are one pixel step smaller.  
**Fix:** Assert mesh extent ≤ target and that scaled size (width_px*pixel_to_mm) fits target; do not require extent to exactly match target. For aspect-ratio test, allow small tolerance (e.g. 0.02) because (width_px−1)/(height_px−1) ≠ width_px/height_px for small grids.

---

### 2026-02-22 — UI-1001 — Target size and 3D preview
**Symptom:** Need to know whether preview (get_mesh_data) should receive target dimensions from frontend.  
**Cause:** Backend get_mesh_data and export commands accept optional target_width_mm/target_height_mm and fall back to persisted settings when absent.  
**Fix:** No need to pass target size from Preview3D: persist in settings from Export Settings panel; backend uses settings for both get_mesh_data and export. Export panel can optionally pass explicit options on export for clarity; backend still falls back to settings when params omitted.

---

*Add new entries above; keep format: Date — Area — Brief title, then Symptom/Cause/Fix.*
