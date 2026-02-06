# External Consultant Recommendations Report

**Project:** SimplePicture3D
**Date:** February 6, 2026
**Consultant:** Independent Technical Review
**Documents Reviewed:** prd.md, todo.md, RESEARCH/architecture.md, CLAUDE.md, CI/CD pipeline, codebase

---

## Executive Summary

SimplePicture3D is an ambitious open-source desktop application for converting 2D images into 2.5D meshes for laser engraving. The project demonstrates **exceptional documentation maturity** and a **well-structured multi-agent development workflow**. The PRD is comprehensive, the sprint planning is detailed, and the architecture is sound.

**Current Progress:** Sprint 1.5 (Manual Depth Adjustments) - approximately 35-40% through Phase 1 MVP.

**Overall Assessment:** The project foundation is strong, but several gaps in testing infrastructure, dependency risks, and timeline assumptions warrant attention before proceeding further.

| Category | Rating | Notes |
|----------|--------|-------|
| Documentation Quality | **Excellent** | PRD, architecture, and sprint docs are professional-grade |
| Architecture Design | **Good** | Sound choices; Tauri + Rust + Python is appropriate |
| Sprint Planning | **Good** | Detailed but timeline estimates may be optimistic |
| Testing Infrastructure | **Needs Improvement** | Significant gaps in coverage, no pytest, no frontend tests |
| Risk Management | **Adequate** | Risks identified but mitigations incomplete |
| CI/CD Pipeline | **Partial** | Basic pipeline exists; missing clippy enforcement, coverage |

---

## 1. Strengths

### 1.1 Documentation Excellence

The project's documentation is its strongest asset:

- **PRD (prd.md):** 1,200+ lines covering vision, personas, features, architecture, security, licensing, and success criteria. This is production-quality documentation.
- **Sprint Planning (todo.md):** 1,700+ lines with detailed task breakdowns, role assignments, and exit criteria for all phases.
- **Architecture:** Clear separation of concerns (Rust backend, Python AI, Svelte frontend) with well-defined IPC contracts.
- **GOTCHAS.md:** Proactive capture of platform-specific issues (Tauri v2 quirks, icon formats, MiDaS deprecation).

### 1.2 Multi-Agent Development Workflow

The `.agents/` persona system and `RESEARCH/AI_DEVELOPMENT_GUIDE.md` provide a robust framework for AI-assisted development:

- Clear role definitions (Architect, Senior Engineer, Junior Engineers, QA, Security)
- Task handover protocol with explicit deliverables
- Sprint artefact storage structure (`SPRINTS/Sprint_X_Y/`)
- Knowledge capture via GOTCHAS.md

### 1.3 Technology Choices

The core stack is well-suited for the requirements:

| Choice | Rationale | Risk Level |
|--------|-----------|------------|
| Tauri v2 | Cross-platform, small binary, Rust backend | Low |
| Rust for core | Performance, memory safety, STL/mesh processing | Low |
| Python for AI | PyTorch ecosystem, Depth-Anything-V2 | Medium |
| Svelte + TailwindCSS | Modern, lightweight, good DX | Low |
| Three.js for preview | Standard WebGL, large community | Low |

### 1.4 Security-First Approach

Security considerations are embedded throughout:

- 100% offline processing requirement
- Threat model defined in PRD 8.1
- Dependency audits (`cargo audit`, `npm audit`) in CI
- Checksum verification for model downloads
- GOTCHAS.md captures permission/IPC security issues

---

## 2. Gaps and Concerns

### 2.1 Critical: Testing Infrastructure

**Current State:** The testing pyramid is inverted.

| Layer | PRD Target | Actual State |
|-------|------------|--------------|
| Rust Unit Tests | >70% coverage | 27 tests, no coverage tracking |
| Python Tests | >70% coverage | **None** - no pytest suite |
| Frontend Tests | >60% coverage | **None** - no test runner |
| E2E Tests | Critical paths | Manual only |

**Evidence from todo.md 1449-1501:**
> "No `cargo clippy` in CI; no coverage (tarpaulin)... No test runner (no Vitest/Jest); no `npm test`... No `pytest` suite; no `test_*.py`"

**Impact:** Phase 1 exit criteria require >70% coverage on core logic. Current trajectory will not meet this without significant investment.

### 2.2 High: Timeline Optimism

The PRD estimates 19-25 sprints (~9-12 months) for v1.0. However:

1. **Sprint 1.5 status:** "Not Started" on all tasks as of Feb 4, despite planned 2-week sprint duration
2. **Phase 1 alone:** 12 sprints planned = 24 weeks (6 months)
3. **Testing debt:** Building test infrastructure retroactively always takes longer than planned

**Recommendation:** Budget 40-50% additional time buffer, or scope reduction.

### 2.3 High: Python Dependency Complexity

The Python-Rust bridge introduces significant complexity:

1. **Model License Issue (PRD 9.2):**
   > "Depth-Anything-V2 weights: CC-BY-NC-4.0 (non-commercial)"

   This **conflicts with commercial use cases** mentioned in success criteria ("Commercial laser shops using in production").

2. **No Python tests in CI:** The `python/` directory has no test coverage, and Python-dependent Rust tests are `#[ignore]`.

3. **Bundled Python distribution:** The PRD doesn't address how Python will be bundled or managed across platforms. This is a common pain point for Tauri apps with Python dependencies.

### 2.4 Medium: CI/CD Gaps

**Current CI (`.github/workflows/ci.yml`):**
- `npm ci && npm run build && npm audit`
- `cargo build && cargo test && cargo audit`

**Missing:**
- `cargo clippy -- -D warnings` (lint enforcement)
- `cargo tarpaulin` (coverage)
- `pytest` (Python tests)
- Matrix builds (Windows, macOS, Linux)
- Release workflow (documented but not implemented)

### 2.5 Medium: Architecture Decision Gaps

Several key decisions are deferred or unclear:

1. **Svelte vs React:** PRD mentions both; current implementation is Svelte, but no formal ADR.
2. **PyO3 vs Subprocess:** PRD mentions both; current implementation uses subprocess, but PRD 12.1 lists "Python-Rust IPC bottleneck" as a risk.
3. **ONNX Runtime:** Mentioned as risk mitigation but no spike or evaluation documented.
4. **Python Distribution:** How will Python + PyTorch + Depth-Anything-V2 be bundled? Current `depth_estimator.py` assumes external Python installation.

### 2.6 Low: Documentation Duplication

Some information is duplicated across documents:

- Architecture diagram appears in PRD 5.2, `RESEARCH/architecture.md`, and `CLAUDE.md`
- Tech stack tables in multiple locations
- Potential for drift as project evolves

---

## 3. Recommendations

### Priority 1: Address Testing Debt Immediately (Sprint 1.5-1.6)

| Action | Owner | Sprint | Effort |
|--------|-------|--------|--------|
| Add `cargo clippy -- -D warnings` to CI | QA Engineer | 1.5 | 1 hour |
| Integrate `cargo tarpaulin` for coverage | QA Engineer | 1.5 | 4 hours |
| Create pytest suite for `depth_estimator.py` | Senior Researcher | 1.5 | 1 day |
| Add `npm test` with Vitest smoke tests | UI Specialist | 1.6 | 1 day |
| Enable Python tests in CI (with stub mode) | Senior Engineer | 1.5 | 4 hours |

**Rationale:** Fixing testing gaps early prevents compounding technical debt.

### Priority 2: Resolve Model License Issue Before Beta

The CC-BY-NC-4.0 license on Depth-Anything-V2 weights is incompatible with commercial use. Options:

| Option | Effort | Trade-off |
|--------|--------|-----------|
| A. Clarify non-commercial scope | Low | Limits adoption |
| B. Offer MiDaS alternative (MIT weights available) | Medium | Lower quality |
| C. Train custom model with permissive license | High | Requires ML expertise |
| D. Dual-licensing: non-commercial default, commercial license purchasable | Medium | Revenue opportunity |

**Recommendation:** Option B for MVP (offer MiDaS as commercial-friendly alternative), pursue Option D for v1.0.

### Priority 3: Create Architecture Decision Records (ADRs)

Document these decisions formally:

1. **ADR-001:** Svelte over React (decision rationale, trade-offs)
2. **ADR-002:** Subprocess over PyO3 for Python bridge (performance implications, error handling)
3. **ADR-003:** Python distribution strategy (bundled vs system Python vs embedded)
4. **ADR-004:** Depth model selection (Depth-Anything-V2 vs MiDaS vs future options)

### Priority 4: Revise Timeline Estimates

**Current:** 19-25 sprints (38-50 weeks)
**Recommended:** 28-35 sprints (56-70 weeks)

