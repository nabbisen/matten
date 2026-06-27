# Target playbooks

Each playbook is a step-by-step guide for moving a `matten` workflow to one specific
ecosystem. They share a common structure: when to choose (and not choose) the target, how
`matten` concepts map onto it, worked example migrations drawn from the
[examples](../../examples/index.md), the conversion path, pitfalls, task-scoped positioning
notes, and a minimal checklist.

## Available now

- [`ndarray`](./ndarray.md) — general Rust N-D arrays; the first stop for dense numeric
  workloads at scale, with a contract-backed bridge crate (`matten-ndarray`).
- [`nalgebra`](./nalgebra.md) — dense linear algebra: vectors, matrices, decompositions,
  and solvers.
- [Polars / Pandas](./polars-and-pandas.md) — dataframe analytics (group-by, joins, pivots,
  query). `matten-data` is an on-ramp and will not grow these.
- [Candle](./candle.md) — ML tensors, training, and device execution — without implying
  `matten` is an ML framework.
- [NumPy](./python-numpy.md) — the Python scientific path, as a manual/conceptual hand-off.

## Decision tree

```text
measured dense numeric hot path?
├─ general N-D arrays / BLAS matmul / axis reductions   → ndarray
└─ small/mid dense linear algebra, decompositions       → nalgebra

need LU / QR / SVD / solvers / eigenvalues?             → nalgebra
need group-by / join / pivot / query?                   → Polars (Rust) / Pandas (Python)
need autodiff / training / GPU?                          → Candle / ML framework
already in Python / NumPy ecosystem?                     → NumPy (matten as upstream producer)
small work, ingestion, glue, learning?                   → stay with matten
```

If you are unsure whether you have outgrown `matten` at all, start with
[When to migrate](../when-to-migrate.md).
