# RFC Implementation Handoffs

This directory holds **implementation handoffs** for proposed RFCs. A handoff
translates an accepted RFC's design into developer-executable work: module layout,
PR boundaries, task breakdown, acceptance checklists, edge cases, and CI commands.

The RFC remains the design authority; a handoff never overrides it. Handoffs are
working documents for the implementation team and may be revised as work proceeds.

## Current handoffs (v0.20+ proposed set, RFC-033–042)

| Handoff | RFC |
|---|---|
| `033-implementation-handoff.md` | RFC-033 `matten-data` Beta-Decision and Scope Lock |
| `034-implementation-handoff.md` | RFC-034 `matten-data` Table Model and Public API Boundary |
| `035-implementation-handoff.md` | RFC-035 CSV Ingestion, Schema Summary, Missing Values, and Numeric Conversion |
| `036-implementation-handoff.md` | RFC-036 `matten-data` Examples, Documentation, and Release Gate |
| `037-implementation-handoff.md` | RFC-037 Deferred Streaming and Large CSV Policy |
| `038-implementation-handoff.md` | RFC-038 Core Numeric Comfort APIs |
| `039-implementation-handoff.md` | RFC-039 Shape Composition API Boundary |
| `040-implementation-handoff.md` | RFC-040 Small Statistics Boundary — Core vs Companion |
| `041-implementation-handoff.md` | RFC-041 Linear Algebra Boundary — Core Lite vs External Crates |
| `042-implementation-handoff.md` | RFC-042 Pandas-Inspired Scope Guard for `matten-data` |
| `063-phase-1-visual-docs-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 1 docs only |
| `063-phase-2-example-reports-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 2 example reports |
| `063-phase-3-local-report-tool-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 3 first local report tool slice |
| `063-phase-3-shape-flow-report-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 3 shape-flow local report slice |
| `063-phase-3-dynamic-readiness-report-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 3 dynamic-readiness local report slice |
| `063-phase-3-mlprep-standardization-report-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 3 mlprep-standardization local report slice |
| `026-037-large-data-streaming-policy-closure-handoff.md` | RFC-026 / RFC-037 Large Data and Streaming Policy — proposed policy-closure slice |
| `049-lifecycle-closure-handoff.md` | RFC-049 lifecycle closure / Phase 4 deferral handoff |
| `054-matten-migrate-readiness-audit.md` | RFC-054 `matten-migrate` readiness audit |
| `054-matten-migrate-first-tool-handoff.md` | RFC-054 `matten-migrate` first local advisory tool slice |
| `054-matten-migrate-target-suggestion-handoff.md` | RFC-054 `matten-migrate` target-specific suggestion slice |
| `054-matten-migrate-explain-api-handoff.md` | RFC-054 `matten-migrate` static API explanation slice |
| `054-matten-migrate-check-bridges-handoff.md` | RFC-054 `matten-migrate` bridge-readiness check slice |
| `054-lifecycle-closure-handoff.md` | RFC-054 lifecycle closure / future automation extraction handoff |
| `066-v1-readiness-audit-handoff.md` | RFC-066 v1.0 readiness audit report handoff |
| `065-educational-visualization-handoff.md` | RFC-065 Educational Visualization and Tensor Learning Path — first docs/positioning slice |
| `065-phase-2-educational-shape-data-path-handoff.md` | RFC-065 Educational Visualization and Tensor Learning Path — Phase 2 educational shape/data docs slice |
| `065-phase-3-educational-path-report-handoff.md` | RFC-065 Educational Visualization and Tensor Learning Path — Phase 3 local educational-path report slice |
| `068-local-html-educational-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — first local HTML educational artifact slice |
| `068-shared-educational-report-model-handoff.md` | RFC-068 Rich Local Visualization Artifacts — shared educational-path report data refactor slice |
| `068-shape-flow-html-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — shape-flow local HTML artifact slice |
| `068-post-032-visualization-continuation-audit.md` | RFC-068 post-0.32 visualization continuation audit and next-slice recommendation |
| `068-dynamic-readiness-html-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — dynamic-readiness local HTML artifact handoff |
| `068-post-033-visualization-continuation-audit.md` | RFC-068 post-0.33 visualization continuation audit and next-slice recommendation |
| `068-mlprep-standardization-html-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — mlprep-standardization local HTML artifact handoff |
| `068-post-034-visualization-gap-audit.md` | RFC-068 post-0.34 visualization gap audit and next-decision recommendation |
| `068-data-readiness-html-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — data-readiness demo-only local HTML artifact handoff |
| `068-post-035-fixed-demo-html-closure-audit.md` | RFC-068 post-0.35 fixed-demo local HTML closure audit |

## Documentation-governance handoffs

These translate the specs supersession analysis into design-team work. They are not tied to a
single RFC; their authority is the supersession map. Run in order: 01 (gap closure) before 02
(archival); 03 (philosophy) may run after 01.

| Handoff | Purpose |
|---|---|
| `docs-governance-01-spec-coverage-gap-closure-handoff.md` | Resolve the three unowned spec fragments (perf targets, golden/fuzz/property tests, `Display`) before archival |
| `docs-governance-02-spec-archival-and-ownership-rule-handoff.md` | Archive the v0.19.0 specs as tracked banner-marked history; declare the ownership rule |
| `docs-governance-03-philosophy-distillation-handoff.md` | Expand `docs/src/philosophy.md` into a guarded, evergreen principles page distilled from the specs |

Some handoffs are planning records and some are retained implementation records.
A handoff alone does not authorize new work; each new implementation slice still
requires the corresponding RFC acceptance or follow-up review before coding begins.
