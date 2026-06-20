# RFC-005: Error Model, Panic Messages, and Boundary APIs

> RFC status: Implemented (0.1.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines `MattenError`, the panic-message standard, and the public boundary API pattern. It refines RFC-001 by specifying error categories and recommending named `try_*` APIs for recoverable user-driven operations.

## 2. Motivation

`matten` intentionally uses panics for trusted local computations because that supports fast PoC feedback. But the crate is also intended for web/API and file-processing boundaries. A consistent error model is required so application developers can catch malformed input without wrapping every tensor operation in `catch_unwind`.

## 3. Goals

- Define error categories.
- Define display/debug expectations.
- Define panic-message format.
- Define which APIs must return `Result`.
- Provide `try_*` APIs where panicking alternatives are likely to be used with user input.
- Keep errors lightweight and dependency-minimal.

## 4. Non-goals

- No typed error hierarchy with many public structs in `0.1.0`.
- No dependency on `anyhow` in the library API.
- No requirement that every internal operation return `Result`.
- No promise that panic-zone APIs are safe for untrusted input.

## 5. Cargo Features

| Feature | Error impact |
|---|---|
| default | `MattenError` with shape, allocation, slice, and feature categories. |
| `serde` | Adds `Serialize`/`Deserialize` errors. |
| `json` | Adds JSON parse errors. |
| `csv` | Adds CSV parse errors with row/column context in the message when practical. |
| `dynamic` | Adds dynamic coercion/missing-value errors. |

Error enum variants may be `#[cfg]`-gated if doing so does not make error handling awkward. A single non-exhaustive enum is preferred.

## 6. Data Model

Recommended public error model:

```rust
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenError {
    Shape { operation: &'static str, message: String },
    Broadcast { left: Vec<usize>, right: Vec<usize> },
    Allocation { requested_elements: usize, message: String },
    Slice { input: Option<String>, message: String },
    Parse { format: DataFormat, message: String },
    Io { path: std::path::PathBuf, source: std::io::Error },
    Unsupported { operation: &'static str, message: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DataFormat {
    Json,
    Csv,
}
```

`MattenError` derives **only `Debug`**: `Io` embeds `std::io::Error`, which is
neither `Clone` nor `PartialEq`, so `MattenError` is **not** `Clone`/`PartialEq`/`Eq`.

Variant mapping (every failure maps onto these seven):

- construction / reshape length & shape errors -> `Shape`;
- broadcasting incompatibility -> `Broadcast` (never folded into `Shape`);
- shape-product overflow / oversized allocation / `arange` length -> `Allocation`;
- `slice_str` and slice-builder parse/bounds errors -> `Slice`;
- JSON / CSV / serde parse errors -> `Parse { format }` (CSV row/column go in `message`);
- file I/O errors -> `Io`;
- disabled-feature or unsupported-operation errors -> `Unsupported`.

`Shape` also covers constructor argument validation when the invalid argument determines the produced tensor shape, such as `arange` step and finite-bound checks.

`DataFormat` is a sanctioned public export because it appears in `Parse`.

Required traits:

```rust
impl std::fmt::Display for MattenError;
impl std::error::Error for MattenError;
```

These are implemented manually; `thiserror` is not required for Phase 1.

Testing rule: match `MattenError` by variant, never by equality:

```rust
let err = Tensor::load_csv("missing.csv").unwrap_err();
assert!(matches!(err, MattenError::Io { .. }));
```

## 7. Data Lifecycle

Error lifecycle:

1. Operation detects invalid input.
2. Panic-zone API converts error into `panic!` with standard message.
3. Result-zone API returns `Err(MattenError::...)`.
4. Application decides whether to show, log, or recover.

Internal helper functions should generally return `Result<T, MattenError>`. Public convenience methods may unwrap with custom panic text.

## 8. Events

Conceptual error events:

| Event | Trigger | Required API behavior |
|---|---|---|
| shape mismatch | constructor/reshape/broadcast | panic or `Err` by zone |
| invalid axis | swap/reduction/slice | panic or `Err` by zone |
| parse failure | `slice_str`, JSON, CSV | always `Err` |
| allocation too large | boundary constructor/parser | `Err` in Result-zone |
| unsupported feature | disabled feature call or config mismatch | compile-time absence preferred; otherwise `Err` |

No public event system is introduced.

## 9. Store Access

Error model covers store access from:

- files loaded by `load_json` / `load_csv`;
- string parse sources;
- serde deserialization sources.

File APIs must convert `std::io::Error` into `MattenError::Io` or equivalent. Parser APIs must include the source category.

## 10. Public API

```rust
pub enum MattenError { /* non-exhaustive */ }

impl Tensor {
    pub fn try_new(data: Vec<f64>, shape: &[usize]) -> Result<Tensor, MattenError>;
    pub fn try_reshape(&self, shape: &[usize]) -> Result<Tensor, MattenError>;
    pub fn slice_str(&self, spec: &str) -> Result<Tensor, MattenError>;

    #[cfg(feature = "serde")]
    pub fn from_json(input: &str) -> Result<Tensor, MattenError>;
}
```

The exact JSON/CSV API is finalized in RFC-009.

## 11. Panic Message Standard

Panic messages must follow this shape:

```text
matten <category> error in <operation>: <specific problem>; <hint>
```

Examples:

```text
matten shape error in Tensor::new: data length 5 cannot fill shape [2, 3] because the shape requires 6 elements; change the data length or the shape
```

```text
matten axis error in swap_axes: axis 3 is out of bounds for shape [2, 3] with rank 2
```

```text
matten broadcast error in add: shapes [2, 3] and [2] are not compatible
```

## 12. Internal Design

### 12.1 Error constructors

Use internal constructors to keep messages consistent:

```rust
impl MattenError {
    pub(crate) fn shape(message: impl Into<String>) -> Self;
    pub(crate) fn slice(message: impl Into<String>) -> Self;
    pub(crate) fn parse(message: impl Into<String>) -> Self;
}
```

### 12.2 Panic wrappers

```rust
fn panic_with_context(operation: &str, err: MattenError) -> ! {
    panic!("matten error in {{}}: {{}}", operation, err);
}
```

Use dedicated helper functions per category if they produce clearer messages.

### 12.3 Boundary limits

Boundary APIs should call a limits-aware validator before allocation. The exact user customization API can be deferred, but tests should validate that extremely large shapes are rejected before allocation attempts.

## 13. Boundary API Classification

MUST return `Result`:

- `try_new`;
- `try_reshape`;
- `slice_str`;
- JSON parsing;
- CSV parsing;
- file loading;
- dynamic coercion helpers where input may be non-numeric;
- any API that accepts a user-provided parser string.

MAY panic:

- `new`;
- `reshape`;
- arithmetic operators;
- `matmul` and reductions on invalid shapes/axes if not provided as `try_*`;
- simple fill constructors if shape is invalid.

## 14. Testing

- `Display` tests for all error categories.
- Panic tests with exact or partial message matching.
- Boundary APIs must not panic on malformed input.
- Fuzz tests for `slice_str` after RFC-008 implementation.
- Allocation-overflow tests.

## 15. Acceptance Criteria

- `MattenError` exists, derives only `Debug`, implements `Display` + `Error`, and exports `DataFormat`.
- Boundary APIs return `Result`.
- Panic messages follow the project prefix and context standard.
- Internal validation helpers return `Result` before panic wrappers convert them.
- Documentation includes a panic-zone vs Result-zone table.
