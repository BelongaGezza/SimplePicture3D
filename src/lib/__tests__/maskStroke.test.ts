// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Unit tests for maskStroke.ts (JR1-1201).
 * interpolateBrushStroke: continuous strokes, no gaps beyond pixelSpacing.
 */
import { describe, it, expect } from "vitest";
import { interpolateBrushStroke } from "../maskStroke";

describe("interpolateBrushStroke", () => {
  it("returns copy of single point", () => {
    const points = [{ x: 10, y: 20 }];
    expect(interpolateBrushStroke(points, 2)).toEqual(points);
  });

  it("returns empty for empty input", () => {
    expect(interpolateBrushStroke([], 2)).toEqual([]);
  });

  it("keeps consecutive points when already within spacing", () => {
    const points = [
      { x: 0, y: 0 },
      { x: 1, y: 0 },
      { x: 2, y: 0 },
    ];
    const out = interpolateBrushStroke(points, 2);
    expect(out).toHaveLength(3);
    expect(out[0]).toEqual({ x: 0, y: 0 });
    expect(out[2]).toEqual({ x: 2, y: 0 });
  });

  it("inserts points when gap exceeds pixelSpacing", () => {
    const points = [
      { x: 0, y: 0 },
      { x: 10, y: 0 },
    ];
    const out = interpolateBrushStroke(points, 2);
    expect(out[0]).toEqual({ x: 0, y: 0 });
    expect(out[out.length - 1]).toEqual({ x: 10, y: 0 });
    expect(out.length).toBeGreaterThan(2);
    for (let i = 1; i < out.length; i++) {
      const dx = out[i].x - out[i - 1].x;
      const dy = out[i].y - out[i - 1].y;
      expect(Math.hypot(dx, dy)).toBeLessThanOrEqual(2 + 1e-6);
    }
  });

  it("handles diagonal stroke", () => {
    const points = [
      { x: 0, y: 0 },
      { x: 6, y: 8 },
    ];
    const out = interpolateBrushStroke(points, 3);
    expect(out[0]).toEqual({ x: 0, y: 0 });
    expect(out[out.length - 1]).toEqual({ x: 6, y: 8 });
    const dist = Math.hypot(6, 8);
    const minSteps = Math.ceil(dist / 3);
    expect(out.length).toBeGreaterThanOrEqual(minSteps);
  });

  it("invalid pixelSpacing returns copy of points", () => {
    const points = [
      { x: 0, y: 0 },
      { x: 100, y: 0 },
    ];
    expect(interpolateBrushStroke(points, 0)).toEqual(points);
    expect(interpolateBrushStroke(points, -1)).toEqual(points);
  });
});
