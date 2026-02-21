# Copyright (c) 2026 SimplePicture3D Contributors
# SPDX-License-Identifier: MIT

# Pytest conftest: make depth_estimator importable when running from repo root.
# Run from repo root: SP3D_USE_STUB=1 pytest python/ -v
# Or from python/: SP3D_USE_STUB=1 PYTHONPATH=python pytest tests/ -v
import sys
from pathlib import Path

_tests_dir = Path(__file__).resolve().parent
_python_pkg = _tests_dir.parent / "python"
if _python_pkg.is_dir() and str(_python_pkg) not in sys.path:
    sys.path.insert(0, str(_python_pkg))
