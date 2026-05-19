// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Image loading for load_image Tauri command (BACK-101–105).
//!
//! Path validation (SEC-101), magic-byte validation (SEC-102), decode, downsampling,
//! RGB normalization, and structured response for frontend.
//!
//! **Path edge cases (JR2-102):** Paths with spaces and Unicode filenames are supported
//! (Rust/OS handle them). Path traversal (`..`) is resolved by `Path::canonicalize()` before
//! any read. On Windows, paths longer than MAX_PATH (260) may require the `\\?\` prefix
//! when provided by the system (e.g. file picker); we rely on canonicalization and OS APIs.

use anyhow::{ensure, Context};
use image::imageops::FilterType;
use image::{
    load_from_memory_with_format, DynamicImage, ExtendedColorType, GenericImageView, ImageEncoder,
    ImageFormat, ImageReader,
};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

/// Maximum dimension per PRD F1.1 (8192×8192).
pub const MAX_DIMENSION: u32 = 8192;

/// Maximum image file size (in bytes) accepted by the depth pipeline (SEC-D-03).
///
/// **Rationale:** A photographic 8K PNG is typically <50 MB, JPEG <15 MB. 128 MiB
/// leaves headroom for high-bit-depth or noisy 8K content while preventing local DoS
/// via pathological multi-GB files passed to the Python subprocess. Aligns with the
/// `MAX_DIMENSION = 8192` policy and the <2 GB memory budget for 4K (PRD F1.1).
/// Documented in `docs/threat-model.md` §2.4.
pub const MAX_IMAGE_FILE_BYTES: u64 = 128 * 1024 * 1024;

/// PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A (docs/threat-model.md §2.4).
const PNG_SIGNATURE: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
/// JPEG SOI: FF D8 FF.
const JPEG_SIGNATURE: &[u8] = &[0xFF, 0xD8, 0xFF];

/// Result of loading an image: dimensions, metadata, and preview data for frontend.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadImageOut {
    pub ok: bool,
    pub width: u32,
    pub height: u32,
    pub file_size_bytes: u64,
    pub downsampled: bool,
    /// Base64-encoded RGB image (PNG) for preview. Frontend can use as data URL.
    pub preview_base64: String,
}

/// Validates path: canonicalize, must be file, not under blocklisted system dirs (SEC-101).
fn validate_path(path: &str) -> anyhow::Result<PathBuf> {
    let p = Path::new(path.trim());
    ensure!(!p.as_os_str().is_empty(), "image path must be non-empty");
    let canonical = p
        .canonicalize()
        .context("path could not be resolved (missing or inaccessible)")?;
    ensure!(canonical.is_file(), "path is not a file");
    ensure!(
        !crate::path_security::is_blocklisted(&canonical),
        "access to system directories is not allowed"
    );
    Ok(canonical)
}

/// Enforces the maximum image file size before any read into RAM (SEC-D-03).
///
/// Uses `fs::metadata` to avoid loading a multi-GB attacker file into memory.
fn enforce_max_file_size(canonical: &Path) -> anyhow::Result<u64> {
    let meta = fs::metadata(canonical).context("read image metadata")?;
    let size = meta.len();
    ensure!(
        size <= MAX_IMAGE_FILE_BYTES,
        "image file too large: {} bytes exceeds {}-byte limit (~{} MiB)",
        size,
        MAX_IMAGE_FILE_BYTES,
        MAX_IMAGE_FILE_BYTES / (1024 * 1024)
    );
    Ok(size)
}

/// Reads image dimensions from the file header without decoding pixel data (SEC-D-03).
///
/// `image::ImageReader::into_dimensions` parses only header chunks (PNG IHDR / JPEG SOF),
/// avoiding decompression-bomb DoS.
fn header_dimensions(bytes: &[u8]) -> anyhow::Result<(u32, u32)> {
    let reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .context("guess image format")?;
    reader
        .into_dimensions()
        .context("read image dimensions from header")
}

