# QA-D-01 — Engraver import test procedure

**Task:** QA-D-01 (Sprint D)  
**Goal:** Export PLY (ASCII) and XYZ from SimplePicture3D and import **PLY into XTool Studio** (xTool F2 Ultra UV, Point Cloud Engraving mode); validate against [engraver charter](../../docs/engraver-charter.md).  
**Results:** Record in [MANUAL_TEST_REPORT.md](./MANUAL_TEST_REPORT.md) (summary + defect log).

**Execution status:** Steps 1–8 and pre-engraver checks can run without the engraver. **Step 9 (engraver import)** uses **XTool Studio** on **xTool F2 Ultra UV** — record Studio version at test time.

---

## A. Test environment (record in manual report)

| Field | Value |
|-------|--------|
| **Tester** | |
| **Date** | |
| **Git commit** | `git rev-parse --short HEAD` from repo root |
| **Build type** | `npm run tauri dev` ☐ / `npm run tauri build` (release) ☐ |
| **OS** | Windows ☐ / macOS ☐ / Linux ☐ — version: |
| **CPU / RAM** (optional) | |
| **GPU** (optional, depth only) | |
| **Python** | `python --version`; venv active ☐ |
| **Stub depth** | `SP3D_USE_STUB=1` ☐ (no model download) / real model ☐ |
| **Target engraver** | Software: **XTool Studio** — Hardware: **xTool F2 Ultra UV** — Studio version: |
| **Secondary viewer** (optional) | CloudCompare / MeshLab version: |

---

## B. Prerequisites

### B.1 Repository and build

1. Clone repo; checkout commit under test.
2. Install toolchain per `SETUP_NOTES.md` (Rust, Node, Python 3.10+).
3. From repo root:
   ```bash
   npm install
   cd python && python -m venv venv && source venv/bin/activate   # Linux/macOS
   pip install -r requirements.txt
   cd ..
   ```
4. **Linux only:** GTK3/WebKit dev packages per `SETUP_NOTES.md` if `tauri dev` fails.
5. Generate test fixtures if missing:
   ```bash
   node scripts/gen_fixtures.mjs
   ```
   Confirms `tests/fixtures/valid_small.png` (and related files).
6. Start app:
   ```bash
   # Optional — skip AI model download for depth:
   export SP3D_USE_STUB=1
   npm run tauri dev
   ```

### B.2 Reference blank (default)

| Parameter | Value |
|-----------|--------|
| Length (X) | 80 mm |
| Width (Y) | 50 mm |
| Height (Z) | 50 mm |
| Margin | 2 mm |
| Interior | 76 × 46 × 46 mm |

Use Export panel preset **“50 × 50 × 80 (standard)”** unless testing another preset deliberately.

### B.3 Sampling defaults (ADR-012)

| Parameter | Default |
|-----------|---------|
| step_x / step_y | 1 |
| depth_threshold | 0.05 |
| PLY | ASCII checked |

### B.4 Test image

| Option | Path / action |
|--------|----------------|
| **Recommended** | `tests/fixtures/valid_small.png` (regenerate via `gen_fixtures.mjs` if corrupt) |
| **Better visual** | Any PNG/JPG 800×600–1920×1080 with clear subject/background |
| **Avoid** | `invalid_not_an_image.png`, `corrupt_truncated.png` |

Record actual file name and dimensions in the manual report.

---

## C. Procedure (application)

Perform steps in order. Note **time** if also capturing QA-D-02 data.

| Step | Action | Expected result | Pass? |
|------|--------|-----------------|-------|
| **C1** | **Load image** — File → Open (or drag-drop) test image | Image in 2D view; status Loaded | ☐ |
| **C2** | **Generate depth** — click **Generate Depth Map** (or equivalent) | Progress completes; grayscale depth visible; no crash. With `SP3D_USE_STUB=1`, deterministic gradient stub | ☐ |
| **C3** | **Set blank** — Export panel → preset **50 × 50 × 80 (standard)** | L=50, W=50, H=80, margin=2 | ☐ |
| **C4** | **(Optional) Preview** — 3D view → **Load point cloud** | Points inside yellow wireframe; rotate/zoom OK | ☐ |
| **C5** | **Export PLY (ASCII)** — Format PLY, **ASCII PLY** checked → **Export point cloud** → save `qa-d01-<image>.ply` | “Saved.”; file exists | ☐ |
| **C6** | **Export XYZ** — Format XYZ → **Export point cloud** → save `qa-d01-<image>.xyz` | “Saved.”; file exists | ☐ |
| **C7** | **Record point count** | PLY: `element vertex N` in header; XYZ: `wc -l file.xyz` → N lines; counts match | ☐ N = |
| **C8** | **Record bbox** (pre-engraver) | CloudCompare/MeshLab: bbox ≈ within 50×50×80 mm; units mm | ☐ min: / max: |

