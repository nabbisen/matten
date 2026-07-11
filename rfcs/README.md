# `matten` RFCs

Design decisions for `matten` are tracked here using the lifecycle policy in
[RFC-000](./done/000-rfc-lifecycle-policy.md). The folder is the source of truth
for RFC state; the Status field inside each file mirrors the folder.

## Done

| ID | Title | Shipped in |
|---:|---|---|
| 000 | [RFC Lifecycle Policy](./done/000-rfc-lifecycle-policy.md) | 0.0.1 |
| 001 | [Threat Model and Boundary Safety Policy](./done/001-threat-model-and-boundary-safety-policy.md) | 0.9.0 |
| 002 | [Public API Minimalism and `Tensor` Contract](./done/002-public-api-minimalism-and-tensor-contract.md) | 0.1.0 |
| 003 | [Shape Model, Scalar Semantics, and Validation](./done/003-shape-model-scalar-semantics-and-validation.md) | 0.1.0 |
| 004 | [Construction and Conversion APIs](./done/004-construction-and-conversion-apis.md) | 0.2.0 |
| 005 | [Error Model, Panic Messages, and Boundary APIs](./done/005-error-model-panic-messages-and-boundary-apis.md) | 0.1.0 |
| 006 | [Broadcasting and Element-Wise Operators](./done/006-broadcasting-and-element-wise-operators.md) | 0.3.0 |
| 007 | [Reshape, Axis Operations, and Indexing](./done/007-reshape-axis-operations-and-indexing.md) | 0.4.0 |
| 008 | [Slicing API: Builder and `slice_str`](./done/008-slicing-api-builder-and-slice-str.md) | 0.4.0 |
| 009 | [Serde, JSON, CSV, and Boundary Integration](./done/009-serde-json-csv-and-boundary-integration.md) | 0.5.0 |
| 010 | [Reductions, Basic Statistics, and Matrix Multiplication](./done/010-reductions-basic-statistics-and-matrix-multiplication.md) | 0.7.0 |
| 011 | [Dynamic `Element` Model and Coercion](./done/011-dynamic-element-model-and-coercion.md) | 0.8.0 |
| 012 | [Dynamic Storage, View Metadata, and Copy-on-Write](./done/012-dynamic-storage-view-metadata-and-cow.md) | 0.8.0 |
| 013 | [Testing, Compatibility, and Release Gates](./done/013-testing-compatibility-and-release-gates.md) | 0.6.0 |
| 014 | [Example Suite and Executable Documentation](./done/014-example-suite-and-executable-documentation.md) | 0.6.0 |
| 015 | [Public API Stabilization and Compatibility Policy](./done/015-public-api-stabilization-and-compatibility-policy.md) | 0.13.3 |
| 016 | [Dynamic Ingestion and Explicit Numeric On-Ramp](./done/016-dynamic-ingestion-and-explicit-numeric-on-ramp.md) | 0.14.0 |
| 017 | [Numeric Conversion Policy](./done/017-numeric-conversion-policy.md) | 0.14.0 |
| 018 | [Shape, Allocation, and Resource Safety Limits](./done/018-shape-allocation-and-resource-safety-limits.md) | 0.14.0 |
| 019 | [Axis Reductions and Small Matrix Statistics](./done/019-axis-reductions-and-small-matrix-statistics.md) | 0.14.0 (core); 0.15.0 (examples) |
| 020 | [Human-Readable Diagnostics and Error Message Quality](./done/020-human-readable-diagnostics-and-error-message-quality.md) | 0.13.3 |
| 021 | [Tutorial Path and Example Quality Gate](./done/021-tutorial-path-and-example-quality-gate.md) | 0.15.0 |
| 022 | [Companion Crate Boundary Policy](./done/022-companion-crate-boundary-policy.md) | 0.16.0 |
| 023 | [`matten-data` Scope and Non-goals](./done/023-matten-data-scope-and-non-goals.md) | 0.22.0 (resolved: Outcome B → Beta) |
| 024 | [`matten-mlprep` Scope and Non-goals](./done/024-matten-mlprep-scope-and-non-goals.md) | 0.18.0 (impl RFC-028; maturity RFC-029) |
| 025 | [Bridge Crate Policy for ndarray, nalgebra, and candle](./done/025-bridge-crate-policy-for-ndarray-nalgebra-and-candle.md) | 0.17.0 (matten-ndarray; nalgebra/candle deferred) |
| 027 | [`matten-ndarray` Design and Implementation](./done/027-matten-ndarray-design-and-implementation.md) | 0.17.0 |
| 028 | [`matten-mlprep` Design and Implementation](./done/028-matten-mlprep-design-and-implementation.md) | 0.18.0 |
| 029 | [Companion Maturity Evaluation (v0.19.0)](./done/029-companion-maturity-evaluation-v0-19.md) | 0.19.0 |
| 030 | [Workspace Versioning Model — Lock-step Family Versioning](./done/030-workspace-versioning-model-lockstep.md) | 0.19.0 |
| 031 | [Feature-Robust Dynamic Rejection and Unconditional `Tensor::is_dynamic()`](./done/031-feature-robust-dynamic-rejection.md) | 0.19.1 |
| 032 | [Companion Dependency and Import Convention](./done/032-companion-dependency-and-import-convention.md) | 0.19.2 |
| 033 | [`matten-data` Beta-Decision and Scope Lock](./done/033-matten-data-beta-decision-and-scope-lock.md) | 0.20.0 (experimental scaffold; beta deferred to v0.21+) |
| 034 | [`matten-data` Table Model and Public API Boundary](./done/034-matten-data-table-model-and-public-api-boundary.md) | 0.20.1 |
| 035 | [CSV Ingestion, Schema Summary, Missing Values, and Numeric Conversion](./done/035-csv-ingestion-schema-summary-missing-values-and-numeric-conversion.md) | 0.20.1 |
| 036 | [`matten-data` Examples, Documentation, and Release Gate](./done/036-matten-data-examples-documentation-and-release-gate.md) | 0.22.0 |
| 038 | [Core Numeric Comfort APIs](./done/038-core-numeric-comfort-apis.md) | 0.20.9 (elementwise); 0.20.10 (selection); 0.20.11 (creation); 0.20.12 (shape) |
| 039 | [Shape Composition API Boundary](./done/039-shape-composition-api-boundary.md) | 0.21.0 (`concatenate` + `stack`; `repeat`/`tile`/`meshgrid` deferred) |
| 040 | [Small Statistics Boundary — Core vs Companion](./done/040-small-statistics-boundary-core-vs-companion.md) | 0.21.2 (`var`/`std` + `var_axis`/`std_axis`, population; quantile/histogram/cov/corr deferred) |
| 041 | [Linear Algebra Boundary — Core Lite vs External Crates](./done/041-linear-algebra-boundary-core-lite-vs-external-crates.md) | 0.21.1 (`norm` + `trace` + `outer`; decomposition/BLAS/sparse rejected) |
| 042 | [Pandas-Inspired Scope Guard for `matten-data`](./done/042-pandas-inspired-scope-guard-for-matten-data.md) | 0.21.3 (three-check anti-scope guard; CI-enforced) |
| 043 | [Example Program Structure, Quality Gate, and Documentation Policy](./done/043-example-program-structure-quality-gate-and-documentation-policy.md) | 0.20.3 |
| 044 | [Beginner Core Math Examples](./done/044-beginner-core-math-examples.md) | 0.20.3 (examples 30–32) |
| 045 | [Matrix Iteration and Graph/Probability Examples](./done/045-matrix-iteration-and-graph-probability-examples.md) | 0.20.4 (examples 33–34) |
| 046 | [Numerical Methods and Scientific Toy Examples](./done/046-numerical-methods-and-scientific-toy-examples.md) | 0.20.7 (35–36); 0.20.13 (39–40) |
| 047 | [Small ML-Like Examples Without ML-Framework Scope](./done/047-small-ml-like-examples-without-ml-framework-scope.md) | 0.20.8 (examples 37–38) |
| 048 | [Companion-Crate Examples](./done/048-companion-crate-examples.md) | 0.20.6 |
| 050 | [Production Migration Guide and Bridge Strategy](./done/050-production-migration-guide-and-bridge-strategy.md) | 0.23.0 |
| 051 | [Bridge Conversion Contracts and Companion-Crate Policy](./done/051-bridge-conversion-contracts-and-companion-crate-policy.md) | 0.23.2 |
| 052 | [Production Target Playbooks](./done/052-production-target-playbooks.md) | 0.23.0–0.23.1 |
| 053 | [Migration Readiness Diagnostics and Report Format](./done/053-migration-readiness-diagnostics-and-report-format.md) | 0.23.4 |
| 055 | [Result-Form Scalar Reductions (`try_sum`/`try_mean`/`try_min`/`try_max`/`try_norm`)](./done/055-result-form-scalar-reductions.md) | 0.24.0 |
| 056 | [Result-Form Axis Reductions (`try_sum_axis`/`try_mean_axis`/`try_min_axis`/`try_max_axis`)](./done/056-result-form-axis-reductions.md) | 0.24.0 |
| 057 | [Promote `matten-ndarray` — Production-Ready Candidate → Production-Ready](./done/057-promote-matten-ndarray-production-ready.md) | 0.25.0 |
| 058 | [Promote `matten-mlprep` — Beta → Production-Ready Candidate](./done/058-promote-matten-mlprep-production-ready-candidate.md) | 0.26.0 |
| 059 | [`matten-data` Maturity Decision — Beta → Production-Ready Candidate](./done/059-promote-matten-data-production-ready-candidate.md) | 0.27.0 |
| 060 | [Surface Benchmark Evidence in the Rendered Documentation](./done/060-surface-benchmark-evidence-in-docs.md) | 0.27.1 |
| 061 | [Maturity-Label Clarity — Keep "Production-Ready", Add an Entrance Note](./done/061-maturity-label-clarity-entrance-note.md) | 0.27.1 |
| 062 | [`matten-ndarray` Supported `ndarray` Version — 0.16 → 0.17](./done/062-matten-ndarray-supported-ndarray-version.md) | 0.28.0 |
| 063 | [Visual Understanding and Reporting](./done/063-visual-understanding-and-reporting.md) | 0.29.0 (visual docs, examples, local `matten-report` tool; public report/viz crates deferred) |
| 064 | [Workspace Core Dependency Requirement Maintenance Policy](./done/064-workspace-core-dependency-requirement-maintenance-policy.md) | post-0.29.0 repository policy |
| 065 | [Educational Visualization and Tensor Learning Path](./done/065-educational-visualization-and-tensor-learning-path.md) | 0.30.0 (educational positioning, learner docs path, local `educational-path` report; public report/viz crates deferred) |

