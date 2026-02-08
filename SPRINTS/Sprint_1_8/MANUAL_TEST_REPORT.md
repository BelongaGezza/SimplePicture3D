# Manual Test Report -- Sprint 1.8: STL Export

**Author:** Quality Engineer (Claude-Code-QA)
**Date:** 2026-02-08
**Sprint:** 1.8 -- STL Export Implementation
**Status:** Procedures documented; execution pending manual testing

---

## Summary

This report documents the manual test procedures for QA-701 (STL export validation), QA-702 (dimension verification), and QA-703 (filename generation). Automated test results are included where applicable. Manual test execution requires running the full Tauri application with external tools (MeshLab/PrusaSlicer).

---

## QA-701: Export STL and Open in External Tool

### Test Procedure

| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Launch app: `npm run tauri dev` | App window opens |
| 2 | Click "Open Image", select a test image | Image preview displayed; width/height shown |
| 3 | Click "Generate Depth Map" | Depth map generated; progress indicator shows completion |
| 4 | Verify Export Panel shows format dropdown (STL selected) | Format dropdown visible; STL is default; OBJ disabled |
| 5 | Click "Export STL" | Native save dialog opens with suggested filename |
| 6 | Choose location, click Save | Export begins; progress indicator shown |
| 7 | Verify success notification | Green notification with filename, "Open Folder" button, dismiss (X) |
| 8 | Click "Open Folder" | OS file manager opens to export directory |
| 9 | Open .stl in MeshLab: File > Import Mesh | Mesh loads without errors or warnings |
| 10 | Inspect mesh visually | Shape corresponds to depth variation in source image |
| 11 | (Optional) Open in PrusaSlicer: File > Add | Mesh loads and displays correctly |

### Pass Criteria
- [ ] STL file created on disk
- [ ] MeshLab opens file without errors
- [ ] Mesh shape visually correct
- [ ] UI feedback (progress, success, open folder) works as expected

### Automated Coverage
The following automated tests partially satisfy this requirement:
- `jr2_702_programmatic_stl_format_validation`: Full binary format validation (substitute for MeshLab CLI)
- `full_pipeline_depth_to_stl`: End-to-end depth -> mesh -> validate -> STL
- `stl_write_to_file_roundtrip`: File I/O round-trip

### Result
**Status:** Procedure documented; execution pending manual testing.

---

## QA-702: Verify Dimensions Match Specified Depth Range

### Test Procedure

| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Launch app | App window opens |
| 2 | Load test image with depth variation | Image loaded |
| 3 | Set depth range: depth_min_mm = 2.0, depth_max_mm = 10.0 | Parameters applied |
| 4 | Generate depth map | Depth map shows variation from 0 to 1 |
| 5 | Export STL | File saved |
| 6 | Open in MeshLab | Mesh loaded |
| 7 | Filters > Quality Measure > Compute Geometric Measures | Bounding box displayed |
| 8 | Read Z min and Z max | Z min ~ 2.0mm, Z max ~ 10.0mm |
| 9 | Verify Z range | Z min within 2.0 +/- 0.1mm; Z max within 10.0 +/- 0.1mm |

### Additional Dimension Tests

| Test Case | depth_min_mm | depth_max_mm | Expected Z Range |
|-----------|-------------|-------------|-----------------|
| Default range | 2.0 | 10.0 | 2.0 -- 10.0 mm |
| Narrow range | 5.0 | 6.0 | 5.0 -- 6.0 mm |
| Wide range | 0.5 | 20.0 | 0.5 -- 20.0 mm |
| Equal (flat) | 5.0 | 5.0 | All Z = 5.0 mm |

### Pass Criteria
- [ ] Z min within +/-0.1mm of depth_min_mm
- [ ] Z max within +/-0.1mm of depth_max_mm
- [ ] X/Y extent reasonable for image size * pixel_to_mm

### Automated Coverage
The following automated tests verify dimensions programmatically:
- `z_range_respected`: Depth 0.0 -> z=2.0mm, depth 0.5 -> z=6.0mm, depth 1.0 -> z=10.0mm (PASS)
- `point_cloud_5x5_vertex_count_and_bounds`: Z bounds within [2, 10] mm (PASS)
- `jr2_703_extreme_depth_zero`: Depth 0.0 -> z=depth_min_mm (PASS)
- `jr2_703_extreme_depth_one`: Depth 1.0 -> z=depth_max_mm (PASS)
- `point_cloud_3x3_step1`: Verifies positions[0].z = 2.0, positions[4].z = 6.0, positions[8].z = 10.0 (PASS)

