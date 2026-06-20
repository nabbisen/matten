# RFC-016: Dynamic Ingestion and Explicit Numeric On-Ramp

**Status:** Implemented (v0.14.0)  
**Target:** v0.14.x  
**Theme:** Dynamic on-ramp hardening  
**Depends on:** RFC-011, RFC-012, RFC-017, RFC-018  
**Related handoff:** `016-dynamic-ingestion-and-explicit-numeric-on-ramp-handoff.md`

## 1. Summary

This RFC defines the long-term positioning of the `dynamic` feature as a messy-data ingestion and cleanup on-ramp, not as a peer computation engine.

`matten` is currently closest to the Sedan-like user story: clean numeric data, easy matrix/tensor operations, minimal code. The dynamic feature should help users handle JSON/CSV/database-like messiness before returning to numeric tensors through explicit conversion.

The intended lifecycle is:

```text
messy input
  -> dynamic Tensor
  -> inspect / clean / fill
  -> explicit try_numeric()
  -> normal f64 Tensor computation
```

## 2. Goals

- Make dynamic data useful without making `matten` a dataframe crate.
- Preserve the Sedan-first core identity.
- Avoid silent mixed-type arithmetic.
- Add lightweight inspection utilities for conversion readiness.
- Improve user confidence when cleaning messy CSV/JSON data.
- Keep default numeric code simple and fast to compile.

## 3. Non-goals

- No dynamic arithmetic in this RFC.
- No dynamic broadcasting.
- No dynamic matrix multiplication.
- No dataframe joins, group-by, pivot, SQL-like filtering, or column-name model.
- No large-data streaming implementation.
- No guarantee that dynamic tensors are optimized for huge datasets.

## 4. External design

### 4.1 Dynamic public story

Documentation should describe dynamic as:

> The `dynamic` feature helps ingest and clean heterogeneous input before explicit conversion into numeric tensors.

Avoid:

> Dynamic tensors support the complete tensor API.

### 4.2 Supported dynamic operations

Current and near-term dynamic operations should be limited to:

| Category | Example |
|---|---|
| Construction | `from_elements`, `try_from_elements` |
| Ingestion | `from_json_dynamic`, `from_csv_dynamic` |
| Inspection | `to_elements`, `count_none`, `none_mask`, `numeric_mask` |
| Cleanup | `fill_none`, `forward_fill_none` (optional later: `fill_none_with`, `try_forward_fill_none`) |
| Conversion | `try_numeric`, `try_numeric_with` |

### 4.3 Rejected dynamic operations

Numeric APIs should reject dynamic tensors with clear messages:

- `matmul`;
- `dot`;
- arithmetic operators;
- reductions;
- numeric `as_slice`;
- numeric `to_vec`;
- numeric `get`.

## 5. Data model

### 5.1 Element

The existing element model remains:

```rust
#[cfg(feature = "dynamic")]
pub enum Element {
    Float(f64),
    Int(i64),
    Text(Arc<str>),
    Bool(bool),
    None,
}
```

`Arc<str>` is preferred over `String` to reduce clone cost and avoid making every text clone allocate.

### 5.2 Dynamic tensor storage

Dynamic tensors may keep internal dynamic storage separate from numeric `Vec<f64>`, but public accessors must never expose the empty numeric field as real data.

Invariant:

```text
If Tensor::is_dynamic() == true:
  numeric accessors either reject or explicitly convert.
```

## 6. Data lifecycle

### 6.1 Ingestion lifecycle

```text
JSON/CSV/string/file input
  -> Result<Tensor, MattenError>
  -> dynamic storage
  -> inspection / cleanup
  -> try_numeric()
  -> numeric Tensor
```

### 6.2 Conversion lifecycle

`try_numeric` is the conversion gate. It should be explicit, visible in examples, and documented as the recommended transition from SUV-like ingestion to Sedan-like computation.

### 6.3 Failure lifecycle

If conversion fails:

```rust
Err(MattenError::Unsupported { operation: "try_numeric", message })
```

or a more specific variant from RFC-017 if accepted.

The error message must identify which element cannot be converted.

## 7. Events and observable behavior

Observable events:

- dynamic parse success/failure;
- missing value detection;
- cleanup transformation;
- numeric conversion success/failure;
- rejection of numeric-only API on dynamic tensor.

All failures at I/O boundaries return `Result`. Numeric API misuse may panic with clear unsupported-operation messages.

## 8. Store access

Dynamic store access should be explicit:

```rust
pub fn to_elements(&self) -> Vec<Element>;
```

Numeric store access should remain numeric-only:

```rust
pub fn as_slice(&self) -> &[f64]; // panics if dynamic
```

Do not add a public borrowed dynamic slice unless lifecycle and aliasing are designed carefully.

## 9. Public API proposal

