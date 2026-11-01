// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

import { describe, it, expect } from "vitest";
import {
  evaluateF2VolumeWarnings,
  etchInteriorDimension,
  formatF2ExceedMessage,
  formatF2NearEdgeMessage,
} from "../f2VolumeWarnings";

describe("etchInteriorDimension", () => {
  it("subtracts uniform margin on both sides", () => {
    expect(etchInteriorDimension(50, 2)).toBe(46);
    expect(etchInteriorDimension(80, 2)).toBe(76);
  });
});

describe("evaluateF2VolumeWarnings", () => {
  it("default 50×50×80 mm with 2 mm margin has no warnings", () => {
    const r = evaluateF2VolumeWarnings(50, 50, 80, 2);
    expect(r.exceeds).toEqual([]);
    expect(r.nearEdge).toEqual([]);
  });

  it("flags crystal blank exceeding F2 length", () => {
    const r = evaluateF2VolumeWarnings(80, 50, 50, 2);
    expect(r.exceeds).toHaveLength(1);
    expect(r.exceeds[0].axis).toBe("length");
    expect(r.exceeds[0].valueMm).toBe(80);
    expect(r.nearEdge).toEqual([]);
  });

  it("flags crystal blank exceeding F2 height", () => {
    const r = evaluateF2VolumeWarnings(50, 50, 160, 2);
    expect(r.exceeds).toHaveLength(1);
    expect(r.exceeds[0].axis).toBe("height");
  });

  it("flags etch volume within 5 mm of F2 limit when blank still fits", () => {
    // L=68, margin 2 → etch 64 mm, clearance 6 mm — OK
    expect(evaluateF2VolumeWarnings(68, 50, 80, 2).nearEdge).toEqual([]);
    // L=70, margin 2 → etch 66 mm, clearance 4 mm
    const r = evaluateF2VolumeWarnings(70, 50, 80, 2);
    expect(r.exceeds).toEqual([]);
    expect(r.nearEdge).toHaveLength(1);
    expect(r.nearEdge[0].axis).toBe("length");
    expect(r.nearEdge[0].clearanceMm).toBeCloseTo(4);
  });

  it("flags height etch near 150 mm limit", () => {
    // H=150, margin 2 → etch 146, clearance 4
    const r = evaluateF2VolumeWarnings(50, 50, 150, 2);
    expect(r.exceeds).toEqual([]);
    expect(r.nearEdge.some((w) => w.axis === "height")).toBe(true);
  });

  it("does not show near-edge on an axis that already exceeds blank limit", () => {
    const r = evaluateF2VolumeWarnings(80, 50, 80, 2);
    expect(r.exceeds.some((w) => w.axis === "length")).toBe(true);
    expect(r.nearEdge.some((w) => w.axis === "length")).toBe(false);
  });

  it("can report exceed and near-edge on different axes", () => {
    const r = evaluateF2VolumeWarnings(80, 50, 150, 2);
    expect(r.exceeds.some((w) => w.axis === "length")).toBe(true);
    expect(r.nearEdge.some((w) => w.axis === "height")).toBe(true);
  });
});

describe("formatF2 messages", () => {
  it("formatF2ExceedMessage includes volume and axis detail", () => {
    const msg = formatF2ExceedMessage([
      { axis: "length", valueMm: 80, limitMm: 70 },
    ]);
    expect(msg).toContain("70 × 70 × 150");
    expect(msg).toContain("Length (L) 80 mm");
  });

  it("formatF2NearEdgeMessage uses too close to the edge wording", () => {
    const msg = formatF2NearEdgeMessage([
      { axis: "length", etchMm: 66, limitMm: 70, clearanceMm: 4 },
    ]);
    expect(msg).toContain("Too close to the edge");
    expect(msg).toContain("Length (L)");
  });
});
