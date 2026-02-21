// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Smoke test — UI-502. Confirms Vitest + jsdom setup works.
 */
import { describe, it, expect } from "vitest";

describe("smoke", () => {
  it("runs in jsdom and basic assert works", () => {
    expect(1 + 1).toBe(2);
  });
});
