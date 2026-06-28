# `matten` benchmark harness (RFC-049) — Phase 1 baseline and Phase 2 peer comparison

A small, reproducible benchmark harness for the `matten` workspace. Its purpose is
to **clarify `matten`'s position with evidence** — execution time, memory behavior,
example-code size, dependency footprint — **not** to claim `matten` is faster than
larger ecosystems. See [the methodology](../docs/src/benchmarks/methodology.md) for
the full policy.

## Status: Phase 1 baseline + Phase 2 peer comparison (opt-in)

Phase 1 (internal Rust baseline) is implemented and **accepted**. **Phase 2** (Rust peer
comparison vs `ndarray` / `nalgebra`) is **complete and accepted** (architect ruling
2026-06-25): the official report is filled from a maintainer run on the baseline's machine
class (`reports/peer-comparison-v0.1.md`). Phase 2 remains **opt-in** behind the `peers`
feature and off by default. Cross-language reference comparisons (NumPy/Pandas, Phase 3) and
any regression thresholds (Phase 4) are designed in RFC-049 but **not yet
implemented/authorized**.

## How to regenerate (with environment capture)

This is the maintainer path: capture the environment, compile-check, then run the sets. Record the
captured fields in the report's Environment table — benchmarks are environment-specific and numbers
from different machines are not comparable.

**1. Capture the environment** (Linux; record each value in the report):

```bash
cat /etc/os-release | grep PRETTY_NAME      # OS
uname -r                                     # kernel
lscpu | grep 'Model name'                    # CPU
grep MemTotal /proc/meminfo                  # RAM
rustc -V && cargo -V                         # toolchain
rustc -vV | grep host                        # target triple
git rev-parse --short HEAD                   # commit
grep '^version' Cargo.toml                   # workspace version
```

**2. Compile-check the harness** (this is also what CI runs):

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --no-run
```

**3. Run the internal baseline sets** (default Criterion settings: 100 samples):

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --bench core      -- --noplot
cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
# optional dynamic micro-workload (off by default)
cargo bench --manifest-path benchmarks/Cargo.toml --bench core --features dynamic -- --noplot
```

**4. Peak RSS** (Linux; informative, **not** a gate). Requires GNU `time` (`/usr/bin/time`); the
shell builtin will not accept `-v`. Install with `apt-get install time` if missing:

```bash
/usr/bin/time -v cargo bench --manifest-path benchmarks/Cargo.toml --bench scenarios -- --noplot
# read "Maximum resident set size"
```

**5. Phase 2 peer comparison** (opt-in; pulls `ndarray` + `nalgebra`; never in the default build or
ordinary CI — compile-checked only by the manual/scheduled peers workflow,
`.github/workflows/benchmarks-peers.yml`):

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --features peers --bench peers -- --noplot
```

macOS (`/usr/bin/time -l`) and Windows are deferred. Curated reports live in `reports/`; the
in-book reader summary is [`docs/src/benchmarks/results.md`](../docs/src/benchmarks/results.md).

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

## What this is not

No "faster than NumPy" claims, no SciPy/Pandas/Candle/GPU suites, no hard CI
speed-fail thresholds, and no public API changes to make benchmarks faster. The
program must not pressure the project into scope creep (RFC-049 §5).
