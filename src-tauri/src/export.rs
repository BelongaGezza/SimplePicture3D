// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Point cloud export formats (ADR-012 surface-map pipeline).
//!
//! Provides PLY, XYZ, and CSV exporters for volumetric point clouds.
//! These are the primary export formats for laser engraver compatibility.

use anyhow::{Context, Result};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::blank_envelope::BlankEnvelope;
use crate::path_security;

/// Resolves an export target to a canonical path (SEC-D-09, extends TD-06).
///
/// Existing files are canonicalized in full. New files canonicalize the parent directory
/// and join the filename so symlink components in the parent are resolved.
pub fn canonicalize_export_target(path: &Path) -> Result<PathBuf> {
    if path.exists() {
        return path
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize export path: {:?}", path));
    }
    let parent = path
        .parent()
        .context("Export path has no parent directory")?;
    let file_name = path
        .file_name()
        .context("Export path has no filename")?;
    let canonical_parent = parent
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize export directory: {:?}", parent))?;
    Ok(canonical_parent.join(file_name))
}

/// Writes export output atomically: temp file in the target directory, then rename (SEC-D-09).
///
/// On Unix, `rename` replacing an existing file is atomic. On Windows, `std::fs::rename`
/// overwrites the destination when present; readers on local NTFS typically see the full
/// new file or the previous file, not a partial write. Network shares may differ.
fn write_export_atomically<F>(path: &Path, write_body: F) -> Result<()>
where
    F: FnOnce(&mut std::io::BufWriter<std::fs::File>) -> Result<()>,
{
    let target = canonicalize_export_target(path)?;
    if path_security::is_blocklisted(&target) {
        anyhow::bail!("Cannot export to system directories");
    }
    let parent = target
        .parent()
        .context("Export path has no parent directory")?;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let temp_path = parent.join(format!(".sp3d_export_{}_{nanos}.tmp", std::process::id()));

    {
        let file = std::fs::File::create(&temp_path).with_context(|| {
            format!("Failed to create temporary export file: {:?}", temp_path)
        })?;
        let mut writer = std::io::BufWriter::new(file);
        write_body(&mut writer)?;
        writer
            .flush()
            .context("Failed to flush temporary export file")?;
    }

    std::fs::rename(&temp_path, &target).with_context(|| {
        format!(
            "Failed to finalize export (rename {:?} -> {:?})",
            temp_path, target
        )
    })?;
    Ok(())
}

/// Export format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// PLY format (ASCII or binary).
    Ply,
    /// XYZ format (ASCII, one point per line).
    Xyz,
    /// CSV format (with header row).
    Csv,
}

impl ExportFormat {
    /// Get the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Ply => "ply",
            ExportFormat::Xyz => "xyz",
            ExportFormat::Csv => "csv",
        }
    }
}

impl FromStr for ExportFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ply" => Ok(ExportFormat::Ply),
            "xyz" => Ok(ExportFormat::Xyz),
            "csv" => Ok(ExportFormat::Csv),
            _ => Err(()),
        }
    }
}

/// Metadata to include in export file headers.
#[derive(Debug, Clone, Default)]
pub struct ExportMetadata {
    /// Source image filename (if available).
    pub source_image: Option<String>,
    /// Blank envelope dimensions.
    pub blank_envelope: Option<BlankEnvelope>,
    /// Generator name and version.
    pub generator: String,
}

impl ExportMetadata {
    pub fn new() -> Self {
        Self {
            source_image: None,
            blank_envelope: None,
            generator: "SimplePicture3D".to_string(),
        }
    }
}

/// Sanitize user-provided text for PLY/CSV comment headers (SEC-D-12).
///
/// Uses basename only (no directory paths), strips ASCII control characters and
/// newlines so comment injection cannot break export parsers.
pub fn sanitize_export_comment_field(value: &str) -> String {
    let basename = Path::new(value)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(value);
    basename
        .chars()
        .filter(|c| !c.is_control())
        .collect::<String>()
        .trim()
        .to_string()
}

