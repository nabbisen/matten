# Choosing a target

There is no universally "best" target — the right destination depends on the *shape* of the
pressure you are feeling. Use the matrix below to map a signal to an ecosystem, then open
the matching [playbook](./playbooks/index.md).

## Target-selection matrix

| Pressure / need | Recommended target | Notes |
|---|---|---|
| General N-D numeric arrays, dense `matmul`, axis reductions at scale | **`ndarray`** | The general Rust N-D array production path; BLAS-backed matmul available. |
| Small/mid dense vectors & matrices, decompositions, solvers (LU/QR/SVD), eigenvalues | **`nalgebra`** | The dense linear-algebra path. |
| Group-by, joins, pivots, query-style dataframe analytics | **[Polars](./playbooks/polars-and-pandas.md)** (Rust) / **Pandas** (Python) | `matten-data` is an ingestion on-ramp only; it will **not** grow these. |
| Autodiff, training loops, GPU/device execution | **[Candle](./playbooks/candle.md)** (Rust) / framework of choice | `matten` is not an ML framework. |
| Existing Python scientific stack, NumPy interop | **[NumPy](./playbooks/python-numpy.md)** (Python) | Manual/conceptual hand-off; no automatic bridge. |
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

## Playbooks

Full per-target playbooks are available for every destination above:
[`ndarray`](./playbooks/ndarray.md), [`nalgebra`](./playbooks/nalgebra.md),
[Polars/Pandas](./playbooks/polars-and-pandas.md), [Candle](./playbooks/candle.md), and
[NumPy](./playbooks/python-numpy.md). The two Rust array/linalg targets carry task-scoped
positioning notes from the accepted RFC-049 peer comparison; the dataframe, ML, and
Python targets are different paradigms with no such benchmark (see each playbook).
