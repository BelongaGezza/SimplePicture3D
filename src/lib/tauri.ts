// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Tauri IPC helpers (UI-004). Types match backend commands in `src-tauri/src/lib.rs`.
 *
 * ADR-012: `setBlankEnvelope`, `setVolumetricParams`, `generatePointCloud`,
 * `exportPly` / `exportXyz` / `exportCsv`.
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
  /** Present when ok is false (if backend returns in-band errors). */
  message?: string;
}

/** Crystal blank dimensions (mm), matches Rust `BlankEnvelope` (camelCase). */
export interface BlankEnvelope {
  lengthMm: number;
  widthMm: number;
  heightMm: number;
  marginMm: number;
}

/** Surface-map sampling (ADR-012); matches Rust `VolumetricParams`. */
export interface VolumetricParams {
  stepX: number;
  stepY: number;
  depthThreshold: number;
}

/** Fit statistics from blank scaling (`fit_to_blank`). */
export interface FitResult {
  scale: number;
  translation: [number, number, number];
  pointCount: number;
  outliers: number;
}

/** Result from `generate_point_cloud`. */
export interface VolumetricResult {
  points: [number, number, number][];
  pointCount: number;
  fitResult: FitResult;
  memoryBytes: number;
}

/**
 * App settings persisted between sessions (BACK-804, BACK-805).
 *
 * Legacy `targetWidthMm` / `targetHeightMm` / `exportFormat` remain for old settings files.
 * The crystal pipeline uses `blankEnvelope`, `pointCloudFormat`, and `volumetricParams`.
 */
export interface AppSettings {
  lastExportDir?: string | null;
  exportFormat?: string | null;
  depthBrightness?: number | null;
  depthContrast?: number | null;
  depthGamma?: number | null;
  depthInvert?: boolean | null;
  depthMinMm?: number | null;
  depthMaxMm?: number | null;
  /** Legacy field from the 2.5D pipeline; retained on disk only. */
  targetWidthMm?: number | null;
  /** Legacy field from the 2.5D pipeline; retained on disk only. */
  targetHeightMm?: number | null;
  windowWidth?: number | null;
  windowHeight?: number | null;
  /** Curve control points (CURVE-001). Persisted in settings; restored on load. */
  curveControlPoints?: CurvePoint[] | null;
  /** Crystal blank L×W×H (mm) + margin; persisted when set via `setBlankEnvelope`. */
  blankEnvelope?: BlankEnvelope | null;
  /** Preferred point cloud export format: `"ply"` | `"xyz"` | `"csv"`. */
  pointCloudFormat?: string | null;
  /** Sampling parameters for `generatePointCloud` (ADR-012). */
  volumetricParams?: VolumetricParams | null;
}

/** Load and validate image at path; returns dimensions, file size, and base64 preview (BACK-101, BACK-105). */
export async function loadImage(path: string): Promise<LoadImageResult> {
  return invoke<LoadImageResult>("load_image", { path });
}

/** Get current app settings (depth range, target dimensions, window size, etc.). */
export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

/** Persist app settings to disk (BACK-804, BACK-805). */
export async function saveSettings(newSettings: AppSettings): Promise<void> {
  return invoke("save_settings", { newSettings });
}

/** Payload for "depth-progress" Tauri event (BACK-205-STREAM). Emitted during depth estimation. */
export interface DepthProgressEvent {
  percent: number;
  stage?: string;
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

/**
 * Run AI depth estimation on the image at path. Returns when complete.
 * Real-time progress is emitted via the "depth-progress" Tauri event (listen with @tauri-apps/api/event).
 * Payload: { percent: number, stage?: string } (DepthProgressEvent).
 */
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

/** Undo/redo state (BACK-1404). Returned by set/reset/undo/redo/clear_history for UI button state. */
export interface UndoRedoState {
  canUndo: boolean;
  canRedo: boolean;
  params: DepthAdjustmentParams;
}

export async function getUndoRedoState(): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("get_undo_redo_state");
}

export async function setDepthAdjustmentParams(
  params: DepthAdjustmentParams
): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("set_depth_adjustment_params", { params });
}

export async function resetDepthAdjustments(): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("reset_depth_adjustments");
}

export async function undo(): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("undo");
}

export async function redo(): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("redo");
}

export async function clearHistory(): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("clear_history");
}

// --- Sprint 2.5: Mask (BACK-1201, ARCH-502, UI-1201) ---

/** Current mask from backend (dimensions + row-major booleans). Null if no depth map. */
export interface MaskData {
  width: number;
  height: number;
  data: boolean[];
}

export async function getMask(): Promise<MaskData | null> {
  return invoke<MaskData | null>("get_mask");
}

/** Set a rectangle of the mask (paint/erase). Returns updated undo/redo state. */
export async function setMaskRegion(
  x: number,
  y: number,
  width: number,
  height: number,
  value: boolean
): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("set_mask_region", {
    x,
    y,
    width,
    height,
    value,
  });
}

