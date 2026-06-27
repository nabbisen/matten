# matten-data

[![license](https://img.shields.io/crates/l/matten-data.svg)](../../LICENSE)

> **Beta (`0.24.x` family).** A scope-locked companion crate (RFC-033).
> The table-to-Tensor API (CSV ingestion, schema summary, column selection,
> missing-value handling, explicit numeric conversion) shipped in v0.20.1
> (RFC-034, RFC-035) and was promoted to Beta in v0.22.0 (RFC-036). The API is
> mostly stable but pre-1.0; pin the minor version.

Part of the [`matten` workspace](../../README.md) — see it for the full family.

## Overview

`matten-data` is a tiny helper for the boring step between table-like input and a
numeric `matten::Tensor`:

```rust
use matten_data::Table;

let csv = "sales,cost,note\n10,2,a\n20,,b\n30,4,c";
let tensor = Table::from_csv_str(csv)?
    .select_columns(["sales", "cost"])?   // pick numeric columns, by name
    .fill_missing(0.0)?                    // clean missing values explicitly
    .try_numeric()?                        // strict, explicit conversion
    .to_tensor()?;                         // -> matten::Tensor, shape [3, 2]

assert_eq!(tensor.shape(), &[3, 2]);
assert_eq!(tensor.as_slice(), &[10.0, 2.0, 20.0, 0.0, 30.0, 4.0]);
# Ok::<(), matten_data::MattenDataError>(())
```

Output shape is `[rows, selected_columns]`; row order is the input row order and
column order is the requested selection order. See
[`examples/`](./examples/) for a runnable version.

## Not a dataframe library

`matten-data` is **not a dataframe library**. It deliberately has no joins,
group-by, pivot, query DSL, lazy execution, indexing / `loc` / `iloc`, rolling or
window operations, datetime engine, categorical dtype system, or large-data
streaming.

For dataframe, query, or large-data workloads use
[Polars](https://pola.rs), [DataFusion](https://datafusion.apache.org), Pandas, or
another dataframe/query tool. `matten-data` is a small conversion helper for
application-validated or trusted data, not a CSV firewall or malicious-input
sandbox.

## Relationship to core `dynamic`

Core `matten`'s `dynamic` feature is *value-level* ingestion (mixed values inside a
`Tensor`, with explicit `try_numeric()`). `matten-data` is *table-level* preparation
(headers, named columns, schema summary, table-shaped missing-value policy) whose
end goal is a numeric `Tensor`. It may use core `dynamic` internally but does not
expose a second computation engine.

## Status and scope

- **Maturity:** beta. The table-to-Tensor API is mostly stable but pre-1.0; pin the minor version.
- **Scope lock:** table-to-Tensor preparation only (RFC-033, RFC-042).
- **Dependency direction:** `matten-data` depends on core `matten`; core never
  depends on `matten-data` (enforced by the dependency-boundary CI check).
- **Safe Rust only:** `#![forbid(unsafe_code)]`.

## Dependency style

This crate depends on `matten`. Official examples import `Tensor` from `matten` directly:

```rust
use matten::Tensor;
use matten_data::Table;
```

Declare both `matten` and this crate in your `Cargo.toml` (RFC-032).

## Compatibility

- **SemVer:** pre-1.0 (`0.x`). Shares the `matten` family version (RFC-030).
- **MSRV:** Rust 1.85 (edition 2024).
