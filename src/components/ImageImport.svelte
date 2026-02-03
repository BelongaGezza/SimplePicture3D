<script lang="ts">
  /**
   * ImageImport — file picker and drop zone for loading images (UI-101, UI-102, UI-105).
   * Calls load_image with selected path; shows loading spinner and error/success feedback.
   */
  import Button from "./Button.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { loadImage, type LoadImageResult } from "$lib/tauri";

  export let loading = false;
  export let errorMessage = "";
  /** Called with path when user picks a file (before load). Used for loading state. */
  export let onLoadStart: (path: string) => void = () => {};
  /** Called with result on success. */
  export let onLoadSuccess: (result: LoadImageResult) => void = () => {};
  /** Called with message on failure. */
  export let onLoadError: (message: string) => void = () => {};
  const ACCEPT_EXTENSIONS = ["png", "jpg", "jpeg"];

  let isDragOver = false;

  async function loadPath(path: string) {
    if (!path.trim()) return;
    onLoadStart(path);
    try {
      const res = await loadImage(path);
      if (res.ok) {
        onLoadSuccess(res);
      } else {
        onLoadError(res.message ?? "Failed to load image");
      }
    } catch (e) {
      onLoadError(String(e));
    }
  }

  async function handleLoadClick() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Images", extensions: ACCEPT_EXTENSIONS }],
    });
    if (selected === null || typeof selected !== "string") return;
    await loadPath(selected);
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
    const file = e.dataTransfer?.files?.[0];
    if (!file) return;
    const path = (file as File & { path?: string }).path;
    if (path) {
      const ext = path.split(".").pop()?.toLowerCase();
      if (ext && ACCEPT_EXTENSIONS.includes(ext)) {
        loadPath(path);
      } else {
        onLoadError("Unsupported format. Use PNG or JPG.");
      }
    } else {
      onLoadError("Could not get file path. Try using the Load button.");
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragOver = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragOver = false;
  }
</script>

<div
  class="image-import rounded border-2 border-dashed p-4 flex flex-col items-center justify-center gap-3 min-h-[200px] text-sm transition-colors {isDragOver
    ? 'border-slate-500 bg-slate-100'
    : 'border-slate-200 hover:border-slate-300'} {errorMessage
    ? 'border-red-300 bg-red-50/50'
    : 'text-slate-500'}"
  role="region"
  aria-label="Image import: drop zone and load button"
  on:drop={handleDrop}
  on:dragover={handleDragOver}
  on:dragleave={handleDragLeave}
>
  {#if loading}
    <div class="flex flex-col items-center gap-2" aria-live="polite">
      <svg
        class="animate-spin h-8 w-8 text-slate-500"
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        aria-hidden="true"
      >
        <circle
          class="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          stroke-width="4"
        />
        <path
          class="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
        />
      </svg>
      <span class="text-slate-600">Loading…</span>
    </div>
  {:else}
    <Button
      variant="primary"
      title="Load image (opens file picker)"
      disabled={loading}
      on:click={handleLoadClick}
    >
      Load
    </Button>
    <span class="text-slate-500">
      {isDragOver ? "Drop image here" : "Drop image or click to load"}
    </span>
  {/if}
  {#if errorMessage}
    <p class="text-red-600 text-center text-xs mt-1" role="alert">
      {errorMessage}
    </p>
  {/if}
</div>
