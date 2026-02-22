# Single-Image 3D Reconstruction Models

**Purpose:** Research and evaluation of AI models that produce full 3D geometry (not just 2.5D depth maps) from a single input image. Evaluated for potential integration into SimplePicture3D as a secondary output mode alongside the existing depth-map relief pipeline.

**Last checked:** 2026-02-22

---

## Context: 2.5D vs Full 3D

SimplePicture3D currently produces **2.5D heightfield** meshes: one Z value per (x, y) pixel, forming a relief surface with no back faces. This is optimal for internal UV laser engraving (single-viewpoint).

**Full 3D** reconstruction produces a **closed, watertight mesh** with geometry on all sides. The AI hallucinates unseen surfaces (back of head, underside of objects, etc.). This unlocks:

- 3D printing (FDM/SLA)
- Rotary / multi-angle laser engraving
- Full 3D visualization and AR/VR export
- Game asset creation

---

## Model Comparison Matrix

| Model | License (Code) | License (Weights) | VRAM | Speed | Direct Mesh? | Windows? | CPU? | Stars |
|-------|---------------|-------------------|------|-------|-------------|----------|------|-------|
| **TripoSR** | MIT | MIT | ~6 GB | ~0.5s | Yes (OBJ) | Yes | Yes (slow) | ~6k |
| **InstantMesh** | Apache 2.0 | Apache 2.0 | ~16 GB | ~10s | Yes (OBJ) | Untested | No | ~4.3k |
| **CRM** | MIT | MIT | 8-16 GB | ~10s | Yes (OBJ+UV) | No (WSL2) | No | ~684 |
| **TRELLIS.2** | MIT | MIT | ~24 GB | 3-60s | Yes (GLB) | No (Linux only) | No | ~12k |
| **StableFast3D** | Community* | Community* | ~6 GB | ~0.5s | Yes (GLB) | Experimental | No | ~1.6k |
| **LGM** | MIT | MIT | ~10 GB | ~5s | No (Gaussian→mesh) | No (WSL2) | No | ~2k |
| **Hunyuan3D-2** | Tencent** | Tencent** | 6-29 GB | 10-40s | Yes (GLB/OBJ) | Yes | No | ~13k |

\* StableFast3D: Free for <$1M annual revenue; enterprise license required above that.
\** Hunyuan3D-2: Excludes EU, UK, South Korea; >1M MAU requires commercial approval.

---

## Detailed Model Assessments

### 1. TripoSR — RECOMMENDED