### Result
**Status:** Procedure documented; execution pending manual testing. Dimension correctness verified by 5 automated unit tests.

---

## QA-703: Filename Generation Testing

### Test Procedure

| Step | Action | Expected Filename | Verified By |
|------|--------|------------------|-------------|
| 1 | Load `photo.png`, open export dialog | `photo_YYYYMMDD_HHMMSS.stl` | Manual |
| 2 | Load `my photo.jpg`, open export dialog | `my_photo_YYYYMMDD_HHMMSS.stl` | Automated: `generate_export_filename_with_spaces` |
| 3 | Load `image (1).png`, open export dialog | `image__1__YYYYMMDD_HHMMSS.stl` | Manual |
| 4 | Load image with Unicode name, open export dialog | Special chars replaced with `_` | Manual |
| 5 | No image loaded, export dialog | `mesh_YYYYMMDD_HHMMSS.stl` | Automated: `generate_export_filename_empty_path` |
| 6 | Load `C:\photos\my_image.png` | `my_image_YYYYMMDD_HHMMSS.stl` | Automated: `generate_export_filename_from_image_path` |
| 7 | Verify timestamp format | 15 chars: YYYYMMDD_HHMMSS | Automated: `generate_export_filename_format` |

### Automated Test Results

| Test | Input | Result | Status |
|------|-------|--------|--------|
| `generate_export_filename_from_image_path` | `C:\photos\my_image.png` | Starts with `my_image_`, ends with `.stl` | PASS |
| `generate_export_filename_empty_path` | `""` | Starts with `mesh_`, ends with `.stl` | PASS |
| `generate_export_filename_with_spaces` | `/home/user/my photo.jpg` | Starts with `my_photo_`, ends with `.stl` | PASS |
| `generate_export_filename_format` | `test.png` | `test_` + 15-char timestamp + `.stl` | PASS |

### Sanitization Logic Review

From `mesh_generator.rs` lines 507-516, the sanitization replaces any character that is not alphanumeric, underscore, or hyphen with an underscore. This covers:
- Spaces -> `_`
- Parentheses -> `_`
- Dots (in stem) -> `_`
- Unicode non-alphanumeric -> `_`
- Unicode alphanumeric (e.g., accented letters) -> preserved (Rust `char::is_alphanumeric()` covers Unicode)

### Missing Automated Edge Cases

The following edge cases are not covered by existing tests but are handled by the sanitization logic:
1. **Unicode alphanumeric** (e.g., `foto`): `is_alphanumeric()` returns true for accented chars, so they would be preserved. This is correct behavior.
2. **All-special-character filename** (e.g., `!@#$.png`): Would produce `____` as stem.
3. **Very long filenames** (e.g., 255+ chars): No truncation in current code. OS will reject if path exceeds max length.
4. **Whitespace-only path** (e.g., `"   "`): Falls back to `mesh` stem.

### Pass Criteria
- [x] Filename format correct (stem_timestamp.stl) -- 4 automated tests PASS
- [x] Spaces handled (replaced with `_`) -- automated test PASS
- [x] Special characters handled (replaced with `_`) -- code review confirms
- [ ] Unicode filenames tested end-to-end in app -- pending manual test
- [x] Empty path fallback works -- automated test PASS

### Result
**Status:** Partially verified (4 automated tests PASS, code review confirms sanitization logic). Manual end-to-end testing with the Tauri app pending.

---

## Overall Manual Test Status

| Task | Procedure | Automated | Manual | Overall |
|------|-----------|-----------|--------|---------|
| QA-701 | Documented | Partial (format validation) | Pending | Pending |
| QA-702 | Documented | Verified (5 dimension tests) | Pending | Pending |
| QA-703 | Documented | Verified (4 filename tests) | Pending | Partially Complete |
| QA-704 | N/A | Complete (8 round-trip tests) | N/A | COMPLETE |

---

**Document Version:** 1.0
**Last Updated:** 2026-02-08
