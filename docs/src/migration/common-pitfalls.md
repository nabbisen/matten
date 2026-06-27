# Common pitfalls

A few mistakes come up repeatedly when moving data out of `matten`. None are hard to avoid
once you know to look for them.

## Memory order: `matten` is row-major

`matten` stores tensor data in **row-major** (C order) logical layout. Some targets differ:
`nalgebra`'s `DMatrix` is **column-major**. If you hand a flat `Vec<f64>` to a column-major
constructor as if it were column-major, you will silently transpose your data. Always use a
constructor that interprets the source order explicitly (for example
`nalgebra::DMatrix::from_row_slice`, which reads row-major source), or transpose
deliberately. The per-target playbooks show the correct constructor for each.

## Conversions copy — plan for it

Both directions of a bridge conversion **copy** the underlying data. That is the right
default for safety, but it means converting inside a tight loop is wasteful. Convert
**once**, at the boundary between "build/ingest in `matten`" and "compute in the specialised
crate", not on every iteration.

```text
do:    build in matten → convert once → run the hot loop in the target
avoid: convert ↔ on every iteration of the hot loop
```

## `f64` vs other dtypes

`matten` tensors are `f64`. Targets that want `f32` (common in ML, e.g. Candle) need an
explicit conversion, which is another copy and a precision change. Decide this at the
boundary and do it once.

## Dynamic tensors must be made numeric first

If your data came through the `dynamic` feature (the messy-data on-ramp), it may hold
non-numeric or missing elements. Bridges reject dynamic tensors rather than guess. Resolve
to a numeric tensor first (fill or drop missing values, then `try_numeric()`), and only then
convert. See the [dynamic reference](../reference/dynamic.md).

## Don't expect `matten-data` to grow dataframe features

`matten-data` is an ingestion on-ramp (CSV/table → `Tensor`). If you find yourself wanting
group-by, joins, pivots, or query expressions, that is a signal to move the tabular work to
Polars or Pandas — not a gap to be filled in `matten-data`. It will not grow those features.

## Migrate the kernel, keep the glue

The goal is rarely to rewrite everything. Keep `matten` for construction, ingestion, and
glue; move only the measured hot kernel. A migration that replaces your whole program is
usually a sign of over-migrating.


