<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<script lang="ts">
  /**
   * DepthMapPreview — displays depth map as grayscale image (JR1-301, UI-301/302).
   * Zoom/pan: mouse wheel zoom, drag to pan (JR1-302).
   * Fit-to-view: when a new depth map loads or "Fit" is used, scale to fit the fixed panel (right sidebar).
   * UI-1202/1203: mask overlay and brush/eraser painting when activeMaskTool is set.
   */
  import { onMount, afterUpdate } from "svelte";
  import { renderDepthToCanvas } from "$lib/depthCanvas";
  import type { MaskData } from "$lib/tauri";

  export let width = 0;
  export let height = 0;
  export let depth: number[] = [];
  /** When true, show loading skeleton (JR1-304; estimating state from UI-304). */
  export let estimating = false;
  /** When true, an image is loaded but depth not yet generated — show hint to click Generate. */
  export let hasImage = false;

  /** UI-1203: mask for overlay (semi-transparent where true). */
  export let maskData: MaskData | null = null;
  /** UI-1203: whether to draw the mask overlay. */
  export let showMaskOverlay = true;
  /** UI-1202: when "brush" or "eraser", pointer events paint/erase mask; when null, pan as usual. */
  export let activeMaskTool: "brush" | "eraser" | null = null;
  /** UI-1204: brush/eraser radius in depth-map pixels. */
  export let brushSize = 20;
  /** Called with depth-map pixel (x, y) and value (true = paint, false = erase). */
  export let onMaskPaint: (x: number, y: number, value: boolean) => void = () => {};

  let canvas: HTMLCanvasElement;
  let maskCanvas: HTMLCanvasElement;
  let container: HTMLDivElement;
  let zoomPanDiv: HTMLDivElement;
  let isPainting = false;

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

  /** UI-1203: draw mask overlay on maskCanvas (semi-transparent where mask is set). */
  function drawMaskOverlay() {
    if (!maskCanvas || !maskData || maskData.width !== width || maskData.height !== height) return;
    const ctx = maskCanvas.getContext("2d");
    if (!ctx) return;
    maskCanvas.width = width;
    maskCanvas.height = height;
    ctx.clearRect(0, 0, width, height);
    const imageData = ctx.createImageData(width, height);
    for (let i = 0; i < maskData.data.length; i++) {
      if (maskData.data[i]) {
        const j = i * 4;
        imageData.data[j] = 0;
        imageData.data[j + 1] = 100;
        imageData.data[j + 2] = 255;
        imageData.data[j + 3] = 90;
      }
    }
    ctx.putImageData(imageData, 0, 0);
  }

  /** Map client coords to depth-map pixel. Returns [px, py] or null if out of bounds. */
  function clientToDepth(clientX: number, clientY: number): [number, number] | null {
    if (!zoomPanDiv || width <= 0 || height <= 0) return null;
    const rect = zoomPanDiv.getBoundingClientRect();
    const x = ((clientX - rect.left) / rect.width) * width;
    const y = ((clientY - rect.top) / rect.height) * height;
    if (x < 0 || x >= width || y < 0 || y >= height) return null;
    return [Math.floor(x), Math.floor(y)];
  }

  function doPaint(clientX: number, clientY: number) {
    const pt = clientToDepth(clientX, clientY);
    if (!pt) return;
    const value = activeMaskTool === "brush";
    onMaskPaint(pt[0], pt[1], value);
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

  /** Re-fit when container is resized so depth map stays fitted. */
  let resizeObserver: ResizeObserver | null = null;

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    const delta = e.deltaY > 0 ? -0.1 : 0.1;
    const newZoom = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoom + delta));
    zoom = newZoom;
  }

  function handleMouseDown(e: MouseEvent) {
    if (e.button !== 0) return;
    if (activeMaskTool === "brush" || activeMaskTool === "eraser") {
      e.preventDefault();
      isPainting = true;
      doPaint(e.clientX, e.clientY);
      return;
    }
    isDragging = true;
    dragStartX = e.clientX;
    dragStartY = e.clientY;
    panStartX = panX;
    panStartY = panY;
  }

  function handleMouseMove(e: MouseEvent) {
    if (isPainting) {
      doPaint(e.clientX, e.clientY);
      return;
    }
    if (!isDragging) return;
    panX = panStartX + (e.clientX - dragStartX);
    panY = panStartY + (e.clientY - dragStartY);
  }

  function handleMouseUp() {
    isPainting = false;
    isDragging = false;
  }

  function handleMouseLeave() {
    isPainting = false;
    isDragging = false;
  }

  $: transformStyle = `transform: scale(${zoom}) translate(${panX}px, ${panY}px);`;

  afterUpdate(() => {
    draw();
    if (maskData && showMaskOverlay) drawMaskOverlay();
  });
  onMount(() => {
    draw();
    if (width > 0 && height > 0 && depth.length > 0) scheduleFitToView();
    const ro = new ResizeObserver(() => {
      if (width > 0 && height > 0 && depth.length > 0) scheduleFitToView();
    });
    const el = container;
    if (el) ro.observe(el);
    return () => ro.disconnect();
  });
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex a11y-no-noninteractive-element-interactions -->
<div
  class="depth-preview-wrapper w-full h-full min-h-[200px] flex items-center justify-center bg-slate-100 rounded overflow-hidden select-none {activeMaskTool === 'brush' || activeMaskTool === 'eraser' ? 'cursor-crosshair' : 'cursor-grab'}"
  class:cursor-grabbing={isDragging && !isPainting}
  bind:this={container}
  style="contain: layout;"
  role="application"
  aria-label="Depth map preview: scroll to zoom, drag to pan"
  tabindex="0"
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
    <div
      bind:this={zoomPanDiv}
      class="depth-zoom-pan"
      style="width: {width}px; height: {height}px; {transformStyle}"
      aria-hidden="true"
    >
      <canvas
        bind:this={canvas}
        class="depth-canvas"
        style="image-rendering: pixelated; image-rendering: crisp-edges; display: block;"
        aria-label="Depth map (grayscale: near dark, far light)"
      />
      {#if maskData && showMaskOverlay}
        <canvas
          bind:this={maskCanvas}
          class="mask-overlay-canvas"
          style="pointer-events: none; image-rendering: pixelated;"
          aria-hidden="true"
        />
      {/if}
    </div>
    <button
      type="button"
      class="depth-fit-btn"
      title="Fit depth map to view"
      on:click={() => applyFitToView()}
      on:mousedown|stopPropagation
    >Fit</button>
  {:else if hasImage}
    <p class="text-slate-500 text-sm text-center px-2">Image loaded. Click <strong>Generate Depth Map</strong> below to create the depth map.</p>
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
    position: absolute;
    top: 0;
    left: 0;
    transform-origin: 0 0;
    will-change: transform;
  }
  .depth-canvas {
    display: block;
  }
  .mask-overlay-canvas {
    position: absolute;
    top: 0;
    left: 0;
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
