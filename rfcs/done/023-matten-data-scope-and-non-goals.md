# RFC-023: `matten-data` Scope and Non-goals

**Status:** Resolved (v0.22.0)  
**Target:** experimental only before v0.20; beta decision no earlier than v0.20+  
**Resolution:** Outcome B (kept Experimental through the v0.21 family). The §9 Beta gate was met in v0.22.0 via the full RFC-036 example suite and an explicit malformed-CSV test, and `matten-data` was promoted to Beta (architect ruling, 2026-06-24).  
**Theme:** Table-to-Tensor companion crate, strict scope control  
**Depends on:** RFC-016, RFC-017, RFC-022  
**Supersedes:** older target language that allowed `matten-data` PoC as the main v0.17 companion milestone

---

## 1. Summary

This RFC defines the scope of a possible future `matten-data` companion crate.

`matten-data` may help users prepare small CSV/table-like data for `matten::Tensor`, but it must not become a dataframe engine. It is the most business-useful companion idea and also the highest scope-creep risk. Therefore, it is intentionally delayed behind the `matten-ndarray` and `matten-mlprep` tracks.

The key lifecycle is:

```text
CSV / table-like data
  -> schema summary
  -> missing-value cleanup
  -> column selection
  -> explicit numeric conversion
  -> matten::Tensor
```

---

## 2. Goals

- Help users reach `Tensor` from practical CSV/table data.
- Keep table semantics outside core `matten`.
- Provide small schema/column/missing-value helpers.
- Support small-to-medium PoC workflows.
- Preserve explicit numeric conversion.
- Avoid becoming pandas, Polars, or DataFusion.

---

## 3. Non-goals

`matten-data` must not implement, before a new RFC explicitly reopens scope:

- joins;
- group-by;
- pivot;
- SQL-like query API;
- lazy execution;
- expression optimizer;
- window functions;
- large-data streaming;
- dataframe-style indexing;
- ML preprocessing.

---

## 4. Revised target and maturity policy

`matten-data` may be scaffolded before v0.20 for planning or API experiments, but it must not be promoted before v0.20.

v0.20+ decision outcomes:

```text
A) promote to beta
B) keep experimental
C) freeze/defer
```

Beta is allowed only if the small table-to-Tensor workflow is useful, teachable, and not dataframe-like.

---

## 5. External design

Possible beta API:

```rust
use matten_data::Table;

let table = Table::from_csv_path("sales.csv")?;
println!("{}", table.schema_summary());

let x = table
    .select_columns(["sales", "cost", "quantity"])?
    .fill_missing(0.0)?
    .to_tensor()?;
```

This API is illustrative until implementation RFC acceptance.

---

## 6. Data model

A future `Table` may store data row-wise or column-wise. Columnar storage is likely better if selection and conversion are primary operations, but the first implementation must not expose storage layout publicly.

Conceptual model:

```rust
pub struct Table {
    columns: Vec<Column>,
    // storage representation TBD
}

pub struct Column {
    name: String,
    kind: ColumnKind,
}
```

---

## 7. Conversion policy

`matten-data` should reuse or mirror core dynamic conversion policy:

- numeric conversion is explicit;
- non-convertible values return an error;
- missing-value handling is explicit;
- column selection happens before tensor conversion.

Column-specific policy belongs in `matten-data`, not core `matten`.

---

## 8. Error policy

`matten-data` defines its own error type.

Example:

```rust
pub enum MattenDataError {
    Csv(csv::Error),
    MissingColumn(String),
    DuplicateColumn(String),
    RaggedRow { row: usize },
    NonNumericColumn { column: String },
    Matten(matten::MattenError),
}
```

Core `MattenError` must not grow table-specific variants.

---

## 9. Beta gate

`matten-data` deserves beta only if:

- README + three examples teach the full useful workflow;
- malformed CSV, duplicate headers, missing values, mixed columns, row length mismatch, and non-convertible values are tested;
- the API remains small;
- no dataframe-like features enter the crate;
- core `matten` has no dependency on `matten-data`.

---

## 10. Examples

Examples belong in `matten-data`, not core `matten`.

Possible future examples:

```text
examples/sales_table_to_tensor.rs
examples/messy_csv_select_numeric.rs
examples/schema_summary.rs
```

Core `matten` may link to released companion examples but must not include broken or feature-gated pseudo-examples.
