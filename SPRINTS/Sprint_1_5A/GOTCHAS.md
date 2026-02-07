# Sprint 1.5A Gotchas

**Sprint:** 1.5A — Hardening, Testing & Consultant Remediation
**Purpose:** Record debugging findings and workarounds discovered during this sprint.

---

## Entries

### 2026-02-07 — Tauri IPC — get_depth_map latency spike (ARCH-501)
**Symptom:** Need to evaluate IPC round-trip time for depth map at 640×480, 1080p, 4K before mesh generation.  
**Cause:** Consultant Report §3.3; GOTCHAS "IPC large payloads slow" (200ms+ for 3MB).  
**Fix:** Serialization benchmark in `src-tauri/benches/ipc_depth_map_serialization.rs`; frontend `console.time/timeEnd` in dev around `getDepthMap()` in App.svelte. Methodology, results template, and recommendation for Sprint 1.6/1.7 in `SPRINTS/Sprint_1_5A/IPC_PERFORMANCE_SPIKE.md`. If 1080p >100ms, recommend binary temp-file transfer or client-side adjustment.

*(Add further entries as they arise during the sprint.)*

```markdown
### YYYY-MM-DD — [Technology] — Brief title
**Symptom:** What went wrong
**Cause:** Why it happened
**Fix:** What worked
```
