// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Unit tests for selection.ts (JR1-1202).
 * pointInPolygon, polygonToMaskData, clampRectangle.
 */
import { describe, it, expect } from "vitest";
import {
  pointInPolygon,
  polygonToMaskData,
  clampRectangle,
  type Point,
  type Rectangle,
} from "../selection";

describe("pointInPolygon", () => {
  const square: Point[] = [
    { x: 0, y: 0 },
    { x: 10, y: 0 },
    { x: 10, y: 10 },
    { x: 0, y: 10 },
  ];

  it("returns true for point inside", () => {
    expect(pointInPolygon(5, 5, square)).toBe(true);
  });

  it("returns false for point outside", () => {
    expect(pointInPolygon(15, 5, square)).toBe(false);
    expect(pointInPolygon(-1, 5, square)).toBe(false);
  });

  it("returns false for polygon with fewer than 3 vertices", () => {
    expect(pointInPolygon(0, 0, [])).toBe(false);
    expect(pointInPolygon(0, 0, [{ x: 0, y: 0 }])).toBe(false);
    expect(pointInPolygon(0, 0, [{ x: 0, y: 0 }, { x: 1, y: 1 }])).toBe(false);
  });
});

describe("polygonToMaskData", () => {
  it("returns all false for empty/small polygon", () => {
    const data = polygonToMaskData(4, 4, []);
    expect(data).toHaveLength(16);
    expect(data.every((v) => !v)).toBe(true);

    const data2 = polygonToMaskData(2, 2, [{ x: 0, y: 0 }, { x: 1, y: 0 }]);
    expect(data2.every((v) => !v)).toBe(true);
  });

  it("fills interior of triangle", () => {
    const tri: Point[] = [
      { x: 1, y: 0 },
      { x: 2, y: 2 },
      { x: 0, y: 2 },
    ];
    const data = polygonToMaskData(3, 3, tri);
    expect(data).toHaveLength(9);
    // Center (1,1) should be inside
    expect(data[1 * 3 + 1]).toBe(true);
  });
});

describe("clampRectangle", () => {
  it("returns rect clamped to bounds", () => {
    const r: Rectangle = { x: -5, y: 10, width: 100, height: 20 };
    const out = clampRectangle(r, 50, 30);
    expect(out).toBeDefined();
    expect(out!.x).toBe(0);
    expect(out!.y).toBe(10);
    expect(out!.width).toBe(50);
    expect(out!.height).toBe(20);
  });

  it("returns undefined when fully outside", () => {
    const r: Rectangle = { x: 10, y: 10, width: 5, height: 5 };
    const out = clampRectangle(r, 10, 10);
    expect(out).toBeUndefined();
  });
});
