# Security Sign-off -- Sprint 1.8: STL Export

**Reviewer:** Security Specialist (Claude-Code-Security)
**Date:** 2026-02-08
**Tasks:** SEC-401 (File Path Handling), SEC-402 (Export Directory Permissions)
**Status:** PASS with hardening applied

---

## Scope

Review of the STL export pipeline for file path security and permission handling:

| File | Purpose |
|------|---------|
| `src-tauri/src/lib.rs` | `export_stl` Tauri command, `get_export_defaults` command |
| `src-tauri/src/mesh_generator.rs` | `write_stl_to_file()`, `write_stl_binary()`, `generate_export_filename()` |
| `src-tauri/src/settings.rs` | `AppSettings`, persistence at `~/.simplepicture3d/settings.json` |
| `src-tauri/capabilities/default.json` | Tauri IPC permissions |
| `src/lib/tauri.ts` | Frontend IPC wrappers |
| `src/components/ExportPanel.svelte` | Frontend export UI |

---

## SEC-401: File Path Handling

### Findings

#### FINDING-1: Path traversal -- no canonicalization (MEDIUM, FIXED)

**Before:** The `export_stl` command accepted a raw `path: String` from IPC and passed it directly to `std::fs::File::create()` via `write_stl_to_file()`. A malicious IPC call could pass `../../sensitive_file` or `C:\Windows\System32\file.stl`.

**Mitigation:** While the frontend uses Tauri's native save dialog (which returns OS-validated paths), the backend IPC command is the security boundary. A compromised webview or devtools injection could bypass the dialog.

**Fix applied:** Added `std::fs::canonicalize()` on the parent directory to resolve `..`, `.`, and symlinks before writing. The canonical path is used for all subsequent operations.

#### FINDING-2: No extension validation (LOW, FIXED)

**Before:** The command would write to any file extension. While the save dialog filters for `.stl`, a direct IPC call could write to `.exe`, `.dll`, etc.

**Fix applied:** Extension validation added -- rejects any path not ending in `.stl` (case-insensitive).

#### FINDING-3: No system directory protection (MEDIUM, FIXED)

**Before:** No check against writing to system directories like `C:\Windows`, `/etc`, `/usr`.

**Fix applied:** Added platform-specific blocked directory lists:
- **Windows:** `C:\Windows`, `C:\Program Files`, `C:\Program Files (x86)`, `C:\ProgramData`
- **Unix:** `/etc`, `/usr`, `/bin`, `/sbin`, `/boot`, `/lib`, `/proc`, `/sys`

#### FINDING-4: Symlink attacks (LOW, ACCEPTED)

**Status:** `std::fs::canonicalize()` resolves symlinks, so the system directory check operates on the real target path. A symlink pointing to `C:\Windows` would be resolved and blocked. This mitigates the symlink attack vector.

#### FINDING-5: Null bytes in path (NO ISSUE)

Rust's `std::fs` APIs reject embedded null bytes (returns `Err`). No additional validation needed.

#### FINDING-6: Filename injection in `generate_export_filename` (NO ISSUE)

The function already sanitizes non-alphanumeric characters to underscores (line 507-515 of `mesh_generator.rs`). The sanitized stem is safe for all OS filesystems. Timestamp is computed internally, not from user input.

---

## SEC-402: Export Directory Permissions

### Findings

#### FINDING-7: No permission check before write (LOW, FIXED)

**Before:** `File::create()` was called directly. If the directory was read-only, the OS error would propagate with the full path in the error message.

**Fix applied:** Added a pre-write permission check: creates and immediately removes a test file (`.sp3d_write_test`) in the target directory. Returns a generic "Export directory is not writable" error without exposing the path.

#### FINDING-8: Error messages leak paths (LOW, FIXED)

**Before:** Error messages like `"STL export failed: Cannot create STL file: C:\Users\..."` included full file paths, which were returned to the frontend.

**Fix applied:** STL write error now returns generic message: `"STL export failed: could not write file"`. Other error messages (mesh generation, validation) do not contain path information.

#### FINDING-9: Settings file permissions (INFORMATIONAL, ACCEPTED)

`~/.simplepicture3d/settings.json` is created via `std::fs::write()` with default OS permissions. On Unix this is typically `0644` (owner read/write, group/other read). The file contains only the last export directory path -- no secrets or sensitive data. The risk of another user reading this is minimal for a desktop application.

**Recommendation for future:** Consider using `std::os::unix::fs::OpenOptionsExt` to set `0600` permissions on Unix in a future sprint, if multi-user deployment is expected.

#### FINDING-10: No temp files during export (NO ISSUE)

