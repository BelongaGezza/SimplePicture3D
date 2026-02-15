<script lang="ts">
  /**
   * FirstRunWizard — UI-901 through UI-905, JR1-901/902 (Sprint 1.10).
   * Modal wizard shown on first run when AI model is not installed.
   * Steps: Welcome/privacy → Download → Complete/Skip.
   */
  import Button from "./Button.svelte";
  import { checkModel, downloadModel, getModelInfo } from "$lib/tauri";
  import type { ModelStatus, ModelInfo, DownloadResult } from "$lib/tauri";
  import { onMount, createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ close: void }>();

  /** Whether the wizard is visible. */
  export let visible = false;

  /** Current wizard step. */
  let step: "welcome" | "downloading" | "complete" | "error" | "skipped" = "welcome";
  /** Model status from backend. */
  let modelStatus: ModelStatus | null = null;
  /** Model info for display. */
  let modelInfo: ModelInfo | null = null;
  /** Download error message. */
  let errorMessage = "";
  /** Download result. */
  let downloadResult: DownloadResult | null = null;
  /** Whether check is in progress. */
  let checking = true;

  /** UI-901: Check model status on mount. */
  onMount(async () => {
    try {
      const [status, info] = await Promise.all([checkModel(), getModelInfo()]);
      modelStatus = status;
      modelInfo = info;
      if (status.installed) {
        // Model already installed; no wizard needed
        visible = false;
        dispatch("close");
      }
    } catch (e) {
      errorMessage = "Could not check model status: " + String(e);
    } finally {
      checking = false;
    }
  });

  /** UI-902: Start model download. */
  async function handleDownload() {
    step = "downloading";
    errorMessage = "";
    try {
      const result = await downloadModel();
      downloadResult = result;
      if (result.status === "success") {
        step = "complete";
      } else {
        errorMessage = result.error ?? "Download failed";
        step = "error";
      }
    } catch (e) {
      errorMessage = String(e);
      step = "error";
    }
  }

  /** JR1-902: Skip download with confirmation. */
  function handleSkip() {
    step = "skipped";
  }

  /** Close the wizard. */
  function handleClose() {
    visible = false;
    dispatch("close");
  }

  /** Retry download after error. */
  function handleRetry() {
    step = "welcome";
    errorMessage = "";
  }
</script>