/// Write points to PLY format (ASCII).
///
/// PLY (Polygon File Format) is widely supported by 3D software and some
/// laser engraving tools.
///
/// # Format
/// ```text
/// ply
/// format ascii 1.0
/// comment Generated by SimplePicture3D
/// element vertex N
/// property float x
/// property float y
/// property float z
/// end_header
/// x1 y1 z1
/// x2 y2 z2
/// ...
/// ```
pub fn write_ply_ascii<W: Write>(
    writer: &mut W,
    points: &[[f32; 3]],
    metadata: &ExportMetadata,
) -> Result<()> {
    // Header
    writeln!(writer, "ply")?;
    writeln!(writer, "format ascii 1.0")?;
    writeln!(writer, "comment Generated by {}", metadata.generator)?;

    if let Some(ref source) = metadata.source_image {
        let safe = sanitize_export_comment_field(source);
        if !safe.is_empty() {
            writeln!(writer, "comment Source: {}", safe)?;
        }
    }

    if let Some(ref envelope) = metadata.blank_envelope {
        writeln!(
            writer,
            "comment Blank: {}x{}x{} mm, margin {} mm",
            envelope.length_mm, envelope.width_mm, envelope.height_mm, envelope.margin_mm
        )?;
    }

    writeln!(writer, "element vertex {}", points.len())?;
    writeln!(writer, "property float x")?;
    writeln!(writer, "property float y")?;
    writeln!(writer, "property float z")?;
    writeln!(writer, "end_header")?;

    // Points
    for p in points {
        writeln!(writer, "{:.6} {:.6} {:.6}", p[0], p[1], p[2])?;
    }

    Ok(())
}

/// Write points to PLY format (binary little-endian).
///
/// Binary PLY is more compact and faster to parse than ASCII.
pub fn write_ply_binary<W: Write>(
    writer: &mut W,
    points: &[[f32; 3]],
    metadata: &ExportMetadata,
) -> Result<()> {
    // Header (ASCII)
    writeln!(writer, "ply")?;
    writeln!(writer, "format binary_little_endian 1.0")?;
    writeln!(writer, "comment Generated by {}", metadata.generator)?;

    if let Some(ref source) = metadata.source_image {
        let safe = sanitize_export_comment_field(source);
        if !safe.is_empty() {
            writeln!(writer, "comment Source: {}", safe)?;
        }
    }

    if let Some(ref envelope) = metadata.blank_envelope {
        writeln!(
            writer,
            "comment Blank: {}x{}x{} mm, margin {} mm",
            envelope.length_mm, envelope.width_mm, envelope.height_mm, envelope.margin_mm
        )?;
    }

    writeln!(writer, "element vertex {}", points.len())?;
    writeln!(writer, "property float x")?;
    writeln!(writer, "property float y")?;
    writeln!(writer, "property float z")?;
    writeln!(writer, "end_header")?;

    // Points (binary)
    for p in points {
        writer.write_all(&p[0].to_le_bytes())?;
        writer.write_all(&p[1].to_le_bytes())?;
        writer.write_all(&p[2].to_le_bytes())?;
    }

    Ok(())
}

/// Export points to PLY file.
pub fn export_ply(
    path: &Path,
    points: &[[f32; 3]],
    metadata: &ExportMetadata,
    binary: bool,
) -> Result<()> {
    write_export_atomically(path, |writer| {
        if binary {
            write_ply_binary(writer, points, metadata)
        } else {
            write_ply_ascii(writer, points, metadata)
        }
    })
}

// ============================================================================
// XYZ Export
// ============================================================================

/// Write points to XYZ format (ASCII).
///
/// XYZ is a simple format with one point per line: `x y z`
/// Widely supported by point cloud processing tools and some laser engravers.
pub fn write_xyz<W: Write>(writer: &mut W, points: &[[f32; 3]]) -> Result<()> {
    for p in points {
        writeln!(writer, "{:.6} {:.6} {:.6}", p[0], p[1], p[2])?;
    }
    Ok(())
}

/// Export points to XYZ file.
pub fn export_xyz(path: &Path, points: &[[f32; 3]]) -> Result<()> {
    write_export_atomically(path, |writer| write_xyz(writer, points))
}

// ============================================================================
// CSV Export
// ============================================================================

/// Write points to CSV format with header.
///
/// CSV format with header row: `x,y,z`
/// Compatible with spreadsheet software and many import tools.
pub fn write_csv<W: Write>(
    writer: &mut W,
    points: &[[f32; 3]],
    metadata: &ExportMetadata,
) -> Result<()> {
    // Optional comment header (some CSV readers ignore lines starting with #)
    writeln!(writer, "# Generated by {}", metadata.generator)?;

    if let Some(ref source) = metadata.source_image {
        let safe = sanitize_export_comment_field(source);
        if !safe.is_empty() {
            writeln!(writer, "# Source: {}", safe)?;
        }
    }

    if let Some(ref envelope) = metadata.blank_envelope {
        writeln!(
            writer,
            "# Blank: {}x{}x{} mm, margin {} mm",
            envelope.length_mm, envelope.width_mm, envelope.height_mm, envelope.margin_mm
        )?;
    }

    // Header row
    writeln!(writer, "x,y,z")?;

    // Points
    for p in points {
        writeln!(writer, "{:.6},{:.6},{:.6}", p[0], p[1], p[2])?;
    }

    Ok(())
}

