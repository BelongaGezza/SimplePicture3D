# SETUP_NOTES.md

This file documents one-time setup steps that must be performed when pulling this
repository onto a new development machine, or after significant toolchain changes.

Claude Code reads this file at session start and will surface any items relevant
to the current machine.

**Mark items complete by appending `[DONE — machine name, date]` after the step.**

---

## All Machines

- Install **Rust** (stable channel via rustup): https://rustup.rs/
  - Minimum version: 1.70+; run `rustup default stable`
  - Verify: `rustc --version && cargo --version`
- Install **Node.js** LTS (18+): https://nodejs.org/
  - npm 9+ bundled; verify with `node --version && npm --version`
- Install **Python** 3.10+: https://www.python.org/
  - Required for AI depth estimation backend
  - Verify: `python --version` (or `python3 --version`)
- Configure Git identity: `git config --global user.name "Your Name"` etc.
- Clone repo and install frontend deps: `npm install`
- Set up Python virtual environment from repo root:
  ```
  python -m venv python/venv
  # Windows: python\venv\Scripts\activate
  # macOS/Linux: source python/venv/bin/activate
  pip install -r python/requirements.txt
  ```
- Verify setup: `cargo test --manifest-path src-tauri/Cargo.toml && npm test`
- Run dev server: `npm run tauri dev`

---

## Windows Only

- Enable **Developer Mode** in Windows Settings → Privacy & Security → For developers
- Install **Visual Studio Build Tools** with "Desktop development with C++" workload
  (required by Rust for Windows targets)
  - Or install the full Visual Studio Community edition
  - Verify Rust sees the MSVC toolchain: `rustup show`
- If `cargo build` in `src-tauri` fails with RC2176 "old DIB" on `icon.ico`, run:
  `npm run tauri icon path/to/1024x1024.png` then rebuild (see `RESEARCH/GOTCHAS.md`)
- Note: Tauri on Windows may not see the same PATH as your terminal. Install Python
  "for all users" or add to the system PATH (not just user PATH).

---

## macOS Only (Phase 3 — Q4 2026 target)

- Install **Xcode Command Line Tools**: `xcode-select --install`
  (required for Rust compiler on macOS; also enables `xcodebuild` for signing)
- Add Apple Silicon target for universal builds:
  `rustup target add aarch64-apple-darwin`
- Add Intel target if needed:
  `rustup target add x86_64-apple-darwin`
- Verify Apple Developer account signing config before building signed release packages
- Build macOS Tauri app: `npm run tauri build -- --target universal-apple-darwin`

---

## Linux Only (Phase 3 — Q4 2026 target)

- Install system libraries required by Tauri:
  ```
  sudo apt-get update && sudo apt-get install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    pkg-config \
    libssl-dev
  ```
- Verify: `pkg-config --libs gtk+-3.0`
- Build Linux Tauri app: `npm run tauri build`
  - Output: AppImage and .deb in `src-tauri/target/release/bundle/`

---

## Python AI Backend (all machines)

- Stub mode (no model download required): `SP3D_USE_STUB=1`
- First real inference run downloads Depth-Anything-V2 from Hugging Face (~200MB)
- For GPU acceleration, install PyTorch with CUDA/Metal support per:
  https://pytorch.org/get-started/locally/
- See `python/README.md` for model paths and `--no-model` usage

---

*Add new setup steps below this line as the project evolves.*