### 9.1 Inspection helpers

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    pub fn is_dynamic(&self) -> bool;
    pub fn to_elements(&self) -> Vec<Element>;

    pub fn count_none(&self) -> usize;
    pub fn none_mask(&self) -> Tensor;
    pub fn numeric_mask(&self) -> Tensor;
    pub fn is_numeric_convertible(&self) -> bool;
}
```

`numeric_mask()` returns a numeric tensor with `1.0` for convertible elements and `0.0` otherwise.

### 9.2 Cleanup helpers

The cleanup API was stabilized in v0.13.2. RFC-016 **keeps the existing public
signatures** to avoid breaking churn (see the v0.13.3 kickoff-questions review):

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    // Existing generic API — covers both fill_none(0.0) and
    // fill_none(Element::text("x")) via Into<Element>. Kept as-is.
    pub fn fill_none(&self, value: impl Into<Element>) -> Tensor;

    // Existing fallback-based API. The fallback solves the leading-None
    // problem, so this stays infallible and keeps its current signature.
    pub fn forward_fill_none(&self, fallback: impl Into<Element>) -> Tensor;
}
```

`fill_none_with(&self, value: Element)` MAY be added later as a pure
discoverability alias delegating to `fill_none`, but only if user feedback shows
a discoverability problem. It is not required.

If a no-fallback fallible forward-fill is later desired, it MUST be added as a
**new** method rather than repurposing the existing one:

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    // Optional future addition — fails if the first element is None
    // (no previous value to carry forward). Not part of the v0.14 baseline.
    pub fn try_forward_fill_none(&self) -> Result<Tensor, MattenError>;
}
```

### 9.3 Conversion helpers

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    pub fn try_numeric(&self) -> Result<Tensor, MattenError>;
}
```

`try_numeric_with` is defined in RFC-017.

## 10. Cargo feature impact

All APIs in this RFC require:

```toml
features = ["dynamic"]
```

Dynamic JSON ingestion additionally requires `json`; dynamic CSV ingestion requires `csv`.

Because default features include `json` and `csv`, ordinary users can write:

```toml
matten = { version = "0.14", features = ["dynamic"] }
```

Feature-isolation CI must still test:

```bash
cargo test --no-default-features --features dynamic
cargo test --no-default-features --features dynamic,json
cargo test --no-default-features --features dynamic,csv
cargo test --no-default-features --features dynamic,json,csv
```

## 11. Internal design notes

### 11.1 Shared dynamic guard

All numeric-only methods should use a common guard:

```rust
#[cfg(feature = "dynamic")]
fn panic_if_dynamic(&self, operation: &'static str) {
    if self.is_dynamic() {
        panic!(
            "matten unsupported error in {operation}: \
             this numeric API is not supported on dynamic tensors; \
             call try_numeric() first"
        );
    }
}
```

### 11.2 Mask construction

Masks should return normal numeric tensors, not dynamic tensors.

Example:

```text
Element::Float(1.0) -> numeric_mask 1.0
Element::Int(2)     -> numeric_mask 1.0
Element::Bool(true) -> policy-dependent, see RFC-017
Element::Text("x")  -> numeric_mask 0.0 by default
Element::None       -> numeric_mask 0.0
```

If conversion policy is configurable, `numeric_mask_with(policy)` may be added later.

## 12. Examples

Required new or modified examples:

```text
examples/dynamic_01_mixed_elements.rs
examples/dynamic_02_missing_values.rs
examples/dynamic_03_fill_none.rs
examples/dynamic_04_numeric_coercion.rs
examples/dynamic_06_numeric_mask.rs
examples/dynamic_07_on_ramp_to_matmul.rs
```

`dynamic_07_on_ramp_to_matmul.rs` should show:

```rust
let raw = Tensor::from_csv_dynamic(csv)?;
let clean = raw.fill_none(0.0);
let numeric = clean.try_numeric()?;
let result = numeric.transpose().matmul(&numeric);
```

## 13. Acceptance criteria

- Dynamic docs describe on-ramp, not full computation engine.
- Dynamic numeric APIs reject rather than silently compute.
- `numeric_mask` and `is_numeric_convertible` are implemented or explicitly deferred.
- `try_numeric` remains the recommended computation boundary.
- Dynamic examples demonstrate cleanup-to-numeric flow.
- Feature-isolation CI includes `dynamic,json` and `dynamic,csv`.

## 14. QA checklist

- [ ] Dynamic ingestion tests
- [ ] Dynamic missing-value tests
- [ ] Dynamic numeric guard tests
- [ ] `try_numeric` success/failure tests
- [ ] Feature matrix tests
- [ ] Dynamic examples compile and run
- [ ] README wording checked for overclaims

## 15. Open questions

1. ~~Should `numeric_mask()` treat `Bool` as convertible by default?~~
   **Resolved (v0.13.3 kickoff review):** No. `numeric_mask` mirrors
   `try_numeric`'s strict default, where only `Float`/`Int` are convertible, so
   `Bool` maps to `0.0`. A future `numeric_mask_with(policy)` may relax this once
   `NumericPolicy` (RFC-017) lands.
2. Should `Text("123")` ever be convertible by default? (Default: no — requires
   explicit `allow_text_parse` policy under RFC-017.)
3. Should dynamic cleanup methods preserve all original element variants, or normalize numeric values eagerly?
