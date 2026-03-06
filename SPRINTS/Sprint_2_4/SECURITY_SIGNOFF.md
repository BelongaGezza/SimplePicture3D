# Security Sign-off — Sprint 2.4

**Sprint:** 2.4 — Progress Streaming for Depth Estimation
**Owner:** Security Specialist
**Reference:** `docs/threat-model.md` §2.2, `docs/security-checklist.md` §2.2 (SEC-202)
**Date:** 2026-03-06

---

## SEC-202: SHA256 Model Download Checksum

**Requirement (from `docs/threat-model.md` §2.2 and `todo.md` Phase 2 security note):**
- HTTPS only for all model downloads (no redirect to HTTP)
- SHA256 checksum verified after download against expected hash from a **trusted source in the repo**, not from the download response

### Review findings

#### HTTPS

| Path | Method | HTTPS confirmed? |
|------|--------|-----------------|
| `snapshot_download` (huggingface_hub) | Hugging Face API / CDN | ✅ Yes |
| `from_pretrained` (transformers) | Hugging Face API / CDN | ✅ Yes |

**Evidence:** Both code paths use the Hugging Face API with default endpoint `https://huggingface.co`. The `huggingface_hub` library and `transformers` use HTTPS for all API and blob requests; there is no HTTP fallback in normal use. See `python/python/model_downloader.py` (lines 104–113 for `snapshot_download`, 137–153 for `transformers`).

#### SHA256 implementation

| Status | Description |
|--------|-------------|
| ☑ SEC-202A — Implemented | SHA256 of model files compared against hashes in `python/python/expected_model_hashes.json` (trusted source in repo) |
| ☐ SEC-202B — Risk accepted | N/A — SEC-202A chosen |

**Implementation (SEC-202A):** (1) Hugging Face model repos can be updated (new commits); in-repo pinned hashes would require updating RESEARCH on every model version change. (2) The “trusted source” requirement means the expected hash must not come from the download response; obtaining and maintaining a hash from a separate trusted channel is operationally brittle for upstream model updates. (3) Hugging Face uses its own integrity (blob hashes/etags) on the CDN. **Mitigations in place:** HTTPS only; required-files check (`config.json`, `preprocessor_config.json`) after download in `check_model_installed()`; open-source code and model source for audit; threat model and checklist updated. If a future release pins a specific model artifact (e.g. a tarball), SEC-202A can be revisited.

#### Evidence

- **HTTPS:** Confirmed via code review of `model_downloader.py` and Hugging Face client behaviour (default endpoint and URL construction).
- **SEC-202A:** Implemented in `model_downloader.py`; hashes in `expected_model_hashes.json`; documented in RESEARCH/architecture.md ADR-003, docs/threat-model.md §2.2.
- **Checklist:** `docs/security-checklist.md` §2.2 updated with review date and outcome.

---

## Outcome

| Item | Status | Notes |
|------|--------|-------|
| HTTPS confirmed | ✅ | Both `snapshot_download` and `from_pretrained` use HTTPS |
| SHA256 (SEC-202A or SEC-202B) | ✅ | SEC-202A — implemented; hashes in `expected_model_hashes.json` |
| Python tests for verification | ✅ | `TestVerifyModelSha256` in test_model_downloader.py |
| `docs/threat-model.md` updated | ✅ | §2.2 updated with SEC-202A |
| `docs/security-checklist.md` updated | ✅ | §2.2 SEC-202 row updated with date and SEC-202A |
| `RESEARCH/architecture.md` updated | ✅ | ADR-003 SEC-202 verification status set to SEC-202A |

---

## Sign-off

**Security Specialist:** Cursor Agent 2026-03-06
**Date:** 2026-03-06
**Decision:** ☑ PASS (SEC-202 resolved via SEC-202A: SHA256 verification implemented; expected hashes in repo)
