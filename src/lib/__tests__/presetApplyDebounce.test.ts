// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/**
 * Unit test: preset apply clears pending debounce so backend is not overwritten with stale params
 * (CONSULTANT_TASK_PRESET_APPLY, Sprint 2.4).
 *
 * Encodes the same pattern as App.svelte: debounced applyParamsToBackend, applyPresetAndRefresh
 * clears the timer then fetches params. Asserts that after "apply preset", advancing time does
 * not invoke the backend write with pre-preset values.
 */
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

const DEBOUNCE_MS = 80;

describe("preset apply clears debounce", () => {
  let setDepthAdjustmentParams: ReturnType<typeof vi.fn>;
  let getDepthAdjustmentParams: ReturnType<typeof vi.fn>;
  let loadPreset: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    vi.useFakeTimers();
    setDepthAdjustmentParams = vi.fn().mockResolvedValue({ canUndo: false, canRedo: false });
    getDepthAdjustmentParams = vi.fn();
    loadPreset = vi.fn().mockResolvedValue(undefined);
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("when applyPresetAndRefresh runs, pending debounce does not call setDepthAdjustmentParams with stale params", async () => {
    // Stale params (from a recent slider change); preset params = what backend returns after load_preset.
    const staleParams = { brightness: 0.2, contrast: 1, gamma: 1, invert: false, depthMinMm: 2, depthMaxMm: 10 };
    const presetParams = { brightness: 0.05, contrast: 1.1, gamma: 1.15, invert: false, depthMinMm: 2, depthMaxMm: 10 };

    getDepthAdjustmentParams.mockResolvedValue(presetParams); // backend state after load_preset

    let debounceTimer: ReturnType<typeof setTimeout> | null = null;
    let adjustmentParams = { ...staleParams };

    function applyParamsToBackend() {
      debounceTimer = null;
      void setDepthAdjustmentParams(adjustmentParams);
    }

    function handleParamsChange(p: typeof adjustmentParams) {
      adjustmentParams = { ...p };
      if (debounceTimer != null) clearTimeout(debounceTimer);
      debounceTimer = setTimeout(applyParamsToBackend, DEBOUNCE_MS);
    }

    async function applyPresetAndRefresh() {
      if (debounceTimer != null) {
        clearTimeout(debounceTimer);
        debounceTimer = null;
      }
      const params = await getDepthAdjustmentParams();
      adjustmentParams = { ...params };
    }

    // User changes slider → debounce scheduled
    handleParamsChange(staleParams);
    expect(debounceTimer).not.toBeNull();

    // User applies preset before debounce fires: clear timer, then get params from backend
    await applyPresetAndRefresh();
    expect(adjustmentParams.brightness).toBe(presetParams.brightness);
    expect(getDepthAdjustmentParams).toHaveBeenCalled();

    // Advance time past debounce. Without clearing the timer, applyParamsToBackend would run
    // here with staleParams and overwrite the backend.
    vi.advanceTimersByTime(DEBOUNCE_MS + 20);
    await Promise.resolve();

    // setDepthAdjustmentParams must not have been called with stale params (the debounce was cleared).
    expect(setDepthAdjustmentParams).not.toHaveBeenCalledWith(
      expect.objectContaining({ brightness: staleParams.brightness })
    );
  });

  it("when debounce is not cleared, setDepthAdjustmentParams is called with current params after timer", async () => {
    const initialParams = { brightness: 0.1, contrast: 1, gamma: 1, invert: false, depthMinMm: 2, depthMaxMm: 10 };
    getDepthAdjustmentParams.mockResolvedValue(initialParams);

    let debounceTimer: ReturnType<typeof setTimeout> | null = null;
    let adjustmentParams = { ...initialParams };

    function applyParamsToBackend() {
      debounceTimer = null;
      void setDepthAdjustmentParams(adjustmentParams);
    }

    function handleParamsChange(p: typeof adjustmentParams) {
      adjustmentParams = { ...p };
      if (debounceTimer != null) clearTimeout(debounceTimer);
      debounceTimer = setTimeout(applyParamsToBackend, DEBOUNCE_MS);
    }

    handleParamsChange(initialParams);
    vi.advanceTimersByTime(DEBOUNCE_MS + 10);
    await Promise.resolve();

    expect(setDepthAdjustmentParams).toHaveBeenCalledWith(expect.objectContaining({ brightness: 0.1 }));
  });
});
