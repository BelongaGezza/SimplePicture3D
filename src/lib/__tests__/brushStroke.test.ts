// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

import { describe, it, expect } from "vitest";
import {
  interpolateLine,
  interpolateStrokePoints,
  type Point,
} from "../brushStroke";

describe("brushStroke (JR1-1201)", () => {
  describe("interpolateLine", () => {
    it("returns both endpoints when distance <= maxSpacing", () => {
      const pts = interpolateLine(0, 0, 1, 0, 2);
      expect(pts).toHaveLength(2);
      expect(pts[0]).toEqual({ x: 0, y: 0 });
      expect(pts[1]).toEqual({ x: 1, y: 0 });
    });

    it("inserts points so gap is at most maxSpacing", () => {
      const pts = interpolateLine(0, 0, 10, 0, 2);
      expect(pts.length).toBeGreaterThanOrEqual(2);
      expect(pts[0]).toEqual({ x: 0, y: 0 });
      expect(pts[pts.length - 1]).toEqual({ x: 10, y: 0 });
      for (let i = 0; i < pts.length - 1; i++) {
        const dx = pts[i + 1].x - pts[i].x;
        const dy = pts[i + 1].y - pts[i].y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        expect(dist).toBeLessThanOrEqual(2 + 0.01);
      }
    });

    it("works for diagonal line", () => {
      const pts = interpolateLine(0, 0, 6, 8, 3);
      expect(pts[0]).toEqual({ x: 0, y: 0 });
      expect(pts[pts.length - 1]).toEqual({ x: 6, y: 8 });
    });

    it("rounds coordinates to integers", () => {
      const pts = interpolateLine(0.4, 0.6, 2.4, 2.6, 1);
      pts.forEach((p) => {
        expect(Number.isInteger(p.x)).toBe(true);
        expect(Number.isInteger(p.y)).toBe(true);
      });
    });
  });

  describe("interpolateStrokePoints", () => {
    it("returns empty for empty input", () => {
      expect(interpolateStrokePoints([])).toEqual([]);
    });

    it("returns single point rounded for single input", () => {
      expect(interpolateStrokePoints([{ x: 1.3, y: 2.7 }])).toEqual([
        { x: 1, y: 3 },
      ]);
    });

    it("concatenates segments without duplicating joints", () => {
      const points: Point[] = [
        { x: 0, y: 0 },
        { x: 5, y: 0 },
        { x: 5, y: 5 },
      ];
      const out = interpolateStrokePoints(points, 2);
      expect(out[0]).toEqual({ x: 0, y: 0 });
      expect(out[out.length - 1]).toEqual({ x: 5, y: 5 });
      // No duplicate (5,0) at the joint
      const seen = new Set(out.map((p) => `${p.x},${p.y}`));
      expect(seen.size).toBe(out.length);
    });

    it("produces continuous path with no gaps larger than maxSpacing", () => {
      const points: Point[] = [
        { x: 0, y: 0 },
        { x: 20, y: 0 },
        { x: 20, y: 15 },
      ];
      const out = interpolateStrokePoints(points, 3);
      for (let i = 0; i < out.length - 1; i++) {
        const dx = out[i + 1].x - out[i].x;
        const dy = out[i + 1].y - out[i].y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        expect(dist).toBeLessThanOrEqual(3 + 0.01);
      }
    });
  });
});
