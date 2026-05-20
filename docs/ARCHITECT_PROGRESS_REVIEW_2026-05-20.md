# Architect Progress Review — Sprint D Wave 4 Closeout

**Date:** 2026-05-20  
**Reviewer:** System Architect  
**Sprint:** D — Validate and Harden (Phase D)  
**Prior review:** `docs/ARCHITECT_REVIEW_2026-05.md` (takeover, 2026-05-19)  
**Orchestration board:** `SPRINTS/Sprint_D/ORCHESTRATION.md`

---

## Executive summary

Sprint D is **~70% complete by task count** and **~40% complete by production gate**. Engineering velocity on security and architecture artefacts is strong; the **critical path is now human validation** (QA-D-01 engraver import) plus **SEC-D-01** ACL split and **Wave 5 closeout** (docs, sign-off, CI verification of QA-B-01).

**Phase D gate status:** **Not passed** — engraver E2E not recorded; `SECURITY_SIGNOFF.md` unsigned; SEC-D-01 pending.

**Recommendation:** Freeze feature scope. Execute Wave 5 in parallel: (1) Senior Engineer — SEC-D-01 + BACK-D-01 standby; (2) Quality Engineer — QA-D-01 on XTool Studio; (3) Documentation + Security — DOC-D-01/DOC-D-02 + sign-off; (4) CI/shell — confirm QA-B-01 via `cargo test` on Windows runner or after Linux GTK install.

---

## Progress since takeover (2026-05-19)

| Track | Tasks | Status | Evidence |
|-------|-------|--------|----------|
| Engraver charter | ARCH-D-01 | **Complete** | `docs/engraver-charter.md` |
| Command audit | ARCH-D-02 | **Complete** | `SPRINTS/Sprint_D/ARCH-D-02_COMMAND_AUDIT.md` — 36/36 aligned, 0 orphans |
| Security P0 | SEC-D-02, SEC-D-03 | **Complete** | Stub hash manifest; 128 MiB + 8192 cap pre-Python |
| Security P1 | SEC-D-04–SEC-D-06, SEC-D-12 | **Complete** | IPC caps, `path_security.rs`, settings/model ACL |
| Export tests | QA-B-01 | **Review** | 8 command tests in `lib.rs`; `cargo test` blocked Linux GTK |
| QA procedure | QA-D-01 | **Ready** | `QA-D-01_TEST_PROCEDURE.md`; execution blocked on hardware session |
| ACL granularity | SEC-D-01 | **Available** | Split per ARCH-D-02 § SEC-D-01 recommendations |
| Defect fixes | BACK-D-01 | **Blocked** | Awaiting QA-D-01 results |
| Interop docs | DOC-D-01 | **Available** | — |
| Threat model | DOC-D-02 | **Partial** | Wave 2 updates; sign-off pending |
| Performance | QA-D-02 | **Not started** | — |
| Pentest | QA-D-03 | **Available** | Checklist exists |

**Automated test health (2026-05-19 Wave 4):**

| Suite | Result | Notes |
|-------|--------|-------|
| `npm test` | 75 passed | After `npm install` |
| `pytest` (stub) | 44 passed | `SP3D_USE_STUB=1` |
| `cargo test` | Blocked | GTK/WebKit/libsoup — `SETUP_NOTES.md` |

---

## Requirements updates (architect decisions)

### 1. Phase D exit criteria — clarified

Phase D is **not** complete until **all** of the following are true:

1. **QA-D-01:** At least one PLY or XYZ export from the app imports into **XTool Studio** (xTool F2 Ultra UV) with axis/units matching `docs/engraver-charter.md`, recorded in `MANUAL_TEST_REPORT.md`.
2. **SEC-D-02 / SEC-D-03:** Complete or explicitly risk-accepted in `SECURITY_SIGNOFF.md` (P0 satisfied in code; real SHA256 digests remain maintainer action).
3. **SEC-D-01 + ARCH-D-02:** Structural audit done; **SEC-D-01 split required** before ACL track marked complete.
4. **QA-B-01:** Export integration tests pass in CI or Windows dev host.
5. **SECURITY_SIGNOFF.md:** Signed for Sprint D scope.

### 2. Engraver target — locked

| Field | Value |
|-------|-------|
| Software | XTool Studio |
| Hardware | xTool F2 Ultra UV |
| Charter | `docs/engraver-charter.md` |
| Procedure | `SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md` |

No alternate engraver is required for Phase D gate; additional apps are Phase E matrix work (DOC-D-01).

### 3. SEC-D-01 — approved split (from ARCH-D-02)

Replace monolithic `allow-generate-depth-map` with:

