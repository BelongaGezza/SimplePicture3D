# SimplePicture3D: Pivot Plan — 2.5D Relief to 3D Volumetric Point Cloud

**Document Version:** 1.0
**Date:** 2026-04-08
**Purpose:** Detailed plan for transitioning SimplePicture3D from a 2.5D relief mesh tool to a clean, focused 3D volumetric point cloud application for internal crystal laser engraving.

---

## Executive Summary

### The Current Reality

SimplePicture3D is **not a confused codebase**—it's a well-structured 2.5D relief application with comprehensive documentation about a *planned* 3D feature track. The confusion is **perceptual, not structural**:

| Aspect | Current State |
|--------|---------------|
| **Code** | 100% 2.5D relief pipeline (depth map → surface mesh) |
| **Documentation** | Mixed messaging: PRD, README describe 2.5D; ADR-011 documents 3D plan |
| **Branch name** | `feature/crystal-volumetric-pointcloud` suggests 3D work, but contains only architectural docs |
| **Implementation** | No volumetric/3D code exists |

### The Goal

Create a **clean, single-purpose 3D volumetric point cloud application** for internal UV laser engraving of crystal blocks, with:
- Clear 3D-first intent visible to collaborating developers
- Reusable infrastructure retained from current work
- 2.5D relief code/concepts removed or clearly deprecated
- Professional documentation reflecting the new focus

---

## Part 1: Component Analysis — What to Retain vs Remove

### Retain (Reusable for 3D)

These components are **technology infrastructure** independent of 2.5D vs 3D:

| Component | Location | Lines | Reuse Rationale |
|-----------|----------|-------|-----------------|
| **Image loading/validation** | `image_loading.rs` | ~600 | Same input: PNG/JPG images |
| **Python depth bridge** | `python_bridge.rs` | ~700 | Depth maps ARE the input for volumetric sampling |
| **Depth estimation** | `python/depth_estimator.py` | ~1000 | Depth-Anything-V2/MiDaS produces depth; 3D needs depth |
| **Depth adjustments** | `depth_adjust.rs` | ~800 | Gamma, curves, contrast apply to depth before volumetric sampling |
| **Mask system** | `mask.rs`, `MaskingTools.svelte` | ~700 | Regional adjustments useful for selective volumetric density |
| **Undo/redo** | `undo.rs` | ~400 | Command pattern is UI concern, not 2.5D-specific |
| **Settings/presets** | `settings.rs`, `preset.rs` | ~700 | Session persistence, presets for depth params |
| **File I/O** | `file_io.rs` | ~300 | Temp file handling, path validation |
| **UI framework** | All Svelte components | ~3000 | Three-panel workspace, controls, preview infrastructure |
| **Three.js preview** | `Preview3D.svelte` | ~400 | Point cloud rendering works for both; add blank wireframe |
| **Tests** | 151 Rust + 39 frontend | — | Many tests are for retained components |

**Total retained:** ~7,500+ lines of production code and infrastructure

### Remove or Replace

| Component | Location | Lines | Action |
|-----------|----------|-------|--------|
| **2.5D mesh generation** | `mesh_generator.rs` | ~1100 | **Replace** with volumetric point cloud generator |
| **STL binary writer** | `mesh_generator.rs` | ~150 | **Replace** with PLY/XYZ/CSV writers (keep STL optional) |
| **OBJ writer** | `mesh_generator.rs` | ~100 | **Replace** with XYZ as primary (keep OBJ optional) |
| **Grid triangulation** | `mesh_generator.rs` | ~200 | **Remove** (volumetric = points, not triangulated surface) |
| **2.5D terminology in docs** | PRD, README, CLAUDE.md | — | **Rewrite** for 3D focus |
| **ADR-006, ADR-008** | `architecture.md` | — | **Mark deprecated** or move to archive |
| **"Relief" mode UI** | Various | — | **Remove** (single-mode app) |

### Add (New for 3D)

| Component | Purpose | Effort |
|-----------|---------|--------|
| **BlankEnvelope** | User specifies crystal dimensions (L×W×H mm) + margin | Small |
| **fit_to_blank()** | Scale 3D bbox to fit crystal envelope with margin | Small |
| **Volumetric sampler** | Column sweep: for each (x,y), emit points along Z from back to depth | Medium |
| **PLY exporter** | Binary/ASCII PLY for engraver software | Small |
| **XYZ exporter** | Simple ASCII x,y,z per line | Trivial |
| **CSV exporter** | Optional, with header | Trivial |
| **Blank wireframe preview** | Three.js box showing crystal bounds + margin | Small |
| **Point density controls** | XY step, Z spacing, total point estimate | Small |
| **Envelope validation** | Warn if points exceed bounds | Small |

