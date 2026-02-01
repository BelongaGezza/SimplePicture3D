# Svelte Onboarding Notes (JR1-004)

**Sprint:** 1.1  
**Task:** JR1-004  
**Role:** Junior Engineer 2D  
**Last Updated:** 2026-02-01

---

## Purpose

Key takeaways from Svelte tutorial and project usage for onboarding. Use when implementing components in Sprint 1.2+.

---

## Key Concepts (Tutorial Recap)

1. **Components** — `.svelte` files: `<script>`, markup, `<style>`. Use `export let prop` for props; `on:event` for forwarding.
2. **Reactivity** — `$:` for derived state; assignments trigger updates. Prefer explicit assignments over mutation.
3. **Events** — `on:click`, `on:submit`, etc. Forward with `on:click` (no args) from child so parent receives the event.
4. **Slots** — `<slot />` for content; named slots with `<slot name="...">` when needed.
5. **Stores** — `writable`, `readable`, `derived` from `svelte/store` for shared state across components.
6. **TypeScript** — Use `lang="ts"` in `<script>`; type props and event handlers for better handover to backend.

---

## Project Conventions (SimplePicture3D)

| Area | Convention |
|------|------------|
| **Components** | `src/components/`; PascalCase filenames (e.g. `Button.svelte`, `ImageImport.svelte`). |
| **Lib** | `src/lib/` for Tauri invoke helpers and shared utilities (e.g. `tauri.ts`). |
| **Styling** | TailwindCSS only; no component-scoped CSS unless necessary. Use `class="..."` with Tailwind classes. |
| **IPC** | Call `invoke()` via `$lib/tauri`; use typed args and return types (JR1-003). |
| **Props** | Type callback props, e.g. `export let onLoad: (path: string) => void`. |
| **Buttons** | Use shared `Button.svelte` (primary/secondary) for Load, Export, and future actions. |

---

## Useful Svelte + Tauri Patterns

- **Invoke in parent:** Keep `invoke` in `App.svelte` (or a parent); pass down handlers like `onLoad`, `onExport` so components stay presentational.
- **Status bar:** Single reactive `status` string in App; set from async handlers after `loadImage` / `exportStl`.
- **Slots:** Button content via default slot: `<Button>Export</Button>`.

---

## Tutorial Completion

- [x] Svelte tutorial concepts reviewed and applied in JR1-002 (Button, ImageImport), JR1-003 (types in tauri.ts).
- [x] Ready to implement further components in Sprint 1.2 (e.g. depth controls, preview wiring).

---

## References

- [Svelte Tutorial](https://svelte.dev/tutorial)
- `RESEARCH/frontend.md` — Framework guidance
- `prd.md` F1.1, F1.4, F1.7 — UI requirements

**Completion Record:** 2026-02-01 — Junior Engineer 2D. Documented Svelte concepts and project conventions; Button and IPC types implemented. Ready for Sprint 1.2 component work.
