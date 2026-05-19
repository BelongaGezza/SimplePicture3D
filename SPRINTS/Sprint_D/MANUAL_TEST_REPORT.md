# Manual Test Report — Sprint D

**Sprint:** D — Validate and Harden  
**Tester:** _TBD_  
**Build:** _commit / version_  
**OS:** _Windows | macOS | Linux_  
**Date:** _TBD_

**Related procedures:** [QA-D-01_TEST_PROCEDURE.md](./QA-D-01_TEST_PROCEDURE.md) · [docs/engraver-charter.md](../../docs/engraver-charter.md)

---

## QA-D-01 — Engraver import

**Task status:** Procedure **ready** · Engraver execution **blocked** (target software TBD)  
**Charter:** [docs/engraver-charter.md](../../docs/engraver-charter.md) (draft, 2026-05-19)

### Environment (fill on execution)

| Field | Value |
|-------|--------|
| Git commit (`git rev-parse --short HEAD`) | |
| Build | `tauri dev` ☐ / release build ☐ |
| OS + version | |
| Engraver software + version | **TBD** |
| Stub depth (`SP3D_USE_STUB=1`) | Yes ☐ / No ☐ |
| Test image (path + size) | e.g. `tests/fixtures/valid_small.png` |
| Blank used | 80×50×50 mm, margin 2 mm ☐ (other: _____) |

### Prerequisites (summary)

- Toolchain per `SETUP_NOTES.md`; `npm run tauri dev` runs.
- Fixtures: `node scripts/gen_fixtures.mjs` if PNG fixtures missing.
- Optional stub depth: `export SP3D_USE_STUB=1` before `npm run tauri dev` (no Hugging Face model).
- Default blank: **80 × 50 × 50 mm**, margin **2 mm** (Standard preset).

**Full steps:** [QA-D-01_TEST_PROCEDURE.md](./QA-D-01_TEST_PROCEDURE.md) § B–C (app), § D (engraver).

### Execution summary

| Phase | Scope | Result | Notes |
|-------|--------|--------|-------|
| App export (C1–C8) | Load → depth → blank → PLY ASCII + XYZ | Not run ☐ / Pass ☐ / Fail ☐ | |
| Engraver import (D1–D7) | PLY + XYZ into target engraver | **Blocked** ☐ / Pass ☐ / Fail ☐ | Engraver: TBD |

### Format results

| Format | Engraver import | Pre-check (CloudCompare/MeshLab) | Point count N | Notes |
|--------|-----------------|----------------------------------|---------------|-------|
| PLY ASCII | Pass / Fail / Blocked | Pass / Fail / N/A | | |
| XYZ | Pass / Fail / Blocked | Pass / Fail / N/A | | |

**PLY/XYZ counts match:** ☐ Yes ☐ No — PLY N= _____ XYZ lines= _____

### Charter verification (docs/engraver-charter.md §5)

| ID | Check | Pass? | Notes |
|----|--------|-------|-------|
| V1 | File opens | ☐ | |
| V2 | Units mm | ☐ | |
| V3 | Axis orientation | ☐ | |
| V4 | Point count reasonable | ☐ | |
| V5 | BBox inside blank | ☐ | |
| V6 | PLY metadata comments | ☐ | |
| V7 | Depth / engraving preview | ☐ | |

**Axis orientation correct:** ☐ Yes ☐ No ☐ N/A (blocked)  
**Units (mm) correct:** ☐ Yes ☐ No ☐ N/A (blocked)

### QA-D-01 overall rubric

| Outcome | Select when |
|---------|-------------|
| ☐ **Pass** | Engraver import Pass; V1–V7 Pass |
| ☐ **Fail** | Wrong units/axes, import error, or bbox violation |
| ☐ **Blocked** | Engraver software not available (sprint gate open) |
| ☐ **Partial** | App export Pass; engraver steps not run |

### Defect log (BACK-D-01 handoff)

| ID | Sev | Format | Symptom | Expected | Actual | Evidence | Status |
|----|-----|--------|---------|----------|--------|----------|--------|
| | P0/P1/P2/P3 | PLY/XYZ/Both | | Charter §2–4 | | path/screenshot | Open |

_Add rows from [QA-D-01_TEST_PROCEDURE.md](./QA-D-01_TEST_PROCEDURE.md) § G during execution._

---

## QA-D-02 — Performance (4K image)

| Step | Time | Notes |
|------|------|-------|
| Load image | | |
| Generate depth | | |
| Generate point cloud | | |
| Export PLY | | |
| **Total** | | Target: &lt; 2 min (PRD) |

---

## QA-D-03 — Security pentest checklist

See `SECURITY_PENTEST_CHECKLIST.md` — copy Pass/Fail column here or attach summary:

| Section | Pass | Fail | Skipped |
|---------|------|------|---------|
| IPC fuzzing | | | |
| Filesystem | | | |
| Model & subprocess | | | |
| WebView / ACL | | | |
| DoS (lab) | | | |

---

## Overall

**Sprint D QA sign-off:** ☐ Pass ☐ Fail — **Date:** ___________

**QA-D-01 blocker:** Target engraver software name/version not assigned; complete § D in [QA-D-01_TEST_PROCEDURE.md](./QA-D-01_TEST_PROCEDURE.md) when available.
