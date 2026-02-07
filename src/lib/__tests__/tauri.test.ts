/**
 * Unit tests for tauri.ts type layer — JR2-502.
 * Mocks @tauri-apps/api/core invoke; verifies command names, return types, error propagation.
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  loadImage,
  exportStl,
  generateDepthMap,
  getDepthMap,
  getDepthAdjustmentParams,
  setDepthAdjustmentParams,
  resetDepthAdjustments,
  type DepthAdjustmentParams,
  type LoadImageResult,
  type DepthMapResult,
  type DepthMapData,
} from "../tauri";

const mockInvoke = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

describe("tauri IPC", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  describe("loadImage", () => {
    it("calls invoke with load_image and path", async () => {
      const result: LoadImageResult = {
        ok: true,
        width: 100,
        height: 100,
        fileSizeBytes: 1024,
        downsampled: false,
        previewBase64: "base64...",
      };
      mockInvoke.mockResolvedValue(result);
      const out = await loadImage("foo.png");
      expect(mockInvoke).toHaveBeenCalledWith("load_image", { path: "foo.png" });
      expect(out).toEqual(result);
      expect(out.ok).toBe(true);
      expect(out.previewBase64).toBe("base64...");
    });

    it("propagates invoke rejection", async () => {
      mockInvoke.mockRejectedValue(new Error("file not found"));
      await expect(loadImage("missing.png")).rejects.toThrow("file not found");
    });
  });

  describe("exportStl", () => {
    it("calls invoke with export_stl and path", async () => {
      mockInvoke.mockResolvedValue(undefined);
      await exportStl("/out/model.stl");
      expect(mockInvoke).toHaveBeenCalledWith("export_stl", { path: "/out/model.stl" });
    });

    it("propagates invoke rejection", async () => {
      mockInvoke.mockRejectedValue(new Error("export failed"));
      await expect(exportStl("/bad.stl")).rejects.toThrow("export failed");
    });
  });

  describe("generateDepthMap", () => {
    it("calls invoke with generate_depth_map and path", async () => {
      const result: DepthMapResult = {
        width: 64,
        height: 48,
        depth: [0.1, 0.2],
        progress: 100,
        stages: ["inference"],
      };
      mockInvoke.mockResolvedValue(result);
      const out = await generateDepthMap("image.png");
      expect(mockInvoke).toHaveBeenCalledWith("generate_depth_map", { path: "image.png" });
      expect(out).toEqual(result);
      expect(out.depth).toHaveLength(2);
    });

    it("propagates invoke rejection", async () => {
      mockInvoke.mockRejectedValue(new Error("depth failed"));
      await expect(generateDepthMap("x.png")).rejects.toThrow("depth failed");
    });
  });

  describe("getDepthMap", () => {
    it("calls invoke with get_depth_map (no args)", async () => {
      const result: DepthMapData = { width: 10, height: 10, depth: [] };
      mockInvoke.mockResolvedValue(result);
      const out = await getDepthMap();
      expect(mockInvoke).toHaveBeenCalledWith("get_depth_map");
      expect(out).toEqual(result);
    });

    it("returns null when backend returns null", async () => {
      mockInvoke.mockResolvedValue(null);
      const out = await getDepthMap();
      expect(out).toBeNull();
    });

    it("propagates invoke rejection", async () => {
      mockInvoke.mockRejectedValue(new Error("no depth"));
      await expect(getDepthMap()).rejects.toThrow("no depth");
    });
  });

  describe("getDepthAdjustmentParams", () => {
    it("calls invoke with get_depth_adjustment_params", async () => {
      const result: DepthAdjustmentParams = {
        brightness: 0,
        contrast: 1,
        gamma: 1.5,
        invert: false,
        depthMinMm: 0,
        depthMaxMm: 100,
      };
      mockInvoke.mockResolvedValue(result);
      const out = await getDepthAdjustmentParams();
      expect(mockInvoke).toHaveBeenCalledWith("get_depth_adjustment_params");
      expect(out).toEqual(result);
      expect(out).toHaveProperty("brightness");
      expect(out).toHaveProperty("contrast");
      expect(out).toHaveProperty("gamma");
      expect(out).toHaveProperty("invert");
      expect(out).toHaveProperty("depthMinMm");
      expect(out).toHaveProperty("depthMaxMm");
    });

    it("propagates invoke rejection", async () => {
      mockInvoke.mockRejectedValue(new Error("params failed"));
      await expect(getDepthAdjustmentParams()).rejects.toThrow("params failed");
    });
  });

  describe("setDepthAdjustmentParams", () => {
    it("calls invoke with set_depth_adjustment_params and params", async () => {
      mockInvoke.mockResolvedValue(undefined);
      const params: DepthAdjustmentParams = {
        brightness: 0.1,
        contrast: 1.2,
        gamma: 1.0,
        invert: true,
        depthMinMm: 5,
        depthMaxMm: 95,
      };
      await setDepthAdjustmentParams(params);
      expect(mockInvoke).toHaveBeenCalledWith("set_depth_adjustment_params", { params });
    });

    it("propagates invoke rejection", async () => {
      mockInvoke.mockRejectedValue(new Error("set failed"));
      await expect(
        setDepthAdjustmentParams({
          brightness: 0,
          contrast: 1,
          gamma: 1,
          invert: false,
          depthMinMm: 0,
          depthMaxMm: 100,
        })
      ).rejects.toThrow("set failed");
    });
  });

  describe("resetDepthAdjustments", () => {
    it("calls invoke with reset_depth_adjustments", async () => {
      mockInvoke.mockResolvedValue(undefined);
      await resetDepthAdjustments();
      expect(mockInvoke).toHaveBeenCalledWith("reset_depth_adjustments");
    });

    it("propagates invoke rejection", async () => {
      mockInvoke.mockRejectedValue(new Error("reset failed"));
      await expect(resetDepthAdjustments()).rejects.toThrow("reset failed");
    });
  });
});

describe("DepthAdjustmentParams interface", () => {
  it("has all expected fields with correct types", () => {
    const params: DepthAdjustmentParams = {
      brightness: 0,
      contrast: 1,
      gamma: 1,
      invert: false,
      depthMinMm: 0,
      depthMaxMm: 100,
    };
    expect(typeof params.brightness).toBe("number");
    expect(typeof params.contrast).toBe("number");
    expect(typeof params.gamma).toBe("number");
    expect(typeof params.invert).toBe("boolean");
    expect(typeof params.depthMinMm).toBe("number");
    expect(typeof params.depthMaxMm).toBe("number");
    expect(Object.keys(params).sort()).toEqual([
      "brightness",
      "contrast",
      "depthMaxMm",
      "depthMinMm",
      "gamma",
      "invert",
    ]);
  });
});
