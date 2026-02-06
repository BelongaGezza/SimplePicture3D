# Sprint 1.5A Security Sign-Off

**Sprint:** 1.5A — Hardening, Testing & Consultant Remediation
**Security Specialist:** *(fill when role claimed)*
**Date:** *(fill when review complete)*

---

## Scope

This security review covers:
1. **SEC-501:** Asset protocol scope in `tauri.conf.json` — restrict from `"**"` to minimal required paths
2. **SEC-502:** Tauri capabilities and permissions — verify minimal privilege

---

## SEC-501: Asset Protocol Scope

**Before:**
```json
"assetProtocol": {
  "enable": true,
  "scope": ["**"]
}
```

**Finding:** *(fill during review)*

**After:** *(fill after fix applied)*

**Verification:** *(fill — confirm app still works)*

---

## SEC-502: Capabilities Review

**Capabilities file:** `src-tauri/capabilities/default.json`

**Permissions reviewed:**
| Permission | Required? | Notes |
|-----------|-----------|-------|
| *(fill)* | *(fill)* | *(fill)* |

**Shell plugin:** *(fill — is it needed? is it minimal?)*

---

## Sign-Off

- [ ] SEC-501 reviewed and remediated
- [ ] SEC-502 reviewed and documented
- [ ] No new security issues identified
- [ ] Findings consistent with `docs/threat-model.md`

**Signed off by:** *(fill)*
**Date:** *(fill)*
