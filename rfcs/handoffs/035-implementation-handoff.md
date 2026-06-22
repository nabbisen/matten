# RFC-035 Developer Handoff: CSV Ingestion, Schema Summary, Missing Values, and Numeric Conversion

**Project:** `matten`  
**RFC:** RFC-035  
**Handoff Kind:** Implementation Handoff  
**Implementation Level:** Concrete parser/boundary behavior required  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-035 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-035 defines the external input behavior for `matten-data`.

Core principle:

```text
CSV input is external.
External input returns Result.
No malformed input panics.
Missing values are explicit.
Numeric conversion is explicit.
```

This RFC should be implemented after RFC-034 public model is accepted.

---

## 2. Internal Design

### 2.1 Module layout

```text
csv.rs
  from_csv_str
  from_csv_path
  parse records into Table

cell.rs
  CellValue parsing helpers

schema.rs
  ColumnKind / ColumnSummary

numeric.rs
  try_numeric
  NumericTable construction
```

### 2.2 CSV policy

Initial strict policy:

```text
first row = header
headers required
empty header rejected
duplicate header rejected
rows must be rectangular
only empty cells are Missing by default
```

### 2.3 Cell parsing policy

Recommended helper:

```rust
fn parse_cell(raw: &str) -> CellValue
```

Default behavior:

```text
"" after trim -> Missing
integer-looking -> Int
float-looking -> Float
true/false -> Bool
otherwise -> Text
```

If this creates too much ambiguity, store all non-missing values as `Text` and let `try_numeric` parse. But do not silently convert bools to numbers.

### 2.4 Numeric conversion

Implement:

```rust
impl Table {
    pub fn try_numeric(&self) -> Result<NumericTable, MattenDataError>;
}
```

Rules:

- Int -> f64 allowed;
- Float -> f64 allowed;
- Bool -> error;
- Text -> either error or parse only if policy says parse text;
- Missing -> error unless filled.

Recommended v0.20 strict behavior:

```text
Text is not numeric by default.
If CSV parser stores numeric-looking values as Int/Float, conversion works.
```

---

## 3. Task Breakdown / PR Plan

### PR-035-1: CSV constructor from string

Implement:

```rust
Table::from_csv_str(input: &str)
```

Acceptance:

```text
[ ] empty input returns EmptyInput
[ ] first row parsed as header
[ ] duplicate header rejected
[ ] empty header rejected
[ ] ragged row rejected
[ ] valid CSV creates Table
```

### PR-035-2: CSV constructor from path

Implement:

```rust
Table::from_csv_path(path)
```

Acceptance:

```text
[ ] missing file returns Io with source
[ ] malformed file returns Csv/Ragged/Duplicate error
[ ] path appears in error where useful
```

### PR-035-3: Schema summary

Implement column summary:

```text
rows
columns
names
missing count
simple kind
```

Acceptance:

```text
[ ] mixed columns reported as Mixed
[ ] missing-only columns reported clearly
[ ] no expensive analytics
```

### PR-035-4: Missing value filling

Implement one of:

```rust
fill_missing(value: impl Into<CellValue>)
```

or typed helpers.

Acceptance:

```text
[ ] missing values filled
[ ] non-missing values unchanged
[ ] shape unchanged
[ ] fill policy documented
```

### PR-035-5: Numeric conversion

Implement `try_numeric`.

Acceptance:

```text
[ ] Int converts to f64
[ ] Float converts to f64
[ ] Bool rejects
[ ] Text rejects unless explicit parse policy accepted
[ ] Missing rejects unless filled
[ ] row/column context in error
```

### PR-035-6: Tensor output integration

Implement or complete:

```rust
NumericTable::to_tensor()
```

Acceptance:

```text
[ ] shape [rows, columns]
[ ] row-major order
[ ] uses Tensor::try_new
[ ] Matten errors wrapped
```

---

## 4. Acceptance / QA Checklist

### Input boundary QA

```text
[ ] no panic on malformed CSV
[ ] empty input tested
[ ] ragged rows tested
[ ] duplicate headers tested
[ ] empty headers tested
[ ] path I/O error tested
```

### Conversion QA

```text
[ ] missing values do not silently become zero
[ ] bools do not silently become 1/0
[ ] text does not silently become number unless explicitly accepted
[ ] errors include column name and row number
```

### Tensor QA

```text
[ ] output shape documented
[ ] row order preserved
[ ] column order preserved
[ ] numeric values exact where possible
```

### Security / reliability QA

```text
[ ] no unsafe
[ ] no network access
[ ] no async runtime
[ ] no heavy dataframe dependencies
[ ] no unbounded recursion
```

### CI commands

```bash
cargo fmt --all --check
cargo clippy -p matten-data --all-targets -- -D warnings
cargo test -p matten-data
cargo test -p matten-data --doc
bash scripts/check-core-dependency-boundary.sh
```

---

## 5. Edge Cases to Test

```text
CSV with only header
CSV with one data row
CSV with quoted comma
CSV with whitespace
CSV with empty cell
CSV with duplicate header
CSV with trailing empty cell
CSV with ragged row
CSV with bool column
CSV with text in numeric selection
CSV with very small numeric values
CSV with NaN/inf strings, if parser allows
```

---

## 6. Do Not Implement

- custom missing token configuration;
- column-specific fill;
- row filtering/drop rows;
- date parsing;
- locale number parsing;
- streaming;
- async;
- group-by/join/pivot.
