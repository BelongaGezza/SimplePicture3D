# Rust Crates (SimplePicture3D)

**Purpose:** Guidance for Rust dependencies used in SimplePicture3D backend.

## Official Sources

| Crate | Version | crates.io | Docs | Last Checked |
|-------|---------|-----------|------|--------------|
| image | 0.25.9 | https://crates.io/crates/image | https://docs.rs/image | 2026-02-01 |
| stl_io | 0.10.0 | https://crates.io/crates/stl_io | https://docs.rs/stl_io | 2026-02-01 |
| tokio | 1.49.0 | https://crates.io/crates/tokio | https://docs.rs/tokio | 2026-02-01 |
| serde | 1.0.228 | https://crates.io/crates/serde | https://docs.rs/serde | 2026-02-01 |
| anyhow | 1.0.100 | https://crates.io/crates/anyhow | https://docs.rs/anyhow | 2026-02-01 |
| obj-exporter | 0.2.0 | https://crates.io/crates/obj-exporter | https://docs.rs/obj-exporter | 2026-02-01 |
| wavefront_obj | (see note) | https://crates.io/crates/wavefront_obj | https://docs.rs/wavefront_obj | 2026-02-01 |

*Researcher: Crate versions above are as of Last Checked; pin in Cargo.toml and run `cargo update` periodically.*

### References / Further reading

| Source | URL | Last Checked |
|--------|-----|--------------|
| Rust Optimization (Achieving warp speed with Rust) | https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1 | 2026-02-02 |

**Rust Optimization (jFransham gist):** Practical optimization guide for Rust: prefer profiling (e.g. `perf`, clippy) over blind optimization; optimize hot paths, not one-time costs; improve algorithms first. Low-level tips: keep data in cache (flat `Vec` for matrices instead of `Vec<Vec<T>>`, consider `smallvec`/stack-based types); keep data in registers (avoid unnecessary pointer writes); avoid `Box<Trait>` in favour of `&mut Trait` or recursive generics; use `assert!` upfront to help LLVM elide bounds checks; use LTO for release; avoid `#[inline(always)]` unless benchmarked; leverage instruction-level parallelism and autovectorization. Includes a `from_str_radix` case study and links to BurntSushi’s ripgrep post-mortem. Relevant when tuning image/depth/mesh code paths.

---

## As-built: STL/OBJ export

**SimplePicture3D does not use `stl_io` or `obj-exporter`.** Binary STL and ASCII OBJ (with optional .mtl) are implemented as **custom** writers in `src-tauri/src/mesh_generator.rs` (Sprint 1.8/1.9, ADR-008). This keeps mesh and export logic in one module and avoids extra crate dependencies. If you need to swap to a crate later, the table below documents the standard options.

---

## Crate Usage

| Crate | Purpose |
|-------|---------|
| image | Load PNG/JPG, validate, downsample, convert to RGB; supports 8-bit and many formats (AVIF, BMP, GIF, JPEG, PNG, TIFF, WebP, etc.). Use `ImageReader::open()` / `decode()`, `resize()` for downsampling. **Used in this project.** |
| stl_io | Read/write STL; **writes binary STL**; reads binary and ASCII. Minimal deps (byteorder, float-cmp). **Not used;** we use custom STL in mesh_generator.rs. |
| tokio | Async runtime (Tauri uses it). Use for non-blocking I/O and subprocess handling. **Used.** |
| serde | Serialize/deserialize (settings, IPC, depth map JSON). Use with `serde_json` for JSON. **Used.** |
| anyhow | Error handling with context (`anyhow::Result`, `.context()`, `?`). Prefer for application code; use `thiserror` for library error types. **Used.** |
| obj-exporter | **OBJ export:** Write Wavefront OBJ (e.g. `export_to_file()`). Depends on `wavefront_obj`. **Not used;** we use custom OBJ in mesh_generator.rs. |
| wavefront_obj | Parse OBJ; define `ObjSet`/`Object`/`Geometry` for export. **Not used.** |

---

## OBJ Export Recommendation

- **Recommended for writing OBJ:** Use **obj-exporter** (0.2.0) to export `ObjSet` to file or stream. Build mesh data (vertices, optional normals) and construct `wavefront_obj` types, then call `obj_exporter::export_to_file()` (or equivalent).
- **Alternative:** Write ASCII OBJ manually: emit `v x y z` and `f i j k` lines from your vertex/index buffers; no extra crate required, full control over format and .mtl.

---

## Large Buffers and Performance

- **Image/depth:** `image` crate uses `ImageBuffer` (vec of pixels); dimensions are (width, height). For very large images (e.g. 8K), consider downsampling before depth or mesh steps to stay within memory budget (<2GB for 4K per PRD).
- **Optional:** `ndarray` can represent depth as 2D array for numeric ops; not required if processing with raw `Vec<f32>` or slices.

---

## Deprecations / MSRV

- **image:** Default features include many formats and `rayon`; for libraries, consider `default-features = false` and explicit format features. No major breaking changes noted in 0.25.x.
- **stl_io:** 2021 edition; stable for read/write STL.
- **tokio:** MSRV tracked on crates.io; 1.49 is current stable.
- **serde / anyhow:** Stable; MSRV typically rustc 1.39+ (anyhow), serde 1.x unchanged.

---

## Research Tasks (Researcher)

- [x] `image` crate: format support, downsampling (`resize`) — documented above.
- [x] OBJ writing: **obj-exporter** + **wavefront_obj** or manual OBJ — documented above.
- [x] `stl_io`: binary write, read binary/ASCII — documented above.
- [x] anyhow vs thiserror: anyhow for app code, thiserror for library types — documented above.
- [x] Version and Last checked dates — in table above.
