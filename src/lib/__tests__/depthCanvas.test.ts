/**
 * Unit tests for depthCanvas.ts — JR2-501.
 * Tests renderDepthToCanvas: happy path, NaN, clamping, length mismatch, empty input.
 */
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { renderDepthToCanvas } from "../depthCanvas";

describe("renderDepthToCanvas", () => {
  let warnSpy: ReturnType<typeof vi.spyOn>;

  beforeEach(() => {
    warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
  });

  afterEach(() => {
    warnSpy.mockRestore();
  });

  it("happy path: 2×2 depth [0, 0.5, 1, 0.25] maps to grayscale 0, 128, 255, 64", () => {
    const ctx = {
      createImageData(w: number, h: number) {
        return { data: new Uint8ClampedArray(w * h * 4), width: w, height: h };
      },
      putImageData: vi.fn(),
    } as unknown as CanvasRenderingContext2D;
    const depth = [0, 0.5, 1, 0.25];
    renderDepthToCanvas(ctx, 2, 2, depth);

    const call = (ctx.putImageData as ReturnType<typeof vi.fn>).mock.calls[0][0];
    const d = call.data;
    // Pixel 0: 0 -> 0,0,0,255
    expect(d[0]).toBe(0);
    expect(d[1]).toBe(0);
    expect(d[2]).toBe(0);
    expect(d[3]).toBe(255);
    // Pixel 1: 0.5 -> 128
    expect(d[4]).toBe(128);
    expect(d[5]).toBe(128);
    expect(d[6]).toBe(128);
    expect(d[7]).toBe(255);
    // Pixel 2: 1 -> 255
    expect(d[8]).toBe(255);
    expect(d[9]).toBe(255);
    expect(d[10]).toBe(255);
    expect(d[11]).toBe(255);
    // Pixel 3: 0.25 -> 64
    expect(d[12]).toBe(64);
    expect(d[13]).toBe(64);
    expect(d[14]).toBe(64);
    expect(d[15]).toBe(255);
  });

  it("NaN in depth is rendered as black (0)", () => {
    const ctx = {
      createImageData(w: number, h: number) {
        return { data: new Uint8ClampedArray(w * h * 4), width: w, height: h };
      },
      putImageData: vi.fn(),
    } as unknown as CanvasRenderingContext2D;
    const depth = [0.5, Number.NaN, 0.5, 0.5];
    renderDepthToCanvas(ctx, 2, 2, depth);
    const call = (ctx.putImageData as ReturnType<typeof vi.fn>).mock.calls[0][0];
    const d = call.data;
    // Pixel 0: 0.5 -> 128
    expect(d[0]).toBe(128);
    // Pixel 1: NaN -> 0
    expect(d[4]).toBe(0);
    expect(d[5]).toBe(0);
    expect(d[6]).toBe(0);
  });

  it("out-of-range values are clamped: <0 -> 0, >1 -> 255", () => {
    const ctx = {
      createImageData(w: number, h: number) {
        return { data: new Uint8ClampedArray(w * h * 4), width: w, height: h };
      },
      putImageData: vi.fn(),
    } as unknown as CanvasRenderingContext2D;
    const depth = [-0.5, 1.5, 0, 1];
    renderDepthToCanvas(ctx, 2, 2, depth);
    const call = (ctx.putImageData as ReturnType<typeof vi.fn>).mock.calls[0][0];
    const d = call.data;
    expect(d[0]).toBe(0);   // -0.5 clamped to 0
    expect(d[4]).toBe(255); // 1.5 clamped to 255
    expect(d[8]).toBe(0);
    expect(d[12]).toBe(255);
  });

  it("length mismatch: depth.length !== width*height returns early and canvas unchanged", () => {
    const ctx = {
      createImageData: vi.fn(),
      putImageData: vi.fn(),
    } as unknown as CanvasRenderingContext2D;
    const depth = [0, 0.5]; // length 2, but 2*2=4
    renderDepthToCanvas(ctx, 2, 2, depth);
    expect(warnSpy).toHaveBeenCalledWith(
      expect.stringContaining("length mismatch")
    );
    expect(ctx.createImageData).not.toHaveBeenCalled();
    expect(ctx.putImageData).not.toHaveBeenCalled();
  });

  it("empty input: width=0, height=0, depth=[] does not crash", () => {
    const ctx = {
      createImageData(w: number, h: number) {
        return { data: new Uint8ClampedArray(w * h * 4), width: w, height: h };
      },
      putImageData: vi.fn(),
    } as unknown as CanvasRenderingContext2D;
    expect(() => renderDepthToCanvas(ctx, 0, 0, [])).not.toThrow();
    expect(ctx.putImageData).toHaveBeenCalledTimes(1);
    const call = (ctx.putImageData as ReturnType<typeof vi.fn>).mock.calls[0][0];
    expect(call.data.length).toBe(0);
  });
});