---

## Part 2: Git Strategy Options

### Option A: Rebase and Squash (Clean History)

**Approach:** Create a new branch, squash all 2.5D-specific commits, preserve infrastructure commits.

```bash
# Create clean branch from main
git checkout main
git checkout -b 3d-volumetric-clean

# Interactive rebase to squash/edit
git rebase -i --root  # or from a sensible starting point

# Rewrite commit messages to remove 2.5D terminology
# Mark 2.5D-specific commits for squash/drop
```

**Pros:**
- Clean linear history
- No "2.5D" commits visible
- Clear narrative for new contributors

**Cons:**
- Force-push required (disruptive if others have cloned)
- Loses granular history for debugging

**Recommended for:** Projects with no external contributors yet

### Option B: Additive Pivot (Preserve History)

**Approach:** Keep full history, add clear "pivot" commit, deprecate 2.5D in docs.

```bash
# Merge current state
git checkout main
git merge feature/crystal-volumetric-pointcloud

# Create pivot commit
git commit --allow-empty -m "PIVOT: SimplePicture3D now 3D volumetric point cloud

This commit marks the architectural pivot from 2.5D relief mesh generation
to 3D volumetric point cloud for internal crystal laser engraving.

Previous 2.5D code is retained for reference but deprecated.
See RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md for details."

# Tag the pivot point
git tag v0.0.0-pivot-3d
```

**Pros:**
- Full history preserved
- Non-destructive
- Easy to find pivot point

**Cons:**
- "2.5D" commits still visible in history
- May confuse contributors reading old commits

**Recommended for:** Projects with existing contributors or public history

### Option C: Fresh Repository (Nuclear Option)

**Approach:** Archive current repo, create new repo with only retained files.

```bash
# Archive current repo
mv SimplePicture3D SimplePicture3D-2.5D-archive

# Create new repo
mkdir SimplePicture3D
cd SimplePicture3D
git init

# Copy retained files (no git history)
# Commit as initial "3D volumetric foundation"
```

**Pros:**
- Completely clean slate
- No historical baggage
- Simple narrative

**Cons:**
- Loses all git history
- Loses contribution attribution
- Requires re-setup of CI, issues, etc.

**Recommended for:** Rarely; only if history is truly problematic

### Recommendation: Option B with Documentation Emphasis

Given that:
- The codebase is clean (no hybrid code)
- Most code is reusable
- You have a single developer context
- The "confusion" is documentation/messaging

**I recommend Option B** with heavy emphasis on documentation updates. The history isn't harmful—it shows the evolution. New contributors will see the pivot tag and understand the current direction.

---

## Part 3: Implementation Roadmap

### Phase 0: Documentation Pivot (Immediate)

Update all documentation to reflect 3D-first intent:

| Document | Changes |
|----------|---------|
| **README.md** | Rewrite tagline, features, roadmap for 3D volumetric |
| **prd.md** | Update executive summary, feature list; deprecate 2.5D features |
| **CLAUDE.md** | Update project overview, architecture diagram |
| **architecture.md** | Promote ADR-011 to primary; deprecate ADR-006/008 |
| **todo.md** | Reorganize phases around 3D milestones |

### Phase 1: Core 3D Infrastructure (Sprints 1-2)

**Sprint P1.1: BlankEnvelope + Fit**
- Add `BlankEnvelope` struct (extent_mm, margin_mm)
- Add to `AppSettings` with defaults (80×50×50 mm, 2mm margin)
- Implement `fit_to_blank()` scaling function
- Unit tests for bbox fitting

**Sprint P1.2: Volumetric Point Generator**
- Replace `depth_to_point_cloud()` with `generate_volumetric_points()`
- Algorithm: column sweep (emit points from back plane to depth surface)
- Parameters: xy_step, z_spacing, max_points
- Unit tests for point counts, bounds

### Phase 2: Export Pipeline (Sprints 3-4)

**Sprint P2.1: New Exporters**
- Implement `export_ply()` (ASCII + binary options)
- Implement `export_xyz()` (ASCII, one point per line)
- Implement `export_csv()` (with header row)
- Remove or deprecate STL/OBJ from primary UI (move to "Advanced")

**Sprint P2.2: Engraver Validation**
- Test exported files with target engraver software
- Validate axis convention (handedness)
- Verify point spacing for laser focal requirements
- Document format compatibility matrix

### Phase 3: UI Refocus (Sprints 5-6)

