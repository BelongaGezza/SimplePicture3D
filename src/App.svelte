<script lang="ts">
  import ImageImport from "./components/ImageImport.svelte";
  import Preview3D from "./components/Preview3D.svelte";
  import Button from "./components/Button.svelte";
  import { exportStl } from "$lib/tauri";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import type { LoadImageResult } from "$lib/tauri";

  let status = "Ready";
  let loadPath = "";
  let loading = false;
  let loadError = "";
  let loadedResult: LoadImageResult | null = null;

  /** Preview URL: base64 from backend or converted path for webview. */
  $: previewUrl =
    loadedResult?.previewData ||
    (loadedResult?.path ? convertFileSrc(loadedResult.path) : "");

  function formatFileSize(bytes: number | undefined): string {
    if (bytes == null) return "—";
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  }

  async function handleExport() {
    status = "Exporting…";
    try {
      await exportStl(loadPath || "output.stl");
      status = "Exported";
    } catch (e) {
      status = "Export error: " + String(e);
    }
  }

  function handleLoadStart(path: string) {
    loadPath = path;
    status = "Loading…";
    loading = true;
    loadError = "";
    loadedResult = null;
  }

  function handleLoadSuccess(result: LoadImageResult) {
    loading = false;
    loadedResult = result;
    loadPath = result.path ?? loadPath;
    loadError = "";
    status = result.downsampled ? "Loaded (downsampled)" : "Loaded";
  }

  function handleLoadError(message: string) {
    loading = false;
    loadError = message;
    loadedResult = null;
    status = "Load error";
  }
</script>

<div class="app-layout min-h-screen flex flex-col bg-slate-50 text-slate-800">
  <main class="flex flex-1 min-h-0">
    <!-- Left sidebar: image import, preview (UI-101–105) -->
    <aside class="w-64 shrink-0 border-r border-slate-200 bg-white p-4 flex flex-col gap-4 min-h-0">
      <h2 class="text-sm font-semibold text-slate-600 uppercase tracking-wide">Image</h2>
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
                <dd>{formatFileSize(loadedResult.fileSize)}</dd>
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

    <!-- Right sidebar: depth controls -->
    <aside class="w-64 shrink-0 border-l border-slate-200 bg-white p-4 flex flex-col gap-4">
      <h2 class="text-sm font-semibold text-slate-600 uppercase tracking-wide">Depth</h2>
      <div class="flex-1 min-h-0 rounded border border-slate-200 flex items-center justify-center text-slate-400 text-sm">
        Controls area
      </div>
    </aside>
  </main>

  <!-- Bottom: export button, status bar -->
  <footer class="shrink-0 border-t border-slate-200 bg-white px-4 py-2 flex items-center justify-between gap-4">
    <div class="text-sm text-slate-500">{status}</div>
    <Button variant="primary" title="Export mesh as STL" on:click={handleExport}>
      Export
    </Button>
  </footer>
</div>

<style>
  .app-layout {
    /* PRD: window min 1024×768; layout uses flex/grid */
  }
</style>
