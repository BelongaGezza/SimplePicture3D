# SimplePicture3D — Manual Testing Checklist

This document provides a repeatable manual testing checklist for QA and beta testers. It reflects the **ADR-012 surface-map point cloud** workflow (PLY/XYZ/CSV export). Automated tests are documented in [Developer Guide](developer-guide.md#testing-and-linting) and [CONTRIBUTING.md](../CONTRIBUTING.md).

**Audience:** Quality Engineers, beta testers, developers before release.  
**Active sprint:** Phase D validation — see [SPRINTS/Sprint_D/](../SPRINTS/Sprint_D/) and [ROADMAP.md](../ROADMAP.md).

---

## 1. Core workflow (every sprint / before release)

Execute with the app running (`npm run tauri dev` or installed build).

| # | Step | How to verify | Pass? |
|---|------|----------------|-------|
| 1 | **Load image** | File → Open or drag-drop PNG/JPG. Image appears in 2D view; no crash. Try small (e.g. 800×600) and large (e.g. 4K); >8K should downsample with notice. | ☐ |
| 2 | **Generate depth** | Click Generate depth. Progress shows; depth map appears (grayscale: darker = closer). First run may trigger model download. | ☐ |
| 3 | **Adjust depth** | Change Brightness, Contrast, Gamma, Invert; depth preview updates. Set depth range (min/max mm). Reset restores defaults. | ☐ |
| 4 | **Set blank envelope** | In Export panel: set L×W×H mm and margin, or pick a preset (e.g. Standard 50×50×80). Values persist after restart. | ☐ |
| 5 | **Set sampling params** | In Volumetric params: adjust pixel step and depth threshold; point count estimate updates. | ☐ |
| 6 | **Generate point cloud / 3D preview** | Click "Load point cloud" in 3D preview. Points appear inside yellow blank wireframe; rotate/zoom; no crash. | ☐ |
| 7 | **Export PLY** | Export → PLY, choose path. File created; verify in engraver software or a point cloud viewer. | ☐ |
| 8 | **Export XYZ / CSV** | Export → XYZ and CSV; files created with expected column layout. | ☐ |
| 9 | **Settings persistence** | Change blank dimensions, export format, depth range. Restart app; settings restored. | ☐ |

**Edge cases (optional for quick pass):**

- Load invalid/corrupt file → app shows error, no crash.
- Load image with unusual aspect (very wide/tall) → depth and point cloud generate; export succeeds.
- Cancel depth generation mid-run (if supported) → app recovers.

---

## 2. Export verification

| # | Step | How to verify | Pass? |
|---|------|----------------|-------|
| 1 | **PLY ASCII / binary** | Exported PLY opens in target engraver or CloudCompare/MeshLab; point count matches estimate (± stepping). | ☐ |
| 2 | **XYZ format** | Three columns (x y z); units in mm; points within blank envelope bounds. | ☐ |
| 3 | **CSV format** | Header row; comma-separated coordinates; imports cleanly. | ☐ |
| 4 | **Engraver import (Phase D gate)** | Import PLY into **XTool Studio** (xTool F2 Ultra UV, Point Cloud Engraving); axis orientation and scale correct. Record Studio version in `SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md`. | ☐ |
| 5 | **Blank fit** | Points fit inside configured blank (L×W×H) with margin; no points outside wireframe in preview. | ☐ |

---

## 3. Settings and blank envelope

| # | Step | How to verify | Pass? |
|---|------|----------------|-------|
| 1 | **Depth range (min/max mm)** | Adjust depth range; regenerate point cloud; Z distribution shifts as expected in preview. | ☐ |
| 2 | **Blank presets** | Select Standard / Cube / Large / Wide; dimensions update; wireframe resizes in preview. | ☐ |
| 3 | **Custom blank dimensions** | Enter custom L×W×H; export; verify scale in external tool. | ☐ |
| 4 | **Window/settings persist** | Resize window; change export format default; restart → settings restored. | ☐ |

---

## 4. Presets (Sprint 2.3)

Requires a depth map to be loaded (complete core workflow step 3 first).

| # | Step | How to verify | Pass? |
|---|------|----------------|-------|
| 1 | **Apply built-in preset** | Open Apply preset dropdown; select e.g. "Portrait"; click Apply. Depth sliders update to preset values; depth preview refreshes. | ☐ |
| 2 | **Save user preset** | Adjust sliders; click "Save as preset…"; enter a name; confirm. Preset appears in Saved presets list and Apply preset dropdown. | ☐ |
| 3 | **Apply user preset** | Select saved preset from dropdown; click Apply. Depth params match the saved values. | ☐ |
| 4 | **Rename preset** | In Saved presets panel, click Rename; enter new name; save. Old name gone; new name appears in list and dropdown. | ☐ |
| 5 | **Delete preset** | Click Delete on a user preset; confirm. Preset removed from list; built-ins are unaffected. | ☐ |
| 6 | **Export preset** | Click "Export preset…"; choose path. JSON file written; open it and confirm depth params are present (brightness, contrast, gamma, etc.). | ☐ |
| 7 | **Import preset** | Click "Import preset…"; choose a valid `.json` file. Depth params update to those in the file immediately. | ☐ |

**Edge cases (optional):**

- Try saving a preset with an invalid name (e.g. `../bad`) → app shows error, no file written.
- Import a malformed JSON file → app shows clear error, no crash.

---

## 5. First-run and model wizard

| # | Step | How to verify | Pass? |
|---|------|----------------|-------|
| 1 | **First launch** | Fresh install or cleared app data: welcome/first-run screen or prompts (e.g. export folder, model). | ☐ |
| 2 | **Model check** | If model not installed: UI shows option to download; starting depth generation prompts for download or shows clear message. | ☐ |
| 3 | **Model download** | Run model download; progress and completion (or clear error) shown. After success, depth generation works. | ☐ |

---

## 6. Platform-specific (Phase F)

Deferred to Phase F; placeholder for:

- Windows 10 / 11
- macOS (Intel + Apple Silicon)
- Linux (e.g. Ubuntu 22.04, Fedora)
- Installer install/uninstall
- Auto-update flow (if implemented)

---

## 7. Accessibility (Phase G)

Deferred to Phase G; placeholder for:

- Keyboard navigation (Tab, Enter, Escape)
- Screen reader (NVDA, VoiceOver)
- High contrast / color-blindness considerations

---

## References

- **Production roadmap:** [ROADMAP.md](../ROADMAP.md)
- **Architect review:** [ARCHITECT_REVIEW_2026-05.md](ARCHITECT_REVIEW_2026-05.md)
- **Automated tests:** [Developer Guide — Testing and Linting](developer-guide.md#testing-and-linting), [CONTRIBUTING.md](../CONTRIBUTING.md)
- **Phase D validation:** [SPRINTS/Sprint_D/](../SPRINTS/Sprint_D/)
- **Operational tasks:** [todo.md](../todo.md)
