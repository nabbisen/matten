# `matten` External Design

**Project:** `matten`  
**Document Kind:** External Design / Public API Contract  
**Document Version:** `0.3.0`  
**Target Crate Version:** `0.1.0` for Phase 1, with Phase 2 public-direction notes  
**Date:** 2026-06-21  
**Status:** Reconciled with v0.16+ companion-crate roadmap  
**Revision note:** `0.3.0` continues the `0.2.0` line (no `1.0` document baseline exists yet); it incorporates the v0.16+ companion-crate reconciliation and the boundary-confirmation corrections.  

---

## 0. Reading Guide

This document defines the external design of `matten`, a Rust multidimensional array library focused on rapid prototyping, data exploration, and business Proof of Concept workflows.

This is not an internal implementation design. It defines the public contract that users, application developers, documentation authors, and implementation RFCs should rely on.

The implementation may change internal storage, indexing, parser strategy, or allocation strategy as long as it preserves the public behavior described here.

Requirement language follows this convention:

- **MUST**: required for the relevant release or feature.
- **SHOULD**: strongly recommended unless an RFC explicitly changes it.
- **MAY**: permitted but not required.
- **MUST NOT**: explicitly prohibited.

### 0.1 Changes in v0.2

This revision updates the external design after the roadmap added **RFC-014: Example Suite and Executable Documentation**. The update is intentionally scoped:

- it adds an examples/documentation contract;
- it adds RFC-014 to the RFC dependency map and release gates;
- it clarifies that examples must demonstrate accepted APIs only;
- it does **not** change the core `Tensor` data model, shape model, storage model, arithmetic contract, or boundary-error policy.



### 0.2 Changes in v0.3

This revision incorporates the v0.16+ companion-crate prospect and roadmap reconciliation:

- core `matten` remains Sedan-first and dependency-light;
- `dynamic` remains an ingestion / cleanup / explicit numeric conversion on-ramp;
- bridge integrations move out of core and into companion crates;
- `matten-ndarray` becomes the first companion implementation target;
- `matten-mlprep` follows as a transparent preprocessing crate;
- `matten-data` is delayed to a v0.20+ beta decision gate;
- streaming / large CSV remains design-only until batch, schema, error, and memory policy are proven.

Old text that described bridge examples as core feature-gated examples is superseded by companion-crate examples.

---

## 1. Design Goals

### 1.1 Mission

`matten` is a developer-centric multidimensional array library for Rust. Its purpose is to make early-stage mathematical, numerical, and data-oriented Rust development feel closer to Python NumPy/Pandas ergonomics while retaining Rust packaging, type checking, and deployment benefits.

The project intentionally prioritizes **Developer Experience (DX)** over peak performance.

### 1.2 Product Positioning

`matten` is the **Family Car** of Rust multidimensional array libraries:

- easy to start;
- predictable in normal usage;
- comfortable for non-expert Rust developers;
- optimized for proof-of-concept speed;
- intentionally not a Formula 1 replacement for `ndarray`, `nalgebra`, or specialized tensor engines.

### 1.3 Primary User Stories

The external design optimizes for these users:

1. **Rust application developer doing a PoC**  
   Wants to quickly represent vectors, matrices, and tensors without lifetime puzzles.

2. **Web/API developer using Rust**  
   Wants to parse JSON/CSV-like data into a tensor and return results through serde-enabled APIs.

3. **Business/data workflow developer**  
   Wants Rust code that can manipulate small-to-medium data arrays without building a full dataframe stack.

4. **Mathematical prototyper**  
   Wants simple shape manipulation, broadcasting, and arithmetic operators before later migrating hot paths to a faster library if necessary.

### 1.4 Non-Goals

`matten` v0.1.x does not attempt to provide:

- GPU acceleration;
- BLAS/LAPACK integration;
- sparse tensors;
- automatic differentiation;
- type-level dimension safety;
- compile-time shape arithmetic;
- zero-copy borrowed views in Phase 1;
- a full dataframe/query engine;
- examples that imply dataframe joins, group-by, pivot, ML training, GPU computing, or advanced linear algebra are current scope;
- distributed computation;
- cryptographic security guarantees;
- real-time or hard memory-bound execution guarantees.

---

## 2. Public API Principles

### 2.1 Single Primary User Type

The primary public user type is:

```rust
use matten::Tensor;
```

Users MUST be able to perform ordinary work using only `Tensor` in the default Phase 1 configuration.

In the `dynamic` feature configuration, `Element` is additionally exposed for mixed-type data:

```rust
#[cfg(feature = "dynamic")]
use matten::{Element, Tensor};
```

Errors at external boundaries are represented by `MattenError`:

```rust
use matten::{MattenError, Tensor};
```

### 2.2 Public Surface Rule

The root crate namespace SHOULD expose only the stable user-facing types and functions.

Allowed public root exports:

```rust
pub use crate::tensor::Tensor;
pub use crate::error::MattenError;

#[cfg(feature = "dynamic")]
pub use crate::element::Element;
```

The implementation MUST NOT require ordinary users to import internal storage types, layout traits, lifetime-bearing view types, or feature-specific engine structs.

The following style is prohibited for normal user-facing examples:

```rust
use matten::internal::Phase1Storage;
use matten::layout::RowMajorIndex;
use matten::traits::BroadcastShape;
```

### 2.3 No Public Lifetime Burden

Public examples MUST NOT require visible lifetime annotations for normal tensor creation, slicing, reshaping, arithmetic, or serialization.

This is a core design rule. If an implementation detail would force public lifetime propagation, the implementation must copy, materialize, or otherwise hide that detail.

### 2.4 Concrete Over Generic

Default `matten` MUST NOT expose a generic tensor type such as:

```rust
Tensor<T, D>
```