**Repository:** [VAST-AI-Research/TripoSR](https://github.com/VAST-AI-Research/TripoSR)
**Paper:** Stability AI + Tripo AI, March 2024
**Stars:** ~6,000 | **Contributors:** 7 | **Commits:** 35

**License:**
- Code: MIT
- Weights: MIT (via `stabilityai/TripoSR` on Hugging Face)
- Commercial use: Fully permitted, no restrictions

**Technical Details:**
- VRAM: ~6 GB (lowest of all candidates)
- Inference: ~0.5 seconds on A100; CPU fallback available (`--device cpu`) but significantly slower
- Output: OBJ mesh with texture atlas (default 2048x2048), optional GLB via `--bake-texture`
- Python: >=3.8, PyTorch, trimesh, transformers, rembg, torchmcubes
- Windows: Works (less documented than Linux)

**Key Dependencies:**
```
omegaconf==2.3.0, Pillow==10.1.0, einops==0.7.0
transformers==4.35.0, trimesh==4.0.5, rembg
huggingface-hub, xatlas==0.0.9, moderngl==5.10.0
git+https://github.com/tatsy/torchmcubes.git
```

**Concern:** `torchmcubes` requires CUDA compilation. Fallback exists but may complicate Windows builds.

**Successor:** StableFast3D (SF3D) is the evolution, but has restrictive licensing.

**Why recommended:**
- MIT license on both code and weights (best licensing of all candidates)
- Lowest VRAM requirement (6 GB = broadest hardware support)
- Fastest inference (0.5s)
- Direct OBJ output (compatible with existing STL/OBJ export pipeline)
- CPU fallback exists (even if slow)
- Active community, well-documented

---

### 2. InstantMesh — VIABLE ALTERNATIVE

**Repository:** [TencentARC/InstantMesh](https://github.com/TencentARC/InstantMesh)
**Paper:** CVPR 2024, April 2024
**Stars:** ~4,300

**License:**
- Code: Apache 2.0
- Weights: Apache 2.0
- Commercial use: Permitted with attribution

**Technical Details:**
- VRAM: ~16 GB (high — limits to enthusiast GPUs: RTX 3090, 4080+)
- Inference: ~10 seconds
- Output: OBJ with vertex colors; optional texture maps via `--export_texmap`
- Python: 3.10+, PyTorch 2.1+, CUDA 12.1+ required
- CPU: Not supported
- Windows: Not officially documented; uses conda (may work)

**Key Dependencies:**
```
pytorch-lightning==2.1.2, transformers==4.34.1
diffusers==0.20.2, trimesh, PyMCubes, xatlas
nvdiffrast (requires CUDA compilation)
```

**Concern:** `nvdiffrast` is an NVIDIA library requiring CUDA headers and compilation. Known build issues on Windows.

**Why viable:** Apache 2.0 is commercially friendly. Higher quality than TripoSR for some object categories (uses multi-view diffusion). But 16 GB VRAM and no CPU fallback limit the user base significantly.

---

### 3. CRM — NOT RECOMMENDED (Windows issues)

**Repository:** [thu-ml/CRM](https://github.com/thu-ml/CRM)
**Paper:** ECCV 2024
**Stars:** ~684

**License:**
- Code: MIT
- Weights: MIT
- Commercial use: Fully permitted

**Technical Details:**
- VRAM: 8 GB minimum (splitting workflow), 16 GB+ recommended
- Inference: ~10 seconds on A800
- Output: OBJ with UV textures
- Python: 3.9 (pinned), PyTorch 1.13+cu117 (legacy)
- CPU: Not supported
- Windows: Not supported (xformers + nvdiffrast build issues)

**Why not recommended:** Pinned to legacy PyTorch 1.13 (EOL). xformers and nvdiffrast require CUDA compilation with known Windows failures. Small community (684 stars, 3 contributors). Would conflict with SimplePicture3D's PyTorch 2.x stack.

---

### 4. TRELLIS.2 — NOT RECOMMENDED (hardware requirements)

**Repository:** [microsoft/TRELLIS.2](https://github.com/microsoft/TRELLIS.2)
**Paper:** CVPR 2025 Spotlight
**Stars:** ~12,000 (original TRELLIS)

**License:**
- Code: MIT
- Weights: MIT
- Commercial use: Fully permitted

**Technical Details:**
- VRAM: ~24 GB minimum (RTX 4090 or A100 class)
- Inference: 3s (512 voxels) to 60s (1536 voxels) on H100
- Output: GLB with full PBR materials (albedo, metallic, roughness)
- Model size: 4B parameters
- Python: 3.10-3.11, PyTorch 2.6+, CUDA 12.4
- CPU: Not supported
- Windows: Linux only (community WSL2 workarounds exist)

**Why not recommended:** 24 GB VRAM requirement excludes the vast majority of hobbyist hardware. Linux-only. Complex dependency chain (flash-attn, kaolin, spconv, nvdiffrast). Highest quality output of all candidates, but impractical for a desktop app targeting hobbyists.

---

### 5. StableFast3D (SF3D) — NOT RECOMMENDED (license)

**Repository:** [Stability-AI/stable-fast-3d](https://github.com/Stability-AI/stable-fast-3d)
**Stars:** ~1,600

**License:**
- Code: Stability AI Community License
- Weights: Stability AI Community License
- Commercial use: **Free only below $1M annual revenue**; enterprise license required above
- Must register at stability.ai for commercial use

**Technical Details:**
- VRAM: ~6 GB (same as TripoSR)
- Inference: ~0.5 seconds
- Output: GLB with UV-unwrapped textures and material parameters
- Windows: Experimental (requires Visual Studio 2022)

**Why not recommended:** License incompatible with MIT project philosophy. Revenue-threshold licensing creates legal uncertainty for users. TripoSR (MIT) achieves similar performance without restrictions.

---

### 6. LGM — NOT RECOMMENDED (indirect mesh output)

**Repository:** [3DTopia/LGM](https://github.com/3DTopia/LGM)
**Paper:** ECCV 2024 Oral
**Stars:** ~2,000

**License:**
- Code: MIT
- Weights: MIT
- Commercial use: Fully permitted

**Technical Details:**
- VRAM: ~10 GB
- Inference: ~5 seconds
- Output: 3D Gaussian Splatting (.ply) — **not a mesh**
- Mesh conversion: Via separate `convert.py` script using marching cubes (mcubes)
- Windows: WSL2 recommended; native not documented

**Why not recommended:** Primary output is Gaussian Splatting, not a mesh. Requires a separate conversion step to produce OBJ/STL, adding complexity and quality loss. TripoSR outputs meshes directly.

---

### 7. Hunyuan3D-2 — CONDITIONAL (geographic license restrictions)

**Repository:** [Tencent-Hunyuan/Hunyuan3D-2](https://github.com/Tencent-Hunyuan/Hunyuan3D-2)
**Stars:** ~13,200 | Latest: v2.1 (June 2025)

**License:**
- Code: Tencent Community License
- Weights: Tencent Community License
- **Geographic exclusions: EU, UK, South Korea**
- Commercial use: <1M MAU free within permitted territories; above requires approval
- Additional restrictions: Cannot train competing AI models from outputs

**Technical Details:**
- VRAM: 6 GB (shape only, mini variant) to 29 GB (full textured, v2.1)
- Inference: 10-40 seconds (standard); ~1s (turbo distillation variants)
- Output: GLB/OBJ with PBR textures (albedo, metallic, roughness)
- Python: 3.10+, PyTorch 2.0+, CUDA 12.1+
- CPU: Not practical (~50-100x slower, undocumented)
- Windows: Yes (community portable builds exist)

**Why conditional:** Highest quality textured output. Shape-only mini variant runs on 6 GB. But geographic restrictions on EU/UK/South Korea are a showstopper for an open-source MIT-licensed project. Cannot guarantee all users can legally use the model. Consider only if the project commits to non-EU distribution or obtains written Tencent approval.

---

## Laser Engraving Point Cloud Requirements

### How Internal UV Laser Engraving Works

Internal UV laser engraving does **not** consume meshes (STL/OBJ with triangle faces). The laser focuses inside a transparent material (K9 crystal, glass, acrylic) and fires pulses. Each pulse creates a micro-fracture at the focal point. The machine steps through a list of 3D coordinates — a **point cloud** — and fires at each one. The accumulation of hundreds of thousands of fracture points creates the visible 3D image.

**Key physical properties:**

| Property | Typical Value | Implication |
|----------|---------------|-------------|
| Point spacing | 0.05–0.15 mm | Determines visual density/quality |
| Point count | 100K–2M+ | Machine-dependent upper limit |
| Material is transparent | Always | Every point visible from every angle — no occlusion |
| Crystal blank size | e.g. 50×50×80 mm | Points must fit within physical bounds |
| Laser power per point | Fixed or variable | Variable power enables grayscale/shading |

**Critical insight:** Because crystal is transparent, **all engraved points are visible simultaneously from every viewing angle**. There is no "back" that the viewer can't see. This means hallucinated back surfaces from AI reconstruction are fully exposed to inspection.

### What the Engraver Actually Imports

Laser engraving software (LaserCAD, ArtCAM, Crystal3D, etc.) typically accepts:

1. **XYZ point lists** (CSV, TXT, or proprietary binary) — most direct
2. **STL meshes** — the software does its own surface-to-point-cloud conversion internally
3. **Proprietary formats** — machine-specific

If SimplePicture3D exports STL, the engraving software handles point sampling. If it exports a point cloud directly, it gives the user more control over density and placement. **Both paths should be supported.**

### 2.5D Pipeline: Point Cloud Is Natural

The existing 2.5D pipeline is inherently a point cloud generator:

```
Depth map (2D grid) → depth_to_point_cloud() → (x_mm, y_mm, z_mm) per sample
```

- Each pixel maps to exactly one 3D point
- The grid structure provides uniform point spacing automatically
- `step_x`/`step_y` control density: step=1 → full resolution, step=2 → 1/4 points
- `pixel_to_mm` (ADR-009) maps pixel indices to physical dimensions
- Normals are computed for preview but **irrelevant to the engraver** (laser fires or doesn't)

The triangulated mesh (`MeshData.indices`) is only needed for STL/OBJ file format compliance and Three.js solid preview. The actual engraving data is the point positions.

### Full 3D Pipeline: New Problems

Converting a TripoSR mesh to a dimensioned point cloud for laser engraving introduces problems that have **no equivalent in the current codebase:**

#### Problem 1: Mesh Surface Sampling

TripoSR outputs a triangle mesh (OBJ). The engraver needs points. A new processing step is required: **sample the mesh surface to produce uniformly-distributed 3D points.**

This is fundamentally different from the 2.5D case where the depth map IS already a grid of points.

**Approaches:**

| Method | Quality | Complexity | Library Support |
|--------|---------|------------|-----------------|
| **Poisson disk sampling** | Best (well-distributed, no clustering) | Medium | `trimesh.sample.sample_surface` (Python), no Rust crate |
| **Random barycentric sampling** | Good (fast, but can cluster) | Low | `trimesh.sample.sample_surface` (Python) |
| **UV-based grid sampling** | Good if UVs are quality | Medium | Depends on UV parameterization |
| **Voxelization** | Fair (axis-aligned artifacts) | Low | Multiple Rust/Python options |

**Recommendation:** Use `trimesh.sample.sample_surface_even` in Python (Poisson disk). This keeps the sampling step in the Python subprocess alongside TripoSR, outputting a final point cloud (not a mesh) back to Rust. Avoids needing a Rust Poisson disk implementation.

#### Problem 2: Dimensioning Is Volumetric, Not Pixel-Based

The current ADR-009 computes scale from a 2D relationship:
```
pixel_to_mm = min(target_width_mm / width_px, target_height_mm / height_px)
```

A TripoSR mesh has no pixel grid. It outputs vertices in arbitrary model-space units. Dimensioning requires:

1. Compute the mesh **3D bounding box** (not 2D)
2. Map the bounding box to fit inside the **crystal blank volume** (3 axes: width, height, depth)
3. Apply uniform scale to all vertices
4. Center within the blank volume
5. Apply Z-offset to position the engraved region correctly within the crystal

**Formula:**
```
bbox = mesh bounding box (min/max on each axis)
scale = min(blank_w / bbox_w, blank_h / bbox_h, blank_d / bbox_d)
```

This is a different parameter set from 2.5D:

| Parameter | 2.5D (existing) | Full 3D (new) |
|-----------|-----------------|---------------|
| XY size | `target_width_mm`, `target_height_mm` | `blank_width_mm`, `blank_height_mm` |
| Z size | `depth_min_mm`, `depth_max_mm` (2–10mm range) | `blank_depth_mm` (full crystal depth) |
| Z position | Depth mapped within range | Centered in blank with margins |
| Scale source | Pixel count → mm | Bounding box → mm |
| Aspect ratio | Image aspect ratio | Object aspect ratio (3D) |

**New `MeshParams` fields (or separate struct):**
- `blank_width_mm`, `blank_height_mm`, `blank_depth_mm` — crystal blank dimensions
- `margin_mm` — inset from crystal edges (laser can't fire at the very edge)

#### Problem 3: Point Density Control

For 2.5D, density is `step_x` / `step_y` — intuitive and predictable. Vertex count = `(w/step) * (h/step)`.

For 3D, density is controlled by:
- **Point spacing** (mm between points), e.g. 0.1 mm — maps to engraver capability
- **Total point count** — maps to machine limits (e.g. max 1M points)

The relationship between spacing and point count depends on the **total surface area** of the 3D mesh, which varies per model and is not known until after reconstruction. The UI must either:
- Let the user set spacing and show estimated point count after sampling, or
- Let the user set a target point count and derive spacing from surface area

**New parameters:**
- `point_spacing_mm: f32` — target distance between points (default 0.1)
- `max_points: Option<u32>` — cap for machine limits (default None)

#### Problem 4: All Surfaces Are Visible

In 2.5D, only the front relief surface is engraved. Quality concerns are limited to depth accuracy.

In full 3D, the crystal viewer sees **every surface from every angle**:
- Front face: derived from the input photo — high quality
- Back face: **entirely hallucinated by TripoSR** — plausible but invented
- Sides: partially hallucinated — quality varies

For a portrait, the viewer walks around the crystal and sees an AI-invented back-of-head. This is the same behavior as commercial 3D crystal engraving (which also uses AI reconstruction), but users should be informed that back surfaces are approximations.

**Mitigation:** Document in the UI: "Back and side surfaces are AI-generated approximations. For highest quality, use 2.5D Relief mode which reproduces only what the camera captured."

#### Problem 5: Normals Are Irrelevant, Intensity May Matter

**Normals:** The current `MeshData` stores per-vertex normals. For laser engraving point clouds, normals serve no purpose — the laser fires a pulse at (x, y, z) regardless of surface orientation. Normals are only needed for Three.js preview rendering.

**Intensity/grayscale:** Some laser engravers support **variable power per point**, creating denser or brighter marks. This enables grayscale effects. For full 3D, per-point intensity could be derived from:
- The mesh texture value at the sampled point (if TripoSR provides texture)
- The surface normal dot product with a virtual light direction (simulated shading)

This is an optional enhancement, not required for MVP. But the point cloud export format should reserve space for an optional intensity channel:

```
Point { x: f32, y: f32, z: f32, intensity: Option<f32> }  // 0.0–1.0
```

#### Problem 6: New Export Format Needed

The current STL/OBJ writers produce triangulated mesh files. For direct engraver consumption, a **point cloud exporter** is needed:

**XYZ format (simplest, widely supported):**
```
# SimplePicture3D point cloud export
# Points: 500000
# Dimensions: 48.2 x 67.1 x 35.4 mm
# Spacing: 0.10 mm
12.450 23.100 5.670
12.550 23.100 5.680
...
```

**PLY format (supports intensity):**
```
ply
format ascii 1.0
element vertex 500000
property float x
property float y
property float z
property float intensity
end_header
12.450 23.100 5.670 0.85
...
```

**CSV format (spreadsheet/scripting friendly):**
```
x,y,z,intensity
12.450,23.100,5.670,0.85
...
```

The existing STL/OBJ exporters remain useful: some engraving software imports STL and does its own point sampling. Both mesh export and direct point cloud export should be offered.

### Revised Full 3D Pipeline

```
Image
  → TripoSR (Python subprocess)
  → OBJ mesh (arbitrary units)
  → Surface sampling (Python: trimesh Poisson disk)
  → Raw point cloud (model-space units)
  → Rust: scale + center to crystal blank dimensions
  → Dimensioned point cloud (mm)
  → Export: XYZ/PLY/CSV (for engraver) OR STL/OBJ (for software that does its own sampling)
  → Preview: Three.js point cloud with crystal blank wireframe overlay
```

Compared to current 2.5D:
```
Image
  → Depth-Anything-V2 (Python subprocess)
  → Depth map (0–1 float grid)
  → Rust: depth_to_point_cloud (grid sampling, pixel_to_mm scaling)
  → Dimensioned point cloud / triangulated mesh (mm)
  → Export: STL/OBJ
  → Preview: Three.js mesh/point cloud
```

### Impact on Code

| Component | Change | Effort |
|-----------|--------|--------|
| `python/reconstruction_3d.py` | **New file.** TripoSR inference + trimesh surface sampling. Outputs point cloud (JSON or binary) to stdout/file. | Medium |
| `mesh_generator.rs` | **New function:** `import_point_cloud` — accepts pre-sampled points, scales to blank dimensions, centers. No grid assumption. | Medium |
| `mesh_generator.rs` | **New struct:** `BlankParams { width_mm, height_mm, depth_mm, margin_mm }` for crystal blank dimensions. | Small |
| `mesh_generator.rs` | **New function:** `scale_to_blank` — bounding-box fit into crystal volume. | Small |
| `mesh_generator.rs` | **New exporter:** `write_xyz`, `write_ply`, `write_csv` for point cloud formats. | Medium |
| `lib.rs` | **New command:** `reconstruct_3d` — invokes Python, receives point cloud, stores in state. | Small |
| `lib.rs` | **Modified command:** `export_stl`/`export_obj` — handle mesh-from-3D-reconstruction path if indices present. | Small |
| `lib.rs` | **New command:** `export_point_cloud` — XYZ/PLY/CSV export. | Small |
| `MeshData` | **New optional field:** `intensities: Option<Vec<f32>>` for per-point laser power. | Small |
| Frontend: `Preview3D.svelte` | Show crystal blank wireframe overlay; orbit camera; point cloud rendering (already supported). | Medium |
| Frontend: new controls | Blank dimensions inputs, point spacing slider, mode toggle. | Medium |
| `FirstRunWizard.svelte` | Additional model category for TripoSR download. | Small |
| `settings.rs` | New fields: blank dimensions, point spacing, last-used 3D mode. | Small |

**Total new code estimate:** ~800–1200 lines Rust, ~300–500 lines Python, ~400–600 lines Svelte/TS.

---

## Recommendation Summary

### Primary: TripoSR

**TripoSR is the clear winner** for SimplePicture3D integration:

| Criterion | TripoSR | Runner-up (InstantMesh) |
|-----------|---------|------------------------|
| License | MIT (code + weights) | Apache 2.0 |
| VRAM | 6 GB | 16 GB |
| Speed | 0.5s | 10s |
| Direct mesh output | Yes (OBJ) | Yes (OBJ) |
| CPU fallback | Yes | No |
| Windows | Yes | Untested |
| Community | ~6k stars | ~4.3k stars |

### Integration Strategy

TripoSR should be offered as a **secondary mode** alongside the existing depth-map pipeline. The Python subprocess performs both reconstruction and surface sampling, returning a point cloud (not a mesh) to the Rust backend:

```
Load Image
    ├── [2.5D Relief] (existing, default)
    │   └── Depth-Anything-V2 / MiDaS → depth map → grid point cloud
    │   └── Best for: single-viewpoint laser engraving, relief surfaces
    │
    └── [Full 3D] (new, optional)
        └── TripoSR → mesh → Poisson disk sampling → 3D point cloud
        └── Best for: 3D crystal engraving, 3D printing, multi-angle viewing
```

**Output for both modes is a dimensioned point cloud in mm** — the fundamental unit of laser engraving. The difference is how the points are generated (grid from depth map vs surface sampling from 3D mesh) and how they are dimensioned (2D target size vs 3D crystal blank volume).

### Timeline Estimate

| Sprint | Work |
|--------|------|
| Phase 2, Sprint 1 | Python TripoSR integration + Poisson disk surface sampling + subprocess contract |
| Phase 2, Sprint 2 | Rust: blank dimensioning, point cloud import, scale-to-blank, new export formats (XYZ/PLY/CSV) |
| Phase 2, Sprint 3 | UI: mode toggle, blank dimension inputs, point spacing control, crystal outline preview |
| Phase 2, Sprint 4 | Testing, integration, model download wizard extension, documentation |
| Phase 2, Sprint 5 | Optional: per-point intensity from texture, variable laser power export |

**Total: ~4–5 sprints (8–10 weeks)**

### Risk Register

| Risk | Severity | Mitigation |
|------|----------|------------|
| torchmcubes CUDA compilation on Windows | Medium | Test early; fallback to pre-compiled wheels or CPU marching cubes |
| Hallucinated back surfaces visible in crystal | Medium | Document limitation; quality is "plausible" not "accurate". 2.5D remains default. |
| 6 GB VRAM excludes some users | Medium | CPU fallback exists; 2.5D pipeline remains default and requires no GPU |
| Poisson disk sampling slow for high-density clouds | Low | trimesh is well-optimized; 1M points in <5s typical. Cap via `max_points`. |
| Point cloud export format incompatible with user's engraver | Medium | Support multiple formats (XYZ, PLY, CSV, STL). Document tested engraver software. |
| TripoSR repo becomes unmaintained | Low | MIT license allows forking; 6k stars community likely to sustain |
| Model download size adds to first-run burden | Low | Separate download from depth models; user chooses which to install |
| Surface area unpredictable → point count surprises user | Low | Show estimated point count after sampling; allow adjustment before export |

---

## Sources

| Source | URL |
|--------|-----|
| TripoSR GitHub | https://github.com/VAST-AI-Research/TripoSR |
| TripoSR Hugging Face | https://huggingface.co/stabilityai/TripoSR |
| TripoSR Paper | https://arxiv.org/abs/2403.02151 |
| InstantMesh GitHub | https://github.com/TencentARC/InstantMesh |
| InstantMesh Paper | https://arxiv.org/abs/2404.07191 |
| CRM GitHub | https://github.com/thu-ml/CRM |
| CRM Paper | https://arxiv.org/abs/2403.05034 |
| TRELLIS.2 GitHub | https://github.com/microsoft/TRELLIS.2 |
| TRELLIS.2 Project Page | https://microsoft.github.io/TRELLIS.2/ |
| StableFast3D GitHub | https://github.com/Stability-AI/stable-fast-3d |
| LGM GitHub | https://github.com/3DTopia/LGM |
| LGM Paper | https://arxiv.org/abs/2402.05054 |
| Hunyuan3D-2 GitHub | https://github.com/Tencent-Hunyuan/Hunyuan3D-2 |
| Stability AI TripoSR Announcement | https://stability.ai/news/triposr-3d-generation |
