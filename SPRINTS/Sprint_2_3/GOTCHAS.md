# Sprint 2.3 — Gotchas Log

**Sprint:** 2.3 — Presets & Templates  
**Purpose:** Sprint-specific debugging and implementation findings. Merge noteworthy items to `RESEARCH/GOTCHAS.md` when sprint closes.

---

## Format

Each entry: **Date — Short title**  
**What happened:** …  
**Fix / workaround:** …

---

## Entries

### 2026-03-02 — Preset UI expects backend commands (UI-1301–1304)
**What happened:** Frontend PresetManager, Save/Load buttons, preset dropdown, and Import/Export call Tauri commands that are not yet registered in the backend (`list_presets`, `save_preset`, `load_preset`, `delete_preset`, `rename_preset`). Until BACK-1302 is complete, these invokes will fail at runtime (empty list or error message in UI).
**Fix / workaround:** Senior Engineer implements BACK-1302 with command names and signatures as defined in `src/lib/tauri.ts` (list_presets → `PresetListItem[]`, save_preset(name, path?), load_preset(name_or_path), delete_preset(name), rename_preset(old_name, new_name)). Preset schema and types align with `src-tauri/src/preset.rs`.

### 2026-03-02 — Preset API snake_case and user-only actions
**What happened:** Rust/Serde expects snake_case for JSON (e.g. `name_or_path`, `old_name`, `new_name`). PresetManager must only allow rename/delete for user presets, not built-ins.
**Fix / workaround:** Frontend invoke uses `name_or_path`, `old_name`, `new_name`. PresetManager filters `list_presets` to `item.kind === "user"` for the list; built-ins appear only in Apply/Load dropdown.

### 2026-03-02 — list_presets returns user names only; frontend merges with built-ins (UI Designer follow-up)
**What happened:** Backend `list_presets` returns `Vec<String>` (user preset names only); `list_builtin_presets` returns built-in ids. Frontend expected a single `PresetListItem[]` with `kind`/`name`/`id`, so dropdown and PresetManager would get raw strings and `item.kind === "user"` would filter out everything.
**Fix / workaround:** In `src/lib/tauri.ts`, `listPresets()` now calls both `list_presets` and `list_builtin_presets`, then builds `PresetListItem[]`: built-ins first (kind: "builtin", name/id from backend ids), then user presets (kind: "user", name/id from names). No backend change required.

### 2026-03-06 — QE verification: fmt and tauri build (Quality Engineer)
**What happened:** `cargo fmt --check` reports diff in `lib.rs` (pre-existing formatting, not Sprint 2.3 code). `npm run tauri build` builds the app binary successfully but bundler step fails with "invalid category, did you mean `Graphics and Design`?" (tauri.conf.json bundle category).
**Fix / workaround:** Run `cargo fmt` to satisfy fmt check. Fix bundle category in tauri.conf.json for installer build; app binary builds and runs.

---

**Document Version:** 1.1  
**Merge to:** `RESEARCH/GOTCHAS.md` (copy any lasting gotchas before sprint close)
