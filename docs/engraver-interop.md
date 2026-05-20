# Engraver interoperability — XTool Studio + F2 Ultra UV

**Task:** DOC-D-01 (Sprint D)  
**Audience:** Makers using SimplePicture3D with an internal UV laser engraver  
**Status:** Draft — known-good Studio settings will be filled in after [QA-D-01](../SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md)

SimplePicture3D exports a **surface-map point cloud**: one laser focal point per sampled image pixel, placed at depth inside your crystal blank. This guide walks from app export to import in **XTool Studio** on the **xTool F2 Ultra UV**.

Everything runs **offline** on your machine. No cloud upload is required.

---

## What you need

| Item | Notes |
|------|--------|
| **SimplePicture3D** | Built or dev install; see [README](../README.md) |
| **XTool Studio** | Desktop app for the F2 Ultra UV |
| **xTool F2 Ultra UV** | Internal UV laser; work volume up to **70 × 70 × 150 mm** (central XY **70 × 70 mm**) |
| **Crystal blank** | Start with preset **50 × 50 × 80 (standard)** unless your blank differs |
| **Export file** | **PLY (ASCII)** — primary format for Studio import |

**Technical contract (axes, units, file layout):** [Engraver charter](engraver-charter.md) — read this if scale or orientation looks wrong.

**Formal QA checklist:** [QA-D-01 test procedure](../SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md)

---

## Coordinate quick reference

Full detail is in the [charter](engraver-charter.md). Summary:

| Axis | Meaning | Units |
|------|---------|-------|
| **X** | Crystal length (left → right in image mapping) | mm |
| **Y** | Crystal width (top → bottom in image mapping) | mm |
| **Z** | Depth into crystal (beam direction) | mm |

- Origin is the corner of the blank at **(0, 0, 0)**.
- After fit-to-blank, points sit inside your margin on all sides.
- Brighter / nearer subject in the image maps toward the **front** of the blank (higher Z before fit).

---

## Step 1 — Prepare in SimplePicture3D

1. **Open your image** — File → Open or drag a PNG/JPG onto the window.
2. **Generate depth** — Click **Generate Depth Map**. Wait until the grayscale depth view appears. (For testing without downloading the AI model, use stub mode — see [SETUP_NOTES](../SETUP_NOTES.md).)
3. **Tune depth (optional)** — Adjust brightness, contrast, gamma, or masks until the subject reads clearly in the depth preview.
4. **Set the blank** — In the **Export** panel, choose preset **50 × 50 × 80 (standard)** (or enter L×W×H and margin to match your physical blank).
   - If dimensions exceed the F2 Ultra UV processing volume (**70 × 70 × 150 mm**), an **amber** warning appears — export may not fit the machine.
   - If the etch interior is within **5 mm** of a limit, a **yellow** “too close to the edge” warning appears. These are advisory; you can still export.
5. **Check sampling** — In volumetric/sampling controls, leave **step 1** and **depth threshold 0.05** unless you need fewer points for a test run.
6. **Preview (recommended)** — In the 3D view, click **Load point cloud**. Confirm points sit inside the yellow wireframe; rotate to recognise your subject.
7. **Export PLY** — Export panel → format **PLY** → enable **ASCII PLY** → **Export point cloud** → save e.g. `my-engraving.ply`.

**Tips**