The default Phase 1 tensor is numerically fixed to `f64`.

Phase 2 dynamic support is feature-gated and exposes mixed values through `Element`, not through user-chosen generic dtype parameters.

---

## 3. Cargo Feature Design

### 3.1 Feature Matrix

| Feature | Default | Purpose | Public Impact |
|---|---:|---|---|
| `serde` | Yes | JSON/web/API integration | Enables serialization/deserialization support. |
| `dynamic` | No | Mixed-type data support | Adds `Element` and dynamic-data methods. |

Recommended dependency forms:

```toml
# Phase 1: default f64 numerical engine
matten = "0.1"

# Phase 2: mixed-type dynamic engine
matten = { version = "0.1", features = ["dynamic"] }
```

### 3.2 Default Feature Contract

The default feature set MUST remain small and fast to compile.

Default `matten` MUST provide:

- `Tensor` backed by numeric `f64` data;
- construction APIs;
- shape inspection;
- reshape;
- transpose/swap axes where implemented;
- builder-based slicing when implemented;
- arithmetic operators;
- broadcasting;
- serde integration when the default `serde` feature is enabled;
- standard conversions from/to common Rust vectors.

### 3.3 `dynamic` Feature Contract

The `dynamic` feature MUST add support for messy data:

```rust
#[cfg(feature = "dynamic")]
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Float(f64),
    Int(i64),
    Text(/* internal representation TBD by RFC */),
    Bool(bool),
    None,
}
```

The external semantic contract is `Element::Text` represents UTF-8 text. The exact internal representation of the text payload is intentionally left to RFC design because memory layout is a known Phase 2 risk.

`dynamic` MUST NOT change the fact that `Tensor` is the primary user-facing container.

### 3.4 Feature Compatibility Rule

Feature-gated functionality MAY add methods, but it MUST NOT make ordinary Phase 1 code fail to compile when `dynamic` is disabled.

The following style SHOULD remain valid in both default and `dynamic` builds where possible:

```rust
use matten::Tensor;

let a = Tensor::ones(&[2, 2]);
let b = Tensor::zeros(&[2, 2]);
let c = &a + &b;
```

---

## 4. Data Model

### 4.1 Tensor Shape Model

A `Tensor` has:

- a flat data buffer;
- a shape vector;
- row-major logical layout;
- a rank equal to `shape.len()`;
- a length equal to the product of all shape dimensions.

Terminology:

| Term | Meaning |
|---|---|
| scalar | rank-0 tensor, shape `[]`, length `1` |
| vector | rank-1 tensor, shape `[n]` |
| matrix | rank-2 tensor, shape `[rows, cols]` |
| tensor | rank-N tensor, shape with `N >= 0` |

### 4.2 Shape Invariants

For all valid tensors:

```text
len(data) == product(shape)
rank == shape.len()
```

Shape product calculation MUST use checked arithmetic. Product overflow MUST NOT silently wrap.

The default maximum rank for v0.1.x is:

```text
MAX_NDIM = 8
```

This is a DX and implementation-simplicity limit, not a mathematical limit.

### 4.3 Empty and Zero Dimensions

Rank-0 scalar tensors are valid:

```rust
let scalar = Tensor::new(vec![42.0], &[]);
assert_eq!(scalar.shape(), &[]);
assert_eq!(scalar.len(), 1);
```

A dimension value of zero is not part of the initial v0.1 contract unless explicitly accepted by a later RFC. For v0.1, constructors SHOULD reject zero dimensions to avoid surprising empty-array behavior.

If zero-sized tensors are later supported, their behavior must be defined in a dedicated RFC.

### 4.4 Phase 1 Storage Contract

In the default configuration, `Tensor` is conceptually equivalent to:

```rust
pub struct Tensor {
    data: Vec<f64>,
    shape: Vec<usize>,
}
```

The fields are not public.

External guarantees:

- values are `f64`;
- tensors own their data;
- operations may allocate freely;
- reshape and slicing materialize independent owned tensors;
- no borrowed view lifetime appears in the public API.

### 4.5 Phase 2 Dynamic Storage Contract

With `dynamic`, a tensor conceptually stores `Element` values.

The initial high-level storage direction is Copy-on-Write using shared backing data, but the exact representation is RFC-owned.

External guarantees:

- `Element::Float`, `Element::Int`, `Element::Text`, `Element::Bool`, and `Element::None` are supported semantically;
- slicing and reshaping SHOULD be cheap where possible;
- mutation MUST NOT unexpectedly mutate unrelated tensor values visible to the user;
- missing values are represented by `Element::None`;
- text storage must be chosen after explicit memory-layout evaluation.

---

## 5. Tensor Lifecycle

### 5.1 Lifecycle Overview

A `Tensor` moves through this lifecycle:

```text
external data / Rust values
        |
        v
construction / parsing
        |
        v
shape validation + materialization
        |
        v
owned Tensor
        |
        +--> reshape / transpose / slicing
        |
        +--> arithmetic / broadcasting
        |
        +--> serialization / conversion / file output
```

### 5.2 Construction

Primary constructors:

```rust
impl Tensor {
    pub fn new(data: Vec<f64>, shape: &[usize]) -> Tensor;
    pub fn try_new(data: Vec<f64>, shape: &[usize]) -> Result<Tensor, MattenError>;

    pub fn zeros(shape: &[usize]) -> Tensor;
    pub fn ones(shape: &[usize]) -> Tensor;
    pub fn full(shape: &[usize], value: f64) -> Tensor;
}
```

Design rules:

- `new`, `zeros`, `ones`, and `full` MAY panic on invalid shape because they are local construction convenience APIs.
- `try_new` MUST return `Result` and MUST NOT panic for invalid shape or length mismatch.
- All constructors MUST validate rank, shape product, and data length.

