# RFC-034: `matten-data` Table Model and Public API Boundary

**Status:** Proposed  
**Target Release:** v0.20.0 design; v0.20.x implementation if RFC-033 approves  
**Related:** RFC-033, RFC-035, RFC-036, RFC-042  
**Scope:** Public table model and API boundary for `matten-data`

---

## 1. Summary

This RFC defines the minimal public model for `matten-data`.

The proposed public surface is intentionally small:

```rust
pub struct Table;
pub struct SchemaSummary;
pub struct NumericTable;

#[non_exhaustive]
pub enum MattenDataError { ... }
```

The canonical user workflow is:

```rust
use matten_data::Table;

let table = Table::from_csv_str(csv)?;

let x = table
    .select_columns(["sales", "cost", "quantity"])?
    .fill_missing(0.0)?
    .try_numeric()?
    .to_tensor()?;
```

`Table` is not a dataframe. It is a small owned table-like preparation object whose end goal is `matten::Tensor`.

---

## 2. Motivation

Users often start with named CSV/table data and want a numeric matrix.

Core `matten::Tensor` is intentionally shape-first and numeric-first. Core dynamic ingestion handles mixed values, but it does not provide:

- column names;
- schema summary;
- named-column selection;
- table-shaped missing-value reporting;
- row/column conversion errors.

`matten-data` can provide those without polluting core.

---

## 3. Design Goals

The API should be:

- easy to learn;
- small enough to fit in a README;
- explicit about conversion;
- explicit about missing values;
- predictable in row/column order;
- strict about malformed data;
- crate-local in errors;
- not dataframe-like.

---

## 4. Public Types

### 4.1 `Table`

`Table` represents a small, owned, rectangular, table-like data set.

Conceptually:

```rust
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<CellValue>>,
}
```

The actual fields are private.

External guarantees:

- row order is preserved;
- column order is preserved;
- column names are stable after loading;
- operations return new `Table` values or documented owned results;
- no borrowed view lifetimes appear in normal use.

### 4.2 `CellValue`

This RFC allows but does not require exposing `CellValue`.

If exposed, it should be:

```rust
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum CellValue {
    Text(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Missing,
}
```

Alternative: keep `CellValue` private and expose only higher-level operations.

Recommendation:

```text
Expose CellValue only if needed for user inspection.
Do not make CellValue central to the beginner path.
```

**`CellValue` is intentionally crate-local (architect ruling, RFC-033–042 review Q4).**
It models table-ingestion cells, not core `Tensor` dynamic values. It is distinct
from core `matten::Element` and is **not** an alias:

```rust
// FORBIDDEN
pub type CellValue = matten::Element;
```

The representation difference is deliberate:

```text
core matten::Element::Text(Arc<str>)   // optimizes repeated tensor values / clone behavior
matten-data CellValue::Text(String)    // optimizes simple owned CSV/table ingestion
```

Keeping them distinct lets table-parsing policy (headers, schema, missing-value
handling) evolve independently of core dynamic ingestion, and lets `matten-data`
change its internal cell storage later without changing the public model (fields
remain private). `matten-data` MAY convert `CellValue` into core `Element` later
if needed, but it does not expose `Element` as its primary cell type.

### 4.3 `SchemaSummary`

`SchemaSummary` is a displayable description of columns.

It SHOULD include:

- row count;
- column count;
- column names;
- inferred simple column kind;
- missing count per column.

Possible shape:

```rust
pub struct SchemaSummary {
    pub rows: usize,
    pub columns: usize,
    // details TBD
}
```

Fields may stay private if display methods are enough.

### 4.4 `NumericTable`

`NumericTable` is an optional intermediate type representing a table that has been explicitly converted to numeric values.

```rust
pub struct NumericTable;
```

It exists to make this lifecycle visible:

```text
Table
  -> select columns
  -> fill missing values
  -> try_numeric()
  -> NumericTable
  -> Tensor
```

This prevents `to_tensor()` from silently parsing text.

Recommendation:

```text
Use NumericTable if it keeps conversion explicit.
Avoid it if the implementation becomes too ceremony-heavy.
```

---

## 5. Public API

### 5.1 Construction

```rust
impl Table {
    pub fn from_csv_str(input: &str) -> Result<Table, MattenDataError>;

    pub fn from_csv_path(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Table, MattenDataError>;
}
```

Optional future:

