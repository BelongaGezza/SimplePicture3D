# GOTCHAS — Debugging Findings

**Purpose:** Record non-obvious bugs, workarounds, and pitfalls discovered during development. All agents contribute.

## How to Add an Entry

When you hit a debugging gotcha:

1. Add a dated entry below
2. Include: technology, symptom, cause, fix
3. Keep it brief; link to issues or docs if relevant

```markdown
### YYYY-MM-DD — [Technology] — Brief title
**Symptom:** What went wrong  
**Cause:** Why it happened  
**Fix:** What worked
```

---

## Entries

### 2026-02-01 — Tauri v2 — IPC large payloads slow
**Symptom:** Sending large depth maps or mesh data (e.g. 3MB+) over Tauri IPC can be slow (e.g. 200ms+ for 3MB).  
**Cause:** Command IPC uses JSON-RPC serialization; Rust → frontend events require serialization.  
**Fix:** Prefer file-based or chunked transfer: Rust writes to temp file or uses Commands that accept array buffers from frontend; avoid sending multi‑MB JSON over invoke for real-time preview.

### 2026-02-01 — Tauri v2 — Subprocess: use shell plugin, not Process
**Symptom:** Need to spawn Python (or other child process) from Rust.  
**Cause:** The Process plugin is for the current process only.  
**Fix:** Use the **shell plugin** (`tauri-plugin-shell`); for bundled Python use **sidecar** with `externalBin` in `tauri.conf.json` and `app.shell().sidecar("name").spawn()`. Grant `shell:allow-spawn` or `shell:allow-execute` in capabilities.

### 2026-02-01 — MiDaS — Repo archived
**Symptom:** Checking MiDaS for updates or issues.  
**Cause:** Repository isl-org/MiDaS was archived (read-only) in 2025.  
**Fix:** Code and weights remain usable; prefer Depth-Anything-V2 for new work; use MiDaS only if already integrated or required.

### 2026-02-01 — Tauri v2 (Windows) — RC2176 "old DIB" on icon.ico
**Symptom:** `cargo build` in src-tauri fails with "error RC2176 : old DIB in ... icons/icon.ico; pass it through SDKPAINT".  
**Cause:** Windows Resource Compiler (rc.exe) rejects ICO files that use PNG-compressed or non-standard DIB format.  
**Fix:** (1) **In this repo:** run `python scripts/gen_icon_win.py` to generate a BMP-based icon.ico in src-tauri/icons/. (2) Alternatively: `npm run tauri icon path/to/1024x1024.png` (Tauri generates icons in src-tauri/icons/). Or open/save the .ico in GIMP as "Windows Icon" without PNG compression.

### 2026-02-01 — Tauri v2 — App command permissions not found
**Symptom:** Build fails with "Permission allow-load-image not found" (or "identifiers can only include lowercase ASCII, hyphens...").  
**Cause:** Tauri v2 does not auto-generate permissions for app commands; capability identifiers must use kebab-case (hyphens), not underscores; app permissions must be defined in `src-tauri/permissions/`.  
**Fix:** (1) Create `src-tauri/permissions/allow-load-image.toml` (and similar) with `[[permission]] identifier = "allow-load-image"` and `commands.allow = ["load_image"]`. (2) In capabilities/default.json use `"allow-load-image"`, `"allow-export-stl"` (kebab-case), not `allow-load_image`.
