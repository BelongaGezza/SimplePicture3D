# UI Screenshots — Index and Usage

**Purpose:** List of all UI screenshots for user-facing documentation.  
**Sprint:** 1.12 — DOC-103  
**Format:** PNG, consistent resolution (e.g. 1280×720 or 1920×1080 for readability).  
**Location:** All screenshot files live in `docs/images/`.

---

## Screenshot list

Capture the following UI states and save with the filenames below. Use a single scale (e.g. 100% or 125% UI) and the same window size where possible for consistency.

| # | Filename | UI state to capture | Status |
|---|----------|---------------------|--------|
| 1 | `user-guide-welcome.png` | Welcome or first-run screen (after install; may include model download prompt or “Get started”) | To be captured |
| 2 | `user-guide-workspace-empty.png` | Main workspace with **no** image loaded — left sidebar empty, right sidebar with Generate disabled | To be captured |
| 3 | `user-guide-workspace-with-image.png` | Main workspace **with** image loaded — image in left panel, ready for Generate | To be captured |
| 4 | `user-guide-depth-preview.png` | Right sidebar after **Generate** — depth map preview visible, Depth controls panel below | To be captured |
| 5 | `user-guide-3d-preview.png` | Center viewport showing **3D mesh preview** (Points, Wireframe, or Solid) | To be captured |
| 6 | `user-guide-export-panel.png` | Export panel or dialog — format choice (STL/OBJ), path selector, Save button | To be captured |
| 7 | `user-guide-export-success.png` | Export success message (toast or dialog: “Export complete” / “Saved to …”) | To be captured |
| 8 | `user-guide-settings.png` | Settings panel — e.g. default export path, target dimensions (if present), model option | To be captured |

---

## Where each screenshot is used

| Filename | Used in |
|----------|--------|
| `user-guide-welcome.png` | [User Guide](../user-guide.md) — Installation section (after install); README optional |
| `user-guide-workspace-empty.png` | User Guide or README — “Main window” / “Workspace” (optional) |
| `user-guide-workspace-with-image.png` | User Guide — First conversion walkthrough, Step 1 (Load an image) |
| `user-guide-depth-preview.png` | User Guide — First conversion walkthrough, Step 2 (Generate depth map) |
| `user-guide-3d-preview.png` | User Guide — First conversion walkthrough, Step 4 (Preview in 3D) |
| `user-guide-export-panel.png` | User Guide — First conversion walkthrough, Step 5 (Export) |
| `user-guide-export-success.png` | User Guide — First conversion walkthrough, Step 5 (Export success) |
| `user-guide-settings.png` | User Guide — Settings and target dimensions; optional in README |

---

## Capture instructions (for human or QA)

1. **Resolution:** Use a fixed window size (e.g. 1280×720 or 1920×1080) and capture at 1:1 (no scaling) for sharp text.
2. **Format:** Save as PNG. Optionally optimize with `pngquant` or similar for smaller file size.
3. **Content:** Avoid sensitive or third-party artwork; use a neutral test image (e.g. project sample or placeholder).
4. **OS:** Prefer one primary OS (e.g. Windows) for the user guide; note in this file if screenshots are OS-specific.
5. **Place files** in `docs/images/` with the exact filenames above so the User Guide links resolve.

---

## Placeholder / missing images

Until screenshots are captured, the User Guide references these filenames with alt text. If an image is missing, the link will 404 until the file is added. Replace “To be captured” in the table above with “Done” and the date when each screenshot is added.

**Last updated:** 2026-02-28 (Sprint 1.12; UI Designer — DOC-103)
