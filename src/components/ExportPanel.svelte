<script lang="ts">
  /**
   * ExportPanel — UI-701 through UI-704 (Sprint 1.8), UI-801 through UI-804 (Sprint 1.9).
   * Export panel with format selector (STL/OBJ), export button, progress indicator,
   * success notification, and settings persistence.
   */
  import Button from "./Button.svelte";
  import { save } from "@tauri-apps/plugin-dialog";
  import { open as shellOpen } from "@tauri-apps/plugin-shell";
  import { exportStl, exportObj, getSettings, saveSettings } from "$lib/tauri";
  import type { AppSettings } from "$lib/tauri";
  import { onMount } from "svelte";

  /** Whether a depth map / mesh is available for export. */
  export let hasDepth = false;
  /** Source image filename (used for default export filename). */
  export let sourceFileName = "";

  /** Selected export format. Both STL and OBJ are now active. */
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
  /** Whether the settings panel is open (UI-802). */
  let showSettings = false;
  /** Default export directory from settings (UI-803). */
  let defaultExportDir = "";

  const formats = [
    { value: "stl" as const, label: "STL (Binary)", enabled: true },
    { value: "obj" as const, label: "OBJ (ASCII)", enabled: true },
  ];

  /** Load saved settings on mount (BACK-804). */
  onMount(async () => {
    try {
      const settings = await getSettings();
      if (settings.exportFormat === "obj") {
        selectedFormat = "obj";
      }
      if (settings.lastExportDir) {
        defaultExportDir = settings.lastExportDir;
      }
    } catch {
      // Settings not available; use defaults
    }
  });

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
    return `${base}_${ts}.${selectedFormat}`;
  }

  /** Extract the containing directory from a file path. */
  function parentDir(filePath: string): string {
    const lastSep = Math.max(filePath.lastIndexOf("/"), filePath.lastIndexOf("\\"));
    return lastSep > 0 ? filePath.substring(0, lastSep) : filePath;
  }

  /** UI-702: Handle export button click — open save dialog, then call export command. */
  async function handleExport() {
    if (!hasDepth || exporting) return;

    exportError = "";
    exportedPath = "";
    showSuccess = false;

    try {
      const ext = selectedFormat;
      const filterName = ext === "stl" ? "STL files" : "OBJ files";

      // Open native save dialog with format-appropriate filter
      const path = await save({
        defaultPath: defaultFileName(),
        filters: [{ name: filterName, extensions: [ext] }],
      });

      // User cancelled the dialog
      if (!path) return;

      // UI-703: Show progress and disable button during export
      exporting = true;

      if (ext === "stl") {
        await exportStl(path);
      } else {
        await exportObj(path);
      }

      // Persist format preference (BACK-804)
      try {
        const settings = await getSettings();
        settings.exportFormat = ext;
        await saveSettings(settings);
      } catch {
        // Non-critical; continue
      }

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
    setTimeout(() => {
      dropdownOpen = false;
    }, 150);
  }

  /** UI-804: Reset settings to defaults. */
  async function handleResetSettings() {
    try {
      await saveSettings({});
      selectedFormat = "stl";
      defaultExportDir = "";
      showSettings = false;
    } catch (e) {
      exportError = "Failed to reset settings: " + String(e);
    }
  }
</script>

<div
  class="export-panel flex items-center gap-3"
  role="region"
  aria-label="Export panel"
>
  <!-- UI-702/UI-801: Format dropdown (both STL and OBJ active) -->
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
          </li>
        {/each}
      </ul>
    {/if}
  </div>

  <!-- UI-702: Export button (now works for both STL and OBJ) -->
  <Button
    variant="primary"
    title={!hasDepth ? "Generate a depth map first" : exporting ? "Exporting..." : "Export mesh as " + selectedFormat.toUpperCase()}
    disabled={!hasDepth || exporting}
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

  <!-- UI-802: Settings button -->
  <button
    type="button"
    class="p-1.5 rounded text-slate-500 hover:text-slate-700 hover:bg-slate-100 focus:outline-none focus:ring-2 focus:ring-slate-500 focus:ring-offset-1"
    on:click={() => (showSettings = !showSettings)}
    title="Export settings"
    aria-label="Toggle export settings"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
    </svg>
  </button>

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

<!-- UI-802: Settings panel (collapsible, positioned above the export bar) -->
{#if showSettings}
  <div
    class="settings-panel absolute bottom-full mb-2 left-0 right-0 bg-white border border-slate-200 rounded-lg shadow-lg p-4 z-30"
    role="dialog"
    aria-label="Export settings"
  >
    <div class="flex items-center justify-between mb-3">
      <h3 class="text-sm font-medium text-slate-800">Export Settings</h3>
      <button
        type="button"
        class="text-slate-400 hover:text-slate-600 focus:outline-none"
        on:click={() => (showSettings = false)}
        aria-label="Close settings"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- UI-803: Default export directory display -->
    {#if defaultExportDir}
      <div class="mb-3">
        <label class="block text-xs text-slate-500 mb-1">Last export directory</label>
        <p class="text-sm text-slate-700 truncate" title={defaultExportDir}>{defaultExportDir}</p>
      </div>
    {/if}

    <!-- UI-801: Default format selector -->
    <div class="mb-3">
      <label class="block text-xs text-slate-500 mb-1" for="settings-format">Default format</label>
      <select
        id="settings-format"
        class="w-full px-2 py-1 text-sm border border-slate-300 rounded focus:outline-none focus:ring-2 focus:ring-slate-500"
        bind:value={selectedFormat}
      >
        {#each formats as fmt}
          <option value={fmt.value} disabled={!fmt.enabled}>{fmt.label}</option>
        {/each}
      </select>
    </div>

    <!-- UI-804: Reset settings button -->
    <div class="flex justify-end">
      <button
        type="button"
        class="px-3 py-1 text-xs text-slate-600 bg-slate-100 hover:bg-slate-200 rounded focus:outline-none focus:ring-2 focus:ring-slate-500"
        on:click={handleResetSettings}
      >
        Reset to Defaults
      </button>
    </div>
  </div>
{/if}

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
