# Sprint 1.10: Model Installer & First-Run Experience

**Sprint Goal:** Implement AI model download wizard on first run.
**Status:** Complete
**Date:** 2026-02-15

---

## Task Assignments & Status

### Senior Researcher (AI-401 through AI-405)
| Task | Description | Status |
|------|-------------|--------|
| AI-401 | Create model download script (Python) | Done |
| AI-402 | Hugging Face API integration (huggingface_hub snapshot_download) | Done |
| AI-403 | SHA256 checksum verification (delegated to HF library) | Done |
| AI-404 | Progress reporting (PROGRESS/STAGE stderr protocol) | Done |
| AI-405 | Store models in `~/.simplepicture3d/models/` | Done |

### Senior Engineer (BACK-901 through BACK-904)
| Task | Description | Status |
|------|-------------|--------|
| BACK-901 | Implement `download_model` Tauri command | Done |
| BACK-902 | Detect if models installed (first run check via `check_model`) | Done |
| BACK-903 | Handle download failures (error result with message) | Done |
| BACK-904 | Background download (non-blocking UI via async invoke) | Done |

### UI Specialist (UI-901 through UI-905)
| Task | Description | Status |
|------|-------------|--------|
| UI-901 | Create FirstRunWizard component | Done |
| UI-902 | Model download dialog (download now / skip) | Done |
| UI-903 | Progress bar with indeterminate animation | Done |
| UI-904 | Privacy notice (offline processing message) | Done |
| UI-905 | Onboarding tour (skip — deferred to future sprint) | Deferred |

### Junior Engineer #1 (JR1-901 through JR1-903)
| Task | Description | Status |
|------|-------------|--------|
| JR1-901 | Style wizard with multi-step layout + progress indicators | Done |
| JR1-902 | Add "Skip" button with confirmation dialog | Done |
| JR1-903 | Test wizard on slow connection | Deferred (manual) |

---

## Test Summary

| Suite | Tests | Status |
|-------|-------|--------|
| Rust (cargo test) | 133 passed, 6 ignored | Pass |
| Frontend (npm test) | 39 passed | Pass |
| Python (pytest) | 32 passed | Pass |
| **Total** | **204** | **All Pass** |

---

## Key Files Changed

### New Files
- `python/python/model_downloader.py` — Python model download script
- `src/components/FirstRunWizard.svelte` — First-run wizard component
- `python/tests/test_model_downloader.py` — 12 Python tests for model downloader

### Modified Files
- `src-tauri/src/lib.rs` — Added model management commands and structs
- `src/lib/tauri.ts` — Added model management IPC bindings
- `src/App.svelte` — Integrated FirstRunWizard with auto-check on mount
