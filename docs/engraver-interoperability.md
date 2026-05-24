# Engraver interoperability notes

**Status:** Draft — pending QA-D-01 validation  
**Owner:** Documentation Specialist (DOC-D-01)  
**Related:** [Engraver charter](engraver-charter.md), `SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md`

---

## Target software

| Field | Value |
|-------|--------|
| Product name | **Xtool Studio** |
| Hardware | **Xtool F2** (UV laser engraver) |
| Version | _Record at QA-D-01_ |
| Import formats to test | PLY (ASCII) first, then XYZ |
| Platform | Windows (primary) |

---

## Known-good configuration

_To be filled after QA-D-01 pass._

| Setting | Value | Notes |
|---------|-------|-------|
| Units | mm | Per charter §2 |
| Axis mapping | _TBD_ | Record engraver X/Y/Z vs SP3D export |
| Point order | Row-major sample order | Per charter §3 |

---

## Export checklist (from app)

1. Load image → generate depth (stub or model).
2. Set blank envelope (e.g. 80×50×50 mm standard preset).
3. Adjust depth if needed; set volumetric step/threshold.
4. Generate point cloud → export PLY or XYZ.
5. Import into engraver; verify scale, orientation, and point count.

---

## Troubleshooting

| Symptom | Likely cause | Mitigation |
|---------|--------------|------------|
| Flat / thin engraving | Depth threshold too high | Lower threshold in Volumetric params |
| Mirrored left–right | Engraver axis convention | Document flip in charter; BACK-D-01 fix |
| Wrong scale | Unit mismatch | Confirm mm in engraver; check `fit_to_blank` |

---

*Last updated: 2026-05-24 (structure only; validation pending).*
