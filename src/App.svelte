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
  import {
    generateDepthMap,
    getDepthMap,
    getDepthHistogram,
    getDepthAdjustmentParams,
    setDepthAdjustmentParams,
    resetDepthAdjustments,
    checkModel,
    getSettings,
    saveSettings,
  } from "$lib/tauri";
  import type { LoadImageResult, DepthAdjustmentParams } from "$lib/tauri";

  let status = "Ready";
  let loadPath = "";
  let loading = false;
  let loadError = "";
  let loadedResult: LoadImageResult | null = null;

  /** Sprint 1.10: Show first-run wizard if model is not installed (UI-901). */
  let showWizard = false;
  import { onMount } from "svelte";
  onMount(async () => {
    try {
      const modelStatus = await checkModel();
      if (!modelStatus.installed) {
        showWizard = true;
      }
    } catch {
      // Model check failed; don't block the app
    }
  });

  /** Depth map for preview (BACK-303, UI-301/302). Adjusted by backend when params change (UI-404). */
  let depthMap: { width: number; height: number; depth: number[] } | null = null;
  /** True while generate_depth_map is running (UI-304). */
  let depthEstimating = false;
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

  /** UI-303: Generate depth map from current image path. */
  async function handleGenerateDepth() {
    if (!loadPath || depthEstimating) return;
    depthEstimating = true;
    depthError = "";
    status = "Estimating depth…";
    try {
      const result = await generateDepthMap(loadPath);
      depthMap = { width: result.width, height: result.height, depth: result.depth };
      const params = await getDepthAdjustmentParams();
      adjustmentParams = params;
      const hist = await getDepthHistogram();
      histogramData = hist;
      status = "Depth ready";
    } catch (e) {
      depthError = String(e);
      status = "Depth error";
    } finally {
      depthEstimating = false;
    }
  }

  /** UI-404: Apply params to backend and refresh preview (debounced). BACK-1101: also fetch histogram. */
  async function applyParamsToBackend() {
    debounceTimer = null;
    if (!depthMap) return;
    try {
      await setDepthAdjustmentParams(adjustmentParams);
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
      await resetDepthAdjustments();
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
</script>

<!-- Sprint 1.10: First-run wizard (UI-901) -->
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
          <!-- UI-304: Indeterminate progress bar during estimation (MVP: no % until complete) -->
          {#if depthEstimating}
            <div
              class="progress-wrapper h-2 rounded-full bg-slate-200 overflow-hidden"
              role="progressbar"
              aria-label="Depth estimation in progress"
              aria-valuetext="Estimating"
            >
              <div class="progress-bar-indeterminate h-full rounded-full bg-slate-500"></div>
            </div>
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
      </div>
    </aside>
  </main>

  <!-- Bottom: output scale (zoom) + export panel + status bar (UI-701–704, scaling) -->
  <footer class="shrink-0 border-t border-slate-200 bg-white px-4 py-2 flex items-center justify-between gap-4 flex-wrap">
    <div class="text-sm text-slate-500" role="status" aria-live="polite" aria-label={status}>{status}</div>
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
  /* UI-304: indeterminate progress bar during depth estimation */
  .progress-bar-indeterminate {
    width: 40%;
    animation: progress-slide 1.5s ease-in-out infinite;
  }
  @keyframes progress-slide {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(350%);
    }
  }
</style>
