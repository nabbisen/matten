# Internal baseline — v0.1

> **These results are workload-specific and environment-specific. They are for
> positioning and regression visibility, not universal ranking.**

**Baseline ID:** `matten-rfc049-internal-baseline-v0.1`
**Architect status:** Accepted as RFC-049 Phase 1 internal baseline v0.1 (2026-06-24).

This is the Phase 1 internal Rust baseline (RFC-049). It measures `matten` against
itself, to establish a reference point and make future regressions visible. It does
**not** compare against `ndarray`, `nalgebra`, NumPy, or Pandas — those are Phases 2–3
and are not yet implemented.

> This is the maintainer-run baseline that satisfies
> [`BASELINE-READY-CHECKLIST.md`](./BASELINE-READY-CHECKLIST.md). Once accepted, it is the
> artifact that unlocks RFC-049 Phase 2 authorization.

## How to regenerate

```bash
# timings (default Criterion settings: 100 samples, default warm-up/measurement time)
cargo bench --manifest-path benchmarks/Cargo.toml --bench core      -- --noplot
cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
# optional dynamic micro-workload
cargo bench --manifest-path benchmarks/Cargo.toml --bench core --features dynamic -- --noplot

# peak RSS (Linux)
/usr/bin/time -v cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
```

## Environment

Numbers below are specific to this machine and not comparable across machines.

| Field | Value |
|---|---|
| OS | Ubuntu 26.04 LTS |
| Kernel | 7.0.0-22-generic |
| CPU | 8 vCPU, AMD Ryzen-based (**virtualized**) |
| RAM | 7,600,956 kB (~7.25 GiB) |
| rustc | 1.93.1 (01f6ddf75 2026-02-11) |
| cargo | 1.93.1 (083ac5135 2025-12-15) |
| target | x86_64-unknown-linux-gnu |
| profile | bench (opt-level 3) |
| Criterion settings | defaults (100 samples), `--noplot` |
| git commit | 387d2b0 |
| workspace version | 0.22.3 |
| peak RSS tool | `/usr/bin/time -v` |

## What was measured

- Core micro set: construction, reshape/flatten, elementwise add/mul, broadcasting,
  `sum`/`mean`, `sum_axis`/`mean_axis`, `matmul`, slice, and (under `--features dynamic`)
  the dynamic `try_numeric` ingestion path.
- Scenario set: cosine similarity, Markov step, PageRank step, linear-regression GD
  step, heat-equation step.

## What was not measured

- Peer libraries (`ndarray`/`nalgebra`) — Phase 2, deferred.
- Cross-language references (NumPy/Pandas) — Phase 3, deferred.
- Allocation-level memory — not instrumented in Phase 1 (peak RSS only).

## Core baseline

Medians from the default-feature run; `dynamic_try_numeric` from a `--features dynamic`
run. Re-runs agreed within Criterion's noise threshold.

| Workload | Time (median) | Notes |
|---|---|---|
| core/construction | 988 ns | 4096-element vector |
| core/reshape_flatten | 1.01 µs | |
| core/elementwise_add | 10.36 µs | 4096 elements |
| core/elementwise_mul | 10.35 µs | 4096 elements |
| core/broadcasting | 20.01 µs | [64,64] + [64] |
| core/sum_mean | 3.25 µs | 4096-element vector |
| core/sum_mean_axis | 1.31 ms | 64×64; measures the **combined** cost of `sum_axis(0)` and `mean_axis(0)` in one workload body; slowest core op (Criterion flagged the 5 s window as tight) |
| core/matmul | 77.53 µs | 64×64 |
| core/slice_rows | 84.32 µs | first 8 rows of 64×64 |
| core/dynamic_try_numeric | 36.54 µs | 4096 elements; requires `--features dynamic` |

## Scenario baseline

| Workload | Time (median) | Notes |
|---|---|---|
| scenario/cosine_similarity | 642 ns | length 512 |
| scenario/markov_step | 713 ns | n = 64 |
| scenario/pagerank_step | 5.32 µs | n = 64 |
| scenario/linreg_gd_step | 1.79 µs | m = 256 |
| scenario/heat_step | 5.21 µs | n = 64 |

## Peak RSS

`Maximum resident set size: 44,728 kB (~44 MiB)`, measured with `/usr/bin/time -v` over the
full `scenarios` bench run. This is the whole `cargo`+Criterion process across all five
scenario workloads, so it is dominated by Criterion's own footprint rather than the small
tensors — coarse, but adequate for a Phase 1 reference point per the methodology.

## Interpretation

Modest, tied strictly to these workloads on this machine:

- The cheapest operations are construction (~1 µs) and the small scenario steps — cosine
  similarity (~640 ns) and a Markov step (~710 ns) — confirming the small famous-problem
  iterations are inexpensive per step.
- Elementwise add/mul over 4096 elements sit around ~10 µs; broadcasting a row over a
  64×64 matrix ~20 µs; a 64×64 `matmul` ~78 µs and an 8-row slice ~84 µs.
- The clearest signal is **`sum_mean_axis` at ~1.31 ms** — roughly **400× the whole-tensor
  `sum_mean`** (~3.25 µs) and **~17× a 64×64 `matmul`**. Axis reductions are currently
  `matten`'s most expensive core path by a wide margin, and the natural first place to look
  if axis-reduction cost ever matters. This is recorded as positioning / regression-visibility
  information, not a defect: `matten` is a DX-first crate, not a performance crate.
- `dynamic_try_numeric` (~36.5 µs for 4096 elements) costs roughly 3.5× an elementwise op —
  reasonable for the dynamic ingestion-and-conversion path.

No cross-library or "faster than X" claim is made or implied.

## Limitations

Single **virtualized** machine (8 vCPU VM), small inputs, microbenchmark methodology.
Outlier rates were moderate (≈5–13% per workload), consistent with a VM; medians are used
throughout. Peak RSS is whole-process and coarse. These numbers describe `matten`'s own
behavior on these workloads on this machine; they are not a cross-library ranking and are
not comparable to figures from a different machine or OS.
