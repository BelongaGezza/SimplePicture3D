# SimplePicture3D - Product Requirements Document

**Version:** 1.0  
**Date:** January 31, 2026  
**Status:** Draft for Development  
**License:** MIT  

---

## 1. Executive Summary

### 1.1 Product Overview
SimplePicture3D is an open-source desktop application that converts 2D images into 2.5D STL/OBJ mesh files optimized for internal UV laser engraving of K9 crystal and similar materials. The application combines AI-powered depth estimation with manual control tools, enabling hobbyists to create stunning 3D engravings from photographs, artwork, and graphics.

### 1.2 Key Objectives
- **Accessibility**: Enable hobbyists without 3D modeling expertise to create laser-engravable meshes
- **Quality**: Produce professional-grade point cloud meshes (2-10mm depth range)
- **Privacy**: 100% offline processing with optional AI model downloads
- **Cross-Platform**: Native support for Windows, macOS, and Linux
- **Open Source**: MIT licensed with community contribution pathway

### 1.3 Success Metrics
- **Technical**: Generate valid STL/OBJ files compatible with standard laser engraving software
- **Usability**: User can complete first conversion within 5 minutes of installation
- **Performance**: Process 4K image to mesh in <2 minutes on mid-range hardware
- **Adoption**: 1,000+ GitHub stars within 6 months of v1.0 release
- **Community**: Active contributor base with 10+ external contributors

---

## 2. Product Vision & Goals

### 2.1 Vision Statement
*"Democratize 3D crystal engraving by making professional-quality depth mapping accessible to every hobbyist with a 2D image and a vision."*

### 2.2 Core Principles
1. **Simplicity First**: Complex AI technology hidden behind intuitive interface
2. **Local Processing**: User data never leaves their machine
3. **Transparency**: Open source enables learning, auditing, and contribution
4. **Quality Output**: Production-ready files that "just work" with laser systems
5. **Progressive Disclosure**: Simple by default, powerful when needed

### 2.3 Strategic Goals
- **Phase 1 (MVP)**: Prove core conversion pipeline with essential features
- **Phase 2 (Enhanced UX)**: Polish workspace and advanced controls
- **Phase 3 (Cross-Platform)**: Achieve parity across all OS targets
- **Phase 4 (Production)**: Release-ready application with professional packaging

---

## 3. Target Audience & User Personas

### 3.1 Primary Persona: "Sarah the Crystal Crafter"
- **Background**: Hobbyist with UV laser access (makerspace/personal)
- **Technical Skill**: Comfortable with photo editing, basic 3D printer slicing
- **Pain Points**: 
  - Existing 3D software too complex (Blender learning curve)
  - Commercial depth-map tools expensive or subscription-based
  - Generic STL converters produce poor results for laser engraving
- **Goals**:
  - Convert family photos to crystal gifts
  - Experiment with artistic depth effects
  - Achieve professional results without professional tools
- **Success Scenario**: Loads wedding photo, adjusts depth curve, exports STL, engraves beautiful crystal in one afternoon

### 3.2 Secondary Persona: "Mike the Maker"
- **Background**: DIY enthusiast building custom laser engraver
- **Technical Skill**: Python scripting, basic Rust knowledge
- **Pain Points**:
  - Needs customization for non-standard laser setup
  - Wants to understand and modify depth algorithms
  - Privacy-conscious about cloud processing
- **Goals**:
  - Fine-tune output for specific material properties
  - Contribute improvements back to project
  - Integrate with custom toolchain
- **Success Scenario**: Forks repo, adds custom export format, submits PR, uses in production workflow

### 3.3 Tertiary Persona: "Alex the Artist"
- **Background**: Digital artist exploring new mediums
- **Technical Skill**: Proficient with Photoshop/GIMP, vector tools
- **Pain Points**:
  - Wants artistic control over depth interpretation
  - Needs to preview results before committing to engraving
  - Limited budget for specialized software
- **Goals**:
  - Create original artwork specifically for 3D engraving
  - Experiment with layering and depth effects
  - Build portfolio of crystal art pieces
- **Success Scenario**: Designs custom illustration, uses advanced mode to sculpt depth map, creates signature style

---

## 4. Feature Requirements

### 4.1 MVP Features (Phase 1)

#### F1.1 Image Import
**Priority:** P0 (Critical)  
**Description:** Load single image file for processing  
**Acceptance Criteria:**
- Supports PNG, JPG formats (8-bit and 16-bit)
- Maximum image size: 8192×8192 pixels
- Automatic downsampling with user notification if exceeded
- Drag-and-drop support
- File browser with image preview

**Technical Notes:**
- Use `image` crate (Rust) for loading
- Validate file integrity before processing
- Display image dimensions and file size in UI

---

#### F1.2 AI Depth Estimation
**Priority:** P0 (Critical)  
**Description:** Generate depth map from 2D image using neural network  
**Acceptance Criteria:**
- Support Depth-Anything-V2 or MiDaS models
- Process on CPU or GPU (auto-detect)
- Display progress indicator during inference
- Normalize depth output to 0-1 range
- Allow model selection (if multiple downloaded)

**Technical Notes:**
- Python backend via PyTorch
- Rust-Python bridge using PyO3 or subprocess
- Model files stored in `~/.simplepicture3d/models/`
- Fallback error handling if model missing

