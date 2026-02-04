# Sprint 1.4 — Manual Test Walkthrough (Windows)

**Purpose:** Step-by-step instructions to run manual test cases on Windows. After each case, note your results and share them so they can be recorded in `MANUAL_TEST_REPORT.md`.

**Project path used below:** `c:\Users\gerry\develop\SimplePicture3D`  
If your repo is elsewhere, replace that path in the commands.

---

## Prerequisites (do once)

### 1. Open a terminal in the project root

- **PowerShell or Command Prompt:**  
  `cd c:\Users\gerry\develop\SimplePicture3D`
- Or in VS Code / Cursor: open the project folder and use the integrated terminal (it usually starts in the project root).

### 2. Install Node dependencies (if not already done)

```powershell
npm install
```

### 3. Python environment (needed for Cases 1, 3, 4)

- **Option A — Use system Python (3.10+):**  
  Ensure `python` is on PATH. In a new terminal run:  
  `python --version`  
  You should see Python 3.10 or higher.

- **Option B — Use a venv (recommended):**

  ```powershell
  python -m venv venv
  .\venv\Scripts\Activate.ps1
  pip install -r python\requirements.txt
  ```

  Then start the app **from the same terminal** (or one where you’ve activated this venv) so that `VIRTUAL_ENV` is set and the Rust backend finds `venv\Scripts\python.exe`.

### 4. Start the app (for all UI cases)

From the project root, with Python available if you’re running depth generation:

```powershell
npm run tauri dev
```

- Wait until the Tauri window opens (Svelte + native window).
- You should see: left sidebar “Original image”, center “3D Preview”, right sidebar “Depth map” and a “Generate Depth Map” button.

Keep this app running for **Case 1**, **Case 2a**, **Case 3**, and **Case 4**. For **Case 2b** (missing Python) you’ll restart the app under a modified environment.

---

## Case 1: Happy path — Generate depth map (QA-301)

**Goal:** Load an image, click Generate Depth Map, see progress and then a grayscale depth map.

### Steps

1. **Start the app** (if not already):  
   `npm run tauri dev` from project root.

