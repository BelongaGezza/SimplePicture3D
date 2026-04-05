# SETUP_NOTES.md

This file documents one-time setup steps when pulling this repository onto a new
development machine, or after significant toolchain changes.

Claude Code reads this file at session start and will surface any items relevant
to the current machine (based on detected OS).

**Mark items complete by appending `[DONE — machine name, date]` after the step.**

---

## All Machines

- Install Node.js LTS: https://nodejs.org
- Install Rust (stable toolchain): https://rustup.rs
  ```
  rustup default stable
  rustup update
  ```
- Install Python 3.11+: https://python.org (required for depth estimation AI pipeline)
- Run `npm install` from repo root
- Run `cargo build` from `src-tauri/` to fetch and compile Rust dependencies
- Set up Python virtual environment for ML pipeline:
  ```
  cd python
  python -m venv .venv
  source .venv/bin/activate       # macOS / Linux
  .venv\Scripts\activate          # Windows
  pip install -r requirements.txt
  ```
- Run `npm run dev` to verify the Tauri dev server starts
- Run `npm test` to verify the Vitest test suite passes
- Run `cd src-tauri && cargo test` to verify Rust unit tests pass

---

## macOS Only

- Install Xcode Command Line Tools (required for Rust compilation):
  ```
  xcode-select --install
  ```
- Install Tauri prerequisites as per https://tauri.app/start/prerequisites/#macos
- For macOS app signing and notarisation, Apple Developer account must be linked
  in Xcode > Settings > Accounts
- Verify with `cargo tauri info` that all Tauri dependencies are satisfied

---

## Windows Only

- Enable Developer Mode in Windows Settings
- Install Microsoft C++ Build Tools (required for Rust on Windows):
  https://visualstudio.microsoft.com/visual-cpp-build-tools/
  Select: "Desktop development with C++"
- Install WebView2 (usually pre-installed on Windows 11):
  https://developer.microsoft.com/en-us/microsoft-edge/webview2/
- Install Tauri prerequisites as per https://tauri.app/start/prerequisites/#windows
- Verify with `cargo tauri info` that all Tauri dependencies are satisfied

---

## Linux Only

- Install system dependencies for Tauri on Linux:
  ```
  sudo apt-get update
  sudo apt-get install libwebkit2gtk-4.1-dev libssl-dev libayatana-appindicator3-dev \
    librsvg2-dev patchelf build-essential curl wget file libxdo-dev libgtk-3-dev
  ```
- Install Tauri prerequisites as per https://tauri.app/start/prerequisites/#linux
- Verify with `cargo tauri info` that all Tauri dependencies are satisfied

---

## Python ML Pipeline Notes

- Depth estimation models (Depth-Anything-V2 / MiDaS) are large and not committed.
  Download as per `RESEARCH/python-ml.md` instructions.
- Model weights should be placed in `python/models/` (gitignored).
- CUDA is optional; CPU inference works but is slower.
- See `RESEARCH/python-ml.md` for PyTorch version requirements.

---

*Add project-specific setup steps below this line as the project evolves.*
