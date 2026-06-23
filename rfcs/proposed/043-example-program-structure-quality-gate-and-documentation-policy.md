# RFC-043: Example Program Structure, Quality Gate, and Documentation Policy

**Status:** Proposed  
**Target Release:** v0.20.x / v0.21.0  
**Owner:** `matten` maintainers  
**Related:** RFC-038, RFC-039, RFC-040, RFC-041, RFC-044, RFC-045, RFC-046, RFC-047, RFC-048  
**Scope:** Overall example strategy and quality gate

---

## 1. Summary

This RFC defines the overall examples program for `matten`.

The goal is to make examples feel:

```text
recognizable
educational
small enough to read
useful as API demonstrations
faithful to matten's Sedan-first philosophy
```

The project should add examples based on famous small math / numerical-computing problems, but avoid implying that `matten` is a NumPy, SciPy, Pandas, or ML-framework clone.

This RFC is a governance and quality-gate RFC. The concrete example groups are defined in RFC-044 through RFC-048.

**Audit first (architect ruling, RFC-043–048 review Q1/Q2).** `matten` already ships
a mature numbered example suite (`00_`–`28_`, the `dynamic_*` set, and several named
examples) plus companion examples. Before any new example is written, implementers
must inventory the existing examples and:

```text
- add new famous-problem examples only in a fresh additive band (30+),
  never renumbering or reorganizing the existing 00–28 suite;
- not duplicate a concept the existing suite already teaches
  (e.g. cosine similarity, pairwise distance) — cross-reference or improve in place;
- treat existing companion examples as already satisfying RFC-048 where adequate.
```

---

## 2. Motivation

`matten` has enough capability to demonstrate real math, not only API fragments.

Good examples can help users understand:

- how `Tensor` maps to vectors and matrices;
- how shape, slicing, reductions, and matrix multiplication work;
- how companion crates fit;
- what `matten` intentionally does not do.

Examples should not become hidden product commitments. They should teach accepted APIs, not future dreams.

---

## 3. Example Categories

The examples program is divided into reasonable units:

```text
RFC-044: Beginner Core Math Examples
  magic square, Fibonacci matrix power, graph path counting
  (Euclidean distance and cosine similarity already ship as
   pairwise_distance / 26_cosine_similarity — cross-reference, do not duplicate).

RFC-045: Matrix Iteration and Graph/Probability Examples
  Markov chain weather model, tiny PageRank, adjacency-matrix walks.

RFC-046: Numerical Methods and Scientific Toy Examples
  linear regression by gradient descent, 1D heat equation,
  optional finite differences / trapezoidal integration after RFC-038.

RFC-047: Small ML-Like Examples Without ML-Framework Scope
  K-means, nearest neighbor classification,
  clear "algorithm demo, not ML framework" boundary.

RFC-048: Companion-Crate Examples
  audit/improve the existing matten-ndarray (from_arrayd/to_arrayd),
  matten-mlprep (standardize_columns/train_test_split), and
  matten-data (csv_to_tensor, shipped in v0.20.1) examples — do not duplicate.
```

---

## 4. Required Structure for Every Example

Each example file SHOULD have this structure:

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

The example body should then be ordinary runnable Rust.

---

## 5. Example Naming Convention

New famous-problem examples use a fresh **additive 30+ band**, placed beside the
existing `00_`–`28_` core suite without renumbering or reorganizing it (architect
ruling, RFC-043–048 review Q1):

```text
examples/30_magic_square_checker.rs
examples/31_fibonacci_matrix_power.rs
examples/32_graph_path_counting.rs
...
```

The existing suite is the canonical home for concepts it already covers; new files
must not collide with `00_`–`28_` and must not duplicate an existing example
(see §3 and the audit requirement in §1).

Companion examples already exist and are audited/improved in place rather than
re-created:

```text
crates/matten-ndarray/examples/from_arrayd.rs, to_arrayd.rs
crates/matten-mlprep/examples/standardize_columns.rs, train_test_split.rs
crates/matten-data/examples/csv_to_tensor.rs   (shipped in v0.20.1)
```

Avoid names that imply unsupported scope:

```text
dataframe_groupby.rs
large_csv_streaming.rs
neural_network_training.rs
svd_pca.rs
```

---

## 6. Difficulty Labels

Every example README entry should include:

```text
Difficulty: Beginner / Intermediate / Advanced-small
```

Definitions:

```text
Beginner:
  one or two Tensor concepts.

Intermediate:
  matrix iteration, reductions, or simple algorithm loop.

Advanced-small:
  still readable in one file, but demonstrates a full toy algorithm.
```

Avoid “Advanced” examples that require heavy math libraries or hidden domain knowledge.

---

## 7. API Compatibility Policy

Examples must compile in CI.

If an example requires future RFC-038 APIs, mark it as deferred until those APIs exist.

Do not add examples that require:

- unaccepted APIs;
- hidden helper crates;
- large dependencies;
- network access;
- external datasets.

If a future API is needed, the example RFC must say:

```text
Requires RFC-038
```

or similar.

---

## 8. Output Policy

Examples should print small deterministic output.

Good:

```text
distance^2 = 25
cosine_similarity = 0.9746
stationary-ish distribution after 10 steps = [...]
```

Avoid:

- large arrays;
- nondeterministic output;
- floating-point output requiring exact long decimals.

Use rounded display where appropriate.

---

## 9. Testing Policy

Each example must be checked by CI:

```bash
cargo check --examples
cargo run --example <name>
```

At minimum:

```bash
cargo check --workspace --examples --all-features
```

Where practical, move algorithmic functions into testable helpers or use doc tests.

---

## 10. Documentation Policy

Extend the existing docs structure (do not introduce a parallel `docs/src/examples.md`):

```text
docs/src/examples/index.md          (existing — extend it)
docs/src/examples/beginner-math.md
docs/src/examples/matrix-iteration.md
docs/src/examples/numerical-methods.md
docs/src/examples/ml-like.md
docs/src/examples/companions.md
```

The existing `docs/src/examples/index.md` already lists the shipped examples;
new pages must preserve that navigation rather than replace it.

Docs should explain:

```text
Examples are small teaching examples, not production algorithm packages.
```

---

## 11. Non-goals

Examples must not imply that `matten` is:

- a dataframe engine;
- a full ML framework;
- a SciPy clone;
- a linear algebra backend;
- a large-data engine;
- a plotting library.

---

## 12. Acceptance Criteria

```text
[ ] example taxonomy documented
[ ] required example file structure documented
[ ] examples have difficulty labels
[ ] examples compile in CI
[ ] examples use stable, small output
[ ] examples do not imply unsupported scope
[ ] companion examples live in companion crates
[ ] future-only examples are not added before their APIs exist
```

---

## 13. Implementation Order

Recommended order:

```text
1. RFC-043 policy/docs
2. RFC-044 beginner core examples
3. RFC-045 matrix iteration examples
4. RFC-048 existing companion examples
5. RFC-046 numerical methods
6. RFC-047 ML-like examples
```

RFC-046 and RFC-047 may depend on RFC-038 comfort APIs for cleaner implementation.
