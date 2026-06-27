# Choosing a target

There is no universally "best" target — the right destination depends on the *shape* of the
pressure you are feeling. Use the matrix below to map a signal to an ecosystem, then open
the matching [playbook](./playbooks/index.md).

## Target-selection matrix

| Pressure / need | Recommended target | Notes |
|---|---|---|
| General N-D numeric arrays, dense `matmul`, axis reductions at scale | **`ndarray`** | The general Rust N-D array production path; BLAS-backed matmul available. |
| Small/mid dense vectors & matrices, decompositions, solvers (LU/QR/SVD), eigenvalues | **`nalgebra`** | The dense linear-algebra path. |
| Group-by, joins, pivots, query-style dataframe analytics | **Polars** (Rust) / **Pandas** (Python) | `matten-data` is an ingestion on-ramp only; it will **not** grow these. *(Playbook in a later revision.)* |
| Autodiff, training loops, GPU/device execution | **Candle** (Rust) / framework of choice | `matten` is not an ML framework. *(Playbook in a later revision.)* |
| Existing Python scientific stack, NumPy interop | **NumPy** (Python) | Manual/conceptual hand-off unless a future bridge is designed. *(Playbook in a later revision.)* |
| Small numeric work, ingestion, glue, learning/PoC | **stay with `matten`** | Migrating here would add dependencies for no real gain. |

## A quick decision path

1. **Is the bottleneck a dense numeric kernel** (matmul, matrix–vector, operator
   application, axis reductions) that you have measured as hot? → `ndarray` (general N-D) or
   `nalgebra` (if it is fundamentally small/mid dense linear algebra needing
   decompositions).
2. **Do you need linear-algebra results** `matten` does not provide (LU/QR/SVD, solvers,
   eigenvalues)? → `nalgebra`.
3. **Is the real need tabular** (group-by/join/pivot/query)? → Polars (Rust) or Pandas
   (Python). Not `matten-data`.
4. **Is it ML** (autodiff/training/GPU)? → Candle or another ML framework.
5. **Are you already in Python?** → NumPy/Pandas, with `matten` as the upstream
   Rust producer if useful.
6. **None of the above, or the work is small?** → **stay with `matten`.**

## Two Rust targets are covered now

This revision ships full playbooks for the two Rust numeric targets the project's
benchmarking program directly compares against:

- [`ndarray`](./playbooks/ndarray.md) — general N-D arrays.
- [`nalgebra`](./playbooks/nalgebra.md) — dense linear algebra.

The Polars/Pandas, Candle, and NumPy playbooks arrive in a later revision; this page already
points you at the right ecosystem in the meantime.
