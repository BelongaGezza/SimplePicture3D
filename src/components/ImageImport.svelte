<script lang="ts">
  import Button from "./Button.svelte";
  export let onLoad: (path: string) => void = () => {};
</script>

<div class="rounded border-2 border-dashed border-slate-200 p-4 flex flex-col items-center justify-center gap-2 min-h-[200px] text-slate-500 text-sm hover:border-slate-300 transition-colors">
  <span class="font-medium">ImageImport</span>
  <span>Load or drop image (placeholder)</span>
  <Button variant="secondary" title="Load image (opens file picker when wired)" on:click={() => onLoad("")}>
    Load
  </Button>
</div>
