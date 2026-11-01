# ARCH-D-02: Command ↔ ACL ↔ Frontend Audit

**Date:** 2026-05-24  
**Scope:** All Tauri commands registered in `src-tauri/src/lib.rs` `invoke_handler`  
**Related:** SEC-D-01 (permission split)

## Summary

| Metric | Result |
|--------|--------|
| Commands in `invoke_handler` | 36 |
| Commands in exactly one permission `.toml` | 36 / 36 |
| User-facing commands with `tauri.ts` wrapper | 35 / 36 |
| Mismatches fixed | 0 (audit clean after SEC-D-01 split) |

**Permission files (7):** `allow-load-image`, `allow-depth-map`, `allow-mask`, `allow-point-cloud`, `allow-settings`, `allow-presets`, `allow-model`.

**SEC-D-01 change:** Replaced monolithic `allow-generate-depth-map` with `allow-depth-map` (depth pipeline + undo/redo) and `allow-mask` (mask I/O). Removed `allow-generate-depth-map.toml`.

## Command audit table

| Command | Permission file | `tauri.ts` wrapper | Notes |
|---------|-----------------|-------------------|-------|
| `load_image` | `allow-load-image.toml` | `loadImage()` | |
| `generate_depth_map` | `allow-depth-map.toml` | `generateDepthMap()` | Progress via `depth-progress` event |
| `get_depth_map` | `allow-depth-map.toml` | `getDepthMap()` | |
| `get_depth_histogram` | `allow-depth-map.toml` | `getDepthHistogram()` | |
| `set_depth_adjustment_params` | `allow-depth-map.toml` | `setDepthAdjustmentParams()` | |
| `get_depth_adjustment_params` | `allow-depth-map.toml` | `getDepthAdjustmentParams()` | |
| `get_undo_redo_state` | `allow-depth-map.toml` | `getUndoRedoState()` | |
| `undo` | `allow-depth-map.toml` | `undo()` | |
| `redo` | `allow-depth-map.toml` | `redo()` | |
| `clear_history` | `allow-depth-map.toml` | `clearHistory()` | |
| `reset_depth_adjustments` | `allow-depth-map.toml` | `resetDepthAdjustments()` | |
| `get_mask` | `allow-mask.toml` | `getMask()` | |
| `set_mask_region` | `allow-mask.toml` | `setMaskRegion()` | |
| `set_mask` | `allow-mask.toml` | `setMask()` | |
| `clear_mask` | `allow-mask.toml` | `clearMask()` | |
| `save_mask_to_path` | `allow-mask.toml` | `saveMaskToPath()` | |
| `load_mask_from_path` | `allow-mask.toml` | `loadMaskFromPath()` | |
| `get_settings` | `allow-settings.toml` | `getSettings()` | |
| `save_settings` | `allow-settings.toml` | `saveSettings()` | |
| `set_blank_envelope` | `allow-point-cloud.toml` | `setBlankEnvelope()` | ADR-012 |
| `set_volumetric_params` | `allow-point-cloud.toml` | `setVolumetricParams()` | ADR-012 |
| `set_point_cloud_format` | `allow-point-cloud.toml` | `setPointCloudFormat()` | |
| `estimate_point_cloud_count` | `allow-point-cloud.toml` | `estimatePointCloudCount()` | |
| `generate_point_cloud` | `allow-point-cloud.toml` | `generatePointCloud()` | ADR-012 |
| `export_ply` | `allow-point-cloud.toml` | `exportPly()` | |
| `export_xyz` | `allow-point-cloud.toml` | `exportXyz()` | |
| `export_csv` | `allow-point-cloud.toml` | `exportCsv()` | |
| `save_preset` | `allow-presets.toml` | `savePreset()` | |
| `load_preset` | `allow-presets.toml` | `loadPreset()` | |
| `list_presets` | `allow-presets.toml` | via `listPresets()` | No standalone export |
| `delete_preset` | `allow-presets.toml` | `deletePreset()` | |
| `rename_preset` | `allow-presets.toml` | `renamePreset()` | |
| `list_builtin_presets` | `allow-presets.toml` | `listBuiltinPresets()` / `listPresets()` | |
| `check_model` | `allow-model.toml` | `checkModel()` | |
| `get_model_info` | `allow-model.toml` | `getModelInfo()` | |
| `download_model` | `allow-model.toml` | `downloadModel()` | |

## Capability wiring

`src-tauri/capabilities/default.json` grants all application permission identifiers to the `main` window:

- `allow-load-image`, `allow-depth-map`, `allow-mask`, `allow-point-cloud`, `allow-presets`, `allow-settings`, `allow-model`

## Frontend invoke policy

All `invoke()` calls are centralised in `src/lib/tauri.ts`. `App.svelte` uses `@tauri-apps/api/event` only (`listen` for `depth-progress`). No direct `invoke` bypass of the helper module.

## Findings

1. **ACL coverage:** Complete — every registered command appears in exactly one permission file.
2. **Frontend coverage:** Complete for user-facing workflows. `list_presets` is intentionally internal to `listPresets()`.
3. **No code fixes required** beyond SEC-D-01 permission split and capability update.

## Verification

```bash
cargo build --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```
