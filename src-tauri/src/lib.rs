mod file_io;
mod image_loading;

// Error handling pattern (BACK-004): use anyhow inside commands for context chain;
// Tauri IPC requires serializable errors, so we use Result<T, String> and map
// anyhow::Error via .map_err(|e| e.to_string()) at the boundary.

#[tauri::command]
fn load_image(path: String) -> Result<image_loading::LoadImageOut, String> {
    image_loading::load_image_impl(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn export_stl(path: String) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err(anyhow::anyhow!("export path must be non-empty").to_string());
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = env_logger::try_init();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![load_image, export_stl])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn load_image_rejects_empty_path() {
        let err = load_image("".to_string()).unwrap_err();
        assert!(err.contains("non-empty") || err.contains("path"));
        let err = load_image("   ".to_string()).unwrap_err();
        assert!(!err.is_empty());
    }

    /// QA-103: Load valid PNG, verify response contains correct dimensions (BACK-101, BACK-105).
    /// Creates a temp PNG at runtime so the test is self-contained and CI does not depend on fixture content.
    #[test]
    fn load_valid_png_returns_dimensions() {
        let temp = std::env::temp_dir().join("sp3d_qa103_test.png");
        let (w, h) = (100u32, 50u32);
        let img = image::ImageBuffer::from_fn(w, h, |_x, _y| image::Rgb([255u8, 0u8, 0u8]));
        img.save(&temp).expect("write test PNG");
        let path = temp.to_string_lossy().to_string();
        let result = load_image(path);
        let _ = std::fs::remove_file(&temp);
        let r = result.expect("load_image should succeed for valid PNG");
        assert!(r.ok);
        assert_eq!(r.width, w);
        assert_eq!(r.height, h);
        assert!(!r.preview_base64.is_empty());
    }

    /// QA-104: Load invalid/corrupt file, verify error (BACK-102, SEC-102).
    #[test]
    fn load_invalid_file_returns_error() {
        let temp = std::env::temp_dir().join("sp3d_qa104_invalid.dat");
        File::create(&temp)
            .unwrap()
            .write_all(b"not an image; wrong magic bytes")
            .unwrap();
        let path = temp.to_string_lossy().to_string();
        let result = load_image(path);
        let _ = std::fs::remove_file(&temp);
        let err = result.expect_err("load_image should fail for non-image file");
        let lower = err.to_lowercase();
        assert!(
            lower.contains("invalid") || lower.contains("corrupt") || lower.contains("format"),
            "error message should indicate invalid/corrupt/format: {}",
            err
        );
    }
}
