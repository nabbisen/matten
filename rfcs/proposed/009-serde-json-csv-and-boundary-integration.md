# RFC-009: Serde, JSON, CSV, and Boundary Integration

> RFC status: Proposed (reconciled v2 — 2026-06-20)  
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines the boundary integration design for serde, JSON, CSV, and file loading. All external input APIs must return `Result<Tensor, MattenError>`. Phase 1 supports numeric data only. Phase 2 dynamic support for mixed JSON/CSV is governed by RFC-011 but must reuse the same boundary-safety principles.

## 2. Motivation

`matten` is intended for Rust web APIs and data-processing PoCs. Application developers need predictable serialization and graceful parsing errors. The crate must avoid crashing services when user input is malformed, ragged, too deeply nested, or too large.

## 3. Goals

- Define canonical serde representation for Phase 1.
- Define JSON parsing from numeric nested arrays and object form.
- Define CSV numeric parsing policy.
- Define file loading APIs.
- Require `Result` at every external boundary.
- Keep CSV/JSON scope from becoming a dataframe engine.

## 4. Non-goals

- No schema inference framework.
- No SQL connector.
- No streaming massive-file engine in `0.1.0`.
- No automatic mixed-type CSV in Phase 1.
- No date/time parsing.
- No dataframe operations.

## 5. Cargo Features

Recommended feature split:

| Feature | Default | Purpose |
|---|---:|---|
| `serde` | yes (default) | Enable `Serialize`/`Deserialize` for `Tensor`. |
| `json` | yes (default, implies `serde`) | Enable `from_json` / `load_json` using `serde_json`. |
| `csv` | yes (default) | Enable `from_csv` / `load_csv` using the `csv` crate. |
| `dynamic` | no | Enable mixed-value JSON/CSV behavior. |

The locked matrix is `default = ["serde", "json", "csv"]` with `json = ["serde", "dep:serde_json"]` and `csv = ["dep:csv"]`. Lean builds use `default-features = false`. `json` and `csv` are kept as separate features because each creates public APIs and CI gates.

## 6. Data Model

### 6.1 Canonical object form

Recommended canonical serde form:

```json
{
  "shape": [2, 2],
  "data": [1.0, 2.0, 3.0, 4.0]
}
```

Rationale:

- unambiguous for any rank;
- avoids deeply nested JSON for high rank;
- preserves row-major storage explicitly.

### 6.2 Convenience nested form

`from_json` should also accept nested arrays for common rank 1 and 2 workflows:

```json
[[1.0, 2.0], [3.0, 4.0]]
```

Nested form must be rectangular. Ragged arrays are rejected.

### 6.3 CSV form

CSV produces a rank-2 tensor with shape `[rows, columns]`.

Phase 1 CSV accepts numeric fields only. Empty fields are errors in Phase 1 unless a later option defines missing numeric behavior.

## 7. Data Lifecycle

### 7.1 Serialization

1. Tensor is borrowed.
2. Shape and flat data are serialized.
3. No shape mutation or computation occurs.

### 7.2 JSON parsing

1. Input string is parsed as JSON.
2. Representation is detected: object or nested array.
3. Shape and data are validated.
4. Allocation budget is checked.
5. Tensor is constructed with `try_new`.

### 7.3 CSV parsing

1. CSV input is parsed row by row.
2. Column count is recorded from first row.
3. Each subsequent row must match column count.
4. Each field must parse as `f64` in Phase 1.
5. Tensor is constructed with shape `[rows, cols]`.

### 7.4 File loading

1. Path is read.
2. I/O errors map to `MattenError::Io`.
3. Content parser returns `Result`.

## 8. Events

| Event | Required behavior |
|---|---|
| deserialize begins | enforce boundary limits |
| representation detected | object/nested array/csv |
| raggedness detected | return `Err` |
| numeric parse fails | return row/column context when practical |
| file read fails | return I/O error category |
| output serialize begins | should not panic for valid tensor |

No public event bus.

## 9. Store Access

File APIs are read-only convenience APIs.

```rust
pub fn load_json(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError>;
pub fn load_csv(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError>;
```

No write-to-file API is required in `0.1.0`; serde users can write serialized output themselves.

## 10. Public API

```rust
#[cfg(feature = "serde")]
impl serde::Serialize for Tensor;

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Tensor;

impl Tensor {
    #[cfg(feature = "json")]
    pub fn from_json(input: &str) -> Result<Tensor, MattenError>;

    #[cfg(feature = "csv")]
    pub fn from_csv(input: &str) -> Result<Tensor, MattenError>;

    #[cfg(feature = "json")]
    pub fn load_json(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError>;

    #[cfg(feature = "csv")]
    pub fn load_csv(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError>;
}
```

`json` and `csv` are separate features. These methods are cfg-gated by those features and are enabled by default through the convenient PoC profile.

## 11. Internal Design

### 11.1 Serde helper struct

```rust
#[derive(Serialize, Deserialize)]
struct TensorSerde {
    shape: Vec<usize>,
    data: Vec<f64>,
}
```

Deserialize should call `Tensor::try_new` after parsing, not construct fields directly without validation.

### 11.2 Nested array inference

Rank inference for JSON nested arrays must reject ragged arrays. In `0.1.0`, support rank 1 and rank 2 first. Higher-rank nested support may be added if implementation stays small.

### 11.3 CSV row/column context

CSV parse errors are reported as `MattenError::Parse { format: DataFormat::Csv, message }`; the row/column detail lives in the message string. They should include row index and column index when practical:

```text
matten csv parse error at row 3, column 2: expected f64, got "active"
```

## 12. Error Handling

All APIs in this RFC are Result-zone.

They must not panic for:

- malformed JSON;
- non-numeric JSON values in Phase 1;
- ragged nested arrays;
- CSV row length mismatch;
- non-numeric CSV fields;
- missing file;
- unreadable file;
- allocation overflow.

## 13. Testing

- serde round-trip canonical object form;
- nested JSON rank 1 and 2;
- ragged JSON rejection;
- non-numeric JSON rejection in Phase 1;
- CSV numeric parsing;
- CSV ragged rows;
- CSV non-numeric field;
- file load missing path;
- boundary allocation limit test;
- dynamic feature JSON tests after RFC-011.

## 14. Acceptance Criteria

- `Tensor` can serialize and deserialize via serde.
- JSON/CSV boundary APIs return `Result`.
- Phase 1 rejects mixed values cleanly.
- CSV output shape is `[rows, cols]`.
- Parser errors are actionable.
- Default dependency decision is documented before implementation.
