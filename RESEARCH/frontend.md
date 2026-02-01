# Frontend Stack (Svelte/React, TypeScript, TailwindCSS)

**Purpose:** Frontend framework guidance for SimplePicture3D UI.

## Official Sources

| Technology | URL | Last Checked |
|------------|-----|--------------|
| Svelte | https://svelte.dev/docs | — |
| React | https://react.dev/ | — |
| TypeScript | https://www.typescriptlang.org/docs/ | — |
| TailwindCSS | https://tailwindcss.com/docs | — |
| Tauri invoke | https://tauri.app/v1/api/js/tauri/#invoke | — |

*Researcher: Update "Last Checked" when verifying. Confirm project choice: Svelte vs React.*

---

## Project Usage

- **Framework:** Svelte or React + TypeScript
- **Styling:** TailwindCSS or CSS Modules
- **State:** Svelte stores or Zustand (React)
- **IPC:** `@tauri-apps/api` → `invoke('command_name', { ... })`

---

## Layout (prd.md §6.3)

- Left: Image import, preview
- Center: 3D preview (Three.js)
- Right: Depth controls (sliders, invert, Generate)
- Bottom: Export button, status bar

---

## Research Tasks (Researcher)

- [ ] Svelte vs React: project decision and rationale
- [ ] TailwindCSS v4 changes if applicable
- [ ] TypeScript strict mode and Tauri types
- [ ] Vite config for Tauri (if used)
- [ ] Version changes, deprecations since knowledge cutoff
