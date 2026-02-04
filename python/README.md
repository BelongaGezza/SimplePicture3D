# Python depth estimation (Sprint 1.3)

**Purpose:** Run the depth estimator used by the Rust backend. JR2-204, ARCH-101.

## Requirements

- **Python:** 3.10 or newer
- **Dependencies:** `pip install -r requirements.txt` (see below)

## Setup

From the **project root** (parent of `python/` and `src-tauri/`):

```bash
# Optional: use a virtual environment (recommended)
python -m venv venv
# Windows:
venv\Scripts\activate
# macOS/Linux:
# source venv/bin/activate

# Install dependencies (from project root or python/)
pip install -r python/requirements.txt
```

## Running the depth estimator

The Rust backend invokes:

```bash
python -m python.depth_estimator --input <path>
```

- **Working directory:** Rust runs the process with `current_dir` = `project_root/python` (or `project_root` if `python/` not found).
- **Input:** Path to a temp image file (PNG/JPEG) written by Rust. No user-controlled args.
- **Output:** One JSON object on stdout: `{"height": H, "width": W, "depth": [f32,...]}` (row-major, 0–1).
- **Progress:** Stderr only, e.g. `PROGRESS 25`, `STAGE inference`.

To run manually from project root:

```bash
# From project root; PYTHONPATH not needed if you cd into python/
cd python
python -m python.depth_estimator --input /path/to/image.png
```

Or with PYTHONPATH from project root:

```bash
# Windows (PowerShell)
$env:PYTHONPATH = (Get-Location).Path + "\python"; python -m python.depth_estimator --input path\to\image.png
```

## Model and device (AI-204, AI-205)

- **Default model:** `depth-anything/Depth-Anything-V2-Small-hf` (Hugging Face). First run downloads weights to HF cache.
- **Local model:** Place a Hugging Face–style model dir under `~/.simplepicture3d/models/` (e.g. `Depth-Anything-V2-Small-hf`) or set `DEPTH_MODEL_PATH` to the model name or path.
- **Stub mode (no model):** Use `--no-model` or set `SP3D_USE_STUB=1` to output stub depth (gradient) for roundtrip tests without PyTorch/model.
- **Device:** Auto-detected: CUDA → MPS (Apple) → CPU. Force CPU for CI with `CUDA_VISIBLE_DEVICES=""` (Linux) or env that disables CUDA.
- **OOM:** On out-of-memory, the script exits non-zero and writes an error to stderr (no crash).

## References

- `docs/architecture.md` — Rust–Python bridge, ARCH-101–104
- `RESEARCH/python-ml.md` — PyTorch, Depth-Anything-V2, MiDaS
- `SPRINTS/Sprint_1_3/SPRINT_1_3_Task_Assignment.md` — Task list
