# Benchmark methodology

This page records how `matten`'s benchmarks are measured and the rules that keep the
program honest. It reflects **Phase 1** (internal Rust baseline) of RFC-049.

## Purpose

Clarify `matten`'s position with reproducible evidence: execution time, memory
behavior, example-code size (ELOC), and dependency footprint. The output is a
positioning and regression-visibility tool, not a ranking or a marketing claim.

## Non-goals

The benchmark program must not:

- claim `matten` is faster than NumPy, or a replacement for `ndarray`/`nalgebra`;
- include SciPy, Pandas, Candle, or GPU suites;
- add hard CI speed-fail thresholds (initially);
- change any public API merely to make a benchmark faster;
- pressure the project into scope creep.

## Metrics

- **Execution time** — measured with [`criterion`](https://github.com/bheisler/criterion.rs)
  for Rust microbenchmarks: inputs are pinned and built outside the timed body, no
  printing happens inside the measured section, and `black_box` is used to prevent the
  optimizer from deleting the work.
- **Memory** — peak resident set size (see below). Informative, not a gate.
- **Example ELOC** and **dependency footprint** — reported alongside timings when
  available, to show approachability and dependency trade-offs.

## Workloads (Phase 1)

A **core micro set**: construction, reshape/flatten, elementwise add/mul,
broadcasting, `sum`/`mean`, `sum_axis`/`mean_axis`, `matmul`, and a small slice. An
optional dynamic `try_numeric` micro-workload is available behind the harness's
`dynamic` feature.

A **scenario set** of five small, well-known computations taken from the examples:
cosine similarity, a Markov-chain step, a tiny PageRank step, a linear-regression
gradient-descent step, and a 1-D heat-equation step.

Heavier examples (k-means, nearest-neighbor, finite differences, trapezoidal
integration) and any peer/reference comparisons are deferred to later phases.

## Memory measurement policy

Phase 1 uses **Linux peak RSS**, which is coarse but adequate and requires no
allocator instrumentation:

```bash
/usr/bin/time -v cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
# record "Maximum resident set size"
```

Measuring smaller per-scenario commands gives a more useful figure than one giant
run. No custom global allocator and no allocation-level instrumentation are added in
Phase 1. macOS (`/usr/bin/time -l`) and Windows are deferred; memory must never block
Phase 1 if allocation-level measurement is not ready.

## Environment recording

Every report records: OS, kernel, CPU, RAM, `rustc` version, target, build profile,
the exact command, and the peak-RSS tool. Benchmarks are workload- and
environment-specific; numbers from different machines are not directly comparable.

A runnable capture snippet for these fields, plus the full regenerate steps, lives in the harness
README under [How to regenerate (with environment
capture)](../../../benchmarks/README.md#how-to-regenerate-with-environment-capture).

## CI policy

CI compile-checks the harness (`cargo bench --manifest-path benchmarks/Cargo.toml
--no-run`) but does **not** run full benchmarks. CI may fail if the harness does not
compile, a report generator breaks, or a result schema is invalid — but never because
a run is slower or uses more memory than a previous run. There are no hard performance
gates.

## Required disclaimer (in every report)

> These results are workload-specific and environment-specific. They are for
> positioning and regression visibility, not universal ranking.

## Phase 2 — Rust peer comparison (implemented)

Phase 2 was authorized once the maintainer-run internal baseline was accepted, and the
peer-comparison harness is implemented. Peer comparison is:

- **task-scoped, not library-scoped** — a task is included only if the compared
  implementations solve the same small mathematical problem with comparable data
  representation and no hidden extra library capability. It is a Rust peer comparison for
  positioning, **never** a competitor ranking or a "faster than X" claim;
- **opt-in** — behind the `peers` feature (`ndarray`/`nalgebra` as optional deps), off by
  default, so the default harness build and ordinary CI stay peer-free. The peers bench is
  compile-checked only in a separate, manually/scheduled workflow, never with speed gates;
- **isolated** — published crates are positively proven free of peer dependencies by
  `scripts/check-published-dependency-isolation.sh` (the `matten-ndarray → ndarray` bridge
  is the one allowed exception).

Run it with `cargo bench --manifest-path benchmarks/Cargo.toml --features peers --bench
peers -- --noplot`; results go in `benchmarks/reports/peer-comparison-v0.1.md`.

The Phase 2 **harness, report template, and official peer report are complete**: the
official Rust peer comparison was filled from a maintainer run on the same machine class as
the accepted internal baseline and **accepted by architect ruling on 2026-06-25**
(`benchmarks/reports/peer-comparison-v0.1.md`, Report ID
`matten-rfc049-rust-peer-comparison-v0.1`). Phase 3 (NumPy/Pandas) and hard performance
gates remain **not** authorized.
