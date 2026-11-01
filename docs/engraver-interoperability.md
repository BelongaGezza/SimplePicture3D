# Engraver interoperability notes

**Status:** Active — Xtool toolchain confirmed; import validation pending QA-D-01  
**Owner:** Documentation Specialist (DOC-D-01)  
**Canonical spec:** [Engraver charter](engraver-charter.md)  
**Test procedure:** `SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md`

---

## Target software

| Field | Value |
|-------|--------|
| Product name | **Xtool Studio** |
| Hardware | **Xtool F2** (UV laser engraver) |
| Version | Record at QA-D-01 in `SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md` |
| Import formats | **PLY (ASCII)** first, then **XYZ** |
| Platform | Windows (primary) |

---

## What SimplePicture3D exports

See [charter §3–5](engraver-charter.md) for axes, units (mm), and ADR-012 surface-map semantics.

| Format | Use in Studio | Notes |
|--------|---------------|--------|
| PLY ASCII | Primary QA path | Header comments include blank size and sanitized source basename |
| XYZ | Fallback if PLY fails | No header; same mm coordinates |
| CSV | Optional | For spreadsheet checks only |

---

## Known-good configuration

_Fill after QA-D-01 pass._

| Setting | Value | Notes |
|---------|-------|-------|
| Units | mm | Per charter §3 |
| Blank preset | 80 × 50 × 50 mm, 2 mm margin | Export panel “standard” |
| Volumetric step | 1 / 1 | Full resolution unless performance requires 2+ |
| Depth threshold | 0.05 default | Raise if background speckle appears |
| Axis mapping | _TBD_ | Record Studio import orientation vs SP3D |

---

## Export workflow (app)

1. Load image → **Generate depth** (stub or downloaded model).
2. Set blank envelope (standard preset or custom mm).
3. Adjust depth; paint mask if needed (preview coords map to full-res — see `RESEARCH/GOTCHAS.md` 2026-05-24 mask IPC entry).
4. Set volumetric step/threshold → **Generate point cloud**.
5. Export **PLY ASCII** (or XYZ) → import into Xtool Studio.

---

## Validation checklist

| Check | How |
|-------|-----|
| PLY header | Automated: `qa_d01_ply_header_matches_engraver_charter` |
| Import into Studio | Manual: `QA-D-01_TEST_PROCEDURE.md` § D |
| Scale/orientation | Compare bbox in Studio to charter §3; log defects for BACK-D-01 |
| Performance | `QA-D-02_BENCHMARK.md` + `MANUAL_TEST_REPORT.md` § QA-D-02 |

---

## Troubleshooting

| Symptom | Likely cause | Mitigation |
|---------|--------------|------------|
| Flat / thin engraving | Depth threshold too high | Lower threshold in Volumetric params |
| Mirrored left–right | Studio axis convention | Document flip; BACK-D-01 if export must change |
| Wrong scale | Unit mismatch | Confirm mm in Studio; check `fit_to_blank` |
| Mask has no effect on 4K | Fixed 2026-05-24 | Update app; IPC preview coords now scale to full-res |
| Import rejects file | Binary PLY or bad path | Use ASCII PLY; check file is not empty |

---

## Related artefacts

- `docs/engraver-charter.md` — committed export contract
- `tests/fixtures/qa-d-01/README.md` — fixture notes
- `SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md` — pass/fail record
- `SPRINTS/Sprint_D/QA-D-02_BENCHMARK.md` — performance measurement

*Last updated: 2026-05-24 (DOC-D-01 complete pending QA-D-01 results).*
