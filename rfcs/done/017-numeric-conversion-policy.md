# RFC-017: Numeric Conversion Policy

**Status:** Implemented (v0.14.0)  
**Target:** v0.14.x  
**Theme:** Dynamic-to-numeric conversion  
**Depends on:** RFC-011, RFC-016  
**Related handoff:** `017-numeric-conversion-policy-handoff.md`

## 1. Summary

This RFC defines explicit conversion policies for turning dynamic `Element` values into numeric `f64` tensors.

The conversion boundary is central to `matten`'s balanced design. Dynamic tensors are allowed to ingest messy data, but mathematical computation should happen on numeric tensors. Therefore, the conversion from dynamic to numeric must be explicit, configurable enough for real-world data, and never silently surprising.

## 2. Goals

- Make `try_numeric()` behavior precise.
- Avoid silent and surprising mixed-type coercion.
- Provide a path for strict and permissive conversion.
- Document integer precision behavior.
- Keep the default conversion simple and safe.
- Enable future dynamic utilities without adding dynamic arithmetic.

## 3. Non-goals

- No dynamic arithmetic.
- No automatic conversion inside math operators.
- No locale-aware text parsing.
- No date/time parsing.
- No decimal or arbitrary precision numeric support.
- No column-specific schema policy in core `matten`.

## 4. External design

### 4.1 Default conversion

Default `try_numeric()` should be conservative:

| Element | Default conversion |
|---|---|
| `Float(f)` | `f` |
| `Int(i)` | `i as f64`, documented precision caveat |
| `Bool(_)` | error by default |
| `Text(_)` | error by default |
| `None` | error by default |

Users should call cleanup methods first:

```rust
let clean = raw.fill_none(0.0);
let numeric = clean.try_numeric()?;
```

### 4.2 Explicit policy conversion

Introduce:

```rust
#[cfg(feature = "dynamic")]
pub struct NumericPolicy { ... }

#[cfg(feature = "dynamic")]
impl Tensor {
    pub fn try_numeric_with(&self, policy: NumericPolicy) -> Result<Tensor, MattenError>;
}
```

Example:

```rust
let x = raw.try_numeric_with(
    NumericPolicy::strict()
        .allow_bool()
        .none_as(0.0)
)?;
```

## 5. Data model

### 5.1 NumericPolicy

Suggested public model:

```rust
#[cfg(feature = "dynamic")]
#[derive(Debug, Clone)]
pub struct NumericPolicy {
    bool_mode: BoolConversion,
    none_mode: NoneConversion,
    text_mode: TextConversion,
    int_mode: IntConversion,
}

#[cfg(feature = "dynamic")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolConversion {
    Reject,
    ZeroOne,
}

#[cfg(feature = "dynamic")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoneConversion {
    Reject,
    Fill(f64),
}

#[cfg(feature = "dynamic")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextConversion {
    Reject,
    ParseAsciiFloat,
}

#[cfg(feature = "dynamic")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntConversion {
    CastToF64,
    RejectUnsafeLarge,
}
```

This is the maximal proposed model. Implementation may start smaller if the API remains extensible.

### 5.2 Error model

Conversion failures should return `MattenError::Unsupported` or a new error variant only if necessary.

Recommended minimal form:

```rust
MattenError::Unsupported {
    operation: "try_numeric",
    message: format!("element at flat index {i} is {element:?} and cannot be converted to f64"),
}
```

No new `MattenError` variant is required for v0.14 unless error matching becomes too coarse.

## 6. Data lifecycle

```text
dynamic Tensor
  -> inspect conversion readiness
  -> optional fill/cleanup
  -> try_numeric_with(policy)
  -> numeric Tensor
```

The resulting numeric tensor must be a normal Sedan tensor with:

- numeric `Vec<f64>`;
- same shape;
- no dynamic sidecar;
- standard numeric API availability.

## 7. Events and observable behavior

Conversion events:

- conversion accepted;
- conversion rejected at a specific index;
- integer precision caveat applied;
- missing value filled;
- boolean coerced;
- text parsed or rejected.

All conversion failures return `Result::Err`.

## 8. Store access

