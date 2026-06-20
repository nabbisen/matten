# RFC-001: Threat Model and Boundary Safety Policy

> RFC status: Proposed  
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines the threat model for `matten` and establishes the library-wide boundary-safety policy. `matten` is not a cryptographic or authentication crate, but it is expected to run inside web APIs, file-processing jobs, data-cleaning scripts, and business PoCs. Therefore the most important risks are availability loss, memory explosion, parser abuse, panic propagation, and silent numerical or data corruption.

This RFC is the parent policy for later RFCs on construction, slicing, serde/CSV parsing, broadcasting, dynamic `Element`, and CoW storage.

## 2. Motivation

The `matten` philosophy intentionally accepts panic-driven feedback for local PoC development. That is useful when a developer writes `&a + &b` and immediately wants a clear shape-mismatch message. However, the same behavior is dangerous when input comes from JSON, CSV, HTTP requests, uploaded files, or database rows. Without an explicit threat model, implementers may accidentally allow malformed user input to panic the process, allocate unbounded memory, or silently coerce data incorrectly.

The goal is not to make every API fully defensive. The goal is to draw a crisp line between:

- **panic-zone APIs**, intended for trusted local computation; and
- **Result-zone APIs**, intended for untrusted or externally supplied input.

## 3. Goals

- Define what `matten` protects and does not protect.
- Define trust boundaries for input, shape, parser, and feature-gated behavior.
- Require checked shape-product calculations.
- Prevent silent integer overflow during allocation-size calculation.
- Require parser-facing APIs to return `Result`.
- Require documentation to label panic-zone and Result-zone APIs.
- Establish fuzz/property-test expectations for parser and shape logic.
- Keep the policy lightweight enough for a small DX-first crate.

## 4. Non-goals

- No cryptographic confidentiality guarantee.
- No side-channel resistance.
- No sandboxing of untrusted native code.
- No hard real-time memory guarantee.
- No guarantee that intentionally huge valid computations are cheap.
- No distributed-computation threat model.

## 5. Cargo Features

This RFC applies to all features.

| Feature | Requirement impact |
|---|---|
| default | Must apply shape, allocation, panic, and Result-zone policy. |
| `serde` | Must apply Result-zone policy to serialization/deserialization helpers. |
| `csv` if introduced | Must apply Result-zone policy to CSV parsing and loading. |
| `dynamic` | Must apply memory-layout and coercion safety policy for `Element`. |
| future parser features | Must return `Result` and be fuzz-testable. |

The default build must not require heavy security or parser dependencies unless they are directly used by default APIs.

## 6. Data Model

This RFC does not introduce new tensor data fields. It introduces policy concepts:

```rust
pub struct Tensor { /* defined by later RFCs */ }

pub enum MattenError {
    ShapeError { /* ... */ },
    AllocationError { /* ... */ },
    ParseError { /* ... */ },
    CsvError { /* ... */ },
    JsonError { /* ... */ },
    SliceError { /* ... */ },
    FeatureError { /* ... */ },
}
```

The exact `MattenError` layout is refined in RFC-005, but the category list is normative for this threat model.

## 7. Data Lifecycle

A tensor lifecycle may cross several trust states:

1. **External input**: JSON, CSV, file, DB row, HTTP payload, or slice string.
2. **Boundary validation**: parser, shape check, raggedness check, allocation budget check.
3. **Materialized tensor**: internally trusted invariant-bearing value.
4. **Internal computation**: arithmetic, broadcasting, reshape, axis operations.
5. **External output**: serde, file write, conversion to standard vectors.

All transitions from state 1 to state 3 must return `Result`. Internal transitions from state 3 to state 4 may panic when the caller violates documented preconditions, provided that panic messages are actionable.

## 8. Events

`matten` does not define a public event bus in this RFC. However, implementers must treat the following as lifecycle events for testing and logging design:

| Event | Publicly emitted? | Required handling |
|---|---:|---|
| Shape product calculated | No | Must use checked multiplication. |
| Allocation requested | No | Must validate size before allocation where practical. |
| Boundary parse begins | No | Must return `Result` on malformed input. |
| Boundary parse fails | No | Error should include actionable context. |
| Internal panic occurs | Panic only | Message must include operation and relevant shapes/indices. |
| Dynamic CoW materializes | No in Phase 2 | Must be testable via internal counters or debug hooks if approved. |

