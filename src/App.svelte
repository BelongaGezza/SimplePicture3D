<script lang="ts">
  import ImageImport from "./components/ImageImport.svelte";
  import Preview3D from "./components/Preview3D.svelte";
  import Button from "./components/Button.svelte";
  import { loadImage, exportStl } from "$lib/tauri";

  let status = "Ready";
  let loadPath = "";

  async function handleExport() {
    status = "Exporting…";
    try {
      await exportStl(loadPath || "output.stl");
      status = "Exported";
    } catch (e) {
      status = "Export error: " + String(e);
    }
  }

  async function handleLoad(path: string) {
    loadPath = path;
    status = "Loading…";
    try {
      const res = await loadImage(path);
      status = res.ok ? "Loaded" : res.message ?? "Loaded";
    } catch (e) {
      status = "Load error: " + String(e);
    }
  }
</script>

<div class="app-layout min-h-screen flex flex-col bg-slate-50 text-slate-800">
  <main class="flex flex-1 min-h-0">
    <!-- Left sidebar: image import, preview -->
    <aside class="w-64 shrink-0 border-r border-slate-200 bg-white p-4 flex flex-col gap-4">
      <h2 class="text-sm font-semibold text-slate-600 uppercase tracking-wide">Image</h2>
      <ImageImport onLoad={handleLoad} />
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
