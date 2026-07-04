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
matten = "0.29.0-pre.6"
matten-data = "0.29.0-pre.6"
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

The data path is intentionally explicit:

```text
CSV text
  |
  v
Table
  headers: region, sales, cost
  rows:    3
  |
  | select_columns(["sales", "cost"])
  v
Table
  headers: sales, cost
  rows:    3
  |
  | fill_missing(0.0)
  v
Table
  missing cost cell is now an explicit numeric value
  |
  | try_numeric()
  v
NumericTable
  all selected cells are f64-compatible
  |
  | to_tensor()
  v
Tensor shape [3, 2]
  rows    = CSV data rows
  columns = selected columns, in requested order
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
| [`data_06_visual_readiness_summary`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_06_visual_readiness_summary.rs) | Readability summary for selected columns, missing counts, conversion, and Tensor shape |
| [`csv_to_tensor`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/csv_to_tensor.rs) | Comprehensive overview of the whole workflow |

```bash
cargo run -p matten-data --example data_00_quickstart
cargo run -p matten-data --example data_06_visual_readiness_summary
```

## Output `Tensor` shape

`to_tensor` produces a tensor of shape `[rows, columns]`, where rows are the data
rows (the header is not a row) and columns are the selected columns in the order you
requested them. The data is **row-major**: row 0's values come first, then row 1's,
and so on. Once converted, the result is an ordinary `matten::Tensor` — every core
operation applies.

For the quickstart input, selecting `sales` and `cost` gives:

```text
source rows

row 0: region=north
row 1: region=south
row 2: region=east

selected columns: sales, cost

selected table

row 0: sales=100  cost=40
row 1: sales=150  cost=0      (filled explicitly)
row 2: sales=120  cost=55

Tensor shape [3, 2]

[ 100   40 ]
[ 150    0 ]
[ 120   55 ]

flat row-major data:

[100, 40, 150, 0, 120, 55]
```

## Missing-value policy

Missing cells are never silently turned into `0`. A missing value that reaches
numeric conversion is a precise `MissingValue { column, row }` error (the row is the
1-based CSV line number). You decide what a missing value means by calling
`fill_missing` with an explicit value before converting.

The policy is visible in the workflow:

```text
missing cell present
        |
        | try_numeric()
        v
MissingValue error

missing cell present
        |
        | fill_missing(value)
        v
explicit value present
        |
        | try_numeric()
        v
NumericTable
```

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

**Production-ready candidate** (`0.29.x` family). The table-to-Tensor API is mostly stable but pre-1.0;
pin the prerelease explicitly. The crate's scope is locked and enforced in CI (RFC-042), and
core `matten` never depends on it (RFC-022).
