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

### 2026-02-08 — Tauri v2 — NPM vs Rust version mismatch
**Symptom:** `npm run tauri dev` fails with "Error Found version mismatched Tauri packages. Make sure the NPM package and Rust crate versions are on the same major/minor releases: tauri (v2.9.5) : @tauri-apps/api (v2.10.1)".  
**Cause:** Rust Cargo.toml uses `tauri = "2"` (resolved to 2.9.5 in Cargo.lock) while package.json had `@tauri-apps/api: "^2.0.0"`, which npm resolved to latest 2.x (2.10.1). Tauri requires the same major/minor on both sides.  
**Fix:** Pin Tauri NPM packages to the same minor as the Rust crate. In package.json: set `@tauri-apps/api` and `@tauri-apps/cli` to `~2.9.0` (or whatever minor the Rust tauri crate uses). Do **not** use `~2.9.0` for `@tauri-apps/plugin-dialog` — that package does not publish 2.9.x (latest is 2.6.x); keep it at `^2.0.0`. After upgrading Rust Tauri to a new minor (e.g. 2.10), bump the NPM packages to the same minor (e.g. `~2.10.0`).

### 2026-02-07 — Svelte 4 / @testing-library/svelte — Build fails with "Component not assignable to Constructor<SvelteComponent>"
**Symptom:** Frontend CI fails at Build step (`tsc && vite build`) with 10 TypeScript errors in `DepthControls.test.ts` and `ImageImport.test.ts`: "Argument of type 'typeof Component' is not assignable to parameter of type 'ComponentImport<SvelteComponent<any, any, any>>'" (or `Constructor<SvelteComponent<...>>`).  
**Causes:** (1) **Technologies have moved forward:** @testing-library/svelte **v5** is designed for **Svelte 5** (new component signature); this codebase uses **Svelte 4** (ADR-001), so types no longer match. (2) Optionally: project's `src/vite-env.d.ts` declared `*.svelte` with a minimal `Component` class that did not extend Svelte's `SvelteComponent`, contributing to type-check failure.  
**Fix (Option A — recommended):** Pin `@testing-library/svelte` to **^4.2.0** in package.json (last major compatible with Svelte 4). If needed, align `src/vite-env.d.ts` with Svelte's types (e.g. `ComponentType` from `svelte`). One-line dependency change; no Svelte 5 migration.  
**Fix (Option B):** Upgrade to Svelte 5 (`svelte` ^5.0.0, `@sveltejs/vite-plugin-svelte` ^4.0.0) and keep @testing-library/svelte v5; larger migration (runes, component API). See RESEARCH/frontend.md for version alignment.

### 2026-02-06 — Rust — Depth adjustment benchmark (1920×1080) (JR2-403)
**Symptom:** Need to confirm real-time preview is feasible for 1080p depth.  
**Cause:** prd §7.1 / UI-404 target: preview update within 100 ms.  
**Fix:** Benchmark in `src-tauri/benches/depth_adjust.rs`. Run `cargo bench --bench depth_adjust` from src-tauri. Result (release build): **~14 ms** per `apply_adjustments` call for 1920×1080 (2.07M samples). Well under 100 ms; real-time debounced preview is feasible.

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

### 2026-02-02 — Tauri v2 (macOS) — missing icon.png
**Symptom:** `cargo build` in src-tauri fails with "failed to open icon ... icon.png: No such file or directory".  
**Cause:** On macOS, Tauri’s build looks for `src-tauri/icons/icon.png`; the repo may only have `icon.ico` (Windows).  
**Fix:** Run `npm run tauri icon /path/to/1024x1024.png` to generate all platform icons (including icon.png). Or copy any valid PNG (e.g. 32×32 or larger) to `src-tauri/icons/icon.png`. See [docs/setting_up_your_Mac.md](../docs/setting_up_your_Mac.md).

### 2026-02-01 — Tauri v2 — App command permissions not found
**Symptom:** Build fails with "Permission allow-load-image not found" (or "identifiers can only include lowercase ASCII, hyphens...").  
**Cause:** Tauri v2 does not auto-generate permissions for app commands; capability identifiers must use kebab-case (hyphens), not underscores; app permissions must be defined in `src-tauri/permissions/`.  
**Fix:** (1) Create `src-tauri/permissions/allow-load-image.toml` (and similar) with `[[permission]] identifier = "allow-load-image"` and `commands.allow = ["load_image"]`. (2) In capabilities/default.json use `"allow-load-image"`, `"allow-export-stl"` (kebab-case), not `allow-load_image`.