/** Clear mask to all false. Returns updated undo/redo state. */
export async function clearMask(): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("clear_mask");
}

/** Set the full mask from row-major data (JR1-1203 load, JR1-1202 lasso fill). Dimensions must match current depth map. */
export async function setMask(
  width: number,
  height: number,
  data: boolean[]
): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("set_mask", { width, height, data });
}

/** Save current mask to a JSON file (JR1-1203). Path must be .json and writable. */
export async function saveMaskToPath(path: string): Promise<void> {
  return invoke("save_mask_to_path", { path });
}

/**
 * Load mask from a JSON file (JR1-1203).
 * Dimension mismatch: backend requires saved mask width×height to match the current depth map.
 * If the loaded image has different dimensions, the backend returns an error; the user must
 * load an image with matching dimensions or re-paint the mask.
 */
export async function loadMaskFromPath(path: string): Promise<UndoRedoState> {
  return invoke<UndoRedoState>("load_mask_from_path", { path });
}

// --- ADR-012: Crystal surface point cloud ---

export async function setBlankEnvelope(envelope: BlankEnvelope): Promise<void> {
  return invoke("set_blank_envelope", { envelope });
}

export async function setVolumetricParams(params: VolumetricParams): Promise<void> {
  return invoke("set_volumetric_params", { params });
}

/** Persist preferred export format (`ply`, `xyz`, or `csv`). */
export async function setPointCloudFormat(format: string): Promise<void> {
  return invoke("set_point_cloud_format", { format });
}

/** Estimated point count from current depth + settings; `null` if no depth loaded. */
export async function estimatePointCloudCount(): Promise<number | null> {
  return invoke<number | null>("estimate_point_cloud_count");
}

/** Generate surface-map point cloud from current adjusted depth (cached for export commands). */
export async function generatePointCloud(): Promise<VolumetricResult> {
  return invoke<VolumetricResult>("generate_point_cloud");
}

/** Write cached point cloud as PLY. Run `generatePointCloud` first. */
export async function exportPly(path: string, binary: boolean): Promise<void> {
  return invoke("export_ply", { path, binary });
}

/** Write cached point cloud as XYZ. */
export async function exportXyz(path: string): Promise<void> {
  return invoke("export_xyz", { path });
}

/** Write cached point cloud as CSV. */
export async function exportCsv(path: string): Promise<void> {
  return invoke("export_csv", { path });
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

// --- Sprint 2.3: Presets (F2.3, BACK-1302, UI-1301–1304) ---

/** Built-in preset id (BACK-1303). */
export type BuiltinPresetId = "portrait" | "landscape" | "high_detail" | "low_relief";

/** One preset entry for list: user preset by name or built-in by id. */
export interface PresetListItem {
  /** "user" | "builtin" */
  kind: "user" | "builtin";
  /** Display name (e.g. "My Preset" or "Portrait") */
  name: string;
  /** For builtin: id; for user: name used as key in presets dir */
  id: string;
}

/** List built-in preset ids (BACK-1303). Backend returns string[] e.g. ["Portrait", "Landscape", "High Detail", "Low Relief"]. */
export async function listBuiltinPresets(): Promise<string[]> {
  return invoke<string[]>("list_builtin_presets");
}

/** List presets: built-ins first, then user presets from ~/.simplepicture3d/presets/ (BACK-1302, UI-1301, UI-1303). */
export async function listPresets(): Promise<PresetListItem[]> {
  const [builtinIds, userNames] = await Promise.all([
    invoke<string[]>("list_builtin_presets"),
    invoke<string[]>("list_presets"),
  ]);
  const builtins: PresetListItem[] = builtinIds.map((id) => ({
    kind: "builtin" as const,
    name: id,
    id,
  }));
  const users: PresetListItem[] = userNames.map((name) => ({
    kind: "user" as const,
    name,
    id: name,
  }));
  return [...builtins, ...users];
}

/** Save current state as preset. path optional (user-chosen for export). BACK-1302, BACK-1304. */
export async function savePreset(name: string, path?: string | null): Promise<void> {
  const args: Record<string, unknown> = { name };
  if (path != null && path !== "") args.path = path;
  return invoke("save_preset", args);
}

/** Load preset by name (from presets dir) or by absolute path (import). Applies to app state. BACK-1302. */
export async function loadPreset(nameOrPath: string): Promise<void> {
  return invoke("load_preset", { name_or_path: nameOrPath });
}

/** Delete user preset by name (BACK-1302, UI-1301). */
export async function deletePreset(name: string): Promise<void> {
  return invoke("delete_preset", { name });
}

/** Rename user preset (BACK-1302, UI-1301). */
export async function renamePreset(oldName: string, newName: string): Promise<void> {
  return invoke("rename_preset", { old_name: oldName, new_name: newName });
}
