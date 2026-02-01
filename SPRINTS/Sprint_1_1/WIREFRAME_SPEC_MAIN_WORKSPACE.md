# Wireframe Spec — Main Workspace (F1.7)

**Sprint:** 1.1  
**Task:** JR1-001  
**Role:** Junior Engineer 2D (with UI Designer)  
**Source:** prd.md F1.7 Basic UI/UX  
**Last Updated:** 2026-02-01

---

## Purpose

This document defines the main workspace layout for SimplePicture3D so that:
- Figma wireframes can be created from it (or this spec serves as the shared wireframe deliverable).
- Implementation (App.svelte and components) stays aligned with PRD F1.7.

---

## PRD F1.7 Acceptance Criteria (Reference)

- Single-window application
- **Left sidebar:** image import and controls
- **Center:** preview panel
- **Right sidebar:** depth adjustment controls
- **Bottom:** export button and status bar
- Window resizable, minimum 1024×768  
- Responsive layout (CSS Grid/Flexbox)
- Keyboard shortcuts for common actions
- Tooltips on hover for all controls

---

## Layout Wireframe (ASCII)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ SimplePicture3D                                                    [_][□][X]   │
├──────────────┬──────────────────────────────────────────────┬───────────────────┤
│              │                                              │                   │
│  IMAGE       │              3D PREVIEW                      │  DEPTH            │
│              │                                              │                   │
│  [Load]      │   ┌────────────────────────────────────┐    │  (Depth range     │
│  [Drop zone] │   │                                    │    │   sliders,        │
│              │   │     Mesh / 3D viewport              │    │   gamma, invert   │
│  (preview    │   │     (orbit: rotate/zoom/pan)       │    │   — placeholder   │
│   thumbnail) │   │                                    │    │   for Sprint 1.2) │
│              │   └────────────────────────────────────┘    │                   │
│              │                                              │                   │
├──────────────┴──────────────────────────────────────────────┴───────────────────┤
│  Status: Ready                    [ Export STL ]                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Minimum window:** 1024×768. Left/right sidebars fixed or min width; center flexes.

---

## Zones (for Figma / Implementation)

| Zone | Content | PRD / Notes |
|------|---------|-------------|
| **Left sidebar** | Section heading "Image"; Load button; drop zone; optional image thumbnail | F1.1 Image Import |
| **Center** | Section heading "3D Preview"; viewport (Three.js mesh, orbit controls) | F1.4 3D Preview |
| **Right sidebar** | Section heading "Depth"; depth adjustment controls (sliders, etc.) | F1.3 Manual Depth Adjustment |
| **Bottom bar** | Status text (left); Export button (right) | F1.6 Export; F1.7 status bar |

---

## Key Elements to Include in Figma (If Created)

1. **Left sidebar**
   - "Image" heading (small, uppercase, muted)
   - Primary "Load" button (opens file picker)
   - Dashed-border drop zone: "Drop image or click to load"
   - Optional: small thumbnail after load

2. **Center**
   - "3D Preview" heading
   - Single canvas/viewport area (aspect ratio ~16:10 or flexible)
   - Placeholder or simple cube for wireframe stage

3. **Right sidebar**
   - "Depth" heading
   - Placeholder blocks for: depth range (2–10 mm), gamma, invert (Sprint 1.2)

4. **Footer**
   - Left: status message (e.g. "Ready", "Loaded", "Exporting…")
   - Right: "Export" or "Export STL" button

5. **Global**
   - Single window chrome (title, min 1024×768)
   - Tooltips on hover (call out in wireframe: "Tooltips on all controls")
   - Keyboard shortcuts (call out: "e.g. Ctrl+O Load, Ctrl+E Export")

---

## Alignment with Current App

The current `App.svelte` layout matches this spec:
- Left aside: Image heading + `ImageImport` (Load stub, drop placeholder)
- Center: 3D Preview heading + `Preview3D`
- Right aside: Depth heading + "Controls area" placeholder
- Footer: status + Export button

Figma wireframes based on this spec should align with the existing structure for handoff to UI Designer and future sprints.

---

## Completion

- [x] Wireframe spec created and shared (this document in repo)
- [x] Aligns with PRD single-window, sidebar, preview, controls

**Completion Record:** 2026-02-01 — Junior Engineer 2D. Created `SPRINTS/Sprint_1_1/WIREFRAME_SPEC_MAIN_WORKSPACE.md` per JR1-001. Spec defines layout per F1.7, ASCII wireframe, zone table, and Figma-oriented element list. Implementation in App.svelte already aligned.
