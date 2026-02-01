mod file_io;

// Error handling pattern (BACK-004): use anyhow inside commands for context chain;
// Tauri IPC requires serializable errors, so we use Result<T, String> and map
// anyhow::Error via .map_err(|e| e.to_string()) at the boundary.
// Prefer: anyhow::Result<T> internally, .context("step") for each fallible op.

fn load_image_impl(path: String) -> anyhow::Result<serde_json::Value> {
    anyhow::ensure!(!path.trim().is_empty(), "image path must be non-empty");
    Ok(serde_json::json!({ "ok": true, "message": "load_image stub", "path": path }))
}

#[tauri::command]
fn load_image(path: String) -> Result<serde_json::Value, String> {
    load_image_impl(path).map_err(|e| e.to_string())
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
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![load_image, export_stl])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_image_stub_accepts_non_empty_path() {
        let r = load_image_impl("/some/path.png".to_string()).unwrap();
        assert!(r.get("ok").and_then(|v| v.as_bool()).unwrap_or(false));
        assert_eq!(r.get("path").and_then(|v| v.as_str()), Some("/some/path.png"));
    }

    #[test]
    fn load_image_stub_rejects_empty_path() {
        assert!(load_image_impl("".to_string()).is_err());
        assert!(load_image_impl("   ".to_string()).is_err());
    }
}