Example:

```rust
let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let b = Tensor::ones(&[2, 2]);
let c = &a + &b;
```

### 5.3 Shape Inspection

Minimum inspection APIs:

```rust
impl Tensor {
    pub fn shape(&self) -> &[usize];
    pub fn ndim(&self) -> usize;
    pub fn len(&self) -> usize;
    pub fn is_scalar(&self) -> bool;
    pub fn is_vector(&self) -> bool;
    pub fn is_matrix(&self) -> bool;
}
```

These methods MUST be cheap and MUST NOT allocate.

### 5.4 Data Access

Phase 1 SHOULD provide safe read access to flat data:

```rust
impl Tensor {
    pub fn as_slice(&self) -> &[f64];
    pub fn to_vec(&self) -> Vec<f64>;
}
```

`as_slice()` is only valid because Phase 1 guarantees contiguous owned `f64` storage.

For `dynamic`, public access must be defined carefully so it does not expose internal CoW layout. The likely contract is:

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    pub fn element(&self, index: &[usize]) -> Option<&Element>;
}
```

The exact API is RFC-owned.

### 5.5 Reshape

```rust
impl Tensor {
    pub fn reshape(&self, new_shape: &[usize]) -> Tensor;
    pub fn try_reshape(&self, new_shape: &[usize]) -> Result<Tensor, MattenError>;
}
```

External behavior:

- The product of `new_shape` must equal the current length.
- `reshape` MAY panic with a descriptive message on mismatch.
- `try_reshape` MUST return `Result`.
- Phase 1 SHOULD deep-copy or otherwise materialize an independent tensor.
- Phase 1 MUST NOT expose layout/contiguity failure to users.

Example:

```rust
let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let flat = a.reshape(&[4]);
```

### 5.6 Transpose and Axis Swapping

```rust
impl Tensor {
    pub fn transpose(&self) -> Tensor;
    pub fn swap_axes(&self, axis1: usize, axis2: usize) -> Tensor;
}
```

External behavior:

- `transpose()` reverses axis order for rank >= 2.
- `swap_axes()` swaps two specified axes.
- Invalid axes MAY panic in the convenience API.
- If later `try_swap_axes()` is added, it MUST return `Result` for invalid axes.
- Phase 1 SHOULD materialize into a contiguous owned tensor after transformation.

### 5.7 Slicing

Slicing has two public forms:

1. builder API, canonical;
2. string API, convenience.

#### 5.7.1 Builder API

The builder API is the canonical slicing API because it is Rust-native and avoids parser ambiguity.

Conceptual example:

```rust
let subset = tensor
    .slice()
    .range(0..2)
    .range(..)
    .build()?;
```

Design rules:

- `slice()` returns a builder value.
- The builder SHOULD return `Result<Tensor, MattenError>` from `build()`.
- Builder slicing MUST validate rank and bounds.
- Phase 1 SHOULD materialize an owned tensor.
- Phase 2 MAY return a cheap CoW-backed view if the public semantics remain value-safe.

#### 5.7.2 String Slice API

String slicing exists for NumPy-like familiarity:

```rust
let subset = tensor.slice_str("0:2, :")?;
```

Design rules:

- `slice_str` MUST return `Result<Tensor, MattenError>`.
- `slice_str` MUST NOT panic on malformed input.
- Input length MUST be bounded internally.
- Grammar MUST be intentionally small for v0.1.
- The builder API remains the canonical form in documentation and implementation design.

Minimum grammar:

| Pattern | Meaning |
|---|---|
| `:` | all elements in an axis |
| `start:end` | half-open range |
| `start:` | from start to axis end |
| `:end` | from axis start to end |
| `index` | single index |
| `::step` | stepped range, optional after RFC approval |

A public slicing macro is not part of the initial external design.

---

## 6. Arithmetic and Broadcasting

### 6.1 Operator Traits

Phase 1 MUST support standard borrowed tensor arithmetic:

```rust
use std::ops::{Add, Div, Mul, Neg, Sub};

impl Add for &Tensor {
    type Output = Tensor;
    fn add(self, rhs: &Tensor) -> Tensor;
}

