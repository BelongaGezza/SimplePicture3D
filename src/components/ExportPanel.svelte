<script lang="ts">
  /**
   * ExportPanel — UI-701 through UI-704.
   * Export panel with format selector, export button, progress indicator, and success notification.
   * Placed in the bottom footer bar of the workspace layout.
   */
  import Button from "./Button.svelte";
  import { save } from "@tauri-apps/plugin-dialog";
  import { open as shellOpen } from "@tauri-apps/plugin-shell";
  import { exportStl } from "$lib/tauri";

  /** Whether a depth map / mesh is available for export. */
  export let hasDepth = false;
  /** Source image filename (used for default export filename). */
  export let sourceFileName = "";

  /** Selected export format. STL is active; OBJ is placeholder for Sprint 1.9. */
  let selectedFormat: "stl" | "obj" = "stl";
  /** True while export is in progress. */
  let exporting = false;
  /** Error message from last export attempt. */
  let exportError = "";
  /** Path of the last successfully exported file. */
  let exportedPath = "";
  /** Whether the success notification is visible. */
  let showSuccess = false;
  /** Whether the format dropdown is open. */
  let dropdownOpen = false;

  const formats = [
    { value: "stl" as const, label: "STL (Binary)", enabled: true },
    { value: "obj" as const, label: "OBJ (ASCII)", enabled: false },
  ];

  /** Generate a default filename from source image name + timestamp. */
  function defaultFileName(): string {
    const base = sourceFileName
      ? sourceFileName.replace(/\.[^.]+$/, "")
      : "export";
    const now = new Date();
    const ts = [
      now.getFullYear(),
      String(now.getMonth() + 1).padStart(2, "0"),
      String(now.getDate()).padStart(2, "0"),
      "_",
      String(now.getHours()).padStart(2, "0"),
      String(now.getMinutes()).padStart(2, "0"),
      String(now.getSeconds()).padStart(2, "0"),
    ].join("");
    return `${base}_${ts}.stl`;
  }

  /** Extract the containing directory from a file path. */
  function parentDir(filePath: string): string {
    // Handle both forward and back slashes (cross-platform)
    const lastSep = Math.max(filePath.lastIndexOf("/"), filePath.lastIndexOf("\\"));
    return lastSep > 0 ? filePath.substring(0, lastSep) : filePath;
  }

  /** UI-702: Handle export button click — open save dialog, then call export_stl. */
  async function handleExport() {
    if (!hasDepth || exporting || selectedFormat !== "stl") return;

    exportError = "";
    exportedPath = "";
    showSuccess = false;

    try {
      // Open native save dialog with STL filter
      const path = await save({
        defaultPath: defaultFileName(),
        filters: [{ name: "STL files", extensions: ["stl"] }],
      });

      // User cancelled the dialog
      if (!path) return;

      // UI-703: Show progress and disable button during export
      exporting = true;

      await exportStl(path);

      // UI-704: Show success notification
      exportedPath = path;
      showSuccess = true;
      exportError = "";
    } catch (e) {
      exportError = String(e);
      showSuccess = false;
    } finally {
      exporting = false;
    }
  }

  /** UI-704: Open the folder containing the exported file in the OS file manager. */
  async function handleOpenFolder() {
    if (!exportedPath) return;
    try {
      await shellOpen(parentDir(exportedPath));
    } catch (e) {
      // Fallback: if opening the directory fails, try opening the file itself
      try {
        await shellOpen(exportedPath);
      } catch {
        exportError = "Could not open folder: " + String(e);
      }
    }
  }

  /** UI-704: Dismiss the success notification. */
  function dismissSuccess() {
    showSuccess = false;
    exportedPath = "";
  }

  /** Close dropdown when clicking outside. */
  function handleDropdownBlur(e: FocusEvent) {
    // Delay to allow click on dropdown item to register
    setTimeout(() => {
      dropdownOpen = false;
    }, 150);
  }
</script>

<div
  class="export-panel flex items-center gap-3"
  role="region"
  aria-label="Export panel"
