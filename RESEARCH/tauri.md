# Tauri Framework

**Purpose:** Tauri v1/v2 guidance for SimplePicture3D desktop shell and IPC.

## Version Recommendation (Feb 2026)

**Use Tauri v2** for new projects.

- **Tauri 2.0** reached stable release (October 2024) and is the current recommended version.
- v2 adds mobile (iOS/Android), improved architecture, and capability-based permissions.
- New projects should use `create-tauri-app` or `tauri init` with CLI `^2.0.0`. Migration from v1 is supported via `tauri migrate`.

## Official Sources

| Source | URL | Last Checked |
|--------|-----|--------------|
| Tauri v2 Docs | https://v2.tauri.app/ | 2026-02-01 |
| Create a Project | https://v2.tauri.app/start/create-project/ | 2026-02-01 |
| IPC (Inter-Process Communication) | https://v2.tauri.app/concept/inter-process-communication | 2026-02-01 |
| Shell plugin (spawn / sidecar) | https://v2.tauri.app/plugin/shell/ | 2026-02-01 |
| Embedding External Binaries (Sidecar) | https://v2.tauri.app/develop/sidecar/ | 2026-02-01 |
| Migrate from Tauri 1 | https://v2.tauri.app/start/migrate/from-tauri-1 | 2026-02-01 |
| GitHub | https://github.com/tauri-apps/tauri | 2026-02-01 |

---

## Project Usage

- **Shell:** Tauri wraps frontend (Svelte/React) in native window.
- **IPC:** Frontend calls Rust via `invoke()`; v2 uses capability-based permissions (e.g. `capabilities/default.json`).
- **Commands:** Defined in Rust, exposed via `tauri::generate_handler![]`; v2 uses `#[tauri::command]` and ACL.

---

## New Project Scaffolding (v2)

- **CLI:** `cargo install tauri-cli --version "^2.0.0" --locked` or `npm create tauri-app@latest` (then choose framework: Svelte, React, etc.).
- **Config:** v2 uses `tauri.conf.json`; bundle options include `externalBin` for sidecars (e.g. Python binary).
- **Capabilities:** Permissions live in `src-tauri/capabilities/default.json` (e.g. `shell:allow-spawn`, `shell:allow-execute` for sidecars).

---

## IPC and Large Payloads

- **Limitation:** Command IPC is JSON-serialized; large payloads (e.g. multi‑MB depth maps or mesh data) can be slow (e.g. 3MB ≈ 200ms in community reports).
- **Recommendation:** Prefer file-based or chunked transfer for large data: e.g. Rust writes depth/mesh to temp file or shared path, frontend requests path or uses a Command that streams/chunks. Frontend → Rust Commands can use array buffers where supported; Rust → frontend events still require serialization.
- **For Python:** Use subprocess/sidecar (see below), not large JSON over IPC.

---

## Subprocess / Python Backend

- **Spawning:** Use the **shell plugin** (`tauri-plugin-shell`). The Process plugin is for the current process only; to run Python (or any child), use shell’s `Command` or **sidecar**.
- **Sidecar:** Bundle a Python binary (e.g. PyInstaller) as a sidecar: add to `tauri.conf.json` under `bundle.externalBin` (e.g. `"binaries/depth-server"`), provide per-target triple (e.g. `depth-server-x86_64-pc-windows-msvc.exe`). In Rust: `app.shell().sidecar("depth-server")?.spawn()`. Configure `shell:allow-spawn` or `shell:allow-execute` in capabilities for that sidecar name.
- **I/O:** Sidecar stdin/stdout/stderr via `CommandEvent` (e.g. `Stdout(line_bytes)`). For large image input, pass temp file path as argument or write to child stdin in chunks; receive depth map via stdout or output file.

---

## Migration Notes (v1 → v2)

- Config layout and capability model differ; run `tauri migrate` and adjust `tauri.conf.json` and `capabilities/default.json`.
- Invoke API and command registration differ slightly; see official migration guide.

---

## Research Tasks (Researcher)

- [x] Confirm Tauri v1 vs v2 for this project → **v2**
- [x] Document IPC serialization limits (large depth maps, mesh data) → file/chunk strategy
- [x] Subprocess spawning from Tauri (Python) → shell plugin, sidecar
- [x] Version and Official Sources with Last checked dates
