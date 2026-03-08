# Sprint 2.5 — Gotchas

**Sprint:** 2.5 — Masking & Regional Adjustments  
**Last Updated:** 2026-03-08

Sprint-specific findings. Merge notable items to `RESEARCH/GOTCHAS.md` when relevant.

---

## Findings

- **BACK-1203 / to_soft_mask:** Box-blur with radius r averages over (2r+1)² pixels. For a single masked pixel with r=1, the center gets 1/9, not 1.0. Test assertions (e.g. `soft[center] > 0.5`) must match this; use “center > 0 and has_mid” for “feather produces partial values”. Similarly, `apply_adjustments_with_mask_feather_blend` with 3×1 and only center masked gives weight 1/3 at center → out[1] = 2/3; assert “blended toward adjusted” (e.g. > 0.5), not “nearly 1.0”.
- Manual QA (QA-1201–QA-1203) deferred until features were complete. Add any test-environment or reproducibility issues here when manual tests are run.
