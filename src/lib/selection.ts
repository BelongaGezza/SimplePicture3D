// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Selection tools for masking (JR1-1202).
 * Rectangle: use set_mask_region with bounds. Lasso: polygon → mask buffer (point-in-polygon).
 */

export interface Point {
  x: number;
  y: number;
}

export interface Rectangle {
  x: number;
  y: number;
  width: number;
  height: number;
}

/**
 * Point-in-polygon (ray casting). Returns true if (px, py) is inside the closed polygon.
 * Boundary is considered inside. Works for non-self-intersecting polygons.
 */
export function pointInPolygon(px: number, py: number, polygon: Point[]): boolean {
  const n = polygon.length;
  if (n < 3) return false;
  let inside = false;
  for (let i = 0, j = n - 1; i < n; j = i++) {
    const xi = polygon[i].x;
    const yi = polygon[i].y;
    const xj = polygon[j].x;
    const yj = polygon[j].y;
    const intersect =
      yi > py !== yj > py && px < ((xj - xi) * (py - yi)) / (yj - yi) + xi;
    if (intersect) inside = !inside;
  }
  return inside;
}

/**
 * Build a row-major mask buffer (true inside polygon) for the given dimensions.
 * Used to apply lasso selection to mask via set_mask (merge with current mask in caller).
 */
export function polygonToMaskData(
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
 * Clamp rectangle to [0, width) x [0, height). Returns undefined if fully outside or invalid.
 */
export function clampRectangle(
  rect: Rectangle,
  width: number,
  height: number
): Rectangle | undefined {
  const x = Math.max(0, Math.min(rect.x, width));
  const y = Math.max(0, Math.min(rect.y, height));
  const x2 = Math.max(0, Math.min(rect.x + rect.width, width));
  const y2 = Math.max(0, Math.min(rect.y + rect.height, height));
  const w = x2 - x;
  const h = y2 - y;
  if (w <= 0 || h <= 0) return undefined;
  return { x, y, width: w, height: h };
}
