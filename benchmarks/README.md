# `matten` benchmark harness (RFC-049, Phase 1)

A small, reproducible benchmark harness for the `matten` workspace. Its purpose is
to **clarify `matten`'s position with evidence** — execution time, memory behavior,
example-code size, dependency footprint — **not** to claim `matten` is faster than
larger ecosystems. See [the methodology](../docs/src/benchmarks/methodology.md) for
the full policy.

## Status: Phase 1 (internal baseline only)

This harness currently implements **Phase 1**: an internal Rust baseline. Peer
comparisons (`ndarray`/`nalgebra`, Phase 2) and cross-language reference comparisons
(NumPy/Pandas, Phase 3), and any regression thresholds (Phase 4), are designed in
RFC-049 but **not yet implemented**.

## Isolation

This crate is intentionally **outside** the Cargo workspace (`workspace.exclude` in
the root `Cargo.toml`) and is `publish = false`. Its benchmark-only dependency
(`criterion`) must never enter the dependency graph of any published crate. It is
always invoked with an explicit manifest path, and it has its own (git-ignored)
`Cargo.lock`.

## Layout

```text
benchmarks/
  Cargo.toml            # excluded from the workspace; publish = false
  src/
    common.rs           # deterministic, pinned input generators
    workloads/
      core.rs           # core micro-workloads
      scenarios.rs      # five scenario workloads (from examples 26/33/34/35/36)
  benches/
    core.rs             # criterion target for the core set
    scenarios.rs        # criterion target for the scenario set
  reports/              # committed: curated reports
  results/              # committed: small sample schemas only (not bulky histories)
```

## Running

Compile-check only (what CI runs):

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --no-run
```

Run a set locally:

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --bench core -- --noplot
cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
```

Optional dynamic micro-workload (off by default):

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --bench core --features dynamic -- --noplot
```

## Memory (peak RSS, Phase 1)

Memory is **informative, not a gate**. On Linux, measure peak resident set size:

```bash
/usr/bin/time -v cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
# read "Maximum resident set size"
```

Record the environment (OS, kernel, CPU, RAM, rustc, target, profile, command) in
every report. macOS (`/usr/bin/time -l`) and Windows are deferred.

## What this is not

No "faster than NumPy" claims, no SciPy/Pandas/Candle/GPU suites, no hard CI
speed-fail thresholds, and no public API changes to make benchmarks faster. The
program must not pressure the project into scope creep (RFC-049 §5).