**Dependencies:**
- Model installer (F1.8)
- Performance within 16GB RAM constraint

---

#### F1.3 Manual Depth Adjustment
**Priority:** P0 (Critical)  
**Description:** Basic controls to modify AI-generated depth map  
**Acceptance Criteria:**
- Depth range slider (2-10mm output)
- Brightness/contrast adjustment
- Gamma curve control
- Invert depth toggle
- Real-time preview of adjustments

**Technical Notes:**
- Apply transforms to normalized depth array
- Use GPU shaders for preview if available
- Preserve original depth map for reset function

---

#### F1.4 3D Preview
**Priority:** P0 (Critical)  
**Description:** Visualize mesh before export  
**Acceptance Criteria:**
- Render point cloud or mesh representation
- Rotate/zoom/pan camera controls
- Toggle wireframe/solid/point modes
- Display mesh statistics (vertex count, dimensions)
- Update preview on "Generate" button click

**Technical Notes:**
- Use wgpu or three.js (via Tauri webview)
- LOD (Level of Detail) for large meshes
- Orthographic and perspective camera modes

---

#### F1.5 Mesh Generation
**Priority:** P0 (Critical)  
**Description:** Convert depth map to 3D point cloud/mesh  
**Acceptance Criteria:**
- Generate point cloud with density control
- Vertex positions in millimeters (real-world scale)
- Z-axis depth mapped to 2-10mm range
- Mesh topology suitable for laser engraving (no overhangs)
- Process completes in <2 minutes for 4K image

**Technical Notes:**
- Rust implementation for performance
- Sampling strategies: uniform grid, adaptive density
- Optional Delaunay triangulation for mesh connectivity
- Memory-efficient streaming for large images

**Performance Targets:**
- 1920×1080 image: <15 seconds
- 3840×2160 image: <90 seconds
- 16GB RAM sufficient for 8K images

---

#### F1.6 STL/OBJ Export
**Priority:** P0 (Critical)  
**Description:** Save generated mesh to standard 3D file formats  
**Acceptance Criteria:**
- Export binary STL format
- Export ASCII OBJ format with material file
- Default save location: `~/Documents/SimplePicture3D/exports/`
- Remember last export location
- Filename auto-generated from source image + timestamp
- User can override filename before export
- Progress indicator for large files

**Technical Notes:**
- Use `stl_io` crate for STL writing
- Validate mesh integrity before export
- Option to include metadata (generator, date, source image)

---

#### F1.7 Basic UI/UX
**Priority:** P0 (Critical)  
**Description:** Functional workspace interface  
**Acceptance Criteria:**
- Single-window application
- Left sidebar: image import and controls
- Center: preview panel
- Right sidebar: depth adjustment controls
- Bottom: export button and status bar
- Window resizable, minimum 1024×768

**Technical Notes:**
- Tauri + Svelte/React frontend
- Responsive layout using CSS Grid/Flexbox
- Keyboard shortcuts for common actions
- Tooltips on hover for all controls

---

#### F1.8 Installation & Setup
**Priority:** P0 (Critical)  
**Description:** User installs application and optionally downloads AI models  
**Acceptance Criteria:**
- Native installer for Windows (.msi/.exe)
- DMG installer for macOS
- AppImage or .deb for Linux
- First-run wizard offers:
  - Model download (local/skip)
  - Default export location selection
  - Privacy notice (100% offline processing)
- Installation size: <500MB without models, <3GB with models
- No admin rights required (user-level install)

**Technical Notes:**
- Model downloader with resume capability
- Verify model checksums (SHA256)
- Graceful degradation if models not installed
- Uninstaller removes all app data

---

### 4.2 Enhanced Features (Phase 2)

#### F2.1 Advanced Depth Controls
**Priority:** P1 (High)  
**Description:** Power user tools for precise depth manipulation  
**Acceptance Criteria:**
- Histogram of depth distribution
- Curves tool (Photoshop-style)
- Masking/selection tools for regional adjustments
- Layer blending (combine AI + manual heightmap)
- Depth gradient brushes

**User Story:** As an artist, I want to manually paint depth in specific areas so that I can emphasize facial features while flattening backgrounds.

---

#### F2.2 Batch Processing
**Priority:** P2 (Medium)  
**Description:** Process multiple images with same settings  
**Acceptance Criteria:**
- Queue multiple input files
- Apply preset or last-used settings
- Progress indicator for batch queue
- Individual error handling (skip failed, continue)
- Export all to designated folder

**User Story:** As a small business owner, I want to process 20 customer photos overnight so that I can prepare batch engravings efficiently.

**Note:** Deferred to post-Phase 4 based on initial requirements.

---

#### F2.3 Presets & Templates
**Priority:** P1 (High)  
**Description:** Save and load processing configurations  
**Acceptance Criteria:**
- Save current settings as named preset
- Load preset applies all depth/mesh parameters
- Built-in presets: "Portrait", "Landscape", "High Detail", "Low Relief"
- Import/export presets as JSON
- Preset manager UI (list, rename, delete)

**Technical Notes:**
- Presets stored in `~/.simplepicture3d/presets/`
- Versioned schema for forward compatibility

---

