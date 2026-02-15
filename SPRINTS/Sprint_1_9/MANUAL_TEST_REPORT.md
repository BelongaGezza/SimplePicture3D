# Manual Test Report -- Sprint 1.9

**Author:** Quality Engineer
**Date:** 2026-02-15
**Sprint:** 1.9 -- OBJ Export & Settings Persistence

---

## QA-801: Export OBJ, import to external tool

### Procedure
1. Load an image in SimplePicture3D
2. Generate depth map
3. Select "OBJ (ASCII)" from the format dropdown
4. Click Export and save as `.obj`
5. Verify `.mtl` file is created alongside `.obj`
6. Open `.obj` in Blender (File > Import > Wavefront OBJ)
7. Verify mesh displays correctly with faces and normals

### Automated Coverage
- 8 OBJ writer tests verify format correctness
- Programmatic roundtrip: write OBJ -> parse -> verify vertices/faces/normals match
- File write roundtrip: OBJ+MTL files created on disk, content validates

### Status: PASS (automated), manual verification pending

---

## QA-802: Settings persistence across app restarts

### Procedure
1. Launch SimplePicture3D
2. Change export format to OBJ
3. Export a file (to set last_export_dir)
4. Close the application
5. Relaunch
6. Verify format dropdown shows "OBJ (ASCII)"
7. Verify settings panel shows last export directory

### Automated Coverage
- Settings save/load roundtrip test
- Extended settings roundtrip (all fields)
- Backwards compatibility test (old settings format)

### Status: PASS (automated), manual verification pending

---

## QA-803: Defaults when settings file missing

### Procedure
1. Delete `~/.simplepicture3d/settings.json`
2. Launch SimplePicture3D
3. Verify format defaults to STL
4. Verify no error messages

### Automated Coverage
- `default_settings_all_none`: verifies all fields are None
- `settings_load_returns_default_when_no_file`: no panic on missing file
- `settings_empty_json_returns_defaults`: empty JSON returns defaults

### Status: PASS (automated)

---

## QA-804: Settings round-trip automated test

### Tests
1. `settings_roundtrip_json`: serialize/deserialize with export_format
2. `settings_extended_roundtrip_json`: all 10 fields round-trip correctly
3. `settings_skip_serializing_none_fields`: defaults serialize to `{}`
4. `settings_partial_json_loads_with_defaults`: old format loads with new defaults
5. `settings_unknown_fields_ignored`: future fields don't break parsing
6. `settings_corrupt_json_returns_error`: corrupt JSON fails gracefully
7. `settings_empty_json_returns_defaults`: `{}` returns all defaults

### Status: PASS (all 7 automated tests pass)

---

**Document Version:** 1.0
**Last Updated:** 2026-02-15
