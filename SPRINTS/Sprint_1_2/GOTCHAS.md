# Sprint 1.2 — Gotchas

Sprint-specific gotchas; merge notable items to `RESEARCH/GOTCHAS.md` when done.

---

## Security (SEC-101, SEC-102)

- **Path validation:** `std::path::Path::canonicalize()` fails if the path does not exist. Resolve existence check and canonicalization order: e.g. check file exists, then canonicalize, then apply blocklist. On Windows, watch for `\\?\` long-path prefixes when comparing canonical paths to blocklist.
- **Magic bytes:** Read at least 8 bytes for PNG (`89 50 4E 47 0D 0A 1A 0A`) and 3 for JPEG (`FF D8 FF`). If the file is shorter than 8 bytes, reject before calling the decoder.
