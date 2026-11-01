// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * xTool F2 Ultra UV processing volume checks for crystal blank / etch interior.
 * @see docs/engraver-charter.md
 */

/** F2 Ultra UV maximum processing area (mm). */
export const F2_ULTRA_UV_PROCESSING_MM = {
  length: 70,
  width: 70,
  height: 150,
} as const;

/** Warn when etch interior is closer than this to a processing limit (mm). */
export const F2_EDGE_CLEARANCE_MM = 5;

export type F2BlankAxis = "length" | "width" | "height";

export interface F2AxisExceedWarning {
  axis: F2BlankAxis;
  /** Crystal blank dimension on this axis (mm). */
  valueMm: number;
  limitMm: number;
}

export interface F2AxisNearEdgeWarning {
  axis: F2BlankAxis;
  /** Etch (interior) dimension on this axis (mm). */
  etchMm: number;
  limitMm: number;
  /** Clearance from etch extent to processing limit (mm). */
  clearanceMm: number;
}

export interface F2VolumeWarningResult {
  exceeds: F2AxisExceedWarning[];
  nearEdge: F2AxisNearEdgeWarning[];
}

const AXIS_LABELS: Record<F2BlankAxis, string> = {
  length: "Length (L)",
  width: "Width (W)",
  height: "Height (H)",
};

export function f2AxisLabel(axis: F2BlankAxis): string {
  return AXIS_LABELS[axis];
}

function finitePositive(n: number): number {
  if (!Number.isFinite(n) || n < 0) return 0;
  return n;
}

/** Interior etch volume dimension after uniform margin (mm). */
export function etchInteriorDimension(blankMm: number, marginMm: number): number {
  const blank = finitePositive(blankMm);
  const margin = finitePositive(marginMm);
  return Math.max(0, blank - 2 * margin);
}

/**
 * Evaluate F2 Ultra UV warnings for the current blank envelope.
 *
 * - **Exceeds:** any crystal blank dimension exceeds 70 × 70 × 150 mm.
 * - **Near edge:** etch interior is within {@link F2_EDGE_CLEARANCE_MM} mm of a limit
 *   on an axis where the crystal blank itself still fits the machine volume.
 */
export function evaluateF2VolumeWarnings(
  lengthMm: number,
  widthMm: number,
  heightMm: number,
  marginMm: number,
): F2VolumeWarningResult {
  const L = finitePositive(lengthMm);
  const W = finitePositive(widthMm);
  const H = finitePositive(heightMm);
  const margin = finitePositive(marginMm);

  const limits = F2_ULTRA_UV_PROCESSING_MM;
  const blankDims: Record<F2BlankAxis, number> = { length: L, width: W, height: H };
  const etchDims: Record<F2BlankAxis, number> = {
    length: etchInteriorDimension(L, margin),
    width: etchInteriorDimension(W, margin),
    height: etchInteriorDimension(H, margin),
  };

  const exceeds: F2AxisExceedWarning[] = [];
  const nearEdge: F2AxisNearEdgeWarning[] = [];

  (["length", "width", "height"] as const).forEach((axis) => {
    const limit = limits[axis];
    const blank = blankDims[axis];
    if (blank > limit) {
      exceeds.push({ axis, valueMm: blank, limitMm: limit });
      return;
    }

    const etch = etchDims[axis];
    const clearance = limit - etch;
    if (clearance >= 0 && clearance < F2_EDGE_CLEARANCE_MM) {
      nearEdge.push({
        axis,
        etchMm: etch,
        limitMm: limit,
        clearanceMm: clearance,
      });
    }
  });

  return { exceeds, nearEdge };
}

export function formatF2ExceedMessage(warnings: F2AxisExceedWarning[]): string {
  if (warnings.length === 0) return "";
  const detail = warnings
    .map((w) => `${f2AxisLabel(w.axis)} ${w.valueMm} mm (max ${w.limitMm} mm)`)
    .join("; ");
  return `Crystal blank exceeds xTool F2 Ultra UV processing volume (70 × 70 × 150 mm). Export may not fit in the engraver: ${detail}.`;
}

export function formatF2NearEdgeMessage(warnings: F2AxisNearEdgeWarning[]): string {
  if (warnings.length === 0) return "";
  const detail = warnings
    .map(
      (w) =>
        `${f2AxisLabel(w.axis)} etch ${w.etchMm} mm (${w.clearanceMm.toFixed(1)} mm to limit)`,
    )
    .join("; ");
  return `Too close to the edge: etch volume is within ${F2_EDGE_CLEARANCE_MM} mm of the F2 Ultra UV limit on ${detail}.`;
}