#### F2.4 Undo/Redo System
**Priority:** P1 (High)  
**Description:** Revert and reapply editing actions  
**Acceptance Criteria:**
- Undo last 20 actions
- Redo previously undone actions
- Keyboard shortcuts (Ctrl+Z, Ctrl+Y)
- Action history panel (optional)
- Clear history on new image load

**Technical Notes:**
- Command pattern for all mutable operations
- Serialize state diffs for memory efficiency
- Disable undo/redo during mesh generation

---

#### F2.5 Enhanced 3D Preview
**Priority:** P1 (High)  
**Description:** Improved visualization and inspection tools  
**Acceptance Criteria:**
- Realistic lighting modes (diffuse, specular)
- Measure tool (distance between points)
- Cross-section view (slice at depth)
- Export preview as image
- Side-by-side original/depth comparison

**Technical Notes:**
- PBR (Physically Based Rendering) shaders
- Screenshot to PNG/JPG

---

#### F2.6 Material Presets
**Priority:** P2 (Medium)  
**Description:** Optimize output for specific engraving materials  
**Acceptance Criteria:**
- Material library: K9 Crystal, Glass, Acrylic
- Each preset adjusts: depth range, point density, base thickness
- User can create custom materials
- Material notes/documentation embedded

**User Story:** As a maker, I want to select "K9 Crystal - Small" preset so that the output parameters match my laser's capabilities without manual calculation.

---

### 4.3 Cross-Platform Features (Phase 3)

#### F3.1 macOS Native Support
**Priority:** P0 (Critical for Phase 3)  
**Description:** Full feature parity on macOS  
**Acceptance Criteria:**
- DMG installer with code signing
- Native menu bar integration
- Retina display support
- Metal GPU acceleration (if available)
- Keyboard shortcuts match macOS conventions
- Tested on Intel and Apple Silicon

---

#### F3.2 Linux Distribution
**Priority:** P0 (Critical for Phase 3)  
**Description:** Full feature parity on Linux  
**Acceptance Criteria:**
- AppImage (universal binary)
- .deb package (Debian/Ubuntu)
- Flatpak support (optional)
- Tested on: Ubuntu 22.04+, Fedora 38+
- GPU support via Vulkan
- Desktop file integration (app launcher)

---

#### F3.3 Platform-Specific Optimizations
**Priority:** P1 (High)  
**Description:** Leverage OS-specific capabilities  
**Acceptance Criteria:**
- GPU auto-detection per platform (DirectX/Metal/Vulkan)
- Native file dialogs
- OS notifications for long-running tasks
- Drag-and-drop from system file browsers
- Platform-specific installer behaviors

---

### 4.4 Production Polish (Phase 4)

#### F4.1 Performance Optimization
**Priority:** P0 (Critical for Phase 4)  
**Description:** Meet all performance targets  
**Acceptance Criteria:**
- Startup time <3 seconds
- UI responsive <16ms frame time
- Memory usage <2GB for typical workflow
- Multi-threading for mesh generation
- GPU acceleration for preview rendering
- Profiling results documented

**Technical Notes:**
- Rust async runtime (Tokio) for non-blocking operations
- Web Workers in frontend for heavy computations
- Lazy loading of AI models

---

#### F4.2 Error Handling & Recovery
**Priority:** P0 (Critical for Phase 4)  
**Description:** Graceful failure and user guidance  
**Acceptance Criteria:**
- All errors display user-friendly messages
- Automatic crash reporting (opt-in)
- Recovery from corrupted settings
- Validate inputs before processing
- Logging system for debugging

**Error Categories:**
- File I/O errors (permissions, corruption)
- Model inference failures (OOM, unsupported format)
- Mesh generation errors (invalid topology)
- Export failures (disk full, write protected)

**Technical Notes:**
- Use `anyhow` crate for Rust error handling
- Frontend error boundaries (React/Svelte)
- Log files in `~/.simplepicture3d/logs/`

---

#### F4.3 Comprehensive Documentation
**Priority:** P0 (Critical for Phase 4)  
**Description:** User manual and developer docs  
**Acceptance Criteria:**
- User Guide: installation, first conversion, FAQ
- Developer Guide: build instructions, architecture, contributing
- API documentation (if extensible)
- Video tutorials (3-5 minutes each)
- Inline help tooltips
- README with badges (build status, license, etc.)

**Deliverables:**
- `docs/user-guide.md`
- `docs/developer-guide.md`
- `CONTRIBUTING.md`
- `README.md`
- Video scripts and screen recordings

---

#### F4.4 Automated Testing
**Priority:** P0 (Critical for Phase 4)  
**Description:** Comprehensive test coverage  
**Acceptance Criteria:**
- Unit tests: >80% coverage for Rust core
- Integration tests: full pipeline (image → STL)
- UI tests: critical user paths automated
- Performance benchmarks: CI regression detection
- Manual test plans: OS-specific behaviors

**Test Infrastructure:**
- Rust: `cargo test`
- Python: `pytest`
- Frontend: Vitest or Jest
- CI/CD: GitHub Actions

---

