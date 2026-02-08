<script lang="ts">
  import ImageImport from "./components/ImageImport.svelte";
  import Preview3D from "./components/Preview3D.svelte";
  import DepthMapPreview from "./components/DepthMapPreview.svelte";
  import DepthControls from "./components/DepthControls.svelte";
  import ExportPanel from "./components/ExportPanel.svelte";
  import Button from "./components/Button.svelte";
  import {
    generateDepthMap,
    getDepthMap,
    getDepthAdjustmentParams,
    setDepthAdjustmentParams,
    resetDepthAdjustments,
  } from "$lib/tauri";
  import type { LoadImageResult, DepthAdjustmentParams } from "$lib/tauri";

  let status = "Ready";
  let loadPath = "";
  let loading = false;
  let loadError = "";
  let loadedResult: LoadImageResult | null = null;

  /** Depth map for preview (BACK-303, UI-301/302). Adjusted by backend when params change (UI-404). */
  let depthMap: { width: number; height: number; depth: number[] } | null = null;
  /** True while generate_depth_map is running (UI-304). */
  let depthEstimating = false;
  /** Backend error from generate_depth_map (timeout, Python, etc.). */
  let depthError = "";

  /** Current depth adjustment params; synced with backend (UI-401–405). */
  let adjustmentParams: DepthAdjustmentParams = {
    brightness: 0,
    contrast: 1,
    gamma: 1,
    invert: false,
    depthMinMm: 2,
    depthMaxMm: 10,
  };

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
  }

  function handleLoadSuccess(result: LoadImageResult) {
    loading = false;
    loadedResult = result;
    loadError = "";
    status = result.downsampled ? "Loaded (downsampled)" : "Loaded";
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
      status = "Depth ready";
    } catch (e) {
      depthError = String(e);
      status = "Depth error";
    } finally {
      depthEstimating = false;
    }
  }

  /** UI-404: Apply params to backend and refresh preview (debounced). */
  async function applyParamsToBackend() {
    debounceTimer = null;
    if (!depthMap) return;
    try {
      await setDepthAdjustmentParams(adjustmentParams);
      if (import.meta.env.DEV) console.time("getDepthMap");
      const result = await getDepthMap();
      if (import.meta.env.DEV) {
        console.timeEnd("getDepthMap");
        if (result) console.debug("getDepthMap dimensions:", result.width, "x", result.height);
      }
      if (result) depthMap = { width: result.width, height: result.height, depth: result.depth };
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

  /** UI-405: Reset adjustments and refresh preview from original depth. */
  async function handleResetDepth() {
    try {
      await resetDepthAdjustments();
      adjustmentParams = await getDepthAdjustmentParams();
      if (import.meta.env.DEV) console.time("getDepthMap");
      const result = await getDepthMap();
      if (import.meta.env.DEV) {
        console.timeEnd("getDepthMap");
        if (result) console.debug("getDepthMap dimensions:", result.width, "x", result.height);
      }
      if (result) depthMap = { width: result.width, height: result.height, depth: result.depth };
      status = "Depth reset";
    } catch (e) {
      status = "Reset error: " + String(e);
    }
  }
</script>

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
            alt="Loaded image preview"
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
        <Preview3D />
      </div>
    </section>

    <!-- Right sidebar: depth preview + controls (UI-301–305, JR1-301) -->
    <aside class="w-64 shrink-0 border-l border-slate-200 bg-white p-4 flex flex-col gap-4" aria-label="Depth map and controls">
      <h2 class="text-sm font-semibold text-slate-600 uppercase tracking-wide">Depth map</h2>
      <div class="flex-1 min-h-0 flex flex-col gap-2 rounded border border-slate-200 overflow-hidden">
        <div class="min-h-[200px] flex-1 min-h-0">
          <DepthMapPreview
            width={depthMap?.width ?? 0}
            height={depthMap?.height ?? 0}
            depth={depthMap?.depth ?? []}
            estimating={depthEstimating}
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
          params={adjustmentParams}
          onParamsChange={handleParamsChange}
          onReset={handleResetDepth}
        />
      </div>
    </aside>
  </main>

  <!-- Bottom: export panel + status bar (UI-701–704) -->
  <footer class="shrink-0 border-t border-slate-200 bg-white px-4 py-2 flex items-center justify-between gap-4">
    <div class="text-sm text-slate-500" role="status" aria-live="polite" aria-label={status}>{status}</div>
    <ExportPanel
      hasDepth={depthMap != null && depthMap.depth.length > 0}
      sourceFileName={sourceFileName}
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
