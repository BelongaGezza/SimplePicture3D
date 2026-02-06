<script lang="ts">
  /**
   * DepthControls — UI-401–405. Sliders and controls for depth adjustment.
   * Depth Range (min/max mm), Brightness, Gamma, Invert, Reset.
   * Disabled when no depth map; parent debounces param changes for preview (UI-404).
   */
  import Button from "./Button.svelte";
  import type { DepthAdjustmentParams } from "$lib/tauri";

  export let hasDepth = false;
  export let params: DepthAdjustmentParams = {
    brightness: 0,
    contrast: 1,
    gamma: 1,
    invert: false,
    depthMinMm: 2,
    depthMaxMm: 10,
  };

  export let onParamsChange: (p: DepthAdjustmentParams) => void = () => {};
  export let onReset: () => void = () => {};

  const DEPTH_MM_MIN = 1;
  const DEPTH_MM_MAX = 20;
  const BRIGHTNESS_MIN = -0.5;
  const BRIGHTNESS_MAX = 0.5;
  const GAMMA_MIN = 0.5;
  const GAMMA_MAX = 2;
  const SLIDER_STEP = 0.01;
  const GAMMA_STEP = 0.05;
  const DEPTH_MM_STEP = 0.5;

  function emitChange(partial: Partial<DepthAdjustmentParams>) {
    const next = { ...params, ...partial };
    if (next.depthMaxMm < next.depthMinMm) next.depthMaxMm = next.depthMinMm;
    if (next.depthMinMm > next.depthMaxMm) next.depthMinMm = next.depthMaxMm;
    onParamsChange(next);
  }

  function handleDepthMinInput(e: Event) {
    const v = parseFloat((e.target as HTMLInputElement).value);
    if (!Number.isNaN(v)) emitChange({ depthMinMm: Math.max(DEPTH_MM_MIN, Math.min(DEPTH_MM_MAX, v)) });
  }

  function handleDepthMaxInput(e: Event) {
    const v = parseFloat((e.target as HTMLInputElement).value);
    if (!Number.isNaN(v)) emitChange({ depthMaxMm: Math.max(DEPTH_MM_MIN, Math.min(DEPTH_MM_MAX, v)) });
  }

  function handleBrightnessInput(e: Event) {
    const v = parseFloat((e.target as HTMLInputElement).value);
    if (!Number.isNaN(v)) emitChange({ brightness: Math.max(BRIGHTNESS_MIN, Math.min(BRIGHTNESS_MAX, v)) });
  }

  function handleGammaInput(e: Event) {
    const v = parseFloat((e.target as HTMLInputElement).value);
    if (!Number.isNaN(v)) emitChange({ gamma: Math.max(GAMMA_MIN, Math.min(GAMMA_MAX, v)) });
  }

  function handleInvertChange(e: Event) {
    emitChange({ invert: (e.target as HTMLInputElement).checked });
  }

  function handleReset() {
    onReset();
  }

  /** JR1-403: Arrow keys change slider value by step when range input is focused. */
  function handleRangeKeydown(
    e: KeyboardEvent,
    current: number,
    min: number,
    max: number,
    step: number,
    apply: (v: number) => void
  ) {
    const el = e.currentTarget as HTMLInputElement;
    if (!el || el.type !== "range") return;
    let delta = 0;
    if (e.key === "ArrowLeft" || e.key === "ArrowDown") delta = -step;
    else if (e.key === "ArrowRight" || e.key === "ArrowUp") delta = step;
    else return;
    e.preventDefault();
    const next = Math.max(min, Math.min(max, current + delta));
    const rounded = Math.round(next / step) * step;
    const clamped = Math.max(min, Math.min(max, rounded));
    apply(clamped);
  }
</script>

<div
  class="depth-controls rounded border border-slate-200 bg-slate-50 p-3 flex flex-col gap-3"
  role="group"
  aria-label="Depth adjustment controls"
  aria-disabled={!hasDepth}
