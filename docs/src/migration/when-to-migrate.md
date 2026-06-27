# When to migrate

The honest default is: **stay with `matten` until a concrete signal tells you to move.**
`matten` is built for PoC, learning, and small serious workflows, and most of those never
need to leave. Migration is a deliberate response to pressure, not a rite of passage.

## Signals that you have outgrown `matten`

Treat any of these as a real reason to plan a migration of the affected part of your
workflow:

- **Data-size pressure.** Your arrays are large enough that `matten`'s copy-on-every-
  reshape/slice behavior shows up in profiles, or you are pushing past comfortable
  in-memory sizes.
- **Runtime pressure.** A dense numeric kernel (matrix multiply, matrix–vector products,
  operator application) is a measured hot path. In the accepted RFC-049 Rust peer
  comparison, dense `matmul` and matrix–vector tasks showed a noticeably larger gap to
  `ndarray`/`nalgebra` than lighter vector tasks did — so those are the kernels most worth
  moving when they get hot.
- **Linear-algebra pressure.** You need decompositions (LU, QR, SVD), solvers, or
  eigenvalues. `matten` intentionally does not provide these.
- **Dataframe pressure.** You need group-by, joins, pivots, or query-style operations.
  `matten-data` is an *ingestion on-ramp* (CSV/table → `Tensor`) and will **not** grow into
  a dataframe engine.
- **ML / device pressure.** You need autodiff, training loops, or GPU execution.
- **Dynamic-ingestion pressure.** You are leaning heavily on the `dynamic` feature for
  large or repeated messy-data cleanup, beyond a one-time on-ramp.

## Signals that you should *stay*

Equally important — these are reasons **not** to migrate:

- The numeric work is small and not on a hot path.
- You are wiring data into a web API (serde in, serde out) with light math in between.
- You are learning, prototyping, or teaching, and approachability matters more than raw
  speed.
- Your messy data needs a one-time clean-then-compute pass, which `matten`'s `dynamic`
  on-ramp handles.

If none of the pressure signals above apply, staying with `matten` is the right call, and
adding a heavyweight dependency would cost you simplicity for no real gain.

## Migrate the hot path, not the whole program

Migration is rarely all-or-nothing. The common, healthy pattern is to keep `matten` for
construction, ingestion, and glue, and move only the measured hot kernel into a specialised
crate:

```text
matten            →  build / ingest / shape the data, light math
specialised crate →  the heavy kernel (matmul, decomposition, training, group-by)
```

The [target-selection matrix](./target-selection.md) helps you map each pressure signal to
a destination, and the [playbooks](./playbooks/index.md) show the per-target mechanics.
