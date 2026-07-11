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
- **Phase 3 — Python reference comparison (NumPy/Pandas): implemented and accepted.**
  Optional scripts record ELOC, dependency footprint, versions, and code-shape notes. Runtime
  context is omitted by default and must never be used as a ranking. The report was refreshed with
  pinned Python dependencies and accepted by review on 2026-07-11.

Still deferred (designed in RFC-049, not yet implemented/authorized):

- **Phase 4** — regression tracking policy and hard thresholds/gates.

## Read next

Two paths, depending on what you need:

- **Just want the results?** → [Results](./results.md) — a curated, readable summary of the latest
  numbers (Phase 1 internal baseline and Phase 2 peer comparison), with the "positioning, not
  ranking" framing. This is the reader's page.
- **Need to regenerate or extend the benchmarks?** → [Methodology](./methodology.md) for what is
  measured and the rules that keep the program honest, then the harness `README.md` in
  `benchmarks/` for the maintainer path: the environment-capture snippet and the exact
  `cargo bench …` commands under [How to regenerate (with environment
  capture)](../../../benchmarks/README.md#how-to-regenerate-with-environment-capture).

The full reports (complete tables, environment, regeneration commands) live in
`benchmarks/reports/`.
