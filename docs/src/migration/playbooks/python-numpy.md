# Migrating to NumPy (Python scientific stack)

[NumPy](https://numpy.org) is the foundation of the Python scientific ecosystem (SciPy,
scikit-learn, Pandas, and most ML tooling sit on top of it). Move here when the workflow's
center of gravity is **Python**, or when you need a library that only exists in that
ecosystem.

This is a **cross-language** boundary, so it is **manual/conceptual**: there is no automatic
Rust↔Python conversion, and `matten` does not provide one. The realistic pattern is to use
`matten` as an upstream Rust producer and hand the data to Python through a serialization
format.

## Choose this target when

- Your team or downstream pipeline is **in Python**.
- You need a Python-only library (SciPy, scikit-learn, a specific ML/stats package).
- The numeric work belongs next to Python data tooling rather than in a Rust binary.

## Do not choose this target when

- You want to stay in Rust → `ndarray` (general arrays) or `nalgebra` (linear algebra).
- The work is small and already lives happily in `matten`.

## Concept mapping

| `matten` (Rust) | NumPy (Python) |
|---|---|
| `Tensor` (`f64`, row-major) | `numpy.ndarray` (default C/row-major) |
| `tensor.shape()` | `array.shape` |
| `tensor.to_vec()` / `into_vec()` (flat row-major) | `array.ravel()` / `array.reshape(..)` |
| `.matmul(&b)` | `a @ b` |
| axis reductions | `a.sum(axis=..)` / `a.mean(axis=..)` |

## Example migrations

- Any numeric example (e.g. `35_linear_regression_gradient_descent`,
  `36_heat_equation_1d`) → reimplement in NumPy when the surrounding pipeline is Python; the
  row-major layout and shape transfer directly.

## Conversion path

Hand data across the language boundary via a serialization format. The flat data is
row-major, which matches NumPy's default, so only the shape needs to travel with it:

```text
matten (Rust):   tensor.to_vec()  +  tensor.shape()
   ↓  write to a shared format (CSV, JSON, or .npy / Arrow for larger data)
NumPy (Python):  np.loadtxt(...)   /  np.load("data.npy").reshape(shape)
```

For small data, CSV/JSON is simplest; for larger or repeated transfers, a binary format
(`.npy`, or Arrow) avoids text parsing overhead. There is no in-process bridge — the two
runtimes do not share memory here.

## Common pitfalls

- **No automatic bridge.** Plan an explicit serialization step; do not expect
  in-process conversion between Rust and Python.
- **Carry the shape.** The flat buffer is row-major (NumPy's default), but you must
  reattach the shape on the Python side.
- **`f64` everywhere in `matten`.** If the Python side wants `float32`, cast there.

## Performance / positioning notes

There is **no `matten`-vs-NumPy benchmark**, and one would be a cross-language RFC-049
Phase 3 comparison, which is **not authorized**. NumPy is C/BLAS-backed and fast on dense
numeric work, but the reason to migrate here is usually **ecosystem and language fit**, not
a measured speed comparison against `matten`.

## Minimal checklist

- [ ] The workflow's home is Python (team, pipeline, or a Python-only library).
- [ ] You have a concrete serialization hand-off (CSV/JSON for small, `.npy`/Arrow for
      larger) and you carry the shape across.
- [ ] You are not expecting an in-process Rust↔Python conversion.
