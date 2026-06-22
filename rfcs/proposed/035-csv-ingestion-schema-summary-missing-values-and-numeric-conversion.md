# RFC-035: CSV Ingestion, Schema Summary, Missing Values, and Numeric Conversion

**Status:** Proposed  
**Target Release:** v0.20.x if RFC-033 approves implementation  
**Related:** RFC-033, RFC-034, RFC-036, RFC-042  
**Scope:** Input boundary behavior for `matten-data`

---

## 1. Summary

This RFC defines the CSV/table ingestion contract for `matten-data`.

The core policy:

```text
CSV input is external input.
External input returns Result.
Malformed data never panics.
Missing values never silently become zero.
Text-to-number conversion is explicit.
```

---

## 2. Motivation

CSV data is common in business PoC workflows, but it is messy:

- missing cells;
- ragged rows;
- duplicate headers;
- non-numeric values;
- whitespace;
- quoted values;
- locale-like number strings;
- empty files.

If `matten-data` accepts CSV, it must handle these cases predictably without expanding into a dataframe library.

---

## 3. CSV Input Model

### 3.1 Constructors

```rust
Table::from_csv_str(input: &str) -> Result<Table, MattenDataError>
Table::from_csv_path(path: impl AsRef<Path>) -> Result<Table, MattenDataError>
```

### 3.2 Header Policy

v0.20 recommendation:

```text
First row is header.
Headers are required.
Duplicate headers are rejected.
Empty header names are rejected.
```

Reason:

- named-column selection is the main value of `matten-data`;
- generated headers add policy complexity;
- duplicate headers create ambiguous selection.

Future RFC may add headerless CSV support.

### 3.3 Row Policy

Rows must be rectangular.

If header has `n` columns, every data row must have `n` cells.

Ragged rows return:

```rust
MattenDataError::RaggedRow {
    row,
    expected,
    actual,
}
```

Row numbering should be documented as either zero-based or one-based.

Recommendation:

```text
Use one-based CSV line numbers in error messages.
Use zero-based internal indices only internally.
```

---

## 4. Cell Parsing

### 4.1 Initial Cell Kind Inference

v0.20 may use simple inference:

```text
empty / configured missing token -> Missing
integer-looking value -> Int
float-looking value -> Float
"true"/"false" -> Bool
otherwise -> Text
```

But numeric conversion must not rely on implicit inference alone.

`try_numeric()` must validate all selected cells.

### 4.2 Whitespace Policy

Recommendation:

```text
Trim unquoted surrounding whitespace for simple values.
Preserve quoted interior text according to csv crate behavior.
```

Document this explicitly.

### 4.3 Missing Values

Default missing tokens:

```text
empty cell
```

Optional future missing tokens:

```text
NA
N/A
null
NULL
nan
NaN
```

Recommendation for v0.20:

```text
Only empty cells are missing by default.
Additional missing tokens require explicit configuration in a future RFC.
```

This avoids surprising conversion of legitimate text.

---

## 5. Schema Summary

`schema_summary()` should help users decide what to select and convert.

Minimum summary content:

```text
row count
column count
column names
per-column missing count
per-column simple inferred kind
```

Possible kind values:

```rust
pub enum ColumnKind {
    Integer,
    Float,
    Boolean,
    Text,
    Mixed,
    MissingOnly,
}
```

`ColumnKind` may be public or internal.

`schema_summary()` must not perform expensive dataframe analysis.

---

## 6. Missing-Value Filling

### 6.1 Explicit Fill

Missing values may be filled by:

```rust
table.fill_missing(0.0)?
```

or typed alternatives.

Rules:

- fill must be explicit;
- fill applies to all missing values in the selected/current table;
- future column-specific fill may be added later;
- missing values must not silently become zero.

### 6.2 Column-Specific Fill

Deferred.

Potential future API:

```rust
table.fill_missing_in("sales", 0.0)?
```

Do not implement in v0.20 unless required by early usage.

---

## 7. Numeric Conversion

### 7.1 Explicit Conversion

Numeric conversion must be explicit:

```rust
let numeric = table.try_numeric()?;
let tensor = numeric.to_tensor()?;
```

Do not provide silent conversion in `select_columns()`.

### 7.2 Allowed Numeric Values

**Locked v0.20 model (architect ruling, RFC-033–042 review Q5): inference + strict.**

CSV cell parsing creates an inferred `CellValue` (see §4.1). `try_numeric()` then
applies strict rules:

```text
Int     -> f64
Float   -> f64
Bool    -> Err   (booleans are not silently 1/0)
Text    -> Err   (non-numeric text is rejected)
Missing -> Err   unless filled first
```

Text is **not** parsed as numeric by default. This keeps the first implementation
easy to explain and aligned with the `matten` philosophy of explicit conversion
and no surprising coercion: numeric-looking values are accepted, non-numeric text
is rejected, booleans are not coerced, and missing values are not silently zero.

Explicit text parsing is reserved for a possible future method
(`try_numeric_parse_text()`) or a future numeric-policy object; it MUST NOT be
added in v0.20 unless absolutely necessary.

### 7.3 Bool Conversion

Bool to numeric is forbidden by default.

Do not convert:

```text
true -> 1
false -> 0
```

unless a future RFC adds explicit policy.

### 7.4 Missing During Numeric Conversion

If missing values remain during numeric conversion, return error:

```rust
MattenDataError::MissingValue {
    column,
    row,
}
```

Users must fill or drop missing values explicitly. Since v0.20 has no drop-row API, fill is the primary path.

---

## 8. Tensor Output

`NumericTable::to_tensor()` returns numeric tensor:

```text
shape = [rows, columns]
data = row-major f64 values
```

Example:

```text
headers: sales,cost
rows:
  10,2
  20,3

Tensor shape: [2, 2]
Tensor data: [10.0, 2.0, 20.0, 3.0]
```

---

## 9. Error Quality

Errors should answer:

1. what failed;
2. where it failed;
3. what value was seen;
4. what the user can do.

Example:

```text
matten-data numeric conversion error: column "sales", row 4 contains "unknown",
which cannot be converted to f64. Fill or clean the column before calling try_numeric().
```

---

## 10. Security / Reliability

Required:

- no panic on malformed CSV;
- no unbounded recursion;
- no hidden file access except explicit path constructor;
- no network access;
- no unsafe;
- shape/row/column counts checked before Tensor construction;
- dependency-boundary script remains green for core.

Input size policy:

v0.20 may document that it is for small inputs and does not enforce a strict byte limit yet. If limits are added, they must be explicit.

---

## 11. Acceptance Criteria

```text
[ ] header policy documented
[ ] duplicate header behavior documented and tested
[ ] ragged row behavior documented and tested
[ ] missing-value policy documented and tested
[ ] numeric conversion is explicit
[ ] bool-to-number is not automatic
[ ] output Tensor shape is documented
[ ] row/column order is tested
[ ] errors include row/column context where practical
[ ] no dataframe operations are introduced
```

---

## 12. Non-goals

- No schema inference framework.
- No date parsing.
- No categorical dtype.
- No locale-aware number parsing.
- No thousands separators.
- No streaming.
- No async.
- No row filtering.
- No group-by or joins.