- Export regenerates the point cloud from current depth and blank settings. If you change depth or blank, export again before importing to Studio.
- **XYZ** and **CSV** are also available for backup or tools like CloudCompare; Studio’s documented point-cloud import is **PLY** ([xTool inner engraving file types](https://support.xtool.com/article/2716)).
- Keep PLY and XYZ from the **same** settings if you compare formats — point counts should match.

---

## Step 2 — Import in XTool Studio

1. Launch **XTool Studio** and connect or select the **F2 Ultra UV**.
2. Open **Inner Engraving** → **Point Cloud Engraving** (3D point-cloud mode).
3. **Import** your `*.ply` file (ASCII from Step 1).
4. Confirm the point cloud appears in the canvas with no parse error.
5. Compare size to your blank:
   - Order of magnitude should match **50 × 50 × 80 mm** (or whatever you set in the app).
   - XY should fit the machine’s **70 × 70 mm** central engraving area when using the standard preset.
6. If Studio offers an engraving or path preview, check that you see **depth relief** (a shaped cloud), not a solid block.

Record **Studio version**, any **scale** or **axis** options you touched, and pass/fail notes in your project log or [MANUAL_TEST_REPORT](../SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md) when running official QA.

---

## Known-good settings (placeholder)

> **Pending QA-D-01** — Fill this table after the first successful import on real hardware.

| Setting | Value | Notes |
|---------|-------|-------|
| XTool Studio version | _TBD_ | From Help → About |
| Import mode | Inner Engraving → Point Cloud Engraving | |
| File format | PLY ASCII | Binary PLY only if Studio rejects ASCII |
| Unit override in Studio | _TBD_ | mm expected; document if Studio asks for inches |
| Scale factor | _TBD_ | Default 1:1 if no override |
| Axis flip / mirror | _TBD_ | None unless QA finds a required flip |
| Power / speed / layer | _TBD_ | Machine-specific; not set by SP3D |
| Material preset | _TBD_ | K9 / glass / acrylic as used in test |

When QA-D-01 completes, copy confirmed values here and in the charter revision history if the contract changes.

---

## Troubleshooting

### Empty or missing point cloud in Studio

| Check | What to do |
|-------|------------|
| **Depth threshold** | Lower threshold slightly or use an image with clearer subject vs background. |
| **Depth map** | Regenerate depth; confirm the depth preview is not flat grey. |
| **Export path** | Open the PLY in a text editor: `element vertex N` should have **N > 0**. |
| **Preview in app** | If 3D preview is empty, fix in the app before re-exporting. |

### Wrong scale (tiny model or huge model)

| Symptom | Likely cause | What to do |
|---------|--------------|------------|
| Model **1000× too small or large** | Unit mismatch (m vs mm) | Confirm Studio uses **mm**. Re-export with blank dimensions matching your physical crystal. |
| Model **fits wrong blank** | Blank preset ≠ physical blank | Set L×W×H in Export panel to match real crystal; export again. |
| **Subject clipped** | Blank too small | Use a larger preset or reduce image aspect via crop before load. |

See [charter §2](engraver-charter.md) for committed units (mm) and bbox rules.

### Left/right or front/back reversed (axis flip)

| Symptom | What to do |
|---------|------------|
| Subject mirrored left–right | Note in QA report; try Studio mirror/flip if available. Do not guess — record setting for [BACK-D-01](../SPRINTS/Sprint_D/SPRINT_D_Task_Assignment.md) if a code-side fix is needed. |
| Depth inverted (near/far swapped) | Check **Invert** on depth in the app; regenerate depth and re-export. |
| Z seems backward vs crystal face | Compare with 3D preview in the app; document which physical face is “front” on your machine. |

Handedness is validated in QA-D-01; results update the [charter](engraver-charter.md) engraver-mapping note.

### PLY will not open

| Check | What to do |
|-------|------------|
| **ASCII vs binary** | For first import, use **ASCII PLY** (checkbox on in Export). |
| **File corruption** | Re-export; avoid editing the PLY by hand. |
| **Header** | First lines should include `ply`, `format ascii 1.0`, and `element vertex N`. |

### PLY vs XYZ counts differ

Export both formats **without changing** depth, blank, or sampling between clicks. `element vertex N` in the PLY header should equal the line count in the XYZ file.

### No engraver available yet

You can still validate exports with [CloudCompare](https://www.cloudcompare.org/) or [MeshLab](https://www.meshlab.net/) (bbox and units). That does **not** replace a Studio import for the Phase D gate — see [QA-D-01](../SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md).

---

## Related documents

| Document | Purpose |
|----------|---------|
| [engraver-charter.md](engraver-charter.md) | Axes, units, PLY/XYZ/CSV spec, verification checklist |
| [QA-D-01_TEST_PROCEDURE.md](../SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md) | Step-by-step QA with pass/fail matrix |
| [testing.md](testing.md) | General manual test checklist |
| [RESEARCH/architecture.md](../RESEARCH/architecture.md) | ADR-012 surface-map algorithm |

---

## Revision history

| Date | Change |
|------|--------|
| 2026-05-20 | DOC-D-01 initial draft; known-good table placeholder for QA-D-01 |
