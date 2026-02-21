// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Python subprocess bridge for depth estimation (BACK-201–205).
//!
//! Spawns `python -m python.depth_estimator --input <path>` with image bytes in a temp file;
//! captures stdout (JSON depth map), stderr (progress/errors); enforces timeout; no user input in argv.
//!
//! Progress protocol (BACK-205, AI-203): Python writes to stderr only. Lines: `PROGRESS <0-100>`, `STAGE <name>`.

#![allow(dead_code)] // Used by integration test (roundtrip) and future generate_depth_map command (Sprint 1.4)

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Read};
use std::process::Stdio;
use std::time::Duration;

use crate::file_io;

/// Default timeout for depth estimation (e.g. 5 minutes for 4K per BACK-204).
const DEFAULT_TIMEOUT_SECS: u64 = 300;

/// JSON depth map output from Python (ARCH-101, ARCH-102). Serialized for Tauri IPC (BACK-303).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthMapOutput {
    pub height: u32,
    pub width: u32,
    pub depth: Vec<f32>,
}

/// Result of running the depth estimator: depth map and stderr lines (progress/errors).
#[derive(Debug, Clone)]
pub struct RunDepthResult {
    pub depth: DepthMapOutput,
    pub stderr_lines: Vec<String>,
}

/// Parses progress protocol from stderr lines and logs at info (BACK-205). Lines: "PROGRESS 25", "STAGE inference".
pub fn log_progress_from_stderr(lines: &[String]) {
    for line in lines {
        let line = line.trim();
        if let Some(n) = line.strip_prefix("PROGRESS ") {
            if let Ok(pct) = n.trim().parse::<u8>() {
                log::info!("depth estimation progress: {}%", pct);
            }
        } else if let Some(stage) = line.strip_prefix("STAGE ") {
            log::info!("depth estimation stage: {}", stage.trim());
        }
    }
}

/// Extracts stage names from stderr lines (BACK-304). Lines "STAGE <name>" become entries in the returned vec.
pub fn stages_from_stderr(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("STAGE ")
                .map(|s| s.trim().to_string())
        })
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stages_from_stderr_extracts_stage_names() {
        let lines = vec![
            "PROGRESS 0".to_string(),
            "STAGE loading_model".to_string(),
            "PROGRESS 50".to_string(),
            "STAGE inference".to_string(),
            "  STAGE  post  ".to_string(),
        ];
        let stages = stages_from_stderr(&lines);
        assert_eq!(stages, ["loading_model", "inference", "post"]);
    }

    #[test]
    fn stages_from_stderr_ignores_empty_stage() {
        let lines = vec!["STAGE ".to_string(), "STAGE  ".to_string()];
        let stages = stages_from_stderr(&lines);
        assert!(stages.is_empty());
    }
}

/// Ensures path is canonical and under system temp dir (SEC-201).
fn validate_input_path(path: &std::path::Path) -> Result<std::path::PathBuf> {
    let canonical = path
        .canonicalize()
        .context("temp file path could not be canonicalized")?;
    let temp_dir = std::env::temp_dir()
        .canonicalize()
        .context("temp dir could not be canonicalized")?;
    if !canonical.starts_with(&temp_dir) {
        anyhow::bail!("input path is outside system temp directory");
    }
    Ok(canonical)
}

/// Finds Python: VIRTUAL_ENV/Scripts/python.exe (Windows) or bin/python (Unix), else "python3" (Unix) or "python" (Windows).
fn python_executable() -> std::path::PathBuf {
    if let Ok(venv) = std::env::var("VIRTUAL_ENV") {
        let path = std::path::Path::new(&venv);
        #[cfg(windows)]
        let exe = path.join("Scripts").join("python.exe");
        #[cfg(not(windows))]
        let exe = path.join("bin").join("python");
        if exe.is_file() {
            return exe;
        }
    }
    #[cfg(windows)]
    return std::path::PathBuf::from("python");
    #[cfg(not(windows))]
    std::path::PathBuf::from("python3")
}

