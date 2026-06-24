# Internal baseline — "report ready" checklist (Phase 2 unlock)

RFC-049 Phase 2 (Rust peer comparison) is **not authorized** until a maintainer-run,
credible internal baseline report exists and is accepted (architect ruling, 2026-06-24,
§B4). A template or sandbox/illustrative sample is **not** sufficient.

The unlock artifact is a completed `benchmarks/reports/internal-baseline-v0.1.md` (or an
equivalently named report) produced on a representative machine, satisfying every item
below.

## Checklist

```text
[ ] run performed on a maintainer-approved representative machine
[ ] OS / kernel recorded
[ ] CPU / RAM recorded
[ ] rustc / cargo / toolchain recorded
[ ] git commit recorded
[ ] workspace version recorded
[ ] feature flags recorded
[ ] benchmark commands recorded
[ ] core microbenchmark table filled
[ ] scenario benchmark table filled
[ ] peak RSS recorded using /usr/bin/time -v, or infeasibility documented
[ ] results are labeled internal baseline, not peer comparison
[ ] report includes limitations
[ ] report includes no marketing claims
```

## After the checklist is complete

Only once this artifact is delivered and accepted should RFC-049 Phase 2 implementation be
authorized — separately, as its own decision. The Phase 2 design itself is already settled
(see RFC-049 §"Phase 2: Rust peer comparison" — peer-dependency isolation, the opt-in
`peers` feature, the fixed comparable-task list, and the build/CI shape).