#### F4.5 Installer & Packaging
**Priority:** P0 (Critical for Phase 4)  
**Description:** Professional distribution packages  
**Acceptance Criteria:**
- Code-signed installers (Windows, macOS)
- Auto-update mechanism (Tauri updater)
- Silent install option for enterprise
- Uninstaller cleans all data
- Size optimization: <100MB core app
- Installation analytics (opt-in)

**Platforms:**
- Windows: MSI and EXE (NSIS)
- macOS: DMG with background image
- Linux: AppImage, .deb, Flatpak

---

#### F4.6 Accessibility
**Priority:** P1 (High)  
**Description:** Usable by people with disabilities  
**Acceptance Criteria:**
- Keyboard navigation for all features
- Screen reader compatible (ARIA labels)
- High contrast mode
- Adjustable UI font sizes
- Color-blind friendly palette
- WCAG 2.1 AA compliance

---

#### F4.7 Internationalization (i18n)
**Priority:** P2 (Medium - Future)  
**Description:** Multi-language support  
**Acceptance Criteria:**
- UI strings externalized
- Initial languages: English
- Framework for community translations
- RTL layout support (future)

**Note:** English-only for v1.0, i18n infrastructure in place.

---

## 5. Technical Architecture

### 5.1 Technology Stack

#### Frontend (UI)
- **Framework:** Tauri (Rust wrapper for web technologies)
- **Web Stack:** Svelte or React + TypeScript
- **Styling:** TailwindCSS or CSS Modules
- **3D Rendering:** Three.js or Babylon.js (via WebGL)
- **State Management:** Svelte stores or Zustand (React)

#### Backend (Core Logic)
- **Primary Language:** Rust (mesh generation, file I/O, performance-critical)
- **AI/ML:** Python (PyTorch, depth estimation models)
- **Inter-Process Communication:** Tauri commands (Rust ↔ JS) + subprocess for Python
- **Async Runtime:** Tokio (Rust)

#### Data Processing
- **Image Loading:** `image` crate (Rust)
- **Depth Map Processing:** NumPy arrays (Python), passed to Rust via JSON or binary
- **Mesh Generation:** Custom Rust implementation
- **File Export:** `stl_io`, `obj` crates (Rust)

#### AI Models
- **Depth Estimation:** Depth-Anything-V2 (recommended) or MiDaS v3.1
- **Model Format:** ONNX (optimized inference) or PyTorch
- **Inference Engine:** PyTorch or ONNXRuntime

### 5.2 System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Tauri Frontend                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ Svelte/React │  │  Three.js    │  │  TailwindCSS │  │
│  │   Components │  │  3D Preview  │  │    Styling   │  │
│  └──────┬───────┘  └──────┬───────┘  └──────────────┘  │
│         │                 │                             │
│         └─────────┬───────┘                             │
│                   │ Tauri Commands (IPC)                │
└───────────────────┼─────────────────────────────────────┘
                    │
┌───────────────────┼─────────────────────────────────────┐
│                   ▼         Rust Backend                │
│  ┌────────────────────────────────────────────────────┐ │
│  │              Core Application Logic                │ │
│  │  • Image Loading (image crate)                     │ │
│  │  • Depth Map Processing (ndarray)                  │ │
│  │  • Mesh Generation (custom algorithms)             │ │
│  │  • File I/O (STL/OBJ export)                       │ │
│  └────────┬───────────────────────────┬────────────────┘ │
│           │                           │                  │
│  ┌────────▼──────────┐       ┌────────▼──────────┐      │
│  │  Python Bridge    │       │   Settings & State│      │
│  │  (PyO3/subprocess)│       │   (serde JSON)    │      │
│  └────────┬──────────┘       └───────────────────┘      │
└───────────┼──────────────────────────────────────────────┘
            │
