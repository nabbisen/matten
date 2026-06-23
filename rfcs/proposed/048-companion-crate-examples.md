# RFC-048: Companion-Crate Examples

**Status:** Proposed  
**Target Release:** v0.20.x / v0.21.0  
**Related:** RFC-022, RFC-030, RFC-033, RFC-036, RFC-043  
**Scope:** Examples for `matten-ndarray`, `matten-mlprep`, and `matten-data` (all shipped; audit existing examples)

---

## 1. Summary

This RFC defines examples for companion crates.

All three companion crates already ship examples (architect ruling, RFC-043–048
review Q2/Q3). This RFC therefore **audits and improves** the existing examples
rather than adding duplicate or renamed files:

```text
matten-ndarray   from_arrayd.rs, to_arrayd.rs        (existing)
matten-mlprep    standardize_columns.rs, train_test_split.rs,
                 add_bias_column.rs, minmax_scale.rs  (existing)
matten-data      csv_to_tensor.rs                     (shipped in v0.20.1)
```

`matten-data` is no longer future-only: it shipped its table API and the
`csv_to_tensor.rs` example in v0.20.1. Do not add a `01_csv_to_tensor.rs`.

Companion examples must preserve the dependency direction:

```text
companions depend on matten
matten does not depend on companions
```

---

## 2. Motivation

Companion crates need examples that show why they exist.

Core examples demonstrate math. Companion examples demonstrate workflow boundaries:

```text
matten-ndarray:
  move data to/from ndarray ecosystem

matten-mlprep:
  prepare numeric Tensor data for small ML-like workflows

matten-data:
  convert small table-like input to Tensor
```

---

## 3. `matten-ndarray`: Roundtrip (audit existing `from_arrayd`/`to_arrayd`)

The roundtrip is already demonstrated by the existing `from_arrayd.rs` and
`to_arrayd.rs` examples. Audit them against the teaching points below and improve
in place (or consolidate into a single roundtrip example only if that reads better);
do not add a duplicate `01_ndarray_roundtrip.rs`.

### Problem

Convert `matten::Tensor` to `ndarray::ArrayD<f64>` and back.

### APIs demonstrated

```rust
matten_ndarray::to_arrayd
matten_ndarray::from_arrayd
```

### Required teaching points

- conversion copies data;
- shape is preserved;
- dynamic tensors are rejected unless converted to numeric first;
- core `matten` does not depend on `ndarray`.

### Acceptance

```text
[ ] example compiles in matten-ndarray crate
[ ] explicit dependency/import style used
[ ] shape printed before/after
[ ] no zero-copy claim
```

---

## 4. `matten-mlprep`: Standardize and Train/Test Split (audit existing examples)

`matten-mlprep` already ships `standardize_columns.rs`, `train_test_split.rs`,
`add_bias_column.rs`, and `minmax_scale.rs`. Audit these against the teaching points
below and improve in place; do not add a duplicate `01_standardize_train_test.rs`.

### Problem

Prepare a small feature matrix.

### APIs demonstrated

```rust
standardize_columns
minmax_scale_columns, optional
add_bias_column
train_test_split
```

### Required teaching points

- rows = samples;
- columns = features;
- split is deterministic;
- zero-variance behavior is explicit;
- no hidden randomness.

### Acceptance

```text
[ ] example compiles in matten-mlprep crate
[ ] deterministic output
[ ] row/sample convention documented
[ ] no model training
```

---

## 5. `matten-data`: CSV to Tensor (shipped v0.20.1 — audit existing `csv_to_tensor`)

### Status

Shipped. `matten-data` shipped its table API and `examples/csv_to_tensor.rs` in
**v0.20.1**. This is no longer future work. Audit and improve the existing
`csv_to_tensor.rs` against the teaching points below; do not add a duplicate or
renamed `01_csv_to_tensor.rs`.

### Problem

Load a small CSV, select numeric columns, fill missing values, convert to Tensor.

### APIs demonstrated

```rust
Table::from_csv_str or Table::from_csv_path
schema_summary
select_columns
fill_missing
try_numeric
to_tensor
```

### Required teaching points

- `matten-data` is experimental;
- not a dataframe;
- missing values explicit;
- numeric conversion explicit;
- output shape `[rows, columns]`.

### Acceptance

```text
[ ] no group-by/join/pivot/query
[ ] no large CSV claim
[ ] output Tensor shape printed
[ ] canonical dependency/import style used
```

---

## 6. Dependency / Import Style

Canonical examples should use explicit dependencies and imports.

Example:

```rust
use matten::Tensor;
use matten_ndarray::to_arrayd;
```

Do not teach:

```rust
use matten_ndarray::Tensor;
```

Do not require or add:

```rust
pub use matten;
```

because the convenience re-export is deferred by RFC-032.

---

## 7. Documentation Requirements

Add docs page:

```text
docs/src/examples/companions.md
```

Include:

- companion purpose;
- maturity label;
- example command;
- what the example proves;
- what the companion does not do.

---

## 8. QA Checklist

```text
[ ] examples live in companion crate directories
[ ] examples compile in CI
[ ] examples use canonical imports
[ ] no core crate dependency on companion
[ ] no hidden maturity overclaim
[ ] no unsupported scope
```

CI (use the real existing example names):

```bash
cargo check -p matten-ndarray --examples --all-features
cargo check -p matten-mlprep --examples --all-features
cargo check -p matten-data --examples --all-features
cargo run -p matten-ndarray --example from_arrayd
cargo run -p matten-ndarray --example to_arrayd
cargo run -p matten-mlprep --example standardize_columns
cargo run -p matten-mlprep --example train_test_split
cargo run -p matten-data --example csv_to_tensor
bash scripts/check-core-dependency-boundary.sh
```

---

## 9. Non-goals

- No companion facade over core.
- No `pub use matten`.
- No ML model training.
- No ndarray wrapper library.
- No dataframe examples.
- No large-data examples.
