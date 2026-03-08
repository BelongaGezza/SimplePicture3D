<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<!--
  MaskingTools — UI-1201–1204, JR1-1202, JR1-1203.
  Brush/eraser/select tools; brush size and hardness (UI-1201, UI-1204).
  Save/Load mask (JR1-1203); Clear mask; Apply selection (rectangle or lasso) (JR1-1202).
-->
<script lang="ts">
  import Button from "./Button.svelte";
  import { save as saveDialog, open as openDialog } from "@tauri-apps/plugin-dialog";
  import {
    getMask,
    setMask,
    setMaskRegion,
    clearMask,
    saveMaskToPath,
    loadMaskFromPath,
  } from "$lib/tauri";
  import { polygonToMaskData, clampRectangle } from "$lib/selection";
  import type { Rectangle, Point } from "$lib/selection";

  export let hasDepth = false;
  /** Current depth map dimensions (for apply selection). */
  export let depthWidth = 0;
  export let depthHeight = 0;

  /** UI-1201: current tool (brush, eraser, select). */
  export let tool: "brush" | "eraser" | "select" = "brush";
  export let onToolChange: (t: "brush" | "eraser" | "select") => void = () => {};
  /** UI-1204: brush size in pixels. */
  export let brushSize = 20;
  export let onBrushSizeChange: (v: number) => void = () => {};
  export let brushHardness = 1;
  export let onBrushHardnessChange: (v: number) => void = () => {};
  /** When provided, Clear mask uses this (parent refreshes mask + undo state). */
  export let onClearMask: (() => void | Promise<void>) | null = null;

  /** Optional rectangle selection (from parent/canvas). Apply to mask when user clicks "Apply selection". */
  export let rectangleSelection: Rectangle | null = null;
  /** Optional lasso (polygon) selection. Apply to mask when user clicks "Apply selection". */
  export let lassoSelection: Point[] | null = null;

  /** Called after mask mutation (e.g. for undo state refresh). */
  export let onMaskChange: () => void = () => {};

  let saving = false;
  let loading = false;
  let clearing = false;
  let applying = false;
  let maskError = "";

  const hasSelection =
    (rectangleSelection != null && rectangleSelection.width > 0 && rectangleSelection.height > 0) ||
    (lassoSelection != null && lassoSelection.length >= 3);

  async function handleSaveMask() {
    if (!hasDepth || saving) return;
    saving = true;
    maskError = "";
    try {
      const path = await saveDialog({
        defaultPath: "mask.json",
        filters: [{ name: "Mask JSON", extensions: ["json"] }],
      });
      if (path) {
        await saveMaskToPath(path);
        onMaskChange();
      }
    } catch (e) {
      maskError = String(e);
    } finally {
      saving = false;
    }
  }

  async function handleLoadMask() {
    if (!hasDepth || loading) return;
    loading = true;
    maskError = "";
    try {
      const path = await openDialog({
        filters: [{ name: "Mask JSON", extensions: ["json"] }],
      });
      if (path && typeof path === "string") {
        await loadMaskFromPath(path);
        onMaskChange();
      }
    } catch (e) {
      maskError = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleClearMask() {
    if (!hasDepth || clearing) return;
    clearing = true;
    maskError = "";
    try {
      if (onClearMask) {
        await onClearMask();
      } else {
        await clearMask();
        onMaskChange();
      }
    } catch (e) {
      maskError = String(e);
    } finally {
      clearing = false;
    }
  }

  function handleBrushSizeInput(e: Event) {
    const v = parseInt((e.target as HTMLInputElement).value, 10);
    if (!Number.isNaN(v)) onBrushSizeChange(v);
  }

  function handleHardnessInput(e: Event) {
    const v = parseFloat((e.target as HTMLInputElement).value);
    if (!Number.isNaN(v)) onBrushHardnessChange(v);
  }

  /** JR1-1202: Apply current selection (rectangle or lasso) to mask. */
  async function handleApplySelection() {
    if (!hasDepth || !hasSelection || applying) return;
    applying = true;
    maskError = "";
    try {
      if (
        rectangleSelection &&
        rectangleSelection.width > 0 &&
        rectangleSelection.height > 0
      ) {
        const clamped = clampRectangle(
          rectangleSelection,
          depthWidth,
          depthHeight
        );
        if (clamped) {
          await setMaskRegion(
            Math.floor(clamped.x),
            Math.floor(clamped.y),
            Math.max(1, Math.floor(clamped.width)),
            Math.max(1, Math.floor(clamped.height)),
            true
          );
          onMaskChange();
        }
      } else if (lassoSelection && lassoSelection.length >= 3) {
        const current = await getMask();
        if (!current || current.width !== depthWidth || current.height !== depthHeight) {
          maskError = "No depth dimensions";
          return;
        }
        const selectionMask = polygonToMaskData(
          depthWidth,
          depthHeight,
          lassoSelection
        );
        const merged = current.data.map((v, i) => v || selectionMask[i]);
        await setMask(depthWidth, depthHeight, merged);
        onMaskChange();
      }
    } catch (e) {
      maskError = String(e);
    } finally {
      applying = false;
    }
  }
</script>

<div class="masking-tools space-y-2">
  <div class="text-sm font-medium text-gray-700 dark:text-gray-300">Mask</div>
  <!-- UI-1201: Tool selector (brush, eraser, select) -->
  <div class="flex gap-1" role="group" aria-label="Mask tool">
    <button
      type="button"
      class="px-2 py-1 text-sm rounded border {tool === 'brush' ? 'border-slate-500 bg-slate-100' : 'border-slate-300 bg-white'} text-slate-700 disabled:opacity-50"
      title="Brush: paint mask"
      disabled={!hasDepth}
      on:click={() => onToolChange('brush')}
      aria-pressed={tool === 'brush'}
    >Brush</button>
    <button
      type="button"
      class="px-2 py-1 text-sm rounded border {tool === 'eraser' ? 'border-slate-500 bg-slate-100' : 'border-slate-300 bg-white'} text-slate-700 disabled:opacity-50"
      title="Eraser: clear mask"
      disabled={!hasDepth}
      on:click={() => onToolChange('eraser')}
      aria-pressed={tool === 'eraser'}
    >Eraser</button>
    <button
      type="button"
      class="px-2 py-1 text-sm rounded border {tool === 'select' ? 'border-slate-500 bg-slate-100' : 'border-slate-300 bg-white'} text-slate-700 disabled:opacity-50"
      title="Select: rectangle or lasso"
      disabled={!hasDepth}
      on:click={() => onToolChange('select')}
      aria-pressed={tool === 'select'}
    >Select</button>
  </div>
  <!-- UI-1204: Brush size and hardness -->
  <div class="grid grid-cols-2 gap-2 text-xs">
    <label class="flex flex-col gap-0.5">
      <span>Brush size</span>
      <input
        type="range"
        min="1"
        max="100"
        value={brushSize}
        on:input={handleBrushSizeInput}
        class="w-full"
      />
    </label>
    <label class="flex flex-col gap-0.5">
      <span>Hardness</span>
      <input
        type="range"
        min="0"
        max="1"
        step="0.05"
        value={brushHardness}
        on:input={handleHardnessInput}
        class="w-full"
      />
    </label>
  </div>
  <div class="flex flex-wrap gap-2">
    <Button
      on:click={handleSaveMask}
      disabled={!hasDepth || saving}
      title="Save mask to JSON file"
    >
      {saving ? "Saving…" : "Save mask"}
    </Button>
    <Button
      on:click={handleLoadMask}
      disabled={!hasDepth || loading}
      title="Load mask from JSON (dimensions must match)"
    >
      {loading ? "Loading…" : "Load mask"}
    </Button>
    <Button
      on:click={handleClearMask}
      disabled={!hasDepth || clearing}
      title="Clear mask to empty"
    >
      {clearing ? "Clearing…" : "Clear mask"}
    </Button>
    {#if hasSelection}
      <Button
        on:click={handleApplySelection}
        disabled={applying}
        title="Apply selection to mask"
      >
        {applying ? "Applying…" : "Apply selection"}
      </Button>
    {/if}
  </div>
  {#if maskError}
    <p class="text-xs text-red-600 dark:text-red-400">{maskError}</p>
  {/if}
</div>
