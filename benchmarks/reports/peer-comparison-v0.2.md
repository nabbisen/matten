# Rust peer comparison — v0.2

**Report ID:** `matten-rfc049-rust-peer-comparison-v0.2`
**Status:** Maintainer-run refresh at workspace `0.28.1`, under the **unchanged** RFC-049 Phase 2
methodology. The methodology and the Phase 2 program are architect-accepted (see v0.1); these
refreshed numbers are a maintainer run and are **not** separately architect-reviewed. The accepted
reference report remains [`peer-comparison-v0.1.md`](./peer-comparison-v0.1.md).

> **These results are workload-specific and environment-specific. They are a Rust peer comparison
> for positioning, not a competitor ranking, and not a "faster than X" claim.**

This is the RFC-049 **Phase 2** Rust peer comparison. For a fixed set of small, equivalent tasks it
places `matten` next to two established Rust numeric crates, `ndarray` and `nalgebra`, using each
library's natural representation. The goal is to show *where* `matten`'s approachable `Tensor` API
sits — including where it is slower but acceptable — not to rank libraries. It does **not** include
NumPy / Pandas / SciPy / Candle / GPU comparisons (those are Phase 3, not authorized).

## How to regenerate

See [the harness README](../README.md#how-to-regenerate-with-environment-capture) for the
environment-capture snippet. Peer benches are opt-in and never built in the default harness or
ordinary CI:

```bash
# compile-check only (what the separate peers workflow runs)
cargo bench --manifest-path benchmarks/Cargo.toml --features peers --bench peers --no-run

# run the comparison (maintainer machine, default Criterion settings)
cargo bench --manifest-path benchmarks/Cargo.toml --features peers --bench peers -- --noplot
```

Each task reports three medians under `peers/<task>/{matten,ndarray,nalgebra}`.

## Environment

Same machine class as the internal baseline refresh. Not comparable across machines.

| Field | Value |
|---|---|
| OS / kernel | Ubuntu 26.04 LTS / 7.0.0-22-generic |
| CPU (note if virtualized) | 8 vCPU, AMD Ryzen-based (**virtualized**) |
| RAM | 7,600,940 kB (~7.25 GiB) |
| rustc / cargo | 1.93.1 / 1.93.1 |
| target | x86_64-unknown-linux-gnu |
| profile | bench (opt-level 3) |
| Criterion settings | defaults, `--noplot` |
| git commit | ef06369 |
| workspace version | 0.28.1 |
| ndarray version | 0.16.1 |
| nalgebra version | 0.33.3 |

The peer harness keeps its own (git-ignored) lockfile and still resolves `ndarray 0.16.1`,
independent of the workspace's `ndarray` requirement (which moved to `0.17` in v0.28.x). The peer
comparison is therefore still "vs `ndarray 0.16.1`"; re-pinning the peer harness to `ndarray 0.17`
is a separate task and not part of this refresh. `nalgebra 0.33.3` is the latest version compatible
with the project's Rust 1.85 floor (`nalgebra 0.35.0` requires Rust 1.89).

## Comparable tasks

Every task below is the *same small mathematical problem* at the same sizes, expressed in each
library's native type (`matten::Tensor`, `ndarray::Array1/Array2`, `nalgebra::DVector/DMatrix`). All
six are small dense vector/matrix operations. The comparison uses each library's natural
representation, so it compares task-level workflow cost rather than identical internal
implementation strategy. Figures are Criterion medians from one run on the machine above.

| Task | Problem | matten | ndarray | nalgebra | What is *not* compared |
|---|---|---|---|---|---|
| cosine_similarity | dot / (‖a‖·‖b‖), len 512 | 1.18 µs | 356 ns | 282 ns | N-D, broadcasting |
| matmul | 64×64 dense product | 155.3 µs | 18.34 µs | 20.51 µs | strided/large/blocked matmul |
| markov_step | v·P, n=64 | 1.61 µs | 1.82 µs | 3.51 µs | sparse transition matrices |
| pagerank_step | M·r + damping, n=64 | 10.66 µs | 1.24 µs | 939 ns | sparse graphs, convergence loop |
| linreg_gd_step | one GD step, m=256 | 2.61 µs | 1.19 µs | 1.15 µs | full training loop, solvers |
| heat_step | operator·u, n=64 | 8.77 µs | 882 ns | 831 ns | stencil/sparse operators |

## Interpretation

Modest and task-scoped, in the "positioning, not ranking" register, and consistent with the
accepted v0.1 report. On these small dense kernels the production-oriented peers generally have less
overhead than `matten`'s approachable `Tensor` API — expected, and consistent with `matten`'s
DX-first role. The size of the difference varies by task and is the useful signal:

- **Smallest / inverted — `markov_step` (v·P).** `matten`'s median (1.61 µs) was lower than both
  peers' here (`ndarray` 1.82 µs, `nalgebra` 3.51 µs). One small task at one size, not a ranking —
  but it shows the gap is not uniform.
- **Modest — `cosine_similarity` and `linreg_gd_step`.** Same order of magnitude, roughly 2–3× the
  peers at these sizes.
- **Widest — `matmul`, `pagerank_step`, `heat_step`.** Roughly 8–11× the peers. These are the dense
  `matmul` and matrix×vector paths.

The consistent internal pattern holds: `matten`'s **matrix×vector** path (`pagerank_step`,
`heat_step`) shows the widest gap, while its **vector×matrix** path (`markov_step`) is competitive.
This echoes the internal baseline's axis-reduction watch item and is recorded as positioning /
regression-visibility information, **not** a defect.

Migration framing is unchanged: when dense `matmul`, matrix×vector, or operator-style hot paths
become production bottlenecks, that part of a workflow is a candidate to move to `ndarray` or
`nalgebra` (via the bridge or a manual port) while keeping `matten` as the PoC/reference. No library
is "better" in general; this is a positioning snapshot, not a verdict.

## Limitations

Single **virtualized** machine, small fixed sizes, microbenchmark methodology, medians over noisy
samples (outlier rates ran ~1–27% per task on this VM this run; `linreg_gd_step`, `heat_step`, and
`cosine_similarity` were the noisiest). These are library-natural representations of *these* tasks
only; a different task, size, or representation could shift every number. Not comparable to figures
from a different machine or OS, and not a general ranking.
