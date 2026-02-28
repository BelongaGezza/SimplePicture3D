# SimplePicture3D User Guide

**Purpose:** End-user documentation for installation, first conversion, and daily use.  
**Audience:** Hobbyists and users converting 2D images to 2.5D for laser engraving.  
**Source:** `prd.md` §4; maintained by UI Designer / Documentation Specialist.

---

## Table of Contents

1. [Installation](#installation)
2. [First conversion walkthrough](#first-conversion-walkthrough)
3. [Depth controls](#depth-controls)
4. [3D preview and export](#3d-preview-and-export)
5. [Settings and target dimensions](#settings-and-target-dimensions)
6. [AI models and licenses](#ai-models-and-licenses)
7. [Troubleshooting FAQ](#troubleshooting-faq)
8. [Related documentation](#related-documentation)

---

## Installation

### Windows

1. **Download the installer**  
   When released, download the Windows installer from the [Releases](https://github.com/BelongaGezza/SimplePicture3D/releases) page (`.msi` or `.exe`).
2. **Run the installer**  
   Double-click the installer. No administrator rights are required for a user-level install.
3. **First launch**  
   On first run, the app may prompt you to download the AI depth model (see [AI models and licenses](#ai-models-and-licenses)) and to choose a default export folder. You can skip the model download and use the app in a limited way until the model is installed.
4. **Optional: Python for depth estimation**  
   The app uses a bundled or system Python for AI depth estimation. If you see "Python not found", install [Python 3.10+](https://www.python.org/downloads/) and ensure "Add Python to PATH" is checked. On some Windows setups, the Tauri app may not see your terminal PATH—installing Python for "all users" or adding it to the system PATH often fixes this.

![Installation complete — welcome screen](images/user-guide-welcome.png)  
*After installation, the welcome or first-run screen.*

### macOS and Linux

- **macOS:** A `.dmg` disk image will be provided at release. Open it and drag SimplePicture3D to Applications. On first run, you may need to allow the app in System Preferences → Security & Privacy.
- **Linux:** AppImage or `.deb` packages will be listed on the Releases page. Make the AppImage executable (`chmod +x`) and run it; or install the `.deb` with your package manager.

**Pre-release / development:** If you are building from source, see the [README](../README.md) **Development Setup** section and [Developer Guide](developer-guide.md).

---

## First conversion walkthrough

Follow these steps to convert your first 2D image into a 2.5D mesh for laser engraving.

### Step 1: Load an image

1. Open SimplePicture3D.
2. **Drag and drop** an image onto the workspace, or click **Open image** / **Load image** in the left sidebar to browse.
3. Supported formats: **PNG**, **JPG** (and optionally TIFF). Maximum size is 8192×8192 pixels; larger images are automatically downsampled and you’ll see a notice.
4. After loading, the image appears in the left panel and the app is ready for depth generation.

![Main workspace with image loaded](images/user-guide-workspace-with-image.png)  
*Main workspace after loading an image.*

### Step 2: Generate depth map

1. Click **Generate** (or **Generate depth**) in the right sidebar.
2. The first time you use a model, the app may download it (see [Troubleshooting FAQ](#troubleshooting-faq) if download fails).
3. A progress indicator shows while the AI runs. When finished, a **depth map preview** appears (grayscale: darker = closer, brighter = farther).
4. If the near/far sense looks wrong, you can fix it later with **Invert depth** in the depth controls.

![Depth map generated — preview in sidebar](images/user-guide-depth-preview.png)  
*Depth map preview after generation.*

### Step 3: Adjust depth

1. In the **Depth controls** panel (right sidebar), refine the result:
   - **Depth range (min/max)** — Set the output depth in millimetres (e.g. 2–10 mm for many laser engravers).
   - **Brightness / Contrast / Gamma** — Tweak the depth distribution.
   - **Invert depth** — Swap near and far if the preview looks backwards.
2. The depth preview updates as you change settings. Use **Reset** to restore the original AI output and start over.

See [Depth controls](#depth-controls) for full details.

### Step 4: Preview in 3D

1. The **center panel** shows a 3D preview of the mesh. Rotate (drag), zoom (scroll or pinch), and pan to inspect.
2. You can switch between **Points**, **Wireframe**, and **Solid** view if the mesh is triangulated.
3. Check that the depth and proportions look correct before exporting.

![3D preview with mesh](images/user-guide-3d-preview.png)  
*3D preview of the mesh.*

### Step 5: Export STL or OBJ

1. Click **Export** (or open the export panel at the bottom).
2. Choose **STL** (binary) or **OBJ** (ASCII with material file). Both are suitable for laser engraving software.
3. Pick a save location. The app suggests a default (e.g. `Documents/SimplePicture3D/exports/`) and can remember the last folder.
4. The filename is often auto-generated from the image name and timestamp; you can change it.
5. Click **Save**. A progress indicator appears for large files. When done, you’ll see a success message.

![Export panel — format and path](images/user-guide-export-panel.png)  
*Export panel: choose format and path.*

![Export success message](images/user-guide-export-success.png)  
*Export complete confirmation.*

You can now open the STL or OBJ in your laser engraving or 3D software.

---

## Depth controls

After generating a depth map, the **Depth controls** panel appears in the right sidebar. All controls are disabled until a depth map is loaded.

### Location

- **Right sidebar**, below the **Generate** button and the depth map preview.
- The panel is visible when a depth map exists; it may be disabled or collapsed when no depth map is loaded.

### Controls

| Control | What it does | Typical range |
|--------|----------------|----------------|
| **Depth range (min)** | Minimum depth in millimetres (darker = closer in preview). | 1–20 mm (default 2) |
| **Depth range (max)** | Maximum depth in millimetres (brighter = farther). | 1–20 mm (default 10) |
| **Brightness** | Shifts all depth values up or down. | -0.5 to 0.5 (default 0) |
| **Contrast** | Expands or compresses midtones. | 0.5–2 (default 1) |
| **Gamma** | Adjusts midtones (higher = brighter midtones). | 0.5–2 (default 1) |
| **Invert depth** | Swaps near and far (checkbox). | On / Off (default Off) |
| **Reset** | Restores the original AI depth and resets all sliders to defaults. | — |

### How the preview updates

- Changing any slider or the invert checkbox updates the depth preview after a short delay so the UI stays responsive.
- The **original** depth from the AI is kept in memory. **Reset** restores that original and sets all adjustment parameters back to their defaults.

### Tips

- Use **Depth range** to match your laser’s working range (e.g. 2–10 mm for many internal engravers).
- Use **Invert depth** if the near/far sense is reversed for your image.
- Use **Reset** to start over from the raw AI output if you’ve changed too much.

---

## 3D preview and export

- **3D preview:** The center viewport shows the mesh. Drag to rotate, scroll to zoom, and use the view options (Points/Wireframe/Solid) as available.
- **Export:** Use the export button or panel to save as **STL** (binary) or **OBJ**. Default save location is typically `Documents/SimplePicture3D/exports/` (or as set in Settings). The app can remember the last export path.

---

## Settings and target dimensions

- **Settings** (e.g. via a gear icon or Settings menu): You can set default export folder, and optionally **target dimensions** (width × height in mm) so the exported mesh fits your laser-etched blank. When set, the mesh XY extent fits inside that rectangle with aspect ratio preserved.
- **First-run wizard:** On first launch, the app may offer model download, default export location, and a short privacy notice (100% offline processing).

![Settings panel — export path and target dimensions](images/user-guide-settings.png)  
*Settings: default export folder and optional target dimensions (mm).*

---

## AI models and licenses

Depth estimation uses AI models with their own licenses:

- **Depth-Anything-V2** (default): High quality. Weights are **CC-BY-NC-4.0** — **non-commercial use only**. Suitable for hobby and personal projects.
- **MiDaS** (optional): **MIT-compatible** — **commercial use allowed**. Choose this in the model setup or download wizard if you need commercial use.

The app shows which model is in use and its license when you set up or download models. For details see [RESEARCH/architecture.md](../RESEARCH/architecture.md) (ADR-004, ADR-005).

---

## Troubleshooting FAQ

### Model download

- **"Model not found" or "Download model"**  
  The first time you run depth estimation, the app needs to download the AI model (several hundred MB). Ensure you have a stable internet connection and enough disk space (e.g. in `~/.simplepicture3d/models/` or the path shown in the app). If the download fails, try again or use the model wizard/settings to retry or choose a different model (e.g. MiDaS).
- **"Python not found"**  
  Depth estimation runs in a Python environment. Install [Python 3.10+](https://www.python.org/downloads/) and ensure it is on your system PATH. On Windows, if the app still doesn’t find Python, install for "all users" or add Python to the system PATH (not only the user PATH). See README **Python environment** for developer-oriented details.

### Export path and file

- **"Export failed" or "Permission denied"**  
  Choose a folder where you have write permission (e.g. Documents, Desktop). Avoid system or protected directories. On Windows, try not saving to `Program Files` or the app install folder.
- **"File already exists"**  
  The app may auto-generate a name from the image and timestamp. Change the filename or save to a different folder if you want to keep an existing file.
- **Where are my exported files?**  
  By default, exports go to `Documents/SimplePicture3D/exports/` (or the path you set in Settings). Check the path shown in the export dialog or in Settings.

### Common errors

- **"Unsupported image format"**  
  Use PNG or JPG. If you use TIFF, ensure it’s supported by the current build. Very large images may be downsampled; if something fails, try a smaller or re-saved image.
- **"Image too large" / slow or out of memory**  
  Images over 8192×8192 are downsampled. For very large files, try resizing in an image editor first to speed up processing and reduce memory use.
- **Depth preview or 3D view not updating**  
  Use **Reset** in the depth panel to restore the original depth, then adjust again. If the 3D view is slow, the app may use a reduced-detail preview for large meshes; export still uses full detail.
- **App won’t start or crashes on launch**  
  Ensure your OS and GPU drivers are up to date. On Windows, if you built from source, run `npm run tauri dev` from the project root and check the terminal for errors. See [Developer Guide](developer-guide.md) and [RESEARCH/GOTCHAS.md](../RESEARCH/GOTCHAS.md) for known issues.

### Supported image formats and size

- **Formats:** PNG, JPG (and TIFF where supported). Check the file picker in the app for the current list.
- **Size:** Max 8192×8192 pixels. Larger images are automatically downsampled with a notification.
- **Privacy:** All processing is done on your machine; no images are sent to the cloud.

---

## Related documentation

- [README](../README.md) — Project overview, quick start, and links.
- [Developer Guide](developer-guide.md) — Build from source, architecture, contribution.
- [Architecture](architecture.md) — Technical design (for developers).
- [Screenshot index](images/SCREENSHOTS.md) — List of UI screenshots and where they are used.

**Last updated:** 2026-02-28 (Sprint 1.12; UI Designer — DOC-101)