/// Deletes temp file when dropped.
struct TempFileGuard(std::path::PathBuf);
impl TempFileGuard {
    fn new(p: std::path::PathBuf) -> Self {
        Self(p)
    }
}
impl Drop for TempFileGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Runs the depth estimator subprocess (BACK-201, BACK-202, BACK-204, BACK-205).
pub fn run_depth_estimation(image_bytes: &[u8]) -> Result<RunDepthResult> {
    run_depth_estimation_with_timeout(image_bytes, Duration::from_secs(DEFAULT_TIMEOUT_SECS))
}

/// Same as run_depth_estimation but with explicit timeout.
pub fn run_depth_estimation_with_timeout(
    image_bytes: &[u8],
    timeout: Duration,
) -> Result<RunDepthResult> {
    let temp_path = file_io::write_temp_file("img_", ".png", image_bytes)
        .context("write temp image for Python")?;
    let input_path = validate_input_path(&temp_path)?;
    let _guard = TempFileGuard::new(temp_path);

    let python = python_executable();
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    // Run from directory containing the "python" package (repo_root/python/). Try cwd/python then parent/python (e.g. when running from src-tauri).
    let work_dir = cwd.join("python");
    let work_dir = if work_dir.is_dir() {
        work_dir
    } else {
        cwd.parent()
            .map(|p| p.join("python"))
            .filter(|p| p.is_dir())
            .unwrap_or(cwd)
    };
    let input_arg = input_path.to_string_lossy().to_string();

    let mut cmd = std::process::Command::new(&python);
    cmd.arg("-m")
        .arg("python.depth_estimator")
        .arg("--input")
        .arg(&input_arg)
        .current_dir(&work_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd
        .spawn()
        .context("failed to spawn Python (is Python 3.10+ on PATH or VIRTUAL_ENV set?)")?;

    let mut stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");

    // Read stderr in a thread (so we don't deadlock if Python buffers).
    let stderr_handle = std::thread::spawn(move || {
        let lines: Vec<String> = std::io::BufReader::new(stderr)
            .lines()
            .map_while(Result::ok)
            .collect();
        lines
    });

    // Read stdout in main thread (blocking until EOF or process exits).
    let mut stdout_bytes = Vec::new();
    let _ = stdout.read_to_end(&mut stdout_bytes);

    // Wait for process with timeout (BACK-204).
    let poll_interval = Duration::from_millis(100);
    let deadline = std::time::Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stderr_lines = stderr_handle.join().unwrap_or_default();
                if !status.success() {
                    let msg = stderr_lines
                        .last()
                        .map(String::as_str)
                        .unwrap_or("unknown error");
                    anyhow::bail!(
                        "depth estimation failed (exit code {}): {}",
                        status.code().unwrap_or(-1),
                        msg
                    );
                }
                let depth: DepthMapOutput = serde_json::from_slice(&stdout_bytes)
                    .context("invalid depth map JSON from Python")?;
                let expected_len = (depth.height as usize)
                    .checked_mul(depth.width as usize)
                    .context("depth dimensions overflow")?;
                if depth.depth.len() != expected_len {
                    anyhow::bail!(
                        "depth map size mismatch: expected {} ({}×{}), got {}",
                        expected_len,
                        depth.height,
                        depth.width,
                        depth.depth.len()
                    );
                }
                log_progress_from_stderr(&stderr_lines);
                return Ok(RunDepthResult {
                    depth,
                    stderr_lines,
                });
            }
            Ok(None) => {}
            Err(e) => {
                let _ = child.kill();
                anyhow::bail!("process wait error: {}", e);
            }
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!(
                "depth estimation timed out after {} seconds",
                timeout.as_secs()
            );
        }
        std::thread::sleep(poll_interval);
    }
}
