# QA-D-03 — Pentest checklist automation map

**Manual checklist:** `SECURITY_PENTEST_CHECKLIST.md`  
**Automated tests:** `cargo test qa_d03 --manifest-path src-tauri/Cargo.toml`  
**Record manual-only results in:** `MANUAL_TEST_REPORT.md` § QA-D-03

| # | Checklist item | Automated | Test / module |
|---|----------------|-----------|---------------|
| 1 | Empty path (`load_image`, export) | Yes | `load_image_rejects_empty_path`, `qa_d03_export_empty_path_rejected` |
| 2 | Nonexistent path | Yes | `generate_depth_map_rejects_nonexistent_path` |
| 3 | Path traversal | Partial | `load_image_impl_path_traversal_resolved_by_canonicalize`, `file_io::sanitize_rejects_path_traversal` |
| 4 | Oversized mask IPC (`set_mask` length) | Yes | `qa_d03_set_mask_wrong_buffer_length_rejected`, `qa_d03_set_mask_dimension_mismatch_rejected` |
| 5 | Huge mask / point cloud | Yes | `qa_d03_point_cloud_generation_cap_enforced` (`MAX_POINT_CLOUD_GENERATION`) |
| 6 | Invalid preset JSON | Yes | `qa_d03_invalid_preset_json_rejected`, `preset::parse_and_validate_json_*` |
| 7 | Absolute preset path | Yes | `qa_d03_preset_import_blocklisted_path_rejected`, `path_security` tests |
| 8 | Export to system dir | Yes | `qa_d03_export_blocklisted_system_path_rejected`, `export` blocklist |
| 9 | Export symlink parent | Manual | Lab: symlinked parent dir |
| 10 | Load blocklisted path | Yes | `load_image_impl_rejects_blocklisted_system_path` |
| 11 | Download with hashes | Manual | Requires SEC-D-02 real digests |
| 12 | Tampered model file | Manual | Lab with installed model |
| 13 | Env hijack | Manual | Launch with altered `VIRTUAL_ENV` |
| 14–15 | Settings/model invoke | Manual | UI / release build |
| 16 | Global Tauri | Manual | Release build: `window.__TAURI__` |
| 17 | Large image file | Yes | `image_loading` max bytes tests |
| 18 | 8K depth workflow | Partial | `ipc_depth_subsample_caps_4k_payload`; full E2E manual |

**Sign-off:** Automated section does not replace manual WebView, subprocess, or engraver tests.