/// Export points to CSV file.
pub fn export_csv(path: &Path, points: &[[f32; 3]], metadata: &ExportMetadata) -> Result<()> {
    write_export_atomically(path, |writer| write_csv(writer, points, metadata))
}

// ============================================================================
// Generic Export
// ============================================================================

/// Export points to file, auto-detecting format from extension.
pub fn export_points(
    path: &Path,
    points: &[[f32; 3]],
    metadata: &ExportMetadata,
    format: ExportFormat,
) -> Result<()> {
    match format {
        ExportFormat::Ply => export_ply(path, points, metadata, false),
        ExportFormat::Xyz => export_xyz(path, points),
        ExportFormat::Csv => export_csv(path, points, metadata),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample_points() -> Vec<[f32; 3]> {
        vec![[0.0, 0.0, 0.0], [10.0, 5.0, 3.0], [5.5, 2.5, 1.5]]
    }

    fn sample_metadata() -> ExportMetadata {
        ExportMetadata {
            source_image: Some("test.png".to_string()),
            blank_envelope: Some(BlankEnvelope::default()),
            generator: "SimplePicture3D-Test".to_string(),
        }
    }

    #[test]
    fn write_ply_ascii_format() {
        let points = sample_points();
        let metadata = sample_metadata();
        let mut output = Cursor::new(Vec::new());

        write_ply_ascii(&mut output, &points, &metadata).unwrap();

        let result = String::from_utf8(output.into_inner()).unwrap();
        assert!(result.starts_with("ply\n"));
        assert!(result.contains("format ascii 1.0"));
        assert!(result.contains("element vertex 3"));
        assert!(result.contains("property float x"));
        assert!(result.contains("end_header"));
        assert!(result.contains("0.000000 0.000000 0.000000"));
        assert!(result.contains("10.000000 5.000000 3.000000"));
    }

    #[test]
    fn write_ply_binary_format() {
        let points = sample_points();
        let metadata = sample_metadata();
        let mut output = Cursor::new(Vec::new());

        write_ply_binary(&mut output, &points, &metadata).unwrap();

        let result = output.into_inner();
        // Header should be ASCII
        let header_end = result
            .windows(11)
            .position(|w| w == b"end_header\n")
            .unwrap();
        let header = String::from_utf8_lossy(&result[..header_end + 11]);
        assert!(header.contains("format binary_little_endian 1.0"));
        assert!(header.contains("element vertex 3"));

        // Binary data follows header: 3 points * 3 floats * 4 bytes = 36 bytes
        let expected_binary_size = 3 * 3 * 4;
        let actual_binary_size = result.len() - (header_end + 11);
        assert_eq!(actual_binary_size, expected_binary_size);
    }

    #[test]
    fn write_xyz_format() {
        let points = sample_points();
        let mut output = Cursor::new(Vec::new());

        write_xyz(&mut output, &points).unwrap();

        let result = String::from_utf8(output.into_inner()).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains("0.000000 0.000000 0.000000"));
        assert!(lines[1].contains("10.000000 5.000000 3.000000"));
    }

    #[test]
    fn write_csv_format() {
        let points = sample_points();
        let metadata = sample_metadata();
        let mut output = Cursor::new(Vec::new());

        write_csv(&mut output, &points, &metadata).unwrap();

        let result = String::from_utf8(output.into_inner()).unwrap();
        assert!(result.contains("# Generated by"));
        assert!(result.contains("# Source: test.png"));
        assert!(result.contains("x,y,z"));
        assert!(result.contains("0.000000,0.000000,0.000000"));
        assert!(result.contains("10.000000,5.000000,3.000000"));
    }

    #[test]
    fn sanitize_export_comment_strips_newlines_and_uses_basename() {
        let dirty = "/home/user/evil\ncomment\ninjection.png";
        assert_eq!(
            sanitize_export_comment_field(dirty),
            "evilcommentinjection.png"
        );
    }

    #[test]
    fn sanitize_export_comment_strips_control_chars() {
        assert_eq!(
            sanitize_export_comment_field("test\u{0000}\u{0007}file.jpg"),
            "testfile.jpg"
        );
    }

    #[test]
    fn write_csv_sanitizes_source_comment() {
        let points = sample_points();
        let metadata = ExportMetadata {
            source_image: Some("/tmp/evil\nline.png".to_string()),
            blank_envelope: Some(BlankEnvelope::default()),
            generator: "SimplePicture3D-Test".to_string(),
        };
        let mut output = Cursor::new(Vec::new());

        write_csv(&mut output, &points, &metadata).unwrap();

        let result = String::from_utf8(output.into_inner()).unwrap();
        assert!(result.contains("# Source: evilline.png"));
        let source_lines: Vec<_> = result
            .lines()
            .filter(|l| l.starts_with("# Source:"))
            .collect();
        assert_eq!(source_lines.len(), 1);
    }

    #[test]
    fn write_ply_ascii_sanitizes_source_comment() {
        let points = sample_points();
        let metadata = ExportMetadata {
            source_image: Some("/tmp/my\nimage.png".to_string()),
            blank_envelope: None,
            generator: "SimplePicture3D-Test".to_string(),
        };
        let mut output = Cursor::new(Vec::new());

        write_ply_ascii(&mut output, &points, &metadata).unwrap();

        let result = String::from_utf8(output.into_inner()).unwrap();
        assert!(result.contains("comment Source: myimage.png"));
        let source_lines: Vec<_> = result
            .lines()
            .filter(|l| l.starts_with("comment Source:"))
            .collect();
        assert_eq!(source_lines.len(), 1);
    }

    #[test]
    fn export_format_extension() {
        assert_eq!(ExportFormat::Ply.extension(), "ply");
        assert_eq!(ExportFormat::Xyz.extension(), "xyz");
        assert_eq!(ExportFormat::Csv.extension(), "csv");
    }

    #[test]
    fn export_format_from_str() {
        assert_eq!("ply".parse(), Ok(ExportFormat::Ply));
        assert_eq!("PLY".parse(), Ok(ExportFormat::Ply));
        assert_eq!("xyz".parse(), Ok(ExportFormat::Xyz));
        assert_eq!("csv".parse(), Ok(ExportFormat::Csv));
        assert_eq!(ExportFormat::from_str("stl"), Err(()));
    }

    #[test]
    fn export_empty_points() {
        let points: Vec<[f32; 3]> = vec![];
        let metadata = ExportMetadata::new();
        let mut output = Cursor::new(Vec::new());

        // Should succeed even with empty points
        write_ply_ascii(&mut output, &points, &metadata).unwrap();
        let result = String::from_utf8(output.into_inner()).unwrap();
        assert!(result.contains("element vertex 0"));
    }

    /// SEC-D-09: export uses temp file + rename; full path is canonicalized.
    #[test]
    fn export_atomic_write_and_canonicalize_target() {
        let points = sample_points();
        let metadata = sample_metadata();
        let stamp = std::process::id();
        let dir = std::env::temp_dir().join(format!("sp3d_sec_d09_export_{stamp}"));
        std::fs::create_dir_all(&dir).expect("temp export dir");

        let target = dir.join("output.ply");
        export_ply(&target, &points, &metadata, false).expect("atomic PLY export");
        assert!(target.is_file(), "export must create final file");
        let canonical = canonicalize_export_target(&target).expect("canonicalize");
        assert_eq!(canonical, target.canonicalize().expect("path canonical"));

        let ply = std::fs::read_to_string(&target).expect("read PLY");
        assert!(ply.contains("element vertex 3"));

        let temp_leftover: Vec<_> = std::fs::read_dir(&dir)
            .expect("read dir")
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .is_some_and(|n| n.starts_with(".sp3d_export_"))
            })
            .collect();
        assert!(
            temp_leftover.is_empty(),
            "temp export files must be renamed away"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// QA-D-01 fixture: write 3-point PLY/XYZ to disk and assert on-disk header format.
    #[test]
    fn export_ply_xyz_writes_disk_fixture_format() {
        let points = sample_points();
        let metadata = sample_metadata();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("sp3d_qa_d01_export_{stamp}"));
        std::fs::create_dir_all(&dir).expect("temp export dir");

        let ply_path = dir.join("fixture.ply");
        let xyz_path = dir.join("fixture.xyz");
        export_ply(&ply_path, &points, &metadata, false).unwrap();
        export_xyz(&xyz_path, &points).unwrap();

        let ply = std::fs::read_to_string(&ply_path).expect("read PLY");
        assert!(ply.starts_with("ply\n"));
        assert!(ply.contains("format ascii 1.0\n"));
        assert!(ply.contains("element vertex 3\n"));
        assert!(ply.contains("end_header\n"));
        assert!(ply.contains("comment Generated by SimplePicture3D-Test"));
        assert!(ply.contains("comment Blank: 50x50x80 mm, margin 2 mm"));

        let xyz = std::fs::read_to_string(&xyz_path).expect("read XYZ");
        let lines: Vec<&str> = xyz.lines().collect();
        assert_eq!(lines.len(), 3, "XYZ: one line per point");
        assert!(lines[0].contains("0.000000 0.000000 0.000000"));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
