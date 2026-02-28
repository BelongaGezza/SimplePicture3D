# SimplePicture3D — Beta Tester Onboarding

Thank you for helping test SimplePicture3D. This guide explains how to get the beta build, what to expect on first run, where to find help, and how to report bugs and feedback.

**Target:** Phase 1 MVP beta (5+ testers). We use your feedback to improve stability and documentation before a wider release.

---

## 1. Getting the beta build

### Option A: Pre-built installer (recommended)

1. Go to the [Releases](https://github.com/BelongaGezza/SimplePicture3D/releases) page.
2. Download the latest **beta** build for your OS (e.g. Windows `.msi` or `.exe`, macOS `.dmg`, Linux AppImage).
3. Install using the normal installer; no admin required for user-level install on Windows.

### Option B: Build from source

If you prefer to run from source (e.g. to test the latest commits):

1. Clone the repo and follow the [Developer Guide](developer-guide.md) — **Prerequisites** and **Build from Source**.
2. Run the app with: `npm run tauri dev`.
3. When reporting bugs, include the git commit hash (e.g. `git rev-parse HEAD`).

---

## 2. First-run expectations

- **First launch:** The app may show a welcome or first-run screen and prompt for a default export folder. You can accept or change it later in Settings.
- **AI depth model:** The first time you click **Generate depth**, the app may offer to download the AI model (several hundred MB). You need this for real depth estimation. If you skip it, you can still explore the UI; depth generation will prompt again or show a clear message.
- **Python (Windows):** If you see "Python not found", install [Python 3.10+](https://www.python.org/downloads/) and ensure "Add Python to PATH" is checked. See the [User Guide — Troubleshooting](user-guide.md#troubleshooting-faq) for more.

---

## 3. Where to find help

| Resource | Purpose |
|----------|---------|
| [User Guide](user-guide.md) | Installation, first conversion walkthrough, depth controls, export, settings, troubleshooting FAQ |
| [README](../README.md) | Project overview, quick start, links to docs |
| [Releases](https://github.com/BelongaGezza/SimplePicture3D/releases) | Download installers and release notes |

---

## 4. How to report bugs and feedback

### Filing a bug report

1. Open [GitHub Issues](https://github.com/BelongaGezza/SimplePicture3D/issues) and click **New issue**.
2. Choose **Bug report** (this uses our [bug report template](../.github/ISSUE_TEMPLATE/bug_report.md)).
3. Fill in:
   - **Priority** (P0–P3: Critical / High / Medium / Low)
   - **Environment:** OS, app version or commit, hardware (RAM, GPU if relevant)
   - **Steps to reproduce** (numbered)
   - **Expected vs actual behaviour**
   - Any **screenshots, logs, or sample image** that help reproduce the issue.

The more precise the steps and environment, the faster we can fix it. Thank you!

### General feedback and questions

- **GitHub Discussions:** Use [Discussions](https://github.com/BelongaGezza/SimplePicture3D/discussions) for questions, feature ideas, or general feedback (not for bugs — use Issues for those).

---

## 5. What we’re looking for in beta

- **Core workflow:** Load image → Generate depth → Adjust depth → Export STL/OBJ. Does it work on your machine? Any crashes or confusing messages?
- **Export:** Do exported STL/OBJ files open correctly in your preferred viewer (e.g. MeshLab, Blender)? Do target dimensions (e.g. 50×70 mm) behave as expected?
- **Settings:** Do settings (depth range, target size, window size) persist after restart?
- **Installation and first run:** Was the installer and first-run experience clear? Any missing dependencies or errors?

If you have time, you can follow the [Manual Testing Checklist](testing.md) and note any step that fails or is unclear.

---

## 6. Contact

- **Bugs:** [GitHub Issues](https://github.com/BelongaGezza/SimplePicture3D/issues) (use the Bug report template).
- **Questions / feedback:** [GitHub Discussions](https://github.com/BelongaGezza/SimplePicture3D/discussions).

Thank you for being a beta tester — your feedback directly shapes the next release.