┌───────────▼──────────────────────────────────────────────┐
│                  Python AI Backend                       │
│  ┌──────────────────────────────────────────────────┐   │
│  │           Depth Estimation Models                │   │
│  │  • PyTorch runtime                               │   │
│  │  • Depth-Anything-V2 / MiDaS                     │   │
│  │  • CUDA/Metal/CPU inference                      │   │
│  └──────────────────────────────────────────────────┘   │
│                                                          │
│  Input: Image bytes → Output: Depth map (NumPy/JSON)    │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│                    File System                           │
│  ~/.simplepicture3d/                                     │
│    ├── models/           (AI models)                     │
│    ├── presets/          (User presets JSON)             │
│    ├── logs/             (Application logs)              │
│    └── cache/            (Temporary processing)          │
│                                                          │
│  ~/Documents/SimplePicture3D/                            │
│    └── exports/          (Default output location)       │
└──────────────────────────────────────────────────────────┘
```

### 5.3 Data Flow

**Image to STL Pipeline:**

1. **User Loads Image** (Frontend)
   - File picker dialog → Image file path
   - Display preview in UI

2. **Image Validation** (Rust)
   - Check format, dimensions, file integrity
   - Downsample if >8K resolution
   - Convert to RGB if needed

3. **Depth Estimation** (Python subprocess)
   - Rust sends image bytes via stdin or temp file
   - Python loads model, runs inference
   - Returns depth map as JSON array or binary buffer

4. **Depth Map Processing** (Rust)
   - Apply user adjustments (gamma, range, invert)
   - Normalize to 2-10mm depth range
   - Send processed depth to frontend for preview

5. **Mesh Generation** (Rust)
   - Sample depth map based on density setting
   - Create vertex array (x, y, z coordinates)
   - Optional: Triangulate for solid mesh
   - Calculate normals if needed

6. **Preview Rendering** (Frontend - Three.js)
   - Receive vertex data via Tauri command
   - Create BufferGeometry
   - Render with orbit controls

7. **Export** (Rust)
   - User clicks "Export"
   - Write binary STL or ASCII OBJ
   - Save to user-selected location
   - Update "last export path" in settings

### 5.4 File Structure

```
SimplePicture3D/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Tauri app entry point
│   │   ├── commands.rs     # IPC command handlers
│   │   ├── image_processing.rs
│   │   ├── mesh_generator.rs
│   │   ├── depth_map.rs
│   │   ├── exporters/
│   │   │   ├── stl.rs
│   │   │   └── obj.rs
│   │   └── python_bridge.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                    # Frontend (Svelte/React)
│   ├── components/
│   │   ├── ImageImport.svelte
│   │   ├── DepthControls.svelte
│   │   ├── Preview3D.svelte
│   │   └── ExportPanel.svelte
│   ├── stores/             # State management
│   ├── lib/                # Utilities
│   └── App.svelte
│
├── python/                 # Python AI backend
│   ├── depth_estimator.py  # Main inference script
│   ├── models/             # Model loading utilities
│   ├── requirements.txt
│   └── setup.py
│
├── tests/
│   ├── rust/               # Cargo tests
│   ├── python/             # Pytest tests
│   └── integration/        # End-to-end tests
│
├── docs/
│   ├── user-guide.md
│   ├── developer-guide.md
│   ├── architecture.md
│   └── api/                # Generated API docs
│
├── scripts/
│   ├── build.sh            # Build automation
│   ├── test.sh             # Test runner
│   └── package.sh          # Create installers
│
├── .github/
│   └── workflows/
│       ├── ci.yml          # CI/CD pipeline
│       └── release.yml     # Release automation
│
├── README.md
├── LICENSE                 # MIT License
├── CONTRIBUTING.md
└── package.json            # Frontend dependencies
```

### 5.5 Performance Considerations

#### Memory Management
- **Image Loading:** Stream large images in chunks, avoid loading entire file into RAM
- **Depth Maps:** Use 32-bit float arrays, release after mesh generation
- **Mesh Data:** Vertex arrays stored in Rust, minimal copies to frontend
- **Model Inference:** PyTorch set to inference mode, clear cache after use

**Memory Budget (Target):**
- Application binary: <100MB
- AI models: 500MB - 2GB (user's choice)
- Runtime (idle): <200MB
- Runtime (processing 4K image): <2GB
- Runtime (processing 8K image): <8GB

#### CPU/GPU Utilization
- **Depth Estimation:** GPU preferred (CUDA/Metal), CPU fallback
- **Mesh Generation:** Multi-threaded (Rayon in Rust)
- **Preview Rendering:** WebGL 2.0, GPU accelerated
- **Export:** Async I/O, non-blocking

#### Disk I/O
- **Model Storage:** User's home directory, SSD recommended
- **Temporary Files:** OS temp directory, cleaned on exit
- **Export:** Direct write to target location, no intermediate copies

### 5.6 Security & Privacy

#### Data Privacy
- **100% Offline Processing:** All computation local by default
- **No Telemetry:** Usage data not collected (opt-in crash reports only)
- **No Cloud Dependencies:** AI models downloaded once, cached locally
- **User Data Isolation:** Each user's settings/models in their home directory

#### Code Security
- **Dependencies:** Regular audit via `cargo audit` and `npm audit`
- **Input Validation:** All user inputs sanitized (file paths, image data)
- **Safe Rust:** Minimize `unsafe` code blocks, justify when used
- **Sandboxing:** Tauri's built-in security model (limited IPC surface)

#### Model Integrity
- **Checksum Verification:** SHA256 hash validation on model download
- **Signed Releases:** Code-signed installers to prevent tampering
- **Reproducible Builds:** CI builds from tagged commits

#### Privacy Notice (First Run)
```
SimplePicture3D processes all images on your computer.
Your images, depth maps, and meshes never leave your device.

Optional features:
□ Download AI models from Hugging Face (one-time, ~1.5GB)
□ Send anonymous crash reports to help improve the app

