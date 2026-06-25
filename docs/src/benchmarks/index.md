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

## Current status

The benchmark program is staged.

- **Phase 1 — internal Rust baseline: implemented and accepted.** A benchmark harness
  (`benchmarks/`, kept outside the workspace and unpublished); a core micro set and five
  scenario workloads drawn from the examples; a peak-RSS memory note on Linux; and an
  accepted internal baseline report.
- **Phase 2 — Rust peer comparison (`ndarray`/`nalgebra`): complete and accepted.** The
  official peer comparison was filled from a maintainer run on the baseline's machine class
  and accepted by architect ruling on 2026-06-25. Peer tasks are opt-in behind the `peers`
  feature (off by default).

Still deferred (designed in RFC-049, not yet implemented/authorized):

- **Phase 3** — ecosystem reference comparison (NumPy/Pandas), script-driven;
- **Phase 4** — regression tracking policy and hard thresholds/gates.

## Read next

- [Methodology](./methodology.md) — what is measured, what is not, and the rules that
  keep the program honest.
- The harness itself lives in `benchmarks/` with its own `README.md`.
