# Internal baseline — v0.1

> **These results are workload-specific and environment-specific. They are for
> positioning and regression visibility, not universal ranking.**

This is the Phase 1 internal Rust baseline (RFC-049). It measures `matten` against
itself, to establish a reference point and make future regressions visible. It does
**not** compare against `ndarray`, `nalgebra`, NumPy, or Pandas — those are Phases 2–3
and are not yet implemented.

> When this report is completed on a representative machine (every item in
> [`BASELINE-READY-CHECKLIST.md`](./BASELINE-READY-CHECKLIST.md) satisfied) and accepted,
> it becomes the artifact that unlocks RFC-049 Phase 2 authorization.

## How to regenerate

```bash
# timings
cargo bench --manifest-path benchmarks/Cargo.toml --bench core      -- --noplot
cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot

# peak RSS (Linux), per set
/usr/bin/time -v cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
```

## Environment

Fill in on the measuring machine; numbers below are not comparable across machines.

| Field | Value |
|---|---|
| OS | _e.g. Ubuntu 24.04_ |
| Kernel | _uname -r_ |
| CPU | _model_ |
| RAM | _total_ |
| rustc | _rustc -V_ |
| target | _e.g. x86_64-unknown-linux-gnu_ |
| profile | bench (opt-level 3) |
| peak RSS tool | `/usr/bin/time -v` |

## What was measured

- Core micro set: construction, reshape/flatten, elementwise add/mul, broadcasting,
  `sum`/`mean`, `sum_axis`/`mean_axis`, `matmul`, slice.
- Scenario set: cosine similarity, Markov step, PageRank step, linear-regression GD
  step, heat-equation step.

## What was not measured

- Peer libraries (`ndarray`/`nalgebra`) — Phase 2, deferred.
- Cross-language references (NumPy/Pandas) — Phase 3, deferred.
- Allocation-level memory — not instrumented in Phase 1 (peak RSS only).
- The optional dynamic `try_numeric` micro-workload unless `--features dynamic` is set.

## Core baseline

| Workload | Time (median) | Notes |
|---|---|---|
| core/construction | _fill_ | |
| core/reshape_flatten | _fill_ | |
| core/elementwise_add | _fill_ | |
| core/elementwise_mul | _fill_ | |
| core/broadcasting | _fill_ | |
| core/sum_mean | _fill_ | |
| core/sum_mean_axis | _fill_ | |
| core/matmul | _fill_ | 64×64 |
| core/slice_rows | _fill_ | |

## Scenario baseline

| Workload | Time (median) | Notes |
|---|---|---|
| scenario/cosine_similarity | _fill_ | length 512 |
| scenario/markov_step | _fill_ | n = 64 |
| scenario/pagerank_step | _fill_ | n = 64 |
| scenario/linreg_gd_step | _fill_ | m = 256 |
| scenario/heat_step | _fill_ | n = 64 |

## Peak RSS

_Record "Maximum resident set size" from `/usr/bin/time -v`, per set._

## Interpretation

_Once the table is filled: note where `matten` is cheap (small ops), where fixed
overhead dominates at small sizes, and where it is acceptable-but-slower. Keep claims
modest and tied to the measured workloads._

## Limitations

Single machine, small inputs, microbenchmark methodology. These numbers describe
`matten`'s own behavior on these workloads; they are not a cross-library ranking.

---

## Appendix — illustrative sample (NOT the official baseline)

Captured in the CI/dev sandbox with reduced Criterion settings
(`--warm-up-time 0.1 --measurement-time 0.3 --sample-size 10`), purely to confirm the
harness runs end-to-end. **Do not cite these as the baseline** — regenerate on a real
measuring machine with default Criterion settings and fill the tables above.

| Workload | Sample median | Workload | Sample median |
|---|---|---|---|
| core/construction | ~2.26 µs | scenario/markov_step | ~1.36 µs |
| core/reshape_flatten | ~1.98 µs | scenario/pagerank_step | ~10.8 µs |
| core/elementwise_add | ~18.6 µs | scenario/linreg_gd_step | ~2.19 µs |
| core/broadcasting | ~36.6 µs | scenario/heat_step | ~10.5 µs |
| core/sum_mean | ~6.26 µs | | |
