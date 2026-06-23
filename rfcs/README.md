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
| 038 | [Core Numeric Comfort APIs](./done/038-core-numeric-comfort-apis.md) | 0.20.9 (elementwise); 0.20.10 (selection); 0.20.11 (creation); 0.20.12 (shape) |
| 043 | [Example Program Structure, Quality Gate, and Documentation Policy](./done/043-example-program-structure-quality-gate-and-documentation-policy.md) | 0.20.3 |
| 044 | [Beginner Core Math Examples](./done/044-beginner-core-math-examples.md) | 0.20.3 (examples 30–32) |
| 045 | [Matrix Iteration and Graph/Probability Examples](./done/045-matrix-iteration-and-graph-probability-examples.md) | 0.20.4 (examples 33–34) |
| 046 | [Numerical Methods and Scientific Toy Examples](./done/046-numerical-methods-and-scientific-toy-examples.md) | 0.20.7 (35–36); 0.20.13 (39–40) |
| 047 | [Small ML-Like Examples Without ML-Framework Scope](./done/047-small-ml-like-examples-without-ml-framework-scope.md) | 0.20.8 (examples 37–38) |
| 048 | [Companion-Crate Examples](./done/048-companion-crate-examples.md) | 0.20.6 |

## Proposed

| ID | Title | Target |
|---:|---|---|
| 023 | [`matten-data` Scope and Non-goals](./proposed/023-matten-data-scope-and-non-goals.md) | experimental only before v0.20; beta decision no earlier than v0.20+ |
| 026 | [Large CSV and Streaming Data Policy](./proposed/026-large-csv-and-streaming-data-policy.md) | design spike no earlier than v0.20+ |
| 036 | [`matten-data` Examples, Documentation, and Release Gate](./proposed/036-matten-data-examples-documentation-and-release-gate.md) | v0.20.0 |
| 037 | [Deferred Streaming and Large CSV Policy](./proposed/037-deferred-streaming-and-large-csv-policy.md) | v0.20.0 / later |
| 039 | [Shape Composition API Boundary](./proposed/039-shape-composition-api-boundary.md) | v0.21+ |
| 040 | [Small Statistics Boundary — Core vs Companion](./proposed/040-small-statistics-boundary-core-vs-companion.md) | v0.21+ |
| 041 | [Linear Algebra Boundary — Core Lite vs External Crates](./proposed/041-linear-algebra-boundary-core-lite-vs-external-crates.md) | v0.21+ |
| 042 | [Pandas-Inspired Scope Guard for `matten-data`](./proposed/042-pandas-inspired-scope-guard-for-matten-data.md) | v0.20+ / v0.21+ |
| 049 | [Benchmarking, Complexity Metrics, and Positioning Report](./proposed/049-benchmarking-complexity-metrics-and-positioning-report.md) | v0.20.x planning / v0.21+ maturity hardening |

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
`matten` gains no benchmark dependency.
