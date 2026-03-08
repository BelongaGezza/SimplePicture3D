// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Brush stroke interpolation for mask painting (JR1-1201).
 * Interpolates between pointer events so fast strokes produce continuous mask regions (no gaps).
 */

export interface Point {
  x: number;
  y: number;
}

/**
 * Interpolate between consecutive points so that no two output points are farther
 * than pixelSpacing apart. This fills gaps when the user moves the pointer quickly.
 *
 * @param points - Raw pointer positions (e.g. from pointermove)
 * @param pixelSpacing - Max distance between consecutive output points (e.g. brush radius or 2–4 px)
 * @returns Dense list of points suitable for drawing circles or sending to set_mask_region
 */
export function interpolateBrushStroke(
  points: Point[],
  pixelSpacing: number
): Point[] {
  if (pixelSpacing <= 0 || !Number.isFinite(pixelSpacing)) {
    return [...points];
  }
  if (points.length === 0) return [];
  if (points.length === 1) return [points[0]];

  const out: Point[] = [points[0]];

  for (let i = 1; i < points.length; i++) {
    const a = points[i - 1];
    const b = points[i];
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const dist = Math.hypot(dx, dy);

    if (dist <= pixelSpacing) {
      out.push(b);
      continue;
    }

    const steps = Math.ceil(dist / pixelSpacing);
    const step = 1 / steps;
    for (let t = step; t < 1; t += step) {
      out.push({
        x: a.x + dx * t,
        y: a.y + dy * t,
      });
    }
    out.push(b);
  }

  return out;
}
