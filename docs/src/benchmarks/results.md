# Benchmark results

This page is the **reader's view**: a curated summary of `matten`'s benchmark results so they are
readable from inside the book. It is a small representative selection, not the full matrix — the
complete numbers, environment details, and regeneration steps live in the reports under
`benchmarks/reports/`. If you want to *run* the benchmarks, see the
[methodology](./methodology.md) and the harness `README.md`.

> **These numbers are workload-specific and environment-specific.** They were produced on one
> virtualized machine with microbenchmark methodology. They are a positioning and
> regression-visibility reference — **not a ranking, and not a "faster than X" claim.** `matten`
> optimizes for time to a runnable PoC, not benchmark leadership.

The numbers below are the **v0.2 maintainer refresh at workspace `0.28.1`**, produced under the
unchanged RFC-049 methodology. The architect-accepted reference baseline is v0.1 (see the reports);
the figures match v0.1 within run-to-run VM variance.

## Phase 1 — internal baseline

`matten` measured against itself, to establish a reference point and make future regressions
visible (RFC-049 Phase 1).

- **Baseline ID:** `matten-rfc049-internal-baseline-v0.2` — maintainer refresh at v0.28.1
  (reference: `…-v0.1`, accepted 2026-06-24).
- **Environment:** Ubuntu 26.04, 8 vCPU AMD (virtualized), rustc 1.93.1, profile `bench`
  (opt-level 3), Criterion defaults; git `ef06369`, workspace `0.28.1`. Not comparable across
  machines.

Representative medians (full table in the report):

| Workload | Time (median) |
|---|---|
| construction (4096-element vector) | ~0.99 µs |
| elementwise add (4096 elements) | ~10.5 µs |
| `matmul` (64×64) | ~78 µs |
| `sum_axis` + `mean_axis` (64×64, combined) | ~1.31 ms |
| cosine similarity (len 512) | ~882 ns |
| linear-regression GD step (m=256) | ~1.80 µs |

Peak RSS was **not captured** in this refresh (the VM lacked GNU `/usr/bin/time`); it is
informative-only and never a gate. The accepted v0.1 baseline recorded ~44 MiB for the full
scenario run under the same methodology, dominated by Criterion's own footprint rather than the
small tensors.

The clearest signal is that **axis reductions are currently `matten`'s most expensive core path** —
the combined `sum_axis`/`mean_axis` workload (~1.31 ms) is roughly 400× the whole-tensor
`sum`/`mean` (~3.23 µs) and ~17× a 64×64 `matmul`. This is recorded as positioning /
regression-visibility information, not a defect: it is the natural first place to look if
axis-reduction cost ever matters for your workload.

## Phase 2 — Rust peer comparison

The same small problems placed next to two established Rust numeric crates, `ndarray` and
`nalgebra`, each in its native type (RFC-049 Phase 2). This shows *where* `matten`'s approachable
`Tensor` API sits — including where it is slower but acceptable — **not** a ranking of libraries.

- **Report ID:** `matten-rfc049-rust-peer-comparison-v0.2` — maintainer refresh at v0.28.1
  (reference: `…-v0.1`, accepted 2026-06-25).
- **Environment:** same machine class as the baseline; git `ef06369`, workspace `0.28.1`,
  `ndarray` 0.16.1, `nalgebra` 0.33.3. Peer tasks are opt-in behind the `peers` feature (off by
  default). **These numbers were measured at `ndarray 0.16.1`;** the harness peer pin was bumped to
  `ndarray 0.17` in v0.28.3 to match the bridge, and will produce `0.17` figures on the next peers
  run. Not comparable across machines.

Representative Criterion medians (full six-task table in the report):

| Task | matten | ndarray | nalgebra |
|---|---|---|---|
| markov step (v·P, n=64) | ~1.61 µs | ~1.82 µs | ~3.51 µs |
| cosine similarity (len 512) | ~1.18 µs | ~356 ns | ~282 ns |
| `matmul` (64×64) | ~155.3 µs | ~18.3 µs | ~20.5 µs |
| heat step (operator·u, n=64) | ~8.77 µs | ~882 ns | ~831 ns |

On these small dense kernels the production-oriented peers generally carry less overhead than
`matten`'s `Tensor` API — expected, and consistent with `matten`'s DX-first role. The size of the
gap is the useful part, and it is not uniform: a vector×matrix step (markov) is competitive here —
ahead of both peers at this size — while dense `matmul` and matrix×vector steps (heat, pagerank)
show the widest gaps (~8–11×). A consistent internal pattern is that `matten`'s matrix×vector path
is its widest gap while its vector×matrix path is competitive — echoing the axis-reduction signal
from Phase 1.

## Read next

- [Methodology](./methodology.md) — what is measured, what is not, and the rules that keep the
  program honest.
- Full reports with complete tables, environment, and regeneration commands:
  `benchmarks/reports/internal-baseline-v0.2.md` and `benchmarks/reports/peer-comparison-v0.2.md`
  (and the accepted v0.1 references alongside them).

Phases 3 (NumPy/Pandas reference) and 4 (regression gates) are designed in RFC-049 but deferred and
not yet measured.
