# RFC-044: Beginner Core Math Examples

**Status:** Implemented (v0.20.3) — beginner examples 30–32 (magic square, Fibonacci-by-matrix, graph path counting).  
**Target Release:** v0.20.x  
**Related:** RFC-043, RFC-038  
**Scope:** Beginner examples using stable core `Tensor` APIs

---

## 1. Summary

This RFC adds a beginner-friendly example set based on famous small math problems.

New example files (additive 30+ band; the existing `00_`–`28_` suite is preserved):

```text
examples/30_magic_square_checker.rs
examples/31_fibonacci_matrix_power.rs
examples/32_graph_path_counting.rs
```

Vector distance and cosine similarity are **not** added as new files. The repository already ships equivalent teaching examples, so this RFC cross-references / improves them in place instead of duplicating (architect ruling, RFC-043–048 review Q1/Q2):

```text
cosine similarity   -> existing examples/26_cosine_similarity.rs
vector distance     -> existing examples/pairwise_distance.rs (and 25_normalize_vector.rs)
```

These examples should work primarily with existing core APIs. Where RFC-038 APIs would improve readability, use local helper functions or mark the improvement as optional.

---

## 2. Motivation

New users need examples that answer:

```text
What can I do with Tensor?
How do vectors and matrices appear in code?
Can I solve recognizable math problems without heavy dependencies?
```

This RFC provides “first win” examples.

---

## 3. Existing: Vector Distance (audit / cross-reference, do not duplicate)

The repository already ships `examples/pairwise_distance.rs` (and the related
`examples/25_normalize_vector.rs`), which teach squared Euclidean distance over
`Tensor` vectors. Per the architect ruling (RFC-043–048 review Q2), this RFC does
**not** add a new `vector_distance` file.

Required action:

```text
[ ] audit existing pairwise_distance.rs
[ ] if it lacks a beginner-friendly problem/math/representation header, improve it in place
[ ] cross-reference it from the beginner-math docs page
```

A separate new file is justified only if it teaches something clearly different
(e.g. nearest-point selection), but that overlaps the nearest-neighbor work in
RFC-047, so avoid it unless needed.

For reference, the underlying idea:

```text
distance^2 = sum((a_i - b_i)^2)
```

If `sqrt()` from RFC-038 later lands, the existing example may also print the
Euclidean distance; before then, squared distance is the deterministic output.

---

## 4. Existing: Cosine Similarity (audit / cross-reference, do not duplicate)

The repository already ships `examples/26_cosine_similarity.rs`. Per the architect
ruling (RFC-043–048 review Q2), this RFC does **not** add a second cosine-similarity
file.

Required action:

```text
[ ] audit existing 26_cosine_similarity.rs
[ ] if it reads as an API fragment rather than a teaching example, improve it in place
    (add problem / math idea / Tensor representation / expected-output header)
[ ] cross-reference it from the beginner-math docs page
```

For reference, the underlying idea:

```text
cos(a, b) = dot(a, b) / (||a|| * ||b||)
norm(a)   = sqrt(dot(a, a))   // computed locally if no core norm() exists
```

The example should make clear this is a single-formula demonstration, not an
embedding/search framework.

---

## 5. Example 30: Magic Square Checker

### Problem

Check whether a square matrix is a magic square.

Classic question:

```text
Do all rows, columns, and the two diagonals have the same sum?
```

### APIs demonstrated

- matrix Tensor construction;
- shape inspection;
- indexing;
- row/column traversal;
- reductions.

### Suggested input

Use the Lo Shu square:

```text
8 1 6
3 5 7
4 9 2
```

Expected magic sum:

```text
15
```

### Acceptance

```text
[ ] verifies rows
[ ] verifies columns
[ ] verifies diagonals
[ ] prints true/false and magic sum
[ ] does not require advanced slicing if not available
```

---

## 6. Example 31: Fibonacci by Matrix Power

### Problem

Compute Fibonacci numbers using matrix multiplication.

Classic identity:

```text
[[1, 1],
 [1, 0]]^n
```

### APIs demonstrated

- matrix construction;
- matrix multiplication;
- small loop;
- reading a matrix element.

### Implementation note

Do not add a generic matrix-power API only for this example. Use a simple local loop.

### Acceptance

```text
[ ] computes a small Fibonacci number, e.g. F(10)
[ ] uses repeated matmul
[ ] comments explain the matrix identity
[ ] no big integer claim
```

---

## 7. Example 32: Graph Path Counting

### Problem

Use an adjacency matrix to count walks in a graph.

Famous graph fact:

```text
(A^k)[i, j] = number of walks of length k from i to j
```

### APIs demonstrated

- matrix representation of graph;
- matrix multiplication;
- matrix powers by local loop;
- interpreting output matrix.

### Acceptance

```text
[ ] uses a tiny graph of 3-5 nodes
[ ] computes paths of length 2 or 3
[ ] prints small matrix
[ ] comments distinguish walks from simple paths
```

---

## 8. Documentation Requirements

Add docs page:

```text
docs/src/examples/beginner-math.md
```

Each example entry includes:

- problem;
- difficulty;
- API demonstrated;
- command to run.

---

## 9. QA Checklist

```text
[ ] all examples compile
[ ] all examples run deterministically
[ ] no external datasets
[ ] no future-only API unless guarded
[ ] examples are beginner-readable
[ ] examples do not imply a larger framework
```

CI:

```bash
cargo check -p matten --examples --all-features
cargo run -p matten --example 30_magic_square_checker
cargo run -p matten --example 31_fibonacci_matrix_power
cargo run -p matten --example 32_graph_path_counting
```

---

## 10. Non-goals

- No generic graph library.
- No number theory package.
- No symbolic math.
- No plotting.
- No large matrices.
