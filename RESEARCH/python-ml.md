# Python & ML Stack

**Purpose:** Python, PyTorch, and depth estimation models for SimplePicture3D.

## Official Sources

| Source | URL | Last Checked |
|--------|-----|--------------|
| PyTorch | https://pytorch.org/docs/ | — |
| Depth-Anything-V2 | https://github.com/DepthAnything/Depth-Anything-V2 | — |
| MiDaS | https://github.com/isl-org/MiDaS | — |
| Hugging Face | https://huggingface.co/ | — |
| ONNX Runtime | https://onnxruntime.ai/ | — |

*Researcher: Update "Last Checked" when verifying. Note model weight licenses (e.g. CC-BY-NC-4.0).*

---

## Project Usage

- **Runtime:** Python 3.10+ (recommended)
- **Framework:** PyTorch for inference
- **Models:** Depth-Anything-V2 (recommended) or MiDaS v3.1
- **Interface:** Subprocess from Rust; image via stdin or temp file; depth map via stdout or file (JSON/msgpack)

---

## Model Notes

- **Depth-Anything-V2:** Apache-2.0 code; model weights CC-BY-NC-4.0 (non-commercial)
- **MiDaS:** MIT code; check weight license for commercial use
- **Storage:** `~/.simplepicture3d/models/`
- **Verification:** SHA256 checksum on download

---

## Research Tasks (Researcher)

- [ ] PyTorch CPU vs CUDA vs Metal (macOS) setup
- [ ] ONNX conversion path for potentially faster inference in Rust
- [ ] Depth map serialization: JSON vs numpy binary (size, speed)
- [ ] Python subprocess: stdin buffer limits for large images
- [ ] Version changes, deprecated APIs since knowledge cutoff
