<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<!-- UI-1102, UI-1103, UI-1104, JR1-1102, JR1-1103: Photoshop-style curve with presets and reset (BACK-1102, BACK-1103). -->
<script lang="ts">
  import type { CurvePoint, DepthAdjustmentParams } from "$lib/tauri";

  export let params: DepthAdjustmentParams;
  export let onParamsChange: (p: DepthAdjustmentParams) => void = () => {};

  const CANVAS_SIZE = 160;
  const PAD = 12;
  const GRID = CANVAS_SIZE - 2 * PAD;

  /** Presets matching backend (BACK-1102). */
  const PRESETS: Record<string, CurvePoint[]> = {
    linear: [
      { x: 0, y: 0 },
      { x: 1, y: 1 },
    ],
    "s-curve": [
      { x: 0, y: 0 },
      { x: 0.25, y: 0.12 },
      { x: 0.5, y: 0.5 },
      { x: 0.75, y: 0.88 },
      { x: 1, y: 1 },
    ],
    exponential: [
      { x: 0, y: 0 },
      { x: 0.5, y: 0.32 },
      { x: 1, y: 1 },
    ],
  };

  $: points = params.curveControlPoints && params.curveControlPoints.length >= 2
    ? [...params.curveControlPoints].sort((a, b) => a.x - b.x)
    : PRESETS.linear;

  let canvasEl: HTMLCanvasElement;
  let dragging: number | null = null;

  function toScreen(p: CurvePoint): { x: number; y: number } {
    return {
      x: PAD + p.x * GRID,
      y: PAD + (1 - p.y) * GRID,
    };
  }

  function toCurve(sx: number, sy: number): CurvePoint {
    const x = (sx - PAD) / GRID;
    const y = 1 - (sy - PAD) / GRID;
    return { x: Math.max(0, Math.min(1, x)), y: Math.max(0, Math.min(1, y)) };
  }

  /** JR1-1102: Catmull-Rom to cubic Bezier; returns control points for segment from pts[i] to pts[i+1]. */
  function getBezierSegment(pts: CurvePoint[], i: number): { c1: CurvePoint; c2: CurvePoint } {
    const p0 = pts[Math.max(0, i - 1)];
    const p1 = pts[i];
    const p2 = pts[i + 1];
    const p3 = pts[Math.min(pts.length - 1, i + 2)];
    const c1: CurvePoint = {
      x: p1.x + (p2.x - p0.x) / 6,
      y: p1.y + (p2.y - p0.y) / 6,
    };
    const c2: CurvePoint = {
      x: p2.x - (p3.x - p1.x) / 6,
      y: p2.y - (p3.y - p1.y) / 6,
    };
    return { c1, c2 };
  }

  function drawCurve(ctx: CanvasRenderingContext2D) {
    ctx.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
    ctx.fillStyle = "#f8fafc";
    ctx.fillRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
    ctx.strokeStyle = "#cbd5e1";
    ctx.lineWidth = 1;
    for (let i = 0.25; i < 1; i += 0.25) {
      const x = PAD + i * GRID;
      ctx.beginPath();
      ctx.moveTo(x, PAD);
      ctx.lineTo(x, PAD + GRID);
      ctx.stroke();
      const y = PAD + i * GRID;
      ctx.beginPath();
      ctx.moveTo(PAD, y);
      ctx.lineTo(PAD + GRID, y);
      ctx.stroke();
    }
    ctx.strokeStyle = "#334155";
    ctx.lineWidth = 2;
    if (points.length >= 2) {
      ctx.beginPath();
      const s0 = toScreen(points[0]);
      ctx.moveTo(s0.x, s0.y);
      for (let i = 0; i < points.length - 1; i++) {
        const { c1, c2 } = getBezierSegment(points, i);
        const s1 = toScreen(points[i + 1]);
        const sc1 = toScreen(c1);
        const sc2 = toScreen(c2);
        ctx.bezierCurveTo(sc1.x, sc1.y, sc2.x, sc2.y, s1.x, s1.y);
      }
      ctx.stroke();
    }
    ctx.fillStyle = "#475569";
    for (const p of points) {
      const s = toScreen(p);
      ctx.beginPath();
      ctx.arc(s.x, s.y, 5, 0, Math.PI * 2);
      ctx.fill();
      ctx.strokeStyle = "#1e293b";
      ctx.lineWidth = 1;
      ctx.stroke();
    }
  }

  $: if (canvasEl) {
    void points; // react to params/points changes
    const ctx = canvasEl.getContext("2d");
    if (ctx) drawCurve(ctx);
  }

  function handlePointerDown(e: MouseEvent) {
    const rect = canvasEl.getBoundingClientRect();
    const sx = e.clientX - rect.left;
    const sy = e.clientY - rect.top;
    for (let i = 0; i < points.length; i++) {
      const s = toScreen(points[i]);
      if (Math.hypot(sx - s.x, sy - s.y) < 10) {
        dragging = i;
        return;
      }
    }
  }

  function handlePointerMove(e: MouseEvent) {
    if (dragging === null) return;
    const rect = canvasEl.getBoundingClientRect();
    const sx = e.clientX - rect.left;
    const sy = e.clientY - rect.top;
    const newPt = toCurve(sx, sy);
    const next = [...points];
    next[dragging] = newPt;
    next.sort((a, b) => a.x - b.x);
    const newIdx = next.findIndex((q) => q === newPt);
    onParamsChange({ ...params, curveControlPoints: next });
    if (newIdx !== dragging) dragging = newIdx;
  }

  function handlePointerUp() {
    dragging = null;
  }

  function applyPreset(name: string) {
    const preset = PRESETS[name];
    if (preset) onParamsChange({ ...params, curveControlPoints: preset });
  }

  function resetCurve() {
    onParamsChange({ ...params, curveControlPoints: PRESETS.linear });
  }

  function handlePresetChange(e: Event) {
    const value = (e.target as HTMLSelectElement).value;
    applyPreset(value);
  }
</script>

<div class="curves-tool rounded border border-slate-200 bg-slate-50 p-3 flex flex-col gap-2" role="group" aria-label="Curves tool">
  <label class="text-xs text-slate-600">Curve</label>
  <div class="flex items-center gap-2 flex-wrap">
    <select
      class="text-xs border border-slate-300 rounded px-2 py-1 bg-white"
      aria-label="Curve preset"
      on:change={handlePresetChange}
    >
      <option value="linear">Linear</option>
      <option value="s-curve">S-curve</option>
      <option value="exponential">Exponential</option>
    </select>
    <button
      type="button"
      class="text-xs text-slate-600 hover:text-slate-800 underline"
      on:click={resetCurve}
    >
      Reset curve
    </button>
  </div>
  <canvas
    bind:this={canvasEl}
    width={CANVAS_SIZE}
    height={CANVAS_SIZE}
    class="rounded border border-slate-200 cursor-crosshair block touch-none"
    role="img"
    aria-label="Curve graph: drag points to adjust"
    on:pointerdown={handlePointerDown}
    on:pointermove={handlePointerMove}
    on:pointerup={handlePointerUp}
    on:pointerleave={handlePointerUp}
  />
</div>
