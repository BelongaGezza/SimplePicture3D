# Test Image Fixtures — Sprint 1.2 (QA-101)

This directory holds (or references) test images for image loading: valid/invalid formats, sizes, and corrupt files. Used by BACK-102, BACK-103, JR2-101, JR2-103, QA-103, QA-104, and manual testing.

## Checked-in fixtures (QA-101)

| File | Purpose | Used by |
|------|---------|--------|
| `valid_1x1.png` | Minimal valid PNG (1×1); success path, dimension assertion | QA-103, JR2-101 |
| `valid_small.png` | Second valid PNG (1×1); duplicate for tests that need two valid files; Rust→Python integration (JR2-201), manual depth test (QA-204) | QA-103, JR2-201, QA-204 |
| `invalid_not_an_image.png` | Plain text with .png extension; wrong magic bytes | BACK-102, SEC-102, QA-104 |
| `corrupt_truncated.png` | Valid PNG signature + truncated content; decode fails | BACK-102, QA-104 |

Generate or refresh with: `node scripts/gen_fixtures.mjs` (from repo root; requires Node). Manual test plan and automated tests reference these in `SPRINTS/Sprint_1_2/TEST_PLAN_1_2.md`.

## Required dataset (full)

| Fixture | Purpose | Size / format | Used by |
|--------|---------|----------------|---------|
| Valid small PNG | Happy path, fast tests | 1×1 or 100×100 PNG (see above) | JR2-101, QA-103 |
| Valid small JPG | Format coverage | e.g. 100×100 JPEG | Manual, JR2-101 |
| Valid 4K | Typical use case | 3840×2160 or 4096×2160 PNG/JPG | Manual, JR1-104 |
| Valid 8K | At PRD limit | 8192×8192 or 7680×4320 | Manual, JR1-104 |
| Valid >8K | Downsampling | e.g. 16384×8192 or 10000×10000 | BACK-103, JR2-103, JR1-104 |
| Invalid format | Reject non-image | See `invalid_not_an_image.png` | BACK-102, SEC-102, QA-104 |
| Corrupt image | Reject after decode | See `corrupt_truncated.png` | BACK-102, QA-104 |

## Obtaining or creating fixtures

- **Valid small PNG/JPG:** Automated Rust tests (QA-103, JR2-101) create minimal images in a temp dir at runtime via the `image` crate. No checked-in file required for CI.
- **Larger images (4K, 8K, >8K):** Create with any tool (ImageMagick, GIMP, script) or use royalty-free samples. Store here or in a sibling folder (e.g. `tests/fixtures/images/`) and reference in test plan. Example ImageMagick:
  - 4K: `magick -size 3840x2160 xc:gray 4k.png`
  - 8K: `magick -size 8192x8192 xc:gray 8k.png`
  - 16K (for downsampling): `magick -size 16384x8192 xc:gray 16k.png`
- **Invalid format:** A text file renamed to `.png` or a file containing e.g. `not an image` (no PNG/JPEG magic bytes).
- **Corrupt image:** Take a valid PNG, then overwrite some bytes in the middle with zeros, or truncate the file. Ensures magic-byte check can pass but decode fails.

## Directory layout (optional)

```
tests/fixtures/
├── README.md          (this file)
├── images/            (optional: valid_small.png, 4k.png, 8k.png, 16k.png)
├── invalid/           (optional: not_an_image.png, wrong_extension.txt)
└── corrupt/           (optional: truncated.png, bad_crc.png)
```

If you don't check in binaries, document in this README and in `SPRINTS/Sprint_1_2/TEST_PLAN_1_2.md` where to obtain or how to generate each fixture. JR2-103 and manual tests can use locally generated 16K and 4K/8K images per above.

## Sprint 1.3 use

- **JR2-201 (integration test):** Use `valid_small.png` or `valid_1x1.png` for Rust→Python roundtrip (image in → depth out).
- **QA-204 (manual depth test):** Use fixtures here for end-to-end depth estimation once the pipeline is ready.

## References

- **Task:** QA-101 (SPRINT_1_2_Task_Assignment.md)
- **PRD:** prd.md F1.1 (max dimensions 8192×8192, PNG/JPG)
- **Test plan:** SPRINTS/Sprint_1_2/TEST_PLAN_1_2.md; Sprint 1.3: SPRINTS/Sprint_1_3/TEST_PLAN_1_3.md
