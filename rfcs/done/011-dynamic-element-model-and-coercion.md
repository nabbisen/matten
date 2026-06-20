# RFC-011: Dynamic `Element` Model and Coercion

> RFC status: Implemented (0.8.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines the external semantic model for the `dynamic` feature: `Element` values, missing-value handling, type coercion, and mixed-data boundary behavior. It intentionally does not hard-code the internal text representation until memory measurements are complete. `Element::Text` externally means UTF-8 text, but the payload may be `String`, `Box<str>`, `Arc<str>`, interned text, or another approved representation.

## 2. Motivation

The `dynamic` feature is the Phase 2 SUV engine for messy business data: JSON, CSV, nullable fields, booleans, text labels, and mixed numeric values. This is useful, but it risks memory bloat and silent coercion bugs. The RFC must define semantics before implementation locks in a poor enum layout.

## 3. Goals

- Define external `Element` variants.
- Define missing-value semantics.
- Define numeric coercion policy.
- Define text memory decision criteria.
- Define mixed JSON/CSV behavior.
- Preserve default Phase 1 performance and API simplicity.

## 4. Non-goals

- No dataframe abstraction.
- No schema inference framework.
- No categorical dtype system in `0.2.0`.
- No date/time dtype.
- No SQL null semantics beyond `Element::None`.
- No silent text-to-number guessing by default.

## 5. Cargo Features

```toml
matten = {{ version = "0.15", features = ["dynamic"] }}
```

`dynamic` adds:

```rust
pub enum Element { /* ... */ }
```

and dynamic-specific helpers. It must not make default `use matten::Tensor` examples require `Element` imports.

## 6. Data Model

External semantic model:

```rust
#[cfg(feature = "dynamic")]
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Float(f64),
    Int(i64),
    Text(TextRepr),
    Bool(bool),
    None,
}
```

`TextRepr` is a placeholder in this RFC text. Public API may expose it as `String` initially only if RFC-011 acceptance includes memory measurements and maintainer approval.

Recommended public constructors/accessors:

```rust
impl Element {
    pub fn is_none(&self) -> bool;
    pub fn is_numeric(&self) -> bool;
    pub fn try_as_f64(&self) -> Option<f64>;
    pub fn as_f64(&self) -> f64; // panic wrapper, optional
    pub fn as_text(&self) -> Option<&str>;
}
```

## 7. Data Lifecycle

Dynamic value lifecycle:

1. External source produces JSON/CSV/native `Element` values.
2. Boundary parser maps values to `Element`.
3. Tensor stores elements using RFC-012 storage.
4. Numeric operations request coercion.
5. Coercible values participate; non-coercible values return error or panic depending on zone.
6. Missing-value helpers transform `None` into replacement values.

## 8. Events

| Event | Required behavior |
|---|---|
| JSON null parsed | map to `Element::None` |
| JSON number parsed | choose `Int` or `Float` by representation/range policy |
| CSV empty field parsed | map to `None` only under dynamic if approved |
| numeric coercion requested | `Int` and `Float` allowed; `Bool` not numeric by default |
| non-coercible arithmetic | error or panic according to API zone |
| missing fill requested | produce new tensor or CoW materialized tensor |

No public event bus.

## 9. Store Access

`Element` values live in tensor storage governed by RFC-012. Text representation must be chosen with store footprint in mind.

Candidate text representations:

| Representation | Pros | Cons |
|---|---|---|
| `String` | simple, standard | enum size may be large; clones allocate |
| `Box<str>` | smaller ownership semantics, immutable | still pointer-sized payload, clones allocate |
| `Arc<str>` | cheap clones, good for repeated view sharing | atomic overhead, still per-value allocation |
| interning | compact for repeated strings | adds store complexity and lifecycle questions |
| `smol_str` | optimized small strings | adds dependency and type semantics |

This RFC requires measuring `std::mem::size_of::<Element>()` and representative allocations before implementation lock.

## 10. Public API

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    pub fn from_elements(data: Vec<Element>, shape: &[usize]) -> Tensor;
    pub fn try_from_elements(data: Vec<Element>, shape: &[usize]) -> Result<Tensor, MattenError>;

    pub fn fill_none(&self, value: impl Into<Element>) -> Tensor;
    pub fn is_none(&self) -> Tensor; // returns bool-like dynamic tensor, if approved

    pub fn try_numeric(&self) -> Result<Tensor, MattenError>;
}
```

The exact return type of `is_none` may be deferred. A simple first implementation may return `Vec<bool>` or a dynamic tensor of booleans.

## 11. Coercion Policy

### 11.1 Numeric coercion

Allowed by default:

- `Float(f64)` -> itself;
- `Int(i64)` -> `f64` when numeric tensor operation requires `f64`.

Not allowed silently:

- `Text("3.14")` -> `3.14`;
- `Bool(true)` -> `1.0`;
- `None` -> `0.0`.

Users must explicitly call cleaning methods such as `fill_none` or future conversion helpers.

### 11.2 Missing values

`Element::None` represents missing/null. It is not equal to numeric zero and is not automatically ignored in arithmetic unless a later RFC defines skip-missing reductions.

### 11.3 JSON numbers

JSON integer-looking numbers may map to `Int(i64)` if exactly representable and within range; otherwise map to `Float(f64)`. This policy must be tested.

## 12. Internal Design

### 12.1 Memory measurement gate

Before accepting implementation:

```rust
println!("Element size = {{}}", std::mem::size_of::<Element>());
```

Measure:

- all-float dynamic tensor;
- mixed numeric/text tensor;
- repeated short text values;
- many null values;
- clone and slice workloads once RFC-012 storage exists.

### 12.2 Avoid contaminating Phase 1

Dynamic code should live under `src/dynamic/` or equivalent and be `#[cfg(feature = "dynamic")]`. Default Phase 1 builds must not compile dynamic parser/coercion code unless needed.

## 13. Error Handling

Boundary parsing returns `Result`.

Internal dynamic arithmetic may panic if called through operator traits and values are non-coercible, but messages must include value categories and operation.

Recommended recoverable API:

```rust
pub fn try_add(&self, rhs: &Tensor) -> Result<Tensor, MattenError>;
```

This can be added in Phase 2 if operator panics are too risky for mixed data.

## 14. Testing

- size and memory measurement tests;
- JSON null/string/bool/int/float mapping;
- CSV empty field policy;
- numeric coercion tests;
- non-coercible arithmetic tests;
- `fill_none` tests;
- default build does not expose `Element`;
- dynamic build keeps numeric examples working.

## 15. Acceptance Criteria

- `Element` external semantics are accepted.
- Text representation is selected only after measurement.
- No silent text-to-number or bool-to-number coercion.
- Missing values are explicit.
- Dynamic feature remains feature-gated.
- Phase 1 public API remains simple and unaffected.
