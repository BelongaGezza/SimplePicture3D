# Issue Brief Template

**Use this template to submit issues/bugs/tasks for senior engineer review.**

Copy this file, fill in the sections, save as: `IB-{YYYYMMDD}-{short-slug}.md`

---

# Issue Brief: {Descriptive Title}

**Date**: {YYYY-MM-DD}
**Author**: {Your name}
**Priority**: 🔴 Critical | 🟡 High | 🟢 Medium | ⚪ Low
**Type**: Bug | Feature | Refactor | Architecture | Performance

---

## Problem Statement

{Clear, concise description of the issue. What's broken or what needs to be built?}

## Expected Behavior

{What should happen?}

## Actual Behavior

{What's currently happening? Include error messages if applicable.}

## Reproduction Steps

1. {Step}
2. {Step}
3. {Step}
4. {Observe issue}

## Environment

- **Platform**: iOS / macOS / watchOS / tvOS / visionOS
- **OS Version**: {e.g., iOS 18.1}
- **Xcode Version**: {e.g., 16.1}
- **Swift Version**: {e.g., 6.0}
- **Device/Simulator**: {e.g., iPhone 16 Pro Simulator}
- **Build Configuration**: Debug / Release

## Affected Files/Components

- `Path/To/File.swift` — {brief description of involvement}
- `Path/To/AnotherFile.swift` — {brief description}

## Related Code

```swift
// Paste relevant code snippet here
// Include enough context for understanding
```

## Error Logs / Stack Trace

```
Paste any relevant console output, crash logs, or stack traces here
```

## Screenshots / Recordings

{Attach or link to visual evidence if applicable}

## What I've Tried

- [ ] {Attempted solution 1} — Result: {what happened}
- [ ] {Attempted solution 2} — Result: {what happened}

## Hypothesis (if any)

{Your best guess at root cause, if you have one}

## Relevant Documentation

- {Link to any docs you've consulted}
- {Link to related issues/PRs}

## Questions for Senior Engineer

1. {Specific question}
2. {Question}

## Constraints / Context

{Any deadlines, dependencies, or business context that affects the solution}

---

**Submission Checklist**:
- [ ] Problem is clearly described
- [ ] Reproduction steps are accurate and complete
- [ ] Relevant code and logs are included
- [ ] I've attempted at least one solution

**Submit by**: Referencing this file in a request to the senior-engineer agent
