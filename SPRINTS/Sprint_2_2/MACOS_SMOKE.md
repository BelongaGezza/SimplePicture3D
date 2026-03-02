# Sprint 2.2 — macOS smoke tests (TD-05 / QA-1402)

**Purpose:** Start macOS smoke testing per Technical Debt TD-05.  
**Last Updated:** 2026-03-01  
**Owner:** Quality Engineer

---

## Status

| Item | Value |
|------|--------|
| **Smoke run** | **Not yet run** |
| **Reason** | No macOS build/environment available in current Sprint 2.2 context. |
| **Plan** | Document environment, build steps, and checklist below; execute when macOS target is available (e.g. Phase 3 / Sprint 3.1). |

---

## Environment (when run)

| Item | Value |
|------|--------|
| macOS version | *(e.g. Ventura 14.x, Sonoma 15.x)* |
| Hardware | *(Intel or Apple Silicon)* |
| Node | *(e.g. 20 LTS)* |
| Rust | `rustup target add aarch64-apple-darwin` or `x86_64-apple-darwin` as needed |
| Python | 3.x, venv; stub mode for tests |

---

## Build steps (macOS)

1. Clone repo (or pull latest).
2. Install dependencies: Node, Rust, Python per project README/CLAUDE.md.
3. Frontend: `npm install`.
4. Tauri macOS build: `npm run tauri build` (or `cargo build` with macOS target).
5. Run: `npm run tauri dev` or launch built app from `src-tauri/target/release/bundle/`.

*If cross-compiling from non-macOS host, document limitations (e.g. need macOS for DMG/code signing).*

---

## Smoke checklist (short)

When macOS build is available, run and check:

| # | Step | Pass / Fail | Notes |
|---|------|-------------|-------|
| 1 | Launch app | [ ] | |
| 2 | Load image (file picker) | [ ] | |
| 3 | Generate depth (stub or real model) | [ ] | |
| 4 | Adjust depth (slider or curve) | [ ] | |
| 5 | Undo/Redo (if implemented) | [ ] | |
| 6 | Preview 3D mesh | [ ] | |
| 7 | Export STL or OBJ | [ ] | |
| 8 | No crash / no blocking errors | [ ] | |

---

## References

- **TD-05:** `todo.md` — Technical Debt Register (macOS/Linux manual testing)
- **Sprint 2.2:** `SPRINTS/Sprint_2_2/SPRINT_2_2_Task_Assignment.md` — QA-1402
- **Phase 3 macOS:** `todo.md` — Sprint 3.1 (macOS build & testing)

---

**Document Version:** 1.0  
**Acceptance (QA-1402):** Document in sprint folder; either smoke run completed or deferred with clear plan. **Status:** Deferred with plan above.
