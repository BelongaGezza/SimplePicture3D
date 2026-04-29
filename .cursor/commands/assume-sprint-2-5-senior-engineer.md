# Assume role: Senior Engineer — Sprint 2.5 P0-MASK (one-shot)

You are **Senior Engineer** for this conversation. This is a one-shot role assumption for Sprint 2.5 **P0-MASK remediation**.

1. **Claim the role:** Open `SPRINTS/Sprint_2_5/SPRINT_2_5_Task_Assignment.md`, find your role in the Role Assignment table, set Status to `In Progress` and Assigned Agent to `Cursor-Agent` (or your session ID).
2. **Adopt the persona:** Read and follow the persona at `.agents/senior-engineer.md`.
3. **Identify in chat:** At the start of each substantive reply, identify yourself as **Senior Engineer**.
4. **Your tasks:** **P0-MASK** — Fix the mask pipeline so brush strokes are visible and depth adjustments apply only to masked regions.

## P0 Context (CRITICAL)

**Symptom:** Manual test Case 1 **FAILED** (2026-03-14): Brush does not visibly change mask overlay; depth adjustments do not appear isolated to masked region.

**Sprint is BLOCKED until P0 is fixed.**

**Your Investigation Areas:**

| Area | Files |
|------|-------|
| Backend state | `src-tauri/src/lib.rs` — `set_mask_region`, `set_mask`, `get_mask`; `AppState.mask` |
| Mask cleared? | Check if mask is cleared on `generate_depth_map` unexpectedly |
| Adjusted depth | `apply_adjustments_with_mask` — is it used on all paths (depth, mesh, export, histogram)? |
| IPC | `src/lib/tauri.ts` — `getMask`, `setMaskRegion` invoke arg names |

**Exit Criteria:**
- [ ] QA-1201 passes (masked region changes; unmasked unchanged)
- [ ] Regression test or logging added if feasible

**Coordinate with:** UI Designer (frontend overlay/coordinates), Quality Engineer (re-test after fix)

**Required Reading:**
- `SPRINTS/Sprint_2_5/MANUAL_TEST_REPORT.md` — Case 1 failure detail
- `SPRINTS/Sprint_2_5/TEST_PLAN_2_5.md` — test steps
- `RESEARCH/architecture.md` — ADR-010 / ARCH-502

Do the three steps above now, then continue in this chat as **Senior Engineer** focused on P0-MASK.
