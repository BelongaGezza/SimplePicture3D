// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Shared path security helpers (SEC-D-05, SEC-D-10).
//!
//! Centralises system-directory blocklists and import-path validation so
//! `load_image`, export, preset import, and mask import share one policy.

use std::path::{Path, PathBuf};

/// Returns true if `canonical` is under a blocklisted system directory.
pub fn is_blocklisted(canonical: &Path) -> bool {
    #[cfg(windows)]
    {
        let s = match canonical.to_str() {
            Some(x) => x.to_uppercase(),
            None => return true,
        };
        let blocked = [
            "C:\\WINDOWS",
            "C:\\PROGRAM FILES",
            "C:\\PROGRAM FILES (X86)",
            "C:\\PROGRAMDATA",
        ];
        for prefix in &blocked {
            if s == *prefix || s.starts_with(&format!("{}\\", prefix)) {
                return true;
            }
        }
        false
    }
    #[cfg(unix)]
    {
        let s = match canonical.to_str() {
            Some(x) => x,
            None => return true,
        };
        let blocked = [
            "/System",
            "/etc",
            "/usr",
            "/bin",
            "/sbin",
            "/boot",
            "/lib",
            "/lib64",
            "/proc",
            "/sys",
            "/dev",
            "/run",
        ];
        for prefix in &blocked {
            if s == *prefix || s.starts_with(&format!("{}/", prefix)) {
                return true;
            }
        }
        // [PLATFORM: macOS] ─── begin ─────────────────────────────────────────
        // Canonicalize resolves /etc → /private/etc; mirror blocklist prefixes.
        #[cfg(target_os = "macos")]
        {
            let private_mirrors = [
                "/private/etc",
                "/private/bin",
                "/private/sbin",
                "/private/usr",
                "/private/dev",
                "/private/proc",
                "/private/sys",
            ];
            for prefix in &private_mirrors {
                if s == *prefix || s.starts_with(&format!("{}/", prefix)) {
                    return true;
                }
            }
        }
        // [PLATFORM: macOS] ─── end ───────────────────────────────────────────
        false
    }
    #[cfg(not(any(windows, unix)))]
    {
        let _ = canonical;
        false
    }
}

/// Validates a user-selected import file (preset or mask JSON).
///
/// Path must resolve to a regular file with the expected extension and must not
/// be under blocklisted system directories.
pub fn validate_import_file_path(path: &str, extension: &str) -> Result<PathBuf, String> {
    if path.trim().is_empty() {
        return Err("Path must be non-empty".to_string());
    }
    let canonical = Path::new(path.trim())
        .canonicalize()
        .map_err(|_| "File does not exist or is not accessible".to_string())?;
    if !canonical.is_file() {
        return Err("Path is not a file".to_string());
    }
    match canonical.extension().and_then(|e| e.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case(extension) => {}
        _ => return Err(format!("File must have .{} extension", extension)),
    }
    if is_blocklisted(&canonical) {
        log::warn!(
            "SEC-D-05: Blocked import from system directory: {}",
            canonical.display()
        );
        return Err("Cannot read files from system directories".to_string());
    }
    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn validate_import_rejects_non_json_extension() {
        let dir = std::env::temp_dir();
        let path = dir.join("sp3d_path_sec_test.txt");
        std::fs::File::create(&path)
            .unwrap()
            .write_all(b"x")
            .unwrap();
        let err = validate_import_file_path(path.to_str().unwrap(), "json").unwrap_err();
        assert!(err.contains("json"));
        let _ = std::fs::remove_file(path);
    }

    /// SEC-D-10: Unix blocklist matches export policy (load_image + export share this helper).
    #[cfg(unix)]
    #[test]
    fn is_blocklisted_rejects_unix_system_prefixes() {
        assert!(is_blocklisted(Path::new("/etc/passwd")));
        assert!(is_blocklisted(Path::new("/bin/sh")));
        assert!(is_blocklisted(Path::new("/sbin/init")));
        assert!(is_blocklisted(Path::new("/proc/self/cmdline")));
        assert!(is_blocklisted(Path::new("/sys/class")));
        assert!(is_blocklisted(Path::new("/dev/null")));
        assert!(is_blocklisted(Path::new("/run/systemd")));
        #[cfg(target_os = "macos")]
        assert!(is_blocklisted(Path::new("/private/etc/hosts")));
        assert!(!is_blocklisted(Path::new("/tmp/sp3d_test.png")));
    }

    #[test]
    fn validate_import_accepts_json_in_temp() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "sp3d_path_sec_test_{}.json",
            std::process::id()
        ));
        std::fs::File::create(&path)
            .unwrap()
            .write_all(b"{}")
            .unwrap();
        let canonical = validate_import_file_path(path.to_str().unwrap(), "json").unwrap();
        assert!(canonical.is_file());
        let _ = std::fs::remove_file(path);
    }
}