You can change these settings anytime in Preferences.
```

---

## 6. UI/UX Requirements

### 6.1 Design Principles
1. **Clarity Over Cleverness:** Obvious controls beat hidden gestures
2. **Progressive Disclosure:** Show basics, reveal advanced on demand
3. **Immediate Feedback:** Every action has visual confirmation
4. **Reversibility:** Undo always available for destructive actions
5. **Consistent Patterns:** Same interaction models across features

### 6.2 Visual Design

#### Color Palette
- **Primary:** Deep blue (#2563EB) - CTAs and highlights
- **Secondary:** Slate gray (#64748B) - UI chrome
- **Accent:** Amber (#F59E0B) - Warnings and notifications
- **Success:** Green (#10B981) - Confirmations
- **Error:** Red (#EF4444) - Errors and destructive actions
- **Background:** Light (#F8FAFC) / Dark (#0F172A) modes

#### Typography
- **Headings:** Inter or system-ui, 16-24px
- **Body:** Inter or system-ui, 14px
- **Monospace:** JetBrains Mono or Fira Code (logs, technical info)

#### Spacing & Layout
- 8px grid system
- 16px padding for cards/panels
- 24px margins between major sections

### 6.3 Workspace Layout

**Default View (MVP):**
```
┌────────────────────────────────────────────────────────────────┐
│  File  Edit  View  Help                                  [_][□][×] │
├───────────┬────────────────────────────────────┬───────────────┤
│           │                                    │               │
│  Image    │         3D Preview                 │  Depth        │
│  Import   │                                    │  Controls     │
│           │    [Rotate │ Zoom │ Pan]           │               │
│  [+Load]  │                                    │  Depth Range  │
│           │                                    │  ├──●────────┤│
│  Preview: │                                    │  2mm    10mm  │
│  ┌──────┐ │                                    │               │
│  │ img  │ │                                    │  Brightness   │
│  └──────┘ │                                    │  ├────●──────┤│
│           │                                    │               │
│           │                                    │  Invert Depth │
│           │                                    │  □            │
│           │                                    │               │
│           │                                    │  [Generate]   │
│           │                                    │               │
├───────────┴────────────────────────────────────┴───────────────┤
│  Status: Ready │ Vertices: 0 │ Export: [STL ▼] [Export...]    │
└────────────────────────────────────────────────────────────────┘
```

**Advanced Mode Toggle:**
- Adds "Advanced" expander in Depth Controls
- Reveals: Histogram, Curves, Masking tools
- Same layout, more controls visible

### 6.4 Key User Flows

#### Flow 1: First Conversion (Happy Path)
1. Launch app → First-run wizard
2. Choose model download option
3. Land in main workspace
4. Click "Load Image" → File picker → Select photo
5. Preview displays in left panel
6. Automatic depth estimation starts (progress bar)
7. 3D preview updates when complete
8. User adjusts depth range slider (2mm → 5mm)
9. Clicks "Generate" → mesh recalculates
10. Preview updates with new depth
11. Clicks "Export..." → File dialog
12. Selects location → Saves "my_image_2026-01-31_143022.stl"
13. Success notification with "Open Folder" button

**Time to complete:** <5 minutes (excluding model download)

#### Flow 2: Advanced Editing
1. Load image (as above)
2. Click "Advanced Mode" toggle
3. Histogram appears showing depth distribution
4. User paints mask over background
5. Applies flatten adjustment to masked area
6. Uses curves tool to enhance midtone depth
7. Generates mesh
8. Exports with custom filename

**Time to complete:** 10-15 minutes

#### Flow 3: Model Download (First Run)
1. App detects no models installed
2. Dialog: "Download AI model for depth estimation?"
   - Option 1: Download now (recommended)
   - Option 2: Skip (manual depth only)
3. User selects "Download now"
4. Progress bar with speed/ETA
5. Download completes → checksum verified
6. "Ready to use!" confirmation
7. Proceeds to main workspace

**Download time:** 5-20 minutes (depends on connection)

### 6.5 Accessibility Requirements
- **Keyboard Navigation:** Tab order logical, Enter/Space activate
- **Focus Indicators:** Visible outline on focused elements
- **Screen Reader Labels:** All inputs and buttons have ARIA labels
- **Color Independence:** Information not conveyed by color alone
- **Contrast Ratios:** WCAG AA compliant (4.5:1 text, 3:1 UI)

---

## 7. Performance & Quality Metrics

### 7.1 Performance Targets

| Operation | Target | Max Acceptable | Test Hardware |
|-----------|--------|----------------|---------------|
| App Startup | <2s | 5s | Mid-range SSD, 8GB RAM |
| Image Load (4K PNG) | <500ms | 1s | - |
| Depth Estimation (4K, GPU) | <10s | 30s | NVIDIA GTX 1060 / Apple M1 |
| Depth Estimation (4K, CPU) | <60s | 120s | Intel i5-8400 / AMD Ryzen 5 |
| Mesh Generation (4K) | <15s | 30s | - |
| Preview Render (100K vertices) | 60fps | 30fps | Integrated GPU |
| STL Export (1M vertices) | <5s | 15s | - |
| Memory (Idle) | <200MB | 500MB | - |
| Memory (Processing 4K) | <2GB | 4GB | - |

### 7.2 Quality Metrics

#### Code Quality
- **Rust:** `clippy` lints passing, `rustfmt` formatted
- **Python:** `flake8` or `ruff`, `black` formatted
- **TypeScript:** ESLint passing, Prettier formatted
- **Test Coverage:** >80% for core logic, >60% overall

#### Output Quality
- **STL Validity:** 100% pass in MeshLab or Netfabb
- **Mesh Integrity:** No degenerate triangles, manifold topology
- **Dimension Accuracy:** ±0.1mm from specified depth range
- **File Size:** Reasonable for vertex count (no bloat)

#### User Experience
- **First Success Rate:** >90% complete first conversion without errors
- **Error Recovery:** <5% abandon app after error (graceful messaging)
- **Documentation Lookup:** <10% need external help for basic tasks

### 7.3 Monitoring & Telemetry

**Opt-In Crash Reporting:**
- Anonymous stack traces via Sentry or similar
- OS version, app version, action leading to crash
- No PII or user images included

**Development Metrics:**
- GitHub Actions: Build time, test pass rate
- Code coverage trends
- Dependency vulnerability scans

---

## 8. Security & Privacy

### 8.1 Threat Model

#### Assets to Protect
1. **User Images:** Potentially personal photos, proprietary artwork
2. **User Data:** Export locations, preferences, presets
3. **Application Integrity:** Prevent malware injection, tampering

#### Threats
1. **Privacy Violation:** User images leaked to third parties
2. **Malicious Models:** Tampered AI models execute malicious code
3. **Path Traversal:** User exports overwrite system files
4. **Dependency Vulnerabilities:** Exploitable bugs in libraries

### 8.2 Security Controls

#### Input Validation
- **File Paths:** Canonicalize, prevent directory traversal
- **Image Files:** Validate magic bytes, reject executable formats
- **User Text:** Sanitize filenames, escape special characters

#### Dependency Management
- **Rust Crates:** `cargo audit` in CI, Dependabot alerts enabled
- **NPM Packages:** `npm audit`, lock file committed
- **Python Packages:** `pip-audit`, pinned versions in `requirements.txt`

#### Model Security
- **Download Source:** Official Hugging Face repositories only
- **Checksum Verification:** SHA256 hashes compared against known-good
- **Model Sandboxing:** Python subprocess isolated, no filesystem access beyond models directory

#### Code Signing
- **Windows:** Authenticode signature via DigiCert or similar
- **macOS:** Developer ID certificate, notarization
- **Linux:** GPG-signed checksums for AppImage/deb

### 8.3 Privacy Guarantees

**User Bill of Rights:**
1. Your images never leave your device
2. No cloud processing, no external API calls
3. No telemetry without your explicit consent
4. Open source code auditable by anyone
5. No ads, no tracking, no monetization of your data

**Privacy Policy (embedded in app):**
```
SimplePicture3D Privacy Statement

