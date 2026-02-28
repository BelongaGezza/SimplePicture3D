<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<!-- UI-1101, JR1-1101: Depth distribution histogram (BACK-1101). Renders with Canvas API. -->
<script lang="ts">
  export let histogram: number[] | null = null;
  export let width = 200;
  export let height = 60;

  let canvasEl: HTMLCanvasElement;

  $: if (canvasEl && histogram && histogram.length > 0) {
    const ctx = canvasEl.getContext("2d");
    if (ctx) {
      const dpr = window.devicePixelRatio ?? 1;
      const w = width * dpr;
      const h = height * dpr;
      canvasEl.width = w;
      canvasEl.height = h;
      canvasEl.style.width = `${width}px`;
      canvasEl.style.height = `${height}px`;
      ctx.scale(dpr, dpr);
      const max = Math.max(1, ...histogram);
      const barW = width / histogram.length;
      ctx.fillStyle = "#e2e8f0";
      ctx.fillRect(0, 0, width, height);
      ctx.fillStyle = "#475569";
      for (let i = 0; i < histogram.length; i++) {
        const barH = (histogram[i] / max) * (height - 2);
        ctx.fillRect(i * barW, height - 1 - barH, Math.max(1, barW - 0.5), barH);
      }
    }
  }
</script>

<div class="histogram-panel" role="group" aria-label="Depth distribution histogram">
  <span class="text-xs text-slate-600 block mb-1">Depth distribution</span>
  {#if histogram && histogram.length > 0}
    <canvas
      bind:this={canvasEl}
      class="rounded border border-slate-200 bg-slate-100 block"
      style="width: {width}px; height: {height}px;"
      aria-hidden="true"
    />
  {:else}
    <p class="text-slate-500 text-xs">No depth data</p>
  {/if}
</div>
