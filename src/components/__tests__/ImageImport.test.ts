// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Component tests for ImageImport.svelte — JR1-502.
 * Tests default render, loading state, error state, Load button, unsupported format on drop.
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import ImageImport from "../ImageImport.svelte";

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}));

vi.mock("$lib/tauri", () => ({
  loadImage: vi.fn(),
}));

describe("ImageImport", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("default render shows Drop image or click to load and Load button", () => {
    render(ImageImport, {
      props: {
        loading: false,
        errorMessage: "",
      },
    });
    expect(screen.getByText(/Drop image or click to load/i)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Load/i })).toBeInTheDocument();
  });

  it("when loading is true, shows spinner and Loading… text; Load button not visible", () => {
    render(ImageImport, {
      props: {
        loading: true,
        errorMessage: "",
      },
    });
    expect(screen.getByText(/Loading…/i)).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /Load/i })).not.toBeInTheDocument();
  });

  it("when errorMessage is set, displays error with role alert", () => {
    render(ImageImport, {
      props: {
        loading: false,
        errorMessage: "Failed to load image",
      },
    });
    const alert = screen.getByRole("alert");
    expect(alert).toHaveTextContent("Failed to load image");
  });

  it("Load button click opens file picker", async () => {
    const { open } = await import("@tauri-apps/plugin-dialog");
    (open as ReturnType<typeof vi.fn>).mockResolvedValue(null);
    render(ImageImport, {
      props: {
        loading: false,
        errorMessage: "",
      },
    });
    const loadButton = screen.getByRole("button", { name: /Load/i });
    await fireEvent.click(loadButton);
    expect(open).toHaveBeenCalledWith(
      expect.objectContaining({
        multiple: false,
        directory: false,
        filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg"] }],
      })
    );
  });

  it("dropping unsupported format (.gif) calls onLoadError with format message", async () => {
    const onLoadError = vi.fn();
    render(ImageImport, {
      props: {
        loading: false,
        errorMessage: "",
        onLoadError,
      },
    });
    const dropZone = screen.getByRole("region", { name: /Image import/i });
    const file = new File(["x"], "image.gif", { type: "image/gif" });
    Object.defineProperty(file, "path", { value: "C:\\temp\\image.gif", writable: true });
    await fireEvent.drop(dropZone, {
      dataTransfer: { files: [file] },
    });
    expect(onLoadError).toHaveBeenCalledWith("Unsupported format. Use PNG or JPG.");
  });
});