Data Collection: None
SimplePicture3D does not collect, transmit, or store any 
user data outside your local device. All image processing 
occurs on your computer.

AI Models: Downloaded once from Hugging Face, cached locally.
No usage data sent to model providers.

Crash Reports (Opt-In): If enabled, anonymous crash logs 
are sent to help us fix bugs. No images or personal data 
are included.

Third-Party Access: None
This software does not integrate with any third-party 
analytics, advertising, or cloud services.

Your Rights: You own your data
All files created by SimplePicture3D are yours. Delete the 
app to remove all associated data from your system.
```

---

## 9. Licensing & Legal

### 9.1 Software License

**Primary License:** MIT License

**Rationale:**
- Permissive, allows commercial use
- Compatible with most open-source libraries
- Minimal restrictions, fosters adoption
- Users can fork and create derivative works

**License Header (all source files):**
```
// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT
```

### 9.2 Dependency Licenses

**Compatibility Matrix:**

| Dependency | License | Compatible with MIT? | Notes |
|------------|---------|----------------------|-------|
| Tauri | MIT | ✅ Yes | - |
| Rust crates (most) | MIT/Apache-2.0 | ✅ Yes | Dual-licensed common |
| PyTorch | BSD-3-Clause | ✅ Yes | - |
| Depth-Anything-V2 | Apache-2.0 | ✅ Yes | Model weights CC-BY-NC-4.0 |
| MiDaS | MIT | ✅ Yes | Model weights for non-commercial |
| Three.js | MIT | ✅ Yes | - |
| Svelte | MIT | ✅ Yes | - |

**Copyleft Dependencies:** None permitted in core application (GPL, AGPL would require relicensing)

**Model Weights Notice:**
- Depth-Anything-V2 weights: CC-BY-NC-4.0 (non-commercial)
- **Action Required:** Clarify commercial use policy or offer commercial-friendly model option

### 9.3 Third-Party Notices

**ATTRIBUTION.md** (shipped with app):
```
SimplePicture3D incorporates the following open-source software:

Tauri (MIT) - https://tauri.app
PyTorch (BSD-3-Clause) - https://pytorch.org
Depth-Anything-V2 (Apache-2.0) - Model weights CC-BY-NC-4.0
Three.js (MIT) - https://threejs.org
[... full list with versions and licenses ...]