2. **Load an image**
   - In the left sidebar, use the control that lets you choose a file (e.g. “Select image” or similar).
   - Navigate to:  
     `c:\Users\gerry\develop\SimplePicture3D\tests\fixtures\`
   - Select **valid_small.png** (or **valid_1x1.png**). **Do not** select `corrupt_truncated.png` or `invalid_not_an_image.png` — those are invalid fixtures and will show “image decode failed (file may be corrupt)” or “unsupported or invalid image format”.
   - If you get “image decode failed”, regenerate fixtures from project root:  
     `node scripts\gen_fixtures.mjs`  
     then try again, or use any other known-good PNG (e.g. from your Pictures folder).
   - After a successful load you should see the image preview in the left panel and metadata (dimensions, size).

3. **Generate depth**
   - In the right sidebar, click **“Generate Depth Map”**.
   - **Observe:**  
     - Button label changes to something like “Estimating…” and the button is disabled.  
     - An indeterminate progress bar appears.  
     - Status (e.g. bottom of window) shows “Estimating depth…”.
   - Wait until processing finishes (stub is fast; real model may take seconds to minutes).

4. **After completion**
   - **Observe:**  
     - Progress/“Estimating…” stops.  
     - Right sidebar shows a grayscale depth image (same aspect ratio as the source).  
     - Left sidebar still shows the original image (side-by-side).  
     - Status text changes to something like “Depth ready”.

5. **Repeat with at least 2 more images (per QA-301)**  
   - Load `tests\fixtures\valid_1x1.png`, click Generate, confirm depth appears.  
   - Load any other PNG or JPG you have (e.g. a photo from your Pictures folder), click Generate, confirm depth appears.

### What to report

- Did the app crash at any point? (Yes/No)
- Did you see progress (button “Estimating…”, progress bar, status text)? (Yes/No)
- Did a grayscale depth map appear in the right panel for all images? (Yes/No)
- Any error message or unexpected behavior? (describe briefly)

---

## Case 2: Error handling (no image, missing Python)

**Goal:** Confirm (a) Generate is disabled with no image, (b) with Python unavailable you get a clear error (no crash).

### Case 2a: No image loaded

1. **Start the app** (or leave it running from Case 1).
2. **Do not load an image** (or clear/reset so no image is loaded — e.g. close and reopen the app and don’t load anything).
3. **Check the “Generate Depth Map” button:**  
   It should be **disabled** (greyed out, not clickable).
4. **Check status/footer:**  
   No need to see a special message; main point is you cannot trigger Generate with no image.

**What to report:** Was the Generate button disabled when no image was loaded? (Yes/No)

---

### Case 2b: Python not available (optional but recommended)

**Note:** Do *not* clear PATH entirely — that would prevent `npm` from running. Use one of the methods below so the **app** starts (npm/node still work) but the **Rust backend** cannot find `python` when it spawns the depth estimator.

**Option A — Temporarily rename Python (simplest)**

1. **Close the app** if it’s running.
2. **Rename the Python executable** that the app would use:
   - If you use a **venv:** rename  
     `c:\Users\gerry\develop\SimplePicture3D\venv\Scripts\python.exe`  
     to  
     `python.exe.bak`  
     (in File Explorer or:  
     `Rename-Item .\venv\Scripts\python.exe python.exe.bak`  
     in PowerShell from project root).
   - If you use **system Python:** rename the `python.exe` in the folder that’s first on your PATH (e.g. `C:\Users\<You>\AppData\Local\Programs\Python\Python312\python.exe`) to `python.exe.bak`.
3. **Start the app** from a normal terminal (so npm works):  
   `cd c:\Users\gerry\develop\SimplePicture3D`  
   `npm run tauri dev`
4. **In the app:** Load any image, then click **“Generate Depth Map”**.
5. **Expected:** No crash; an error message (e.g. “failed to spawn Python” or “is Python on PATH”) in the right sidebar or near the button.
6. **Clean up:** Close the app, then rename the file back (e.g. `python.exe.bak` → `python.exe`).

**Option B — Remove only Python/venv from PATH (keep Node)**

1. **Close the app.** Open a **new** PowerShell at project root.
2. **Set PATH to exclude Python/venv** (Node stays so npm works):
   ```powershell
   $env:Path = ($env:Path -split ';' | Where-Object { $_ -notmatch 'Python' -and $_ -notmatch '\\venv\\' }) -join ';'
   ```
3. **Start the app in this terminal:**  
   `npm run tauri dev`
4. **In the app:** Load an image, click **“Generate Depth Map”**. Expect error (Python not found).
5. **Clean up:** Close the app; close this terminal. Use a normal terminal (with full PATH) for other work.

**What to report:**  
- Did you run 2b? (Yes/No)  
- If yes: Did you get a clear error message and no crash? (Yes/No)  
- Exact or approximate error text (optional): …

---

## Case 3: Depth accuracy — qualitative (QA-302)

**Goal:** Check that depth looks plausible (e.g. foreground vs background differ); note if it’s inverted or all same.

1. **Start the app** with Python available; load an image that has a **clear subject and background** (e.g. person in front of sky, or object in front of a wall).  
   - If you don’t have one, use any photo with distinct near/far regions.
2. Click **“Generate Depth Map”** and wait for completion.
3. **Look at the grayscale depth in the right panel:**
   - **Convention we use:** nearer = darker, farther = lighter (or the opposite — we’ll document what you see).
   - Check: do closer and farther regions have different shades, or does everything look the same?
   - Note if it clearly looks “inverted” (e.g. sky darker than person).

**What to report:**  
- Did foreground and background show different shades? (Yes/No / N/A e.g. stub)  
- Did depth “look correct” or “inverted” or “all same”? (one short phrase)  
- If using stub only: say “stub only — no real model”; that’s still a valid result.

---

## Case 4: Performance — 4K inference time (QA-303)

**Goal:** Measure time from clicking Generate until the depth map is shown, for a 4K (or largest available) image.

1. **Get a 4K image (3840×2160)**  
   - If you don’t have one: create a placeholder with ImageMagick (if installed):  
     `magick -size 3840x2160 xc:gray c:\Users\gerry\develop\SimplePicture3D\tests\fixtures\4k.png`  
   - Or use any image you have that’s 3840×2160 or the largest you can use (e.g. 1920×1080); note the resolution in your report.
2. **Start the app** with Python available.
3. **Load the 4K (or large) image.**
4. **Start a timer** (phone stopwatch, or PowerShell:  
   `(Get-Date); ... do click ...; (Get-Date)`).
5. **Click “Generate Depth Map”** and leave the app in focus.
6. **Stop the timer** when the depth map appears in the right panel (grayscale visible, “Estimating…” finished).
7. **Note:**  
   - Elapsed time (seconds).  
   - Image size used (e.g. 3840×2160 or 1920×1080).  
   - Whether you’re on GPU or CPU (if you know; e.g. “NVIDIA GPU” or “CPU only”).

**What to report:**  
- Image dimensions: …  
- Elapsed time (seconds): …  
- GPU or CPU (if known): …  
- Target is &lt;30s for 4K on GPU; note “met” or “gap” if you have a target in mind.

---

## Case 5: Dimensions match (QA-304)

This is covered by an **automated** test. You can skip manual steps unless you want to double-check.

**Optional manual check:**  
Load an image of known size (e.g. 100×100), generate depth, and see if the depth preview has the same aspect ratio or dimensions (e.g. in UI or devtools). Reporting this is optional.

**What to report:**  
- “Skipped — automated test only” or “Checked manually: dimensions matched / didn’t match.”

---

## After you’re done

Reply with something like:

- **Case 1:** Pass/Fail + your answers (crash? progress? depth shown? any errors).
- **Case 2a:** Pass/Fail (button disabled with no image?).
- **Case 2b:** Pass/Fail + error message if you ran it.
- **Case 3:** Pass/Fail + one-line description (e.g. “looks correct”, “stub only”, “inverted”).
- **Case 4:** Pass/Fail + time, resolution, GPU/CPU.
- **Case 5:** Skipped or optional result.

I’ll record your answers in `MANUAL_TEST_REPORT.md` (Actual result and Pass/Fail for each case).
