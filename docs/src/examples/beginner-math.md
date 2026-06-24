# Beginner applied math

A small set of recognizable math problems that show what a `matten::Tensor` can
represent and how short vector/matrix algorithms look in `matten`. They use only
the default numeric Tensor API — no extra features, no external crates, and small
hard-coded inputs with stable output.

These examples are teaching examples, not a production algorithm package. They sit
in a `30+` filename band so the established `00_`–`28_` suite stays untouched.

## Examples

### `30_magic_square_checker.rs`

*Difficulty: Beginner.* Checks whether a square matrix is a magic square — every
row, column, and both diagonals share one sum. Demonstrates 2-D `Tensor::new`,
`shape`, and element access with `get(&[row, col])`. Uses the classic 3×3 Lo Shu
square (magic constant 15).

```bash
cargo run --example 30_magic_square_checker
```

Source: [`30_magic_square_checker.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/30_magic_square_checker.rs)

### `31_fibonacci_matrix_power.rs`

*Difficulty: Beginner.* Computes Fibonacci numbers from the identity
`Q^n = [[F(n+1), F(n)], [F(n), F(n-1)]]` with `Q = [[1, 1], [1, 0]]`. Demonstrates
repeated `Tensor::matmul` (recall that `*` is element-wise, never a matrix product)
and reading one element with `get`. A demonstration of the identity, not a
big-integer routine.

```bash
cargo run --example 31_fibonacci_matrix_power
```

Source: [`31_fibonacci_matrix_power.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/31_fibonacci_matrix_power.rs)

### `32_graph_path_counting.rs`

*Difficulty: Beginner.* Counts walks in a directed graph using the fact that
`(A^k)[i, j]` is the number of walks of length `k` from node `i` to node `j`.
Demonstrates representing a graph as an adjacency `Tensor` and taking matrix powers
via `matmul`. Note the distinction between a *walk* (may repeat nodes/edges) and a
*simple path* (may not).

```bash
cargo run --example 32_graph_path_counting
```

Source: [`32_graph_path_counting.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/32_graph_path_counting.rs)

## Already covered (cross-references)

Two classic beginner problems already ship as examples, so this band does not add
duplicates:

- **Vector distance** — `54_pairwise_distance.rs` (and `25_normalize_vector.rs`).
- **Cosine similarity** — `26_cosine_similarity.rs`.

## What this is not

These examples do not imply that `matten` is a graph library, a number-theory
package, or an ML framework. They are single-file demonstrations of accepted APIs.
