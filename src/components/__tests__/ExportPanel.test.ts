// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Smoke tests for ExportPanel.svelte — QA-D-06.
 * Verifies blank preset, format selector, and export controls render.
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import ExportPanel from "../ExportPanel.svelte";

const mockInvoke = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  save: vi.fn(),
}));

describe("ExportPanel", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    mockInvoke.mockResolvedValue(undefined);
  });

  it("renders blank preset, format selector, and export button", () => {
    render(ExportPanel, {
      props: {
        hasDepth: true,
        sourceFileName: "portrait.png",
      },
    });

    expect(screen.getByText("Blank preset")).toBeInTheDocument();
    expect(screen.getByRole("combobox", { name: /Blank preset/i })).toBeInTheDocument();
    expect(screen.getByRole("option", { name: /80 × 50 × 50 \(standard\)/i })).toBeInTheDocument();

    expect(screen.getByText("Format")).toBeInTheDocument();
    const formatSelect = screen.getByRole("combobox", { name: /Format/i });
    expect(formatSelect).toBeInTheDocument();
    expect(screen.getByRole("option", { name: /^PLY$/i })).toBeInTheDocument();
    expect(screen.getByRole("option", { name: /^XYZ$/i })).toBeInTheDocument();
    expect(screen.getByRole("option", { name: /^CSV$/i })).toBeInTheDocument();

    expect(
      screen.getByRole("button", { name: /Generate and export point cloud/i })
    ).toBeInTheDocument();
    expect(screen.getByLabelText(/Blank length millimetres/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/Blank margin millimetres/i)).toBeInTheDocument();
  });

  it("disables export when hasDepth is false", () => {
    render(ExportPanel, {
      props: {
        hasDepth: false,
      },
    });

    expect(screen.getByRole("button", { name: /Generate and export point cloud/i })).toBeDisabled();
  });
});