/// Enforces the header-declared dimension cap (SEC-D-03). Aligns with `MAX_DIMENSION`.
///
/// Used by `read_image_bytes_for_depth` to reject oversized images **before** spawning
/// the Python subprocess. `load_image_impl` continues to downsample (BACK-103).
fn enforce_max_dimensions(w: u32, h: u32) -> anyhow::Result<()> {
    ensure!(
        w > 0 && h > 0,
        "image has invalid header dimensions ({}×{})",
        w,
        h
    );
    ensure!(
        w <= MAX_DIMENSION && h <= MAX_DIMENSION,
        "image dimensions too large: {}×{} exceeds {}×{} limit",
        w,
        h,
        MAX_DIMENSION,
        MAX_DIMENSION
    );
    Ok(())
}

/// Validates magic bytes (SEC-102). Returns format for decode or error.
fn validate_magic_bytes(bytes: &[u8]) -> anyhow::Result<ImageFormat> {
    ensure!(bytes.len() >= 8, "file too short to be a valid image");
    if bytes.starts_with(PNG_SIGNATURE) {
        return Ok(ImageFormat::Png);
    }
    if bytes.len() >= 3 && bytes.starts_with(JPEG_SIGNATURE) {
        return Ok(ImageFormat::Jpeg);
    }
    anyhow::bail!("unsupported or invalid image format (expected PNG or JPEG)");
}

/// Decode image from bytes after magic-byte check. Validates integrity (BACK-102).
/// Dimensions are validated after decode; images >8K are downsampled in downsample_if_needed (BACK-103).
fn decode_image(bytes: &[u8], format: ImageFormat) -> anyhow::Result<DynamicImage> {
    let img = load_from_memory_with_format(bytes, format)
        .context("image decode failed (file may be corrupt)")?;
    let (w, h) = img.dimensions();
    ensure!(w > 0 && h > 0, "image has invalid dimensions");
    Ok(img)
}

/// Downsample to fit within MAX_DIMENSION, preserve aspect ratio (BACK-103).
fn downsample_if_needed(img: DynamicImage) -> (DynamicImage, bool) {
    let (w, h) = img.dimensions();
    if w <= MAX_DIMENSION && h <= MAX_DIMENSION {
        return (img, false);
    }
    let (nw, nh) = scale_down_dimensions(w, h, MAX_DIMENSION);
    let resized = img.resize_exact(nw, nh, FilterType::Triangle);
    (resized, true)
}

fn scale_down_dimensions(w: u32, h: u32, max: u32) -> (u32, u32) {
    if w <= max && h <= max {
        return (w, h);
    }
    let rw = max as f64 / w as f64;
    let rh = max as f64 / h as f64;
    let r = rw.min(rh);
    let nw = (w as f64 * r).round() as u32;
    let nh = (h as f64 * r).round() as u32;
    (nw.max(1), nh.max(1))
}

/// Normalize to RGB for preview and future pipeline (BACK-104).
fn to_rgb8(img: DynamicImage) -> image::RgbImage {
    img.to_rgb8()
}

/// Encode RGB image as PNG bytes. Used for preview base64 and for Python bridge (temp file).
pub fn rgb_to_png_bytes(rgb: &image::RgbImage) -> anyhow::Result<Vec<u8>> {
    use std::io::Cursor;
    let mut buf = Cursor::new(Vec::new());
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    let (w, h) = rgb.dimensions();
    encoder
        .write_image(rgb.as_raw(), w, h, ExtendedColorType::Rgb8)
        .context("encode PNG")?;
    Ok(buf.into_inner())
}

/// Encode RGB image as PNG base64 for preview.
fn rgb_to_preview_base64(rgb: &image::RgbImage) -> anyhow::Result<String> {
    let bytes = rgb_to_png_bytes(rgb)?;
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &bytes,
    ))
}