## Proposed

| ID | Title | Target |
|---:|---|---|
| 026 | [Large CSV and Streaming Data Policy](./proposed/026-large-csv-and-streaming-data-policy.md) | design spike no earlier than v0.20+ |
| 037 | [Deferred Streaming and Large CSV Policy](./proposed/037-deferred-streaming-and-large-csv-policy.md) | v0.20.0 / later |
| 049 | [Benchmarking, Complexity Metrics, and Positioning Report](./proposed/049-benchmarking-complexity-metrics-and-positioning-report.md) | **Accepted** — Phase 1 baseline (0.22.1) + Phase 2 Rust peer comparison (accepted 2026-06-25) complete; Phases 3–4 deferred |
| 054 | [`matten-migrate` Assisted Migration Tool](./proposed/054-matten-migrate-assisted-migration-tool.md) | **Accepted as future direction** (2026-06-24; deferral confirmed in deep review 2026-06-27) — implementation deferred |
Implementation handoffs for the v0.20+ proposed set (RFC-033–042), the examples
program (RFC-043–048), and the benchmarking program (RFC-049) live in
[`./handoffs/`](./handoffs/). They translate each RFC into PR boundaries, checks,
and acceptance criteria; the RFC remains the design authority. The examples handoff
([`043-048-examples-implementation-handoff.md`](./handoffs/043-048-examples-implementation-handoff.md))
opens with a Phase 0 inventory of the existing example suite: new famous-problem
examples use an additive 30+ band, and existing distance/cosine/companion examples
are audited/improved rather than duplicated. The benchmarking handoff
([`049-benchmarking-developer-handoff.md`](./handoffs/049-benchmarking-developer-handoff.md))
keeps all benchmark tooling in an isolated `publish = false` package so core
`matten` gains no benchmark dependency. The Phase 3 Python reference-comparison
implementation handoff is tracked separately in
[`049-phase-3-python-reference-comparison-handoff.md`](./handoffs/049-phase-3-python-reference-comparison-handoff.md)
as the accepted code-shape-first NumPy/Pandas reference slice.

The production-migration set (RFC-050–054) ships its handoff bundle in
[`./handoffs/`](./handoffs/): the
[`050-053-production-migration-implementation-handoff.md`](./handoffs/050-053-production-migration-implementation-handoff.md)
covers the documentation/policy/template work (RFC-050–053) for v0.23.x, with an
[acceptance/QA checklist](./handoffs/050-053-acceptance-qa-checklist.md) and a
[release-guard checklist](./handoffs/050-053-release-guard-checklist.md); RFC-054's
[`054-deferred-implementation-note.md`](./handoffs/054-deferred-implementation-note.md)
keeps the `matten-migrate` CLI explicitly deferred. These RFCs add no core dependency: all
migration support lives in docs, bridge crates, and (later, if ever) workspace-excluded
tooling.