>
  <h3 class="text-xs font-semibold text-slate-600 uppercase tracking-wide">Adjust depth</h3>

  {#if !hasDepth}
    <p class="text-slate-500 text-sm">Generate a depth map to adjust.</p>
  {:else}
    <!-- Depth Range (min/max mm) - UI-402 -->
    <div class="flex flex-col gap-1" role="group" aria-label="Depth range in millimeters">
      <label for="depth-min-mm" class="text-xs text-slate-600">Depth min (mm)</label>
      <div class="flex items-center gap-2 min-h-8">
        <input
          id="depth-min-mm"
          type="range"
          min={DEPTH_MM_MIN}
          max={DEPTH_MM_MAX}
          step={DEPTH_MM_STEP}
          value={params.depthMinMm}
          on:input={handleDepthMinInput}
          on:keydown={(e) => handleRangeKeydown(e, params.depthMinMm, DEPTH_MM_MIN, DEPTH_MM_MAX, DEPTH_MM_STEP, (v) => emitChange({ depthMinMm: v }))}
          class="depth-slider flex-1 min-w-0 h-2 rounded-full appearance-none bg-slate-200 accent-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-1 cursor-grab active:cursor-grabbing"
          aria-valuemin={DEPTH_MM_MIN}
          aria-valuemax={DEPTH_MM_MAX}
          aria-valuenow={params.depthMinMm}
        />
        <input
          type="number"
          min={DEPTH_MM_MIN}
          max={DEPTH_MM_MAX}
          step={DEPTH_MM_STEP}
          value={params.depthMinMm}
          on:input={handleDepthMinInput}
          class="w-14 text-right text-sm border border-slate-300 rounded px-1.5 py-0.5 bg-white"
          aria-label="Depth min value"
        />
      </div>
    </div>
    <div class="flex flex-col gap-1" role="group" aria-label="Depth range max">
      <label for="depth-max-mm" class="text-xs text-slate-600">Depth max (mm)</label>
      <div class="flex items-center gap-2 min-h-8">
        <input
          id="depth-max-mm"
          type="range"
          min={DEPTH_MM_MIN}
          max={DEPTH_MM_MAX}
          step={DEPTH_MM_STEP}
          value={params.depthMaxMm}
          on:input={handleDepthMaxInput}
          on:keydown={(e) => handleRangeKeydown(e, params.depthMaxMm, DEPTH_MM_MIN, DEPTH_MM_MAX, DEPTH_MM_STEP, (v) => emitChange({ depthMaxMm: v }))}
          class="depth-slider flex-1 min-w-0 h-2 rounded-full appearance-none bg-slate-200 accent-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-1 cursor-grab active:cursor-grabbing"
          aria-valuemin={DEPTH_MM_MIN}
          aria-valuemax={DEPTH_MM_MAX}
          aria-valuenow={params.depthMaxMm}
        />
        <input
          type="number"
          min={DEPTH_MM_MIN}
          max={DEPTH_MM_MAX}
          step={DEPTH_MM_STEP}
          value={params.depthMaxMm}
          on:input={handleDepthMaxInput}
          class="w-14 text-right text-sm border border-slate-300 rounded px-1.5 py-0.5 bg-white"
          aria-label="Depth max value"
        />
      </div>
    </div>

    <!-- Brightness - UI-402 -->
    <div class="flex flex-col gap-1" role="group" aria-label="Brightness">
      <label for="brightness-slider" class="text-xs text-slate-600">Brightness</label>
      <div class="flex items-center gap-2 min-h-8">
        <input
          id="brightness-slider"
          type="range"
          min={BRIGHTNESS_MIN}
          max={BRIGHTNESS_MAX}
          step={SLIDER_STEP}
          value={params.brightness}
          on:input={handleBrightnessInput}
          on:keydown={(e) => handleRangeKeydown(e, params.brightness, BRIGHTNESS_MIN, BRIGHTNESS_MAX, SLIDER_STEP, (v) => emitChange({ brightness: v }))}
          class="depth-slider flex-1 min-w-0 h-2 rounded-full appearance-none bg-slate-200 accent-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-1 cursor-grab active:cursor-grabbing"
          aria-valuemin={BRIGHTNESS_MIN}
          aria-valuemax={BRIGHTNESS_MAX}
          aria-valuenow={params.brightness}
        />
        <input
          type="number"
          min={BRIGHTNESS_MIN}
          max={BRIGHTNESS_MAX}
          step={SLIDER_STEP}
          value={params.brightness}
          on:input={handleBrightnessInput}
          class="w-14 text-right text-sm border border-slate-300 rounded px-1.5 py-0.5 bg-white"
          aria-label="Brightness value"
        />
      </div>
    </div>

    <!-- Gamma - UI-402 -->
    <div class="flex flex-col gap-1" role="group" aria-label="Gamma">
      <label for="gamma-slider" class="text-xs text-slate-600">Gamma</label>
      <div class="flex items-center gap-2 min-h-8">
        <input
          id="gamma-slider"
          type="range"
          min={GAMMA_MIN}
          max={GAMMA_MAX}
          step={GAMMA_STEP}
          value={params.gamma}
          on:input={handleGammaInput}
          on:keydown={(e) => handleRangeKeydown(e, params.gamma, GAMMA_MIN, GAMMA_MAX, GAMMA_STEP, (v) => emitChange({ gamma: v }))}
          class="depth-slider flex-1 min-w-0 h-2 rounded-full appearance-none bg-slate-200 accent-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-1 cursor-grab active:cursor-grabbing"
          aria-valuemin={GAMMA_MIN}
          aria-valuemax={GAMMA_MAX}
          aria-valuenow={params.gamma}
        />
        <input
          type="number"
          min={GAMMA_MIN}
          max={GAMMA_MAX}
          step={GAMMA_STEP}
          value={params.gamma}
          on:input={handleGammaInput}
          class="w-14 text-right text-sm border border-slate-300 rounded px-1.5 py-0.5 bg-white"
          aria-label="Gamma value"
        />
      </div>
    </div>

    <!-- Invert - UI-403 -->
    <div class="flex items-center gap-2" role="group" aria-label="Invert depth">
      <input
        id="invert-depth"
        type="checkbox"
        checked={params.invert}
        on:change={handleInvertChange}
        class="h-4 w-4 rounded border-slate-300 text-slate-600 focus:ring-slate-400"
        aria-label="Invert depth (near and far swapped)"
      />
      <label for="invert-depth" class="text-sm text-slate-700 select-none cursor-pointer">Invert depth</label>
    </div>

    <!-- Reset - UI-405 -->
    <Button
      variant="secondary"
      title="Restore original depth map (reset all adjustments)"
      on:click={handleReset}
    >
      Reset
    </Button>
  {/if}
</div>

<style>
  /* JR1-401: Slider track and thumb — consistent with app theme; usable at 1024×768 (min touch target) */
  .depth-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: theme("colors.slate.600");
    cursor: pointer;
    border: 0;
  }
  .depth-slider::-moz-range-thumb {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: theme("colors.slate.600");
    cursor: pointer;
    border: 0;
  }
  .depth-slider::-webkit-slider-runnable-track {
    height: 8px;
    border-radius: 4px;
    background: theme("colors.slate.200");
  }
  .depth-slider::-moz-range-track {
    height: 8px;
    border-radius: 4px;
    background: theme("colors.slate.200");
  }
</style>
