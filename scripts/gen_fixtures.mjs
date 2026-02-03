#!/usr/bin/env node
/**
 * Generate test image fixtures for Sprint 1.2 (QA-101).
 * Run from repo root: node scripts/gen_fixtures.mjs
 * Output: tests/fixtures/valid_1x1.png, valid_small.png, invalid_not_an_image.png, corrupt_truncated.png
 */
import { writeFileSync, mkdirSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = join(__dirname, "..");
const outDir = join(root, "tests", "fixtures");

// Minimal 1×1 PNG (67 bytes) — known-good from "world's smallest PNG" (evanhahn.com)
const minimalPng = Buffer.from(
  "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDQADhgGAWjR9awAAAABJRU5ErkJggg==",
  "base64"
);

// Second valid PNG (same 1×1 for CI; replace with 100×100 via ImageMagick if needed for size tests)
const validSmallPng = Buffer.from(minimalPng);

// Invalid: not an image (text with .png extension)
const invalidNotImage = Buffer.from("not an image file\n", "utf8");

// Corrupt: valid PNG signature then garbage (truncated)
const corruptTruncated = Buffer.concat([
  minimalPng.subarray(0, 24),
  Buffer.from([0x00, 0x00, 0x00]), // truncated
]);

mkdirSync(outDir, { recursive: true });
writeFileSync(join(outDir, "valid_1x1.png"), minimalPng);
writeFileSync(join(outDir, "valid_small.png"), validSmallPng);
writeFileSync(join(outDir, "invalid_not_an_image.png"), invalidNotImage);
writeFileSync(join(outDir, "corrupt_truncated.png"), corruptTruncated);
console.log("Fixtures written to tests/fixtures/");
console.log("  valid_1x1.png, valid_small.png, invalid_not_an_image.png, corrupt_truncated.png");
console.log("valid_1x1.png dimensions: 1×1 (67 bytes)");
console.log("valid_small.png: same 1×1 (use for second valid fixture; replace with 100×100 if needed)");
console.log("invalid_not_an_image.png: text — for magic-byte / QA-104");
console.log("corrupt_truncated.png: truncated PNG — for decode-failure / QA-104");
