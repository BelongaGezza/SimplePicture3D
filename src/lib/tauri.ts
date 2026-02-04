/**
 * Tauri IPC helpers (UI-004, JR1-003). Stub invoke calls for load_image, export_stl, generate_depth_map, get_depth_map.
 * Types match backend command signatures (src-tauri/src/lib.rs). See docs/architecture.md § Sprint 1.4 command contract.
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
  return invoke<LoadImageResult>("load_image", { path });
}

export async function exportStl(path: string): Promise<void> {
  return invoke("export_stl", { path });
}

/** Success response from generate_depth_map (BACK-303, BACK-304). Row-major depth array in [0, 1]. */
export interface DepthMapResult {
  width: number;
  height: number;
  depth: number[];
  /** 100 when command returns on success. */
  progress?: number;
  /** Stages from Python stderr (e.g. "loading_model", "inference"). */
  stages?: string[];
}

/** Depth map data only (from get_depth_map or for preview). */
export interface DepthMapData {
  width: number;
  height: number;
  depth: number[];
}

export async function generateDepthMap(path: string): Promise<DepthMapResult> {
  return invoke<DepthMapResult>("generate_depth_map", { path });
}

export async function getDepthMap(): Promise<DepthMapData | null> {
  return invoke<DepthMapData | null>("get_depth_map");
}
