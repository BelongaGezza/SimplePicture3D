"""Tests for model_downloader (Sprint 1.10, AI-401 through AI-405)."""

import json
import os
import sys
from pathlib import Path
from unittest.mock import patch, MagicMock

import pytest

# Ensure the python package is importable
_python_pkg = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(_python_pkg))
sys.path.insert(0, str(_python_pkg / "python"))

from model_downloader import (
    check_model_installed,
    get_model_info,
    get_model_dir,
    REQUIRED_FILES,
    DEFAULT_MODEL_ID,
    MODELS_BASE_DIR,
)


class TestCheckModelInstalled:
    """Tests for check_model_installed (BACK-902)."""

    def test_returns_not_installed_when_dir_missing(self, tmp_path):
        """When model directory does not exist, installed=False."""
        with patch("model_downloader.MODELS_BASE_DIR", tmp_path):
            result = check_model_installed()
            assert result["installed"] is False
            assert len(result["missingFiles"]) > 0

    def test_returns_not_installed_when_files_missing(self, tmp_path):
        """When model directory exists but required files are missing."""
        model_dir = tmp_path / "Depth-Anything-V2-Small-hf"
        model_dir.mkdir(parents=True)
        with patch("model_downloader.MODELS_BASE_DIR", tmp_path):
            result = check_model_installed()
            assert result["installed"] is False
            assert len(result["missingFiles"]) == len(REQUIRED_FILES)

    def test_returns_installed_when_all_files_present(self, tmp_path):
        """When all required files exist, installed=True."""
        model_dir = tmp_path / "Depth-Anything-V2-Small-hf"
        model_dir.mkdir(parents=True)
        for f in REQUIRED_FILES:
            (model_dir / f).write_text("{}")
        with patch("model_downloader.MODELS_BASE_DIR", tmp_path):
            result = check_model_installed()
            assert result["installed"] is True
            assert result["missingFiles"] == []

    def test_reports_size_when_dir_exists(self, tmp_path):
        """When directory exists, sizeBytes and sizeMb are reported."""
        model_dir = tmp_path / "Depth-Anything-V2-Small-hf"
        model_dir.mkdir(parents=True)
        for f in REQUIRED_FILES:
            (model_dir / f).write_text("x" * 1024)  # 1KB each
        with patch("model_downloader.MODELS_BASE_DIR", tmp_path):
            result = check_model_installed()
            assert "sizeBytes" in result
            assert result["sizeBytes"] > 0

    def test_partial_files_reports_missing(self, tmp_path):
        """When only some required files exist, reports specific missing ones."""
        model_dir = tmp_path / "Depth-Anything-V2-Small-hf"
        model_dir.mkdir(parents=True)
        # Create only the first required file
        (model_dir / REQUIRED_FILES[0]).write_text("{}")
        with patch("model_downloader.MODELS_BASE_DIR", tmp_path):
            result = check_model_installed()
            assert result["installed"] is False
            assert REQUIRED_FILES[0] not in result["missingFiles"]
            assert len(result["missingFiles"]) == len(REQUIRED_FILES) - 1


class TestGetModelInfo:
    """Tests for get_model_info."""

    def test_returns_expected_fields(self):
        """Model info contains all required fields."""
        info = get_model_info()
        assert "modelId" in info
        assert "license" in info
        assert "estimatedSizeMb" in info
        assert "description" in info
        assert "modelDir" in info

    def test_model_id_matches_default(self):
        """Model ID matches the configured default."""
        info = get_model_info()
        assert info["modelId"] == DEFAULT_MODEL_ID

    def test_estimated_size_is_reasonable(self):
        """Estimated size should be > 0 and < 10GB."""
        info = get_model_info()
        assert 0 < info["estimatedSizeMb"] < 10000


class TestGetModelDir:
    """Tests for get_model_dir."""

    def test_returns_path_under_models_base(self):
        """Model dir should be under MODELS_BASE_DIR."""
        model_dir = get_model_dir()
        assert str(MODELS_BASE_DIR) in str(model_dir)

    def test_returns_path_object(self):
        """get_model_dir should return a Path object."""
        model_dir = get_model_dir()
        assert isinstance(model_dir, Path)


class TestJsonOutput:
    """Tests for JSON serialization of outputs."""

    def test_check_model_json_serializable(self, tmp_path):
        """check_model_installed output must be JSON serializable."""
        with patch("model_downloader.MODELS_BASE_DIR", tmp_path):
            result = check_model_installed()
            output = json.dumps(result)
            parsed = json.loads(output)
            assert parsed["installed"] is False

    def test_get_model_info_json_serializable(self):
        """get_model_info output must be JSON serializable."""
        info = get_model_info()
        output = json.dumps(info)
        parsed = json.loads(output)
        assert "modelId" in parsed
