# Frontend Stack (Svelte/React, TypeScript, TailwindCSS)

**Purpose:** Frontend framework guidance for SimplePicture3D UI.

## Official Sources

| Technology | URL | Last Checked |
|------------|-----|--------------|
| Svelte | https://svelte.dev/docs | 2026-02-01 |
| React | https://react.dev/ | 2026-02-01 |
| TypeScript | https://www.typescriptlang.org/docs/ | 2026-02-01 |
| TailwindCSS | https://tailwindcss.com/docs | 2026-02-01 |
| Vite | https://vite.dev/ | 2026-02-01 |
| Tauri v2 Frontend / Vite | https://v2.tauri.app/start/frontend/vite | 2026-02-01 |
| Tauri invoke (v2) | https://v2.tauri.app/reference/javascript/api/ | 2026-02-01 |

---

## Svelte vs React (Recommendation)

- **Tauri:** Framework-agnostic; both Svelte and React work with `create-tauri-app` and Vite.
- **Recommendation for SimplePicture3D:** **Svelte** — smaller bundle, less boilerplate, reactive bindings fit sliders and preview state; single-page app without SSR. Alternatively **React** if the team prefers React ecosystem (Zustand, etc.). Document choice in sprint/architecture; both are supported by Tauri v2 and Vite.
- **Rationale (Svelte):** Less runtime, simple stores, good fit for a desktop tool with 3D preview and controls. **Rationale (React):** Larger ecosystem, familiar to many developers. Choose one and use consistently (e.g. Svelte + Svelte stores, or React + Zustand).

---

## TypeScript

- Use **TypeScript** for frontend (per PRD). Enable **strict** mode in `tsconfig.json`.
- Bundler: **Vite** (recommended for Tauri v2); Tauri dev server points at Vite dev server (e.g. `http://localhost:5173`).

---

## TailwindCSS

- **v3 vs v4:** Tailwind v4 is available (2025) with CSS-first config, `@tailwindcss/vite` plugin, and no `tailwind.config.js` required in many setups. v3 remains widely used and stable.
- **With Vite:** Use `@tailwindcss/vite` for v4, or PostCSS + `tailwindcss` for v3. Add to Vite config and main CSS (e.g. `@import "tailwindcss"` or `@tailwind base/components/utilities`).
- **Last checked:** 2026-02-01; confirm exact plugin names and config in Tailwind docs when scaffolding.

---

## Project Usage

- **Framework:** Svelte or React + TypeScript (choose one; recommend Svelte).
- **Styling:** TailwindCSS (v3 or v4 with Vite).
- **State:** Svelte stores or Zustand (React).
- **IPC:** `@tauri-apps/api` → `invoke('command_name', { ... })` (Tauri v2; ensure shell/frontend permissions in capabilities if needed).

---

## Layout (prd.md §6.3)

- Left: Image import, preview.
- Center: 3D preview (Three.js).
- Right: Depth controls (sliders, invert, Generate).
- Bottom: Export button, status bar.

---

## Research Tasks (Researcher)

- [x] Svelte vs React: **recommend Svelte** with rationale; React acceptable — documented above.
- [x] Tailwind v3/v4 and Vite — documented above.
- [x] TypeScript strict mode and Tauri types — use strict; types from `@tauri-apps/api`.
- [x] Vite config for Tauri — use Vite; Tauri points at dev server and `dist`.
- [x] Official sources and Last checked — in table above.
