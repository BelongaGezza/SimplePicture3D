// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

import { describe, it, expect } from "vitest";
import {
  rasterizeRect,
  rasterizePolygon,
  mergeSelectionWithMask,
} from "../selectionTools";

describe("selectionTools (JR1-1202)", () => {
  describe("rasterizeRect", () => {
    it("fills interior of rect within bounds", () => {
      const w = 10,
        h = 10;
      const data = rasterizeRect(w, h, 2, 2, 3, 3);
      expect(data.length).toBe(100);
      for (let y = 2; y < 5; y++) {
        for (let x = 2; x < 5; x++) {
          expect(data[y * w + x]).toBe(true);
        }
      }
      expect(data[0]).toBe(false);
      expect(data[1 * w + 2]).toBe(false);
      expect(data[5 * w + 4]).toBe(false);
    });

    it("clips to canvas bounds", () => {
      const w = 5,
        h = 5;
      const data = rasterizeRect(w, h, -1, -1, 10, 10);
      expect(data.every((b) => b === true)).toBe(true);
    });

    it("returns all false for zero-size rect", () => {
      const data = rasterizeRect(4, 4, 1, 1, 0, 0);
      expect(data.every((b) => b === false)).toBe(true);
    });
  });

  describe("rasterizePolygon", () => {
    it("returns all false for fewer than 3 vertices", () => {
      expect(rasterizePolygon(5, 5, [{ x: 0, y: 0 }, { x: 1, y: 1 }]).every((b) => !b)).toBe(true);
      expect(rasterizePolygon(5, 5, []).every((b) => !b)).toBe(true);
    });

    it("fills interior of triangle", () => {
      const w = 10,
        h = 10;
      const tri = [{ x: 2, y: 2 }, { x: 7, y: 2 }, { x: 4.5, y: 8 }];
      const data = rasterizePolygon(w, h, tri);
      expect(data.length).toBe(100);
      expect(data[2 * w + 4]).toBe(true);
      expect(data[2 * w + 2]).toBe(true);
      expect(data[0]).toBe(false);
    });
  });

  describe("mergeSelectionWithMask", () => {
    it("adds selection to mask when addToMask true", () => {
      const w = 2,
        h = 2;
      const current = [false, true, false, false];
      const selection = [true, false, true, false];
      const out = mergeSelectionWithMask(w, h, current, selection, true);
      expect(out).toEqual([true, true, true, false]);
    });

    it("replaces with selection when addToMask false", () => {
      const w = 2,
        h = 2;
      const current = [true, true, true, true];
      const selection = [true, false, false, true];
      const out = mergeSelectionWithMask(w, h, current, selection, false);
      expect(out).toEqual(selection);
    });

    it("throws when lengths do not match", () => {
      expect(() =>
        mergeSelectionWithMask(2, 2, [false, false, false], [false, false, false, false], true)
      ).toThrow("must match");
    });
  });
});
