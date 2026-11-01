<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<script lang="ts">
  /**
   * VolumetricParamsPanel — UI-C-02 (ADR-012).
   * XY sampling step, depth threshold, and live point count estimate.
   */
  import { onDestroy } from "svelte";
  import {
    setBlankEnvelope,
    setVolumetricParams,
    estimatePointCloudCount,
  } from "$lib/tauri";

  export let hasDepth = false;
  export let blankLengthMm = 50;
  export let blankWidthMm = 50;
  export let blankHeightMm = 80;
  export let blankMarginMm = 2;

  /** Pixel stride for X and Y sampling (1 = every pixel). */
  export let stepX = 1;
  export let stepY = 1;
  /** Minimum depth [0, 1] required to emit a point. */
  export let depthThreshold = 0.05;

  const STEP_MIN = 1;
  const STEP_MAX = 8;
  const THRESHOLD_STEP = 0.01;
  const DEBOUNCE_MS = 150;

  let pointEstimate: number | null = null;
  let estimateError = "";
  let syncing = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  /** Keep X/Y step aligned (single control in UI). */
  function handleStepInput(e: Event) {
    const v = parseInt((e.target as HTMLInputElement).value, 10);
    if (Number.isNaN(v)) return;
    const clamped = Math.max(STEP_MIN, Math.min(STEP_MAX, v));
    stepX = clamped;
    stepY = clamped;
  }

  async function syncBackendAndEstimate() {
    if (!hasDepth) {
      pointEstimate = null;
      estimateError = "";
      return;
    }
    syncing = true;
    try {
      await setBlankEnvelope({
        lengthMm: blankLengthMm,
        widthMm: blankWidthMm,
        heightMm: blankHeightMm,
        marginMm: blankMarginMm,
      });
      await setVolumetricParams({ stepX, stepY, depthThreshold });
      pointEstimate = await estimatePointCloudCount();
      estimateError = "";
    } catch (e) {
      pointEstimate = null;
      estimateError = String(e);
    } finally {
      syncing = false;
    }
  }

  function scheduleSync() {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      debounceTimer = null;
      void syncBackendAndEstimate();
    }, DEBOUNCE_MS);
  }

  $: {
    hasDepth;
    stepX;
    stepY;
    depthThreshold;
    blankLengthMm;
    blankWidthMm;
    blankHeightMm;
    blankMarginMm;
    scheduleSync();
  }

  onDestroy(() => {
    if (debounceTimer) clearTimeout(debounceTimer);
  });

  $: stepDisplay = stepX;
  $: estimateLabel =
    !hasDepth
      ? "Load a depth map to estimate points"
      : syncing
        ? "Estimating…"
        : estimateError
          ? estimateError
          : pointEstimate != null
            ? `~${pointEstimate.toLocaleString()} points`
            : "—";
</script>

<div
  class="volumetric-params flex flex-wrap items-end gap-4"
  role="region"
  aria-label="Point cloud sampling parameters"
  aria-disabled={!hasDepth}
>
  <label class="flex flex-col gap-1 min-w-[10rem]">
    <span class="text-xs font-medium text-slate-600">
      Pixel step
      <span class="font-normal text-slate-500">({stepDisplay} px)</span>
    </span>
    <input
      type="range"
      min={STEP_MIN}
      max={STEP_MAX}
      step="1"
      value={stepDisplay}
      disabled={!hasDepth}
      class="w-full accent-slate-600 disabled:opacity-50"
      aria-label="Pixel sampling step for X and Y"
      aria-valuemin={STEP_MIN}
      aria-valuemax={STEP_MAX}
      aria-valuenow={stepDisplay}
      on:input={handleStepInput}
    />
  </label>

  <label class="flex flex-col gap-1 min-w-[10rem]">
    <span class="text-xs font-medium text-slate-600">
      Depth threshold
      <span class="font-normal text-slate-500">({depthThreshold.toFixed(2)})</span>
    </span>
    <input
      type="range"
      min="0"
      max="1"
      step={THRESHOLD_STEP}
      bind:value={depthThreshold}
      disabled={!hasDepth}
      class="w-full accent-slate-600 disabled:opacity-50"
      aria-label="Minimum depth value to emit a point"
      aria-valuemin={0}
      aria-valuemax={1}
      aria-valuenow={depthThreshold}
    />
  </label>

  <div class="flex flex-col gap-0.5 pb-0.5 min-w-[8rem]">
    <span class="text-xs font-medium text-slate-600">Point estimate</span>
    <p
      class="text-sm tabular-nums {estimateError ? 'text-red-600' : 'text-slate-700'}"
      role="status"
      aria-live="polite"
      title={estimateLabel}
    >
      {estimateLabel}
    </p>
  </div>
</div>