```rust
pub fn from_records(headers: Vec<String>, rows: Vec<Vec<String>>) -> Result<Table, MattenDataError>;
```

Do not add database/network loaders in v0.20.

### 5.2 Inspection

```rust
impl Table {
    pub fn row_count(&self) -> usize;
    pub fn column_count(&self) -> usize;
    pub fn column_names(&self) -> &[String];
    pub fn schema_summary(&self) -> SchemaSummary;
}
```

If `column_names()` returning `&[String]` is considered too exposing, use:

```rust
pub fn column_names(&self) -> impl Iterator<Item = &str>;
```

But for beginner DX, `&[String]` is acceptable.

### 5.3 Selection

```rust
impl Table {
    pub fn select_columns<I, S>(&self, columns: I) -> Result<Table, MattenDataError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;
}
```

Behavior:

- errors if a requested column is missing;
- preserves requested column order;
- duplicate selection names are either rejected or duplicated by explicit policy;
- recommendation: reject duplicate selections in v0.20 for simplicity.

### 5.4 Missing-Value Handling

```rust
impl Table {
    pub fn fill_missing(&self, value: impl Into<CellValue>) -> Result<Table, MattenDataError>;
}
```

If `CellValue` is private, use:

```rust
pub fn fill_missing_text(&self, value: impl Into<String>) -> Result<Table, MattenDataError>;
pub fn fill_missing_f64(&self, value: f64) -> Result<Table, MattenDataError>;
```

Recommendation:

```text
Prefer one generic `fill_missing` if CellValue is public.
Prefer explicit typed fill helpers if CellValue stays private.
```

### 5.5 Numeric Conversion

```rust
impl Table {
    pub fn try_numeric(&self) -> Result<NumericTable, MattenDataError>;
}

impl NumericTable {
    pub fn to_tensor(&self) -> Result<matten::Tensor, MattenDataError>;
}
```

`try_numeric()` must be explicit.

`Table::to_tensor()` should not exist unless it is named clearly:

```rust
to_tensor_numeric()
```

Recommendation:

```text
Use `try_numeric().to_tensor()` as the canonical flow.
```

---

## 6. Error Type

```rust
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenDataError {
    Csv { message: String },
    Io { path: std::path::PathBuf, source: std::io::Error },
    EmptyInput,
    MissingColumn { name: String },
    DuplicateColumn { name: String },
    DuplicateSelection { name: String },
    RaggedRow { row: usize, expected: usize, actual: usize },
    NonNumericValue { column: String, row: usize, value: String },
    MissingValue { column: String, row: usize },
    EmptySelection,
    Matten(matten::MattenError),
}
```

Requirements:

- implement `Display`;
- implement `std::error::Error`;
- preserve `source()` for I/O and wrapped `matten` errors;
- do not extend core `MattenError`.

---

## 7. Shape Contract

`NumericTable::to_tensor()` produces:

```text
shape = [row_count, column_count]
```

where:

```text
row_count = number of data rows after selection/filtering
column_count = number of selected numeric columns
```

Since v0.20 has no filtering API, row count usually equals input row count.

Order:

```text
tensor row i = table row i
tensor column j = selected column j
```

---

## 8. Feature Flags

Initial recommendation:

```toml
[features]
default = ["csv"]
csv = ["dep:csv"]
```

Do not add:

```text
polars
arrow
datafusion
sql
async
```

A future `serde` feature may be added if `Table` serialization is needed.

---

## 9. Documentation Rules

README must show:

1. Load CSV.
2. Print schema summary.
3. Select columns.
4. Fill missing values.
5. Convert explicitly.
6. Produce Tensor.

README must not show:

- group-by;
- join;
- pivot;
- rolling;
- query DSL;
- large-data streaming.

---

## 10. Acceptance Criteria

```text
[ ] Table model is owned and lifetime-free for normal users
[ ] API fits in a short README
[ ] conversion to Tensor is explicit
[ ] missing-value handling is explicit
[ ] errors are crate-local
[ ] core matten does not depend on matten-data
[ ] no dataframe-like APIs exist
[ ] docs say experimental
```

---

## 11. Open Questions

1. Should `CellValue` be public in v0.20?
2. Should duplicate column names be rejected or disambiguated?
3. Should duplicate selected columns be rejected or allowed?
4. Should schema summary be a struct with public fields or a display-first type?
5. Should `from_records` exist in v0.20, or only CSV constructors?

Recommendation: answer these in implementation PR review before beta.
