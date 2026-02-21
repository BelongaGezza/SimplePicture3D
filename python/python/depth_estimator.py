#!/usr/bin/env python3
# Copyright (c) 2026 SimplePicture3D Contributors
# SPDX-License-Identifier: MIT

"""
Depth estimator CLI: --input <path> → stdout JSON depth map, stderr progress (AI-201–AI-205).

Contract (ARCH-101, ARCH-102, AI-301):
  - Input: image file path (temp file from Rust).
  - Output: single JSON object on stdout: {"height": H, "width": W, "depth": [f32,...]} row-major.
  - Depth values are guaranteed in [0, 1] for all code paths (stub and Depth-Anything-V2).
    Out-of-range and NaN are clamped to 0 or 1 before output.
  - Progress: stderr only, e.g. "PROGRESS 25", "STAGE loading_model".
  - Exit 0 on success; non-zero + stderr message on error.

Uses Depth-Anything-V2 (Hugging Face) when available; falls back to stub depth for roundtrip testing
when model is not installed or SP3D_USE_STUB=1.

Model licensing (AI-502):
  - Depth-Anything-V2 weights: CC-BY-NC-4.0 (non-commercial use only).
  - Stub mode (--no-model / SP3D_USE_STUB=1): no model loaded; no license applies.
  Use --show-license to print license information to stdout.
"""

import argparse
import json
import os
import sys
from pathlib import Path


def emit_progress(percent: int, stderr=sys.stderr) -> None:
    """Emit PROGRESS line to stderr (BACK-205 / AI-203)."""
    print(f"PROGRESS {percent}", file=stderr, flush=True)


def emit_stage(stage: str, stderr=sys.stderr) -> None:
    """Emit STAGE line to stderr (optional)."""
    print(f"STAGE {stage}", file=stderr, flush=True)


def clamp_depth_to_01(depth: list[float]) -> list[float]:
    """
    Enforce [0, 1] range for depth output (AI-301, ARCH-102).
    Clamps out-of-range values; NaN/Inf treated as 0.
    """
    result: list[float] = []
    for x in depth:
        if x != x:  # NaN
            result.append(0.0)
        elif x <= 0.0:
            result.append(0.0)
        elif x >= 1.0:
            result.append(1.0)
        else:
            result.append(x)
    return result


def get_device() -> str:
    """Return best available device: cuda > mps (Apple) > cpu (AI-205)."""
    try:
        import torch
        if torch.cuda.is_available():
            return "cuda"
        if getattr(torch.backends, "mps", None) and torch.backends.mps.is_available():
            return "mps"
    except ImportError:
        pass
    return "cpu"


def load_image_dimensions(path: Path) -> tuple[int, int]:
    """Read image file and return (width, height). Uses PIL if available, else minimal PNG/JPEG parse."""
    try:
        from PIL import Image
        with Image.open(path) as img:
            return img.size[0], img.size[1]
    except ImportError:
        pass
    # Fallback: read magic and minimal dimensions (PNG: bytes 16-23 are width/height big-endian).
    data = path.read_bytes()
    if len(data) < 24:
        raise ValueError("file too short to be a valid image")
    if data[:8] == b"\x89PNG\r\n\x1a\n":
        import struct
        w, h = struct.unpack(">II", data[16:24])
        return w, h
    if data[:3] == b"\xff\xd8\xff":
        i = 2
        while i < len(data) - 7:
            if data[i] == 0xFF and data[i + 1] in (0xC0, 0xC1, 0xC2):
                h = (data[i + 5] << 8) | data[i + 6]
                w = (data[i + 7] << 8) | data[i + 8]
                return w, h
            i += 1
        raise ValueError("JPEG has no SOF segment")
    raise ValueError("unsupported or invalid image format (expected PNG or JPEG)")


def run_inference_stub(width: int, height: int) -> list[float]:
    """Produce stub depth map (0.0–1.0 row-major). No model; for roundtrip testing."""
    n = width * height
    return [y / max(height - 1, 1) for y in range(height) for _ in range(width)]


