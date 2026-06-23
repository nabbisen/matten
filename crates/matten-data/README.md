# matten-data

[![license](https://img.shields.io/crates/l/matten-data.svg)](../../LICENSE)

> **Experimental (scaffold, `0.20.x` family).** An approved, scope-locked companion
> crate (RFC-033). It currently has **no public API** — table ingestion and
> conversion arrive in later releases (RFC-034, RFC-035). Pin the minor version.

Part of the [`matten` workspace](../../README.md) — see it for the full family.

## Overview

`matten-data` will be a tiny helper for the boring step between table-like input
and a numeric `matten::Tensor`:

```text
small CSV / table-like data
  -> inspect schema
  -> select columns by name
  -> clean missing values explicitly
  -> convert to numeric explicitly
  -> matten::Tensor
```

That is its whole purpose. It helps you *reach* a `Tensor`; core `matten` computes
with the `Tensor`; `matten-mlprep` prepares the numeric `Tensor` for modelling.

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

- **Maturity:** experimental. The API (when added) may change; pin the version.
- **Scope lock:** table-to-Tensor preparation only (RFC-033, RFC-042).
- **Dependency direction:** `matten-data` depends on core `matten`; core never
  depends on `matten-data` (enforced by the dependency-boundary CI check).
- **Safe Rust only:** `#![forbid(unsafe_code)]`.

## Dependency style

This crate depends on `matten`. When the public API lands, official examples will
import `Tensor` from `matten` directly:

```rust
use matten::Tensor;
// use matten_data::Table;   // (not yet available)
```

Declare both `matten` and this crate in your `Cargo.toml` (RFC-032).

## Compatibility

- **SemVer:** pre-1.0 (`0.x`). Shares the `matten` family version (RFC-030).
- **MSRV:** Rust 1.85 (edition 2024).
