# `matten` Examples Implementation Handoff

**Project:** `matten`  
**Related RFCs:** RFC-043 through RFC-048  
**Document Kind:** Compact Developer Handoff  
**Status:** Draft for implementation planning  
**Scope:** Example program implementation, documentation, CI checks, and QA review  
**Numbering Note:** This handoff covers the examples RFC pack only. It does not replace the RFCs.

---

## 0. Purpose

This handoff turns RFC-043 through RFC-048 into a practical implementation plan.

The examples program should make `matten` feel useful and approachable by solving recognizable small math / numerical-computing problems, while preserving the project philosophy:

```text
matten is a small, Tensor-centered Rust numeric crate.

It is not:
  a dataframe engine
  a SciPy clone
  a Pandas clone
  an ML framework
  a large-data engine
  a serious linear algebra backend
```

The examples should teach:

```text
vectors
matrices
shape
reductions
matrix multiplication
simple iteration
companion-crate boundaries
```

They should not create hidden product promises.

---

## 1. Implementation Strategy

Use one incremental examples program, not one heavy implementation project per RFC.

Recommended implementation phases:

```text
Phase 0:
  Inventory existing examples FIRST (architect ruling, RFC-043-048 review Q1/Q2).
  Run `ls crates/*/examples` and read docs/src/examples/index.md.
  The core crate already ships 00_-28_, dynamic_*, and several named examples;
  companions already ship their own examples. New famous-problem examples go in a
  fresh additive 30+ band; do NOT renumber 00-28 and do NOT duplicate a concept
  the suite already teaches (cosine similarity, pairwise distance, companion
  roundtrip/standardize/csv_to_tensor).

Phase 1:
  RFC-043 examples program structure and documentation/CI policy.

Phase 2:
  RFC-044 beginner core math examples (new 30+ files; audit/cross-reference
  existing distance/cosine examples instead of duplicating).

Phase 3:
  RFC-045 matrix iteration and graph/probability examples.

Phase 4:
  RFC-048 audit/improve existing companion-crate examples (matten-data shipped
  csv_to_tensor in v0.20.1).

Phase 5:
  RFC-046 numerical-method examples.

Phase 6:
  RFC-047 small ML-like examples.
```

Phase 5 and Phase 6 may be split or delayed if they need APIs from RFC-038, such as `linspace`, `sqrt`, `argmin`, `argmax`, `squeeze`, or `expand_dims`.

---

## 2. Global Rules for All Examples

### 2.1 File structure

Each example should begin with a short explanation:

```rust
//! # Example: <name>
//!
//! ## Problem
//! Short statement of the recognizable math problem.
//!
//! ## Math idea
//! One short paragraph.
//!
//! ## Tensor representation
//! How vectors/matrices map to `Tensor`.
//!
//! ## What this demonstrates
//! List of matten APIs shown.
//!
//! ## Expected output
//! Short, stable printed output.
```

### 2.2 Output style

Examples should print small deterministic output.

Good:

```text
squared distance = 25
cosine similarity = 0.9746
magic square = true, sum = 15
```

Avoid:

```text
large tensors
long floating-point dumps
nondeterministic output
external dataset paths
```

### 2.3 Data policy

Examples must use:

```text
small hard-coded values
tiny matrices
deterministic algorithms
no network access
no external datasets
```

### 2.4 Scope policy

Examples must not imply:

```text
dataframe analysis
large CSV processing
ML framework behavior
autograd
GPU acceleration
full graph library
full statistics library
serious linear algebra backend
```

### 2.5 Dependency policy

Core examples must use only `matten`.

Companion examples must live in the companion crates and preserve dependency direction:

```text
matten-ndarray -> matten
matten-mlprep  -> matten
matten-data -> matten
```

Core `matten` must not depend on companion crates.

### 2.6 Import policy

Canonical import style:

```rust
use matten::Tensor;
```

Companion examples:

```rust
use matten::Tensor;
use matten_ndarray::to_arrayd;
```

Do not teach:

```rust
use matten_ndarray::Tensor;
use matten_data::Tensor;
```

Do not add `pub use matten;` to companions. That convenience path is deferred by RFC-032.

---

## 3. Phase 1 — RFC-043 Examples Program Structure

### Goal

Create the documentation and CI foundation before adding many examples.

### Tasks

1. Add documentation index:

```text
docs/src/examples/index.md
docs/src/examples/beginner-math.md
docs/src/examples/matrix-iteration.md
docs/src/examples/numerical-methods.md
docs/src/examples/ml-like.md
docs/src/examples/companions.md
```

2. Add or update navigation in mdBook / docs index.

3. Document example difficulty labels:

```text
Beginner
Intermediate
Advanced-small
```

4. Add CI support for examples:

```bash
cargo check --workspace --examples --all-features
```

5. Add release-doc checklist items:

```text
examples are deterministic
examples do not imply unsupported scope
companion examples live in companion crates
future-only examples are not added before APIs exist
```

### Acceptance checklist

```text
[ ] docs example index exists
[ ] example categories are documented
[ ] difficulty labels are documented
[ ] CI checks examples
[ ] examples policy says not a dataframe / SciPy / ML framework
```

---

## 4. Phase 2 — RFC-044 Beginner Core Math Examples

### Goal

Add small, friendly examples that demonstrate the core `Tensor` model.

### Example files

New files (additive 30+ band; existing `00_`–`28_` suite untouched):

```text
crates/matten/examples/30_magic_square_checker.rs
crates/matten/examples/31_fibonacci_matrix_power.rs
crates/matten/examples/32_graph_path_counting.rs
```

Cross-reference / improve (do NOT create new files):

```text
existing crates/matten/examples/pairwise_distance.rs   (vector distance)
existing crates/matten/examples/26_cosine_similarity.rs
existing crates/matten/examples/25_normalize_vector.rs
```

---

### 4.1 Vector distance — audit existing `pairwise_distance.rs`

Architect ruling (RFC-043–048 review Q2): do not add a new vector-distance file.
Audit the existing `pairwise_distance.rs`; if it lacks a beginner-friendly
problem/math/representation header, improve it in place and cross-reference it from
the beginner-math docs page.

Concept (for reference only):

```text
Compute squared Euclidean distance between two vectors.
distance^2 = sum((a_i - b_i)^2)
```

Before RFC-038 `sqrt`, squared distance is the deterministic output; after RFC-038
the existing example may optionally also print the Euclidean distance.

---

### 4.2 Cosine similarity — audit existing `26_cosine_similarity.rs`

Architect ruling (RFC-043–048 review Q2): do not add a second cosine-similarity
file. Audit the existing `26_cosine_similarity.rs`; if it reads as an API fragment,
improve it in place (add problem / math idea / Tensor representation / expected
output) and cross-reference it from the beginner-math docs page.

Concept (for reference only):

```text
cos(a, b) = dot(a, b) / (||a|| * ||b||)
norm(a)   = sqrt(dot(a, a))   // computed locally if no core norm() exists
```

Keep output rounded; make clear this is a single-formula demo, not a search framework.

---

### 4.3 `30_magic_square_checker.rs`

Problem:

```text
Check whether the Lo Shu square is a magic square.
```

Input:

```text
8 1 6
3 5 7
4 9 2
```

Expected sum:

```text
15
```

Demonstrates:

```text
matrix shape
row/column access
diagonal access
reductions
```

Acceptance:

```text
[ ] verifies rows
[ ] verifies columns
[ ] verifies diagonals
[ ] prints true/false and magic sum
```

---

### 4.4 `31_fibonacci_matrix_power.rs`

Problem:

```text
Compute Fibonacci numbers using repeated matrix multiplication.
```

Demonstrates:

```text
matrix construction
matmul
iteration
reading output element
```

Implementation note:

- Use a local loop.
- Do not add a generic matrix-power API only for this example.

Acceptance:

```text
[ ] computes small Fibonacci number such as F(10)
[ ] comments explain the matrix identity
[ ] no big-integer claim
```

---

### 4.5 `32_graph_path_counting.rs`

Problem:

```text
Use adjacency-matrix powers to count walks of length k.
```

Demonstrates:

```text
graph as matrix
matmul
matrix powers
interpreting tensor output
```

Acceptance:

```text
[ ] tiny graph of 3-5 nodes
[ ] computes walks of length 2 or 3
[ ] comments distinguish walks from simple paths
```

---

### Phase 2 QA

```bash
cargo check -p matten --examples --all-features
cargo run -p matten --example 30_magic_square_checker
cargo run -p matten --example 31_fibonacci_matrix_power
cargo run -p matten --example 32_graph_path_counting
```

---

## 5. Phase 3 — RFC-045 Matrix Iteration and Graph/Probability Examples

### Goal

Add intermediate examples showing repeated matrix/vector updates.

### Example files

```text
crates/matten/examples/33_markov_chain_weather.rs
crates/matten/examples/34_tiny_pagerank.rs
```

Optional:

```text
crates/matten/examples/XX_adjacency_walks_extended.rs
```

---

### 5.1 `33_markov_chain_weather.rs`

Problem:

```text
Compute weather-state probabilities after N days.
```

Example model:

```text
Sunny -> Sunny: 0.8
Sunny -> Rainy: 0.2
Rainy -> Sunny: 0.4
Rainy -> Rainy: 0.6
```

Demonstrates:

```text
state vector
transition matrix
matrix-vector update
iteration
probability interpretation
```

Acceptance:

```text
[ ] transition orientation documented
[ ] initial state printed
[ ] final distribution printed
[ ] probability total approximately preserved
```

---

### 5.2 `34_tiny_pagerank.rs`

Problem:

```text
Rank a tiny graph of pages by link structure.
```

Demonstrates:

```text
adjacency matrix
normalization
damping factor
iterative update
```

Implementation note:

- Hard-code 4 or 5 pages.
- Use fixed iteration count.
- No web/network input.

Acceptance:

```text
[ ] no network access
[ ] damping factor documented
[ ] output deterministic
[ ] comments say this is a toy PageRank example
```

---

### 5.3 Optional `08_adjacency_walks_extended.rs`

Only add if it teaches something beyond RFC-044's graph path counting.

Acceptance:

```text
[ ] not redundant
[ ] tiny graph
[ ] small output
```

---

### Phase 3 QA

```bash
cargo check -p matten --examples --all-features
cargo run -p matten --example 33_markov_chain_weather
cargo run -p matten --example 34_tiny_pagerank
```

---

## 6. Phase 4 — RFC-048 Audit/Improve Existing Companion Examples

### Goal

All three companion crates already ship examples. Audit and improve them in place;
do NOT add duplicate or renamed files (architect ruling, RFC-043–048 review Q2/Q3).

### Existing example files to audit

```text
crates/matten-ndarray/examples/from_arrayd.rs, to_arrayd.rs
crates/matten-mlprep/examples/standardize_columns.rs, train_test_split.rs,
                               add_bias_column.rs, minmax_scale.rs
crates/matten-data/examples/csv_to_tensor.rs        (shipped in v0.20.1)
```

---

### 6.1 `matten-ndarray`: audit `from_arrayd.rs` / `to_arrayd.rs`

Problem:

```text
Convert Tensor to ndarray ArrayD and back.
```

Demonstrates:

```text
to_arrayd
from_arrayd
shape preservation
copy behavior
companion boundary
```

Required teaching points:

```text
conversion copies data
shape is preserved
dynamic tensors are rejected unless converted to numeric first
core matten does not depend on ndarray
```

Acceptance:

```text
[ ] lives in matten-ndarray examples
[ ] canonical imports
[ ] no zero-copy claim
[ ] shape printed before/after
```

---

### 6.2 `matten-mlprep`: audit `standardize_columns.rs` / `train_test_split.rs`

Problem:

```text
Prepare a small feature matrix.
```

Demonstrates:

```text
standardize_columns
add_bias_column
train_test_split
```

Required teaching points:

```text
rows = samples
columns = features
split deterministic
no hidden randomness
no model training
```

Acceptance:

```text
[ ] lives in matten-mlprep examples
[ ] deterministic output
[ ] no model type
[ ] no training framework language
```

---

### 6.3 `matten-data`: audit existing `csv_to_tensor.rs` (shipped v0.20.1)

`matten-data` shipped its table API and `examples/csv_to_tensor.rs` in v0.20.1.
Audit/improve the existing example; do not add a `01_csv_to_tensor.rs`.

Shipped workflow (already implemented):

```rust
use matten_data::Table;

let table = Table::from_csv_str(csv)?;

let x = table
    .select_columns(["sales", "cost", "quantity"])?
    .fill_missing(0.0)?
    .try_numeric()?
    .to_tensor()?;
```

Acceptance (audit the existing example against these):

```text
[ ] says matten-data is experimental
[ ] not a dataframe example
[ ] missing values explicit
[ ] numeric conversion explicit
[ ] output shape printed
```

---

### Phase 4 QA

```bash
cargo check -p matten-ndarray --examples --all-features
cargo check -p matten-mlprep --examples --all-features
cargo run -p matten-ndarray --example from_arrayd
cargo run -p matten-ndarray --example to_arrayd
cargo run -p matten-mlprep --example standardize_columns
cargo run -p matten-mlprep --example train_test_split
bash scripts/check-core-dependency-boundary.sh
```

---

## 7. Phase 5 — RFC-046 Numerical Methods Examples

### Goal

Add “so-so complicated” examples that demonstrate real numerical workflows while staying small.

### Example files

```text
crates/matten/examples/35_linear_regression_gradient_descent.rs
crates/matten/examples/36_heat_equation_1d.rs
```

Optional after RFC-038:

```text
crates/matten/examples/39_finite_difference_derivative.rs
crates/matten/examples/40_trapezoidal_integration.rs
```

---

### 7.1 `35_linear_regression_gradient_descent.rs`

Problem:

```text
Fit y = ax + b to small hard-coded points.
```

Demonstrates:

```text
design matrix
weights vector
prediction = Xw
residuals
mean squared error
manual gradient descent loop
```

Implementation note:

- Do not add optimizer abstraction.
- Use fixed learning rate and fixed iteration count.
- Keep data tiny.

Acceptance:

```text
[ ] deterministic final parameters
[ ] comments explain design matrix
[ ] says this is not an ML framework
[ ] no autograd
```

---

### 7.2 `36_heat_equation_1d.rs`

Problem:

```text
Simulate heat diffusion along a 1D rod.
```

Update rule:

```text
next[i] = current[i] + alpha * (current[i-1] - 2*current[i] + current[i+1])
```

Demonstrates:

```text
vector as grid
iteration
indexing/slicing
boundary policy
stencil update
```

Acceptance:

```text
[ ] small grid
[ ] fixed boundary policy
[ ] prints initial and final state
[ ] no PDE solver claim
```

---

### 7.3 Optional examples after RFC-038

Add only when APIs are ready:

```text
finite difference derivative
trapezoidal integration
```

Acceptance:

```text
[ ] marked as numerical approximation
[ ] no symbolic math claim
[ ] no SciPy replacement language
```

---

### Phase 5 QA

```bash
cargo check -p matten --examples --all-features
cargo run -p matten --example 35_linear_regression_gradient_descent
cargo run -p matten --example 36_heat_equation_1d
```

---

## 8. Phase 6 — RFC-047 Small ML-Like Examples

### Goal

Show small ML-adjacent algorithms without creating ML-framework scope.

### Example files

```text
crates/matten/examples/37_kmeans_small.rs
crates/matten/examples/38_nearest_neighbor_classification.rs
```

---

### 8.1 `37_kmeans_small.rs`

Problem:

```text
Cluster a tiny set of 2D points into two groups.
```

Demonstrates:

```text
distance calculation
nearest center assignment
mean update
fixed iteration loop
```

Implementation note:

- Use fixed initial centers.
- Use fixed iteration count.
- Before `argmin`, use local helper.

Acceptance:

```text
[ ] tiny hard-coded dataset
[ ] deterministic output
[ ] no random initialization unless seed fixed
[ ] no KMeans public type
[ ] says algorithm demo, not ML framework
```

---

### 8.2 `38_nearest_neighbor_classification.rs`

Problem:

```text
Classify a point by the label of the nearest training point.
```

Demonstrates:

```text
distance matrix or row distance
argmin-like selection
labels kept outside Tensor
```

Implementation note:

- Keep labels as normal Rust strings/integers.
- Do not create dataset or classifier abstractions.

Acceptance:

```text
[ ] tiny dataset
[ ] one or two query points
[ ] deterministic prediction
[ ] no dataset loader
[ ] no classifier abstraction
```

---

### Phase 6 QA

```bash
cargo check -p matten --examples --all-features
cargo run -p matten --example 37_kmeans_small
cargo run -p matten --example 38_nearest_neighbor_classification
```

---

## 9. Global Acceptance Checklist

Before accepting the examples program:

```text
[ ] all current examples compile
[ ] all new examples run deterministically
[ ] examples have problem/math/tensor/expected-output comments
[ ] examples use only accepted APIs
[ ] examples do not require external datasets
[ ] examples do not create new public API needs silently
[ ] companion examples live in companion crates
[ ] docs index lists examples with difficulty
[ ] CI checks examples
[ ] release-doc checks prevent scope drift
```

---

## 10. Global CI Commands

Run at the end of the examples program:

```bash
cargo fmt --all --check
bash scripts/check-core-dependency-boundary.sh
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo check --workspace --examples --all-features
```

Run examples individually where stable output matters:

```bash
cargo run -p matten --example 30_magic_square_checker
cargo run -p matten --example 31_fibonacci_matrix_power
cargo run -p matten --example 32_graph_path_counting
cargo run -p matten --example 33_markov_chain_weather
cargo run -p matten --example 34_tiny_pagerank
cargo run -p matten --example 35_linear_regression_gradient_descent
cargo run -p matten --example 36_heat_equation_1d
cargo run -p matten-ndarray --example from_arrayd
cargo run -p matten-ndarray --example to_arrayd
cargo run -p matten-mlprep --example standardize_columns
cargo run -p matten-mlprep --example train_test_split
```

---

## 11. PR Plan Summary

### PR-EX-1: Example structure and docs

Implements RFC-043.

```text
docs index
difficulty labels
CI example check
example policy
```

### PR-EX-2: Beginner examples

Implements RFC-044.

```text
30_magic_square_checker
31_fibonacci_matrix_power
32_graph_path_counting
```

### PR-EX-3: Matrix iteration examples

Implements RFC-045.

```text
33_markov_chain_weather
34_tiny_pagerank
```

### PR-EX-4: Current companion examples

Implements current part of RFC-048.

```text
matten-ndarray roundtrip
matten-mlprep standardize/train-test
```

### PR-EX-5: Numerical-method examples

Implements stable part of RFC-046.

```text
35_linear_regression_gradient_descent
36_heat_equation_1d
```

### PR-EX-6: ML-like examples

Implements RFC-047 after reviewing API needs.

```text
37_kmeans_small
38_nearest_neighbor_classification
```

### PR-EX-7: Future examples after APIs land

Deferred examples:

```text
finite_difference_derivative
trapezoidal_integration
matten-data csv_to_tensor
```

---

## 12. Do Not Implement

Do not add examples for:

```text
large CSV
streaming CSV
dataframe group-by
join / merge / pivot
SVD / PCA as core examples
eigen decomposition as core example
neural network training
autograd
GPU/device usage
sparse matrices
database ingestion
web/network data loading
```

Do not introduce new public APIs merely to make an example shorter. If an example needs a new API, open or reference the correct RFC.

---

## 13. Review Checklist for Each Example PR

Reviewers should ask:

```text
[ ] Is the problem recognizable?
[ ] Is the math idea explained briefly?
[ ] Is the Tensor representation clear?
[ ] Does the example compile and run?
[ ] Is the output small and deterministic?
[ ] Does it use only accepted APIs?
[ ] Does it avoid unsupported product claims?
[ ] Is it in the correct crate?
[ ] Does it avoid heavy dependencies?
[ ] Does it fit the Sedan-first philosophy?
```

---

## 14. Final Recommendation

Implement the examples program as a compact sequence of small PRs.

Do not create per-RFC developer handoffs unless a specific example becomes unexpectedly complex.

The examples should make `matten` look useful by solving real small problems, not by pretending to be a large numerical ecosystem.
