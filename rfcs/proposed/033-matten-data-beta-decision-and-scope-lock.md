# RFC-033: `matten-data` Beta-Decision and Scope Lock

**Status:** Proposed  
**Target Release:** v0.20.0 decision/design milestone  
**Owner:** `matten` maintainers  
**Related:** RFC-022, RFC-023, RFC-030, RFC-034, RFC-035, RFC-036, RFC-037, RFC-042  
**Scope:** Companion crate decision and scope lock  
**RFC Numbering Note:** RFC-032 is intentionally not used here because it has been consumed by another issue.

---

## 1. Summary

This RFC decides whether the project should proceed with `matten-data` as a small optional companion crate.

The recommended decision is:

```text
Proceed with `matten-data` as an experimental companion crate,
but lock its scope to small table-to-Tensor preparation workflows.
Do not promote it to beta in v0.20.0.
Do not allow dataframe, query, or large-data scope.
```

The crate's purpose is:

```text
small CSV / table-like input
  -> inspect schema
  -> select columns
  -> clean missing values
  -> explicitly convert to numeric values
  -> produce matten::Tensor
```

The crate's purpose is **not**:

```text
dataframe analytics
query engine
large-data engine
Pandas clone
Polars clone
DataFusion clone
ML preprocessing pipeline
```

---

## 2. Motivation

After v0.19, `matten` has a stable pre-1.0 core and two companions:

```text
matten
  core Tensor library

matten-ndarray
  Tensor <-> ndarray bridge

matten-mlprep
  deterministic preprocessing helpers
```

The next natural user pain is not numerical computation itself. It is reaching a clean numeric `Tensor` from common business-like inputs:

- CSV files;
- small table-like data;
- named columns;
- missing values;
- text values mixed with numeric values;
- explicit conversion into a numeric matrix.

Core `matten` already has dynamic ingestion, but dynamic is value-level and Tensor-oriented. It does not provide a table model, column names, schema summary, or named-column selection.

`matten-data` can fill this gap, but it is dangerous because table APIs tend to grow into dataframe APIs. This RFC therefore treats v0.20 as a **decision and scope lock**, not automatic beta promotion.

---

## 3. Product Position

`matten-data` should be described as:

> A tiny table-to-Tensor preparation companion for small PoC datasets.

It should not be described as:

- a dataframe library;
- a data analytics framework;
- a CSV engine;
- a query processor;
- a Pandas replacement;
- a Polars replacement.

The correct mental model:

```text
matten-data helps users reach Tensor.
matten computes with Tensor.
matten-mlprep prepares numeric Tensor.
```

---

## 4. Goals

`matten-data` v0.20 experimental SHOULD support:

1. Loading small CSV/table-like data from a string or path.
2. Inspecting column names and a small schema summary.
3. Selecting columns by name.
4. Handling missing values explicitly.
5. Converting selected values to numeric data explicitly.
6. Producing `matten::Tensor` with documented shape.
7. Returning crate-local errors with useful row/column context.
8. Remaining optional and dependency-light.
9. Preserving core `matten` dependency boundaries.

---

## 5. Non-goals

`matten-data` v0.20 MUST NOT include:

- joins;
- merge;
- group-by;
- pivot;
- window functions;
- rolling operations;
- query DSL;
- SQL-like expressions;
- lazy execution;
- index alignment;
- multi-index;
- datetime engine;
- categorical dtype system;
- automatic feature engineering;
- model preprocessing beyond conversion to `Tensor`;
- streaming CSV implementation;
- Arrow/Polars/DataFusion dependency;
- async API;
- remote data loading.

These may be useful in other ecosystems, but they do not fit v0.20 `matten-data`.

---

## 6. Decision

`matten-data` is approved only as an **experimental** companion crate candidate.

The allowed v0.20 path is:

```text
v0.20.0
  RFCs accepted
  optional crate scaffold
  no beta claim

v0.20.x
  minimal implementation if approved by maintainers
  examples and release gate

v0.21+
  beta / keep experimental / freeze decision
```