**Note:** Each export click runs `generate_point_cloud` again; use the **same** blank/sampling settings for PLY and XYZ. If you change depth or blank between exports, note it in the report.

---

## D. Procedure (engraver import) — XTool Studio + F2 Ultra UV

| Step | Action | Expected result | Pass? |
|------|--------|-----------------|-------|
| **D1** | Open **XTool Studio**; connect/select **F2 Ultra UV**; choose **Inner Engraving** → **Point Cloud Engraving** | 3D import mode available | ☐ |
| **D2** | **Import PLY** (`qa-d01-*.ply`) | No parse error; point cloud visible in canvas | ☐ |
| **D3** | **Import XYZ** (`qa-d01-*.xyz`) (if Studio supports; else skip and note) | Same geometry as PLY (within float tolerance), or N/A with reason | ☐ |
| **D4** | **Units** | Extents match configured blank order of magnitude (mm); subject within **70 × 70 mm** central processing area if using F2 Ultra UV defaults | ☐ |
| **D5** | **Axis orientation** | Subject left/right and front/back match expectation; document if engraver uses different axes | ☐ |
| **D6** | **BBox inside blank** | No points outside configured blank volume in engraver view | ☐ |
| **D7** | **Engraving path preview** (if tool supports) | Recognisable depth relief, not solid block | ☐ |

Document engraver import settings (scale factor, axis flip, unit override) in the manual report **Notes** column.

---

## E. Charter verification matrix

Map to [docs/engraver-charter.md](../../docs/engraver-charter.md) §5.

| ID | Check | Procedure step | Pass? | Notes |
|----|--------|----------------|-------|-------|
| V1 | File opens | C8 / D2–D3 | ☐ | |
| V2 | Units mm | D4 | ☐ | |
| V3 | Axis orientation | D5 | ☐ | |
| V4 | Point count reasonable | C7 | ☐ | |
| V5 | BBox inside blank | C8 / D6 | ☐ | |
| V6 | PLY metadata | Inspect PLY header comments | ☐ | |
| V7 | Depth appearance | D7 | ☐ | |

---

## F. Pass / Fail rubric (QA-D-01)

| Outcome | Criteria |
|---------|----------|
| **Pass** | D2–D7 all Pass; V1–V7 Pass; no P0/P1 defects open |
| **Fail** | Any of D2–D7 Fail, or wrong units/scale, or points outside blank, or PLY/XYZ mismatch |
| **Blocked** | XTool Studio or F2 Ultra UV unavailable — procedure C1–C8 may still run; sprint gate not met |
| **Partial** | Pre-engraver (C1–C8) Pass; engraver steps Blocked — document in report |

---

## G. Defect log (handoff to BACK-D-01)

Copy rows to [MANUAL_TEST_REPORT.md](./MANUAL_TEST_REPORT.md) § QA-D-01 defect log as issues are found.

| ID | Sev | Format | Symptom | Expected (charter) | Actual | Evidence | BACK-D-01 status |
|----|-----|--------|---------|-------------------|--------|----------|------------------|
| D-01 | | PLY/XYZ/Both | | | | screenshot / file | Open |
| D-02 | | | | | | | |

**Severity:** P0 = cannot import; P1 = wrong scale/axes; P2 = metadata/count cosmetic; P3 = engraver-only quirk (document, no code change).

---

## H. Quick file inspection (command line)

Run from directory containing exports:

```bash
# PLY header + vertex count
head -20 qa-d01-*.ply
grep '^element vertex' qa-d01-*.ply

# XYZ line count (should match vertex count)
wc -l qa-d01-*.xyz

# Sample coordinates (mm)
head -5 qa-d01-*.xyz
```

---

## I. Automated export fixture (optional QA aid)

Rust unit test `export_ply_xyz_writes_disk_fixture_format` in `src-tauri/src/export.rs` writes a 3-point PLY/XYZ to a temp directory and asserts header/line format. Run:

```bash
cargo test --manifest-path src-tauri/Cargo.toml export_ply_xyz_writes_disk_fixture_format
```

Use output paths printed on failure for parser regression only — not a substitute for full QA-D-01.

---

## References

- [SPRINT_D_Task_Assignment.md](./SPRINT_D_Task_Assignment.md) — QA-D-01 acceptance criteria
- [docs/engraver-charter.md](../../docs/engraver-charter.md)
- [docs/testing.md](../../docs/testing.md)