def run_inference_depth_anything_v2(
    image_path: Path,
    model_name_or_path: str,
    device: str,
    stderr,
) -> tuple[int, int, list[float]]:
    """
    Run Depth-Anything-V2 inference (AI-204). Returns (width, height, depth_list) 0–1 row-major.
    Raises on failure; OOM and decode errors surfaced to stderr by caller.
    """
    from PIL import Image
    import torch
    from transformers import AutoImageProcessor, AutoModelForDepthEstimation

    with Image.open(image_path) as img:
        image = img.convert("RGB")
        width, height = image.size

    emit_stage("loading_model", stderr)
    # Prefer local path (~/.simplepicture3d/models/<name>) then Hugging Face
    local_base = Path.home() / ".simplepicture3d" / "models"
    local_path = local_base / model_name_or_path.split("/")[-1]
    if local_path.is_dir():
        model_id = str(local_path)
    else:
        model_id = model_name_or_path

    image_processor = AutoImageProcessor.from_pretrained(model_id)
    model = AutoModelForDepthEstimation.from_pretrained(model_id).to(device)
    model.eval()

    emit_stage("model_license: CC-BY-NC-4.0 (non-commercial)", stderr)

    emit_stage("inference", stderr)
    inputs = image_processor(images=image, return_tensors="pt")
    inputs = {k: v.to(device) for k, v in inputs.items()}

    with torch.no_grad():
        outputs = model(**inputs)

    post = image_processor.post_process_depth_estimation(
        outputs,
        target_sizes=[(height, width)],
    )
    predicted_depth = post[0]["predicted_depth"]  # (1, H, W)
    predicted_depth = predicted_depth.squeeze(0).cpu().float()  # (H, W)

    # Normalize to 0–1 (ARCH-102)
    d_min, d_max = predicted_depth.min().item(), predicted_depth.max().item()
    if d_max > d_min:
        depth_np = (predicted_depth - d_min) / (d_max - d_min)
    else:
        depth_np = predicted_depth * 0.0  # constant map

    depth_list = depth_np.flatten().tolist()
    return width, height, depth_list


def get_license_info() -> str:
    """Return human-readable license information for the depth model (AI-502)."""
    lines = [
        "Depth-Anything-V2 (default model): CC-BY-NC-4.0 (non-commercial use only).",
        "Stub mode (--no-model or SP3D_USE_STUB=1): No model loaded; no license applies.",
    ]
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="Depth estimator for SimplePicture3D")
    parser.add_argument("--input", type=Path, help="Path to input image (temp file)")
    parser.add_argument(
        "--model",
        default=os.environ.get("DEPTH_MODEL_PATH", "depth-anything/Depth-Anything-V2-Small-hf"),
        help="Model name (HF id) or path to local dir (default: Depth-Anything-V2-Small-hf)",
    )
    parser.add_argument(
        "--no-model",
        action="store_true",
        help="Use stub depth only (no PyTorch); for roundtrip tests without model",
    )
    parser.add_argument(
        "--show-license",
        action="store_true",
        help="Print model license information to stdout and exit",
    )
    args = parser.parse_args()

    if args.show_license:
        print(get_license_info(), flush=True)
        return 0

    if not args.input:
        print("error: --input required for depth estimation (or use --show-license)", file=sys.stderr)
        return 1
    if not args.input.is_file():
        print("error: input path is not a file", file=sys.stderr)
        return 1

    use_stub = args.no_model or os.environ.get("SP3D_USE_STUB", "").lower() in ("1", "true", "yes")

    try:
        emit_stage("loading_image", sys.stderr)
        width, height = load_image_dimensions(args.input)
        emit_progress(10)

        if use_stub:
            emit_stage("inference")
            depth = run_inference_stub(width, height)
        else:
            try:
                device = get_device()
                width, height, depth = run_inference_depth_anything_v2(
                    args.input, args.model, device, sys.stderr
                )
            except ImportError as e:
                print(
                    "warning: PyTorch/transformers not available, using stub depth",
                    file=sys.stderr,
                )
                emit_stage("inference")
                depth = run_inference_stub(width, height)
            except Exception as e:
                if "out of memory" in str(e).lower() or "oom" in str(e).lower():
                    print(f"error: GPU/system out of memory: {e}", file=sys.stderr)
                else:
                    print(f"error: {e}", file=sys.stderr)
                return 1

        emit_progress(90)
        emit_stage("writing_output")
        depth = clamp_depth_to_01(depth)  # AI-301: guarantee [0, 1] for all code paths
        # AI-302: contract shape check — depth array must be row-major, length height × width
        expected_len = height * width
        if len(depth) != expected_len:
            raise RuntimeError(
                f"depth array length mismatch: expected {expected_len} (height={height} × width={width}), got {len(depth)}"
            )
        out = {"height": height, "width": width, "depth": depth}
        print(json.dumps(out), flush=True)
        emit_progress(100)
        return 0
    except Exception as e:
        print(f"error: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
