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
  **Linux:** if `python -m venv` fails with `ensurepip is not available`, install the venv package first:
  `sudo apt-get install -y python3-venv` (or `python3.12-venv` on Ubuntu 24.04).
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

- Install system libraries required by Tauri (Ubuntu/Debian):
  ```
  sudo apt-get update && sudo apt-get install -y \
    build-essential \
    python3-venv \
    libgtk-3-dev \
    libglib2.0-dev \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    libsoup-3.0-dev \
    pkg-config \
    libssl-dev
  ```
  **Why `libsoup-3.0-dev`?** Tauri's Linux build links against WebKit/GTK stacks that use libsoup 3. If it is missing, `cargo test` / `cargo build` fails with:
  `The system library libsoup-3.0 required by crate soup3-sys was not found.`
  (`libwebkit2gtk-4.1-dev` pulls this in as a dependency, but listing it explicitly makes the fix obvious.)

- Verify all pkg-config probes (every line must print `OK`):
  ```
  for pkg in gtk+-3.0 webkit2gtk-4.1 libsoup-3.0 librsvg-2.0; do
    printf '%-20s ' "$pkg"
    pkg-config --exists "$pkg" && echo OK || echo MISSING
  done
  ```
  Quick single check: `pkg-config --libs gtk+-3.0`

- Verify Rust build after system libs are installed:
  ```
  cargo test --manifest-path src-tauri/Cargo.toml
  ```

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
