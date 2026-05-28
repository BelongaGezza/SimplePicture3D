# QA-D-01 engraver validation fixtures

Reference exports for **Xtool Studio** import checks (see `SPRINTS/Sprint_D/QA-D-01_TEST_PROCEDURE.md`).

## Automated format check

CI runs `qa_d01_ply_header_matches_engraver_charter` in `src-tauri/src/lib.rs`, which exports a small point cloud and asserts the PLY header matches `docs/engraver-charter.md` §4.

## Manual golden files (optional)

For side-by-side import in Xtool Studio, export from the app with:

- Blank: **80 × 50 × 50 mm**, 2 mm margin (standard preset)
- Format: **PLY ASCII**
- Image: `tests/fixtures/valid_small.png` or your 4K test image

Store exports here as `manual_<date>_4k.ply` and note Studio version in `SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md`.