Rationale:
- Testing infrastructure buildout: +3 sprints
- Platform-specific debugging (historically underestimated): +2 sprints
- Integration testing and bug fixes: +2 sprints
- Buffer for unknowns: +2 sprints

Alternatively, reduce Phase 1 scope:
- Defer OBJ export to Phase 2
- Defer model download wizard to Phase 2
- Focus on STL export + manual depth mode for MVP

### Priority 5: Python Distribution Strategy

Before Sprint 1.10 (Model Installer), resolve Python bundling:

| Approach | Pros | Cons |
|----------|------|------|
| System Python | Simple, user manages | Version conflicts, missing deps |
| Bundled Python (PyInstaller) | Self-contained | 200-500MB size increase |
| ONNX Runtime in Rust | No Python needed | Re-implementation effort, model conversion |
| Tauri Sidecar with embedded Python | Clean isolation | Complexity, size |

**Recommendation:** For MVP, document "System Python 3.10+ required" with pip install instructions. Plan ONNX migration for v1.1.

---

## 4. Risk Assessment Updates

### Risks to Elevate

| Risk | Current | Recommended | Rationale |
|------|---------|-------------|-----------|
| Testing debt compounds | Not listed | **High** | Every sprint without tests increases integration risk |
| Python bundling complexity | Medium | **High** | Cross-platform Python distribution is notoriously difficult |
| Model license commercial conflict | Low | **Medium** | Success criteria mention commercial users |

### Risks to Add

| New Risk | Probability | Impact | Mitigation |
|----------|-------------|--------|------------|
| Depth-Anything-V2 model becomes unavailable | Low | High | Mirror model weights; document alternative models |
| Tauri v2 breaking changes | Medium | Medium | Pin Tauri version; subscribe to release notes |
| PyTorch version conflicts on user systems | Medium | Medium | Bundle Python or migrate to ONNX |
| Multi-agent workflow coordination failures | Medium | Low | Daily sync protocol; clear task ownership |

---

## 5. Suggested Immediate Actions

### This Week (Sprint 1.5)

- [ ] Add `cargo clippy` step to CI (QA-005 from todo.md)
- [ ] Create `python/tests/test_depth_estimator.py` with stub mode tests
- [ ] Document Python installation requirements in README.md
- [ ] Create ADR-001 for Svelte decision

### Next Sprint (1.6)

- [ ] Integrate Vitest for frontend testing
- [ ] Add coverage reporting to CI (tarpaulin + codecov)
- [ ] Evaluate ONNX Runtime spike for Rust-native inference
- [ ] Draft Python distribution strategy document

### Before Phase 1 Exit

- [ ] Achieve 70% Rust coverage with tarpaulin
- [ ] Complete pytest suite for Python
- [ ] Resolve model license issue with documented decision
- [ ] Create release workflow in GitHub Actions

---

## 6. Positive Observations

Despite the gaps noted, this project is **well-positioned for success**:

1. **Documentation-first approach** will pay dividends during onboarding and maintenance
2. **Security mindset** embedded from the start prevents costly retrofits
3. **Modular architecture** allows parallel development streams
4. **GOTCHAS.md pattern** captures institutional knowledge effectively
5. **Sprint artefact structure** enables clear accountability and progress tracking

The team has clearly invested significant effort in planning. The recommendations above are intended to strengthen an already solid foundation.

---

## 7. Appendix: Document Cross-Reference

| Topic | Primary Source | Supporting Docs |
|-------|---------------|-----------------|
| Vision & Requirements | prd.md 1-3 | CLAUDE.md |
| Feature Specifications | prd.md 4 | todo.md (sprint tasks) |
| Architecture | prd.md 5, RESEARCH/architecture.md | RESEARCH/*.md |
| Security | prd.md 8 | docs/threat-model.md (referenced) |
| Testing Strategy | todo.md 1448-1545 | SPRINTS/TEST_PLAN_TEMPLATE.md |
| Sprint Planning | todo.md | SPRINTS/SPRINT_TASKING_TEMPLATE.md |
| Known Issues | RESEARCH/GOTCHAS.md | SPRINTS/Sprint_X_Y/GOTCHAS.md |

---

**Report Prepared By:** External Technical Consultant
**Review Status:** For project stakeholder review
**Next Review:** Recommended at Phase 1 Exit Gate

---

*This report is based on documentation and codebase review as of February 6, 2026. Recommendations are advisory and should be evaluated against project constraints and priorities.*
