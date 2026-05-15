<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<script lang="ts">
  /**
   * ExportPanel — ADR-012 point cloud export (PLY / XYZ / CSV).
   * Persists crystal blank bounds to the Rust backend before generate/export.
   */
  import { save as saveDialog } from "@tauri-apps/plugin-dialog";
  import {
    setBlankEnvelope,
    setPointCloudFormat,
    generatePointCloud,
    exportPly,
    exportXyz,
    exportCsv,
  } from "$lib/tauri";

  export let hasDepth = false;
  export let sourceFileName = "";
  /** Legacy props (kept for App / layout parity). */
  export let effectiveTargetWidthMm: number | undefined = undefined;
  export let effectiveTargetHeightMm: number | undefined = undefined;

  export let blankLengthMm = 80;
  export let blankWidthMm = 50;
  export let blankHeightMm = 50;
  export let blankMarginMm = 2;

  type CloudFormat = "ply" | "xyz" | "csv";
  let format: CloudFormat = "ply";
  let plyAscii = true;
  let exporting = false;
  let exportMessage = "";

  function stemName(): string {
    if (!sourceFileName) return "pointcloud";
    const dot = sourceFileName.lastIndexOf(".");
    return dot > 0 ? sourceFileName.slice(0, dot) : sourceFileName;
  }

  function applyPreset(which: string) {
    switch (which) {
      case "standard":
        blankLengthMm = 80;
        blankWidthMm = 50;
        blankHeightMm = 50;
        blankMarginMm = 2;
        break;
      case "cube60":
        blankLengthMm = 60;
        blankWidthMm = 60;
        blankHeightMm = 60;
        blankMarginMm = 2;
        break;
      case "large":
        blankLengthMm = 100;
        blankWidthMm = 60;
        blankHeightMm = 60;
        blankMarginMm = 2;
        break;
      case "tall":
        blankLengthMm = 50;
        blankWidthMm = 50;
        blankHeightMm = 80;
        blankMarginMm = 2;
        break;
      default:
        break;
    }
  }

  function onPresetSelect(ev: Event) {
    const t = ev.target as HTMLSelectElement;
    applyPreset(t.value);
  }

  async function handleFormatChange() {
    try {
      await setPointCloudFormat(format);
    } catch {
      // Non-critical
    }
  }

  async function handleExport() {
    if (!hasDepth || exporting) return;
    exporting = true;
    exportMessage = "";
    try {
      await setBlankEnvelope({
        lengthMm: blankLengthMm,
        widthMm: blankWidthMm,
        heightMm: blankHeightMm,
        marginMm: blankMarginMm,
      });
      await handleFormatChange();
      await generatePointCloud();
      const ext = format;
      const path = await saveDialog({
        defaultPath: `${stemName()}.${ext}`,
        filters: [{ name: ext.toUpperCase(), extensions: [ext] }],
      });
      if (path == null || typeof path !== "string") {
        return;
      }
      if (format === "ply") {
        await exportPly(path, !plyAscii);
      } else if (format === "xyz") {
        await exportXyz(path);
      } else {
        await exportCsv(path);
      }
      exportMessage = "Saved.";
    } catch (e) {
      exportMessage = String(e);
    } finally {
      exporting = false;
    }
  }
</script>

<div
  class="export-panel flex flex-wrap items-end gap-3"
  role="region"
  aria-label="Crystal blank and point cloud export"
  data-has-depth={hasDepth}
  data-source-file-name={sourceFileName}
  data-target-width-mm={effectiveTargetWidthMm ?? ""}
  data-target-height-mm={effectiveTargetHeightMm ?? ""}
>
  <div class="flex flex-col gap-1">
    <span class="text-xs font-medium text-slate-600" id="preset-label">Blank preset</span>
    <select
      class="rounded border border-slate-300 bg-white text-sm px-2 py-1.5 focus:outline-none focus:ring-2 focus:ring-slate-400"
      aria-labelledby="preset-label"
      on:change={onPresetSelect}
    >
      <option value="">Custom</option>
      <option value="standard">80 × 50 × 50 (standard)</option>
      <option value="cube60">60 cube</option>
      <option value="large">100 × 60 × 60</option>
      <option value="tall">50 × 50 × 80</option>
    </select>
  </div>
  <div class="flex flex-wrap gap-2 items-end" role="group" aria-labelledby="dims-label">
    <span id="dims-label" class="sr-only">Blank dimensions millimetres</span>
    <label class="flex flex-col gap-0.5 text-xs text-slate-600">
      L
      <input
        type="number"
        min="1"
        step="0.1"
        class="w-16 rounded border border-slate-300 px-1.5 py-1 text-sm tabular-nums"
        bind:value={blankLengthMm}
        aria-label="Blank length millimetres"
      />
    </label>
    <label class="flex flex-col gap-0.5 text-xs text-slate-600">
      W
      <input
        type="number"
        min="1"
        step="0.1"
        class="w-16 rounded border border-slate-300 px-1.5 py-1 text-sm tabular-nums"
        bind:value={blankWidthMm}
        aria-label="Blank width millimetres"
      />
    </label>
    <label class="flex flex-col gap-0.5 text-xs text-slate-600">
      H
      <input
        type="number"
        min="1"
        step="0.1"
        class="w-16 rounded border border-slate-300 px-1.5 py-1 text-sm tabular-nums"
        bind:value={blankHeightMm}
        aria-label="Blank height millimetres"
      />
    </label>
    <label class="flex flex-col gap-0.5 text-xs text-slate-600">
      Margin
      <input
        type="number"
        min="0"
        step="0.1"
        class="w-16 rounded border border-slate-300 px-1.5 py-1 text-sm tabular-nums"
        bind:value={blankMarginMm}
        aria-label="Blank margin millimetres"
      />
    </label>
  </div>

  <div class="flex flex-col gap-1">
    <span class="text-xs font-medium text-slate-600" id="fmt-label">Format</span>
    <select
      class="rounded border border-slate-300 bg-white text-sm px-2 py-1.5 focus:outline-none focus:ring-2 focus:ring-slate-400"
      aria-labelledby="fmt-label"
      bind:value={format}
      on:change={handleFormatChange}
    >
      <option value="ply">PLY</option>
      <option value="xyz">XYZ</option>
      <option value="csv">CSV</option>
    </select>
  </div>

  {#if format === "ply"}
    <label class="flex items-center gap-1.5 text-xs text-slate-600 cursor-pointer pb-1">
      <input type="checkbox" bind:checked={plyAscii} aria-label="Use ASCII PLY (unchecked uses binary)" />
      ASCII PLY
    </label>
  {/if}

  <button
    type="button"
    class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded border border-slate-400 bg-slate-700 text-sm text-white hover:bg-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-500 disabled:opacity-50 disabled:cursor-not-allowed"
    disabled={!hasDepth || exporting}
    aria-label={exporting ? "Exporting point cloud" : "Generate and export point cloud"}
    title="Writes PLY, XYZ, or CSV using current depth adjustments"
    on:click={handleExport}
  >
    {exporting ? "Exporting…" : "Export point cloud"}
  </button>

  {#if exportMessage}
    <p
      class="text-xs max-w-[14rem] truncate {exportMessage === 'Saved.' ? 'text-green-700' : 'text-red-600'}"
      role={exportMessage === "Saved." ? "status" : "alert"}
      title={exportMessage}
    >
      {exportMessage}
    </p>
  {/if}
</div>
