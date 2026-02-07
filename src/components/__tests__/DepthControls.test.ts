/**
 * Component tests for DepthControls.svelte — JR1-501.
 * Tests disabled/enabled state, slider interaction, invert, reset, clamping, keyboard.
 */
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import DepthControls from "../DepthControls.svelte";
import type { DepthAdjustmentParams } from "$lib/tauri";

const defaultParams: DepthAdjustmentParams = {
  brightness: 0,
  contrast: 1,
  gamma: 1,
  invert: false,
  depthMinMm: 2,
  depthMaxMm: 10,
};

describe("DepthControls", () => {
  it("when hasDepth is false, shows message and no sliders", () => {
    render(DepthControls, {
      props: {
        hasDepth: false,
        params: defaultParams,
      },
    });
    expect(screen.getByText("Generate a depth map to adjust.")).toBeInTheDocument();
    expect(screen.queryByLabelText(/Brightness/i)).not.toBeInTheDocument();
    expect(screen.queryByLabelText(/Depth min/i)).not.toBeInTheDocument();
  });

  it("when hasDepth is true, renders all sliders with correct initial values", () => {
    render(DepthControls, {
      props: {
        hasDepth: true,
        params: defaultParams,
      },
    });
    expect(screen.queryByText("Generate a depth map to adjust.")).not.toBeInTheDocument();
    const brightnessSlider = screen.getByRole("slider", { name: /Brightness/i });
    const contrastSlider = screen.getByRole("slider", { name: /Contrast/i });
    const gammaSlider = screen.getByRole("slider", { name: /Gamma/i });
    const depthMin = screen.getByRole("slider", { name: /Depth min/i });
    const depthMax = screen.getByRole("slider", { name: /Depth max/i });
    expect(brightnessSlider).toHaveValue("0");
    expect(contrastSlider).toHaveValue("1");
    expect(gammaSlider).toHaveValue("1");
    expect(depthMin).toHaveValue("2");
    expect(depthMax).toHaveValue("10");
  });

  it("changing brightness slider fires onParamsChange with updated brightness", async () => {
    const onParamsChange = vi.fn();
    render(DepthControls, {
      props: {
        hasDepth: true,
        params: defaultParams,
        onParamsChange,
      },
    });
    const slider = screen.getByRole("slider", { name: /Brightness/i });
    await fireEvent.input(slider, { target: { value: "0.2" } });
    expect(onParamsChange).toHaveBeenCalled();
    const lastCall = onParamsChange.mock.calls[onParamsChange.mock.calls.length - 1][0];
    expect(lastCall.brightness).toBe(0.2);
  });

  it("toggling invert checkbox fires onParamsChange with invert true", async () => {
    const onParamsChange = vi.fn();
    render(DepthControls, {
      props: {
        hasDepth: true,
        params: { ...defaultParams, invert: false },
        onParamsChange,
      },
    });
    const checkbox = screen.getByRole("checkbox", { name: /Invert depth/i });
    await fireEvent.click(checkbox);
    expect(onParamsChange).toHaveBeenCalled();
    const lastCall = onParamsChange.mock.calls[onParamsChange.mock.calls.length - 1][0];
    expect(lastCall.invert).toBe(true);
  });

  it("clicking Reset calls onReset", async () => {
    const onReset = vi.fn();
    render(DepthControls, {
      props: {
        hasDepth: true,
        params: defaultParams,
        onReset,
      },
    });
    const resetButton = screen.getByRole("button", { name: /Reset/i });
    await fireEvent.click(resetButton);
    expect(onReset).toHaveBeenCalledTimes(1);
  });

  it("emitChange corrects depthMaxMm when less than depthMinMm", async () => {
    const onParamsChange = vi.fn();
    render(DepthControls, {
      props: {
        hasDepth: true,
        params: { ...defaultParams, depthMinMm: 8, depthMaxMm: 10 },
        onParamsChange,
      },
    });
    const depthMaxSlider = screen.getByRole("slider", { name: /Depth max/i });
    await fireEvent.input(depthMaxSlider, { target: { value: "5" } });
    expect(onParamsChange).toHaveBeenCalled();
    const lastCall = onParamsChange.mock.calls[onParamsChange.mock.calls.length - 1][0];
    expect(lastCall.depthMaxMm).toBe(8);
    expect(lastCall.depthMinMm).toBe(8);
  });

  it("arrow key on slider changes value by step", async () => {
    const onParamsChange = vi.fn();
    render(DepthControls, {
      props: {
        hasDepth: true,
        params: defaultParams,
        onParamsChange,
      },
    });
    const brightnessSlider = screen.getByRole("slider", { name: /Brightness/i });
    brightnessSlider.focus();
    await fireEvent.keyDown(brightnessSlider, { key: "ArrowRight" });
    expect(onParamsChange).toHaveBeenCalled();
    const lastCall = onParamsChange.mock.calls[onParamsChange.mock.calls.length - 1][0];
    expect(lastCall.brightness).toBeCloseTo(0.01, 2);
  });
});
