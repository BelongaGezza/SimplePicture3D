# Setting Up Your Mac for SimplePicture3D Development

This guide covers the developer tools and libraries needed to build and run SimplePicture3D on a Mac (including **Apple Silicon M1/M2/M3**). It is based on a review of the codebase and verification on a MacBook Pro M1 Pro running macOS 26.x (Tahoe).

---

## 1. Required Tools and Versions

| Tool | Minimum version | Purpose |
|------|-----------------|---------|
| **Xcode Command Line Tools** (or full Xcode) | Latest stable | C/C++ linker (`clang`), system headers; required for Rust and Tauri native build on macOS |
| **Rust** (rustc, cargo) | 1.70+ | Backend (Tauri, mesh, file I/O). Use `rustup` and `stable` toolchain |
| **Node.js** | 18+ (LTS 20 recommended) | Frontend (Vite, Svelte), Tauri CLI via npm |
| **npm** | 9+ | Bundled with Node.js; installs frontend and Tauri CLI |
| **Python** | 3.9+ (3.10+ for PyTorch) | Optional until `python/requirements.txt` exists; needed for AI depth pipeline (PyTorch, Depth-Anything-V2) |
| **Git** | 2.x | Clone, branches, contribution |

---

## 2. Install Steps (macOS)

### 2.1 Xcode Command Line Tools (required for Rust/Tauri)

Tauri’s native build on macOS needs a C compiler and system libraries. Either install **Xcode** from the App Store or only the **Command Line Tools**:

```bash
xcode-select --install
```

If you use full **Xcode** (e.g. for iOS or code signing), open it once after install and accept the license. Ensure the active developer directory is set:

```bash
xcode-select -p
# Should show: /Applications/Xcode.app/Contents/Developer
```

### 2.2 Rust (rustup)

Install Rust with the official installer (use `stable` for this project):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose the default installation. Restart your terminal, then:

```bash
rustc --version
cargo --version
rustup default stable
```

### 2.3 Node.js and npm

