# `matten-data` — table to `Tensor`

`matten-data` is a small, **production-ready candidate** companion crate for the boring step between a
small table-like input (such as a CSV) and a numeric [`matten::Tensor`]. It is a
conversion helper, **not** a dataframe library or query engine.

For joins, group-by, lazy queries, datetime handling, or large/streaming data, use
[Polars](https://pola.rs), [DataFusion](https://datafusion.apache.org), or Pandas.
`matten-data` deliberately does none of those.

## Install

```toml
[dependencies]
matten = "0.27"
matten-data = "0.27"
```

Both crates share one lock-step family version (RFC-030); maturity is a per-crate
Status label, not a separate version number.

## Quickstart

```rust
use matten::Tensor;
use matten_data::Table;

# fn main() -> Result<(), matten_data::MattenDataError> {
let csv = "region,sales,cost\nnorth,100,40\nsouth,150,\neast,120,55";

let tensor: Tensor = Table::from_csv_str(csv)?
    .select_columns(["sales", "cost"])? // choose columns by name, in this order
    .fill_missing(0.0)?                  // the missing south/cost becomes 0.0
    .try_numeric()?                      // strict, explicit conversion to f64
    .to_tensor()?;                       // a normal [rows, columns] Tensor

assert_eq!(tensor.shape(), &[3, 2]);
# Ok(())
# }
```

## The example suite

The numbered tutorial suite teaches one step at a time; `csv_to_tensor` is a single
comprehensive overview.

| Example | What it shows |
|---|---|
| [`data_00_quickstart`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_00_quickstart.rs) | The full happy path in one place |
| [`data_01_schema_summary`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_01_schema_summary.rs) | Row/column counts, names, missing counts, inferred kinds |
| [`data_02_select_columns`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_02_select_columns.rs) | Select by name; output order matches the request |
| [`data_03_missing_values`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_03_missing_values.rs) | Missing values never become zero silently; explicit fill |
| [`data_04_to_tensor`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_04_to_tensor.rs) | Output shape, row-major order, core `matten` interop |
| [`data_05_errors`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_05_errors.rs) | Duplicate header, ragged row, non-numeric, missing-at-conversion |
| [`csv_to_tensor`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/csv_to_tensor.rs) | Comprehensive overview of the whole workflow |

```bash
cargo run -p matten-data --example data_00_quickstart
```

## Output `Tensor` shape

`to_tensor` produces a tensor of shape `[rows, columns]`, where rows are the data
rows (the header is not a row) and columns are the selected columns in the order you
requested them. The data is **row-major**: row 0's values come first, then row 1's,
and so on. Once converted, the result is an ordinary `matten::Tensor` — every core
operation applies.

## Missing-value policy

Missing cells are never silently turned into `0`. A missing value that reaches
numeric conversion is a precise `MissingValue { column, row }` error (the row is the
1-based CSV line number). You decide what a missing value means by calling
`fill_missing` with an explicit value before converting.

## Numeric conversion policy

Conversion is strict and explicit (`try_numeric` then `to_tensor`): integers and
floats become `f64`; booleans and non-numeric text are rejected (they are never
coerced to `1`/`0`); and a remaining missing cell is rejected. This keeps the
boundary between "table-like text" and "numbers" honest and visible.

## Limitations

`matten-data` has no joins, group-by, pivot, query DSL, lazy execution,
indexing/`loc`/`iloc`, rolling/window operations, datetime engine, categorical dtype
system, or large-data streaming. It is for small, application-validated or trusted
data. When you need those capabilities, reach for a dataframe/query engine
(Polars, DataFusion, Pandas) instead.

## Status and maturity

**Production-ready candidate** (`0.27.x` family). The table-to-Tensor API is mostly stable but pre-1.0;
pin the minor version. The crate's scope is locked and enforced in CI (RFC-042), and
core `matten` never depends on it (RFC-022).
