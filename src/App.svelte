<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<script lang="ts">
  import ImageImport from "./components/ImageImport.svelte";
  import Preview3D from "./components/Preview3D.svelte";
  import DepthMapPreview from "./components/DepthMapPreview.svelte";
  import DepthControls from "./components/DepthControls.svelte";
  import ExportPanel from "./components/ExportPanel.svelte";
  import FirstRunWizard from "./components/FirstRunWizard.svelte";
  import Button from "./components/Button.svelte";
  import PresetManager from "./components/PresetManager.svelte";
  import MaskingTools from "./components/MaskingTools.svelte";
  import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
  import { listen } from "@tauri-apps/api/event";
  import {
    generateDepthMap,
    getDepthMap,
    getDepthHistogram,
    getDepthAdjustmentParams,
    setDepthAdjustmentParams,
    resetDepthAdjustments,
    getUndoRedoState,
    undo as tauriUndo,
    redo as tauriRedo,
    getMask,
    setMaskRegion,
    clearMask,
    checkModel,
    getSettings,
    saveSettings,
    listPresets,
    savePreset,
    loadPreset,
  } from "$lib/tauri";
  import type {
    LoadImageResult,
    DepthAdjustmentParams,
    PresetListItem,
    DepthProgressEvent,
    MaskData,
  } from "$lib/tauri";

  let status = "Ready";
  let loadPath = "";
  let loading = false;
  let loadError = "";
  let loadedResult: LoadImageResult | null = null;

  /** Show first-run wizard if model is not installed (UI-901). */
  let showWizard = false;
  import { onMount, onDestroy } from "svelte";
  onMount(async () => {
    try {
      const modelStatus = await checkModel();
      if (!modelStatus.installed) {
        showWizard = true;
      }
    } catch {
      // Model check failed; don't block the app
    }
    // CURVE-001: Sync depth params (including restored curve) from backend so CurvesTool shows persisted curve.
    try {
      const params = await getDepthAdjustmentParams();
      adjustmentParams = params;
    } catch {
      // Non-critical; keep default params
    }
    // UI-1401: Sync undo/redo button state from backend (BACK-1404).
    try {
      const undoState = await getUndoRedoState();
      canUndo = undoState.canUndo;
      canRedo = undoState.canRedo;
    } catch {
      // Non-critical
    }
    refreshPresetList();
    window.addEventListener("keydown", onKeyDown);
  });
  onDestroy(() => {
    window.removeEventListener("keydown", onKeyDown);
  });

  /** Depth map for preview (BACK-303, UI-301/302). Adjusted by backend when params change (UI-404). */
  let depthMap: { width: number; height: number; depth: number[] } | null = null;
  /** True while generate_depth_map is running (UI-304). */
  let depthEstimating = false;
  /** Progress 0–100 from depth-progress events during estimation (UI-304). */
  let progressPercent = 0;
  /** Backend error from generate_depth_map (timeout, Python, etc.). */
  let depthError = "";

  /** Current depth adjustment params; synced with backend (UI-401–405, BACK-1102). */
  let adjustmentParams: DepthAdjustmentParams = {
    brightness: 0,
    contrast: 1,
    gamma: 1,
    invert: false,
    depthMinMm: 2,
    depthMaxMm: 10,
    curveControlPoints: undefined,
  };

  /** BACK-1101: Histogram of current adjusted depth (for HistogramPanel). Fetched with preview. */
  let histogramData: number[] | null = null;
  /** UI-1105: Advanced mode (histogram + curves). */
  let advancedMode = false;

  /** Scaling (Sprint 2.1): default 40×40 mm target; on image load dimension depth-map and 3D preview to this. */
  const TARGET_BASE_MM = 40;
  let effectiveTargetWidthMm = 40;
  let effectiveTargetHeightMm = 40;
  let zoomScale = 1; // 1 = 100%; 0.5 = 50%, 1.5 = 150%, 2 = 200%

  /** UI-404: Debounce interval for preview updates (ms). */
  const DEBOUNCE_MS = 80;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  /** UI-1401/1402: Undo/Redo state from backend (BACK-1404). Buttons disabled when nothing to undo/redo. */
  let canUndo = false;
  let canRedo = false;

  /** Sprint 2.5: mask for regional adjustments (UI-1201–1204). Fetched from backend; mirrored for overlay. */
  let maskData: MaskData | null = null;
  let maskTool: "brush" | "eraser" | "select" = "brush";
  let brushSize = 20;
  let brushHardness = 1;
  let showMaskOverlay = true;

  /** Sprint 2.3: Preset list (built-in + user) for dropdown and Load preset (UI-1301, UI-1303). */
  let presetList: PresetListItem[] = [];
  let presetDropdownOpen = false;
  let presetSaveLoadOpen = false;
  let savingPreset = false;
  let presetMessage = "";
  let presetListLoading = false;

  /** Preview URL: base64 from backend (BACK-101, UI-103). */
  $: previewUrl = loadedResult?.previewBase64
    ? `data:image/png;base64,${loadedResult.previewBase64}`
    : "";

  function formatFileSize(bytes: number | undefined): string {
    if (bytes == null) return "—";
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  }

  /** Extract just the filename from a path (cross-platform). */
  $: sourceFileName = loadPath ? (loadPath.split(/[/\\]/).pop() ?? "") : "";

  function handleLoadStart(path: string) {
    loadPath = path;
    status = "Loading…";
    loading = true;
    loadError = "";
    loadedResult = null;
    depthMap = null;
    depthError = "";
    histogramData = null;
    maskData = null;
    // Backend clears undo history on new image load (ARCH); keep UI in sync.
    canUndo = false;
    canRedo = false;
  }

  /** Fetch current mask from backend for overlay (UI-1203). */
  async function refreshMask() {
    if (!depthMap) return;
    try {
      maskData = await getMask();
    } catch {
      maskData = null;
    }
  }

  async function handleLoadSuccess(result: LoadImageResult) {
    loading = false;
    loadedResult = result;
    loadError = "";
    status = result.downsampled ? "Loaded (downsampled)" : "Loaded";
    // Scaling: on image import set default target 40×40 mm so depth-map and 3D preview are dimensioned (zoom to fit).
    effectiveTargetWidthMm = TARGET_BASE_MM;
    effectiveTargetHeightMm = TARGET_BASE_MM;
    zoomScale = 1;
    try {
      const settings = await getSettings();
      settings.targetWidthMm = TARGET_BASE_MM;
      settings.targetHeightMm = TARGET_BASE_MM;
      await saveSettings(settings);
    } catch {
      // Non-critical
    }
  }

  function handleLoadError(message: string) {
    loading = false;
    loadError = message;
    loadedResult = null;
    status = "Load error";
  }

  /** UI-303/UI-304: Generate depth map; subscribe to depth-progress for real-time percentage. */
  async function handleGenerateDepth() {
    if (!loadPath || depthEstimating) return;
    depthEstimating = true;
    progressPercent = 0;
    depthError = "";
    status = "Estimating depth…";
    const unlisten = await listen<DepthProgressEvent>("depth-progress", (event) => {
      progressPercent = event.payload.percent;
    });
    try {
      const result = await generateDepthMap(loadPath);
      depthMap = { width: result.width, height: result.height, depth: result.depth };
      const params = await getDepthAdjustmentParams();
      adjustmentParams = params;
      const hist = await getDepthHistogram();
      histogramData = hist;
      await refreshMask();
      status = "Depth ready";
    } catch (e) {
      depthError = String(e);
      status = "Depth error";
    } finally {
      unlisten();
      depthEstimating = false;
    }
  }

  /** UI-404: Apply params to backend and refresh preview (debounced). BACK-1101: also fetch histogram. */
  async function applyParamsToBackend() {
    debounceTimer = null;
    if (!depthMap) return;
    try {
      const undoState = await setDepthAdjustmentParams(adjustmentParams);
      canUndo = undoState.canUndo;
      canRedo = undoState.canRedo;
      if (import.meta.env.DEV) console.time("getDepthMap");
      const [result, hist] = await Promise.all([getDepthMap(), getDepthHistogram()]);
      if (import.meta.env.DEV) {
        console.timeEnd("getDepthMap");
        if (result) console.debug("getDepthMap dimensions:", result.width, "x", result.height);
      }
      if (result) depthMap = { width: result.width, height: result.height, depth: result.depth };
      histogramData = hist;
    } catch (e) {
      status = "Adjustment error: " + String(e);
    }
  }

  /** UI-401–402: Params changed from DepthControls; update local state and debounce backend + preview. */
  function handleParamsChange(params: DepthAdjustmentParams) {
    adjustmentParams = params;
    if (debounceTimer != null) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(applyParamsToBackend, DEBOUNCE_MS);
  }

  /** Scaling: change zoom (50%, 100%, 150%, 200%); effective target = 40 * scale. */
  async function setZoom(scale: number) {
    zoomScale = scale;
    effectiveTargetWidthMm = Math.round(TARGET_BASE_MM * scale);
    effectiveTargetHeightMm = Math.round(TARGET_BASE_MM * scale);
    try {
      const settings = await getSettings();
      settings.targetWidthMm = effectiveTargetWidthMm;
      settings.targetHeightMm = effectiveTargetHeightMm;
      await saveSettings(settings);
    } catch {
      // Non-critical
    }
  }

  /** UI-405: Reset adjustments and refresh preview from original depth. */
  async function handleResetDepth() {
    try {
      const undoState = await resetDepthAdjustments();
      canUndo = undoState.canUndo;
      canRedo = undoState.canRedo;
      adjustmentParams = await getDepthAdjustmentParams();
      if (import.meta.env.DEV) console.time("getDepthMap");
      const [result, hist] = await Promise.all([getDepthMap(), getDepthHistogram()]);
      if (import.meta.env.DEV) {
        console.timeEnd("getDepthMap");
        if (result) console.debug("getDepthMap dimensions:", result.width, "x", result.height);
      }
      if (result) depthMap = { width: result.width, height: result.height, depth: result.depth };
      histogramData = hist;
      status = "Depth reset";
    } catch (e) {
      status = "Reset error: " + String(e);
    }
  }

  /** UI-1401: Undo last action; refresh params and preview from backend (BACK-1404). */
  async function handleUndo() {
    try {
      const state = await tauriUndo();
      canUndo = state.canUndo;
      canRedo = state.canRedo;
      adjustmentParams = await getDepthAdjustmentParams();
      if (depthMap) {
        const [result, hist] = await Promise.all([getDepthMap(), getDepthHistogram()]);
        if (result) depthMap = { width: result.width, height: result.height, depth: result.depth };
        histogramData = hist;
        await refreshMask();
      }
      status = "Undo";
    } catch {
      canUndo = false;
      canRedo = false;
    }
  }

  /** UI-1401: Redo last undone action; refresh params and preview from backend (BACK-1404). */
  async function handleRedo() {
    try {
      const state = await tauriRedo();
      canUndo = state.canUndo;
      canRedo = state.canRedo;
      adjustmentParams = await getDepthAdjustmentParams();
      if (depthMap) {
        const [result, hist] = await Promise.all([getDepthMap(), getDepthHistogram()]);
        if (result) depthMap = { width: result.width, height: result.height, depth: result.depth };
        histogramData = hist;
        await refreshMask();
      }
      status = "Redo";
    } catch {
      canUndo = false;
      canRedo = false;
    }
  }

  /** UI-1201: Clear mask via backend; refresh overlay and undo state. */
  async function handleClearMask() {
    if (!depthMap) return;
    try {
      const state = await clearMask();
      canUndo = state.canUndo;
      canRedo = state.canRedo;
      await refreshMask();
      status = "Mask cleared";
    } catch (e) {
      status = "Clear mask failed: " + String(e);
    }
  }

  /** UI-1202: Paint/erase one stamp at depth-map pixel (x, y). Backend applies and pushes undo. */
  async function handleMaskPaint(x: number, y: number, value: boolean) {
    if (!depthMap || (maskTool !== "brush" && maskTool !== "eraser")) return;
    const w = depthMap.width;
    const h = depthMap.height;
    const half = Math.max(0, Math.floor(brushSize / 2));
    const x0 = Math.max(0, Math.min(w, x - half));
    const y0 = Math.max(0, Math.min(h, y - half));
    const rw = Math.min(brushSize, w - x0);
    const rh = Math.min(brushSize, h - y0);
    if (rw <= 0 || rh <= 0) return;
    try {
      const state = await setMaskRegion(x0, y0, rw, rh, value);
      canUndo = state.canUndo;
      canRedo = state.canRedo;
      await refreshMask();
    } catch {
      // Non-fatal; e.g. rapid strokes
    }
  }

  /** Sprint 2.3: Refresh preset list (UI-1301, UI-1303). */
  async function refreshPresetList() {
    presetListLoading = true;
    try {
      presetList = await listPresets();
    } catch {
      presetList = [];
    } finally {
      presetListLoading = false;
    }
  }

  /** Apply preset and refresh UI state (depth params, preview, undo state). */
  async function applyPresetAndRefresh() {
    // Clear any pending debounce so applyParamsToBackend does not overwrite backend state
    // with stale params after load_preset (CONSULTANT_TASK_PRESET_APPLY: preset-apply race fix).
    if (debounceTimer != null) {
      clearTimeout(debounceTimer);
      debounceTimer = null;
    }
    try {
      const params = await getDepthAdjustmentParams();
      // Force new object reference so Svelte/reactivity and child sliders update.
      adjustmentParams = { ...params };
      const undoState = await getUndoRedoState();
      canUndo = undoState.canUndo;
      canRedo = undoState.canRedo;
      if (depthMap) {
        const [result, hist] = await Promise.all([getDepthMap(), getDepthHistogram()]);
        if (result) depthMap = { width: result.width, height: result.height, depth: result.depth };
        histogramData = hist;
      }
      const settings = await getSettings();
      if (settings.targetWidthMm != null && settings.targetHeightMm != null) {
        effectiveTargetWidthMm = settings.targetWidthMm;
        effectiveTargetHeightMm = settings.targetHeightMm;
      }
      status = "Preset applied";
    } catch (e) {
      status = "Preset error: " + String(e);
    }
  }

  /** UI-1302: Save current settings as preset (prompt for name). */
  async function handleSaveAsPreset() {
    const name = window.prompt("Preset name");
    if (name == null || name.trim() === "") return;
    savingPreset = true;
    presetMessage = "";
    try {
      await savePreset(name.trim());
      presetMessage = "Preset saved";
      await refreshPresetList();
    } catch (e) {
      presetMessage = "Save failed: " + String(e);
    } finally {
      savingPreset = false;
    }
  }

  /** UI-1302: Load preset by name or path; then refresh state. */
  async function handleLoadPreset(nameOrPath: string) {
    presetDropdownOpen = false;
    presetSaveLoadOpen = false;
    if (debounceTimer != null) {
      clearTimeout(debounceTimer);
      debounceTimer = null;
    }
    try {
      await loadPreset(nameOrPath);
      await applyPresetAndRefresh();
    } catch (e) {
      status = "Load preset failed: " + String(e);
    }
  }

  /** UI-1304: Export preset to user-chosen JSON file. */
  async function handleExportPreset() {
    try {
      const path = await saveDialog({
        defaultPath: "preset.json",
        filters: [{ name: "JSON", extensions: ["json"] }],
      });
      if (path == null) return;
      await savePreset("Export", path);
      status = "Preset exported";
    } catch (e) {
      status = "Export failed: " + String(e);
    }
  }

  /** UI-1304: Import preset from JSON file. */
  async function handleImportPreset() {
    if (debounceTimer != null) {
      clearTimeout(debounceTimer);
      debounceTimer = null;
    }
    try {
      const path = await openDialog({
        multiple: false,
        filters: [{ name: "JSON", extensions: ["json"] }],
      });
      if (path == null || typeof path !== "string") return;
      await loadPreset(path);
      await applyPresetAndRefresh();
      status = "Preset imported";
    } catch (e) {
      status = "Import failed: " + String(e);
    }
  }

  /** Sprint 2.5: After mask change (save/load/clear/apply selection), refresh overlay and undo state. */
  async function handleMaskChange() {
    try {
      const undoState = await getUndoRedoState();
      canUndo = undoState.canUndo;
      canRedo = undoState.canRedo;
      await refreshMask();
    } catch {
      // Non-critical
    }
  }

  /** UI-1402: Keyboard shortcuts for undo (Ctrl+Z) and redo (Ctrl+Y / Ctrl+Shift+Z). */
  function onKeyDown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === "z") {
      if (e.shiftKey) {
        e.preventDefault();
        if (canRedo) handleRedo();
      } else {
        e.preventDefault();
        if (canUndo) handleUndo();
      }
    } else if ((e.ctrlKey || e.metaKey) && e.key === "y") {
      e.preventDefault();
      if (canRedo) handleRedo();
    }
  }
