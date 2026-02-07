# Sprint 1.6 Test Plan

**Sprint:** 1.6 — Mesh Generation Algorithm  
**Source:** `SPRINT_1_6_Task_Assignment.md`, `SPRINTS/TEST_PLAN_TEMPLATE.md`  
**Last Updated:** 2026-02-07

---

## 1. Scope

| Item | Description |
|------|-------------|
| Sprint goal | Convert depth map to 3D point cloud/mesh in Rust. |
| Features in scope | Point cloud generation, uniform grid sampling, mm scaling (2–10mm Z), optional triangulation, vertex normals, memory optimization; architecture (algorithm, vertex format, topology, memory). |
| Out of scope | 3D preview UI (Sprint 1.7), STL/OBJ export (Sprint 1.8). |

---

## 2. Automated Tests

### 2.1 What runs in CI

| Suite | Command | When |
|-------|---------|------|
| Rust | `cargo test --manifest-path src-tauri/Cargo.toml` | Every push/PR |
| Rust | `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` | Every push/PR |
| Frontend | `npm run build` | Every push/PR |
| Python | `SP3D_USE_STUB=1 PYTHONPATH=python/python python -m pytest python/ -v` | Every push/PR |

### 2.2 New or updated automated tests this sprint

| Test | Location | Purpose |
|------|----------|---------|
| Point cloud generation (JR2-501) | src-tauri (mesh_generator or lib) | Unit tests: small depth → vertex count, bounds |
| Edge cases (JR2-502) | src-tauri | Empty, 1×1, single row/column depth |
| Mesh statistics (QA-504) | src-tauri | Bounds, normals (unit length) |

### 2.3 Coverage

Per project; mesh_generator (and related) should be covered by new tests.

---

## 3. Manual Test Plan

### 3.1 Environment

| Item | Value |
|------|--------|
| OS(s) | Windows 11 (primary); macOS/Linux if available |
| Rust | stable |
| Depth map source | From Sprint 1.4/1.5 (load image → generate depth); or synthetic in tests |

### 3.2 Manual test cases

*(QA fills when claiming role; link to MANUAL_TEST_REPORT.md.)*

- **QA-501:** Generate mesh (via backend command or test harness), verify vertex count.
- **QA-502:** Validate mesh dimensions match depth range (Z min/max 2–10mm).
- **QA-503:** Performance test: 4K depth → mesh in <15s; document hardware and time.

### 3.3 Regression

- Depth loading and depth map generation (Sprint 1.4) still work.
- Depth adjustments (Sprint 1.5) still work; adjusted depth is valid input to mesh generation.

---

## 4. Security

- **SEC-301:** Integer overflow review (vertex/index math).
- **SEC-302:** Depth map input validation (dimensions, length, range).

---

## 5. Success Criteria (todo.md Sprint 1.6)

- Mesh generation produces valid point cloud
- Vertex positions in correct units (mm)
- Performance: <15s for 4K
- Memory: <2GB for 4K
- Algorithm documented in architecture.md
