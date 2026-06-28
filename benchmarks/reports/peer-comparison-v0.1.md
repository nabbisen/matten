# Rust peer comparison — v0.1

**Report ID:** `matten-rfc049-rust-peer-comparison-v0.1`
**Architect status:** Accepted as RFC-049 Phase 2 official Rust peer comparison v0.1 (2026-06-25).

> **For current numbers see [`peer-comparison-v0.2.md`](./peer-comparison-v0.2.md)** (a
> maintainer-run refresh at v0.28.1). This v0.1 report is retained as the **architect-accepted**
> reference report; the methodology is unchanged between the two.

> **Report status:** Official maintainer-run numbers (commit `007031c`, workspace v0.22.6,
> same machine class as the accepted internal baseline), accepted by architect ruling
> 2026-06-25.

> **These results are workload-specific and environment-specific. They are a Rust peer
> comparison for positioning, not a competitor ranking, and not a "faster than X" claim.**

This is the RFC-049 **Phase 2** Rust peer comparison. For a fixed set of small, equivalent
tasks it places `matten` next to two established Rust numeric crates, `ndarray` and
`nalgebra`, using each library's natural representation. The goal is to show *where*
`matten`'s approachable `Tensor` API sits — including where it is slower but acceptable —
not to rank libraries. It does **not** include NumPy / Pandas / SciPy / Candle / GPU
comparisons (those are Phase 3, not authorized).

## How to regenerate

Peer benches are opt-in and never built in the default harness or ordinary CI:

```bash
# compile-check only (what the separate peers workflow runs)
cargo bench --manifest-path benchmarks/Cargo.toml --features peers --bench peers --no-run

# run the comparison (maintainer machine, default Criterion settings)
cargo bench --manifest-path benchmarks/Cargo.toml --features peers --bench peers -- --noplot
```

Each task reports three medians under `peers/<task>/{matten,ndarray,nalgebra}`.

## Environment

Same machine class as the accepted internal baseline. Not comparable across machines.

| Field | Value |
|---|---|
| OS / kernel | Ubuntu 26.04 LTS / 7.0.0-22-generic |
| CPU (note if virtualized) | 8 vCPU, AMD Ryzen-based (**virtualized**) |
| RAM | 7,600,956 kB (~7.25 GiB) |
| rustc / cargo | 1.93.1 / 1.93.1 |
| target | x86_64-unknown-linux-gnu |
| profile | bench (opt-level 3) |
| Criterion settings | defaults, `--noplot` |
| git commit | 007031c |
| workspace version | 0.22.6 |
| ndarray version | 0.16.1 |
| nalgebra version | 0.33.3 |

`nalgebra 0.33.3` is selected because Cargo resolves dependencies compatible with the
project's Rust 1.85 compatibility floor (the MSRV-aware resolver locked the peer deps to the
latest 1.85-compatible versions). `nalgebra 0.35.0` requires Rust 1.89, so a future bump
would need an explicit MSRV-policy decision — it is **not** a constraint of any particular
maintainer toolchain.

## Comparable tasks

Every task below is the *same small mathematical problem* at the same sizes, expressed in
each library's native type (`matten::Tensor`, `ndarray::Array1/Array2`,
`nalgebra::DVector/DMatrix`). All six are small dense vector/matrix operations, which is
why both peers cover all six. The comparison uses each library's natural representation, so
it compares task-level workflow cost rather than identical internal implementation strategy.
Figures are Criterion medians from one run on the machine above; the `matten` column is a
separate run from the internal baseline, so small differences vs the baseline are run-to-run
VM variance.

| Task | Problem | matten | ndarray | nalgebra | What is *not* compared |
|---|---|---|---|---|---|
| cosine_similarity | dot / (‖a‖·‖b‖), len 512 | 674 ns | 231 ns | 160 ns | N-D, broadcasting |
| matmul | 64×64 dense product | 118.9 µs | 12.74 µs | 16.23 µs | strided/large/blocked matmul |
| markov_step | v·P, n=64 | 1.03 µs | 1.34 µs | 1.41 µs | sparse transition matrices |
| pagerank_step | M·r + damping, n=64 | 6.21 µs | 607 ns | 607 ns | sparse graphs, convergence loop |
| linreg_gd_step | one GD step, m=256 | 1.75 µs | 769 ns | 832 ns | full training loop, solvers |
| heat_step | operator·u, n=64 | 5.23 µs | 556 ns | 565 ns | stencil/sparse operators |

## Interpretation

Modest and task-scoped, in the "positioning, not ranking" register. On these small dense
vector/matrix kernels the production-oriented peers generally have less overhead than
`matten`'s approachable `Tensor` API — which is expected and consistent with `matten`'s
DX-first role. The size of the difference varies by task and is the useful signal:

- **Smallest / inverted — `markov_step` (v·P).** At this size `matten`'s median (1.03 µs)
  was slightly lower than both peers' (`ndarray` 1.34 µs, `nalgebra` 1.41 µs). This is one
  small task at one size, not a ranking — but it shows the gap is not uniform.
- **Modest — `cosine_similarity` and `linreg_gd_step`.** `matten` is in the same order of
  magnitude, roughly 2–4× the peers at these sizes.
- **Widest — `matmul`, `pagerank_step`, `heat_step`.** Roughly 7–10× the peers. These are
  the dense `matmul` and matrix×vector paths.

A consistent internal pattern is worth recording: `matten`'s **matrix×vector** path
(`pagerank_step`, `heat_step`) shows the widest gap (~9–10×), while its **vector×matrix**
path (`markov_step`) is competitive. This echoes the internal baseline's axis-reduction
watch item and is recorded as positioning / regression-visibility information, **not** a
defect — `matten` is DX-first, not a performance crate.

Migration framing: when dense `matmul`, matrix×vector, or operator-style hot paths become
production bottlenecks, that part of a workflow is a candidate to move to `ndarray` or
`nalgebra` (via a bridge or manual port) while keeping `matten` as the PoC/reference. Where
the workload is light or vector×matrix-shaped at small sizes, `matten` is competitive. No
library is "better" in general; this is a positioning snapshot, not a verdict.

## Limitations

Single **virtualized** machine, small fixed sizes, microbenchmark methodology, medians over
noisy samples (outlier rates ran ~1–23% per task on this VM; `pagerank_step/matten` and
`cosine_similarity` were the noisiest). These are library-natural representations of *these*
tasks only; a different task, size, or representation could shift every number. Not
comparable to figures from a different machine or OS, and not a general ranking.
