# GOTCHAS — Sprint 2.4

**Sprint:** 2.4 — Progress Streaming for Depth Estimation
**Instructions:** Add entries here as debugging findings are discovered. Merge to `RESEARCH/GOTCHAS.md` when the sprint closes.

---

## Template

```
### [Date] — [Role] — [Short title]
**Context:** [What were you doing]
**Problem:** [What went wrong]
**Root cause:** [Why]
**Fix:** [How it was resolved]
**Watch out for:** [What to avoid in future]
```

---

## Entries

### 2026-03-06 — Quality Engineer — Tauri "Couldn't find callback id" after depth run
**Context:** Manual QA (QA-304-STREAM); depth estimation run completed; console open.
**Problem:** Console showed: `[TAURI] Couldn't find callback id 738117956. This might happen when the app is reloaded while Rust is running an asynchronous operation.` (and a second id). Tester did not reload the app during the run.
**Root cause:** Unknown. Could be: (1) frontend unlisten or command response handler cleaned up before Rust finished invoking callback; (2) duplicate or stale invoke; (3) Tauri internal callback ID reuse.
**Fix:** Not fixed. Worth reproducing and checking: ensure `unlisten()` is only called after command Promise settles; avoid invoking generate_depth_map twice in quick succession.
**Watch out for:** If users see these messages without reloading, consider filing a bug to trace callback lifecycle (invoke vs. emit vs. unlisten timing).

### 2026-03-06 — Quality Engineer — Preset Apply does not update sliders
**Context:** Manual QA (QA-1301, QA-1302). User can save presets and see them (with built-ins) in the list; applying any preset does not change depth sliders/settings.
**Problem:** After clicking Apply (or Load preset) for any preset, sliders (brightness, gamma, depth min/max, etc.) do not update. Backend `load_preset` updates `state.adjustment_params`; frontend then calls `applyPresetAndRefresh()` → `getDepthAdjustmentParams()` and sets `adjustmentParams = params`.
**Root cause:** Typo in `src/lib/tauri.ts` `loadPreset()`: the IPC arg key was `nameOr_path` instead of `name_or_path`. Tauri failed to deserialize the command argument, so `load_preset` threw before touching `state.adjustment_params`. The error was caught in `handleLoadPreset`, which set status to "Load preset failed" and never called `applyPresetAndRefresh()`. The Svelte reactivity and the `{ ...params }` spread fix were both correct; the call simply never reached the backend successfully.
**Fix:** Changed `{ nameOr_path: nameOrPath }` → `{ name_or_path: nameOrPath }` in `tauri.ts:326`. Updated the corresponding unit-test assertion in `tauri.test.ts:271`.
**Watch out for:** Tauri v2 IPC arg keys must exactly match the Rust parameter name in snake_case (e.g. `name_or_path`, `old_name`, `new_name`). A typo that is neither snake_case nor camelCase silently fails at the IPC boundary — the backend command never runs, but the error may be swallowed by a `try/catch` that only sets a status string, making the symptom look like a frontend reactivity problem. Always verify the arg key against the `#[tauri::command]` function signature, and write a unit test that asserts the exact `invoke` call (command name + args object).

### 2026-03-14 — Senior Engineer — Preset apply: debounce overwrote backend after load_preset
**Context:** CONSULTANT_TASK_PRESET_APPLY. After fixing IPC arg key, applying a preset still did not update depth sliders when the user had recently moved a slider.
**Problem:** `load_preset` updated backend state; `applyPresetAndRefresh()` then called `getDepthAdjustmentParams()` and set `adjustmentParams`, but sliders showed old values. Backend was correct; frontend was displaying stale params.
**Root cause:** A pending debounce timer from `handleParamsChange` (slider change) could fire *after* `load_preset` returned but before or during `applyPresetAndRefresh()`. That timer called `applyParamsToBackend()` with the *previous* `adjustmentParams`, overwriting the backend with old values. Then `getDepthAdjustmentParams()` returned those old values, so the UI showed the wrong state.
**Fix:** Clear the debounce timer at the start of `handleLoadPreset`, `handleImportPreset`, and `applyPresetAndRefresh()` so no pending `applyParamsToBackend` runs after `load_preset` (see `App.svelte`).
**Watch out for:** When a flow updates backend state and then syncs UI from backend, cancel any debounced writes that might overwrite that state before the sync completes.

---

## Known risks to watch for in Sprint 2.4

### Tauri v2 AppHandle in synchronous commands
If `generate_depth_map` is kept as `fn` (not `async fn`), passing `tauri::AppHandle` still works in Tauri v2 — it is injected automatically. However, calling `app_handle.emit()` from a background `std::thread` inside `run_depth_estimation_with_progress` is safe because `AppHandle` is `Clone + Send`. Clone the handle before spawning the stderr thread.

### std::thread + Tauri event emit ownership
When the stderr reader runs on a `std::thread::spawn` closure, any captured variables must be `'static`. Cloning `AppHandle` before the spawn and moving the clone into the closure avoids lifetime issues.

### Frontend listen() must be awaited before invoke()
The `listen()` call returns a Promise; it must be `await`ed before `generateDepthMap()` is called, or early events will be missed. Pattern:
```ts
const unlisten = await listen("depth-progress", handler);
try {
  const result = await generateDepthMap(path);
} finally {
  unlisten();
}
```

### Progress bar width: CSS `transition` on `style` attribute in Svelte
Setting `style="width: {pct}%"` on a div with `transition-[width]` (Tailwind) requires that Tailwind includes the `transition-[width]` utility. In JIT mode this should work; if the class is not generated at build time, add it to the `safelist` in `tailwind.config.js`.

### SEC-202: HuggingFace model shards are versioned
HuggingFace model snapshots may include multiple shards (e.g. `model.safetensors`). The hash of the shard file depends on the exact model revision. Pin to a specific git commit/revision of the model repo when recording expected hashes. See `RESEARCH/python-ml.md` for model source details.
