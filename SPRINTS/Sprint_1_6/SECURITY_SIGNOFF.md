# Security Sign-off — Sprint 1.6

**Sprint:** 1.6 (Mesh Generation Algorithm)  
**Reviewed by:** Security Specialist — cursor-session-2026-02-07-security  
**Date:** 2026-02-07

---

## SEC-301: Integer overflow in vertex calculations

**Scope:** `src-tauri/src/mesh_generator.rs` and related call sites (`lib.rs` `get_mesh_data`).

### Code paths reviewed

| Location | Operation | Finding |
|----------|------------|--------|
| `validate_depth_input` | `expected_len = width * height` | **Safe:** Uses `(width as usize).checked_mul(height as usize)`; returns `Err` on overflow. |
| `depth_to_point_cloud` | `vertex_count = num_rows * num_cols` | **Safe:** Uses `num_rows.checked_mul(num_cols).context("vertex count overflow")?`. |
| `depth_to_point_cloud` | `idx = row * width_usize + col` | **Safe for current bounds:** `row` and `col` are derived from grid indices with `num_rows = div_ceil(height, step_y)` and `num_cols = div_ceil(width, step_x)`. Maximum `idx` is less than `width * height` (validated and bounded by `MAX_DIMENSION` 8192). On 64- and 32-bit, 8192×8192 fits in `usize`. |
| `Vec::with_capacity(vertex_count)` | Allocation size | **Safe:** `vertex_count` is from `checked_mul`; no unchecked allocation. |

### Recommendations

- **No unchecked overflow in hot path.** All dimension-derived sizes use `checked_mul` or are bounded by validated `width * height`.
- **MAX_DIMENSION (8192)** caps dimensions so that `width * height` and index arithmetic stay within `usize`. If `MAX_DIMENSION` is ever increased, consider using `checked_mul` / `checked_add` for the inner-loop index `row * width_usize + col` (e.g. when targeting very large meshes on 32-bit).
- Documented in `mesh_generator.rs`: `MAX_DIMENSION` and overflow-safe validation.

**SEC-301 status:** Complete. No changes required; findings documented.

---

## SEC-302: Validate depth map inputs before processing

**Scope:** Entry point for depth map into mesh generation: `depth_to_point_cloud` and `validate_depth_input`.

### Validation implemented

| Check | Implementation | Result |
|-------|----------------|--------|
| Dimensions > 0 | `ensure!(width > 0, ...)` and `ensure!(height > 0, ...)` | Rejects zero; returns `Err`, no panic. |
| Dimensions ≤ MAX_DIMENSION | `ensure!(width <= MAX_DIMENSION && height <= MAX_DIMENSION, ...)` | Rejects oversized; returns `Err`. |
| Depth length matches width×height | `expected_len = checked_mul(width, height)?; ensure!(depth.len() == expected_len, ...)` | Rejects length mismatch; overflow-safe. |
| Depth value range | Not rejected; mesh code uses `clamp(0.0, 1.0)` when sampling | Documented in `validate_depth_input` doc comment; out-of-range values clamped, no undefined behavior. |

### Call path

- `get_mesh_data` (lib.rs) uses `original.width`, `original.height` from app state (set by `load_image` / depth pipeline). It passes `adjusted` (depth slice) and dimensions to `depth_to_point_cloud`, which calls `validate_depth_input` first. Invalid dimensions or length result in `Err` propagated to frontend; no panic.

### Recommendations

- Validation is centralized in `validate_depth_input` and invoked at the single entry point `depth_to_point_cloud`. Any future direct callers of `depth_to_point_cloud` must pass validated dimensions and slice (or call `validate_depth_input` themselves).
- Depth value range: current contract is “normalized 0–1”; clamping is acceptable for MVP. If strict rejection is required later, add an optional check (e.g. `depth.iter().all(\|v\| *v >= 0.0 && *v <= 1.0)`) and document in architecture.

**SEC-302 status:** Complete. Input validation in place; behavior documented in code and this sign-off.

---

## Summary

| Task | Status | Notes |
|------|--------|--------|
| SEC-301 | Complete | Integer overflow review; checked_mul used for vertex count and expected length; index math safe for MAX_DIMENSION 8192. |
| SEC-302 | Complete | Dimensions and depth length validated; invalid input returns error; no panic. |

Security Specialist sign-off for Sprint 1.6 mesh generation (SEC-301, SEC-302): **Approved.**