</script>

<!-- First-run wizard (UI-901) -->
<FirstRunWizard visible={showWizard} on:close={() => (showWizard = false)} />

<!-- UI-305: Side-by-side layout — original (left) and depth map (right) on same screen; min 1024×768 per prd -->
<div class="app-layout min-h-screen flex flex-col bg-slate-50 text-slate-800">
  <main class="flex flex-1 min-h-0" role="region" aria-label="Workspace: original image, 3D preview, and depth map">
    <!-- Left sidebar: original image (UI-101–105, UI-305 side-by-side) -->
    <aside class="w-64 shrink-0 border-r border-slate-200 bg-white p-4 flex flex-col gap-4 min-h-0" aria-label="Original image">
      <h2 class="text-sm font-semibold text-slate-600 uppercase tracking-wide">Original image</h2>
      <ImageImport
        loading={loading}
        errorMessage={loadError}
        onLoadStart={handleLoadStart}
        onLoadSuccess={handleLoadSuccess}
        onLoadError={handleLoadError}
      />
      <!-- UI-103: loaded image preview -->
      {#if previewUrl && !loading}
        <div class="rounded border border-slate-200 overflow-hidden bg-slate-100 flex-1 min-h-0 flex flex-col">
          <img
            src={previewUrl}
            alt="Loaded preview"
            class="w-full h-auto max-h-[240px] object-contain"
          />
          <!-- UI-104: image metadata -->
          {#if loadedResult}
            <dl class="p-2 text-xs text-slate-600 mt-auto border-t border-slate-200" role="group" aria-label="Image metadata">
              <div class="flex justify-between gap-2">
                <dt class="text-slate-500">Dimensions</dt>
                <dd>{loadedResult.width ?? "—"} × {loadedResult.height ?? "—"}</dd>
              </div>
              <div class="flex justify-between gap-2">
                <dt class="text-slate-500">Size</dt>
                <dd>{formatFileSize(loadedResult.fileSizeBytes)}</dd>
              </div>
              {#if loadedResult.downsampled}
                <p class="text-amber-600 mt-1" role="status">Image was downsampled to fit 8K limit.</p>
              {/if}
            </dl>
          {/if}
        </div>
      {/if}
    </aside>

    <!-- Center: 3D preview -->
    <section class="flex-1 min-w-0 flex flex-col p-4">
      <h2 class="text-sm font-semibold text-slate-600 uppercase tracking-wide mb-2">3D Preview</h2>
      <div class="flex-1 min-h-0 rounded overflow-hidden">
        <Preview3D
          targetWidthMm={effectiveTargetWidthMm}
          targetHeightMm={effectiveTargetHeightMm}
        />
      </div>
    </section>

    <!-- Right sidebar: depth preview + controls (UI-301–305, JR1-301). min-h-0 + overflow so column doesn't extend page. -->
    <aside class="w-64 shrink-0 min-h-0 border-l border-slate-200 bg-white p-4 flex flex-col gap-4 overflow-hidden" aria-label="Depth map and controls">
      <h2 class="text-sm font-semibold text-slate-600 uppercase tracking-wide shrink-0">Depth map</h2>
      <div class="flex-1 min-h-0 flex flex-col gap-2 rounded border border-slate-200 overflow-y-auto overflow-x-hidden">
        <div class="depth-map-slot min-h-[200px] max-h-[40vh] h-[280px] shrink-0 overflow-hidden rounded">
          <DepthMapPreview
            width={depthMap?.width ?? 0}
            height={depthMap?.height ?? 0}
            depth={depthMap?.depth ?? []}
            estimating={depthEstimating}
            hasImage={!!loadPath}
            maskData={maskData}
            showMaskOverlay={showMaskOverlay}
            activeMaskTool={maskTool === "brush" || maskTool === "eraser" ? maskTool : null}
            brushSize={brushSize}
            onMaskPaint={handleMaskPaint}
          />
        </div>
        <!-- UI-303: Generate Depth Map button; UI-304: progress during inference -->
        <div class="p-2 border-t border-slate-200 flex flex-col gap-2">
          <Button
            variant="primary"
            title="Run AI depth estimation on the loaded image"
            disabled={!loadPath || depthEstimating}
            on:click={handleGenerateDepth}
          >
            {depthEstimating ? "Estimating…" : "Generate Depth Map"}
          </Button>
          <!-- UI-304: Determinate progress bar from depth-progress events -->
          {#if depthEstimating}
            <div
              class="progress-wrapper h-2 rounded-full bg-slate-200 overflow-hidden"
              role="progressbar"
              aria-label="Depth estimation in progress"
              aria-valuenow={progressPercent}
              aria-valuemin={0}
              aria-valuemax={100}
              aria-valuetext={progressPercent === 0 ? "Starting…" : `${progressPercent}%`}
            >
              <div
                class="h-full rounded-full bg-slate-500 transition-[width] duration-200"
                style="width: {progressPercent === 0 ? 2 : progressPercent}%"
              ></div>
            </div>
            <p class="text-xs text-slate-500" role="status">
              {progressPercent === 0 ? "Starting…" : `${progressPercent}%`}
            </p>
          {/if}
          {#if depthError}
            <p
              class="text-sm text-red-600 bg-red-50 border border-red-200 rounded px-2 py-1.5 break-words"
              role="alert"
              aria-live="polite"
              title={depthError}
            >
              {depthError}
            </p>
          {/if}
        </div>
        <!-- UI-401–405: Depth adjustment controls (debounced preview in App) -->
        <DepthControls
          hasDepth={depthMap != null && depthMap.depth.length > 0}
          histogram={histogramData}
          params={adjustmentParams}
          onParamsChange={handleParamsChange}
          onReset={handleResetDepth}
          advancedMode={advancedMode}
          onAdvancedModeChange={(v) => (advancedMode = v)}
        />
        <!-- UI-1201–1204: Mask tools (brush, eraser, select, size, hardness); JR1-1202/1203 save/load/apply selection -->
        <MaskingTools
          hasDepth={depthMap != null && depthMap.depth.length > 0}
          depthWidth={depthMap?.width ?? 0}
          depthHeight={depthMap?.height ?? 0}
          tool={maskTool}
          onToolChange={(t) => (maskTool = t)}
          brushSize={brushSize}
          onBrushSizeChange={(v) => (brushSize = v)}
          brushHardness={brushHardness}
          onBrushHardnessChange={(v) => (brushHardness = v)}
          onClearMask={handleClearMask}
          onMaskChange={handleMaskChange}
        />
        <!-- UI-1301, UI-1302: Preset manager and Save/Load -->
        <PresetManager onListChange={refreshPresetList} onPresetApplied={applyPresetAndRefresh} />
        <div class="flex flex-col gap-2 pt-2 border-t border-slate-200" role="group" aria-label="Save and load presets">
          <Button
            variant="secondary"
            title="Save current depth and mesh settings as a preset"
            disabled={savingPreset || !depthMap}
            on:click={handleSaveAsPreset}
          >
            {savingPreset ? "Saving…" : "Save as preset"}
          </Button>
          <div class="relative">
            <button
              type="button"
              class="w-full px-3 py-1.5 text-sm rounded border border-slate-300 bg-white text-slate-700 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-slate-400 flex items-center justify-between"
              title="Load a saved or built-in preset"
              disabled={presetListLoading}
              on:click={() => { presetSaveLoadOpen = !presetSaveLoadOpen; presetDropdownOpen = false; if (!presetSaveLoadOpen) return; refreshPresetList(); }}
              aria-haspopup="listbox"
              aria-expanded={presetSaveLoadOpen}
            >
              <span>{presetListLoading ? "Loading…" : "Load preset"}</span>
              <span class="text-slate-400">▼</span>
            </button>
            {#if presetSaveLoadOpen}
              <ul
                class="absolute left-0 right-0 mt-1 max-h-48 overflow-y-auto rounded border border-slate-200 bg-white shadow z-10"
                role="listbox"
              >
                {#each presetList as item (item.id)}
                  <li role="option" aria-selected="false">
                    <button
                      type="button"
                      class="w-full text-left px-3 py-2 text-sm hover:bg-slate-100 border-b border-slate-100 last:border-0 {item.kind === 'builtin' ? 'text-slate-500' : 'text-slate-700'}"
                      on:click={() => handleLoadPreset(item.id)}
                    >
                      {item.name}{item.kind === "builtin" ? " (built-in)" : ""}
                    </button>
                  </li>
                {/each}
                {#if presetList.length === 0 && !presetListLoading}
                  <li class="px-3 py-2 text-sm text-slate-500">No presets</li>
                {/if}
              </ul>
            {/if}
          </div>
          {#if presetMessage}
            <p class="text-xs text-slate-600" role="status">{presetMessage}</p>
          {/if}
        </div>
      </div>
    </aside>
  </main>

  <!-- Bottom: status, undo/redo (UI-1401), output scale, export panel -->
  <footer class="shrink-0 border-t border-slate-200 bg-white px-4 py-2 flex items-center justify-between gap-4 flex-wrap">
    <div class="flex items-center gap-3">
      <div class="text-sm text-slate-500" role="status" aria-live="polite" aria-label={status}>{status}</div>
      <!-- UI-1401: Undo/Redo (BACK-1404); disabled when nothing to undo/redo -->
      <div class="flex items-center gap-1" role="group" aria-label="Undo and redo">
        <button
          type="button"
          class="px-2 py-1 text-sm rounded border border-slate-300 bg-white text-slate-700 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-slate-500 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-white"
          title="Undo (Ctrl+Z)"
          disabled={!canUndo}
          on:click={handleUndo}
        >
          Undo
        </button>
        <button
          type="button"
          class="px-2 py-1 text-sm rounded border border-slate-300 bg-white text-slate-700 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-slate-500 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-white"
          title="Redo (Ctrl+Y)"
          disabled={!canRedo}
          on:click={handleRedo}
        >
          Redo
        </button>
      </div>
    </div>
    <!-- UI-1303: Preset dropdown (built-in + user); UI-1304: Import/Export -->
    <div class="flex items-center gap-2" role="group" aria-label="Presets">
      <span class="text-xs text-slate-500">Preset:</span>
      <div class="relative">
        <button
          type="button"
          class="px-2 py-1 text-sm rounded border border-slate-300 bg-white text-slate-700 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-slate-500 disabled:opacity-50 flex items-center gap-1"
          title="Apply a preset"
          disabled={presetListLoading}
          on:click={() => { presetDropdownOpen = !presetDropdownOpen; presetSaveLoadOpen = false; if (presetDropdownOpen) refreshPresetList(); }}
          aria-haspopup="listbox"
          aria-expanded={presetDropdownOpen}
        >
          <span>Apply preset</span>
          <span class="text-slate-400 text-xs">▼</span>
        </button>
        {#if presetDropdownOpen}
          <ul
            class="absolute left-0 bottom-full mb-1 max-h-48 overflow-y-auto rounded border border-slate-200 bg-white shadow z-20 min-w-[140px]"
            role="listbox"
          >
            {#each presetList as item (item.id)}
              <li role="option" aria-selected="false">
                <button
                  type="button"
                  class="w-full text-left px-3 py-2 text-sm text-slate-700 hover:bg-slate-100 border-b border-slate-100 last:border-0"
                  on:click={() => handleLoadPreset(item.id)}
                >
                  {item.name}
                </button>
              </li>
            {/each}
            {#if presetList.length === 0 && !presetListLoading}
              <li class="px-3 py-2 text-sm text-slate-500">No presets</li>
            {/if}
          </ul>
        {/if}
      </div>
      <button
        type="button"
        class="px-2 py-1 text-xs rounded border border-slate-300 bg-white text-slate-700 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-slate-500"
        title="Export preset to JSON file"
        on:click={handleExportPreset}
      >
        Export preset
      </button>
      <button
        type="button"
        class="px-2 py-1 text-xs rounded border border-slate-300 bg-white text-slate-700 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-slate-500"
        title="Import preset from JSON file"
        on:click={handleImportPreset}
      >
        Import preset
      </button>
    </div>
    <!-- Scaling: default 40×40 mm; zoom to scale target (depth-map and 3D preview dimension to fit). -->
    <div class="flex items-center gap-2" role="group" aria-label="Output scale and target size">
      <span class="text-xs text-slate-500">Target:</span>
      <span class="text-sm text-slate-700 font-medium">{effectiveTargetWidthMm}×{effectiveTargetHeightMm} mm</span>
      <span class="text-xs text-slate-400">Zoom:</span>
      <div class="flex gap-0.5">
        {#each [0.5, 1, 1.5, 2] as scale}
          <button
            type="button"
            class="px-2 py-0.5 text-xs rounded border {zoomScale === scale ? 'bg-slate-200 border-slate-400 font-medium' : 'bg-slate-50 border-slate-200 hover:bg-slate-100'}"
            title="{scale * 100}% — target {Math.round(TARGET_BASE_MM * scale)}×{Math.round(TARGET_BASE_MM * scale)} mm"
            on:click={() => setZoom(scale)}
          >
            {scale * 100}%
          </button>
        {/each}
      </div>
    </div>
    <ExportPanel
      hasDepth={depthMap != null && depthMap.depth.length > 0}
      sourceFileName={sourceFileName}
      effectiveTargetWidthMm={effectiveTargetWidthMm}
      effectiveTargetHeightMm={effectiveTargetHeightMm}
    />
  </footer>
</div>

<style>
  .app-layout {
    /* PRD: window min 1024×768; layout uses flex/grid */
  }
</style>
