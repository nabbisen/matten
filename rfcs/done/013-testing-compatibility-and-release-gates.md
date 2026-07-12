# RFC-013: Testing, Compatibility, and Release Gates

> RFC status: Implemented (0.6.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines the testing strategy, feature-compatibility rules, and release gates for `matten`. It turns the earlier requirements and RFCs into a verifiable engineering discipline: unit tests, property tests, fuzz tests, golden tests against NumPy where useful, doc tests, feature-matrix builds, and release checklists.

## 2. Motivation

`matten` intentionally prioritizes developer experience over raw performance. That makes correctness and communication even more important. If broadcasting, slicing, or dynamic coercion silently produces wrong results, users lose trust. A small crate can remain reliable by using focused tests and explicit release gates.

## 3. Goals

- Define test categories by feature area.
- Require default and feature builds in CI.
- Define compatibility expectations across `dynamic` and default builds.
- Require documentation examples to compile.
- Define release gates for alpha, beta, RC, and public release.
- Prevent scope creep through review checklists.

## 4. Non-goals

- No full benchmarking framework requirement for `0.1.0`.
- No strict performance target beyond avoiding obvious exponential behavior.
- No long-term MSRV promise unless a later RFC sets it. The Rust 2024 edition implies a practical floor of Rust 1.85, so set `rust-version = "1.85"` in `Cargo.toml`.
- No formal verification requirement.

## 5. Cargo Features

CI must test:

```bash
cargo test                          # default PoC profile (serde + json + csv)
cargo test --no-default-features    # lean core profile; strict compile-time baseline
cargo test --no-default-features --features serde
cargo test --no-default-features --features json
cargo test --no-default-features --features csv
cargo test --no-default-features --features dynamic
cargo test --all-features
cargo test --doc
```

Because `json` and `csv` are separate locked features, they must remain in the CI matrix. Any future feature that exposes public APIs must be added to the same matrix.

## 6. Data Model

This RFC introduces no runtime data model. It defines test artifacts:

```text
tests/
  construction.rs
  shape.rs
  broadcasting.rs
  slicing.rs
  serde_json.rs
  csv.rs
  reductions.rs
  matmul.rs
  dynamic_element.rs
  dynamic_cow.rs
  golden_numpy/
```

Fuzz tests may live under:

```text
fuzz/
  fuzz_targets/slice_str.rs
  fuzz_targets/json_nested.rs
```

## 7. Data Lifecycle Under Test

Each major lifecycle must be tested:

1. construction;
2. validation failure;
3. transformation;
4. computation;
5. boundary parse;
6. serialization;
7. feature-gated dynamic behavior;
8. export/migration.

For every lifecycle, tests should include at least one success and one failure case.

## 8. Events

Testing should verify conceptual events from earlier RFCs:

| Event | Test expectation |
|---|---|
| shape overflow detected | no wrapped allocation |
| boundary parser fails | returns `Err`, not panic |
| broadcast mismatch | panic message includes both shapes |
| slice parser fails | error includes original spec |
| dynamic coercion fails | no silent conversion |
| CoW materializes | shared tensor remains unchanged |

No public event bus is introduced.

## 9. Store Access

File tests must use temporary files and should not depend on external network access. No test should require a database.

Golden test data should be small, checked into the repository, and human-readable.

## 10. Public API Test Requirements

All public examples in README and rustdoc must compile.

Recommended doctest example style:

```rust
use matten::Tensor;

let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let b = Tensor::ones(&[2, 2]);
let c = &a + &b;
assert_eq!(c.shape(), &[2, 2]);
```

No public doc example should import `matten::internal` or require explicit lifetimes.

## 11. Internal Design

### 11.1 Property tests

Use property tests for:

- shape product round trips;
- flatten/reshape element preservation;
- broadcasting compatibility symmetry where applicable;
- coordinate flatten/unflatten round trips;
- slicing builder vs `slice_str` equivalence.

### 11.2 Golden tests against NumPy

Golden tests should be generated from NumPy for a small selected set:

- broadcasting cases;
- transpose/swap axes;
- slicing;
- reductions;
- matmul.

Golden files should store inputs and expected outputs as JSON, not require Python at test time.

### 11.3 Fuzz tests

Fuzzing targets:

- `slice_str` parser;
- nested JSON shape inference;
- CSV row parsing if custom parser is used.

Fuzz tests are not required in every CI run but should be runnable locally and in scheduled CI.

## 12. Compatibility Policy

### 12.1 Default vs dynamic

Phase 1 examples should compile under `dynamic` unless a specific RFC documents a difference.

### 12.2 v0.x changes

Breaking changes are allowed during `0.x` but must be documented. Public API churn should decrease after `0.1.0`.

### 12.3 Feature-gated API additions

Feature-gated methods may be added, but disabling a feature must not break ordinary default examples.

## 13. Release Gates

### Alpha

- core APIs compile;
- unit tests pass for implemented scope;
- docs may be incomplete but must not misrepresent behavior.

### Beta

- public API mostly settled for target release;
- README quickstart works;
- boundary APIs return `Result`;
- known limitations documented.

### RC

- all doc tests pass;
- feature matrix passes;
- panic/error message review completed;
- release notes drafted;
- no known critical correctness bugs.

### Public release

- user confirmation for v1 releases is required by project policy;
- for `0.1.0`, maintainers must confirm non-goals and limitations are clear.

## 14. Acceptance Criteria

- CI matrix is defined.
- Every earlier RFC has test obligations mapped.
- Boundary parser failures are tested as `Err`.
- Broadcasting and slicing have golden tests.
- Dynamic memory/coercion tests are required before Phase 2 beta.
- Release checklist prevents scope creep and performance-first drift.

---

## Lifecycle note (pre-v0.19.0 audit, 2026-06-23)

RFC-013 established the *intent* for systematic testing, including property tests, fuzz
tests, and golden checks. The discipline actually implemented and enforced as release
gates relies on unit/integration tests, golden/reference checks, example smoke runs,
feature-matrix sweeps, the dependency-boundary guard, and the release-docs guard
(374 `#[test]` functions across the workspace at v0.20.x).

Property-based tests and a fuzz harness are **not** part of the current release gates and
their absence is not a defect — they remain *future hardening candidates*, to be added
selectively (shape/broadcast/reduction invariants for property tests; CSV/JSON/parser
boundaries for fuzzing) rather than broadly. A future focused item ("Testing Strategy
Refresh: Property Tests and Fuzz Boundary", to be assigned the next free RFC number when it
lands — it holds no fixed reservation, having previously floated through RFC-050, RFC-055, and
RFC-057 earmarks as each was taken by the migration set, the v0.24 reduction set, and the
`matten-ndarray` promotion respectively) may formalize this if the team chooses. This note records that
RFC-013's broad strategy is partially aspirational relative to the shipped discipline.

## Docs-governance coverage note (2026-07-12)

The current shipped NumPy golden coverage is checked in under:

```text
crates/matten/tests/golden/numpy_broadcasting.json
crates/matten/tests/golden/numpy_matmul.json
crates/matten/tests/smoke.rs
```

These tests cover broadcasting and matrix multiplication without requiring Python at test time.
Property-based tests and fuzz targets remain future hardening candidates, not current release
gates. The durable resolution record for this inventory is
`docs/design/coverage-gap-resolution.md`.
