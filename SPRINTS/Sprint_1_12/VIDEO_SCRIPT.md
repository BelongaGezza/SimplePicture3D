# SimplePicture3D — Video Tutorial Script (3–5 min)

**Purpose:** Script for a short video tutorial covering the core workflow.  
**Audience:** New users and beta testers.  
**Sprint:** 1.12 — DOC-102  
**Estimated duration:** 3–5 minutes when read at normal pace (with screen actions).

---

## Pre-roll (0:00–0:15)

**[Screen: App icon or title card]**

- "Hi, this is a quick tour of SimplePicture3D — an app that turns 2D images into 2.5D meshes for UV laser engraving in crystal and glass."
- "We’ll load an image, generate depth with AI, adjust it, preview in 3D, and export as STL or OBJ."

---

## 1. Open the app and load an image (0:15–0:45)

**[Screen: Welcome or first-run screen, then main workspace empty]**

- "Open SimplePicture3D. If it’s your first run, you may see a welcome or setup wizard — you can download the AI model now or skip and do it later."
- "The main window has a workspace with a left sidebar for the image and a right sidebar for depth controls."
- "To load an image: drag and drop a photo onto the window, or click **Load image** and pick a PNG or JPG from your computer."
- **[Action: Load a sample image.]**
- "Once the image is loaded, it appears here on the left. The app supports images up to 8192 by 8192; bigger ones are automatically resized."

---

## 2. Generate depth map (0:45–1:30)

**[Screen: Right sidebar with Generate button; then depth preview visible]**

- "Next, generate the depth map. Click **Generate** in the right sidebar."
- **[Action: Click Generate.]**
- "The first time you do this, the app may download the AI model — that can take a few minutes and needs internet. After that, it runs fully offline."
- "You’ll see a progress indicator. When it’s done, a grayscale depth preview appears: darker means closer to the viewer, brighter means farther away."
- "If the near and far look reversed for your image, don’t worry — we’ll fix that in the next step with one click."

---

## 3. Adjust depth (1:30–2:15)

**[Screen: Depth controls panel — sliders and Invert]**

- "Use the **Depth controls** panel below the preview. The **depth range** sliders set the output in millimetres — for example 2 to 10 mm for many laser engravers."
- "**Brightness**, **contrast**, and **gamma** fine-tune how the depth is mapped. **Invert depth** flips near and far if the preview looked backwards."
- **[Action: Adjust one slider and show preview updating; optionally toggle Invert.]**
- "The preview updates as you change settings. If you go too far, click **Reset** to go back to the original AI result and start over."

---

## 4. Preview in 3D (2:15–2:50)

**[Screen: Center 3D viewport with mesh]**

- "The center area shows a 3D preview of your mesh. Drag to rotate, scroll to zoom, and pan to inspect from different angles."
- **[Action: Rotate and zoom the 3D view.]**
- "You can switch between Points, Wireframe, and Solid view if your mesh supports it. Check that the depth and proportions look good before exporting."

---

## 5. Export STL or OBJ (2:50–3:30)

**[Screen: Export button/panel; then save dialog]**

- "When you’re happy with the result, click **Export**."
- "Choose **STL** — good for most laser software — or **OBJ** if your workflow needs it. Pick a save location; the app often suggests a folder like Documents, SimplePicture3D, exports."
- **[Action: Select STL or OBJ, choose path, save.]**
- "The filename is usually generated from your image name and a timestamp; you can change it. After saving, you’ll get a confirmation. Your file is ready to open in your laser engraving or 3D software."

---

## 6. Optional: first-run wizard and settings (3:30–4:00)

**[Screen: First-run wizard or Settings panel — brief]**

- "Quick optional tips: On first run, the wizard can set your default export folder and remind you that everything runs offline — no data is sent to the cloud."
- "In Settings you can set **target dimensions** in millimetres so the exported mesh fits your crystal or blank size. The app keeps the aspect ratio and fits the mesh inside that rectangle."

---

## Outro (4:00–4:20)

**[Screen: Final frame — app or docs link]**

- "That’s the basic workflow: load image, generate depth, adjust, preview in 3D, and export. For more details, installation on macOS and Linux, and troubleshooting, see the user guide in the docs folder or on GitHub."
- "Thanks for watching."

---

## Cues for recording

| Section        | Screen action focus                    | Duration (approx) |
|----------------|----------------------------------------|-------------------|
| Pre-roll      | Title card or app icon                 | 15 s              |
| Load image    | Drag-drop or file picker; image in UI  | 30 s              |
| Generate      | Click Generate; show progress & result  | 45 s              |
| Adjust        | Sliders, Invert; preview update        | 45 s              |
| 3D preview    | Rotate, zoom 3D view                   | 35 s              |
| Export        | Export panel, format, path, save       | 40 s              |
| Optional      | Wizard or Settings (short)             | 30 s              |
| Outro         | Link to docs                           | 20 s              |

**Total:** ~4 min 20 s (with minimal pause). Stretch to 5 min by slowing narration or showing more examples.

---

## Alignment with user guide

- Flow matches [docs/user-guide.md](../../docs/user-guide.md) **First conversion walkthrough** (Steps 1–5).
- Optional section aligns with **Settings and target dimensions** and first-run wizard in the user guide.
- Outro points viewers to the user guide for installation (Windows/macOS/Linux), troubleshooting, and FAQ.

**Last updated:** 2026-02-28 (Sprint 1.12; UI Designer — DOC-102)
