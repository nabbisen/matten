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
| 027 | [`matten-ndarray` Design and Implementation](./done/027-matten-ndarray-design-and-implementation.md) | 0.17.0 (matten-ndarray 0.1.0) |
| 028 | [`matten-mlprep` Design and Implementation](./done/028-matten-mlprep-design-and-implementation.md) | 0.18.0 (matten-mlprep 0.1.0) |
| 029 | [Companion Maturity Evaluation (v0.19.0)](./done/029-companion-maturity-evaluation-v0-19.md) | 0.19.0 |
| 030 | [Workspace Versioning Model — Lock-step Family Versioning](./done/030-workspace-versioning-model-lockstep.md) | 0.19.0 |

## Proposed

| ID | Title | Target |
|---:|---|---|
| 023 | [`matten-data` Scope and Non-goals](./proposed/023-matten-data-scope-and-non-goals.md) | experimental only before v0.20; beta decision no earlier than v0.20+ |
| 024 | [`matten-mlprep` Scope and Non-goals](./proposed/024-matten-mlprep-scope-and-non-goals.md) | v0.18 experimental; v0.19 beta decision |
| 025 | [Bridge Crate Policy for ndarray, nalgebra, and candle](./proposed/025-bridge-crate-policy-for-ndarray-nalgebra-and-candle.md) | policy in v0.16; `matten-ndarray` experimental in v0.17; nalgebra/candle deferred |
| 026 | [Large CSV and Streaming Data Policy](./proposed/026-large-csv-and-streaming-data-policy.md) | design spike no earlier than v0.20+ |