- **Option A (recommended):** Install **Node.js LTS** (e.g. 20.x) from [nodejs.org](https://nodejs.org/). This includes `npm`.
- **Option B:** Install via **Homebrew**: `brew install node` (gives you both Node and npm).

Check:

```bash
node --version   # v18.x or v20.x LTS recommended
npm --version    # 9+
```

### 2.4 Python (optional until AI pipeline is added)

Required when the project has a `python/` app and `python/requirements.txt`. For PyTorch and depth models, use **Python 3.10+** (see [RESEARCH/python-ml.md](../RESEARCH/python-ml.md)).

- **Option A:** Install from [python.org](https://www.python.org/downloads/).
- **Option B:** Homebrew: `brew install python@3.12`.

Check:

```bash
python3 --version   # 3.9+ (3.10+ for PyTorch)
```

When `python/requirements.txt` exists, use a venv:

```bash
cd python
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

### 2.5 Git

Usually included with Xcode Command Line Tools. If not:

```bash
brew install git
# or download from https://git-scm.com/
git --version
```

### 2.6 Homebrew (optional but useful)

Not required for SimplePicture3D, but convenient for installing Node, Python, and other CLI tools:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

---

## 3. Project Setup (after cloning)

From the repository root:

```bash
# 1. Frontend dependencies (see note below on npm peer dependency)
npm install --legacy-peer-deps

# 2. Rust backend
cd src-tauri && cargo build && cd ..

# 3. (Optional) Python venv when python/requirements.txt exists
# cd python && python3 -m venv venv && source venv/bin/activate && pip install -r requirements.txt && cd ..
```

**npm dependency note:** As of the last check, `@sveltejs/vite-plugin-svelte@4` has a peer dependency on Svelte 5 while the project pins Svelte 4. Use `npm install --legacy-peer-deps` until the project upgrades Svelte or pins a compatible plugin version.

**macOS icon:** On macOS, the Tauri build looks for `src-tauri/icons/icon.png`. If `cargo build` in `src-tauri` fails with “failed to open icon ... icon.png”, add an app icon and generate all icons (including `icon.png`) with:

```bash
npm run tauri icon /path/to/your/1024x1024.png
```

See [RESEARCH/GOTCHAS.md](../RESEARCH/GOTCHAS.md) for the “Tauri v2 (macOS) — missing icon.png” entry.

---

## 4. Verify Your Setup

Run from the project root:

```bash
# Rust
rustc --version
cargo --version
cargo build --manifest-path src-tauri/Cargo.toml

# Node / frontend
node --version
npm --version
npm run build

# Full app (Vite dev server + Tauri window)
npm run tauri dev
```

If `cargo build` fails with the icon error above, fix the icon first. If `npm run build` fails, ensure you used `npm install --legacy-peer-deps` and that any existing TypeScript/Svelte issues in the repo are addressed.

---

## 5. Optional: Linting and Security

- **Rust:** `cargo fmt`, `cargo clippy --manifest-path src-tauri/Cargo.toml`, `cargo test --manifest-path src-tauri/Cargo.toml`, `cargo audit --manifest-path src-tauri/Cargo.toml`
- **Frontend:** `npm run lint` (when configured)
- **Python:** `pytest`, `pip-audit` (when `python/` has tests and requirements)

See [CLAUDE.md](../CLAUDE.md) and [CONTRIBUTING.md](../CONTRIBUTING.md) for the full command list.

---

## 6. Verified on This Mac (M1 Pro, macOS 26.2)

| Item | Result |
|------|--------|
| **Architecture** | `arm64` (Apple Silicon) |
| **macOS** | 26.2 (Tahoe), Build 25C56 |
| **Xcode** | 26.2 (Build 17C52); `xcode-select -p` → `/Applications/Xcode.app/Contents/Developer` |
| **Git** | 2.50.1 (Apple Git-155) |
| **Node** | v25.1.0 (Homebrew); LTS 20 is recommended for CI parity |
| **npm** | 11.6.2 |
| **Rust** | rustc 1.92.0, cargo 1.92.0 (rustup stable) |
| **Python** | 3.12.1 (system/Library framework) |
| **npm install** | Succeeds with `npm install --legacy-peer-deps` (peer dependency conflict Svelte 4 vs plugin expecting Svelte 5) |
| **cargo build (src-tauri)** | Succeeds once `icon.png` is present in `src-tauri/icons/` (run `npm run tauri icon /path/to/1024x1024.png` if missing). |
| **npm run build** | May fail on existing TypeScript/Svelte errors (e.g. `InvokeArgs` types, `App.svelte` module resolution); these are codebase issues, not Mac setup. |
| **npm run tauri dev** | Not fully verified here because frontend build has TS/Svelte errors; once those are fixed, this command should run. (Rust build succeeds with icon.png in place.) |

**Summary:** Install Xcode (or CLT), Rust (rustup), Node.js 18+ (LTS 20 recommended), and optionally Python 3.10+. Use `npm install --legacy-peer-deps` and ensure `src-tauri/icons/icon.png` exists (via `npm run tauri icon ...`) so that `cargo build` and `npm run tauri dev` can succeed on this Mac.

---

## 7. References

- [README.md](../README.md) — Quick start and development setup
- [CONTRIBUTING.md](../CONTRIBUTING.md) — Prerequisites, coding standards, testing
- [RESEARCH/tauri.md](../RESEARCH/tauri.md) — Tauri v2 and shell/sidecar
- [RESEARCH/python-ml.md](../RESEARCH/python-ml.md) — Python, PyTorch, depth models
- [RESEARCH/GOTCHAS.md](../RESEARCH/GOTCHAS.md) — Known issues (Windows icon.ico, macOS icon.png, etc.)
- [Tauri v2 Prerequisites](https://v2.tauri.app/start/prerequisites/) — Official macOS prerequisites