/// Reads and validates an image file for depth estimation (BACK-301, SEC-D-03).
///
/// Validation order (fail-closed, cheapest first):
/// 1. Path canonicalize + blocklist (SEC-101).
/// 2. File size ≤ `MAX_IMAGE_FILE_BYTES` via `fs::metadata` — checked **before** reading
///    bytes so a multi-GB hostile file cannot exhaust RAM (SEC-D-03 / threat model §2.4).
/// 3. Magic bytes — PNG/JPEG only (SEC-102).
/// 4. Header-declared dimensions ≤ `MAX_DIMENSION` — checked **before** the Python
///    subprocess is spawned so decompression-bomb images never reach the AI worker
///    (SEC-D-03). `ImageReader::into_dimensions` reads header chunks only and does
///    not decode pixels.
///
/// Returns raw file bytes for the Python bridge. Does not decode or downsample.
pub fn read_image_bytes_for_depth(path: &str) -> anyhow::Result<Vec<u8>> {
    let canonical = validate_path(path)?;
    enforce_max_file_size(&canonical)?;
    let bytes = fs::read(&canonical).context("read image file")?;
    validate_magic_bytes(&bytes)?;
    let (w, h) = header_dimensions(&bytes)?;
    enforce_max_dimensions(w, h)?;
    Ok(bytes)
}