{#if visible && !checking}
  <!-- Modal overlay -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/40"
    role="dialog"
    aria-modal="true"
    aria-label="First run setup wizard"
  >
    <div class="bg-white rounded-xl shadow-2xl w-full max-w-lg mx-4 overflow-hidden">
      <!-- Header -->
      <div class="px-6 pt-6 pb-4 border-b border-slate-200">
        <h2 class="text-lg font-semibold text-slate-800">
          {#if step === "welcome"}
            Welcome to SimplePicture3D
          {:else if step === "downloading"}
            Downloading AI Model
          {:else if step === "complete"}
            Setup Complete
          {:else if step === "error"}
            Download Error
          {:else if step === "skipped"}
            Model Download Skipped
          {/if}
        </h2>
        <!-- JR1-901: Step indicator -->
        <div class="flex gap-1.5 mt-3" aria-label="Wizard progress">
          {#each ["welcome", "downloading", "complete"] as s, i}
            <div
              class="h-1 flex-1 rounded-full {
                step === 'skipped' || step === 'error'
                  ? 'bg-slate-200'
                  : i <= ['welcome', 'downloading', 'complete'].indexOf(step)
                    ? 'bg-slate-700'
                    : 'bg-slate-200'
              }"
              aria-hidden="true"
            />
          {/each}
        </div>
      </div>

      <!-- Body -->
      <div class="px-6 py-5 min-h-[200px] flex flex-col">
        {#if step === "welcome"}
          <!-- UI-904: Privacy notice and model info -->
          <p class="text-sm text-slate-600 mb-4">
            SimplePicture3D uses an AI model for depth estimation. The model needs to be downloaded once before you can generate depth maps.
          </p>

          <div class="bg-slate-50 rounded-lg p-3 mb-4 text-sm">
            <h3 class="font-medium text-slate-700 mb-2">AI Model Details</h3>
            <dl class="space-y-1 text-slate-600">
              <div class="flex justify-between">
                <dt>Model</dt>
                <dd class="text-right font-mono text-xs">{modelInfo?.modelId ?? "Depth-Anything-V2-Small"}</dd>
              </div>
              <div class="flex justify-between">
                <dt>Size</dt>
                <dd>{modelInfo?.estimatedSizeMb ?? 100} MB</dd>
              </div>
              <div class="flex justify-between">
                <dt>License</dt>
                <dd>{modelInfo?.license ?? "CC-BY-NC-4.0"}</dd>
              </div>
            </dl>
          </div>

          <!-- UI-904: Privacy notice -->
          <div class="bg-green-50 border border-green-200 rounded-lg p-3 mb-4 text-sm text-green-800">
            <p class="font-medium mb-1">100% Offline Processing</p>
            <p>After download, all depth estimation runs locally on your machine. No images are sent to any cloud service.</p>
          </div>

          {#if modelStatus?.missingFiles?.length}
            <p class="text-xs text-slate-500 mb-4">
              Missing files: {modelStatus.missingFiles.join(", ")}
            </p>
          {/if}

          <div class="flex gap-3 mt-auto pt-4">
            <Button variant="primary" on:click={handleDownload}>
              Download Model
            </Button>
            <button
              type="button"
              class="px-4 py-2 text-sm text-slate-600 hover:text-slate-800 hover:bg-slate-100 rounded focus:outline-none focus:ring-2 focus:ring-slate-500"
              on:click={handleSkip}
            >
              Skip for now
            </button>
          </div>

        {:else if step === "downloading"}
          <!-- UI-903: Download progress -->
          <div class="flex flex-col items-center justify-center flex-1 gap-4">
            <svg class="animate-spin h-10 w-10 text-slate-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" aria-hidden="true">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
            </svg>
            <p class="text-sm text-slate-600">Downloading AI model from Hugging Face...</p>
            <p class="text-xs text-slate-400">This may take a few minutes depending on your connection.</p>
            <!-- Indeterminate progress bar -->
            <div class="w-full h-2 bg-slate-200 rounded-full overflow-hidden">
              <div class="wizard-progress-bar h-full bg-slate-600 rounded-full"></div>
            </div>
          </div>

        {:else if step === "complete"}
          <!-- Download complete -->
          <div class="flex flex-col items-center justify-center flex-1 gap-4">
            <div class="w-14 h-14 rounded-full bg-green-100 flex items-center justify-center">
              <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <p class="text-sm text-slate-700 font-medium">AI Model installed successfully!</p>
            {#if downloadResult?.sizeMb}
              <p class="text-xs text-slate-500">{downloadResult.sizeMb} MB downloaded to model directory</p>
            {/if}
            <Button variant="primary" on:click={handleClose}>
              Get Started
            </Button>
          </div>

        {:else if step === "error"}
          <!-- Download error -->
          <div class="flex flex-col items-center justify-center flex-1 gap-4">
            <div class="w-14 h-14 rounded-full bg-red-100 flex items-center justify-center">
              <svg class="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </div>
            <p class="text-sm text-red-700 font-medium">Download failed</p>
            <p class="text-xs text-red-600 text-center max-w-sm break-words">{errorMessage}</p>
            <div class="flex gap-3">
              <Button variant="primary" on:click={handleRetry}>
                Try Again
              </Button>
              <button
                type="button"
                class="px-4 py-2 text-sm text-slate-600 hover:text-slate-800 hover:bg-slate-100 rounded focus:outline-none focus:ring-2 focus:ring-slate-500"
                on:click={handleClose}
              >
                Close
              </button>
            </div>
          </div>

        {:else if step === "skipped"}
          <!-- JR1-902: Skip confirmation -->
          <div class="flex flex-col items-center justify-center flex-1 gap-4">
            <div class="w-14 h-14 rounded-full bg-amber-100 flex items-center justify-center">
              <svg class="w-8 h-8 text-amber-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4.5c-.77-.833-2.694-.833-3.464 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z" />
              </svg>
            </div>
            <p class="text-sm text-slate-700 font-medium">Model download skipped</p>
            <p class="text-xs text-slate-500 text-center max-w-sm">
              Depth estimation will not work until the model is downloaded. You can download it later from the application menu.
            </p>
            <Button variant="primary" on:click={handleClose}>
              Continue Without Model
            </Button>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .wizard-progress-bar {
    width: 40%;
    animation: wizard-slide 1.5s ease-in-out infinite;
  }
  @keyframes wizard-slide {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(350%);
    }
  }
</style>
