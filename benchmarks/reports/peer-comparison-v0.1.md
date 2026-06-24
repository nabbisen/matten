# Rust peer comparison — v0.1

> **Report status:** Template only. Official maintainer-run peer numbers are pending.
> Do not cite the appendix sandbox sample as official evidence.

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

Fill in on the measuring machine; must be the **same environment** as the internal
baseline for the numbers to relate. Not comparable across machines.

| Field | Value |
|---|---|
| OS / kernel | _fill_ |
| CPU (note if virtualized) | _fill_ |
| RAM | _fill_ |
| rustc / cargo | _fill_ |
| target | _fill_ |
| profile | bench (opt-level 3) |
| Criterion settings | defaults, `--noplot` |
| git commit | _fill_ |
| workspace version | _fill_ |
| ndarray version | 0.16.x |
| nalgebra version | 0.33.x |

## Comparable tasks

Every task below is the *same small mathematical problem* at the same sizes, expressed in
each library's native type (`matten::Tensor`, `ndarray::Array1/Array2`,
`nalgebra::DVector/DMatrix`). All six are small dense vector/matrix operations, which is
why both peers cover all six.

| Task | Problem | matten | ndarray | nalgebra | What is *not* compared |
|---|---|---|---|---|---|
| cosine_similarity | dot / (‖a‖·‖b‖), len 512 | _fill_ | _fill_ | _fill_ | N-D, broadcasting |
| matmul | 64×64 dense product | _fill_ | _fill_ | _fill_ | strided/large/blocked matmul |
| markov_step | v·P, n=64 | _fill_ | _fill_ | _fill_ | sparse transition matrices |
| pagerank_step | M·r + damping, n=64 | _fill_ | _fill_ | _fill_ | sparse graphs, convergence loop |
| linreg_gd_step | one GD step, m=256 | _fill_ | _fill_ | _fill_ | full training loop, solvers |
| heat_step | operator·u, n=64 | _fill_ | _fill_ | _fill_ | stencil/sparse operators |

## Interpretation

Keep it modest and task-scoped, in the "positioning, not ranking" register. The expected
shape of the story: on these small dense vector/matrix kernels, `ndarray`/`nalgebra` are
production-oriented numeric crates and will typically have less overhead than `matten`'s
approachable `Tensor` API. Frame any gap as `matten` trading some performance for
approachability and a small, `Tensor`-centered API aimed at PoC, learning, and small
workflows — and as information that helps a user decide when to stay with `matten` and when
to bridge/migrate intentionally. State where the gap is small, where it is large (for
example dense `matmul`), and where it is irrelevant at these sizes.

Use wording like "`matten` trades performance for approachability on these small dense
kernels." Avoid "matten loses / is worse / too slow" or "ndarray/nalgebra are superior".
This is the family-car-to-super-car migration framing, not a verdict.

## Limitations

Single machine, small fixed sizes, microbenchmark methodology, medians over noisy samples
(more so on a VM). These are library-natural representations of *these* tasks only; a
different task, size, or representation could shift every number. This is not a general
statement that any library is "better" — it is a positioning snapshot for `matten` on a
fixed, comparable task set.

---

## Appendix — illustrative sandbox sample (NOT official)

Captured in the dev sandbox with reduced Criterion settings to confirm the harness runs;
**do not cite**. Regenerate on the representative machine with default settings.

| Task | matten | ndarray | nalgebra |
|---|---|---|---|
| cosine_similarity | ~1.84 µs | ~333 ns | ~292 ns |
| matmul (64×64) | ~267 µs | ~20.5 µs | ~20.5 µs |
