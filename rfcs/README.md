# `matten` RFCs

Design decisions for `matten` are tracked here using the lifecycle policy in
[RFC-000](./done/000-rfc-lifecycle-policy.md). The folder is the source of truth
for RFC state; the Status field inside each file mirrors the folder.

## Done

| ID | Title | Shipped in |
|---:|---|---|
| 000 | [RFC lifecycle policy](./done/000-rfc-lifecycle-policy.md) | — |
| 002 | [Public API Minimalism and `Tensor` Contract](./done/002-public-api-minimalism-and-tensor-contract.md) | 0.1.0 |
| 003 | [Shape Model, Scalar Semantics, and Validation](./done/003-shape-model-scalar-semantics-and-validation.md) | 0.1.0 |
| 004 | [Construction and Conversion APIs](./done/004-construction-and-conversion-apis.md) | 0.2.0 |
| 005 | [Error Model, Panic Messages, and Boundary APIs](./done/005-error-model-panic-messages-and-boundary-apis.md) | 0.1.0 |
| 006 | [Broadcasting and Element-Wise Operators](./done/006-broadcasting-and-element-wise-operators.md) | 0.3.0 |
| 007 | [Reshape, Axis Operations, and Indexing](./done/007-reshape-axis-operations-and-indexing.md) | 0.4.0 |
| 008 | [Slicing API: Builder and `slice_str`](./done/008-slicing-api-builder-and-slice-str.md) | 0.4.0 |

## Proposed

| ID | Title |
|---:|---|
| 001 | [Threat Model and Boundary Safety Policy](./proposed/001-threat-model-and-boundary-safety-policy.md) |
| 009 | [Serde, JSON, CSV, and Boundary Integration](./done/009-serde-json-csv-and-boundary-integration.md) | 0.5.0 |
| 010 | [Reductions, Basic Statistics, and Matrix Multiplication](./proposed/010-reductions-basic-statistics-and-matrix-multiplication.md) |
| 011 | [Dynamic `Element` Model and Coercion](./proposed/011-dynamic-element-model-and-coercion.md) |
| 012 | [Dynamic Storage, View Metadata, and Copy-on-Write](./proposed/012-dynamic-storage-view-metadata-and-cow.md) |
