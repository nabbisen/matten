# RFC-034 Developer Handoff: `matten-data` Table Model and Public API Boundary

**Project:** `matten`  
**RFC:** RFC-034  
**Handoff Kind:** Implementation Handoff  
**Implementation Level:** Concrete API and internal structure required  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-034 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-034 defines the minimal public API for `matten-data`.

Target public API:

```rust
pub struct Table;
pub struct SchemaSummary;
pub struct NumericTable;

#[non_exhaustive]
pub enum MattenDataError { ... }
```

Canonical flow:

```rust
let table = Table::from_csv_str(csv)?;

let x = table
    .select_columns(["sales", "cost", "quantity"])?
    .fill_missing(0.0)?
    .try_numeric()?
    .to_tensor()?;
```

This RFC should be implemented after RFC-033 is accepted.

---

## 2. Internal Design

### 2.1 Module layout

Recommended:

```text
crates/matten-data/src/
  lib.rs
  error.rs
  table.rs
  schema.rs
  cell.rs
  numeric.rs
```

### 2.2 Internal data model

Recommended internal representation:

```rust
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<CellValue>>,
}

pub enum CellValue {
    Text(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Missing,
}

pub struct NumericTable {
    headers: Vec<String>,
    rows: Vec<Vec<f64>>,
}
```

Fields should remain private.

### 2.3 Public `CellValue` decision

Default recommendation:

```text
Keep `CellValue` public only if needed by examples.
If public, mark #[non_exhaustive].
If private, provide typed fill helpers.
```

For v0.20, public `CellValue` is acceptable if it simplifies `fill_missing`.

### 2.4 Schema summary

Recommended display-first model:

```rust
pub struct SchemaSummary {
    rows: usize,
    columns: usize,
    columns_info: Vec<ColumnSummary>,
}
```

Avoid making this a rich dataframe-style schema system.

### 2.5 Error model

Implement `MattenDataError` in `error.rs`.

Requirements:

- `Debug`;
- `Display`;
- `std::error::Error`;
- `source()` for I/O and wrapped `matten::MattenError`.

Do not add variants to core `MattenError`.

---

## 3. Task Breakdown / PR Plan

### PR-034-1: Public type skeleton

- Add `Table`, `SchemaSummary`, `NumericTable`.
- Add crate docs and status.
- Add compile-only public API tests.

Acceptance:

```text
[ ] public types exist
[ ] fields private
[ ] docs say Experimental
[ ] no dataframe terms in type names
```

### PR-034-2: Error type

- Implement `MattenDataError`.
- Add Display messages.
- Add Error source chaining.
- Add tests for formatting.

Acceptance:

```text
[ ] error type is crate-local
[ ] wraps matten::MattenError without modifying core
[ ] I/O source preserved
```

### PR-034-3: Table inspection methods

Implement:

```rust
row_count()
column_count()
column_names()
schema_summary()
```

Acceptance:

```text
[ ] methods do not allocate unnecessarily except schema_summary
[ ] row/column counts correct
[ ] column order preserved
```

### PR-034-4: Selection API

Implement:

```rust
select_columns(...)
```

Rules:

- missing column -> error;
- duplicate requested column -> reject initially;
- selected column order follows request;
- returns owned `Table`.

Acceptance:

```text
[ ] missing column tested
[ ] duplicate selection tested
[ ] selected order tested
[ ] no lifetime-bearing view type exposed
```

### PR-034-5: NumericTable and `to_tensor`

- Implement `NumericTable`.
- Implement `NumericTable::to_tensor()`.
- Use `matten::Tensor::try_new`.

Acceptance:

```text
[ ] output shape [rows, columns]
[ ] row-major order tested
[ ] MattenError wrapped as MattenDataError::Matten
```

---

## 4. Acceptance / QA Checklist

### API checklist

```text
[ ] `Table` exists and is owned
[ ] `SchemaSummary` exists and is displayable/debuggable
[ ] `NumericTable` exists or a justified alternative is documented
[ ] `MattenDataError` is crate-local
[ ] no broad dataframe API exists
```

### Behavior checklist

```text
[ ] row order preserved
[ ] column order preserved
[ ] selected column order follows user request
[ ] duplicate column/selection policy tested
[ ] conversion to Tensor explicit
```

### CI checklist

```bash
cargo fmt --all --check
cargo clippy -p matten-data --all-targets -- -D warnings
cargo test -p matten-data
cargo test -p matten-data --doc
bash scripts/check-core-dependency-boundary.sh
```

### Documentation checklist

```text
[ ] README shows canonical explicit dependency style
[ ] README imports Tensor from `matten`, not `matten_data`
[ ] README says not a dataframe
[ ] rustdoc examples are small
```

---

## 5. Implementation Risks

### Risk: `Table` becomes a dataframe

Mitigation:

- no row filtering;
- no group-by;
- no joins;
- no query API;
- keep methods preparation-focused.

### Risk: schema summary grows too much

Mitigation:

- summary only;
- no type system;
- no inference framework.

### Risk: too much generic API design

Mitigation:

- prefer concrete owned types;
- avoid trait-heavy abstractions.
