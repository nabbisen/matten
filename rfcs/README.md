# `matten` RFCs

Design decisions for `matten` are tracked here using the lifecycle policy in
[RFC-000](./done/000-rfc-lifecycle-policy.md). The folder is the source of truth
for RFC state; the Status field inside each file mirrors the folder.

## Done

| ID | Title | Shipped in |
|---:|---|---|
| 000 | [— RFC lifecycle policy](./done/000-rfc-lifecycle-policy.md) | — |
| 001 | [Threat Model and Boundary Safety Policy](./done/001-threat-model-and-boundary-safety-policy.md) | — |
| 002 | [Public API Minimalism and `Tensor` Contract](./done/002-public-api-minimalism-and-tensor-contract.md) | — |
| 003 | [Shape Model, Scalar Semantics, and Validation](./done/003-shape-model-scalar-semantics-and-validation.md) | — |
| 004 | [Construction and Conversion APIs](./done/004-construction-and-conversion-apis.md) | — |
| 005 | [Error Model, Panic Messages, and Boundary APIs](./done/005-error-model-panic-messages-and-boundary-apis.md) | — |
| 006 | [Broadcasting and Element-Wise Operators](./done/006-broadcasting-and-element-wise-operators.md) | — |
| 007 | [Reshape, Axis Operations, and Indexing](./done/007-reshape-axis-operations-and-indexing.md) | — |
| 008 | [Slicing API: Builder and `slice_str`](./done/008-slicing-api-builder-and-slice-str.md) | — |
| 009 | [Serde, JSON, CSV, and Boundary Integration](./done/009-serde-json-csv-and-boundary-integration.md) | — |
| 010 | [Reductions, Basic Statistics, and Matrix Multiplication](./done/010-reductions-basic-statistics-and-matrix-multiplication.md) | — |
| 011 | [Dynamic `Element` Model and Coercion](./done/011-dynamic-element-model-and-coercion.md) | — |
| 012 | [Dynamic Storage, View Metadata, and Copy-on-Write](./done/012-dynamic-storage-view-metadata-and-cow.md) | — |
| 013 | [Testing, Compatibility, and Release Gates](./done/013-testing-compatibility-and-release-gates.md) | — |
| 014 | [Example Suite and Executable Documentation](./done/014-example-suite-and-executable-documentation.md) | — |
| 015 | [Public API Stabilization and Compatibility Policy](./done/015-public-api-stabilization-and-compatibility-policy.md) | v0.13.3 |
| 020 | [Human-Readable Diagnostics and Error Message Quality](./done/020-human-readable-diagnostics-and-error-message-quality.md) | v0.13.3 |

## Proposed

| ID | Title | Target |
|---:|---|---|
| 016 | [Dynamic Ingestion and Explicit Numeric On-Ramp](./proposed/016-dynamic-ingestion-and-explicit-numeric-on-ramp.md) | v0.14.x |
| 017 | [Numeric Conversion Policy](./proposed/017-numeric-conversion-policy.md) | v0.14.x |
| 018 | [Shape, Allocation, and Resource Safety Limits](./proposed/018-shape-allocation-and-resource-safety-limits.md) | v0.14.x |
| 019 | [Axis Reductions and Small Matrix Statistics](./proposed/019-axis-reductions-and-small-matrix-statistics.md) | v0.15.x |
| 021 | [Tutorial Path and Example Quality Gate](./proposed/021-tutorial-path-and-example-quality-gate.md) | v0.15.x |
| 022 | [Companion Crate Boundary Policy](./proposed/022-companion-crate-boundary-policy.md) | v0.16+ |
| 023 | [`matten-data` Scope and Non-goals](./proposed/023-matten-data-scope-and-non-goals.md) | v0.16+ design, v0.17+ possible PoC |
| 024 | [`matten-mlprep` Scope and Non-goals](./proposed/024-matten-mlprep-scope-and-non-goals.md) | v0.16+ design, v0.18+ possible PoC |
| 025 | [Bridge Crate Policy for ndarray, nalgebra, and candle](./proposed/025-bridge-crate-policy-for-ndarray-nalgebra-and-candle.md) | v0.16+ design, v0.19+ possible PoC |
| 026 | [Large CSV and Streaming Data Policy](./proposed/026-large-csv-and-streaming-data-policy.md) | v0.16+ design, v0.20+ exploration |