>
  <!-- UI-702: Format dropdown -->
  <div class="relative" role="group" aria-label="Export format">
    <button
      type="button"
      class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded border border-slate-300 bg-white text-sm text-slate-700 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-slate-500 focus:ring-offset-1 disabled:opacity-50 disabled:pointer-events-none"
      disabled={!hasDepth || exporting}
      on:click={() => (dropdownOpen = !dropdownOpen)}
      on:blur={handleDropdownBlur}
      aria-haspopup="listbox"
      aria-expanded={dropdownOpen}
      aria-label="Select export format: {formats.find((f) => f.value === selectedFormat)?.label}"
    >
      <span>{formats.find((f) => f.value === selectedFormat)?.label ?? "STL"}</span>
      <svg class="w-3.5 h-3.5 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    {#if dropdownOpen}
      <ul
        class="absolute bottom-full mb-1 left-0 w-44 bg-white border border-slate-200 rounded shadow-lg z-20 py-1"
        role="listbox"
        aria-label="Export formats"
      >
        {#each formats as fmt}
          <li
            role="option"
            aria-selected={selectedFormat === fmt.value}
            aria-disabled={!fmt.enabled}
            class="px-3 py-1.5 text-sm cursor-pointer
              {fmt.enabled
                ? selectedFormat === fmt.value
                  ? 'bg-slate-100 text-slate-800 font-medium'
                  : 'text-slate-700 hover:bg-slate-50'
                : 'text-slate-400 cursor-not-allowed'}"
            on:click|stopPropagation={() => {
              if (fmt.enabled) {
                selectedFormat = fmt.value;
                dropdownOpen = false;
              }
            }}
            on:keydown={(e) => {
              if (e.key === 'Enter' && fmt.enabled) {
                selectedFormat = fmt.value;
                dropdownOpen = false;
              }
            }}
          >
            {fmt.label}
            {#if !fmt.enabled}
              <span class="text-xs text-slate-400 ml-1">(Sprint 1.9)</span>
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
  </div>

  <!-- UI-702: Export button -->
  <Button
    variant="primary"
    title={!hasDepth ? "Generate a depth map first" : exporting ? "Exporting..." : "Export mesh as " + selectedFormat.toUpperCase()}
    disabled={!hasDepth || exporting || selectedFormat !== "stl"}
    on:click={handleExport}
  >
    {#if exporting}
      <svg
        class="animate-spin -ml-0.5 mr-1.5 h-4 w-4"
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        aria-hidden="true"
      >
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
      </svg>
      Exporting...
    {:else}
      Export
    {/if}
  </Button>

  <!-- UI-703: Progress indicator (indeterminate bar during export) -->
  {#if exporting}
    <div
      class="export-progress w-24 h-1.5 rounded-full bg-slate-200 overflow-hidden"
      role="progressbar"
      aria-label="Export in progress"
      aria-valuetext="Exporting"
    >
      <div class="export-progress-bar h-full rounded-full bg-slate-600"></div>
    </div>
  {/if}

  <!-- UI-703: Export error display -->
  {#if exportError}
    <div
      class="flex items-center gap-2 text-sm text-red-600 bg-red-50 border border-red-200 rounded px-2.5 py-1 max-w-xs"
      role="alert"
      aria-live="polite"
    >
      <span class="truncate" title={exportError}>{exportError}</span>
      <button
        type="button"
        class="shrink-0 text-red-400 hover:text-red-600 focus:outline-none focus:ring-1 focus:ring-red-400 rounded"
        on:click={() => (exportError = "")}
        aria-label="Dismiss error"
        title="Dismiss"
      >
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  {/if}

  <!-- UI-704: Success notification -->
  {#if showSuccess}
    <div
      class="flex items-center gap-2 text-sm text-green-700 bg-green-50 border border-green-200 rounded px-2.5 py-1 max-w-md"
      role="status"
      aria-live="polite"
    >
      <svg class="shrink-0 w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
      </svg>
      <span class="truncate" title={exportedPath}>
        Exported: {exportedPath.split(/[/\\]/).pop() ?? exportedPath}
      </span>
      <button
        type="button"
        class="shrink-0 px-2 py-0.5 text-xs rounded bg-green-100 hover:bg-green-200 text-green-700 focus:outline-none focus:ring-1 focus:ring-green-400"
        on:click={handleOpenFolder}
        title="Open containing folder in file manager"
      >
        Open Folder
      </button>
      <button
        type="button"
        class="shrink-0 text-green-400 hover:text-green-600 focus:outline-none focus:ring-1 focus:ring-green-400 rounded"
        on:click={dismissSuccess}
        aria-label="Dismiss notification"
        title="Dismiss"
      >
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  {/if}
</div>

<style>
  /* UI-703: Indeterminate progress bar animation during export */
  .export-progress-bar {
    width: 40%;
    animation: export-slide 1.5s ease-in-out infinite;
  }
  @keyframes export-slide {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(350%);
    }
  }
</style>
