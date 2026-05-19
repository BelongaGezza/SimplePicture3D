# Manual Test Report — Sprint D

**Sprint:** D — Validate and Harden  
**Tester:** _TBD_  
**Build:** _commit / version_  
**OS:** _Windows | macOS | Linux_  
**Date:** _TBD_

---

## QA-D-01 — Engraver import

| Format | Engraver software | Result | Notes |
|--------|-------------------|--------|-------|
| PLY | | Pass / Fail | |
| XYZ | | Pass / Fail | |

**Axis orientation correct:** ☐ Yes ☐ No  
**Units (mm) correct:** ☐ Yes ☐ No  

---

## QA-D-02 — Performance (4K image)

| Step | Time | Notes |
|------|------|-------|
| Load image | | |
| Generate depth | | |
| Generate point cloud | | |
| Export PLY | | |
| **Total** | | Target: &lt; 2 min (PRD) |

---

## QA-D-03 — Security pentest checklist

See `SECURITY_PENTEST_CHECKLIST.md` — copy Pass/Fail column here or attach summary:

| Section | Pass | Fail | Skipped |
|---------|------|------|---------|
| IPC fuzzing | | | |
| Filesystem | | | |
| Model & subprocess | | | |
| WebView / ACL | | | |
| DoS (lab) | | | |

---

## Overall

**Sprint D QA sign-off:** ☐ Pass ☐ Fail — **Date:** ___________
