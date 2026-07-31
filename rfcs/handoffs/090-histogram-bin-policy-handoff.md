# RFC-090 Histogram: Implementation Handoff

**Project:** `matten`
**Governing RFC:** `rfcs/proposed/090-histogram-bin-policy.md`
**Document kind:** Detailed implementation handoff
**Status:** Inherits RFC-090's state — **implemented and reviewed 2026-07-31** (approved after one
correction). This handoff's own §3 edge formula was found to be wrong — `(range * bins) / bins` can
overflow before the division recovers — and the implementer's pinning of `edges[bins] = hi` was a
correctness fix rather than the tidy-up the formula implied
**Date:** 2026-07-31

---

## 1. Purpose

Add `histogram` and its `Histogram` return struct to `matten-stats`, closing the bin-selection policy
RFC-040 §8 left open in v0.21.2.

**The policy is decided: no automatic bin rule.** `bins` is a required argument. If you find yourself
adding Sturges, Freedman–Diaconis, Scott, or an `"auto"` mode, stop — RFC-090 §4.1 rejects all of them
and §9 lists them as non-goals.

No release, no version bump. Version stays `0.41.0`.

## 2. API

```rust
/// counts.len() == bins ; edges.len() == bins + 1
pub struct Histogram {
    pub counts: Vec<usize>,
    pub edges: Vec<f64>,
}

pub fn histogram(x: &Tensor, bins: usize) -> Result<Histogram, MattenStatsError>;
```

New module `crates/matten-stats/src/histogram.rs`; `mod` + `pub use` in `lib.rs` beside the others.

**No `try_` pairing.** All six existing `matten-stats` functions return `Result` and there are no
panicking convenience forms in this crate. Do not introduce the pattern here.

`Histogram` is a plain data struct: public fields, no methods, no `impl` beyond derives. Derive what
the siblings derive (check `SchemaSummary`-style precedent in the workspace; `Debug` + `Clone` at
minimum).

## 3. Algorithm

One pass for the range, one to bin. **No sorting** — that was `quantile`'s cost, not this one.

```text
1. reject dynamic (x.is_dynamic())
2. reject bins == 0                    -> InvalidBinCount
3. reject empty input                  -> Empty
4. reject any non-finite value         -> NonFiniteValue
5. lo = min(x), hi = max(x)
6. reject lo == hi                     -> ZeroVariance          (§4 below)
7. check `bins` against the allocation limit BEFORE allocating  (§5)
8. edges[i] = lo + (hi - lo) * i / bins,  for i in 0..=bins
9. for each value v: idx = floor((v - lo) / (hi - lo) * bins), clamped so that
   v == hi lands in bin bins-1        (§4.3 of the RFC — the closed last bin)
10. counts[idx] += 1
```

Compute `edges[i]` from `lo` and `i` as above rather than by repeated addition of a width — repeated
addition accumulates float error and can leave `edges[bins] != hi`.

## 4. The two divergences from NumPy, both deliberate

**Closed last bin — MATCH NumPy.** Bin `i` covers `[edges[i], edges[i+1])` except the last, which is
`[edges[bins-1], edges[bins]]`. Without this the maximum value falls in no bin and disappears from the
counts. That would be a *silent* wrong answer, so the ecosystem convention wins (RFC-087 §6).

**Constant input — DIVERGE from NumPy.** When `min == max`, NumPy widens the range to
`(v - 0.5, v + 0.5)`. Return `MattenStatsError::ZeroVariance` instead. The `0.5` comes from nowhere in
the data, and the divergence is *visible* — an error — so RFC-087 §6 permits it. It also matches
`correlation`/`skewness`/`kurtosis`, which already reject zero variance rather than return `NaN`.

## 5. Allocation guard — new for this crate

`bins` is caller-supplied and unbounded. Validate against `matten::MattenLimits::default().max_elements`
**before** allocating `counts` or `edges`.

`MattenLimits` is publicly exported from core `matten`, so **no new dependency** — `matten-stats` must
still depend on `matten` alone. This is the crate's first use of it; check how
`crates/matten/src/composition.rs` uses `check_shape` and follow the spirit, but note you are bounding
a count, not a shape, so a direct comparison against `max_elements` may be the cleaner fit. Whichever
you choose, the failure must not be `Empty` — that variant means "too few elements", the opposite
condition.

## 6. The error variant

Add `InvalidBinCount` to `MattenStatsError`, with a `Display` arm in the same style as its siblings.
Additive and non-breaking under `#[non_exhaustive]`.

Follow `matten-data`'s `InvalidBatchSize` (RFC-082) as the model — same situation, same shape.

Also update the `Empty` variant's doc comment if it enumerates which functions can produce it.

## 7. The boundary amendment — this is the part reviewers will look at hardest

RFC-078 §5 said `matten-stats` is `Tensor -> f64`. `Histogram` breaks that, deliberately, and RFC-090
§5 amends the rule to:

> `matten-stats` computes **statistical summaries**. A summary is returned as `f64` where it is
> scalar, and as a **small owned struct** where it is inherently vector-valued. `matten-stats` never
> returns a `Tensor`.

**The old framing sits at five sites across three files**, measured at `766ac52` — do not stop at
the first one in each file:

```text
crates/matten-stats/src/lib.rs:1     "small, explicit scalar statistics over Tensor"
crates/matten-stats/src/lib.rs:12    "computes scalar statistical summaries: Tensor -> f64"
crates/matten-stats/README.md:16     "small, explicit scalar statistics over Tensor"
crates/matten-stats/README.md:27     "computes scalar statistical summaries: Tensor -> f64"
docs/src/reference/stats.md:108      "Tensor -> f64 scalar statistics that core deliberately excludes"
```

Re-measure rather than trusting those line numbers, and leave the old wording nowhere — the crate
would otherwise document a rule its own API breaks.

**It does not unblock matrix-wide or axis-wise statistics.** Those return a `Tensor`, which the
amended rule still forbids. Do not add them, and do not soften the "never returns a `Tensor`" clause.

## 8. Required tests

```text
[ ] counts.len() == bins and edges.len() == bins + 1
[ ] SUM INVARIANT: counts.iter().sum::<usize>() == x.len(), on several inputs.
    Include one where the maximum lands exactly on the final edge -- this is the
    test that proves the closed last bin (§4). Without it, an off-by-one drops
    the max and the sum is x.len() - 1
[ ] a hand-computed example: known input, known bins, exact counts AND exact edges
[ ] edges evenly spaced: edges[0] == min, edges[bins] == max (exactly, per §3's
    formula), and consecutive differences equal within f64 tolerance
[ ] bins == 1 puts everything in one bin
[ ] bins == 0                -> InvalidBinCount
[ ] constant input           -> ZeroVariance   (NOT a widened range)
[ ] dynamic tensor           -> DynamicTensor
[ ] NaN and infinity         -> NonFiniteValue
[ ] a huge `bins` (> max_elements) returns the allocation error WITHOUT allocating
[ ] the six existing functions still pass untouched
```

## 9. Documentation

```text
crates/matten-stats/src/histogram.rs   doc comments: the bins argument and WHY there is no
                                       automatic rule (§4.1 of the RFC, in one sentence);
                                       the closed-last-bin rule and why; the ZeroVariance
                                       divergence from NumPy. A runnable doctest.
crates/matten-stats/src/lib.rs         the amended boundary (§7)
crates/matten-stats/README.md          amended boundary + public-API list (now 7 functions)
docs/src/reference/stats.md            amended boundary + histogram section
crates/matten-stats/examples/          one example, stats_* prefixed, WITH its [[example]]
                                       entry in Cargo.toml (matten-stats DOES use explicit
                                       entries, unlike core matten) and a CI smoke-run line
```

Note the asymmetry in that last line: RFC-087's review established that **core** `matten` needs no
`[[example]]` entry because Cargo auto-discovers, but `matten-stats` declares its examples explicitly
with `stats_*` names to avoid a binary-name collision. Check `crates/matten-stats/Cargo.toml` and
follow what is there.

## 10. Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test -p matten-stats --features dynamic
cargo test --doc --all-features
cargo +1.85.0 build && cargo +1.85.0 test --all-features
cargo run -p matten-stats --example <the new example>
bash scripts/check-release-docs.sh
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash scripts/check-matten-data-scope.sh
mdbook build docs
git diff --check
```

`check-release-docs.sh` asserts `matten-stats` declares **production-ready candidate** — unchanged by
this slice, so it must keep passing unmodified. If it fails, a maturity label moved that should not
have.

Scope confirmation:

```bash
git diff --name-only -- crates/matten crates/matten-data crates/matten-mlprep crates/matten-ndarray
# expect EMPTY
grep -n 'Tensor -> f64\|output: f64' crates/matten-stats/README.md crates/matten-stats/src/lib.rs docs/src/reference/stats.md
# expect the OLD framing to be gone (§7)
grep -m1 '^version' Cargo.toml     # still 0.41.0
```

## 11. Known pitfalls

1. **Adding an automatic bin rule.** The whole point of the RFC is that there isn't one.
2. **Dropping the maximum value** through an open last bin. The sum invariant catches it (§8).
3. **Computing edges by repeated addition of a width** — float drift leaves `edges[bins] != max` (§3).
4. **Widening the range on constant input** the way NumPy does (§4).
5. **Leaving the old `Tensor -> f64` wording** in any of the three doc locations (§7).
6. **Reading the amendment as permission** to add matrix-wide or axis-wise stats (§7).
7. **Using `Empty` for a too-large `bins`** — it means the opposite condition (§5).
8. **Adding a dependency.** `MattenLimits` is re-exported by core `matten`.
9. **Adding `try_histogram`.** This crate has no panicking forms (§2).

## 12. What the review request must report

```text
[ ] the sum-invariant test, including the max-on-final-edge case
[ ] the hand-computed example with exact counts and edges
[ ] the allocation test, showing rejection without a large allocation
[ ] every error case with its asserted variant, incl. ZeroVariance on constant input
[ ] the amended boundary quoted from all three doc locations, and evidence the old
    `Tensor -> f64` wording survives nowhere
[ ] check-release-docs.sh passing UNMODIFIED with the maturity label unchanged
[ ] confirmation matten-stats still depends only on matten
[ ] full gate set incl. MSRV and mdbook; version still 0.41.0
```

## 13. Review stop

Acceptance makes this a commit point. It authorizes no release, version bump, tag, or publish.
