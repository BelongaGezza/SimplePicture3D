# Rust Crates (SimplePicture3D)

**Purpose:** Guidance for Rust dependencies used in SimplePicture3D backend.

## Official Sources

| Crate | crates.io | Docs | Last Checked |
|-------|-----------|------|--------------|
| image | https://crates.io/crates/image | https://docs.rs/image | — |
| stl_io | https://crates.io/crates/stl_io | https://docs.rs/stl_io | — |
| tokio | https://crates.io/crates/tokio | https://docs.rs/tokio | — |
| serde | https://crates.io/crates/serde | https://docs.rs/serde | — |
| anyhow | https://crates.io/crates/anyhow | https://docs.rs/anyhow | — |

*Researcher: Add `obj` or equivalent OBJ crate; update "Last Checked" when verifying.*

---

## Crate Usage

| Crate | Purpose |
|-------|---------|
| image | Load PNG/JPG, validate, downsample, convert to RGB |
| stl_io | Write binary STL |
| tokio | Async runtime (Tauri uses it) |
| serde | Serialize/deserialize (settings, IPC, depth map JSON) |
| anyhow | Error handling with context |
| ndarray | Optional: depth map as 2D array |

---

## Research Tasks (Researcher)

- [ ] `image` crate: 8-bit vs 16-bit support, max dimensions, downsampling APIs
- [ ] OBJ writing: identify crate (e.g. `obj`, `wavefront_obj`) and format
- [ ] `stl_io`: binary vs ASCII, triangle/vertex limits
- [ ] anyhow vs thiserror: when to use which
- [ ] Version changes, deprecations since knowledge cutoff
