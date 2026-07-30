# matten-data

[![license](https://img.shields.io/crates/l/matten-data.svg)](../../LICENSE)

> **Production-ready (`0.41.x` family).** A scope-locked companion crate (RFC-033).
> The table-to-Tensor API (CSV ingestion, schema summary, column selection,
> missing-value handling, explicit numeric conversion) shipped in v0.20.1
> (RFC-034, RFC-035) and was promoted to Beta in v0.22.0 (RFC-036), then to production-ready candidate in v0.27.0 (RFC-059), then to production-ready (RFC-085). The API is
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
window operations, datetime engine, or categorical dtype system.

For dataframe or query workloads use
[Polars](https://pola.rs), [DataFusion](https://datafusion.apache.org), Pandas, or
another dataframe/query tool. `matten-data` is a small conversion helper for
application-validated or trusted data, not a CSV firewall or malicious-input
sandbox.

## Streaming (optional, `streaming` feature)

`CsvBatchReader` (RFC-082) reads a CSV file in row-count-bounded `Table` batches,
off by default behind the `streaming` feature — the `csv` feature is implied, no
new dependency. This is a memory strategy, not a dataframe engine: no schema
evolution, no lenient/skip-malformed mode, no streaming numeric conversion, and
no async. A batch is exactly a `Table`; concatenating every batch reproduces
`Table::from_csv_path`'s output for ordinary input. Two malformed-input edge
cases deliberately diverge from `Table::from_csv_path` (a file that is blank
but not empty — one that trims to nothing but contains at least one
whitespace character other than a line terminator, such as a space or tab —
and invalid UTF-8) — see `CsvBatchReader`'s own documentation for the exact
behavior. This crate's production-ready promotion (RFC-085) covers the `streaming`
feature too: stable in what it does — its two methods and their semantics are
settled — but its scope may still grow (RFC-082 §5 defers nine further items,
including async and resumability).

```toml
[dependencies]
matten-data = { version = "0.41.0", features = ["streaming"] }
```

```rust
use matten_data::CsvBatchReader;

# fn main() -> Result<(), matten_data::MattenDataError> {
# let path = std::path::Path::new("large.csv");
let mut reader = CsvBatchReader::open(path, 10_000)?;
while let Some(batch) = reader.next_batch()? {
    // process one Table batch at a time
}
# Ok(())
# }
```

## Relationship to core `dynamic`

Core `matten`'s `dynamic` feature is *value-level* ingestion (mixed values inside a
`Tensor`, with explicit `try_numeric()`). `matten-data` is *table-level* preparation
(headers, named columns, schema summary, table-shaped missing-value policy) whose
end goal is a numeric `Tensor`. It may use core `dynamic` internally but does not
expose a second computation engine.

## Status and scope

- **Maturity:** production-ready (RFC-085). The table-to-Tensor API is mostly stable but pre-1.0; pin the release explicitly.
- **Scope lock:** table-to-Tensor preparation only (RFC-033, RFC-042).
- **Dependency direction:** `matten-data` depends on core `matten`; core never
  depends on `matten-data` (enforced by the dependency-boundary CI check).
- **Safe Rust only:** `#![forbid(unsafe_code)]`.

## Public API

The complete surface (the breaking-change baseline for this crate):

```rust
impl Table {
    pub fn from_csv_str(input: &str)                      -> Result<Table, MattenDataError>;
    pub fn from_csv_path<P: AsRef<Path>>(path: P)          -> Result<Table, MattenDataError>;
    pub fn row_count(&self)                                -> usize;
    pub fn column_count(&self)                             -> usize;
    pub fn column_names(&self)                             -> &[String];
    pub fn schema_summary(&self)                           -> SchemaSummary;
    pub fn select_columns<I, S>(&self, columns: I)         -> Result<Table, MattenDataError>;
    pub fn fill_missing(&self, value: impl Into<CellValue>) -> Result<Table, MattenDataError>;
    pub fn try_numeric(&self)                              -> Result<NumericTable, MattenDataError>;
}

impl NumericTable {
    pub fn row_count(&self)      -> usize;
    pub fn column_count(&self)   -> usize;
    pub fn column_names(&self)   -> &[String];
    pub fn to_tensor(&self)      -> Result<matten::Tensor, MattenDataError>;
}

// `streaming` feature only (off by default; implies `csv`) — RFC-082
impl CsvBatchReader {
    pub fn open(path: &Path, batch_rows: usize) -> Result<Self, MattenDataError>;
    pub fn next_batch(&mut self)                -> Result<Option<Table>, MattenDataError>;
}

pub enum CellValue { Text(String), Float(f64), Int(i64), Bool(bool), Missing }

#[non_exhaustive]
pub enum ColumnKind { Integer, Float, Boolean, Text, Mixed, MissingOnly }

pub struct ColumnSummary { pub name: String, pub kind: ColumnKind, pub missing: usize }

pub struct SchemaSummary {
    pub rows: usize,
    pub columns: usize,
    // per-column detail via column_summaries() -> &[ColumnSummary]
}

#[non_exhaustive]
pub enum MattenDataError {
    Csv { message: String },
    Io { path: PathBuf, source: std::io::Error },
    EmptyInput,
    MissingColumn { name: String },
    DuplicateColumn { name: String },
    DuplicateSelection { name: String },
    RaggedRow { row: usize, expected: usize, actual: usize },
    NonNumericValue { column: String, row: usize, value: String },
    MissingValue { column: String, row: usize },
    EmptySelection,
    InvalidBatchSize, // `streaming` feature only
    Matten(matten::MattenError),
}
```

## Dependency style

This crate depends on `matten`. Official examples import `Tensor` from `matten` directly:

```rust
use matten::Tensor;
use matten_data::Table;
```

Declare both `matten` and this crate in your `Cargo.toml` (RFC-032).

## Compatibility

- **SemVer:** pre-1.0 (`0.x`). Released with the `matten` family version (RFC-030).
- **`matten`:** the published manifest uses the workspace's broad pre-1.0 core
  requirement for maintenance (`matten = "0"`, RFC-064); users should still
  declare the matched family explicitly.
- **MSRV:** Rust 1.85 (edition 2024).
