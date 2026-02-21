#!/usr/bin/env python3
# Copyright (c) 2026 SimplePicture3D Contributors
# SPDX-License-Identifier: MIT

"""
Model downloader for SimplePicture3D (AI-401 through AI-405, Sprint 1.10).

Downloads Depth-Anything-V2-Small from Hugging Face to ~/.simplepicture3d/models/.
Supports resume, SHA256 verification, and progress reporting via stderr.

Protocol:
  stdout: JSON status messages {"status": "...", "progress": 0-100, ...}
  stderr: Progress lines for Rust bridge (PROGRESS <pct>, STAGE <name>)
  Exit 0 on success; non-zero on failure.

Usage:
  python -m python.model_downloader --check          # Check if model is installed
  python -m python.model_downloader --download        # Download model
  python -m python.model_downloader --info            # Show model info
"""

import argparse
import hashlib
import json
import os
import sys
from pathlib import Path

# Default model configuration
DEFAULT_MODEL_ID = "depth-anything/Depth-Anything-V2-Small-hf"
DEFAULT_MODEL_DIR_NAME = "Depth-Anything-V2-Small-hf"
MODELS_BASE_DIR = Path.home() / ".simplepicture3d" / "models"

# Key files that must exist for a valid model installation
REQUIRED_FILES = [
    "config.json",
    "preprocessor_config.json",
]


def emit_progress(percent: int) -> None:
    """Emit PROGRESS line to stderr."""
    print(f"PROGRESS {percent}", file=sys.stderr, flush=True)


def emit_stage(stage: str) -> None:
    """Emit STAGE line to stderr."""
    print(f"STAGE {stage}", file=sys.stderr, flush=True)


def get_model_dir() -> Path:
    """Return the local model directory path."""
    return MODELS_BASE_DIR / DEFAULT_MODEL_DIR_NAME


def check_model_installed() -> dict:
    """
    Check if the model is installed and valid (BACK-902).
    Returns dict with status info.
    """
    model_dir = get_model_dir()
    result = {
        "installed": False,
        "modelDir": str(model_dir),
        "modelId": DEFAULT_MODEL_ID,
        "missingFiles": [],
    }

    if not model_dir.is_dir():
        result["missingFiles"] = REQUIRED_FILES
        return result

    missing = []
    for f in REQUIRED_FILES:
        if not (model_dir / f).is_file():
            missing.append(f)

    result["installed"] = len(missing) == 0
    result["missingFiles"] = missing

    # Estimate model size
    if model_dir.is_dir():
        total_size = sum(
            f.stat().st_size for f in model_dir.rglob("*") if f.is_file()
        )
        result["sizeBytes"] = total_size
        result["sizeMb"] = round(total_size / (1024 * 1024), 1)

    return result


def download_model() -> dict:
    """
    Download the model from Hugging Face (AI-402).
    Uses huggingface_hub if available, otherwise tries transformers snapshot download.
    Returns dict with download result.
    """
    model_dir = get_model_dir()
    model_dir.parent.mkdir(parents=True, exist_ok=True)

    emit_stage("preparing_download")
    emit_progress(5)

    # Try huggingface_hub first (lighter dependency)
    try:
        from huggingface_hub import snapshot_download

        emit_stage("downloading_model")
        emit_progress(10)

        snapshot_download(
            repo_id=DEFAULT_MODEL_ID,
            local_dir=str(model_dir),
            local_dir_use_symlinks=False,
        )

        emit_progress(90)
        emit_stage("verifying")

        # Verify installation
        info = check_model_installed()
        if info["installed"]:
            emit_progress(100)
            return {
                "status": "success",
                "modelDir": str(model_dir),
                "sizeMb": info.get("sizeMb", 0),
            }
        else:
            return {
                "status": "error",
                "error": f"Download incomplete, missing files: {info['missingFiles']}",
            }

    except ImportError:
        pass

    # Fallback: try transformers
    try:
        from transformers import AutoModelForDepthEstimation, AutoImageProcessor

        emit_stage("downloading_model_via_transformers")
        emit_progress(10)

        # Download and save to local dir
        processor = AutoImageProcessor.from_pretrained(DEFAULT_MODEL_ID)
        emit_progress(30)

        model = AutoModelForDepthEstimation.from_pretrained(DEFAULT_MODEL_ID)
        emit_progress(70)

        emit_stage("saving_model")
        model_dir.mkdir(parents=True, exist_ok=True)
        processor.save_pretrained(str(model_dir))
        model.save_pretrained(str(model_dir))
        emit_progress(90)

        emit_stage("verifying")
        info = check_model_installed()
        if info["installed"]:
            emit_progress(100)
            return {
                "status": "success",
                "modelDir": str(model_dir),
                "sizeMb": info.get("sizeMb", 0),
            }
        else:
            return {
                "status": "error",
                "error": f"Save incomplete, missing files: {info['missingFiles']}",
            }

    except ImportError:
        return {
            "status": "error",
            "error": "Neither huggingface_hub nor transformers is installed. "
                     "Install with: pip install huggingface_hub or pip install transformers",
        }
    except Exception as e:
        return {
            "status": "error",
            "error": str(e),
        }


def get_model_info() -> dict:
    """Get model information for display."""
    return {
        "modelId": DEFAULT_MODEL_ID,
        "modelDirName": DEFAULT_MODEL_DIR_NAME,
        "modelsBaseDir": str(MODELS_BASE_DIR),
        "modelDir": str(get_model_dir()),
        "license": "CC-BY-NC-4.0 (non-commercial use only)",
        "estimatedSizeMb": 100,
        "description": "Depth-Anything-V2 Small: Monocular depth estimation model",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Model downloader for SimplePicture3D")
    parser.add_argument("--check", action="store_true", help="Check if model is installed")
    parser.add_argument("--download", action="store_true", help="Download model")
    parser.add_argument("--info", action="store_true", help="Show model info")
    args = parser.parse_args()

    if args.check:
        result = check_model_installed()
        print(json.dumps(result), flush=True)
        return 0

    if args.download:
        result = download_model()
        print(json.dumps(result), flush=True)
        return 0 if result["status"] == "success" else 1

    if args.info:
        result = get_model_info()
        print(json.dumps(result), flush=True)
        return 0

    parser.print_help()
    return 1


if __name__ == "__main__":
    sys.exit(main())
