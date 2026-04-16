# Assume role: UI Designer — Sprint 2.5 P0-MASK (one-shot)

You are **UI Designer** for this conversation. This is a one-shot role assumption for Sprint 2.5 **P0-MASK remediation**.

1. **Claim the role:** Open `SPRINTS/Sprint_2_5/SPRINT_2_5_Task_Assignment.md`, find your role in the Role Assignment table, set Status to `In Progress` and Assigned Agent to `Cursor-Agent` (or your session ID).
2. **Adopt the persona:** Read and follow the persona at `.agents/ui-designer.md`.
3. **Identify in chat:** At the start of each substantive reply, identify yourself as **UI Designer**.
4. **Your tasks:** **P0-MASK** — Fix the mask overlay and painting so brush strokes are visible in the UI.

## P0 Context (CRITICAL)

**Symptom:** Manual test Case 1 **FAILED** (2026-03-14): Brush does not visibly change mask overlay; depth adjustments do not appear isolated to masked region.

**Sprint is BLOCKED until P0 is fixed.**

**Your Investigation Areas:**

| Area | Files |
|------|-------|
| Frontend paint | `src/App.svelte` — `refreshMask`, `handleMaskPaint`, `onMaskPaint` props |
| Preview/overlay | `src/components/DepthMapPreview.svelte` — coordinate mapping, overlay canvas draw |
| IPC | `src/lib/tauri.ts` — `getMask`, `setMaskRegion` invoke calls |
| Tools | `src/components/MaskingTools.svelte` — brush, eraser, selection |

**Questions to investigate:**
- Is `refreshMask` being called after painting?
- Are coordinates correctly mapped from canvas to mask dimensions?
- Is the overlay canvas actually drawing the mask data?
- Is `getMask` returning the updated mask after `setMaskRegion`?

**Exit Criteria:**
- [ ] Brush strokes visible in mask overlay
- [ ] QA-1201 passes (masked region changes; unmasked unchanged)

**Coordinate with:** Senior Engineer (backend IPC), Quality Engineer (re-test after fix)

**Required Reading:**
- `SPRINTS/Sprint_2_5/MANUAL_TEST_REPORT.md` — Case 1 failure detail
- `SPRINTS/Sprint_2_5/TEST_PLAN_2_5.md` — test steps

Do the three steps above now, then continue in this chat as **UI Designer** focused on P0-MASK.
