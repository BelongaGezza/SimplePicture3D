// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Selection tools for mask (JR1-1202): rectangle and lasso (polygon) rasterization.
 * Produces a boolean grid (row-major) that can be merged with current mask and sent via setMask.
 */

export interface Point {
  x: number;
  y: number;
}

/**
 * Rasterize a rectangle [x, y, width, height] to a row-major boolean array for the given canvas size.
 * Coordinates are in pixel space; out-of-bounds are clipped.
 */
export function rasterizeRect(
  width: number,
  height: number,
  x: number,
  y: number,
  rectWidth: number,
  rectHeight: number
): boolean[] {
  const data = new Array<boolean>(width * height).fill(false);
  const x0 = Math.max(0, Math.floor(x));
  const y0 = Math.max(0, Math.floor(y));
  const x1 = Math.min(width, Math.ceil(x + rectWidth));
  const y1 = Math.min(height, Math.ceil(y + rectHeight));
  for (let py = y0; py < y1; py++) {
    for (let px = x0; px < x1; px++) {
      data[py * width + px] = true;
    }
  }
  return data;
}

/**
 * Point-in-polygon test (ray casting). Returns true if (px, py) is inside the polygon.
 * Polygon is an array of vertices; we use even-odd rule.
 */
function pointInPolygon(px: number, py: number, polygon: Point[]): boolean {
  const n = polygon.length;
  if (n < 3) return false;
  let inside = false;
  let j = n - 1;
  for (let i = 0; i < n; i++) {
    const xi = polygon[i].x;
    const yi = polygon[i].y;
    const xj = polygon[j].x;
    const yj = polygon[j].y;
    if (yi > py !== yj > py && px < ((xj - xi) * (py - yi)) / (yj - yi) + xi) {
      inside = !inside;
    }
    j = i;
  }
  return inside;
}

/**
 * Rasterize a polygon (lasso) to a row-major boolean array for the given canvas size.
 * Pixels inside the polygon (using ray-casting) are true.
 */
export function rasterizePolygon(
  width: number,
  height: number,
  polygon: Point[]
): boolean[] {
  const data = new Array<boolean>(width * height).fill(false);
  if (polygon.length < 3) return data;
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      if (pointInPolygon(x + 0.5, y + 0.5, polygon)) {
        data[y * width + x] = true;
      }
    }
  }
  return data;
}

/**
 * Merge selection (row-major booleans) with current mask (row-major booleans).
 * "Add to mask": result[i] = current[i] || selection[i].
 * "Replace": result = selection.
 * Both arrays must have length width*height.
 */
export function mergeSelectionWithMask(
  width: number,
  height: number,
  currentMask: boolean[],
  selection: boolean[],
  addToMask: boolean = true
): boolean[] {
  const n = width * height;
  if (currentMask.length !== n || selection.length !== n) {
    throw new Error("Mask and selection length must match width*height");
  }
  if (addToMask) {
    return currentMask.map((m, i) => m || selection[i]);
  }
  return selection.slice();
}

/**
 * Apply a rectangular selection to the mask via the backend.
 * Gets current mask, merges selection (add to mask), then sets full mask (one undo step).
 * Caller must invoke getMask/setMask from tauri (this module has no Tauri dependency).
 */
export function buildMaskWithRect(
  width: number,
  height: number,
  currentMaskData: boolean[],
  x: number,
  y: number,
  rectWidth: number,
  rectHeight: number,
  addToMask: boolean
): boolean[] {
  const selection = rasterizeRect(width, height, x, y, rectWidth, rectHeight);
  return mergeSelectionWithMask(width, height, currentMaskData, selection, addToMask);
}

/**
 * Apply a polygon (lasso) selection to the mask.
 * Returns the new mask data to send via setMask.
 */
export function buildMaskWithPolygon(
  width: number,
  height: number,
  currentMaskData: boolean[],
  polygon: Point[],
  addToMask: boolean
): boolean[] {
  const selection = rasterizePolygon(width, height, polygon);
  return mergeSelectionWithMask(width, height, currentMaskData, selection, addToMask);
}
