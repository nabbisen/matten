# RFC-021: Tutorial Path and Example Quality Gate

**Status:** Proposed  
**Target:** v0.15.x  
**Theme:** Executable documentation maturity  
**Depends on:** RFC-014, RFC-016, RFC-019, RFC-020  
**Related handoff:** `021-tutorial-path-and-example-quality-gate-handoff.md`

## 1. Summary

This RFC matures `matten` examples and tutorials into a coherent learning path.

`matten` is a DX-first library. For this type of crate, examples are part of the product. Users should be able to copy a small example, run it, and understand how to adapt it.

## 2. Goals

- Define a stable tutorial path.
- Keep examples scoped to accepted APIs.
- Group examples by user intent.
- Add examples for dynamic on-ramp and axis reductions.
- Prevent examples from turning into hidden feature requests.
- Ensure examples compile in CI.

## 3. Non-goals

- No large notebooks.
- No benchmark marketing.
- No dataframe tutorial.
- No ML training tutorial.
- No external bridge tutorials until bridge crates exist.
- No huge fixture data.

## 4. External design

### 4.1 Tutorial path

Recommended mdBook path:

```text
1. Quickstart
2. Shapes and construction
3. Formula-like numeric operations
4. Broadcasting
5. Slicing
6. Reductions and matmul
7. JSON/CSV boundary APIs
8. Dynamic ingestion on-ramp
9. When to use companion crates
```

### 4.2 Example classes

| Class | Prefix | Purpose |
|---|---|---|
| Core | `00_` to `14_` | Basic tensor usage |
| Math | `20_` to `29_` | Numeric computing |
| Pattern | `30_` to `49_` | Small PoC workflows |
| Dynamic | `dynamic_` | Messy-data on-ramp |
| Future companion | not in core examples | Avoid broken examples |

## 5. Data model

No data model change.

Examples should teach:

- numeric tensor lifecycle;
- dynamic-to-numeric lifecycle;
- error boundary lifecycle.

## 6. Data lifecycle

Examples should align with these lifecycles:

### Numeric

```text
construct -> inspect -> compute -> print/serialize
```

### Boundary

```text
load/parse -> Result handling -> compute
```

### Dynamic

```text
parse dynamic -> inspect -> clean -> try_numeric -> compute
```

## 7. Events and observable behavior

Examples are executable observable behavior.

Breaking an example is treated as a release-quality regression.

## 8. Store access

Examples may use `as_slice()` for small PoC math patterns, but should explain that dynamic tensors require `to_elements()` or `try_numeric()`.

## 9. Public API requirements

Examples may not introduce unaccepted APIs.

Any proposed example requiring new API must be:

1. marked future; or
2. blocked until the relevant RFC is accepted.

## 10. Cargo feature impact

Example CI must include:

```bash
cargo check --examples
cargo check --examples --all-features
cargo check --examples --no-default-features --features dynamic,json
cargo check --examples --no-default-features --features dynamic,csv
```

Dynamic examples must include explicit feature instructions.

## 11. Internal design

### 11.1 Example header standard

Each example should start with:

```rust
//! What this example teaches.
//!
//! Run:
//! cargo run --example 00_quickstart
//!
//! Notes:
//! This example is intended for small PoC workloads.
```

### 11.2 Fixture policy

Fixtures must be:

- small;
- committed;
- human-readable;
- stable;
- not domain-heavy.

Directory:

```text
examples/data/
  numeric_2x3.csv
  tensor_2x2.json
  messy_business_rows.csv
```

## 12. Examples

Required additions or updates:

```text
examples/27_axis_reductions.rs
examples/28_column_mean.rs
examples/29_row_scores.rs
examples/dynamic_06_numeric_mask.rs
examples/dynamic_07_on_ramp_to_matmul.rs
examples/14_readable_errors.rs
```

Tutorial docs should reference existing examples rather than duplicate all code.

## 13. Acceptance criteria

- Every example compiles.
- Core examples are small and focused.
- Dynamic examples show on-ramp, not full dynamic computation.
- Axis reduction examples are added if RFC-019 lands.
- Error example is added if RFC-020 lands.
- README has a clear “start here” path.

## 14. QA checklist

- [ ] `cargo check --examples`
- [ ] `cargo check --examples --all-features`
- [ ] Dynamic example checks
- [ ] Example data fixtures reviewed
- [ ] Example headers consistent
- [ ] README tutorial path updated
- [ ] mdBook links checked

## 15. Open questions

1. Should examples be tested with expected stdout?
2. Should examples be grouped in directories with explicit `[[example]]`, or remain flat for Cargo discovery?
3. Should there be a separate `tutorials/` directory in addition to mdBook?
