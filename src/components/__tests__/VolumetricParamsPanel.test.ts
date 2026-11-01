// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Smoke tests for VolumetricParamsPanel.svelte — QA-D-06.
 * Verifies pixel step and depth threshold controls render.
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte/svelte5";
import VolumetricParamsPanel from "../VolumetricParamsPanel.svelte";

const mockInvoke = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

describe("VolumetricParamsPanel", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    mockInvoke.mockResolvedValue(0);
  });

  it("renders pixel step and depth threshold sliders", () => {
    render(VolumetricParamsPanel, {
      props: {
        hasDepth: false,
        stepX: 2,
        stepY: 2,
        depthThreshold: 0.1,
      },
    });

    expect(
      screen.getByRole("slider", { name: /Pixel sampling step for X and Y/i })
    ).toBeInTheDocument();
    expect(
      screen.getByRole("slider", { name: /Minimum depth value to emit a point/i })
    ).toBeInTheDocument();
    expect(screen.getByText("Point estimate")).toBeInTheDocument();
    expect(screen.getByText(/Load a depth map to estimate points/i)).toBeInTheDocument();
  });

  it("disables sliders when hasDepth is false", () => {
    render(VolumetricParamsPanel, {
      props: {
        hasDepth: false,
      },
    });

    expect(screen.getByRole("slider", { name: /Pixel sampling step/i })).toBeDisabled();
    expect(screen.getByRole("slider", { name: /Minimum depth value/i })).toBeDisabled();
  });
});
