# Sprint 2.6 — Cancelled (Superseded)

**Task:** QA-D-08  
**Status:** Cancelled — do not execute  
**Last Updated:** 2026-05-24

---

## Summary

Sprint 2.6 (**Enhanced 3D Preview** — lighting, measurements, cross-sections, preview export) is **cancelled** and must not be started. Work planned under `SPRINTS/Sprint_2_6/SPRINT_2_6_Task_Assignment.md` is superseded by the **2026-05-10 product restart** and the **ADR-012 surface-map point cloud** pivot.

---

## Why cancelled

| Date / event | Impact |
|--------------|--------|
| **2026-05-10** | Algorithm restart: column-sweep volumetric fill deprecated; surface-map (one point per sample) is canonical (`RESEARCH/architecture.md` ADR-012, `RESEARCH/GOTCHAS.md` 2026-05-10) |
| **Phases A–C** | Production path is point cloud + PLY/XYZ/CSV export, not 2.5D relief mesh / STL-centric preview enhancements |
| **Phase D** | Active validation sprint: engraver export, security hardening (`SPRINTS/Sprint_D/`) |
| **Sprint 2.6 scope** | Targeted v0.6.0-beta “enhanced preview” (PRD §F2.5) tied to pre-pivot mesh preview assumptions |

---

## What to use instead

| Former Sprint 2.6 intent | Current direction |
|--------------------------|-------------------|
| Enhanced 3D preview (lighting, measurements) | Three.js point cloud + blank wireframe in Phase C UI; future UX in roadmap post–v1.0 beta |
| Cross-sections / preview export | Deferred; track in `ROADMAP.md` / `todo.md`, not Sprint 2.6 |
| Mesh bounds for QA | ADR-012 point cloud bounds via `generate_point_cloud` / export tests (QA-B-01) |

---

## Artefacts

| Path | Action |
|------|--------|
| `SPRINTS/Sprint_2_6/SPRINT_2_6_Task_Assignment.md` | **Historical reference only** — do not assign tasks |
| `SPRINTS/Sprint_2_6/TEST_PLAN_2_6.md` | Do not create |
| `SPRINTS/Sprint_2_6/VERIFICATION_CHECKLIST.md` | Do not create |

For active QA planning, use **`SPRINTS/Sprint_D/TEST_PLAN_D.md`** and **`docs/testing.md`**.

---

## References

- `RESEARCH/architecture.md` — ADR-012 (canonical)
- `RESEARCH/GOTCHAS.md` — 2026-05-10 column-sweep advisory
- `ROADMAP.md` — Phase D Validate & Harden
- `todo.md` — Phase milestones; Sprint 2.6 tasks marked superseded
- `SPRINTS/Sprint_D/ORCHESTRATION.md` — current sprint orchestration

---

**Document Version:** 1.0  
**Author:** Quality Engineer (QA-D-08)
