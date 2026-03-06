# SimplePicture3D — Manual Testing Checklist

This document provides a repeatable manual testing checklist for QA and beta testers. It aligns with [todo.md § Manual Testing Checklist](../todo.md) and [Sprint 1.11 Test Plan](../SPRINTS/Sprint_1_11/TEST_PLAN_1_11.md). Automated tests (Rust, frontend, Python) are documented in [Developer Guide](developer-guide.md#testing-and-linting) and [CONTRIBUTING.md](../CONTRIBUTING.md).

**Audience:** Quality Engineers, beta testers, developers before release.  
**Phase 1 MVP:** Focus on Core Workflow and Export Verification below.

---

## 1. Core workflow (every sprint / before release)

Execute with the app running (`npm run tauri dev` or installed build).

| # | Step | How to verify | Pass? |
|---|------|----------------|-------|
| 1 | **Load image** | File → Open or drag-drop PNG/JPG. Image appears in 2D view; no crash. Try small (e.g. 800×600) and large (e.g. 4K); >8K should downsample with notice. | ☐ |
| 2 | **Generate depth** | Click Generate depth. Progress shows; depth map appears (grayscale: darker = closer). First run may trigger model download. | ☐ |
| 3 | **Adjust depth** | Change Brightness, Contrast, Gamma, Invert; depth preview updates. Set depth range (min/max mm). Reset restores defaults. | ☐ |
| 4 | **Generate mesh / 3D preview** | Mesh builds from depth; 3D preview shows. Rotate, zoom; no crash. Vertex count and dimensions reasonable for image size. | ☐ |
| 5 | **Export STL** | Export → STL, choose path. File created; open in MeshLab (or Netfabb/Blender) and confirm mesh displays. | ☐ |
| 6 | **Export OBJ** | Export → OBJ, choose path. File + MTL created; open in viewer and confirm mesh displays. | ☐ |
| 7 | **Settings persistence** | Change a setting (e.g. depth range, target dimensions). Restart app; setting is restored. | ☐ |

**Edge cases (optional for quick pass):**

- Load invalid/corrupt file → app shows error, no crash.
- Load image with unusual aspect (very wide/tall) → depth and mesh generate; export succeeds.
- Cancel depth generation mid-run (if supported) → app recovers.

---

## 2. Export verification

| # | Step | How to verify | Pass? |
|---|------|----------------|-------|
| 1 | **STL binary** | Exported STL opens in MeshLab; mesh is manifold (no obvious holes); normals consistent. | ☐ |
| 2 | **OBJ + MTL** | Exported OBJ and MTL open in viewer; materials/textures as expected (if any). | ☐ |
| 3 | **Target dimensions** | In Export settings, set Output size to **50×70 mm**. Export STL; open in MeshLab. Mesh XY bounds fit inside 50×70 mm; aspect ratio matches image (no stretch). | ☐ |

---

## 3. Settings and target dimensions

| # | Step | How to verify | Pass? |
|---|------|----------------|-------|
| 1 | **Depth range (min/max mm)** | Set e.g. 2–10 mm; export; mesh Z range is in that range when measured in MeshLab. | ☐ |
| 2 | **Target width/height presets** | Select preset (e.g. 50×70 mm); export; verify dimensions in external tool. | ☐ |
| 3 | **Custom target dimensions** | Enter custom width × height (mm); export; verify. | ☐ |
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

## 6. Platform-specific (Phase 3)

Deferred to Phase 3; placeholder for:

- Windows 10 / 11
- macOS (Intel + Apple Silicon)
- Linux (e.g. Ubuntu 22.04, Fedora)
- Installer install/uninstall
- Auto-update flow (if implemented)

---

## 7. Accessibility (Phase 4)

Deferred to Phase 4; placeholder for:

- Keyboard navigation (Tab, Enter, Escape)
- Screen reader (NVDA, VoiceOver)
- High contrast / color-blindness considerations

---

## References

- **Automated tests:** [Developer Guide — Testing and Linting](developer-guide.md#testing-and-linting), [CONTRIBUTING.md](../CONTRIBUTING.md)
- **Sprint 1.11:** [TEST_PLAN_1_11.md](../SPRINTS/Sprint_1_11/TEST_PLAN_1_11.md), [MANUAL_TEST_REPORT.md](../SPRINTS/Sprint_1_11/MANUAL_TEST_REPORT.md)
- **Source checklist:** [todo.md — Manual Testing Checklist](../todo.md)
