//! Safe temp file I/O for image → Python handoff and other backend use.
//!
//! Paths are restricted to the system temp directory; path traversal is rejected
//! per docs/threat-model.md §2.3, §2.5.
//!
//! Used by load_image (image→temp) and Python bridge when implemented; allow dead_code until then.

#![allow(dead_code)]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Prefix for temp files created by this app (no path components).
const TEMP_PREFIX: &str = "simplepicture3d_";

/// Writes bytes to a new temp file in the system temp directory.
///
/// The file name is `{prefix}{unique}{suffix}`. `prefix` and `suffix` must
/// contain only ASCII alphanumeric, hyphen, or underscore (no path traversal).
///
/// # Arguments
/// * `prefix` - Filename prefix (e.g. `"img_"`). Sanitized; defaults to app prefix if empty.
/// * `suffix` - Filename suffix (e.g. `".png"`). Sanitized; no path components.
/// * `contents` - Bytes to write.
///
/// # Returns
/// Path to the created temp file.
///
/// # Errors
/// Returns error if temp dir is unavailable, sanitization fails, or write fails.
pub fn write_temp_file(
    prefix: &str,
    suffix: &str,
    contents: &[u8],
) -> io::Result<PathBuf> {
    let prefix = sanitize_temp_component(prefix).unwrap_or_else(|| TEMP_PREFIX.to_string());
    let suffix = sanitize_temp_component(suffix).unwrap_or_default();
    let temp_dir = std::env::temp_dir();
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_else(|_| "0".to_string());
    let path = temp_dir.join(format!("{}{}{}", prefix, unique, suffix));
    fs::write(&path, contents)?;
    Ok(path)
}

/// Reads a file only if it is under the system temp directory (no path traversal).
///
/// # Arguments
/// * `path` - Path to the file. Must resolve under `std::env::temp_dir()`.
///
/// # Returns
/// File contents as bytes.
///
/// # Errors
/// Returns error if path is outside temp dir, path cannot be canonicalized, or read fails.
pub fn read_file_in_temp_dir(path: &Path) -> io::Result<Vec<u8>> {
    let path = path.canonicalize().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("path cannot be canonicalized: {}", e),
        )
    })?;
    let temp_dir = std::env::temp_dir().canonicalize().map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("temp dir cannot be canonicalized: {}", e),
        )
    })?;
    if !path.starts_with(&temp_dir) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "path is outside system temp directory",
        ));
    }
    fs::read(&path)
}

/// Sanitizes a string for use as a temp filename component (prefix or suffix).
/// Allows only ASCII alphanumeric, hyphen, and underscore. Returns None if empty or invalid.
fn sanitize_temp_component(s: &str) -> Option<String> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let ok: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect();
    if ok.is_empty() || ok.contains("..") {
        return None;
    }
    Some(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_temp_file_creates_file() {
        let path = write_temp_file("test_", ".bin", b"hello").unwrap();
        assert!(path.is_file());
        assert!(path.parent().unwrap() == std::env::temp_dir());
        let content = fs::read(&path).unwrap();
        assert_eq!(content, b"hello");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn read_file_in_temp_dir_reads_allowed_path() {
        let path = write_temp_file("read_", ".bin", b"data").unwrap();
        let content = read_file_in_temp_dir(&path).unwrap();
        assert_eq!(content, b"data");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn read_file_in_temp_dir_rejects_outside_temp() {
        // Parent of temp dir is outside temp dir
        let outside = std::env::temp_dir().join("..").canonicalize().unwrap();
        let result = read_file_in_temp_dir(&outside);
        assert!(result.is_err());
    }

    #[test]
    fn sanitize_allows_safe_chars() {
        assert_eq!(
            sanitize_temp_component("img_01"),
            Some("img_01".to_string())
        );
        assert_eq!(
            sanitize_temp_component(".png"),
            Some(".png".to_string())
        );
    }

    #[test]
    fn sanitize_rejects_path_traversal() {
        assert!(sanitize_temp_component("..").is_none());
        assert!(sanitize_temp_component("a/../b").is_none());
    }
}
