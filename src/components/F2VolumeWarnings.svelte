<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<script lang="ts">
  /**
   * Non-blocking F2 Ultra UV volume warnings for crystal blank / etch interior.
   */
  import {
    evaluateF2VolumeWarnings,
    formatF2ExceedMessage,
    formatF2NearEdgeMessage,
  } from "$lib/f2VolumeWarnings";

  export let lengthMm = 50;
  export let widthMm = 50;
  export let heightMm = 80;
  export let marginMm = 2;
  /** `compact` uses smaller text for preview overlay. */
  export let compact = false;

  $: result = evaluateF2VolumeWarnings(lengthMm, widthMm, heightMm, marginMm);
  $: exceedMessage = formatF2ExceedMessage(result.exceeds);
  $: nearEdgeMessage = formatF2NearEdgeMessage(result.nearEdge);

  const boxClass = compact ? "text-xs px-2 py-1" : "text-sm px-3 py-2";
</script>

{#if exceedMessage}
  <div
    class="f2-volume-warning f2-volume-warning--exceed rounded border border-amber-300 bg-amber-50 text-amber-950 {boxClass}"
    role="status"
    data-warning="f2-exceed"
  >
    {exceedMessage}
  </div>
{/if}

{#if nearEdgeMessage}
  <div
    class="f2-volume-warning f2-volume-warning--near-edge rounded border border-yellow-300 bg-yellow-50 text-yellow-950 {boxClass}"
    role="status"
    data-warning="f2-near-edge"
  >
    {nearEdgeMessage}
  </div>
{/if}
