# Internal baseline — v0.2

> **These results are workload-specific and environment-specific. They are for
> positioning and regression visibility, not universal ranking.**

**Baseline ID:** `matten-rfc049-internal-baseline-v0.2`
**Status:** Maintainer-run refresh at workspace `0.28.3`, under the **unchanged** RFC-049 Phase 1
methodology. The methodology and the Phase 1 program are architect-accepted (see v0.1); these
refreshed numbers are a maintainer run and are **not** separately architect-reviewed. The accepted
reference baseline remains [`internal-baseline-v0.1.md`](./internal-baseline-v0.1.md).

This is the Phase 1 internal Rust baseline (RFC-049). It measures `matten` against itself, to
establish a reference point and make future regressions visible. It does **not** compare against
`ndarray`, `nalgebra`, NumPy, or Pandas — those are Phases 2–3.

## How to regenerate

See [the harness README](../README.md#how-to-regenerate-with-environment-capture) for the
environment-capture snippet. The timings here were produced with:

```bash
# timings (default Criterion settings: 100 samples, default warm-up/measurement time)
cargo bench --manifest-path benchmarks/Cargo.toml --bench core      -- --noplot
cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
# optional dynamic micro-workload (dynamic_try_numeric below)
cargo bench --manifest-path benchmarks/Cargo.toml --bench core --features dynamic -- --noplot

# peak RSS (Linux) — requires GNU time (/usr/bin/time), see the Peak RSS note below
/usr/bin/time -v cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
```

## Environment

Numbers below are specific to this machine and not comparable across machines.

| Field | Value |
|---|---|
| OS | Ubuntu 26.04 LTS |
| Kernel | 7.0.0-22-generic |
| CPU | 8 vCPU, AMD Ryzen-based (**virtualized**) |
| RAM | 7,600,936 kB (~7.25 GiB) |
| rustc | 1.93.1 (01f6ddf75 2026-02-11) |
| cargo | 1.93.1 (083ac5135 2025-12-15) |
| target | x86_64-unknown-linux-gnu |
| profile | bench (opt-level 3) |
| Criterion settings | defaults (100 samples), `--noplot` |
| git commit | 5953c9f |
| workspace version | 0.28.3 |
| peak RSS tool | not available this run (no GNU `/usr/bin/time`) |

## What was measured

- Core micro set: construction, reshape/flatten, elementwise add/mul, broadcasting,
  `sum`/`mean`, `sum_axis`/`mean_axis`, `matmul`, slice, and (under `--features dynamic`)
  the dynamic `try_numeric` ingestion path.
- Scenario set: cosine similarity, Markov step, PageRank step, linear-regression GD step,
  heat-equation step.

## What was not measured

- Peer libraries (`ndarray`/`nalgebra`) — Phase 2, see the peer-comparison report.
- Cross-language references (NumPy/Pandas) — Phase 3, deferred.
- Peak RSS — not captured this run (see note below).

## Core baseline

Medians from the default-feature run; `dynamic_try_numeric` from a `--features dynamic` run.

| Workload | Time (median) | Notes |
|---|---|---|
| core/construction | 996 ns | 4096-element vector |
| core/reshape_flatten | 938 ns | |
| core/elementwise_add | 10.28 µs | 4096 elements |
| core/elementwise_mul | 10.34 µs | 4096 elements |
| core/broadcasting | 20.27 µs | [64,64] + [64] |
| core/sum_mean | 3.23 µs | 4096-element vector |
| core/sum_mean_axis | 1.303 ms | 64×64; **combined** cost of `sum_axis(0)` and `mean_axis(0)` in one workload body; slowest core op (Criterion flagged the 5 s window as tight) |
| core/matmul | 77.76 µs | 64×64 |
| core/slice_rows | 83.89 µs | first 8 rows of 64×64 |
| core/dynamic_try_numeric | 36.07 µs | 4096 elements; requires `--features dynamic` |

## Scenario baseline

| Workload | Time (median) | Notes |
|---|---|---|
| scenario/cosine_similarity | 803 ns | length 512 |
| scenario/markov_step | 892 ns | n = 64 |
| scenario/pagerank_step | 6.65 µs | n = 64 |
| scenario/linreg_gd_step | 2.23 µs | m = 256 |
| scenario/heat_step | 6.49 µs | n = 64 |

## Peak RSS

**Not captured in this run.** The VM lacked GNU `/usr/bin/time` (the shell builtin `time` does not
accept `-v`), so the `Maximum resident set size` line was unavailable. Peak RSS is informative-only
and never a gate (see the methodology); to capture it, install GNU `time` (`apt-get install time`)
and re-run the memory command above. For reference, the accepted v0.1 baseline recorded
~44 MiB for the full `scenarios` run under the same methodology and machine class — dominated by
Criterion's own footprint rather than the small tensors.

## Interpretation

Consistent with the accepted v0.1 baseline; tied strictly to these workloads on this machine:

- The cheapest operations remain construction (~1 µs) and the small scenario steps — cosine
  similarity (~800 ns) and a Markov step (~890 ns).
- Elementwise add/mul over 4096 elements sit around ~10 µs; broadcasting a row over a 64×64
  matrix ~20 µs; a 64×64 `matmul` ~78 µs and an 8-row slice ~84 µs.
- The clearest signal is again **`sum_mean_axis` at ~1.30 ms** — roughly **~400× the whole-tensor
  `sum_mean`** (~3.23 µs) and **~17× a 64×64 `matmul`**. Axis reductions remain `matten`'s most
  expensive core path by a wide margin, and the natural first place to look if axis-reduction cost
  ever matters. Positioning / regression-visibility information, not a defect: `matten` is a DX-first
  crate, not a performance crate.
- `dynamic_try_numeric` (~36 µs for 4096 elements) costs roughly 3.5× an elementwise op —
  reasonable for the dynamic ingestion-and-conversion path.

These figures match the v0.1 baseline within run-to-run VM variance; no internal regression is
visible from v0.22.3 to v0.28.3. Absolute timings drift a little run-to-run with VM load (the small
scenario steps in particular); the shape of the results, not the exact microseconds, is the signal.
No cross-library or "faster than X" claim is made or implied.

## Limitations

Single **virtualized** machine (8 vCPU VM), small inputs, microbenchmark methodology. Outlier rates
were moderate (≈4–13% per workload), consistent with a VM; medians are used throughout. Peak RSS was
not captured this run. These numbers describe `matten`'s own behavior on these workloads on this
machine; they are not a cross-library ranking and are not comparable to figures from a different
machine or OS.
