# Tauri Framework

**Purpose:** Tauri v1/v2 guidance for SimplePicture3D desktop shell and IPC.

## Official Sources

| Source | URL | Last Checked |
|--------|-----|--------------|
| Tauri Docs | https://tauri.app/v1/guides/ | — |
| Tauri v2 Docs | https://v2.tauri.app/ | — |
| GitHub | https://github.com/tauri-apps/tauri | — |
| API Reference | https://tauri.app/v1/api/js/ | — |

*Researcher: Update "Last Checked" when verifying these sources.*

---

## Project Usage

- **Shell:** Tauri wraps frontend (Svelte/React) in native window
- **IPC:** `invoke()` from frontend calls Rust commands
- **Commands:** Defined in `src-tauri/src/lib.rs` or `main.rs`, exposed via `tauri::generate_handler![]`

---

## Key Concepts

- **Commands:** `#[tauri::command]` async fn → callable from JS via `invoke('command_name', { args })`
- **State:** `manage()` for shared Rust state across commands
- **Permissions:** Tauri v2 uses capability system; v1 uses `tauri.conf.json` allowlist
- **Assets:** Frontend build output served from `dist/` or configured path

---

## Research Tasks (Researcher)

- [ ] Confirm Tauri v1 vs v2 for this project
- [ ] Document IPC serialization limits (large depth maps, mesh data)
- [ ] Subprocess spawning from Tauri (Python) — any sandbox restrictions
- [ ] Version changes and deprecations since last check
