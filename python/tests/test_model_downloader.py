# Copyright (c) 2026 SimplePicture3D Contributors
# SPDX-License-Identifier: MIT

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
    download_model,
    get_model_info,
    get_model_dir,
    load_expected_hashes,
    verify_model_sha256,
    write_current_hashes,
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


class TestVerifyModelSha256:
    """Tests for SEC-202 SHA256 verification."""

    def test_verify_empty_expected_passes(self, tmp_path):
        """When expected_hashes is empty, verification passes."""
        (tmp_path / "config.json").write_text("{}")
        ok, err = verify_model_sha256(tmp_path, {})
        assert ok is True
        assert err == ""

    def test_verify_matching_hash_passes(self, tmp_path):
        """When file exists and hash matches, verification passes."""
        content = b"test content"
        (tmp_path / "config.json").write_bytes(content)
        import hashlib
        expected = {"config.json": hashlib.sha256(content).hexdigest().lower()}
        ok, err = verify_model_sha256(tmp_path, expected)
        assert ok is True
        assert err == ""

    def test_verify_mismatch_fails(self, tmp_path):
        """When hash does not match, verification fails."""
        (tmp_path / "config.json").write_text("x")
        expected = {"config.json": "0" * 64}
        ok, err = verify_model_sha256(tmp_path, expected)
        assert ok is False
        assert "mismatch" in err

    def test_verify_missing_file_fails(self, tmp_path):
        """When expected file is missing, verification fails."""
        expected = {"config.json": "a" * 64}
        ok, err = verify_model_sha256(tmp_path, expected)
        assert ok is False
        assert "not found" in err

    def test_verify_nonexistent_dir_fails(self, tmp_path):
        """When model_dir does not exist, verification fails."""
        ok, err = verify_model_sha256(tmp_path / "nonexistent", {"a": "b"})
        assert ok is False
        assert "does not exist" in err

    def test_load_expected_hashes_empty_or_missing(self):
        """load_expected_hashes returns empty dict when file missing or invalid."""
        with patch("model_downloader._expected_hashes_path") as p:
            p.return_value = Path("/nonexistent/path/hashes.json")
            assert load_expected_hashes() == {}

    def test_load_expected_hashes_skips_todo_placeholders(self, tmp_path):
        """TODO placeholder hashes are ignored until real digests are committed (SEC-D-02)."""
        hashes_file = tmp_path / "expected_model_hashes.json"
        hashes_file.write_text(
            json.dumps(
                {
                    "hashes": {
                        "config.json": "TODO:REPLACE_WITH_SHA256_AFTER_TRUSTED_DOWNLOAD",
                        "preprocessor_config.json": "a" * 64,
                    }
                }
            ),
            encoding="utf-8",
        )
        with patch("model_downloader._expected_hashes_path", return_value=hashes_file):
            loaded = load_expected_hashes()
        assert "config.json" not in loaded
        assert loaded["preprocessor_config.json"] == "a" * 64

    def test_load_expected_hashes_from_repo_stub_file(self):
        """Committed stub file documents keys but does not enforce verification yet."""
        loaded = load_expected_hashes()
        assert loaded == {}


class TestDownloadModelVerification:
    """Integration tests for SEC-D-02 download_model SHA256 enforcement."""

    def _setup_model_dir(self, tmp_path):
        model_dir = tmp_path / "Depth-Anything-V2-Small-hf"
        model_dir.mkdir(parents=True)
        for f in REQUIRED_FILES:
            (model_dir / f).write_text("{}")
        return model_dir

    def _mock_huggingface_hub(self, fake_snapshot_download):
        """Inject a fake huggingface_hub module (optional dep in CI/dev)."""
        mock_module = MagicMock()
        mock_module.snapshot_download = fake_snapshot_download
        return patch.dict(sys.modules, {"huggingface_hub": mock_module})

    def test_download_fails_when_configured_hash_mismatch(self, tmp_path):
        """download_model returns error when enforced hashes do not match files."""
        model_dir = self._setup_model_dir(tmp_path)
        (model_dir / "config.json").write_text('{"mismatch": true}')

        def fake_snapshot_download(**kwargs):
            return str(model_dir)

        expected = {"config.json": "0" * 64}

        with patch("model_downloader.MODELS_BASE_DIR", tmp_path), patch(
            "model_downloader.load_expected_hashes", return_value=expected
        ), self._mock_huggingface_hub(fake_snapshot_download):
            result = download_model()

        assert result["status"] == "error"
        assert "integrity check failed" in result["error"].lower()
        assert "mismatch" in result["error"].lower()

    def test_download_succeeds_when_configured_hashes_match(self, tmp_path):
        """download_model succeeds when enforced hashes match downloaded files."""
        import hashlib

        model_dir = self._setup_model_dir(tmp_path)
        content = b'{"model": "ok"}'
        (model_dir / "config.json").write_bytes(content)
        expected = {"config.json": hashlib.sha256(content).hexdigest().lower()}

        def fake_snapshot_download(**kwargs):
            return str(model_dir)

        with patch("model_downloader.MODELS_BASE_DIR", tmp_path), patch(
            "model_downloader.load_expected_hashes", return_value=expected
        ), self._mock_huggingface_hub(fake_snapshot_download):
            result = download_model()

        assert result["status"] == "success"
        assert result["modelDir"] == str(model_dir)

    def test_download_skips_verification_when_no_enforced_hashes(self, tmp_path):
        """download_model skips SHA256 check when only stub/TODO hashes are present."""
        model_dir = self._setup_model_dir(tmp_path)

        def fake_snapshot_download(**kwargs):
            return str(model_dir)

        with patch("model_downloader.MODELS_BASE_DIR", tmp_path), patch(
            "model_downloader.load_expected_hashes", return_value={}
        ), self._mock_huggingface_hub(fake_snapshot_download):
            result = download_model()

        assert result["status"] == "success"


class TestEnforceModelHashesStrictMode:
    """SEC-D-02: optional strict CI when SP3D_ENFORCE_MODEL_HASHES=1."""

    def test_enforce_model_hashes_rejects_todo_placeholders_when_env_set(self):
        """Fails in strict mode until real SHA256 digests replace TODO placeholders."""
        if os.environ.get("SP3D_ENFORCE_MODEL_HASHES") != "1":
            pytest.skip("SP3D_ENFORCE_MODEL_HASHES not set (optional strict CI step)")

        path = Path(__file__).resolve().parent.parent / "python" / "expected_model_hashes.json"
        data = json.loads(path.read_text(encoding="utf-8"))
        hashes = data.get("hashes", {})
        bad = {name: value for name, value in hashes.items() if "TODO:REPLACE" in str(value)}
        assert not bad, (
            "Replace stub model hashes after a trusted download before strict enforcement: "
            f"{list(bad.keys())}"
        )


class TestExpectedHashesJsonStructure:
    """SEC-D-02: committed hash manifest structure."""

    def test_repo_hash_file_has_required_keys(self):
        """expected_model_hashes.json documents model id, revision, and file keys."""
        path = Path(__file__).resolve().parent.parent / "python" / "expected_model_hashes.json"
        data = json.loads(path.read_text(encoding="utf-8"))
        assert data.get("model_id") == DEFAULT_MODEL_ID
        assert data.get("revision") == "main"
        assert isinstance(data.get("hashes"), dict)
        assert "config.json" in data["hashes"]
        assert "preprocessor_config.json" in data["hashes"]
        assert data.get("_hash_status") == "stub_pending_trusted_download"


class TestJsonOutput:

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