The STL write goes directly to the target file via `BufWriter`. No temporary files are created during the export process. If the write fails partway, a partial file may remain, but this is standard file I/O behavior and not a security concern.

---

## Tauri IPC Surface Review

### Permissions (capabilities/default.json)

```json
{
    "permissions": [
        "core:default",
        "shell:allow-open",
        "dialog:allow-open",
        "dialog:allow-save",
        "allow-load-image",
        "allow-export-stl",
        "allow-get-export-defaults",
        "allow-generate-depth-map"
    ]
}
```

**Assessment:**
- `shell:allow-open` -- Used for "Open Folder" feature. Opens directory in OS file manager. The path comes from a successful export (server-side validated). Acceptable.
- `dialog:allow-save` -- Required for native save dialog. Dialog is OS-level, returns safe paths. Acceptable.
- `allow-export-stl` -- The command now has server-side path validation (SEC-401 fixes). Acceptable.
- `allow-get-export-defaults` -- Returns suggested filename and last directory. No write capability. Read-only, safe.
- Missing commands from permissions: `get_depth_map`, `set_depth_adjustment_params`, `get_depth_adjustment_params`, `reset_depth_adjustments`, `get_mesh_data` -- these are callable but not listed in capabilities. This is likely by design (Tauri allows unlisted commands by default). **Note:** Consider adding explicit permissions for all commands in a future security hardening sprint.

---

## Frontend Review

### ExportPanel.svelte

- Uses Tauri `save()` dialog -- OS-native, returns sanitized paths. Good.
- Path from dialog passed directly to `exportStl(path)` -- acceptable since backend now validates.
- Error messages displayed to user come from backend (now sanitized). Good.
- `handleOpenFolder()` uses `shellOpen(parentDir(exportedPath))` -- the `exportedPath` is the path returned by the successful export, which has been server-side validated. Acceptable.

### tauri.ts

- `exportStl(path)` is a thin wrapper around `invoke("export_stl", { path })`. No client-side validation (correctly deferred to server). Acceptable.

---

## Summary Table

| Finding | Severity | Status | SEC Task |
|---------|----------|--------|----------|
| FINDING-1: Path traversal (no canonicalization) | MEDIUM | FIXED | SEC-401 |
| FINDING-2: No extension validation | LOW | FIXED | SEC-401 |
| FINDING-3: No system directory protection | MEDIUM | FIXED | SEC-401 |
| FINDING-4: Symlink attacks | LOW | MITIGATED (canonicalize) | SEC-401 |
| FINDING-5: Null bytes in path | NONE | N/A (Rust handles) | SEC-401 |
| FINDING-6: Filename injection | NONE | N/A (already sanitized) | SEC-401 |
| FINDING-7: No permission check before write | LOW | FIXED | SEC-402 |
| FINDING-8: Error messages leak paths | LOW | FIXED | SEC-402 |
| FINDING-9: Settings file permissions | INFO | ACCEPTED | SEC-402 |
| FINDING-10: No temp files | NONE | N/A (clean design) | SEC-402 |

---

## Code Changes Applied

### `src-tauri/src/lib.rs` -- `export_stl` command

1. **Path canonicalization:** Parent directory resolved via `std::fs::canonicalize()` before any file operation.
2. **Extension validation:** Rejects non-`.stl` extensions.
3. **System directory blocklist:** Platform-specific blocked prefixes prevent writes to OS directories.
4. **Write permission pre-check:** Test file creation in target directory before export attempt.
5. **Error message sanitization:** STL write errors no longer include full paths.
6. **Canonical path used throughout:** All operations (write, settings update, log) use the resolved canonical path.

---

## Verification

- **Build:** `cargo build` -- PASS
- **Tests:** 113 passed, 0 failed, 6 ignored (Python-dependent) -- PASS
- **No regressions** from security hardening

---

## Recommendations for Future Sprints

1. **Add explicit Tauri permissions** for all IPC commands (currently some commands are callable without being listed in capabilities).
2. **Settings file permissions on Unix:** Set `0600` on `~/.simplepicture3d/settings.json`.
3. **Rate limiting on export:** Consider preventing rapid-fire export calls (denial of service via disk fill). Low priority for desktop app.
4. **OBJ export (Sprint 1.9):** Apply same path validation pattern when OBJ export is implemented.

---

## Sign-off

**SEC-401 (File Path Handling):** PASS -- Path traversal, symlink, system directory, and extension attacks mitigated.
**SEC-402 (Export Directory Permissions):** PASS -- Permission pre-check added, error messages sanitized, no temp file concerns.

**Overall:** APPROVED for Sprint 1.8 release.

---

**Signed:** Security Specialist (Claude-Code-Security)
**Date:** 2026-02-08