/// Full load_image implementation: validate path, read, magic-check, decode, downsample, RGB, response.
///
/// SEC-D-03: the same file-size cap as `read_image_bytes_for_depth` is enforced here
/// to bound the worst-case `fs::read` allocation. Header-declared dimensions are not
/// pre-checked because `downsample_if_needed` already enforces `MAX_DIMENSION` after
/// decode (BACK-103).
pub fn load_image_impl(path: String) -> anyhow::Result<LoadImageOut> {
    let start = std::time::Instant::now();
    let canonical = validate_path(&path)?;
    enforce_max_file_size(&canonical)?;
    let bytes = fs::read(&canonical).context("read image file")?;
    let file_size_bytes = bytes.len() as u64;
    let format = validate_magic_bytes(&bytes)?;
    let img = decode_image(&bytes, format)?;
    let (img, downsampled) = downsample_if_needed(img);
    let rgb = to_rgb8(img);
    let (width, height) = rgb.dimensions();
    let preview_base64 = rgb_to_preview_base64(&rgb)?;
    // JR2-104: load duration at debug level; no user paths or image content (threat model §2.1).
    log::debug!(
        "load_image completed in {:?} (dimensions {}×{})",
        start.elapsed(),
        width,
        height
    );
    Ok(LoadImageOut {
        ok: true,
        width,
        height,
        file_size_bytes,
        downsampled,
        preview_base64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_down_dimensions_fits_max() {
        let (w, h) = scale_down_dimensions(16384, 8192, MAX_DIMENSION);
        assert!(w <= MAX_DIMENSION && h <= MAX_DIMENSION);
        assert_eq!(w, 8192);
        assert_eq!(h, 4096);
    }

    #[test]
    fn scale_down_dimensions_under_max_unchanged() {
        let (w, h) = scale_down_dimensions(100, 100, MAX_DIMENSION);
        assert_eq!(w, 100);
        assert_eq!(h, 100);
    }

    #[test]
    fn png_magic_detected() {
        let mut bytes = vec![0u8; 16];
        bytes[..8].copy_from_slice(PNG_SIGNATURE);
        let fmt = validate_magic_bytes(&bytes).unwrap();
        assert!(matches!(fmt, ImageFormat::Png));
    }

    #[test]
    fn jpeg_magic_detected() {
        let bytes = vec![0xFF, 0xD8, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00];
        let fmt = validate_magic_bytes(&bytes).unwrap();
        assert!(matches!(fmt, ImageFormat::Jpeg));
    }

    #[test]
    fn invalid_magic_rejected() {
        assert!(validate_magic_bytes(&[0x00, 0x00, 0x00]).is_err());
        assert!(validate_magic_bytes(b"GIF89a----").is_err());
    }

    #[test]
    fn rgb_to_png_bytes_produces_valid_png() {
        let rgb = image::ImageBuffer::from_fn(2, 2, |_, _| image::Rgb([100u8, 150, 200]));
        let bytes = rgb_to_png_bytes(&rgb).unwrap();
        assert!(!bytes.is_empty());
        assert!(bytes.starts_with(PNG_SIGNATURE), "output must be valid PNG");
        let fmt = validate_magic_bytes(&bytes).unwrap();
        assert!(matches!(fmt, ImageFormat::Png));
    }

    // JR2-101: Unit tests for image loading — success and error paths.

    /// Success path: valid PNG returns dimensions and ok (BACK-101, BACK-105).
    #[test]
    fn load_image_impl_valid_png_returns_dimensions() {
        let temp = std::env::temp_dir().join("jr2_101_valid.png");
        let (w, h) = (64u32, 48u32);
        let img = image::ImageBuffer::from_fn(w, h, |x, y| {
            image::Rgb(if (x + y) % 2 == 0 {
                [255u8, 0, 0]
            } else {
                [0u8, 0, 255]
            })
        });
        img.save(&temp).expect("write test PNG");
        let path = temp.to_string_lossy().to_string();
        let result = load_image_impl(path.clone());
        let _ = std::fs::remove_file(&temp);
        let r = result.expect("valid PNG should load");
        assert!(r.ok);
        assert_eq!(r.width, w);
        assert_eq!(r.height, h);
        assert!(!r.preview_base64.is_empty());
        assert!(r.file_size_bytes > 0);
    }

    /// Error path: nonexistent path returns error (BACK-101).
    #[test]
    fn load_image_impl_invalid_path_returns_error() {
        let path = std::env::temp_dir().join("nonexistent_jr2_101_12345.png");
        let result = load_image_impl(path.to_string_lossy().to_string());
        let err = result.expect_err("nonexistent path should fail");
        assert!(!err.to_string().is_empty());
    }

    /// Error path: file with wrong magic bytes returns error (BACK-102, SEC-102).
    #[test]
    fn load_image_impl_corrupt_or_non_image_returns_error() {
        let temp = std::env::temp_dir().join("jr2_101_not_an_image.png");
        std::fs::write(&temp, b"not an image; wrong magic bytes").expect("write");
        let path = temp.to_string_lossy().to_string();
        let result = load_image_impl(path.clone());
        let _ = std::fs::remove_file(&temp);
        let err = result.expect_err("non-image file should fail");
        let lower = err.to_string().to_lowercase();
        assert!(
            lower.contains("invalid") || lower.contains("corrupt") || lower.contains("format"),
            "error should mention invalid/corrupt/format: {}",
            err
        );
    }

    // JR2-102: Path edge cases — Unicode, spaces, path traversal resolved by canonicalize.

    /// Paths with spaces work (canonicalize + read).
    #[test]
    fn load_image_impl_path_with_spaces() {
        let temp = std::env::temp_dir().join("jr2 102 test image.png");
        let (w, h) = (2u32, 2u32);
        let img = image::ImageBuffer::from_fn(w, h, |_, _| image::Rgb([100u8, 100, 100]));
        img.save(&temp).expect("write test PNG");
        let path = temp.to_string_lossy().to_string();
        let result = load_image_impl(path.clone());
        let _ = std::fs::remove_file(&temp);
        let r = result.expect("path with spaces should load");
        assert!(r.ok);
        assert_eq!(r.width, w);
        assert_eq!(r.height, h);
    }

    /// Paths with Unicode in filename work (Rust/OS handle UTF-8 or UTF-16).
    #[test]
    fn load_image_impl_path_with_unicode() {
        let name = "jr2_102_画像.png";
        let temp = std::env::temp_dir().join(name);
        let (w, h) = (2u32, 2u32);
        let img = image::ImageBuffer::from_fn(w, h, |_, _| image::Rgb([80u8, 80, 80]));
        img.save(&temp).expect("write test PNG");
        let path = temp.to_string_lossy().to_string();
        let result = load_image_impl(path.clone());
        let _ = std::fs::remove_file(&temp);
        let r = result.expect("path with Unicode should load");
        assert!(r.ok);
        assert_eq!(r.width, w);
        assert_eq!(r.height, h);
    }

    /// Path with ".." canonicalizes to real path; no traversal (SEC-101, threat model §2.3).
    #[test]
    fn load_image_impl_path_traversal_resolved_by_canonicalize() {
        let temp_dir = std::env::temp_dir();
        let subdir = temp_dir.join("jr2_102_subdir");
        let _ = std::fs::create_dir_all(&subdir);
        let image_path = subdir.join("image.png");
        let (w, h) = (3u32, 3u32);
        let img = image::ImageBuffer::from_fn(w, h, |_, _| image::Rgb([60u8, 60, 60]));
        img.save(&image_path).expect("write test PNG");
        // Path like .../jr2_102_subdir/../jr2_102_subdir/image.png resolves to same file.
        let path_with_dotdot = subdir.join("..").join("jr2_102_subdir").join("image.png");
        let path = path_with_dotdot.to_string_lossy().to_string();
        let result = load_image_impl(path);
        let _ = std::fs::remove_file(&image_path);
        let _ = std::fs::remove_dir(&subdir);
        let r = result.expect("canonicalized path with .. should load");
        assert!(r.ok);
        assert_eq!(r.width, w);
        assert_eq!(r.height, h);
    }

    // SEC-D-03: max file size + dimension cap enforced before Python subprocess.

    /// `enforce_max_file_size` accepts files at exactly the limit and rejects above it.
    /// Uses a sparse file (`File::set_len`) so the test does not consume 128 MiB of disk.
    #[test]
    fn enforce_max_file_size_rejects_over_limit() {
        use std::fs::File;
        let temp = std::env::temp_dir().join("sec_d_03_oversize.bin");
        let f = File::create(&temp).expect("create sparse file");
        f.set_len(MAX_IMAGE_FILE_BYTES + 1)
            .expect("set sparse length");
        drop(f);
        let result = enforce_max_file_size(&temp);
        let _ = std::fs::remove_file(&temp);
        let err = result.expect_err("file 1 byte over limit must be rejected");
        let msg = err.to_string().to_lowercase();
        assert!(
            msg.contains("too large") && msg.contains("limit"),
            "error message should describe size cap: {}",
            err
        );
    }

    /// `enforce_max_file_size` returns the file size for files at or under the limit.
    #[test]
    fn enforce_max_file_size_accepts_under_limit() {
        let temp = std::env::temp_dir().join("sec_d_03_under_limit.bin");
        std::fs::write(&temp, b"some bytes").expect("write small file");
        let size = enforce_max_file_size(&temp).expect("under-limit file must be accepted");
        let _ = std::fs::remove_file(&temp);
        assert_eq!(size, b"some bytes".len() as u64);
    }

    /// `enforce_max_dimensions` rejects anything wider/taller than MAX_DIMENSION.
    #[test]
    fn enforce_max_dimensions_rejects_over_cap() {
        assert!(enforce_max_dimensions(MAX_DIMENSION + 1, 1).is_err());
        assert!(enforce_max_dimensions(1, MAX_DIMENSION + 1).is_err());
        assert!(enforce_max_dimensions(MAX_DIMENSION + 1, MAX_DIMENSION + 1).is_err());
        assert!(enforce_max_dimensions(0, 1).is_err());
        assert!(enforce_max_dimensions(1, 0).is_err());
    }

    /// `enforce_max_dimensions` accepts dimensions at exactly the cap.
    #[test]
    fn enforce_max_dimensions_accepts_at_cap() {
        assert!(enforce_max_dimensions(MAX_DIMENSION, MAX_DIMENSION).is_ok());
        assert!(enforce_max_dimensions(1, 1).is_ok());
    }

    /// `header_dimensions` returns the header-declared size without decoding pixels.
    #[test]
    fn header_dimensions_reads_png_header_without_decoding() {
        let (w, h) = (32u32, 17u32);
        let img = image::ImageBuffer::from_fn(w, h, |_, _| image::Rgb([10u8, 20, 30]));
        let bytes = rgb_to_png_bytes(&img).expect("encode PNG");
        let (rw, rh) = header_dimensions(&bytes).expect("read header dims");
        assert_eq!((rw, rh), (w, h));
    }

    /// `read_image_bytes_for_depth` rejects images whose header dimensions exceed MAX_DIMENSION.
    /// Uses an 8193×10 PNG (just over the cap); ~250 KB on disk so the size cap does not fire first.
    #[test]
    fn read_image_bytes_for_depth_rejects_oversize_dimensions() {
        let (w, h) = (MAX_DIMENSION + 1, 10u32);
        let temp = std::env::temp_dir().join("sec_d_03_oversize_dims.png");
        let img = image::ImageBuffer::from_fn(w, h, |x, _| {
            image::Rgb([(x % 256) as u8, 0u8, 0u8])
        });
        img.save(&temp).expect("write oversize-width PNG");
        let result = read_image_bytes_for_depth(&temp.to_string_lossy());
        let _ = std::fs::remove_file(&temp);
        let err = result.expect_err("8193×10 image must be rejected before Python");
        let msg = err.to_string().to_lowercase();
        assert!(
            msg.contains("dimensions too large"),
            "error must mention dimension cap: {}",
            err
        );
    }

    /// `read_image_bytes_for_depth` accepts in-bounds images and returns the raw file bytes
    /// (validates the SEC-D-03 happy path: path + size + magic + dimensions all pass).
    #[test]
    fn read_image_bytes_for_depth_accepts_within_limits() {
        let temp = std::env::temp_dir().join("sec_d_03_within_limits.png");
        let img = image::ImageBuffer::from_fn(8u32, 8u32, |_, _| image::Rgb([42u8, 42, 42]));
        img.save(&temp).expect("write small PNG");
        let bytes =
            read_image_bytes_for_depth(&temp.to_string_lossy()).expect("in-bounds PNG must pass");
        let _ = std::fs::remove_file(&temp);
        assert!(bytes.starts_with(PNG_SIGNATURE), "raw PNG bytes returned");
        let (w, h) = header_dimensions(&bytes).expect("header readable");
        assert_eq!((w, h), (8, 8));
    }

    /// `read_image_bytes_for_depth` rejects oversized files via the metadata-based size cap,
    /// **before** any bytes are read into RAM. Uses a sparse file so disk usage is negligible.
    #[test]
    fn read_image_bytes_for_depth_rejects_oversize_file_via_metadata() {
        use std::fs::File;
        // Sparse file: length > limit, but no real bytes allocated.
        let temp = std::env::temp_dir().join("sec_d_03_sparse_oversize.png");
        let f = File::create(&temp).expect("create sparse file");
        f.set_len(MAX_IMAGE_FILE_BYTES + 1)
            .expect("set sparse length");
        drop(f);
        let result = read_image_bytes_for_depth(&temp.to_string_lossy());
        let _ = std::fs::remove_file(&temp);
        let err = result.expect_err("sparse file over size cap must be rejected");
        let msg = err.to_string().to_lowercase();
        assert!(
            msg.contains("too large"),
            "error should be the size-cap error (fired before magic/header): {}",
            err
        );
    }

    // JR2-103: Downsampling test — image >8K is downsampled, dimensions ≤ 8192, aspect ratio preserved.

    /// Load image with one dimension >8K; assert downsampled, dimensions ≤ MAX_DIMENSION, aspect ratio preserved.
    #[test]
    fn load_image_impl_downsamples_over_8k() {
        // 8193×10: width just over 8192, so scale by 8192/8193 → 8192×10 (aspect preserved).
        let (orig_w, orig_h) = (8193u32, 10u32);
        let temp = std::env::temp_dir().join("jr2_103_over8k.png");
        let img = image::ImageBuffer::from_fn(orig_w, orig_h, |x, y| {
            image::Rgb([(x % 256) as u8, (y % 256) as u8, 128u8])
        });
        img.save(&temp).expect("write test PNG");
        let path = temp.to_string_lossy().to_string();
        let result = load_image_impl(path.clone());
        let _ = std::fs::remove_file(&temp);
        let r = result.expect(">8K image should load (downsampled)");
        assert!(r.ok, "load should succeed");
        assert!(r.downsampled, "image should be marked as downsampled");
        assert!(
            r.width <= MAX_DIMENSION && r.height <= MAX_DIMENSION,
            "dimensions {}×{} must be ≤ {}",
            r.width,
            r.height,
            MAX_DIMENSION
        );
        // Aspect ratio preserved: orig_w/orig_h ≈ r.width/r.height (within one-pixel rounding).
        let orig_ratio = orig_w as f64 / orig_h as f64;
        let new_ratio = r.width as f64 / r.height as f64;
        let ratio_diff = (orig_ratio - new_ratio).abs();
        assert!(
            ratio_diff < 0.2,
            "aspect ratio preserved: orig {}×{} ratio {}, got {}×{} ratio {}",
            orig_w,
            orig_h,
            orig_ratio,
            r.width,
            r.height,
            new_ratio
        );
    }
}
