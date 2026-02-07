<script lang="ts">
  /**
   * DepthMapPreview — displays depth map as grayscale image (JR1-301, UI-301/302).
   * Zoom/pan: mouse wheel zoom, drag to pan (JR1-302).
   * Fit-to-view: when a new depth map loads or "Fit" is used, scale to fit the fixed panel (right sidebar).
   */
  import { onMount, afterUpdate } from "svelte";
  import { renderDepthToCanvas } from "$lib/depthCanvas";

  export let width = 0;
  export let height = 0;
  export let depth: number[] = [];
  /** When true, show loading skeleton (JR1-304; estimating state from UI-304). */
  export let estimating = false;

  let canvas: HTMLCanvasElement;
  let container: HTMLDivElement;

  /** Allow zoom out enough to fit large images in the fixed w-64 sidebar (~256px). e.g. 4K width needs ~0.067. */
  const MIN_ZOOM = 0.02;
  const MAX_ZOOM = 10;
  let zoom = 1;
  let panX = 0;
  let panY = 0;
  let isDragging = false;
  let dragStartX = 0;
  let dragStartY = 0;
  let panStartX = 0;
  let panStartY = 0;

  function draw() {
    if (!canvas || width <= 0 || height <= 0 || depth.length === 0) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    canvas.width = width;
    canvas.height = height;
    renderDepthToCanvas(ctx, width, height, depth);
  }

  /** Set zoom and pan so the depth map fits inside the container and is centered. */
  function applyFitToView() {
    if (!container || width <= 0 || height <= 0) return;
    const cw = container.clientWidth;
    const ch = container.clientHeight;
    if (cw <= 0 || ch <= 0) return;
    const fitZoom = Math.min(cw / width, ch / height);
    zoom = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, fitZoom));
    panX = (cw - width * zoom) / 2;
    panY = (ch - height * zoom) / 2;
  }

  function scheduleFitToView() {
    requestAnimationFrame(() => applyFitToView());
  }

  /** Run fit-to-view when a new depth map is set (dimensions change), so large images fit in the fixed panel. */
  let prevFitKey = "";
  $: fitKey = width > 0 && height > 0 && depth.length > 0 ? `${width}-${height}` : "";
  $: if (fitKey && fitKey !== prevFitKey) {
    prevFitKey = fitKey;
    scheduleFitToView();
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    const delta = e.deltaY > 0 ? -0.1 : 0.1;
    const newZoom = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoom + delta));
    zoom = newZoom;
  }

  function handleMouseDown(e: MouseEvent) {
    if (e.button !== 0) return;
    isDragging = true;
    dragStartX = e.clientX;
    dragStartY = e.clientY;
    panStartX = panX;
    panStartY = panY;
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;
    panX = panStartX + (e.clientX - dragStartX);
    panY = panStartY + (e.clientY - dragStartY);
  }

  function handleMouseUp() {
    isDragging = false;
  }

  function handleMouseLeave() {
    isDragging = false;
  }

  $: transformStyle = `transform: scale(${zoom}) translate(${panX}px, ${panY}px);`;

  afterUpdate(draw);
  onMount(() => {
    draw();
    if (width > 0 && height > 0 && depth.length > 0) scheduleFitToView();
  });
</script>

<div
  class="depth-preview-wrapper w-full h-full min-h-[200px] flex items-center justify-center bg-slate-100 rounded overflow-hidden select-none cursor-grab"
  class:cursor-grabbing={isDragging}
  bind:this={container}
  role="application"
  aria-label="Depth map preview: scroll to zoom, drag to pan"
  on:wheel|preventDefault={handleWheel}
  on:mousedown={handleMouseDown}
  on:mousemove={handleMouseMove}
  on:mouseup={handleMouseUp}
  on:mouseleave={handleMouseLeave}
  aria-busy={estimating}
>
  {#if estimating}
    <div class="depth-skeleton" aria-live="polite" aria-label="Depth estimation in progress">
      <div class="skeleton-bars">
        <div class="skeleton-bar"></div>
        <div class="skeleton-bar"></div>
        <div class="skeleton-bar"></div>
      </div>
      <p class="text-slate-500 text-sm mt-2">Estimating depth…</p>
    </div>
  {:else if width > 0 && height > 0 && depth.length > 0}
    <div class="depth-zoom-pan" style={transformStyle}>
      <canvas
        bind:this={canvas}
        class="depth-canvas"
        style="image-rendering: pixelated; image-rendering: crisp-edges;"
        aria-label="Depth map (grayscale: near dark, far light)"
        role="img"
      />
    </div>
    <button
      type="button"
      class="depth-fit-btn"
      title="Fit depth map to view"
      on:click={() => applyFitToView()}
      on:mousedown|stopPropagation
    >Fit</button>
  {:else}
    <p class="text-slate-500 text-sm">No depth map</p>
  {/if}
</div>

<style>
  .depth-preview-wrapper {
    user-select: none;
    position: relative;
  }
  .depth-preview-wrapper:focus {
    outline: 2px solid var(--tw-ring-color, theme(colors.slate.400));
    outline-offset: 2px;
  }
  .depth-zoom-pan {
    transform-origin: 0 0;
    will-change: transform;
  }
  .depth-canvas {
    display: block;
  }
  .depth-fit-btn {
    position: absolute;
    top: 0.25rem;
    right: 0.25rem;
    padding: 0.2rem 0.5rem;
    font-size: 0.75rem;
    font-weight: 500;
    color: theme(colors.slate.600);
    background: theme(colors.slate.100);
    border: 1px solid theme(colors.slate.300);
    border-radius: 4px;
    cursor: pointer;
    z-index: 1;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }
  .depth-fit-btn:hover {
    background: theme(colors.slate.200);
    color: theme(colors.slate.800);
  }
  .depth-fit-btn:focus {
    outline: 2px solid theme(colors.slate.400);
    outline-offset: 1px;
  }

  /* JR1-304: loading skeleton during generation */
  .depth-skeleton {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 200px;
    padding: 1rem;
  }
  .skeleton-bars {
    display: flex;
    gap: 0.5rem;
    align-items: flex-end;
  }
  .skeleton-bar {
    width: 2rem;
    height: 3rem;
    background: linear-gradient(
      90deg,
      theme(colors.slate.200) 25%,
      theme(colors.slate.300) 50%,
      theme(colors.slate.200) 75%
    );
    background-size: 200% 100%;
    animation: skeleton-shimmer 1.2s ease-in-out infinite;
    border-radius: 4px;
  }
  .skeleton-bar:nth-child(2) {
    height: 4rem;
    animation-delay: 0.1s;
  }
  .skeleton-bar:nth-child(3) {
    height: 2.5rem;
    animation-delay: 0.2s;
  }
  @keyframes skeleton-shimmer {
    0% {
      background-position: 200% 0;
    }
    100% {
      background-position: -200% 0;
    }
  }
</style>
