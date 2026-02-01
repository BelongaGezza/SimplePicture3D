# Python & ML Stack

**Purpose:** Python, PyTorch, and depth estimation models for SimplePicture3D.

## Official Sources

| Source | URL | Last Checked |
|--------|-----|--------------|
| PyTorch (Get Started) | https://pytorch.org/get-started/locally/ | 2026-02-01 |
| PyTorch Docs | https://pytorch.org/docs/stable/index.html | 2026-02-01 |
| Depth-Anything-V2 (GitHub) | https://github.com/DepthAnything/Depth-Anything-V2 | 2026-02-01 |
| Depth-Anything-V2 (Hugging Face) | https://huggingface.co/depth-anything/Depth-Anything-V2-Base | 2026-02-01 |
| MiDaS (GitHub) | https://github.com/isl-org/MiDaS | 2026-02-01 |
| Hugging Face | https://huggingface.co/ | 2026-02-01 |
| ONNX Runtime (Python) | https://onnxruntime.ai/docs/get-started/with-python.html | 2026-02-01 |

---

## PyTorch

- **Current stable:** 2.7.0 (as of early 2026).
- **Python:** Minimum **Python 3.10**; supported 3.10, 3.11, 3.12 (and up to 3.14 on macOS per docs).
- **Recommendation:** Use **Python 3.10+** for this project.

### Install (official)

- **CPU:**  
  `pip3 install torch torchvision torchaudio`
- **CUDA (example 11.8):**  
  `pip3 install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118`  
  (Other indices: cu121, cu124, etc. — check get-started page.)
- **macOS (Metal/CPU):**  
  `pip3 install torch torchvision torchaudio`  
  (Uses Metal on Apple Silicon when available.)

---

## Depth-Anything-V2

- **Repo:** DepthAnything/Depth-Anything-V2 (GitHub).
- **Hugging Face:** e.g. `depth-anything/Depth-Anything-V2-Base`, `Depth-Anything-V2-Large`; also Transformers-friendly variants (e.g. `Depth-Anything-V2-Large-hf`).
- **Weights license:** **CC-BY-NC-4.0** (non-commercial). Document in app and comply for distribution.
- **Usage:** Load image → model inference → depth map (normalize to 0–1 as needed). Check repo for any API or dependency changes when integrating.

---

## MiDaS

- **Repo:** isl-org/MiDaS (GitHub). **Note:** Repository was archived (read-only) as of 2025; code remains usable.
- **License:** MIT (code). Check weight/model files for separate licenses if distributing.
- **Versions:** MiDaS v3.1 (e.g. BEiT, Swin2, Swin, etc.) available; PyTorch Hub: `pytorch.hub.load("intel-isl/MiDaS", ...)`.
- **Recommendation:** Prefer Depth-Anything-V2 for new work; MiDaS remains an alternative if needed.

---

## Python ↔ Rust Interface

- **Model storage:** `~/.simplepicture3d/models/` (per PRD).
- **Interface:** Subprocess or Tauri sidecar: Rust sends image via stdin or temp file; Python returns depth map via stdout or output file (JSON array or binary). Prefer file-based for large images to avoid stdin buffer limits.
- **Verification:** SHA256 on downloaded model files.

---

## ONNX / ONNXRuntime

- **Optional path:** Depth-Anything ONNX ports exist (e.g. community repos for Depth-Anything-V2 ONNX); inference can be run with **ONNX Runtime** in Python (`pip install onnxruntime` or `onnxruntime-gpu`).
- **Recommendation:** PyTorch-native inference is the primary path for Sprint 1.1. ONNX can be considered later for optimization or potential Rust-side inference (e.g. ort crate); not required for MVP.

---

## Project Usage (Summary)

- **Runtime:** Python 3.10+.
- **Framework:** PyTorch for inference.
- **Models:** Depth-Anything-V2 (recommended) or MiDaS v3.1.
- **Interface:** Subprocess/sidecar from Rust; image via file or stdin; depth map via file or stdout (JSON/binary).

---

## Research Tasks (Researcher)

- [x] PyTorch CPU/CUDA/Metal install and version — documented above.
- [x] Depth-Anything-V2 and MiDaS: repo status, weights licenses (CC-BY-NC-4.0, MIT) — documented above.
- [x] Python 3.x minimum — 3.10+.
- [x] ONNX: optional; not required for MVP — documented above.
- [x] Official sources and Last checked — in table above.
