/**
 * Tauri IPC helpers (UI-004, JR1-003). Stub invoke calls for load_image and export_stl.
 * Types match backend command signatures (src-tauri/src/lib.rs).
 */
import { invoke } from "@tauri-apps/api/core";

/** Arguments for load_image command. */
export interface LoadImageArgs {
  path: string;
}

/** Return type from load_image command (BACK-101, BACK-105). Matches LoadImageOut in Rust. */
export interface LoadImageResult {
  ok: boolean;
  width: number;
  height: number;
  fileSizeBytes: number;
  downsampled: boolean;
  /** Base64-encoded PNG (RGB) for preview; use as data URL: data:image/png;base64,${previewBase64} */
  previewBase64: string;
}

/** Arguments for export_stl command. */
export interface ExportStlArgs {
  path: string;
}

export async function loadImage(path: string): Promise<LoadImageResult> {
  const args: LoadImageArgs = { path };
  return invoke<LoadImageResult>("load_image", args);
}

export async function exportStl(path: string): Promise<void> {
  const args: ExportStlArgs = { path };
  return invoke("export_stl", args);
}
