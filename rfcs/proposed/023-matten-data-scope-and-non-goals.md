# RFC-023: `matten-data` Scope and Non-goals

**Status:** Proposed  
**Target:** v0.16+ design, v0.17+ possible PoC  
**Theme:** Companion crate exploration  
**Depends on:** RFC-016, RFC-017, RFC-022  
**Related handoff:** `023-matten-data-scope-and-non-goals-handoff.md`

## 1. Summary

This RFC defines the tentative scope of a future `matten-data` companion crate.

`matten-data` would provide lightweight table-like ingestion and cleanup helpers that produce `matten::Tensor`. It must not become pandas or polars. Its job is to help users get from practical CSV/JSON rows to numeric tensors while keeping `matten` core clean.

## 2. Goals

- Explore schema/column utilities outside core.
- Provide table-to-tensor conversion helpers.
- Keep dataframe-like complexity out of `matten`.
- Support small PoC datasets.
- Reuse `matten::Tensor`, `Element`, and `NumericPolicy`.

## 3. Non-goals

- No full dataframe engine.
- No joins.
- No group-by.
- No pivot.
- No SQL query API.
- No lazy query optimizer.
- No large streaming guarantee in this crate unless RFC-026 delegates it here.
- No replacement for polars.

## 4. External design

Possible future API:

```rust
use matten_data::Table;

let table = Table::from_csv("sales.csv")?
    .fill_missing("sales", 0.0)
    .select_columns(["sales", "cost", "month"]);

let tensor = table.to_tensor()?;
```

This API is illustrative, not accepted implementation.

## 5. Data model

Possible model:

```rust
pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<Element>>,
}

pub struct Column {
    name: String,
    kind: ColumnKind,
}
```

But memory layout should be carefully reviewed before implementation.

Alternative columnar storage may be better:

```rust
columns: Vec<(Column, Vec<Element>)>
```

## 6. Data lifecycle

```text
CSV/JSON rows
  -> Table
  -> schema summary / cleanup
  -> selected numeric columns
  -> matten::Tensor
```

`matten-data` should end at `Tensor`.

Math remains in `matten`.

## 7. Events and observable behavior

Observable user events:

- schema inference result;
- missing fill;
- column selection;
- conversion failure;
- tensor output.

All file/parser operations return `Result`.

## 8. Store access

`matten-data` should not access `matten` internals.

It should construct tensors through:

```rust
Tensor::try_new
Tensor::try_from_elements
Tensor::try_numeric_with
```

or accepted public APIs.

## 9. Public API requirements

No `matten` core API requirement yet.

Potential future need:

- stable `Element`;
- stable `NumericPolicy`;
- stable conversion errors.

## 10. Cargo feature impact

`matten-data` should be a separate crate, not a `matten` feature.

Possible dependencies:

```toml
matten = { version = "...", features = ["dynamic", "csv", "json"] }
```

Avoid heavy dependencies until justified.

## 11. Internal design

### 11.1 Schema inference

Keep schema inference simple:

```text
all numeric -> numeric
mixed numeric + none -> numeric nullable
text present -> text/mixed
bool present -> bool/mixed
```

Do not infer dates, currencies, or locale numbers in the first PoC.

### 11.2 Column names

Column names belong in `matten-data`, not `matten`.

### 11.3 Conversion

`Table::to_tensor()` should require explicit column selection and conversion policy.

## 12. Examples

Examples should live in `matten-data`, not core `matten`, once the crate exists.

Potential examples:

```text
examples/sales_table_to_tensor.rs
examples/messy_csv_select_numeric.rs
examples/schema_summary.rs
```

Core `matten` may link to them after release.

## 13. Acceptance criteria for a future PoC

- Uses `matten` public API only.
- Does not add dependencies to core.
- Clearly says it is not a dataframe engine.
- Provides table-to-tensor path.
- Handles small CSV fixtures.
- Avoids joins/group-by/pivot.

## 14. QA checklist

- [ ] API does not leak into core
- [ ] No heavy dependencies without review
- [ ] Small fixtures only
- [ ] Conversion policy explicit
- [ ] Documentation states non-goals

## 15. Open questions

1. Should `matten-data` use row-major or columnar internal storage?
2. Should it live in the same workspace?
3. Should it depend on `csv` directly or reuse `matten` dynamic CSV ingestion?
