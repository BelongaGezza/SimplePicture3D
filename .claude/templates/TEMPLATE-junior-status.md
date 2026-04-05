# Junior Engineer Status Template

**Use this template to respond to senior engineer reviews and track implementation progress.**

Copy this file, fill in the sections, save as: `JS-{YYYYMMDD}-{short-slug}.md`

---

# Junior Engineer Status: {Issue Title}

**Date**: {YYYY-MM-DD}
**Engineer**: {Your name}
**Senior Review Reference**: `{link to SR-*.md file}`
**Original Issue**: `{link to IB-*.md file}`
**Status**: IN_PROGRESS | BLOCKED | READY_FOR_REVIEW | CHANGES_APPLIED

---

## Summary of Changes

{Brief overview of what you've implemented/fixed since last review}

## Acceptance Criteria Progress

Copy acceptance criteria from senior review and update status:

- [x] {Completed criterion}
- [ ] {In-progress criterion} — {note on progress}
- [ ] {Not started criterion}
- [ ] Unit tests pass
- [ ] No new warnings introduced

## Implementation Details

### Files Modified

| File | Changes Made |
|------|--------------|
| `Path/To/File.swift` | {Description of changes} |
| `Path/To/File.swift` | {Description} |

### Code Changes

**Before** (if applicable):
```swift
// Original problematic code
```

**After**:
```swift
// Your implementation
```

### Approach Taken

{Explain your implementation approach and any decisions you made}

## Addressing Senior Engineer Feedback

### 🔴 Critical Items

| Finding | Status | Notes |
|---------|--------|-------|
| {Issue from SR} | ✅ Fixed | {How you addressed it} |
| {Issue from SR} | 🔄 In Progress | {Current state} |

### 🟡 Important Items

| Finding | Status | Notes |
|---------|--------|-------|
| {Issue} | ✅ Fixed | {Notes} |
| {Issue} | ⏭️ Deferred | {Reason for deferral, if agreed upon} |

### 🟢 Minor Items

| Finding | Status | Notes |
|---------|--------|-------|
| {Issue} | ✅ Fixed | {Notes} |

## Testing Performed

- [ ] Unit tests written and passing
- [ ] Manual testing completed
- [ ] Edge cases verified
- [ ] Memory profiled (Instruments)
- [ ] Thread Sanitizer run

### Test Results

```
Paste test output here
```

### Manual Testing Notes

{Describe manual testing performed and results}

## Blockers / Issues Encountered

{Describe any blockers or unexpected issues}

### Blockers

- **Blocker**: {Description}
  - **Impact**: {What's blocked}
  - **Help Needed**: {Specific ask}

### Issues Encountered

- {Issue encountered during implementation and how you resolved it}

## Questions for Senior Engineer

1. {Question about implementation}
2. {Question about approach}
3. {Clarification needed}

## Documentation Updates

- [ ] Code comments added
- [ ] README updated (if applicable)
- [ ] API documentation updated (if applicable)

## Time Tracking

| Activity | Time Spent |
|----------|------------|
| Investigation | {X hours} |
| Implementation | {X hours} |
| Testing | {X hours} |
| **Total** | **{X hours}** |

## Next Steps

{What happens next? What do you need from senior engineer?}

- [ ] {Next action item}
- [ ] Request senior engineer re-review

---

## Revision History

| Date | Changes |
|------|---------|
| {YYYY-MM-DD} | Initial implementation |
| {YYYY-MM-DD} | {Revision description} |

---

**Ready for Re-Review Checklist**:
- [ ] All critical items addressed
- [ ] Tests passing
- [ ] No new warnings/errors
- [ ] Code builds successfully
- [ ] This status file is complete

**Submit by**: Notify senior-engineer agent that this file is ready for re-review