impl Sub for &Tensor { /* element-wise subtraction */ }
impl Mul for &Tensor { /* element-wise multiplication */ }
impl Div for &Tensor { /* element-wise division */ }
impl Neg for &Tensor { /* element-wise negation */ }
```

Scalar operations SHOULD be supported:

```rust
impl Mul<f64> for &Tensor { /* tensor * scalar */ }
impl Mul<&Tensor> for f64 { /* scalar * tensor */ }
```

Actual implementation may add owned variants for ergonomic chaining, but examples SHOULD prefer borrowed operands to avoid accidental moves.

### 6.2 Broadcasting

Broadcasting follows NumPy-style right-aligned shape compatibility.

Rules:

1. Compare dimensions from the trailing axis backward.
2. Two dimensions are compatible when:
   - they are equal; or
   - one of them is `1`.
3. Missing leading dimensions are treated as `1`.
4. Result dimension is the maximum of the two compatible dimensions.
5. Incompatible shapes cause a descriptive panic in operator APIs.

Examples:

```text
[3, 4] + [4]    => [3, 4]
[2, 3, 4] + [4] => [2, 3, 4]
[2, 1] + [3]    => [2, 3]
[] + [3, 4]     => [3, 4]
[2, 3] + [4]    => error
```

### 6.3 Numeric Semantics

Phase 1 uses ordinary `f64` semantics:

- `NaN` propagates according to IEEE 754 behavior;
- division by zero follows Rust `f64` behavior and may produce `inf`, `-inf`, or `NaN`;
- `matten` MUST NOT silently sanitize `NaN` or `inf` during ordinary arithmetic;
- explicit cleaning helpers MAY be added later.

### 6.4 Matrix Multiplication

Matrix multiplication is not required in the minimum v0.1 API unless explicitly scheduled by RFC.

If added, it SHOULD use a clear method name:

```rust
let c = a.matmul(&b);
```

The `*` operator remains element-wise multiplication.

---

## 7. Data Integration

### 7.1 Serde Contract

`serde` integration is first-class.

The canonical serialized tensor representation SHOULD be an object form:

```json
{
  "shape": [2, 2],
  "data": [1.0, 2.0, 3.0, 4.0]
}
```

Rationale:

- stable across dimensions;
- easy to validate;
- compact enough for APIs;
- avoids ambiguity in ragged nested arrays;
- maps directly to internal flat storage.

### 7.2 JSON Parsing Convenience

For DX, `Tensor::from_json` MAY accept both canonical object form and nested numeric arrays:

```rust
let a = Tensor::from_json(r#"[[1.0, 2.0], [3.0, 4.0]]"#)?;
```

Boundary rule:

```rust
impl Tensor {
    pub fn from_json(input: &str) -> Result<Tensor, MattenError>;
}
```

`from_json` MUST NOT panic for malformed JSON, ragged arrays, unsupported values, shape overflow, or allocation rejection.

### 7.3 CSV Parsing

CSV parsing is an external input boundary.

```rust
impl Tensor {
    pub fn from_csv(input: &str) -> Result<Tensor, MattenError>;
}
```

Phase 1 CSV behavior:

- numeric-only CSV;
- all rows must have equal column count;
- every value must parse as `f64`;
- malformed rows return `MattenError` with row/column context where practical.

Phase 2 CSV behavior under `dynamic`:

- numeric, boolean, text, and missing values MAY be supported;
- coercion policy must be defined by RFC;
- missing cells SHOULD map to `Element::None`.

### 7.4 File Loading

File I/O MUST return `Result`.

```rust
impl Tensor {
    pub fn load_json(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError>;
    pub fn load_csv(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError>;
}
```

File loading MUST NOT panic for missing files, permission errors, invalid UTF-8 where applicable, parse errors, or shape mismatch.

### 7.5 Standard Rust Conversions

Required conversions:

```rust
impl From<Vec<f64>> for Tensor;
impl TryFrom<Vec<Vec<f64>>> for Tensor;
impl From<Tensor> for Vec<f64>;
```

Recommended conversion behavior:

- `Vec<f64>` becomes shape `[len]`.
- `Vec<Vec<f64>>` becomes shape `[rows, cols]` if rectangular.
- Ragged nested vectors MUST return `Result` through `TryFrom`, not panic.
- `From<Tensor> for Vec<f64>` consumes the tensor and returns flat row-major data.

A panicking `From<Vec<Vec<f64>> for Tensor>` SHOULD NOT be used because nested vectors are often external-ish data and may be ragged.

---

## 8. Error Model

### 8.1 Public Error Type

External boundary errors use:

```rust
pub enum MattenError {
    InvalidShape { shape: Vec<usize>, reason: String },
    ShapeMismatch { left: Vec<usize>, right: Vec<usize>, operation: &'static str },
    LengthMismatch { expected: usize, actual: usize, shape: Vec<usize> },
    DimensionOutOfRange { axis: usize, ndim: usize },
    SliceParse { input: String, reason: String },
    SliceOutOfBounds { axis: usize, start: usize, end: usize, dim: usize },
    Parse { source: String, reason: String },
    Io { path: Option<std::path::PathBuf>, reason: String },
    AllocationTooLarge { requested_elements: usize, reason: String },
    UnsupportedType { context: String },
    NumericCoercion { value: String, target: &'static str },
}
```

Exact variant names are RFC-owned, but the final error type MUST express these categories.

### 8.2 Panic Zone vs Result Zone

`matten` intentionally has two error zones.

#### Panic Zone

Local, internal, developer-authored math may panic with descriptive messages.

Examples:

```rust
let c = &a + &b;          // may panic on incompatible shapes
let d = a.reshape(&[3]);  // may panic on invalid element count
let z = Tensor::zeros(&[usize::MAX]); // may panic or abort before allocation attempt
```

Panic messages MUST include actionable context:

- operation name;
- left/right shapes where applicable;
- expected and actual element counts;
- invalid axis or slice range.

#### Result Zone

External boundary APIs MUST return `Result<Tensor, MattenError>` and MUST NOT panic for ordinary invalid input.

Examples:

```rust
Tensor::try_new(data, shape);
Tensor::from_json(input);
Tensor::from_csv(input);
Tensor::load_json(path);
Tensor::load_csv(path);
tensor.slice_str(input);
tensor.try_reshape(shape);
```

### 8.3 Error Message Tone

Error messages SHOULD be human-readable and developer-friendly.

Poor:

```text
assertion failed: left == right
```

Good:

```text
matten reshape error: cannot reshape tensor with 4 elements and shape [2, 2] into shape [3] because the new shape requires 3 elements.
```

---

## 9. Dynamic Feature External Design

### 9.1 Element Semantics

With `dynamic`, values are represented as:

```rust
#[cfg(feature = "dynamic")]
pub enum Element {
    Float(f64),
    Int(i64),
    Text(/* representation decided by RFC */),
    Bool(bool),
    None,
}
```

The semantic meaning is stable even if the internal representation changes.

### 9.2 Missing Values

`Element::None` represents missing data.

Required behaviors:

```rust
#[cfg(feature = "dynamic")]
impl Element {
    pub fn is_none(&self) -> bool;
    pub fn try_as_f64(&self) -> Option<f64>;
}
```

Possible tensor-level helpers:

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    pub fn fill_none(&self, fallback: Element) -> Tensor;
    pub fn fill_none_f64(&self, fallback: f64) -> Tensor;
}
```

Exact method names are RFC-owned.

### 9.3 Numeric Coercion Policy

Dynamic arithmetic MUST NOT guess ambiguous conversions silently.

Permitted automatic numeric coercions:

| From | To | Allowed? |
|---|---|---:|
| `Int(i64)` | `Float(f64)` | Yes, if documented |
| `Float(f64)` | `Float(f64)` | Yes |
| `Bool(bool)` | numeric | No by default |
| `Text(..)` | numeric | No by default |
| `None` | numeric | No by default unless explicit fill/coercion API used |

String-to-number parsing MUST require an explicit user-facing conversion API, not ordinary arithmetic.

### 9.4 Dynamic Text Storage

The external API MUST NOT prematurely require `Text(String)` as the final storage representation.

Reason: Rust enum size is determined by the largest variant, and a text variant may inflate every element. This is a known Phase 2 memory risk.

Phase 2 RFCs must compare at least:

- `String`;
- `Box<str>`;
- `Arc<str>`;
- small-string optimization crate options;
- string interning for repeated values;
- separate typed columns or side storage if needed.

External contract: users see text as UTF-8 string data. Internal layout is not user-visible.

---

## 10. Formatting and Developer Output

### 10.1 Debug Formatting

`Debug` SHOULD include shape and compact data:

```text
Tensor(shape=[2, 2], data=[1.0, 2.0, 3.0, 4.0])
```

For large tensors, debug output SHOULD truncate values.

### 10.2 Display Formatting

`Display` MAY provide a matrix-like format for rank-1 and rank-2 tensors.

For rank >= 3, `Display` MAY fall back to compact shape-first output.

### 10.3 Error-Friendly Examples

Documentation examples SHOULD show both convenience and boundary-safe usage:

```rust
let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);

let b = Tensor::try_new(vec![1.0, 2.0, 3.0], &[2, 2]);
assert!(b.is_err());
```

---

## 11. Testing and Compatibility Contract

### 11.1 Public API Tests

Every public API example in this document SHOULD be represented by a doctest or integration test once implemented.

### 11.2 Shape and Broadcasting Tests

Required test categories:

- valid construction;
- invalid construction;
- scalar shape;
- vector shape;
- matrix shape;
- reshape success/failure;
- broadcasting success cases;
- broadcasting failure cases;
- arithmetic output values;
- transposition layout correctness;
- slice builder success/failure;
- `slice_str` parse success/failure.

### 11.3 Golden Comparison Tests

For operations that intentionally follow NumPy semantics, the project SHOULD maintain small golden fixtures generated from NumPy.

Examples:

- broadcasting result shapes;
- element-wise arithmetic;
- reshape;
- transpose;
- slicing.

Golden tests are for semantic compatibility, not for claiming NumPy performance parity.

### 11.4 Fuzz and Property Testing

Boundary-facing APIs SHOULD have fuzz or property tests:

- `try_new` shape validation;
- broadcasting shape calculation;
- `slice_str` parser;
- JSON parsing;
- CSV parsing;
- dynamic coercion once implemented.

Property examples:

```text
reshape preserves element count
broadcast result dimensions are compatible with both operands
slice_str never panics for arbitrary UTF-8 input
from_json never panics for arbitrary UTF-8 input
```

---

## 12. Threat Model and Boundary Safety Policy

### 12.1 Scope

`matten` is not a cryptographic library, authentication library, sandbox, or network service. Its threat model is therefore focused on:

- host process availability;
- memory exhaustion prevention;
- panic containment at external boundaries;
- parser robustness;
- numerical/data correctness;
- predictable public API behavior.

This section is part of the external contract because `matten` is expected to be used inside web APIs, data pipelines, CLI tools, and business PoCs.

### 12.2 Assets Protected

| Asset | Protection Goal |
|---|---|
| Host process availability | Malformed external data should not crash web servers or jobs through ordinary boundary APIs. |
| Memory budget | Shape/product overflow and unrealistic allocations should be detected before allocation where practical. |
| Numerical correctness | Broadcasting, reshaping, slicing, and coercion should not silently corrupt results. |
| API predictability | Users should know which APIs may panic and which return `Result`. |
| Serialization stability | JSON/serde representation should remain documented and migration-safe. |

### 12.3 Explicit Non-Assets

`matten` does not protect:

- confidentiality of tensor contents;
- side-channel leakage;
- malicious native code in the same process;
- denial-of-service caused by intentionally huge valid computations after the caller opts into them;
- untrusted plugin execution;
- malicious dependency compromise beyond normal Rust supply-chain controls.

### 12.4 Trust Boundaries

| Boundary | Examples | Required Behavior |
|---|---|---|
| JSON input | `Tensor::from_json` | Return `Result`, never ordinary parse panic. |
| CSV input | `Tensor::from_csv` | Return `Result` with row/column context where practical. |
| File input | `load_json`, `load_csv` | Return `Result` for I/O and parse errors. |
| Shape input | `try_new`, `try_reshape` | Checked product, rank validation, no silent overflow. |
| String slicing | `slice_str("0:2, :")` | Bounded parser, `Result`, no panic on malformed text. |
| Dynamic data | `Element::Text`, `Element::None` | Explicit memory and coercion policy. |
| Feature boundary | default vs `dynamic` | Same primary `Tensor` type and predictable behavior. |

### 12.5 Threats and Required Mitigations

#### T1. Shape Product Overflow

Threat:

```rust
Tensor::zeros(&[usize::MAX, usize::MAX]);
```

Mitigation:

- all shape product calculation MUST use checked multiplication;
- overflow MUST produce `MattenError` in Result-zone APIs;
- convenience APIs MAY panic with an actionable message;
- overflow MUST NOT wrap into a smaller allocation.

#### T2. Allocation Explosion / OOM

Threat:

```rust
Tensor::zeros(&[1_000_000_000, 1_000_000_000]);
```

Mitigation:

- constructors SHOULD check requested element count before allocation;
- boundary APIs MUST reject allocation requests that exceed internal safety limits or platform feasibility checks;
- documentation MUST warn that Phase 1 uses cloning/materialization and is not intended for huge data;
- Phase 2 RFCs must include memory-budget tests.

#### T3. Panic Propagation into Services

Threat: A web service calls a panicking tensor API on user-provided shape/data and crashes the request handler or process.

Mitigation:

- external data APIs MUST return `Result`;
- docs MUST clearly label panic-zone APIs;
- examples for web/API usage SHOULD use `try_*` or parsing APIs;
- panic messages MUST be descriptive for local debugging.

#### T4. Parser Abuse

Threat: malformed or intentionally strange `slice_str`, JSON, or CSV input causes panic, extreme allocation, or excessive processing.

Mitigation:

- parser-facing APIs MUST return `Result`;
- `slice_str` parser MUST be small and bounded;
- parser code SHOULD be fuzz-tested;
- deeply nested or ragged JSON must be rejected cleanly;
- CSV row length mismatch must be rejected cleanly.

#### T5. Silent Data Corruption

Threat: broadcasting, slicing, reshaping, or dynamic coercion produces plausible but incorrect results.

Mitigation:

- broadcasting rules MUST be explicitly NumPy-like and tested;
- shape mismatch MUST not be silently aligned except where rules define it;
- Phase 2 string-to-number coercion MUST be explicit;
- NaN/Inf behavior MUST be documented;
- golden tests SHOULD compare selected behavior against NumPy.

#### T6. Dynamic `Element` Memory Bloat

Threat: a large enum variant such as text inflates every element, causing Phase 2 to miss memory goals.

Mitigation:

- dynamic storage representation MUST be RFC-designed before stabilization;
- `Element::Text` internal representation MUST be memory-measured;
- Phase 2 memory tests MUST include mixed arrays with many non-text elements and repeated text;
- public API MUST avoid exposing layout choices that would prevent later optimization.

#### T7. Dependency and Macro Complexity

Threat: adding heavy parser/macro/type-system dependencies undermines the low-friction compile-time goal.

Mitigation:

- default dependencies MUST remain minimal;
- proc-macros MUST NOT be required for the core API;
- public API must avoid type-level dimension arithmetic;
- dependency additions SHOULD be justified in RFCs.

### 12.6 Unsafe Code Policy

Phase 1 SHOULD use:

```rust
#![forbid(unsafe_code)]
```

Any future use of `unsafe` must require a dedicated RFC and must justify:

- why safe Rust is insufficient;
- what invariant the unsafe block relies on;
- how the invariant is tested;
- what user-visible behavior improves.

### 12.7 Threat Model Ownership

The full engineering threat model should be owned by a dedicated RFC:

```text
RFC-00X: Threat Model and Boundary Safety Policy
```

This external-design section is the public contract summary. The RFC should contain detailed internal policy, parser limits, fuzzing scope, dynamic memory budget, and unsafe-code review process.

---

## 13. Documentation and Executable Examples Contract

### 13.1 Documentation as Public DX Contract

Documentation is part of the public developer experience contract for `matten`.

The project MUST treat examples as executable documentation, not decorative samples. A user should be able to find a small example that matches their immediate PoC pattern, copy it, and modify it without learning internal storage, lifetimes, or advanced Rust traits.

Documentation and examples MUST NOT create hidden new API requirements. An example may reveal that an accepted API name is awkward, but it must not introduce a new public capability that has not already been accepted through requirements, external design, or RFC.

### 13.2 Required Documentation Pages

The project SHOULD provide:

```text
README.md
docs/design/philosophy.md
docs/design/panic-vs-result.md
docs/design/threat-model.md
docs/design/examples-contract.md
```

The `docs/design/examples-contract.md` page SHOULD summarize RFC-014 for ordinary contributors and reviewers.

### 13.3 Documentation Principles

Docs MUST:

- start with simple examples;
- avoid exposing internal storage terminology early;
- compare familiar NumPy-like ideas where helpful;
- clearly label performance limitations;
- clearly label panic-zone APIs;
- clearly show `Result`-returning boundary APIs for JSON, CSV, files, and parser input;
- include migration hints to `ndarray`/`nalgebra` for performance-critical code;
- avoid making `matten` look like a full NumPy, pandas, nalgebra, ML, GPU, or dataframe replacement.

### 13.4 Example Scope Rule

Every example MUST fit exactly one of these classifications:

| Classification | Meaning | Release Relationship |
|---|---|---|
| Required for `0.1.0` | Demonstrates accepted Phase 1 API needed for the first release. | Blocks `0.1.0` release readiness. |
| Recommended for `0.1.x` | Demonstrates practical PoC patterns using already-accepted Phase 1 APIs. | Does not block `0.1.0`. |
| Required for `0.2.0 dynamic` | Demonstrates accepted `dynamic` feature APIs. | Blocks `0.2.0` dynamic release readiness. |
| Future theme | Useful later, but depends on additional feature design or dependency policy. | Not part of current release gates. |
| Out of current scope | Breaks the simplicity contract or implies another kind of framework. | Must not be added without a new roadmap decision. |

Examples MUST demonstrate accepted APIs only. They MUST NOT become a backdoor for adding advanced linear algebra, dataframe behavior, ML training, GPU usage, sparse tensors, or heavy integration dependencies.

### 13.5 Required `0.1.0` Examples

The following examples are required before `0.1.0` release readiness:

```text
examples/
  00_quickstart.rs
  01_create_tensor.rs
  02_shape_and_size.rs
  03_reshape_flatten.rs
  04_elementwise_ops.rs
  05_scalar_ops.rs
  06_broadcasting.rs
  07_transpose_swap_axes.rs
  08_slicing_builder.rs
  09_slice_str.rs
  10_json_roundtrip.rs
  11_csv_numeric_loading.rs
  12_boundary_error_handling.rs

examples/math/
  20_dot_product.rs
  21_matrix_vector_product.rs
  22_matrix_multiplication.rs
  23_sum_mean.rs
  24_min_max.rs
  25_normalize_vector.rs
  26_cosine_similarity.rs
```

These examples are allowed because they directly demonstrate the accepted `Tensor` construction, shape, reshape, operator, broadcasting, slicing, serde/CSV boundary, and basic mathematical-computing APIs.

### 13.6 Recommended `0.1.x` Examples

The following examples are recommended after `0.1.0`, but MUST NOT block the initial release:

```text
examples/patterns/
  standardize_columns.rs
  minmax_scaling.rs
  rowwise_scoring.rs
  column_summary.rs
  moving_average.rs
  rolling_windows_basic.rs
  pairwise_distance.rs
  gram_matrix.rs
```

These examples are useful PoC patterns, but they are application-level demonstrations. They should be added only after the core API has stabilized enough that the examples do not force new public APIs.

### 13.7 Required `0.2.0 dynamic` Examples

The following examples are required before the `dynamic` feature is considered release-ready:

```text
examples/dynamic/
  00_dynamic_quickstart.rs
  01_mixed_elements.rs
  02_missing_values.rs
  03_fill_none.rs
  04_numeric_coercion.rs
  05_dirty_csv_cleanup.rs
```

Dynamic examples MUST clearly state that `matten` is not a full dataframe library. The intended workflow is messy-data ingestion and cleanup for small-to-medium PoCs before converting to numeric tensors or ordinary Rust data structures.

### 13.8 Future Example Themes

The following examples are future themes and MUST NOT be required for the core `matten` crate:

```text
examples/business/
  sales_matrix_summary.rs
  kpi_score_matrix.rs
  forecast_feature_matrix.rs
  inventory_risk_score.rs
  customer_feature_scaling.rs
```

Business examples may be useful, but they must remain small tutorials around accepted core APIs or move to companion crates if they require table/domain semantics.

#### Superseded bridge-example policy

The older plan for core examples such as:

```text
examples/integration/ndarray_bridge.rs
examples/integration/nalgebra_bridge.rs
examples/integration/candle_bridge.rs
```

is superseded.

Bridge examples now belong in companion crates:

```text
matten-ndarray/examples/
matten-nalgebra/examples/    # deferred
matten-candle/examples/      # deferred
```

Core `matten` MUST NOT add optional `ndarray`, `nalgebra`, or `candle` features only to support bridge examples.

### 13.9 Explicitly Out-of-Scope Examples

The following example types are out of current scope:

- inverse, determinant, eigenvalues, SVD, QR, Cholesky, or other advanced linear algebra examples;
- automatic differentiation;
- neural-network training;
- dataframe joins, group-by, pivot, or query examples;
- GPU examples;
- sparse matrix examples;
- huge dataset benchmarks;
- SQL/database integration examples that imply ORM or DB-layer ownership.

These topics may only be reconsidered through a future roadmap/RFC update.

### 13.10 Example Style

Preferred example style:

```rust
//! Demonstrates element-wise addition and shape inspection.
//!
//! Run:
//! cargo run --example 04_elementwise_ops

use matten::Tensor;

fn main() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::ones(&[2, 2]);
    let c = &a + &b;

    assert_eq!(c.shape(), &[2, 2]);
    assert_eq!(c.as_slice(), &[2.0, 3.0, 4.0, 5.0]);
}
```

Boundary examples SHOULD prefer `Result`-returning `main` functions:

```rust
use matten::Tensor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tensor = Tensor::from_csv_path("examples/data/numeric_2x3.csv")?;
    println!("{:?}", tensor);
    Ok(())
}
```

Avoid examples that require complicated turbofish, explicit lifetimes, advanced trait imports, or non-default features unless the example is explicitly in a feature-gated integration section.

### 13.11 Example Fixture Policy

Example fixtures MUST be small, readable, and committed under:

```text
examples/data/
  numeric_2x3.csv
  tensor_payload.json
```

Dynamic examples MAY add:

```text
examples/data/
  messy_business_rows.csv
```

Fixtures MUST NOT contain large datasets, private-looking business data, generated benchmark blobs, or external downloads.

### 13.12 Example CI Contract

The release process MUST include example checks:

```bash
cargo check --examples
cargo test --examples
cargo run --example 00_quickstart
cargo run --example 06_broadcasting
cargo run --example 22_matrix_multiplication
cargo run --example 10_json_roundtrip
cargo run --example 11_csv_numeric_loading
```

When the `dynamic` feature becomes release-targeted, CI MUST also include:

```bash
cargo check --examples --features dynamic
```

Bridge examples are checked in their companion crates, not through core feature-gated jobs.

Superseded core commands:

```bash
cargo check --examples --features ndarray
cargo check --examples --features nalgebra
cargo check --examples --features candle
```

Required v0.16+ boundary CI instead:

```bash
scripts/check-core-dependency-boundary.sh
```

The boundary check must prove that core `matten` does not depend on `ndarray`, `nalgebra`, `candle-core`, `polars`, `arrow`, `datafusion`, or companion crates.


---

## 14. Release Compatibility Policy

### 14.1 v0.x Compatibility

During v0.x, API changes are allowed but SHOULD be minimized after a public release.

Breaking changes MUST be documented in release notes.

### 14.2 v1.0 Stabilization Expectations

Before v1.0, the project should stabilize:

- `Tensor` construction;
- shape inspection;
- reshape;
- arithmetic and broadcasting;
- canonical serde representation;
- error categories;
- panic-vs-Result policy;
- threat model policy;
- executable examples/documentation contract;
- dynamic feature scope if included before v1.0.

### 14.3 Migration Path to Performance Libraries

Documentation SHOULD show how to move data out of `matten`:

```rust
let flat: Vec<f64> = tensor.into();
```

This supports migration to `ndarray`, `nalgebra`, Candle, or custom numeric code when PoC code becomes production/performance-sensitive.

---

## 15. RFC Dependency Map

The following RFCs should be created or updated after this external design.

| RFC Theme | Reason |
|---|---|
| Threat Model and Boundary Safety Policy | Governs panic/Result zones, parser limits, allocation checks, fuzzing, unsafe policy. |
| Core Tensor Data Model | Defines shape invariants, rank limit, row-major layout, and construction behavior. |
| Arithmetic and Broadcasting | Defines operator behavior and NumPy-like compatibility rules. |
| Slicing API | Defines builder API, `slice_str` grammar, and parser safety. |
| Serde/JSON/CSV Boundary APIs | Defines canonical format, nested-array convenience, and parse errors. |
| Dynamic `Element` Model | Defines mixed values and coercion policy. |
| Dynamic Storage / CoW | Defines memory representation and CoW lifecycle after memory measurements. |
| Testing and Compatibility | Defines golden tests, fuzz targets, property tests, and documentation tests. |
| Example Suite and Executable Documentation | Defines required/recommended/future examples, fixture rules, CI commands, and scope guardrails. |

---

## 16. Open Questions

These questions should be resolved by RFC, not ad hoc implementation:

1. Should zero-sized tensors be supported in v0.1, or rejected until later?
2. What exact default allocation limit should boundary APIs enforce, if any?
3. Should `serde` be a default feature or an always-on dependency?
4. Should canonical JSON serialization support nested form, object form, or both?
5. Should `slice_str` support negative indexes in the future?
6. Should dynamic `Element::Text` use `Box<str>`, `Arc<str>`, interning, or another representation?
7. Should Phase 2 dynamic tensors remain one `Tensor` type internally, or use hidden storage engines behind the same public type?
8. Which APIs need `try_*` variants before `0.1.0`?
9. Should matrix multiplication be included before or after slicing?
10. What is the minimum supported Rust version policy?
11. Which `0.1.x` practical-pattern examples should graduate into required examples after real user feedback?

---

## 17. Acceptance Criteria for This External Design

This external design is ready for implementation RFC work when:

- the public API philosophy is accepted;
- Phase 1 and Phase 2 boundaries are understood;
- the panic-zone vs Result-zone policy is accepted;
- the threat model section is accepted as the external contract summary;
- the executable examples/documentation contract is accepted as a release-readiness gate;
- the RFC dependency map includes RFC-014 as a scope-controlled examples RFC;
- the RFC dependency map is accepted as the next planning step;
- no public API exposes internal lifetime-bearing view types;
- no initial requirement forces premature dynamic text storage layout.

---

## Appendix A. Minimal Phase 1 Example

```rust
use matten::Tensor;

fn main() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::ones(&[2, 2]);

    let c = &a + &b;
    let d = c.reshape(&[4]);

    println!("a = {:?}", a);
    println!("d = {:?}", d);
}
```

## Appendix B. Boundary-Safe Example

```rust
use matten::Tensor;

fn parse_user_json(input: &str) -> Result<Tensor, matten::MattenError> {
    let tensor = Tensor::from_json(input)?;
    let flat = tensor.try_reshape(&[tensor.len()])?;
    Ok(flat)
}
```

## Appendix C. Dynamic Feature Example

```rust
#[cfg(feature = "dynamic")]
use matten::{Element, Tensor};

#[cfg(feature = "dynamic")]
fn main() -> Result<(), matten::MattenError> {
    let input = r#"[
        [1.0, "active", true],
        [2.0, null, false]
    ]"#;

    let data = Tensor::from_json(input)?;
    println!("shape = {:?}", data.shape());

    Ok(())
}
```


---

## 18. v0.16+ Companion-Crate External Contract

### 18.1 Core contract

Core `matten` remains focused on `Tensor`, shape manipulation, broadcasting, slicing, reductions, matrix multiplication, dynamic ingestion/on-ramp utilities, limits, errors, and examples.

Core `matten` MUST NOT depend on companion crates or heavy external numeric/data frameworks.

### 18.2 Companion crate order

```text
v0.16.0
  companion boundary confirmation

v0.17.0
  matten-ndarray experimental

v0.18.0
  matten-mlprep experimental

v0.19.0
  matten-ndarray production-ready candidate
  matten-mlprep beta decision / hardening

v0.20+
  matten-data beta decision phase

v0.25.0
  matten-ndarray production-ready

v0.26.0
  matten-mlprep production-ready candidate

v0.27.0
  matten-data production-ready candidate

later
  nalgebra / candle / streaming only after separate RFCs
```

### 18.3 Versioning

Companion crates use independent SemVer. A core `matten` version does not imply maturity of any companion crate.

### 18.4 Error policy

Each companion crate defines its own error type. Companion crates may wrap `matten::MattenError`, but core `MattenError` must not absorb companion-specific failure modes.

### 18.5 External examples

Examples for bridge, preprocessing, and table workflows live in the corresponding companion crate once that crate exists. Core `matten` may link to released companion examples but must not contain broken pseudo-examples or add feature-gated bridge dependencies.
