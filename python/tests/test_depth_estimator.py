"""
Pytest suite for depth_estimator (AI-401).
All tests run without downloading the AI model (use --no-model or SP3D_USE_STUB=1).
"""
import json
import os
import subprocess
import sys
from pathlib import Path

import pytest

# Import after conftest has set up path
import depth_estimator as de

# Repo root (parent of python/); fixtures live in tests/fixtures/
REPO_ROOT = Path(__file__).resolve().parent.parent.parent
FIXTURES = REPO_ROOT / "tests" / "fixtures"
VALID_SMALL = FIXTURES / "valid_small.png"
VALID_1X1 = FIXTURES / "valid_1x1.png"
INVALID_IMAGE = FIXTURES / "invalid_not_an_image.png"


def _run_cli(args: list[str], env: dict | None = None) -> tuple[int, str, str]:
    """Run depth_estimator CLI; return (returncode, stdout, stderr)."""
    cmd = [sys.executable, "-m", "depth_estimator", "--input", str(args[0])] + args[1:]
    if env is None:
        env = os.environ.copy()
    env.setdefault("SP3D_USE_STUB", "1")
    env["PYTHONPATH"] = str(REPO_ROOT / "python" / "python")
    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        cwd=REPO_ROOT,
        env=env,
    )
    return result.returncode, result.stdout, result.stderr


# ----- Unit tests: clamp_depth_to_01 -----


def test_clamp_depth_to_01_in_range():
    """Values in [0, 1] are unchanged."""
    assert de.clamp_depth_to_01([0.0, 0.5, 1.0]) == [0.0, 0.5, 1.0]


def test_clamp_depth_to_01_below_zero():
    """Values below 0 are clamped to 0."""
    assert de.clamp_depth_to_01([-0.1, -1.0]) == [0.0, 0.0]


def test_clamp_depth_to_01_above_one():
    """Values above 1 are clamped to 1."""
    assert de.clamp_depth_to_01([1.1, 2.0]) == [1.0, 1.0]


def test_clamp_depth_to_01_nan():
    """NaN is replaced with 0."""
    out = de.clamp_depth_to_01([float("nan"), 0.5])
    assert out[0] == 0.0 and out[1] == 0.5


# ----- Unit tests: run_inference_stub -----


def test_run_inference_stub_shape():
    """Stub output length is height * width."""
    depth = de.run_inference_stub(10, 5)
    assert len(depth) == 50


def test_run_inference_stub_normalized_0_1():
    """Stub output values are in [0, 1]."""
    depth = de.run_inference_stub(3, 4)
    assert all(0.0 <= v <= 1.0 for v in depth)


def test_run_inference_stub_row_major():
    """Stub is row-major: first row 0, then row 1, etc."""
    depth = de.run_inference_stub(2, 3)
    # Row 0: y=0 -> 0, 0; row 1: y=1 -> 0.5, 0.5; row 2: y=2 -> 1.0, 1.0
    assert depth[0] == 0.0 and depth[1] == 0.0
    assert depth[2] == 0.5 and depth[3] == 0.5
    assert depth[4] == 1.0 and depth[5] == 1.0


def test_run_inference_stub_1x1():
    """1x1 stub gives single value 0.0 (y=0, height-1=0 -> 0/max(1)=0)."""
    depth = de.run_inference_stub(1, 1)
    assert len(depth) == 1 and depth[0] == 0.0


# ----- Unit tests: load_image_dimensions -----


def test_load_image_dimensions_valid_small():
    """Valid PNG returns (width, height)."""
    w, h = de.load_image_dimensions(VALID_SMALL)
    assert w > 0 and h > 0


def test_load_image_dimensions_valid_1x1():
    """1x1 PNG returns (1, 1)."""
    w, h = de.load_image_dimensions(VALID_1X1)
    assert (w, h) == (1, 1)


def test_load_image_dimensions_missing_file():
    """Missing file raises."""
    with pytest.raises(FileNotFoundError):
        de.load_image_dimensions(REPO_ROOT / "nonexistent.png")


def test_load_image_dimensions_invalid_image():
    """Invalid image file raises ValueError or PIL.UnidentifiedImageError."""
    try:
        from PIL.Image import UnidentifiedImageError
    except ImportError:
        UnidentifiedImageError = ValueError  # fallback when PIL not used
    with pytest.raises((ValueError, UnidentifiedImageError)):
        de.load_image_dimensions(INVALID_IMAGE)


# ----- CLI / integration tests -----


@pytest.mark.skipif(not VALID_SMALL.is_file(), reason="fixture valid_small.png not found")
def test_cli_stub_output_shape():
    """CLI with stub: stdout JSON has height, width, depth; len(depth)=H*W."""
    code, out, err = _run_cli([str(VALID_SMALL), "--no-model"])
    assert code == 0, (out, err)
    data = json.loads(out)
    assert "height" in data and "width" in data and "depth" in data
    assert len(data["depth"]) == data["height"] * data["width"]


@pytest.mark.skipif(not VALID_SMALL.is_file(), reason="fixture valid_small.png not found")
def test_cli_stub_normalization_0_1():
    """CLI stub output depth values are in [0, 1]."""
    code, out, err = _run_cli([str(VALID_SMALL), "--no-model"])
    assert code == 0
    data = json.loads(out)
    for v in data["depth"]:
        assert 0.0 <= v <= 1.0, f"depth value {v} out of [0,1]"


@pytest.mark.skipif(not VALID_SMALL.is_file(), reason="fixture valid_small.png not found")
def test_cli_stub_json_format():
    """CLI stdout is single JSON object with height, width, depth."""
    code, out, err = _run_cli([str(VALID_SMALL), "--no-model"])
    assert code == 0
    data = json.loads(out)
    assert isinstance(data["height"], int)
    assert isinstance(data["width"], int)
    assert isinstance(data["depth"], list)
    assert all(isinstance(x, (int, float)) for x in data["depth"])


@pytest.mark.skipif(not VALID_1X1.is_file(), reason="fixture valid_1x1.png not found")
def test_cli_flag_no_model():
    """--no-model forces stub (no model download)."""
    code, out, err = _run_cli([str(VALID_1X1), "--no-model"])
    assert code == 0
    data = json.loads(out)
    assert data["height"] == 1 and data["width"] == 1 and len(data["depth"]) == 1


def test_cli_missing_file():
    """Missing input path: non-zero exit and stderr message."""
    code, out, err = _run_cli([str(REPO_ROOT / "nonexistent.png")])
    assert code != 0
    assert "error" in err.lower() or "not" in err.lower()


@pytest.mark.skipif(not INVALID_IMAGE.is_file(), reason="fixture invalid_not_an_image.png not found")
def test_cli_invalid_image():
    """Invalid image file: non-zero exit."""
    code, out, err = _run_cli([str(INVALID_IMAGE)])
    assert code != 0


@pytest.mark.skipif(not VALID_1X1.is_file(), reason="fixture valid_1x1.png not found")
def test_cli_env_stub():
    """SP3D_USE_STUB=1 uses stub without --no-model."""
    code, out, err = _run_cli([str(VALID_1X1)], env={**os.environ, "SP3D_USE_STUB": "1"})
    assert code == 0
    data = json.loads(out)
    assert "depth" in data and len(data["depth"]) == 1
