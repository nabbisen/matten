# Benchmark results

This page surfaces a **curated summary** of `matten`'s accepted benchmark results so they are
readable from inside the book. It is a small representative selection, not the full matrix — the
complete numbers, environment details, and regeneration steps live in the reports under
`benchmarks/reports/`.

> **These numbers are workload-specific and environment-specific.** They were produced on one
> virtualized machine with microbenchmark methodology. They are a positioning and
> regression-visibility reference — **not a ranking, and not a "faster than X" claim.** `matten`
> optimizes for time to a runnable PoC, not benchmark leadership.

## Phase 1 — internal baseline

`matten` measured against itself, to establish a reference point and make future regressions
visible (RFC-049 Phase 1).

- **Baseline ID:** `matten-rfc049-internal-baseline-v0.1` — accepted 2026-06-24.
- **Environment:** Ubuntu, 8 vCPU AMD (virtualized), rustc 1.93.1, profile `bench` (opt-level 3),
  Criterion defaults; git `387d2b0`, workspace `0.22.3`. Not comparable across machines.

Representative medians (full table in the report):

| Workload | Time (median) |
|---|---|
| construction (4096-element vector) | ~0.99 µs |
| elementwise add (4096 elements) | ~10.4 µs |
| `matmul` (64×64) | ~77.5 µs |
| `sum_axis` + `mean_axis` (64×64, combined) | ~1.31 ms |
| cosine similarity (len 512) | ~642 ns |
| linear-regression GD step (m=256) | ~1.79 µs |

Peak RSS over the full scenario run was ~44 MiB, dominated by Criterion's own footprint rather than
the small tensors.

The clearest signal is that **axis reductions are currently `matten`'s most expensive core path** —
the combined `sum_axis`/`mean_axis` workload (~1.31 ms) is roughly 400× the whole-tensor
`sum`/`mean` (~3.25 µs) and ~17× a 64×64 `matmul`. This is recorded as positioning /
regression-visibility information, not a defect: it is the natural first place to look if
axis-reduction cost ever matters for your workload.

## Phase 2 — Rust peer comparison

The same small problems placed next to two established Rust numeric crates, `ndarray` and
`nalgebra`, each in its native type (RFC-049 Phase 2). This shows *where* `matten`'s approachable
`Tensor` API sits — including where it is slower but acceptable — **not** a ranking of libraries.

- **Report ID:** `matten-rfc049-rust-peer-comparison-v0.1` — accepted 2026-06-25.
- **Environment:** same machine class as the baseline; git `007031c`, workspace `0.22.6`,
  `ndarray` 0.16.1, `nalgebra` 0.33.3. Peer tasks are opt-in behind the `peers` feature (off by
  default). Not comparable across machines.

Representative Criterion medians (full six-task table in the report):

| Task | matten | ndarray | nalgebra |
|---|---|---|---|
| markov step (v·P, n=64) | ~1.03 µs | ~1.34 µs | ~1.41 µs |
| cosine similarity (len 512) | ~674 ns | ~231 ns | ~160 ns |
| `matmul` (64×64) | ~118.9 µs | ~12.7 µs | ~16.2 µs |
| heat step (operator·u, n=64) | ~5.23 µs | ~556 ns | ~565 ns |

On these small dense kernels the production-oriented peers generally carry less overhead than
`matten`'s `Tensor` API — expected, and consistent with `matten`'s DX-first role. The size of the
gap is the useful part, and it is not uniform: a vector×matrix step (markov) is competitive here,
while dense `matmul` and matrix×vector steps (heat, pagerank) show the widest gaps (~7–10×). A
consistent internal pattern is that `matten`'s matrix×vector path is its widest gap while its
vector×matrix path is competitive — echoing the axis-reduction signal from Phase 1.

## Read next

- [Methodology](./methodology.md) — what is measured, what is not, and the rules that keep the
  program honest.
- Full reports with complete tables, environment, and regeneration commands:
  `benchmarks/reports/internal-baseline-v0.1.md` and `benchmarks/reports/peer-comparison-v0.1.md`.

Phases 3 (NumPy/Pandas reference) and 4 (regression gates) are designed in RFC-049 but deferred and
not yet measured.
