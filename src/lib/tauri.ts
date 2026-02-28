// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

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

/** Arguments for export_stl command (ADR-009: optional target dimensions). */
export interface ExportStlArgs {
  path: string;
  target_width_mm?: number | null;
  target_height_mm?: number | null;
}

/** Arguments for export_obj command (BACK-801, ADR-009). */
export interface ExportObjArgs {
  path: string;
  target_width_mm?: number | null;
  target_height_mm?: number | null;
}

/** App settings persisted between sessions (BACK-804, BACK-805, ADR-009). */
export interface AppSettings {
  lastExportDir?: string | null;
  exportFormat?: string | null;
  depthBrightness?: number | null;
  depthContrast?: number | null;
  depthGamma?: number | null;
  depthInvert?: boolean | null;
  depthMinMm?: number | null;
  depthMaxMm?: number | null;
  /** Target output size in mm for mesh/export (ADR-009). When both set, mesh XY fits inside this rectangle. */
  targetWidthMm?: number | null;
  /** Target output size in mm for mesh/export (ADR-009). */
  targetHeightMm?: number | null;
  windowWidth?: number | null;
  windowHeight?: number | null;
}

/** Load and validate image at path; returns dimensions, file size, and base64 preview (BACK-101, BACK-105). */
export async function loadImage(path: string): Promise<LoadImageResult> {
  return invoke<LoadImageResult>("load_image", { path });
}

/** Options for export (ADR-009: optional target size in mm). */
export interface ExportOptions {
  targetWidthMm?: number | null;
  targetHeightMm?: number | null;
}

export async function exportStl(path: string, options?: ExportOptions): Promise<void> {
  const args: Record<string, unknown> = { path };
  if (options?.targetWidthMm != null && options.targetWidthMm > 0) args.target_width_mm = options.targetWidthMm;
  if (options?.targetHeightMm != null && options.targetHeightMm > 0) args.target_height_mm = options.targetHeightMm;
  return invoke("export_stl", args);
}

export async function exportObj(path: string, options?: ExportOptions): Promise<void> {
  const args: Record<string, unknown> = { path };
  if (options?.targetWidthMm != null && options.targetWidthMm > 0) args.target_width_mm = options.targetWidthMm;
  if (options?.targetHeightMm != null && options.targetHeightMm > 0) args.target_height_mm = options.targetHeightMm;
  return invoke("export_obj", args);
}

/** Get current app settings (depth range, target dimensions, window size, etc.). */
export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

/** Persist app settings to disk (BACK-804, BACK-805). */
export async function saveSettings(newSettings: AppSettings): Promise<void> {
  return invoke("save_settings", { newSettings });
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

/** Single curve control point (BACK-1102). x/y in [0, 1]. */
export interface CurvePoint {
  x: number;
  y: number;
}

/** Depth adjustment params (BACK-401–405, BACK-1102). Matches Rust DepthAdjustmentParams (camelCase). */
export interface DepthAdjustmentParams {
  brightness: number;
  contrast: number;
  gamma: number;
  invert: boolean;
  depthMinMm: number;
  depthMaxMm: number;
  /** Optional curve control points (BACK-1102). When null/undefined or length < 2, no curve. */
  curveControlPoints?: CurvePoint[] | null;
}

export async function generateDepthMap(path: string): Promise<DepthMapResult> {
  return invoke<DepthMapResult>("generate_depth_map", { path });
}

export async function getDepthMap(): Promise<DepthMapData | null> {
  return invoke<DepthMapData | null>("get_depth_map");
}

/** Histogram of current (adjusted) depth map, 256 bins over [0, 1] (BACK-1101). Returns null if no depth. */
export async function getDepthHistogram(): Promise<number[] | null> {
  return invoke<number[] | null>("get_depth_histogram");
}

export async function getDepthAdjustmentParams(): Promise<DepthAdjustmentParams> {
  return invoke<DepthAdjustmentParams>("get_depth_adjustment_params");
}

export async function setDepthAdjustmentParams(
  params: DepthAdjustmentParams
): Promise<void> {
  return invoke("set_depth_adjustment_params", { params });
}

export async function resetDepthAdjustments(): Promise<void> {
  return invoke("reset_depth_adjustments");
}

/** Mesh data for 3D preview (BACK-601, BACK-602, 3D_PREVIEW_API.md). Positions/normals in mm. */
export interface MeshData {
  positions: [number, number, number][];
  normals: [number, number, number][];
}

/** Optional preview_step for reduced-detail preview (BACK-603). Optional target dimensions in mm (ADR-009, scaling). */
export interface GetMeshDataOptions {
  previewStep?: number;
  targetWidthMm?: number | null;
  targetHeightMm?: number | null;
}

// --- Sprint 1.10: Model management ---

/** Model installation status (BACK-902). */
export interface ModelStatus {
  installed: boolean;
  modelDir: string;
  modelId: string;
  missingFiles: string[];
  sizeMb?: number;
}

/** Model info for display. */
export interface ModelInfo {
  modelId: string;
  modelDir: string;
  license: string;
  estimatedSizeMb: number;
  description: string;
}

/** Download result. */
export interface DownloadResult {
  status: string;
  modelDir?: string;
  sizeMb?: number;
  error?: string;
}

export async function checkModel(): Promise<ModelStatus> {
  return invoke<ModelStatus>("check_model");
}

export async function getModelInfo(): Promise<ModelInfo> {
  return invoke<ModelInfo>("get_model_info");
}

export async function downloadModel(): Promise<DownloadResult> {
  return invoke<DownloadResult>("download_model");
}

/**
 * Get mesh data (positions, normals) for 3D preview from current adjusted depth map (BACK-601, BACK-603).
 * Use previewStep to request reduced vertex count for faster preview; export always uses full resolution.
 */
export async function getMeshData(
  options?: GetMeshDataOptions
): Promise<MeshData | null> {
  const args: Record<string, unknown> = {};
  if (options?.previewStep != null) {
    args.preview_step = Math.max(1, options.previewStep);
  }
  if (options?.targetWidthMm != null && options.targetWidthMm > 0) {
    args.target_width_mm = options.targetWidthMm;
  }
  if (options?.targetHeightMm != null && options.targetHeightMm > 0) {
    args.target_height_mm = options.targetHeightMm;
  }
  return invoke<MeshData | null>("get_mesh_data", args);
}
