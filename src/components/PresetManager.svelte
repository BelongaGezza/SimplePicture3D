<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<script lang="ts">
  /**
   * PresetManager — UI-1301. List user presets, rename, delete.
   * Integrates with list_presets, delete_preset, rename_preset (BACK-1302).
   */
  import Button from "./Button.svelte";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import {
    listPresets,
    deletePreset,
    renamePreset,
    savePreset,
    loadPreset,
  } from "$lib/tauri";
  import type { PresetListItem } from "$lib/tauri";

  /** Emitted when preset list may have changed (e.g. after delete/rename). */
  export let onListChange: (() => void) | undefined = undefined;
  /** Called after import/load so parent can refresh depth state (UI-1304). */
  export let onPresetApplied: (() => void) | undefined = undefined;

  let presets: PresetListItem[] = [];
  let loading = false;
  let error = "";
  /** Name being renamed; when set, show inline rename form. */
  let renamingId: string | null = null;
  let renameValue = "";
  let renameError = "";
  /** Name being deleted (for confirm). */
  let deletingId: string | null = null;
  let deleteError = "";
  /** UI-1304: Import/Export. */
  let importExportError = "";
  let exporting = false;
  let importing = false;

  async function loadList() {
    loading = true;
    error = "";
    try {
      presets = await listPresets();
      // Only show user presets in manager (built-ins are apply-only in dropdown)
      presets = presets.filter((p) => p.kind === "user");
    } catch (e) {
      error = String(e);
      presets = [];
    } finally {
      loading = false;
      onListChange?.();
    }
  }

  function startRename(item: PresetListItem) {
    if (item.kind !== "user") return;
    renamingId = item.id;
    renameValue = item.name;
    renameError = "";
  }

  function cancelRename() {
    renamingId = null;
    renameValue = "";
    renameError = "";
  }

  async function submitRename() {
    if (!renamingId || renameValue.trim() === "") return;
    const newName = renameValue.trim();
    if (newName === renamingId) {
      cancelRename();
      return;
    }
    renameError = "";
    try {
      await renamePreset(renamingId, newName);
      cancelRename();
      await loadList();
    } catch (e) {
      renameError = String(e);
    }
  }

  function startDelete(item: PresetListItem) {
    if (item.kind !== "user") return;
    deletingId = item.id;
    deleteError = "";
  }

  function cancelDelete() {
    deletingId = null;
    deleteError = "";
  }

  async function confirmDelete() {
    if (!deletingId) return;
    deleteError = "";
    try {
      await deletePreset(deletingId);
      cancelDelete();
      await loadList();
    } catch (e) {
      deleteError = String(e);
    }
  }

  /** UI-1304: Export current settings to a JSON file (save dialog). */
  async function handleExport() {
    importExportError = "";
    try {
      const path = await save({
        defaultPath: "preset.json",
        filters: [{ name: "Preset JSON", extensions: ["json"] }],
      });
      if (path == null) return;
      const name = path.replace(/\.json$/i, "").split(/[/\\]/).pop() ?? "exported";
      exporting = true;
      await savePreset(name, path);
      onListChange?.();
    } catch (e) {
      importExportError = String(e);
    } finally {
      exporting = false;
    }
  }

  /** UI-1304: Import preset from JSON file (open dialog). */
  async function handleImport() {
    importExportError = "";
    try {
      const path = await open({
        multiple: false,
        directory: false,
        filters: [{ name: "Preset JSON", extensions: ["json"] }],
      });
      if (path == null || typeof path !== "string") return;
      importing = true;
      await loadPreset(path);
      onPresetApplied?.();
    } catch (e) {
      importExportError = String(e);
    } finally {
      importing = false;
    }
  }

  import { onMount } from "svelte";
  onMount(() => {
    loadList();
  });
</script>

<div
  class="preset-manager rounded border border-slate-200 bg-slate-50 p-3 flex flex-col gap-2"
  role="group"
  aria-label="Preset manager: list, rename, delete saved presets"
>
  <h3 class="text-xs font-semibold text-slate-600 uppercase tracking-wide">Saved presets</h3>

  {#if loading}
    <p class="text-sm text-slate-500">Loading…</p>
  {:else if error}
    <p class="text-sm text-amber-600" role="alert">{error}</p>
    <Button variant="secondary" on:click={loadList}>Retry</Button>
  {:else if presets.length === 0}
    <p class="text-sm text-slate-500">No saved presets. Save current settings as a preset from the depth controls.</p>
  {/if}

  {#if importExportError}
    <p class="text-sm text-red-600 bg-red-50 border border-red-200 rounded px-2 py-1 mt-1" role="alert">{importExportError}</p>
  {/if}

  <div class="flex flex-wrap gap-2 pt-2 border-t border-slate-200 mt-2">
    <Button
      variant="secondary"
      title="Export current settings to a JSON file"
      disabled={exporting}
      on:click={handleExport}
    >
      {exporting ? "Exporting…" : "Export preset…"}
    </Button>
    <Button
      variant="secondary"
      title="Import preset from a JSON file"
      disabled={importing}
      on:click={handleImport}
    >
      {importing ? "Importing…" : "Import preset…"}
    </Button>
  </div>

  {#if presets.length > 0}
    <ul class="space-y-1.5" role="list">
      {#each presets as item (item.id)}
        <li class="flex items-center gap-2 flex-wrap">
          {#if renamingId === item.id}
            <form
              class="flex items-center gap-1 flex-1 min-w-0"
              on:submit|preventDefault={submitRename}
              role="group"
              aria-label="Rename preset"
            >
              <input
                type="text"
                bind:value={renameValue}
                class="flex-1 min-w-0 text-sm border border-slate-300 rounded px-2 py-1 bg-white"
                placeholder="Preset name"
                aria-label="New preset name"
              />
              <button
                type="submit"
                class="px-2 py-0.5 text-xs rounded border border-slate-300 bg-white text-slate-700 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-slate-400"
              >
                Save
              </button>
              <button
                type="button"
                class="px-2 py-0.5 text-xs rounded border border-slate-300 bg-white text-slate-700 hover:bg-slate-50"
                on:click={cancelRename}
              >
                Cancel
              </button>
            </form>
            {#if renameError}
              <span class="text-xs text-red-600 w-full">{renameError}</span>
            {/if}
          {:else}
            <span class="text-sm text-slate-700 truncate flex-1 min-w-0" title={item.name}>{item.name}</span>
            <button
              type="button"
              class="text-xs text-slate-600 hover:text-slate-800 focus:outline-none focus:ring-2 focus:ring-slate-400 rounded px-1"
              title="Rename preset"
              on:click={() => startRename(item)}
            >
              Rename
            </button>
            <button
              type="button"
              class="text-xs text-red-600 hover:text-red-800 focus:outline-none focus:ring-2 focus:ring-red-400 rounded px-1"
              title="Delete preset"
              on:click={() => startDelete(item)}
            >
              Delete
            </button>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}

  <!-- Delete confirmation -->
  {#if deletingId}
    <div
      class="mt-2 p-2 rounded border border-slate-200 bg-white"
      role="dialog"
      aria-label="Confirm delete preset"
    >
      <p class="text-sm text-slate-700">Delete preset &quot;{deletingId}&quot;?</p>
      {#if deleteError}
        <p class="text-xs text-red-600 mt-1">{deleteError}</p>
      {/if}
      <div class="flex gap-2 mt-2">
        <Button variant="secondary" on:click={confirmDelete}>Delete</Button>
        <Button variant="secondary" on:click={cancelDelete}>Cancel</Button>
      </div>
    </div>
  {/if}
</div>