| Permission ID | Commands |
|---------------|----------|
| `allow-depth-inference` | `generate_depth_map` |
| `allow-depth-read` | `get_depth_map`, `get_depth_histogram` |
| `allow-depth-adjust` | `set_depth_adjustment_params`, `get_depth_adjustment_params`, undo/redo/history/reset |
| `allow-mask` | mask get/set/save/load/clear |

`default.json` must grant all four for the main window capability (product is single-window; least-privilege is documentation + future multi-window readiness).

### 4. IPC limits — requirement satisfied (document)

| Limit | Value | Task |
|-------|-------|------|
| Preview point cloud | 1M points cap | SEC-D-04 |
| Generation cap | 8M points | SEC-D-04 |
| Image pre-Python | 128 MiB, 8192 px | SEC-D-03 |

Threat model §2.6 must reflect these caps (DOC-D-02).

### 5. Deferred to Phase E (no Sprint D scope creep)

- Module rename `volumetric` → `surface_map` (naming debt)
- `withGlobalTauri: false` (SEC-D-11) — release engineering
- TripoSR / full 3D reconstruction
- Auto-refresh preview on depth change
- Real model SHA256 population (maintainer machine; SEC-D-02 follow-up)

### 6. `todo.md` / `ROADMAP.md` sync

Operational docs were stale on security gaps (pre-Wave 1). **Updated 2026-05-20** to reflect Waves 1–4 completion and remaining gate items.

---

## Risk register (updated)

| ID | Risk | Severity | Mitigation | Owner |
|----|------|----------|------------|-------|
| R-D-01 | No engraver import sign-off | **Critical** | QA-D-01 execution | QE + Product |
| R-D-02 | Linux dev cannot run `cargo test` | High | CI Windows job; `SETUP_NOTES.md` on Linux | Senior Engineer |
| R-D-03 | Empty model SHA256 table | Medium | Trusted download + `--write-hashes`; stub manifest documents intent | Security |
| R-D-04 | Coarse depth/mask ACL | Medium | SEC-D-01 Wave 5 | Senior Engineer |
| R-D-05 | Doc drift (mesh/STL references) | Low | DOC-D-01, Phase E doc sweep | Documentation |

---

## Wave 5 delegation (orchestrator)

**Orchestrator:** Senior Engineer (`SPRINTS/Sprint_D/ORCHESTRATION.md`)

| Priority | Task | Sub-agent | Deliverable |
|----------|------|-----------|-------------|
| P0 | SEC-D-01 | `senior-engineer` | Four permission TOMLs; `default.json` updated; deprecate or remove old `allow-generate-depth-map.toml` |
| P0 | QA-B-01 verify | `shell` | `cargo test` green or CI log excerpt in PROGRESS_REPORT |
| P1 | DOC-D-02 | `generalPurpose` + Security persona | `docs/threat-model.md` §2.4/§2.6; `docs/security-checklist.md` |
| P1 | DOC-D-01 | `generalPurpose` + Docs persona | `docs/engraver-interop.md` + README link |
| P2 | SEC-D-07–SEC-D-11 | Senior Engineer | Phase E unless security sign-off demands earlier |
| Blocked | BACK-D-01 | Senior Engineer | After QA-D-01 defect log |
| Human | QA-D-01 | Quality Engineer (user session) | `MANUAL_TEST_REPORT.md` C1–D7 |
| Gate | SECURITY_SIGNOFF | Security Specialist | After DOC-D-02 + P0 review |

---

## Phase gate forecast

| Phase | Prior assessment | Current |
|-------|------------------|---------|
| A — Clean House | Mostly complete | **Complete** |
| B — Backend Wiring | Mostly complete | **Complete** except QA-B-01 CI proof |
| C — Frontend UI | Mostly complete | **Complete** except QA-C-01 |
| D — Validate & Harden | Not started (takeover) | **In progress** — Waves 1–4 done; gate items open |
| E — v1.0 MVP | — | Blocked on Phase D |

**Estimated time to Phase D gate:** 1 engraver session + 1–2 dev days (SEC-D-01, docs, sign-off, CI).

---

## Architect sign-off

| Item | Status |
|------|--------|
| ARCH-D-01 | ✅ Approved |
| ARCH-D-02 | ✅ Approved — SEC-D-01 is mandatory follow-up |
| Algorithm (ADR-012) | ✅ No change required |
| Feature freeze for Sprint D | ✅ Confirmed |
| Wave 5 orchestration | ✅ Delegated to Senior Engineer orchestrator |

*Next architect touchpoint: after QA-D-01 results — review BACK-D-01 scope and Phase E performance requirements (QA-D-02).*