**Sprint P3.1: Blank-First Workflow**
- Add blank dimension inputs (L×W×H mm) to UI
- Add margin control
- Show estimated point count before generation
- Remove "target width/height" (2.5D concept); use blank dimensions

**Sprint P3.2: Enhanced Preview**
- Add blank wireframe box to Three.js preview
- Highlight margin zone
- Color-code points outside envelope (validation)
- Add point density visualization

### Phase 4: Polish + Optional AI 3D (Sprints 7+)

**Sprint P4.1: Documentation + Onboarding**
- Complete user guide for 3D workflow
- Tutorial: image → point cloud → crystal
- Developer guide for contributing

**Sprint P4.2 (Optional): TripoSR Integration**
- Per RESEARCH/3d-reconstruction.md
- Full 3D reconstruction from single image
- Surface sampling to point cloud
- Same `fit_to_blank()` pipeline

---

## Part 4: Detailed File Changes

### Files to DELETE or Archive

```
# None needed—code is clean. Archive these in docs if desired:
# (These are implementation files, not dead code)
```

### Files to RENAME (Optional, for clarity)

| Current | Proposed | Rationale |
|---------|----------|-----------|
| `mesh_generator.rs` | `point_cloud_generator.rs` | Reflects 3D point focus |
| `MeshData` | `PointCloudData` | Semantic clarity |
| `MeshParams` | `VolumetricParams` | Semantic clarity |

### Files to MODIFY

| File | Changes |
|------|---------|
| `mesh_generator.rs` | Replace relief algorithm with volumetric column sweep |
| `lib.rs` | Add `BlankEnvelope` to `AppState`; update export commands |
| `settings.rs` | Add blank dimensions, remove target_width/height prominence |
| `Preview3D.svelte` | Add blank wireframe; keep point cloud rendering |
| `ExportPanel.svelte` | PLY/XYZ/CSV as primary; STL/OBJ as "Advanced" |
| `App.svelte` | Add blank dimension inputs; remove "relief mode" concepts |
| `README.md` | Full rewrite for 3D focus |
| `prd.md` | §1-4 rewrite; deprecation notices for 2.5D features |
| `CLAUDE.md` | Update overview, architecture, commands |

### New Files to ADD

| File | Purpose |
|------|---------|
| `blank_envelope.rs` | `BlankEnvelope` struct, `fit_to_blank()` |
| `export_ply.rs` | PLY binary/ASCII writer |
| `export_xyz.rs` | XYZ ASCII writer |
| `export_csv.rs` | CSV writer with header |
| `BlankSetup.svelte` | UI for blank dimensions input |

---

## Part 5: Documentation Rewrite Guide

### README.md — New Tagline

**Before:**
> Convert 2D images into 2.5D STL meshes for UV laser engraving in crystal and glass

**After:**
> Transform 2D images into volumetric 3D point clouds for internal UV laser engraving of crystal blocks

### README.md — New Feature List

```markdown
## Features

- **AI-Powered Depth Estimation** - Depth-Anything-V2 extracts depth from any image
- **Volumetric Point Cloud Generation** - Dense 3D coordinates filling your crystal blank
- **Crystal Blank Presets** - Common sizes (80×50×50mm, 60×60×60mm, custom)
- **Fit-to-Blank Scaling** - Automatic scaling with margin safety
- **Multiple Export Formats** - PLY, XYZ, CSV for engraver compatibility
- **Real-Time 3D Preview** - See point cloud inside blank wireframe
- **100% Offline Processing** - Your images never leave your device
- **Cross-Platform** - Windows, macOS, Linux
```

### prd.md — Updated Vision Statement

**Before:**
> Democratize 3D crystal engraving by making professional-quality depth mapping accessible to every hobbyist with a 2D image and a vision.

**After:**
> Democratize volumetric crystal engraving by transforming any 2D image into a laser-ready 3D point cloud—no 3D modeling expertise required.

---

## Part 6: Terminology Migration

| 2.5D Term (Deprecate) | 3D Term (Use) |
|-----------------------|---------------|
| Relief mesh | Volumetric point cloud |
| Heightfield | Depth-derived volume |
| Depth surface | Interior focal points |
| Target width/height | Blank dimensions (L×W×H) |
| Mesh generation | Point cloud generation |
| STL/OBJ export (primary) | PLY/XYZ/CSV export (primary) |
| Triangulation | (Not applicable—points only) |
| Grid step | XY sampling density |
| Depth range | Z fill extent |

---

## Part 7: Migration Checklist

### Pre-Migration
- [ ] Review and approve this plan
- [ ] Backup current repository state
- [ ] Decide on git strategy (Option A, B, or C)
- [ ] Notify any collaborators of pivot

