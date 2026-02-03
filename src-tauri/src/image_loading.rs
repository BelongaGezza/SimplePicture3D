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
    ImageFormat,
};
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum dimension per PRD F1.1 (8192×8192).
pub const MAX_DIMENSION: u32 = 8192;

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
        !is_blocklisted(&canonical),
        "access to system directories is not allowed"
    );
    Ok(canonical)
}

/// Returns true if the canonical path is under a blocklisted system directory (threat model §2.3).
fn is_blocklisted(canonical: &Path) -> bool {
    #[cfg(windows)]
    {
        let s = match canonical.to_str() {
            Some(x) => x.to_uppercase(),
            None => return true,
        };
        // Block system dirs; allow user dirs (e.g. C:\Users\..., D:\...).
        s.starts_with("C:\\WINDOWS\\")
            || s.starts_with("C:\\PROGRAM FILES\\")
            || s.starts_with("C:\\PROGRAM FILES (X86)\\")
            || s.starts_with("C:\\PROGRAMDATA\\")
            || s == "C:\\WINDOWS"
            || s == "C:\\PROGRAM FILES"
            || s == "C:\\PROGRAM FILES (X86)"
            || s == "C:\\PROGRAMDATA"
    }
    #[cfg(unix)]
    {
        let s = match canonical.to_str() {
            Some(x) => x,
            None => return true,
        };
        // macOS/iOS: /System, /usr; Linux etc: /usr, /etc
        s.starts_with("/System/")
            || s.starts_with("/usr/bin")
            || s.starts_with("/usr/sbin")
            || s.starts_with("/etc/")
            || s == "/System"
            || s == "/usr"
            || s == "/etc"
    }
    #[cfg(not(any(windows, unix)))]
    {
        let _ = canonical;
        false
    }
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

/// Encode RGB image as PNG base64 for preview.
fn rgb_to_preview_base64(rgb: &image::RgbImage) -> anyhow::Result<String> {
    use std::io::Cursor;
    let mut buf = Cursor::new(Vec::new());
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    let (w, h) = rgb.dimensions();
    encoder
        .write_image(rgb.as_raw(), w, h, ExtendedColorType::Rgb8)
        .context("encode preview PNG")?;
    let bytes = buf.into_inner();
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &bytes,
    ))
}

/// Full load_image implementation: validate path, read, magic-check, decode, downsample, RGB, response.
pub fn load_image_impl(path: String) -> anyhow::Result<LoadImageOut> {
    let start = std::time::Instant::now();
    let canonical = validate_path(&path)?;
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
