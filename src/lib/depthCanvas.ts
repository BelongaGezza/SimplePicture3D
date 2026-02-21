// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Canvas rendering for depth map (JR1-301).
 * Maps depth values 0–1 to grayscale (0→black, 1→white). Row-major order per architecture.
 */

/**
 * Renders a depth array to a canvas 2D context.
 * Depth values 0–1 are mapped to grayscale (0→black, 1→white).
 * Out-of-range and NaN values are clamped to [0, 1] to avoid render errors.
 *
 * @param ctx - Canvas 2D context (caller must set canvas width/height for correct dimensions)
 * @param width - Depth map width (pixels)
 * @param height - Depth map height (pixels)
 * @param depth - Row-major float array, length must be width * height
 */
export function renderDepthToCanvas(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
  depth: number[]
): void {
  const expectedLength = width * height;
  if (depth.length !== expectedLength) {
    console.warn(
      `[depthCanvas] length mismatch: depth.length=${depth.length}, width*height=${expectedLength}`
    );
    return;
  }

  const imageData = ctx.createImageData(width, height);
  const data = imageData.data;

  for (let i = 0; i < depth.length; i++) {
    let v = depth[i];
    // Clamp and guard against NaN for safe rendering
    if (Number.isNaN(v)) v = 0;
    else if (v < 0) v = 0;
    else if (v > 1) v = 1;
    const gray = Math.round(v * 255);
    const j = i * 4;
    data[j] = gray;     // R
    data[j + 1] = gray; // G
    data[j + 2] = gray; // B
    data[j + 3] = 255; // A
  }

  ctx.putImageData(imageData, 0, 0);
}
