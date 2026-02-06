# SimplePicture3D User Guide

**Purpose:** End-user documentation for installation, first conversion, and daily use.  
**Audience:** Hobbyists and users converting 2D images to 2.5D for laser engraving.  
**Source:** `prd.md` §4.4; maintained by Documentation Specialist.

---

## Installation

SimplePicture3D is not yet released. When available, installers will be listed on the [README](../README.md) (Windows, macOS, Linux). For development setup, see the README’s **Development** section and the [Python environment](../README.md#python-environment-ai-backend) notes for the AI backend.

---

## First Conversion (Planned Flow)

1. **Load an image** — Use the file picker or drag-and-drop to load a supported image (e.g. PNG, JPG). The app validates format and size.
2. **Generate depth map** — Click **Generate** to run AI depth estimation. Progress is shown; when complete, a depth map preview appears in the right sidebar.
3. **Adjust depth** — Use the depth controls (see below) to refine the result.
4. **Preview** — The depth preview updates as you change settings. A 3D mesh preview is planned for a later release.
5. **Export** — Save as STL or OBJ when mesh export is available.

---

## Depth Controls (Sprint 1.5)

After generating a depth map, the **Depth controls** panel appears in the right sidebar. All controls are disabled until a depth map is loaded.

### Where to find them

- **Location:** Right sidebar, below the **Generate** button and the depth map preview.
- **Visibility:** The panel is shown when a depth map exists; it is disabled or hidden when no depth map is loaded.

### Controls

| Control | What it does | Typical range |
|--------|----------------|----------------|
| **Depth range (min)** | Minimum depth in millimetres (darker = closer to viewer in preview). | 1–20 mm (default 2) |
| **Depth range (max)** | Maximum depth in millimetres (brighter = farther). | 1–20 mm (default 10) |
| **Brightness** | Shifts all depth values up or down (additive). | -0.5 to 0.5 (default 0) |
| **Gamma** | Adjusts midtones (higher = brighter midtones, lower = darker). | 0.5–2 (default 1) |
| **Invert depth** | Swaps near and far (checkbox). | On / Off (default Off) |
| **Reset** | Restores the original AI-generated depth and resets all sliders to defaults. | — |

### How the preview updates

- Changing any slider or the invert checkbox updates the depth preview after a short delay (debounced, ~100 ms) so the UI stays responsive.
- The **original** depth from the AI is kept in memory. **Reset** restores that original and sets all adjustment parameters back to their defaults (brightness 0, contrast 1, gamma 1, invert off, depth range 2–10 mm).

### Tips

- Use **Depth range** to match your laser’s working range (e.g. 2–10 mm for many internal engravers).
- Use **Invert depth** if the near/far sense is reversed for your image.
- Use **Reset** to start over from the raw AI output if you’ve changed too much.

---

## FAQ (Planned)

- **Supported image formats?** — See the app’s file picker; common formats include PNG, JPG, TIFF. Very large images may be downsampled.
- **Where is the depth map stored?** — Processed entirely on your machine; nothing is sent to the cloud.
- **Can I undo adjustments?** — Use **Reset** to return to the original depth; per-step undo is planned for a later release.

---

## Related Documentation

- [Architecture](architecture.md) — Technical design and depth-adjustment pipeline (for developers).
- [README](../README.md) — Project overview, build, and testing.

**Last updated:** 2026-02-06 (Sprint 1.5; Documentation Specialist)