A future diagnostics feature may expose internal events, but this RFC does not require it.

## 9. Store Access

`matten` is primarily an in-memory library. It does not own a database or persistent store.

Store-like access appears in three places:

1. **In-memory tensor storage**: `Vec<f64>` in Phase 1, and RFC-defined storage in Phase 2.
2. **File loading APIs**: `load_json`, `load_csv`, or similar helpers.
3. **Serde/DB integration**: caller-owned storage where `matten` serializes/deserializes values.

All file and external storage access must be Result-zone. Direct in-memory operations may be panic-zone if they are clearly documented as local computation APIs.

## 10. Public API Policy

### 10.1 Panic-zone examples

```rust
let a = Tensor::ones(&[2, 3]);
let b = Tensor::ones(&[2]);
let c = &a + &b; // MAY panic if shapes are incompatible.
```

Panic-zone APIs must never panic due to internal invalid state if the tensor was constructed through public APIs.

### 10.2 Result-zone examples

```rust
let t = Tensor::from_json(input)?;
let s = t.slice_str(user_slice)?;
let r = Tensor::try_new(user_data, &user_shape)?;
```

Result-zone APIs must not use ordinary parser panics as error handling.

## 11. Internal Design Requirements

### 11.1 Checked shape product

All shape product calculations must use checked multiplication:

```rust
fn checked_len(shape: &[usize]) -> Result<usize, MattenError> {
    shape.iter().try_fold(1usize, |acc, &dim| {
        acc.checked_mul(dim).ok_or_else(/* Shape overflow error */)
    })
}
```

Convenience panic-zone constructors may call a checked helper and convert an error into a panic with an actionable message.

### 11.2 Allocation budgets

Boundary APIs must support rejecting obviously excessive allocations. The exact default budget is left open for RFC-005 or implementation policy, but the mechanism must exist before exposing boundary APIs intended for untrusted input.

Recommended initial internal representation:

```rust
#[derive(Debug, Clone)]
pub(crate) struct Limits {
    pub max_rank: usize,
    pub max_elements_from_boundary: usize,
    pub max_slice_spec_len: usize,
    pub max_json_depth: usize,
}
```

These limits need not be public in `0.1.0`, but tests must be able to exercise them.

### 11.3 Unsafe policy

Phase 1 should use safe Rust only and should include:

```rust
#![forbid(unsafe_code)]
```

If any future RFC proposes `unsafe`, it must document:

- why safe Rust is insufficient;
- exact invariants;
- tests that cover the unsafe boundary;
- why the unsafe code does not leak into public API obligations.

## 12. Error and Panic Requirements

A panic message must include:

- library prefix: `matten`;
- operation name;
- received shape/index/input category;
- expected shape/range/condition;
- concise remediation hint when useful.

Example:

```text
matten broadcast error in add: shapes [2, 3] and [2] are not compatible; align trailing dimensions or reshape the right operand to [1, 2]
```

Boundary errors must not rely on panic text. They must be structured enough for applications to display or log them.

## 13. Testing Requirements

- Unit tests for checked shape products and overflow.
- Property tests for product validation and broadcasting compatibility.
- Fuzz tests for `slice_str` once implemented.
- Malformed JSON/CSV tests must assert `Err`, not panic.
- Panic-message tests should cover at least common shape mismatch cases.
- Phase 2 memory tests must measure selected `Element` representation before lock-in.

## 14. Acceptance Criteria

This RFC is accepted when:

- maintainers agree that `matten` is availability/correctness focused, not cryptographic;
- panic-zone and Result-zone are accepted as public concepts;
- all boundary RFCs reference this RFC;
- checked shape product is required in implementation;
- parser-facing APIs are required to return `Result`;
- Phase 1 safe-Rust-only policy is accepted unless superseded by a later RFC.

## 15. Open Questions

1. Should boundary limits be configurable by users in `0.1.0`, or internal only?
2. Should `try_new` be included in `0.1.0`, or can external users rely on `from_json`/`from_csv` first?
3. Should `MattenError` expose machine-readable error codes from the beginning?
