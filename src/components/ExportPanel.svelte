<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<script lang="ts">
  /**
   * ExportPanel — Sprint A stub.
   *
   * The 2.5D STL/OBJ export pipeline has been retired alongside `mesh_generator`. The
   * point cloud exporters (PLY/XYZ/CSV, ADR-012) are scheduled for Sprint B (BACK-B-03)
   * and the corresponding frontend wiring lands in Sprint C (UI-C-04). For now this
   * panel preserves its public prop surface so `App.svelte` does not need to change,
   * and shows a disabled "Export point cloud (Sprint B)" button with an explanatory
   * tooltip. The export-format dropdown, settings drawer, and target-size presets
   * have been removed; once point cloud exports land, this panel will be rebuilt
   * around PLY / XYZ / CSV format selection.
   */

  /** Whether a depth map is available. Currently unused — Sprint B will gate the
   *  point cloud export button on this. The props below are surfaced as `data-`
   *  attributes on the root div so they are observable in the DOM and so the
   *  Svelte compiler does not warn about unused exports during Sprint A. */
  export let hasDepth = false;
  /** Source image filename — Sprint B will use this to seed the default export
   *  filename. Retained as a prop to keep `App.svelte` integration stable. */
  export let sourceFileName = "";
  /** Legacy 2.5D target dimensions from App. Sprint C will replace these with
   *  blank-envelope (L×W×H mm) inputs per ADR-012. */
  export let effectiveTargetWidthMm: number | undefined = undefined;
  export let effectiveTargetHeightMm: number | undefined = undefined;
</script>

<div
  class="export-panel flex items-center gap-3"
  role="region"
  aria-label="Export panel"
  data-has-depth={hasDepth}
  data-source-file-name={sourceFileName}
  data-target-width-mm={effectiveTargetWidthMm ?? ""}
  data-target-height-mm={effectiveTargetHeightMm ?? ""}
>
  <button
    type="button"
    class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded border border-slate-300 bg-slate-100 text-sm text-slate-500 cursor-not-allowed"
    disabled
    title="Point cloud export (PLY / XYZ / CSV) is wired in Sprint B (ADR-012)."
    aria-label="Export point cloud — coming in Sprint B"
  >
    Export point cloud (Sprint B)
  </button>
</div>
