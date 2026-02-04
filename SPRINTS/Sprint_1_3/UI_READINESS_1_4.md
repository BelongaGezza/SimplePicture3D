# Sprint 1.4 UI Readiness (UI Designer handover)

**Purpose:** Prep for Sprint 1.4 depth UI. No dedicated Sprint 1.3 tasks for UI Designer; this doc captures components and states so implementation can start as soon as `generate_depth_map` is available.

**Source:** prd.md F1.2 (AI Depth Estimation), F1.7 (Basic UI/UX), §6 layout; docs/architecture.md § Frontend implications (depth pipeline).

---

## Layout (already in place)

- **Left sidebar:** Image import, preview, metadata (ImageImport.svelte — done in 1.2)
- **Center:** 3D preview (Preview3D.svelte — placeholder)
- **Right sidebar:** Depth controls — currently placeholder ("Controls area"); will hold Generate button, progress, sliders (gamma, range, invert)
- **Footer:** Status bar, Export button

---

## Components to add or extend (Sprint 1.4)

| Component | Purpose | Notes |
|-----------|---------|--------|
| **Progress indicator** | Show % and optional stage during depth estimation | F1.2: "Display progress indicator during inference". Consume progress from `generate_depth_map` (or future event/channel). |
| **Error display** | Show backend errors (missing Python, model, timeout, OOM) | Use status bar or inline in right sidebar; no panic; clear, copyable message. |
| **Depth controls panel** | Replace "Controls area" placeholder | Generate button; after depth ready: sliders (gamma, range), invert toggle (F1.3). |
| **Depth preview** (optional for 1.4) | 2D grayscale depth map view | F1.4 / F2.5; can follow after mesh preview if scope tight. |
| **3D preview** | Show mesh from depth | Preview3D already exists; will receive vertex/data from `get_mesh_data` when mesh pipeline is ready. |

---

## UI states (depth flow)

1. **Idle** — Image loaded or not; no depth run in progress.
2. **Estimating** — `generate_depth_map` in progress; show progress % (and optional stage).
3. **Depth ready** — Depth map received; show depth controls; enable mesh preview when implemented.
4. **Error** — Show message (timeout, missing model, Python error); allow retry.

---

## Contract from backend (when ready)

- **Progress:** Rust will parse Python stderr and expose progress (0–100, optional stage) — see docs/architecture.md § Frontend implications. For 1.4, progress may be returned on command completion (polling) or via a channel/event; TBD with Senior Engineer.
- **Errors:** `generate_depth_map` returns `Result`; frontend displays `Err` message in status bar or dedicated error area.
- **Result:** Depth map (dimensions + normalized float array) for preview and depth controls.

---

## Accessibility & quality (prd §6)

- Keyboard nav for Generate and depth controls
- Focus indicators; screen-reader labels for progress and errors
- Theme-aware colors; no sensitive data in progress text

---

**Document owner:** UI Designer (Sprint 1.3). Update when BACK-205 / Tauri command design is finalized.