Conversion reads dynamic storage and writes new numeric storage.

It must not mutate the original dynamic tensor.

Expected ownership model:

```text
input dynamic Tensor: unchanged
output numeric Tensor: newly allocated Vec<f64>
```

## 9. Public API proposal

### 9.1 Minimal v0.14 API

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    pub fn try_numeric(&self) -> Result<Tensor, MattenError>;
    pub fn try_numeric_with(&self, policy: NumericPolicy) -> Result<Tensor, MattenError>;
}

#[cfg(feature = "dynamic")]
impl NumericPolicy {
    pub fn strict() -> Self;
    pub fn allow_bool_as_zero_one(self) -> Self;
    pub fn none_as(self, value: f64) -> Self;
    pub fn parse_ascii_float_text(self) -> Self;
    pub fn reject_large_int_precision_loss(self) -> Self;
}
```

### 9.2 Deferred API

Column-specific policies are deferred because `matten` core does not have column names:

```rust
// not in matten core
policy.for_column("price").parse_text_float()
```

This belongs in a future `matten-data` crate.

## 10. Cargo feature impact

All items require:

```toml
features = ["dynamic"]
```

No new cargo feature is introduced.

## 11. Internal design

### 11.1 Conversion loop

The conversion loop should preallocate exactly `self.len()` numeric elements.

Pseudo-code:

```rust
let mut out = Vec::with_capacity(self.len());
for (i, elem) in elements.iter().enumerate() {
    out.push(policy.convert(i, elem)?);
}
Tensor::try_new(out, self.shape())
```

### 11.2 Integer precision

`i64 as f64` can lose precision for values outside the exact integer range of IEEE 754 double precision.

Default should allow this but document it.

`reject_large_int_precision_loss` should reject values where roundtrip is not exact:

```rust
let f = i as f64;
if f as i64 != i {
    return Err(...);
}
```

This test has edge cases near `i64::MIN` / `MAX`; implementation should use careful helper tests.

### 11.3 Text parsing

If text parsing is enabled, support only ASCII/standard Rust float parsing:

```rust
str::parse::<f64>()
```

No locale-specific comma decimals, currency strings, dates, percentages, or whitespace-heavy heuristics in core.

## 12. Examples

Required examples:

```text
examples/dynamic_04_numeric_coercion.rs
examples/dynamic_08_numeric_policy_strict.rs
examples/dynamic_09_numeric_policy_permissive.rs
```

The permissive example should be explicit and readable:

```rust
let policy = NumericPolicy::strict()
    .allow_bool()
    .none_as(0.0);

let numeric = raw.try_numeric_with(policy)?;
```

## 13. Acceptance criteria

- Default `try_numeric()` behavior is documented and tested.
- `NumericPolicy` exists or the RFC is explicitly split before implementation.
- Bool/Text/None behavior is not silent.
- Large integer precision behavior is documented and tested.
- `try_numeric_with` preserves shape.
- Conversion does not mutate the original dynamic tensor.

## 14. QA checklist

- [ ] Float conversion test
- [ ] Int conversion test
- [ ] Large int precision caveat test
- [ ] Bool rejection test
- [ ] Bool zero/one policy test
- [ ] None rejection test
- [ ] None fill policy test
- [ ] Text rejection test
- [ ] Optional ASCII float text parse test
- [ ] Shape preservation test
- [ ] Example smoke tests

## 15. Open questions

1. Is `TextConversion::ParseAsciiFloat` too permissive for core?
2. Should the first version expose policy enums, or only builder methods?
3. Should `NumericPolicy` be `#[non_exhaustive]`?

## Implementation notes

The implemented API in v0.14.0:

```rust
NumericPolicy::strict()        // default
NumericPolicy::permissive()    // all variants
.allow_bool()                  // true→1.0, false→0.0
.allow_text_parse()            // parse &str as f64
.none_as(value)               // treat None as value
.none_as_nan()                 // treat None as NaN
```

The `reject_large_int_precision_loss` method was not implemented. Large `Int(i64)` values are documented in `Element::try_as_f64` as using `as f64` semantics with possible precision loss.
