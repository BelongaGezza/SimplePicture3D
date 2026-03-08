// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Brush stroke smoothing for mask painting (JR1-1201).
 * Interpolates between pointer events so fast strokes produce continuous mask regions (no gaps).
 * Use from canvas pointer handlers; send interpolated points to backend set_mask_region or batch.
 */

export interface Point {
  x: number;
  y: number;
}

/**
 * Interpolate points along a line from (x0,y0) to (x1,y1) so that consecutive points
 * are at most `maxSpacing` pixels apart. Uses integer pixel coordinates.
 * Includes the start point; the end point (x1,y1) is included so the next segment can attach.
 *
 * @param x0 - Start x (pixel)
 * @param y0 - Start y (pixel)
 * @param x1 - End x (pixel)
 * @param y1 - End y (pixel)
 * @param maxSpacing - Max distance between consecutive output points (default 2)
 * @returns Array of points along the line, including both endpoints
 */
export function interpolateLine(
  x0: number,
  y0: number,
  x1: number,
  y1: number,
  maxSpacing: number = 2
): Point[] {
  const dx = x1 - x0;
  const dy = y1 - y0;
  const len = Math.sqrt(dx * dx + dy * dy);
  if (len <= maxSpacing || !Number.isFinite(len)) {
    return [{ x: Math.round(x0), y: Math.round(y0) }, { x: Math.round(x1), y: Math.round(y1) }];
  }
  const n = Math.ceil(len / maxSpacing);
  const points: Point[] = [];
  for (let i = 0; i <= n; i++) {
    const t = i / n;
    points.push({
      x: Math.round(x0 + dx * t),
      y: Math.round(y0 + dy * t),
    });
  }
  return points;
}

/**
 * Given a sequence of stroke points (e.g. from pointermove), interpolate between consecutive
 * points so that no gap is larger than maxSpacing. Use before sending to backend or drawing.
 *
 * @param points - Raw pointer positions (e.g. from pointerdown + pointermove)
 * @param maxSpacing - Max pixel distance between consecutive output points (default 2)
 * @returns Flattened list of points along the stroke with no gaps > maxSpacing
 */
export function interpolateStrokePoints(
  points: Point[],
  maxSpacing: number = 2
): Point[] {
  if (points.length === 0) return [];
  if (points.length === 1) return [{ x: Math.round(points[0].x), y: Math.round(points[0].y) }];
  const out: Point[] = [];
  for (let i = 0; i < points.length - 1; i++) {
    const a = points[i];
    const b = points[i + 1];
    const segment = interpolateLine(a.x, a.y, b.x, b.y, maxSpacing);
    // Avoid duplicate joint when concatenating segments
    if (i === 0) {
      out.push(segment[0]);
    }
    for (let j = 1; j < segment.length; j++) {
      out.push(segment[j]);
    }
  }
  return out;
}