Full license texts available in the 'licenses/' directory.
```

### 9.4 Contribution Agreement

**CLA:** Not required for MIT (implicit grant via PR)

**CONTRIBUTING.md Excerpt:**
```
By submitting a pull request, you agree that your contributions 
will be licensed under the MIT License. You confirm that you have 
the right to license your contributions under these terms.
```

### 9.5 Trademark

**Name:** "SimplePicture3D" (no trademark registration initially)

**Logo:** Original artwork, CC-BY-SA-4.0 (allow derivatives)

**Usage Policy:**
- Open-source forks may use name/logo with attribution
- Commercial derivatives should rebrand to avoid confusion

---

## 10. Success Criteria

### 10.1 MVP Success (Phase 1)
- ✅ Application builds on Windows without errors
- ✅ User can load PNG/JPG, generate depth map, export STL
- ✅ AI model download completes successfully
- ✅ Exported STL opens in Meshmixer/PrusaSlicer without errors
- ✅ 5+ beta testers complete first conversion successfully
- ✅ GitHub repository public with README and LICENSE

### 10.2 Beta Success (Phase 2-3)
- ✅ 50+ GitHub stars
- ✅ macOS and Linux builds functional
- ✅ 90% of beta testers rate UI "intuitive" or better
- ✅ Advanced mode used by >30% of users
- ✅ Zero critical bugs in 2-week testing period
- ✅ Documentation complete (user guide + dev guide)

### 10.3 v1.0 Release Success (Phase 4)
- ✅ 500+ downloads in first month
- ✅ 4.0+ average rating (if listed on platform)
- ✅ 10+ community contributions (PRs, issues, docs)
- ✅ Performance targets met on reference hardware
- ✅ No unresolved security vulnerabilities
- ✅ Featured in >2 maker/laser engraving communities

### 10.4 Long-Term Success (6-12 months post-release)
- ✅ 1,000+ GitHub stars
- ✅ Active community forum or Discord
- ✅ Commercial laser shops using in production
- ✅ 3+ derivative projects or plugins
- ✅ Sustainable donation funding for hosting/development

---

## 11. Out of Scope (Future Expansion)

### 11.1 Explicitly Deferred Features
The following are valuable but not required for v1.0:

1. **Batch Processing** - Process multiple images in queue
2. **Multi-Layer Engraving** - Separate depth layers by color channel
3. **Toolpath Generation** - Direct G-code export for lasers
4. **Material Simulation** - Realistic crystal preview with light refraction
5. **Cloud Sync** - Backup presets/projects to cloud storage
6. **Mobile App** - iOS/Android companion for preview
7. **Plugin System** - Third-party extensions for custom algorithms
8. **Video Input** - Convert video frames to animated depth sequences
9. **3D Scanning Integration** - Import depth maps from structured light scanners
10. **Commercial Material Library** - Paid presets from professional engravers

### 11.2 Non-Goals
These will NOT be pursued:

- ❌ Full 3D modeling suite (use Blender instead)
- ❌ Laser hardware control (safety/liability concerns)
- ❌ Image editing beyond depth adjustment (use GIMP/Photoshop)
- ❌ Marketplace for selling models (focus on creation tool)
- ❌ Social features (sharing/comments/likes)

---

## 12. Risks & Mitigation

### 12.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Python-Rust IPC performance bottleneck | Medium | High | Benchmark early, consider ONNX runtime in Rust |
| AI model size exceeds user tolerance | Low | Medium | Offer quantized models, progressive download |
| Cross-platform GPU bugs | High | Medium | Fallback to CPU, extensive testing matrix |
| Memory overflow on 8K images | Medium | High | Stream processing, downsampling warnings |
| Tauri security vulnerability | Low | High | Subscribe to security advisories, rapid patching |

### 12.2 User Adoption Risks

| Risk | Mitigation |
|------|------------|
| "Too complicated for hobbyists" | Usability testing, default presets, video tutorials |
| "AI models too large to download" | Offer manual-only mode, cloud option (future) |
| "Output doesn't work with my laser" | Validate STL spec compliance, gather user feedback |
| "No support/documentation" | Comprehensive docs, active GitHub Discussions |

### 12.3 Community Risks

| Risk | Mitigation |
|------|------------|
| Toxic community members | Code of conduct, active moderation |
| Low contributor engagement | Good-first-issue labels, clear CONTRIBUTING.md |
| Competing fork fragments community | Encourage collaboration, merge useful PRs |
| Burnout of core maintainers | Distribute responsibilities, onboard co-maintainers |

---

## 13. Appendices

### 13.1 Glossary

- **2.5D:** Representation where each (x,y) coordinate has single depth (z) value (unlike full 3D)
- **Depth Map:** Grayscale image where brightness represents distance from camera
- **Heightmap:** Similar to depth map, used for terrain or relief modeling
- **K9 Crystal:** Optical-grade glass commonly used for laser engraving (K9 ≈ BK7)
- **Manifold Mesh:** 3D mesh with no holes, valid for physical fabrication
- **Point Cloud:** Set of vertices in 3D space without connectivity (faces/edges)
- **STL:** STereoLithography file format, de facto standard for 3D printing/engraving
- **UV Laser:** Ultraviolet laser capable of internal engraving in transparent materials

### 13.2 References

**AI Models:**
- Depth-Anything-V2: https://github.com/DepthAnything/Depth-Anything-V2
- MiDaS: https://github.com/isl-org/MiDaS

**Technologies:**
- Tauri: https://tauri.app
- Rust: https://www.rust-lang.org
- PyTorch: https://pytorch.org
- Three.js: https://threejs.org

**Standards:**
- STL Format Spec: https://en.wikipedia.org/wiki/STL_(file_format)
- OBJ Format Spec: https://en.wikipedia.org/wiki/Wavefront_.obj_file

**Laser Engraving:**
- K9 Crystal Properties: [Material safety data sheets]
- UV Laser Safety: ANSI Z136.1 standards

### 13.3 Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-31 | AI Assistant + User | Initial PRD creation |

---

**END OF DOCUMENT**

*This PRD is a living document and will be updated as requirements evolve. For questions or clarifications, please open a GitHub Discussion in the SimplePicture3D repository.*
