# RFC-014: Example Suite and Executable Documentation

> RFC status: Implemented (0.6.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers, documentation owners  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines the example suite for `matten` as executable documentation and part of the public Developer Experience (DX) contract.

`matten` is a DX-first mathematical computing crate for rapid Rust PoCs. For this project, examples are not decorative. They are the primary way many users will learn the crate, verify the mental model, and decide whether `matten` is easier to start with than lower-level or more specialized libraries.

This RFC therefore defines:

- the required example directory layout;
- which examples are required before `0.1.0`;
- which examples are recommended for `0.1.x`;
- which examples are required before `0.2.0` with `dynamic`;
- what example categories are future themes or out of current scope;
- fixture-data rules;
- cargo-feature rules;
- CI/release gates;
- review rules to prevent examples from silently expanding the library scope.

## 2. Motivation

The `matten` concept is a "Family Car" for multidimensional mathematical computing in Rust. It intentionally prioritizes ease of use, readability, low type complexity, and PoC velocity over benchmark-oriented performance.

For that concept to work, examples must show practical starting points:

- how to create tensors;
- how shapes behave;
- how element-wise arithmetic works;
- how broadcasting works;
- how reshaping and slicing work;
- how basic mathematical computing works;
- how boundary-safe JSON/CSV usage works.

However, an example suite can also become dangerous. If examples include advanced linear algebra, dataframe-like workflows, ML training, database integrations, GPU acceleration, or large benchmarks too early, users and developers may misunderstand `matten` as a replacement for `nalgebra`, `ndarray`, `polars`, `candle`, or BLAS-backed numerical stacks.

This RFC keeps examples useful while keeping the project simple and clean.

## 3. Goals

- Treat examples as executable documentation.
- Provide a small but solid `0.1.0` example suite.
- Demonstrate the accepted public API without adding hidden API requirements.
- Make mathematical computing patterns discoverable.
- Demonstrate boundary-safe `Result` usage for external input.
- Separate default examples, dynamic examples, and future integration examples.
- Ensure examples compile in CI.
- Keep examples aligned with the `matten` philosophy and non-goals.

## 4. Non-goals

- No full NumPy replacement examples.
- No pandas/dataframe replacement examples.
- No advanced linear algebra examples such as inverse, determinant, eigenvalues, SVD, QR, LU, or Cholesky.
- No automatic differentiation examples.
- No neural network training examples.
- No GPU examples.
- No sparse matrix examples.
- No large-scale benchmark examples.
- No SQL/database integration examples for `0.1.0`.
- No example that requires public generics, public lifetimes, storage internals, or a `Tensor<T>` API.
- No example may define a new public API requirement by itself.

## 5. Relationship to Other RFCs

RFC-014 depends on the following accepted or proposed RFC areas:

| Dependency | Why it matters |
|---|---|
| RFC-001 Threat Model and Boundary Safety Policy | Examples must demonstrate safe boundary usage and not encourage uncontrolled panics for external input. |
| RFC-002 Public API Minimalism and `Tensor` Contract | Examples must use `matten::Tensor` as the primary user-facing type. |
| RFC-003 Shape Model, Scalar Semantics, and Validation | Shape examples must match scalar, vector, matrix, and ND semantics. |
| RFC-004 Construction and Conversion APIs | Creation and conversion examples depend on accepted constructors. |
| RFC-005 Error Model, Panic Messages, and Boundary APIs | Boundary examples must use `Result` APIs. |
| RFC-006 Broadcasting and Element-Wise Operators | Arithmetic examples depend on accepted operator semantics. |
| RFC-007 Reshape, Axis Operations, and Indexing | Shape manipulation examples depend on accepted reshape/axis contracts. |
| RFC-008 Slicing API: Builder and `slice_str` | Slicing examples depend on the canonical builder and secondary string parser. |
| RFC-009 Serde, JSON, CSV, and Boundary Integration | JSON/CSV examples depend on accepted boundary formats. |
| RFC-010 Reductions, Basic Statistics, and Matrix Multiplication | Mathematical examples depend on accepted math operations. |
| RFC-011 Dynamic `Element` Model and Coercion | Dynamic examples depend on accepted `Element` behavior. |
| RFC-012 Dynamic Storage, View Metadata, and Copy-on-Write | Dynamic examples must not misrepresent CoW behavior. |
| RFC-013 Testing, Compatibility, and Release Gates | Examples become part of release QA. |

RFC-014 should not block early implementation of core APIs. It should block `0.1.0` release readiness.

## 6. External Design

### 6.1 User-facing documentation contract

The public contract is:

> `matten` examples are runnable, copy-pasteable documentation for accepted APIs.

Every required example must:

1. compile with the documented command;
2. focus on one concept;
3. use only accepted public APIs;
4. avoid public implementation details;
5. avoid hidden dependencies unless explicitly feature-gated;
6. include a short header explaining what it teaches;
7. use tiny embedded data or committed fixture files;
8. avoid performance marketing;
9. be honest about limitations.

### 6.2 Public API policy

Examples may reveal ergonomic problems. They may motivate future RFCs. But examples do not create public API by themselves.

Allowed:

```rust
use matten::Tensor;

let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let b = Tensor::ones(&[2, 2]);
let c = &a + &b;
```

Not allowed in examples:

```rust
use matten::internal::Storage;
use matten::traits::ShapeLayout;

let x: Tensor<f64> = todo!();
```

The examples must reinforce that the user interacts primarily with `matten::Tensor`, and with `matten::Element` only when the `dynamic` feature requires it.

### 6.3 Required `0.1.0` examples

The following examples are required before `0.1.0` release readiness.

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
  20_dot_product.rs
  21_matrix_vector_product.rs
  22_matrix_multiplication.rs
  23_sum_mean.rs
  24_min_max.rs
  25_normalize_vector.rs
  26_cosine_similarity.rs
```

If schedule pressure exists, the maintainers may split required examples into `0.1.0-alpha` and `0.1.0` gates. The final `0.1.0` release should not ship without the full required set unless a project owner explicitly approves the reduction.

### 6.4 Recommended `0.1.x` examples

These examples are recommended after the first release once the basic API has stabilized.

```text
examples/
  standardize_columns.rs
  minmax_scaling.rs
  rowwise_scoring.rs
  column_summary.rs
  moving_average.rs
  rolling_windows_basic.rs
  pairwise_distance.rs
  gram_matrix.rs
```

These examples are useful but should not delay `0.1.0` if the core examples are complete.

### 6.5 Required `0.2.0 dynamic` examples

Dynamic examples are required only after RFC-011 and RFC-012 are accepted and implemented.

```text
examples/
  dynamic_00_quickstart.rs
  dynamic_01_mixed_elements.rs
  dynamic_02_missing_values.rs
  dynamic_03_fill_none.rs
  dynamic_04_numeric_coercion.rs
  dynamic_05_dirty_csv_cleanup.rs
```

Dynamic examples must clearly state that `matten` is not a full dataframe library. Their purpose is to help users ingest and clean messy PoC data before converting to numeric tensors.

### 6.6 Future examples

The following example families are future themes, not current commitments:

```text
examples/business/
  sales_matrix_summary.rs
  kpi_score_matrix.rs
  forecast_feature_matrix.rs
  inventory_risk_score.rs
  customer_feature_scaling.rs

examples/integration/
  ndarray_bridge.rs
  nalgebra_bridge.rs
  candle_bridge.rs
  axum_json_tensor.rs
```

Future bridge examples must be feature-gated and must not add default dependencies.

### 6.7 Explicitly out of current scope

The current example suite must not include:

| Example type | Reason |
|---|---|
| inverse / determinant / eigenvalues / SVD / QR / Cholesky | Pulls `matten` toward advanced linear algebra scope. |
| automatic differentiation | Pulls `matten` toward ML framework scope. |
| neural network training | Better served by ML crates such as `candle` or `burn`. |
| dataframe joins / group-by / pivot | Better served by `polars` or dataframe crates. |
| huge dataset benchmarks | Conflicts with DX-over-benchmarks positioning. |
| GPU examples | Explicitly future/out of scope. |
| sparse matrix examples | Specialized future scope. |
| SQL/database examples | Too much integration surface for the first release. |

## 7. Data Model

RFC-014 does not introduce a new runtime data model. It introduces an example-documentation model.

### 7.1 Example file model

Each example is a Rust source file with a focused teaching purpose.

Recommended file header:

```rust
//! Demonstrates matrix-vector multiplication with `matten::Tensor`.
//!
//! Run:
//! cargo run --example 21_matrix_vector_product
//!
//! This example is intended for small PoC workloads.
//! For advanced or performance-critical linear algebra, bridge to a specialized crate.
```

### 7.2 Fixture data model

Example fixtures live under:

```text
examples/data/
```

Allowed fixtures for `0.1.0`:

```text
examples/data/
  numeric_2x3.csv
  numeric_3x3.csv
  tensor_2x2.json
  malformed_numeric.csv
```

Fixture rules:

- fixtures must be tiny;
- fixtures must be human-readable;
- fixtures must not contain private, real, or business-sensitive data;
- fixtures must not require network access;
- fixtures must not be generated during normal test runs;
- fixtures must be stable across releases unless the documented format changes.

### 7.3 Example metadata model

Metadata is written in comments, not in a separate manifest for `0.1.0`.

Required metadata in each example header:

- purpose;
- run command;
- intended scope;
- feature flag if needed;
- note about limitations where relevant.

A separate machine-readable example manifest may be introduced later only if the example suite becomes hard to maintain.

## 8. Data Lifecycle

### 8.1 Example creation lifecycle

1. API/RFC is accepted.
2. Example is drafted against accepted public API.
3. Example is compiled locally.
4. Example is reviewed for scope and simplicity.
5. Example is added to CI.
6. Example becomes part of release QA.

### 8.2 Example maintenance lifecycle

When a public API changes:

1. update the source implementation;
2. update rustdoc examples;
3. update affected files under `examples/`;
4. run `cargo check --examples`;
5. run selected `cargo run --example ...` smoke tests;
6. update README/tutorial links if affected.

Examples must not be allowed to silently rot.

### 8.3 Fixture lifecycle

1. Add fixture only when embedded inline data would harm readability.
2. Keep fixture under `examples/data/`.
3. Document which example uses it.
4. Do not overwrite fixture content in tests.
5. Treat fixture schema changes as public documentation changes.

## 9. Events

There is no public event bus. In this RFC, “events” are project workflow events and release-quality events.

| Event | Required behavior |
|---|---|
| example added | must have header, command, focused purpose, and CI coverage |
| example modified | must be checked against the current public API |
| public API changed | affected examples must be updated in the same PR or a linked PR before release |
| fixture added | fixture must be tiny, documented, and non-sensitive |
| example CI fails | release candidate is blocked unless the example is explicitly removed from the required gate |
| dynamic example proposed | blocked until RFC-011/RFC-012 relevant behavior is accepted |
| future integration example proposed | must be feature-gated and reviewed for dependency impact |

## 10. Store Access

Examples must avoid hidden external store access.

Allowed:

- in-memory values;
- local committed files under `examples/data/`;
- stdout printing;
- temporary files only if explicitly needed by a boundary API example.

Not allowed for current examples:

- network access;
- database access;
- reading user home directories;
- depending on environment variables for normal success;
- downloading test data;
- writing persistent files outside temporary directories.

## 11. Cargo Features

### 11.1 Default examples

Default examples must run with default features unless their purpose is specifically to demonstrate feature behavior.

Required command:

```bash
cargo check --examples
```

### 11.2 Dynamic examples

Dynamic examples require:

```bash
cargo check --examples --features dynamic
cargo run --example dynamic_00_quickstart --features dynamic
```

Dynamic example file placement should make the feature requirement obvious. If Cargo’s example discovery makes nested examples awkward, filenames may be prefixed instead:

```text
examples/dynamic_00_quickstart.rs
examples/dynamic_01_mixed_elements.rs
```

Decision (v2): all runnable examples use flat filenames in `examples/` (e.g. `dynamic_00_quickstart.rs`, `22_matrix_multiplication.rs`). Cargo only auto-discovers `examples/*.rs`, so nested directories are not used for runnable examples.

### 11.3 Future integration examples

Future integration examples must use non-default features such as:

```toml
[features]
ndarray = ["dep:ndarray"]
nalgebra = ["dep:nalgebra"]
candle = ["dep:candle-core"]
axum = ["dep:axum"]
```

These features are future placeholders, not `0.1.0` commitments.

## 12. Public API Demonstration Requirements

### 12.1 Core examples

Core examples should demonstrate:

- `Tensor::new`;
- `Tensor::zeros`;
- `Tensor::ones`;
- shape inspection methods;
- reshape/flatten;
- element-wise operators;
- scalar operators;
- broadcasting;
- transpose/swap axes if accepted;
- slicing builder;
- `slice_str` as a secondary convenience;
- JSON/CSV boundary APIs;
- boundary error handling.

### 12.2 Math examples

Math examples should demonstrate only accepted basic math operations:

- dot product;
- matrix-vector multiplication;
- matrix-matrix multiplication;
- sum/mean;
- min/max;
- vector normalization;
- cosine similarity.

The examples may implement small helper functions locally if they clarify mathematical use, but local helper functions must not look like missing crate APIs unless explicitly marked as example-only.

### 12.3 Boundary examples

Boundary examples should prefer `Result`-returning `main` functions:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tensor = Tensor::load_csv("examples/data/numeric_2x3.csv")?;
    println!("shape = {:?}", tensor.shape());
    Ok(())
}
```

Quickstart examples may use direct construction and simple panicking examples. External input examples should not encourage `unwrap()`-heavy code.

## 13. Internal Design

### 13.1 Example smoke test strategy

Not every example needs to be run on every CI job if runtime becomes noticeable. But every example must at least compile.

Recommended baseline:

```bash
cargo check --examples
cargo test --examples
```

Recommended smoke-run subset for `0.1.0`:

```bash
cargo run --example 00_quickstart
cargo run --example 06_broadcasting
cargo run --example 08_slicing_builder
cargo run --example 12_boundary_error_handling
cargo run --example 22_matrix_multiplication
cargo run --example 25_normalize_vector
```

### 13.2 Output stability

Examples should not require snapshot testing in `0.1.0`.

A simple rule is enough:

- examples must compile;
- selected examples must run successfully;
- examples should print small, human-readable output;
- tests should verify behavior in unit/integration tests, not by fragile stdout matching.

### 13.3 Duplication control

Examples may repeat small setup code for readability. Avoid excessive abstraction in examples.

Good:

```rust
let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
```

Bad:

```rust
let x = examples_support::load_demo_matrix();
```

An `examples/common` helper module should not be introduced for `0.1.0` unless the examples become genuinely unmaintainable.

### 13.4 User mental model comments

It is acceptable and encouraged to include short comments comparing a `matten` operation to a NumPy-like mental model.

Example:

```rust
// Similar idea to NumPy: matrix + row_vector broadcasts across rows.
let result = &matrix + &row_vector;
```

Do not claim full NumPy compatibility.

## 14. Required Example Inventory

### 14.1 Core examples

| File | Demonstrates | Required for |
|---|---|---|
| `00_quickstart.rs` | Create, add, reshape, print | `0.1.0` |
| `01_create_tensor.rs` | `new`, `zeros`, `ones` | `0.1.0` |
| `02_shape_and_size.rs` | shape, ndim, len, scalar/vector/matrix | `0.1.0` |
| `03_reshape_flatten.rs` | reshape and flatten | `0.1.0` |
| `04_elementwise_ops.rs` | add/sub/mul/div/neg | `0.1.0` |
| `05_scalar_ops.rs` | scalar arithmetic | `0.1.0` |
| `06_broadcasting.rs` | scalar and row-vector broadcasting | `0.1.0` |
| `07_transpose_swap_axes.rs` | transpose and axis swap | `0.1.0` if RFC-007 implemented |
| `08_slicing_builder.rs` | canonical Rust-native slicing | `0.1.0` |
| `09_slice_str.rs` | string slicing convenience | `0.1.0` if RFC-008 parser implemented; otherwise `0.1.x` |
| `10_json_roundtrip.rs` | serde JSON format | `0.1.0` if RFC-009 JSON implemented |
| `11_csv_numeric_loading.rs` | numeric CSV loading | `0.1.0` if RFC-009 CSV implemented |
| `12_boundary_error_handling.rs` | malformed external input returns `Result` | `0.1.0` |

### 14.2 Math examples

| File | Demonstrates | Required for |
|---|---|---|
| `20_dot_product.rs` | vector dot product | `0.1.0` if RFC-010 implemented |
| `21_matrix_vector_product.rs` | matrix-vector multiplication | `0.1.0` if RFC-010 implemented |
| `22_matrix_multiplication.rs` | matrix-matrix multiplication | `0.1.0` if RFC-010 implemented |
| `23_sum_mean.rs` | basic reductions | `0.1.0` if RFC-010 implemented |
| `24_min_max.rs` | min/max and NaN note | `0.1.0` if RFC-010 implemented |
| `25_normalize_vector.rs` | L2 normalization using basic ops | `0.1.0` |
| `26_cosine_similarity.rs` | cosine similarity using dot/norm | `0.1.0` if RFC-010 implemented |

## 15. Acceptance Criteria

RFC-014 is accepted when:

- the example categories and release gates are approved;
- required `0.1.0` examples are explicitly listed;
- dynamic examples are deferred to `0.2.0`;
- future/out-of-scope examples are explicitly identified;
- examples are defined as executable documentation, not API generators;
- fixture and CI policies are defined;
- no advanced linear algebra, dataframe, ML, GPU, sparse, or benchmark scope is introduced.

## 16. Implementation Acceptance Criteria

The implementation is complete for `0.1.0` when:

- all required examples whose underlying APIs are implemented exist;
- each example has a short header and run command;
- `cargo check --examples` passes;
- selected smoke-run examples pass;
- examples use only public APIs;
- boundary examples use `Result` rather than encouraging uncontrolled `unwrap()` usage;
- fixture files are tiny, committed, and documented;
- README or docs link to the example suite.

## 17. QA Requirements

Minimum QA commands:

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --examples
cargo test --examples
cargo test --doc
```

Recommended `0.1.0` smoke commands:

```bash
cargo run --example 00_quickstart
cargo run --example 06_broadcasting
cargo run --example 08_slicing_builder
cargo run --example 12_boundary_error_handling
cargo run --example 22_matrix_multiplication
cargo run --example 25_normalize_vector
```

When `dynamic` examples are introduced:

```bash
cargo check --examples --features dynamic
cargo run --example dynamic_00_quickstart --features dynamic
```

## 18. Open Questions

1. Should nested example directories be used? **Resolved (v2): flat filenames** in `examples/` for all runnable examples (Cargo only discovers `examples/*.rs`).
2. Should `slice_str` examples be required for `0.1.0` or allowed in `0.1.x` if the parser is deferred?
3. Should JSON and CSV examples both be required for `0.1.0`, or should CSV be allowed in `0.1.x` if parser dependencies need more review?
4. Should example smoke runs be part of every PR or only release-candidate CI?

## 19. Final Position

`matten` should ship with examples early, but only with examples that reinforce the project identity.

The correct policy is:

> Examples make accepted APIs easier to understand.  
> They do not expand `matten` into a dataframe library, advanced linear algebra package, ML framework, or benchmark-oriented numerical engine.