### Documentation Phase
- [ ] Rewrite README.md for 3D focus
- [ ] Update prd.md executive summary and features
- [ ] Update CLAUDE.md project overview
- [ ] Add deprecation notices to ADR-006, ADR-008
- [ ] Promote ADR-011 as primary architectural reference
- [ ] Update todo.md with new phase structure

### Code Phase 1 (Foundation)
- [ ] Add `BlankEnvelope` struct and settings
- [ ] Implement `fit_to_blank()` function
- [ ] Refactor `mesh_generator.rs` → `point_cloud_generator.rs`
- [ ] Implement column sweep volumetric algorithm
- [ ] Add unit tests for volumetric generation

### Code Phase 2 (Export)
- [ ] Implement PLY exporter (binary + ASCII)
- [ ] Implement XYZ exporter
- [ ] Implement CSV exporter
- [ ] Update `ExportPanel.svelte` for new formats
- [ ] Test with target engraver software

### Code Phase 3 (UI)
- [ ] Add blank dimension inputs to UI
- [ ] Add blank wireframe to 3D preview
- [ ] Update depth controls for volumetric context
- [ ] Add point density/count estimates
- [ ] Remove or hide 2.5D-specific UI elements

### Validation
- [ ] Run full test suite
- [ ] Manual testing of full pipeline
- [ ] Test export files with engraver software
- [ ] Review documentation for 2.5D terminology leaks
- [ ] Code review for semantic clarity

---

## Part 8: Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Engraver software incompatibility | Export files don't work | E1 charter: validate formats early |
| Point count explosion | Memory/performance issues | Add max_points limit; progressive LOD |
| Loss of 2.5D users | Existing users can't use relief mode | Option: keep relief as "Legacy Mode" |
| Depth estimation unchanged | AI still produces surface depth | Column sweep converts surface to volume |
| TripoSR complexity | Full 3D reconstruction is complex | Keep as optional Phase 4 |

---

## Appendix A: What "3D" Means in This Context

**Clarification for developers:**

SimplePicture3D produces **pseudo-3D point clouds**—not true 3D reconstructions. The "3D" refers to:

1. **Output format:** Points with (x, y, z) coordinates in 3D space
2. **Engraving target:** Points distributed through the **interior volume** of a crystal, not just a surface
3. **Dimensioning:** Fitted to a physical 3D blank envelope

**It does NOT mean:**
- True 3D reconstruction from multiple views
- Watertight mesh with back faces (unless using optional TripoSR)
- Full geometry hallucination

The depth map from a single image provides Z values for the front surface; the volumetric sampler fills from a back plane to that front surface, creating a 3D point distribution.

---

## Appendix B: Retained ADRs

| ADR | Status | Notes |
|-----|--------|-------|
| ADR-001 (Svelte) | Keep | Frontend framework choice unchanged |
| ADR-002 (Subprocess) | Keep | Python bridge approach unchanged |
| ADR-003 (System Python) | Keep | Distribution strategy unchanged |
| ADR-004 (Depth models) | Keep | Still using Depth-Anything-V2/MiDaS |
| ADR-005 (Licensing) | Keep | Model license considerations unchanged |
| ADR-006 (Relief mesh) | **Deprecate** | 2.5D-specific |
| ADR-007 (Binary transfer) | Keep | IPC optimization still relevant |
| ADR-008 (Triangulation) | **Deprecate** | 2.5D-specific |
| ADR-009 (Target dimensions) | **Supersede** | Replaced by BlankEnvelope (ADR-011) |
| ADR-010 (State management) | Keep | Undo/redo architecture unchanged |
| ADR-011 (Crystal volumetric) | **Promote to primary** | Canonical 3D architecture |

---

## Appendix C: Commit Message Template for Pivot

```
PIVOT: SimplePicture3D transitions to 3D volumetric point cloud

This commit marks the architectural pivot from 2.5D relief mesh generation
to 3D volumetric point cloud for internal crystal laser engraving.

Breaking changes:
- Primary export formats: PLY, XYZ, CSV (STL/OBJ moved to Advanced)
- New required input: Blank dimensions (L×W×H mm)
- Removed: Grid triangulation, relief-specific UI

Retained:
- Image loading, depth estimation, depth adjustments
- Mask system, undo/redo, presets
- Three.js 3D preview (enhanced with blank wireframe)

See: RESEARCH/PIVOT_PLAN_2.5D_TO_3D.md

---

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>
```

---

*Document prepared by Claude Code based on comprehensive codebase analysis.*
*All estimates are implementation effort, not calendar time.*
