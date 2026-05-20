# ARCH-D-02 — Tauri command ↔ frontend ↔ ACL audit

**Date:** 2026-05-19  
**Sprint:** D — Validate and Harden  
**Auditor:** System Architect (ARCH-D-02); orchestrator integration pass  
**Scope:** `src-tauri/src/lib.rs`, `src/lib/tauri.ts`, `src-tauri/permissions/*.toml`, `src-tauri/capabilities/default.json`

---

## Executive summary

| Metric | Count |
|--------|------:|
| Registered Tauri commands | 36 |
| Commands in permission `allow` lists (union) | 36 |
| Custom permission identifiers in `default.json` | 6 |
| Frontend `invoke` wrappers in `tauri.ts` | 36 |
| Structural orphans | **0** |
| Policy gaps (SEC-D-01) | **1** (coarse `allow-generate-depth-map`) |

**ARCH-D-02 acceptance:** No orphan commands between registration, ACL, and frontend. Remaining ACL work is **SEC-D-01** (split depth/mask permission group), not missing permissions.

---

## Command audit table

| Command name | Registered in lib.rs | In tauri.ts | Permission file | In default.json | Status |
|--------------|---------------------|-------------|-----------------|-----------------|--------|
| `load_image` | Yes | `loadImage` | `allow-load-image.toml` | Yes | OK |
| `generate_depth_map` | Yes | `generateDepthMap` | `allow-generate-depth-map.toml` | Yes | OK (coarse group) |
| `get_depth_map` | Yes | `getDepthMap` | `allow-generate-depth-map.toml` | Yes | OK |
| `get_depth_histogram` | Yes | `getDepthHistogram` | `allow-generate-depth-map.toml` | Yes | OK |
| `set_depth_adjustment_params` | Yes | `setDepthAdjustmentParams` | `allow-generate-depth-map.toml` | Yes | OK |
| `get_depth_adjustment_params` | Yes | `getDepthAdjustmentParams` | `allow-generate-depth-map.toml` | Yes | OK |
| `get_undo_redo_state` | Yes | `getUndoRedoState` | `allow-generate-depth-map.toml` | Yes | OK |
| `undo` | Yes | `undo` | `allow-generate-depth-map.toml` | Yes | OK |
| `redo` | Yes | `redo` | `allow-generate-depth-map.toml` | Yes | OK |
| `clear_history` | Yes | `clearHistory` | `allow-generate-depth-map.toml` | Yes | OK |
| `reset_depth_adjustments` | Yes | `resetDepthAdjustments` | `allow-generate-depth-map.toml` | Yes | OK |
| `get_mask` | Yes | `getMask` | `allow-generate-depth-map.toml` | Yes | OK |
| `set_mask_region` | Yes | `setMaskRegion` | `allow-generate-depth-map.toml` | Yes | OK |
| `set_mask` | Yes | `setMask` | `allow-generate-depth-map.toml` | Yes | OK |
| `clear_mask` | Yes | `clearMask` | `allow-generate-depth-map.toml` | Yes | OK |
| `save_mask_to_path` | Yes | `saveMaskToPath` | `allow-generate-depth-map.toml` | Yes | OK |
| `load_mask_from_path` | Yes | `loadMaskFromPath` | `allow-generate-depth-map.toml` | Yes | OK |
| `get_settings` | Yes | `getSettings` | `allow-settings.toml` | Yes | OK |
| `save_settings` | Yes | `saveSettings` | `allow-settings.toml` | Yes | OK |
| `set_blank_envelope` | Yes | `setBlankEnvelope` | `allow-point-cloud.toml` | Yes | OK |
| `set_volumetric_params` | Yes | `setVolumetricParams` | `allow-point-cloud.toml` | Yes | OK |
| `set_point_cloud_format` | Yes | `setPointCloudFormat` | `allow-point-cloud.toml` | Yes | OK |
| `estimate_point_cloud_count` | Yes | `estimatePointCloudCount` | `allow-point-cloud.toml` | Yes | OK |
| `generate_point_cloud` | Yes | `generatePointCloud` | `allow-point-cloud.toml` | Yes | OK |
| `export_ply` | Yes | `exportPly` | `allow-point-cloud.toml` | Yes | OK |
| `export_xyz` | Yes | `exportXyz` | `allow-point-cloud.toml` | Yes | OK |
| `export_csv` | Yes | `exportCsv` | `allow-point-cloud.toml` | Yes | OK |
| `save_preset` | Yes | `savePreset` | `allow-presets.toml` | Yes | OK |
| `load_preset` | Yes | `loadPreset` | `allow-presets.toml` | Yes | OK |
| `list_presets` | Yes | `listPresets` | `allow-presets.toml` | Yes | OK |
| `delete_preset` | Yes | `deletePreset` | `allow-presets.toml` | Yes | OK |
| `rename_preset` | Yes | `renamePreset` | `allow-presets.toml` | Yes | OK |
| `list_builtin_presets` | Yes | `listBuiltinPresets` | `allow-presets.toml` | Yes | OK |
| `check_model` | Yes | `checkModel` | `allow-model.toml` | Yes | OK |
| `get_model_info` | Yes | `getModelInfo` | `allow-model.toml` | Yes | OK |
| `download_model` | Yes | `downloadModel` | `allow-model.toml` | Yes | OK |

**Non-command capabilities in `default.json`:** `core:default`, `shell:allow-open`, `dialog:allow-open`, `dialog:allow-save` (plugins; not IPC commands).

---

## Orphan checks

| Check | Result |
|--------|--------|
| Rust command registered, not in any permission TOML | None |
| Permission TOML command, not in `generate_handler!` | None |
| Permission ID in `default.json` without TOML | None |
| TOML not referenced in `default.json` | None |
| Frontend `invoke` without Rust command | None |
| Rust command without `tauri.ts` wrapper | None |

**Retired 2.5D:** No `export_stl`, `export_obj`, or `get_mesh_data` in live code. Generated schema under `src-tauri/gen/schemas/` may still mention legacy `allow-export-stl` — regenerate on next `tauri build` if drift matters.

---

## SEC-D-01 recommendations

`allow-generate-depth-map.toml` bundles **16 commands**, including high-risk `generate_depth_map` (Python subprocess) with read-only and in-memory APIs.

Suggested split:

1. **`allow-depth-inference.toml`** — `generate_depth_map` only
2. **`allow-depth-read.toml`** — `get_depth_map`, `get_depth_histogram`
3. **`allow-depth-adjust.toml`** — adjustment + undo/redo commands
4. **`allow-mask.toml`** — mask get/set/save/load commands

Keep `allow-load-image`, `allow-point-cloud`, `allow-settings`, `allow-presets`, `allow-model` as-is. Update `default.json` to reference new permission IDs.

Do **not** split point-cloud export from generation without product input — exports depend on cached `generate_point_cloud` output.

---

## Verification checklist linkage

- **SEC-D-06:** Satisfied — settings and model commands in dedicated TOMLs and `default.json`
- **ARCH-D-02:** Structural audit complete; SEC-D-01 is follow-up least-privilege work

*No code changes in this audit document.*
