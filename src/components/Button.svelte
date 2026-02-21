<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<script lang="ts">
  /**
   * Reusable button component (JR1-002). Styled with TailwindCSS.
   * Used for Load and Export actions; triggers IPC via parent handlers.
   */
  export let variant: "primary" | "secondary" = "primary";
  export let disabled = false;
  export let type: "button" | "submit" = "button";
  export let title = "";

  const base =
    "inline-flex items-center justify-center rounded font-medium focus:outline-none focus:ring-2 focus:ring-offset-2 transition-colors disabled:opacity-50 disabled:pointer-events-none";
  const primary =
    "px-4 py-2 bg-slate-700 text-white text-sm hover:bg-slate-800 focus:ring-slate-500";
  const secondary =
    "px-3 py-1.5 bg-slate-600 text-white text-xs hover:bg-slate-700 focus:ring-slate-500";
  const classes = variant === "primary" ? primary : secondary;
</script>

<button
  {type}
  {disabled}
  {title}
  class="{base} {classes}"
  on:click
>
  <slot />
</button>