`matten-data` MUST remain separate from core `matten`.

Allowed dependency direction:

```text
matten-data -> matten
```

Forbidden dependency direction:

```text
matten -> matten-data
```

---

## 7. Maturity Label

Initial maturity:

```text
Experimental
```

README/rustdoc text SHOULD say:

> `matten-data` is experimental. It is intended for small table-to-Tensor preparation workflows. APIs may change before beta. Use Polars, DataFusion, or another dataframe/query system for serious dataframe or large-data workflows.

Beta is not granted by this RFC.

A future beta decision must evaluate real implementation evidence.

---

## 8. Allowed User Workflow

The canonical workflow:

```rust
use matten_data::Table;

let table = Table::from_csv_path("sales.csv")?;

println!("{}", table.schema_summary());

let x = table
    .select_columns(["sales", "cost", "quantity"])?
    .fill_missing(0.0)?
    .try_numeric()?
    .to_tensor()?;
```

Output shape:

```text
[rows, selected_columns]
```

Order guarantees:

```text
row order = input row order
column order = requested selection order
```

---

## 9. Relationship to Core `dynamic`

Core `dynamic` is value-level ingestion:

```text
mixed values inside Tensor
missing-value cleanup
explicit try_numeric()
```

`matten-data` is table-level preparation:

```text
headers
columns
schema summary
named selection
table-shaped missing-value policy
conversion to Tensor
```

`matten-data` MAY use core `dynamic` internally, but it MUST NOT expose a second computation engine.

Rule:

```text
dynamic is value-level ingestion.
matten-data is column/table-level preparation to reach Tensor.
```

---

## 10. Dependency Policy

`matten-data` may depend on:

```text
matten
csv
serde, only if needed and already accepted by workspace policy
```

`matten-data` MUST NOT depend on:

```text
polars
arrow
datafusion
duckdb
sqlx
candle
nalgebra
ndarray, unless a future RFC gives a precise reason
```

Core `matten` dependency-boundary CI must continue to prove that core does not depend on `matten-data`.

---

## 11. Security and Reliability Requirements

All external input APIs MUST return `Result`.

Malformed CSV must not panic.

Required error cases:

- empty input;
- malformed CSV;
- ragged rows;
- duplicate header, if rejected by policy;
- missing column;
- non-numeric value during numeric conversion;
- invalid missing-value fill;
- output Tensor construction failure.

`matten-data` is not a sandbox. Documentation SHOULD say:

> `matten-data` is a small conversion helper for application-validated or trusted data. It is not a secure CSV firewall or malicious input sandbox.

---

## 12. Acceptance Criteria

This RFC is accepted only if the project agrees to all of the following:

```text
[ ] matten-data starts experimental, not beta
[ ] allowed workflow is table-to-Tensor only
[ ] forbidden dataframe/query scope is explicit
[ ] output Tensor shape is [rows, selected_columns]
[ ] core matten remains dependency-free from matten-data
[ ] heavy dataframe/query dependencies are forbidden
[ ] streaming is deferred to RFC-037
[ ] beta decision is deferred until implementation evidence exists
```

---

## 13. Implementation Handoff

After this RFC is accepted, the implementation team should:

1. Implement RFC-034 first if the public table model is approved.
2. Implement RFC-035 for CSV/missing/numeric conversion behavior.
3. Implement RFC-036 for examples/docs/CI gates.
4. Keep all APIs behind `matten-data`, never core `matten`.
5. Keep README status as experimental until a future beta decision.

---

## 14. Rejection Conditions

The crate should be frozen or deferred if:

- the API starts needing joins/group-by/pivot;
- users cannot understand the crate without dataframe concepts;
- Polars/DataFusion would solve the target use case better;
- the implementation requires heavy dependencies;
- missing-value policy becomes too large;
- streaming is required before the crate is useful.

---

## 15. Final Decision Request

Approve `matten-data` as an experimental v0.20+ companion only under the scope lock above.
