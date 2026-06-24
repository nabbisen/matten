# Benchmarks

`matten` keeps a small, reproducible benchmarking and positioning program
(RFC-049). Its goal is to describe `matten`'s position **honestly and with
evidence**, not to win a performance contest.

The program answers questions like:

- What is `matten` good at?
- Where is it intentionally simpler?
- Where is it slower but acceptable?
- Where would performance become a blocker?
- How much code does a user write to solve small problems?

It deliberately does **not** claim that `matten` replaces `ndarray`, `nalgebra`,
NumPy, SciPy, Pandas, or Candle. `matten` is a small, approachable, `Tensor`-centered
Rust numeric crate for PoC, learning, and small workflows; the benchmarks exist to
make that position legible.

## Current status: Phase 1

The benchmark program is staged. Only **Phase 1 — internal Rust baseline** is
implemented today:

- a benchmark harness (`benchmarks/`, kept outside the workspace and unpublished);
- a core micro set and five scenario workloads drawn from the examples;
- a peak-RSS memory note on Linux;
- an internal baseline report.

Deferred (designed in RFC-049, not yet implemented):

- **Phase 2** — Rust peer comparison (`ndarray`/`nalgebra`, small matrix/vector tasks);
- **Phase 3** — ecosystem reference comparison (NumPy/Pandas), script-driven;
- **Phase 4** — regression tracking policy and thresholds.

## Read next

- [Methodology](./methodology.md) — what is measured, what is not, and the rules that
  keep the program honest.
- The harness itself lives in `benchmarks/` with its own `README.md`.
