# RFC-036: `matten-data` Examples, Documentation, and Release Gate

**Status:** Implemented (v0.22.0)  
**Target Release:** v0.22.0  
**Implemented:** The six-example suite (`data_00`–`data_05`) plus an explicit malformed-CSV test landed in v0.22.0, completing the RFC-023 §9 Beta gate and promoting `matten-data` to Beta (architect ruling, 2026-06-24).  
**Related:** RFC-033, RFC-034, RFC-035, RFC-037, RFC-042  
**Scope:** Example suite, documentation, CI, and release quality gates for `matten-data`

---

## 1. Summary

This RFC defines the example and documentation gate for `matten-data`.

The main rule:

```text
Examples demonstrate accepted APIs.
Examples must not create hidden dataframe scope.
```

`matten-data` examples should teach the table-to-Tensor workflow and nothing more.

---

## 2. Motivation

`matten-data` has high scope-creep risk. Examples are one of the easiest ways for scope to expand accidentally.

For example, adding examples such as:

```text
sales_by_region_groupby.rs
join_customer_orders.rs
pivot_monthly_sales.rs
rolling_average.rs
```

would imply dataframe expectations even if the public API does not yet include them.

This RFC prevents examples from becoming product promises.

---

## 3. Required Example Suite

If `matten-data` is implemented, the minimum example suite is:

```text
examples/data_00_quickstart.rs
examples/data_01_schema_summary.rs
examples/data_02_select_columns.rs
examples/data_03_missing_values.rs
examples/data_04_to_tensor.rs
examples/data_05_errors.rs
```

### 3.1 `data_00_quickstart`

Must show the full happy path:

```text
CSV string
  -> Table
  -> select columns
  -> fill missing
  -> try_numeric
  -> Tensor
```

### 3.2 `data_01_schema_summary`

Must show:

- row count;
- column count;
- column names;
- missing count;
- simple inferred column kinds if implemented.

### 3.3 `data_02_select_columns`

Must show:

- select by name;
- output column order matches request;
- missing column error, if concise.

### 3.4 `data_03_missing_values`

Must show:

- missing values do not silently become zero;
- explicit fill;
- conversion succeeds after fill.

### 3.5 `data_04_to_tensor`

Must show:

- output shape `[rows, columns]`;
- row-major data order;
- use with core `matten`.

### 3.6 `data_05_errors`

Must show one or more boundary errors:

- duplicate header;
- ragged row;
- non-numeric value;
- missing value during conversion.

---

## 4. Forbidden Examples

Do not add examples implying:

- group-by;
- join;
- merge;
- pivot;
- rolling/window calculations;
- query expressions;
- SQL-like filters;
- dataframe indexing;
- datetime processing;
- large CSV streaming;
- ML training or feature engineering;
- Polars/DataFusion replacement.

These terms may appear in docs only under **Non-goals** or **When to use another crate**.

---

## 5. README Structure

The `matten-data` README should contain:

1. Status label: Experimental.
2. One-sentence purpose.
3. Quickstart.
4. Scope.
5. Non-goals.
6. API overview.
7. Error behavior.
8. Relationship to core `matten::Tensor`.
9. Relationship to core `dynamic`.
10. When to use Polars/DataFusion/Pandas instead.

Recommended first paragraph:

> `matten-data` is an experimental companion crate for turning small table-like inputs into `matten::Tensor`. It is not a dataframe library or query engine.

---

## 6. mdBook / Project Docs

Project docs should include a short page:

```text
docs/src/companions/matten-data.md
```

Required sections:

- purpose;
- install;
- quickstart;
- output Tensor shape;
- missing-value policy;
- limitations;
- status/maturity.

Canonical install style:

```toml
[dependencies]
matten = "0.20"
matten-data = "0.20"
```

Canonical import style:

```rust
use matten::Tensor;
use matten_data::Table;
```

Do not teach `use matten_data::Tensor`.

---

## 7. CI Requirements

CI must include:

```bash
cargo check -p matten-data --examples
cargo test -p matten-data
cargo test -p matten-data --doc
```

Workspace CI must still include:

```bash
bash scripts/check-core-dependency-boundary.sh
```

---

## 8. Release-Docs Checks

The release-doc script SHOULD check:

```text
[ ] matten-data README says Experimental
[ ] examples do not contain forbidden dataframe terms except in non-goals docs
[ ] examples use canonical imports
[ ] docs mention output Tensor shape
[ ] docs mention explicit missing-value handling
[ ] docs mention explicit numeric conversion
[ ] docs mention when to use Polars/DataFusion instead
```

Forbidden terms should not be naively banned everywhere because they are needed in non-goals. The script may check examples more strictly and docs more selectively.

---

## 9. Documentation Tone

Use clear language:

Good:

```text
Select numeric columns and convert them to Tensor.
```

Avoid:

```text
Analyze your dataframe.
Run table queries.
Transform big data.
```

Good:

```text
For joins, group-by, lazy queries, or large data, use Polars or DataFusion.
```

Avoid:

```text
matten-data is like Pandas for Rust.
```

---

## 10. Acceptance Criteria

```text
[ ] required examples exist
[ ] examples compile and run in CI
[ ] README status is Experimental
[ ] README explicitly says not a dataframe
[ ] docs explain Tensor output shape
[ ] docs explain missing-value and numeric conversion policies
[ ] release-doc check guards examples from scope creep
[ ] no examples add hidden product promises
```

---

## 11. Non-goals

- No benchmark examples.
- No large-data examples.
- No ML examples.
- No dataframe operation examples.
- No database examples.
- No web framework examples in v0.20.
